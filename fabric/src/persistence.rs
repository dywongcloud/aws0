// SPDX-License-Identifier: CC0-1.0

//! Durable, append-only persistence with per-frame and per-snapshot checksums.
//!
//! Every journal frame is independent: an eight-byte big-endian length, a
//! 32-byte SHA-256 of the record, then the record bytes. Frames are scanned
//! independently, so a checksum failure in one complete frame never hides
//! later valid frames; `truncate_corrupt_tail` rewrites the journal with only
//! checksum-verified, View-decodable frames and drops torn trailing bytes.
//! `recover` adopts the newest durable state and binds the recovered state
//! hash to the checksum recorded in the last frame or in the snapshot.
//!
//! `snapshot.json` is a versioned envelope carrying the canonical View bytes,
//! their SHA-256, a domain-separated expected state hash, and the journal
//! position the checkpoint reflects. The checksum is verified on every read;
//! a mismatch is an error. Snapshots are written to a per-process unique
//! temporary file under the journal's exclusive advisory lock, renamed into
//! place, and followed by a sync of the containing directory.
//!
//! Journal reads are bounded by `MAX_JOURNAL_BYTES` and only a bounded suffix
//! of frames is retained for replay. Unkeyed SHA-256 detects accidental
//! corruption, not forgery by a writer able to recompute checksums.
//!
//! All filesystem and decoding failures are returned as [`Error::Journal`].
//! Recovery never creates files. The only process-wide mutable state is an
//! atomic counter used solely to name temporary snapshot files; every path is
//! otherwise derived from the explicitly supplied directory.

use crate::crdt::AppliedRecord;
use crate::state::View;
use crate::types::{digest, Error, NodeId, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const JOURNAL_FILE: &str = "aws0.journal";
const SNAPSHOT_FILE: &str = "snapshot.json";
const NODE_ID_FILE: &str = "node.id";
const COMMITTED_FILE: &str = "committed";
const SNAPSHOT_VERSION: u32 = 2;
const APPLIED_FIELD: &str = "aws0_applied_v1";
const STATE_DOMAIN: &[u8] = b"aws0/state/v1\0";
const LEN_LEN: usize = 8;
const HASH_LEN: usize = 32;
const HEADER_LEN: usize = LEN_LEN + HASH_LEN;
const MAX_RECORD_BYTES: u64 = 64 * 1024 * 1024;
const MAX_JOURNAL_BYTES: u64 = 256 * 1024 * 1024;
const MAX_SNAPSHOT_BYTES: u64 = 128 * 1024 * 1024 + 4096;
const MAX_HASH_FILE_BYTES: u64 = 256;
const MAX_RETAINED_FRAMES: usize = 32;

static SNAPSHOT_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A handle to one node's on-disk journal, snapshot and identity files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Journal {
    pub dir: PathBuf,
}

/// The outcome of a full recovery, including every damaged frame detected.
/// [`Journal::recover`] refuses to return a view while this count is non-zero;
/// `recover_with_report` exposes the count for an explicit repair decision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReport {
    pub view: View,
    pub records_replayed: u64,
    pub corrupt_tail_frames: usize,
    pub state_hash: [u8; 32],
}

/// One checksum-verified journal record, indexed by its frame position.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Frame {
    checksum: [u8; 32],
    record: Vec<u8>,
}

/// How much of each frame `scan_bytes` keeps.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameRetention {
    Counts,
    TrailingFrames,
    Rebuild,
}

/// A pure view of the journal bytes: counts, a bounded suffix of valid frames,
/// the rebuilt valid prefix for repair, and the damaged suffix.
struct Scan {
    frames: VecDeque<(u64, Frame)>,
    rebuilt: Vec<u8>,
    valid_frames: u64,
    corrupt_frames: usize,
    invalid_frames: usize,
    tail_frames: usize,
    tail_partial: bool,
    tail_start: u64,
}

/// A versioned, checksummed snapshot envelope.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SnapshotEnvelope {
    version: u32,
    payload: String,
    checksum: String,
    state_hash: String,
    last_applied_ms: u64,
    journal_frames: u64,
    journal_bytes: u64,
    /// Hash over every field above, including the recovery-ordering metadata.
    /// Without it an operator edit to `journal_frames` can make a stale
    /// snapshot outrank newer journal frames without failing any checksum.
    integrity: String,
}

/// Domain-separated hash of the complete envelope. All variable-length fields
/// carry a length prefix, so no two distinct envelopes share one preimage.
#[must_use]
fn envelope_integrity(envelope: &SnapshotEnvelope) -> [u8; 32] {
    let mut buffer = b"aws0/snapshot-envelope/v2\0".to_vec();
    buffer.extend_from_slice(&envelope.version.to_be_bytes());
    for field in [
        envelope.payload.as_bytes(),
        envelope.checksum.as_bytes(),
        envelope.state_hash.as_bytes(),
    ] {
        let length = u64::try_from(field.len()).unwrap_or(u64::MAX);
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.extend_from_slice(field);
    }
    buffer.extend_from_slice(&envelope.last_applied_ms.to_be_bytes());
    buffer.extend_from_slice(&envelope.journal_frames.to_be_bytes());
    buffer.extend_from_slice(&envelope.journal_bytes.to_be_bytes());
    digest(&buffer)
}

/// A verified snapshot together with the metadata used to order recovery.
struct Snapshot {
    view: View,
    state_hash: [u8; 32],
    last_applied_ms: u64,
    journal_frames: u64,
}

/// One durable state candidate and the hash recorded for it at write time.
struct Candidate {
    view: View,
    expected_hash: [u8; 32],
    last_applied_ms: u64,
    sequence: u64,
    snapshot_frames: u64,
    has_durable_hash: bool,
}

/// Removes a temporary snapshot file unless it was already published.
struct TempPath {
    path: PathBuf,
}

impl Drop for TempPath {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[must_use]
fn journal_error(message: impl Into<String>) -> Error {
    Error::Journal(message.into())
}

#[must_use]
fn io_error(context: &str, error: std::io::Error) -> Error {
    Error::Journal(format!("{context}: {error}"))
}

/// Constant-time comparison of two public frame checksums.
#[must_use]
fn hash_eq(left: &[u8; 32], right: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for (a, b) in left.iter().zip(right.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}

/// Domain-separated hash of canonical View bytes. It is deliberately distinct
/// from the raw frame or snapshot checksum, so the recovered-state check has
/// its own expected value.
#[must_use]
fn state_digest(canonical: &[u8]) -> [u8; 32] {
    let mut buffer = Vec::with_capacity(STATE_DOMAIN.len().saturating_add(canonical.len()));
    buffer.extend_from_slice(STATE_DOMAIN);
    buffer.extend_from_slice(canonical);
    digest(&buffer)
}

#[must_use]
fn read_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    let tail = bytes.get(offset..)?;
    let raw = tail.get(..LEN_LEN)?;
    let array = <[u8; LEN_LEN]>::try_from(raw).ok()?;
    Some(u64::from_be_bytes(array))
}

#[must_use]
fn read_hash(bytes: &[u8], offset: usize) -> Option<[u8; 32]> {
    let tail = bytes.get(offset..)?;
    let raw = tail.get(..HASH_LEN)?;
    <[u8; HASH_LEN]>::try_from(raw).ok()
}

/// Decode a 64-hex-character hash field.
#[must_use]
fn decode_hex_32(text: &str, label: &str) -> Result<[u8; 32]> {
    if text.len() != 2 * HASH_LEN {
        return Err(journal_error(format!(
            "{label} must be 64 hexadecimal characters"
        )));
    }
    let decoded = hex::decode(text.as_bytes())
        .map_err(|e| journal_error(format!("{label} is not hexadecimal: {e}")))?;
    if decoded.len() != HASH_LEN {
        return Err(journal_error(format!("{label} must decode to 32 bytes")));
    }
    let mut out = [0u8; HASH_LEN];
    for (dst, src) in out.iter_mut().zip(decoded.iter()) {
        *dst = *src;
    }
    Ok(out)
}

/// Decode frame structure without performing IO. Each complete frame is
/// checksum-verified independently; only a torn trailing segment ends the
/// scan. `retention` selects whether records are retained, and how.
fn scan_bytes(bytes: &[u8], retention: FrameRetention) -> Result<Scan> {
    let total = bytes.len();
    let mut frames: VecDeque<(u64, Frame)> = VecDeque::new();
    let mut rebuilt: Vec<u8> = Vec::new();
    let mut offset = 0usize;
    let mut index = 0u64;
    let mut valid_frames = 0u64;
    let mut corrupt_frames = 0usize;
    let mut invalid_frames = 0usize;
    let mut tail_frames = 0usize;
    let mut tail_partial = false;

    while offset < total {
        let declared = match read_u64(bytes, offset) {
            Some(v) => v,
            None => {
                tail_frames = 1;
                tail_partial = true;
                break;
            }
        };
        if declared > MAX_RECORD_BYTES {
            tail_frames = 1;
            tail_partial = true;
            break;
        }
        let body_len = match usize::try_from(declared) {
            Ok(v) => v,
            Err(_) => {
                tail_frames = 1;
                tail_partial = true;
                break;
            }
        };
        let frame_end = match offset
            .checked_add(HEADER_LEN)
            .and_then(|v| v.checked_add(body_len))
        {
            Some(v) => v,
            None => {
                tail_frames = 1;
                tail_partial = true;
                break;
            }
        };
        if frame_end > total {
            tail_frames = 1;
            tail_partial = true;
            break;
        }
        let checksum = match read_hash(bytes, offset.saturating_add(LEN_LEN)) {
            Some(v) => v,
            None => {
                tail_frames = 1;
                tail_partial = true;
                break;
            }
        };
        let record = match bytes.get(offset.saturating_add(HEADER_LEN)..frame_end) {
            Some(v) => v,
            None => {
                tail_frames = 1;
                tail_partial = true;
                break;
            }
        };
        let frame_index = index;
        index = index
            .checked_add(1)
            .ok_or_else(|| journal_error("journal frame index overflow"))?;
        if !hash_eq(&digest(record), &checksum) {
            corrupt_frames = corrupt_frames.saturating_add(1);
            offset = frame_end;
            continue;
        }
        // Every complete, checksum-valid frame must also be a canonical View.
        // Validating only the retained suffix would let a malformed older frame
        // be counted as durable state and silently lost on recovery.
        let view_ok = decode_view(record, "journal frame").is_ok();
        if !view_ok {
            invalid_frames = invalid_frames.saturating_add(1);
            offset = frame_end;
            continue;
        }
        valid_frames = valid_frames
            .checked_add(1)
            .ok_or_else(|| journal_error("journal frame count overflow"))?;
        match retention {
            FrameRetention::Counts => {}
            FrameRetention::TrailingFrames => {
                if frames.len() >= MAX_RETAINED_FRAMES {
                    frames.pop_front();
                }
                frames.push_back((
                    frame_index,
                    Frame {
                        checksum,
                        record: record.to_vec(),
                    },
                ));
            }
            FrameRetention::Rebuild => {
                let raw = bytes
                    .get(offset..frame_end)
                    .ok_or_else(|| journal_error("journal frame slice is unavailable"))?;
                rebuilt.extend_from_slice(raw);
            }
        }
        offset = frame_end;
    }
    let tail_start =
        u64::try_from(offset).map_err(|_| journal_error("journal byte offset exceeds u64"))?;
    Ok(Scan {
        frames,
        rebuilt,
        valid_frames,
        corrupt_frames,
        invalid_frames,
        tail_frames,
        tail_partial,
        tail_start,
    })
}

/// Read at most `limit` bytes, returning an error if the file is larger.
fn read_bounded(file: &mut File, limit: u64, label: &str) -> Result<Vec<u8>> {
    let cap = limit
        .checked_add(1)
        .ok_or_else(|| journal_error(format!("{label} byte bound overflow")))?;
    let mut reader = file.take(cap);
    let mut bytes = Vec::new();
    reader
        .read_to_end(&mut bytes)
        .map_err(|e| io_error(&format!("read {label}"), e))?;
    let read_len = u64::try_from(bytes.len())
        .map_err(|_| journal_error(format!("{label} length exceeds u64")))?;
    if read_len > limit {
        return Err(journal_error(format!(
            "{label} exceeds the {limit}-byte bound"
        )));
    }
    Ok(bytes)
}

/// Read the journal under a shared advisory lock. A missing journal is empty.
fn read_journal(path: &Path) -> Result<Vec<u8>> {
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(io_error("open journal", e)),
    };
    file.lock_shared()
        .map_err(|e| io_error("share-lock journal", e))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|e| io_error("seek journal", e))?;
    read_bounded(&mut file, MAX_JOURNAL_BYTES, "journal")
}

/// Serialize a whole [`View`] to canonical bytes. A plain `serde_json::to_vec`
/// encoding is used whenever it succeeds; every serialization error is either
/// propagated or handled by the one known non-string-map-key fallback. The
/// fallback carries the idempotency ledger in a versioned, hex-keyed field.
fn encode_view(view: &View) -> Result<Vec<u8>> {
    match serde_json::to_vec(view) {
        Ok(bytes) => return Ok(bytes),
        Err(error) => {
            if !error.to_string().contains("key must be a string") {
                return Err(journal_error(format!("encode view: {error}")));
            }
        }
    }
    let mut sanitized = view.clone();
    let applied = std::mem::take(&mut sanitized.state.applied);
    let value = serde_json::to_value(&sanitized)
        .map_err(|e| journal_error(format!("encode view: {e}")))?;
    let mut root: BTreeMap<String, serde_json::Value> = match value {
        serde_json::Value::Object(map) => map.into_iter().collect(),
        _ => return Err(journal_error("view is not a JSON object")),
    };
    let mut aux = BTreeMap::new();
    for (key, record) in &applied {
        let encoded = serde_json::to_value(record)
            .map_err(|e| journal_error(format!("encode applied record: {e}")))?;
        aux.insert(hex::encode(*key), encoded);
    }
    let aux_value = serde_json::to_value(&aux)
        .map_err(|e| journal_error(format!("encode applied index: {e}")))?;
    root.insert(APPLIED_FIELD.to_owned(), aux_value);
    serde_json::to_vec(&root).map_err(|e| journal_error(format!("encode view: {e}")))
}

/// Decode a snapshot or journal frame. Malformed JSON is always an error, and
/// a canonical round-trip check rejects injected auxiliary fields and any
/// non-canonical encoding before the View is trusted.
fn decode_view(bytes: &[u8], context: &str) -> Result<View> {
    let mut root: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|e| journal_error(format!("{context}: malformed JSON: {e}")))?;
    let aux = match root.as_object_mut() {
        Some(object) => object.remove(APPLIED_FIELD),
        None => None,
    };
    let mut view: View = serde_json::from_value(root)
        .map_err(|e| journal_error(format!("{context}: malformed View: {e}")))?;
    if let Some(value) = aux {
        let records: BTreeMap<String, AppliedRecord> = serde_json::from_value(value)
            .map_err(|e| journal_error(format!("{context}: malformed applied index: {e}")))?;
        for (key, record) in records {
            let decoded = hex::decode(key.as_bytes()).map_err(|e| {
                journal_error(format!("{context}: applied key is not hexadecimal: {e}"))
            })?;
            if decoded.len() != HASH_LEN {
                return Err(journal_error(format!(
                    "{context}: applied key must decode to 32 bytes"
                )));
            }
            let mut id = [0u8; HASH_LEN];
            for (dst, src) in id.iter_mut().zip(decoded.iter()) {
                *dst = *src;
            }
            view.state.applied.insert(id, record);
        }
    }
    let canonical = encode_view(&view)?;
    if canonical.as_slice() != bytes {
        return Err(journal_error(format!(
            "{context}: View encoding is not canonical"
        )));
    }
    Ok(view)
}

/// Read and verify a checksummed snapshot. A missing snapshot is optional; a
/// present but malformed or checksum-mismatched one is an error.
fn read_snapshot(dir: &Path) -> Result<Option<Snapshot>> {
    let path = dir.join(SNAPSHOT_FILE);
    let mut file = match OpenOptions::new().read(true).open(&path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(io_error("open snapshot", e)),
    };
    file.lock_shared()
        .map_err(|e| io_error("share-lock snapshot", e))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|e| io_error("seek snapshot", e))?;
    let bytes = read_bounded(&mut file, MAX_SNAPSHOT_BYTES, "snapshot")?;
    let envelope: SnapshotEnvelope = serde_json::from_slice(&bytes)
        .map_err(|e| journal_error(format!("snapshot: malformed envelope: {e}")))?;
    if envelope.version != SNAPSHOT_VERSION {
        return Err(journal_error("snapshot: unsupported envelope version"));
    }
    let expected = decode_hex_32(&envelope.integrity, "snapshot envelope integrity")?;
    if !hash_eq(&envelope_integrity(&envelope), &expected) {
        return Err(journal_error("snapshot: envelope metadata integrity mismatch"));
    }
    let payload = hex::decode(envelope.payload.as_bytes())
        .map_err(|e| journal_error(format!("snapshot: payload is not hexadecimal: {e}")))?;
    let payload_len = u64::try_from(payload.len())
        .map_err(|_| journal_error("snapshot payload length exceeds u64"))?;
    if payload_len > MAX_RECORD_BYTES {
        return Err(journal_error("snapshot payload exceeds the frame bound"));
    }
    let checksum = decode_hex_32(&envelope.checksum, "snapshot checksum")?;
    if !hash_eq(&digest(&payload), &checksum) {
        return Err(journal_error("snapshot checksum mismatch"));
    }
    let state_hash = decode_hex_32(&envelope.state_hash, "snapshot state hash")?;
    let view = decode_view(&payload, "snapshot")?;
    let recovered = state_digest(&encode_view(&view)?);
    if !hash_eq(&recovered, &state_hash) {
        return Err(journal_error("snapshot state hash mismatch"));
    }
    let last_applied_ms = view.last_applied_ms;
    let journal_frames = envelope.journal_frames;
    Ok(Some(Snapshot {
        view,
        state_hash,
        last_applied_ms,
        journal_frames,
    }))
}

/// Read a 64-hex-character identity or checkpoint hash. Absent is optional;
/// present but malformed is an error.
fn read_hex_32(path: &Path, label: &str) -> Result<Option<[u8; 32]>> {
    let mut file = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(io_error(&format!("read {label}"), e)),
    };
    file.lock_shared()
        .map_err(|e| io_error(&format!("share-lock {label}"), e))?;
    file.seek(SeekFrom::Start(0))
        .map_err(|e| io_error(&format!("seek {label}"), e))?;
    let bytes = read_bounded(&mut file, MAX_HASH_FILE_BYTES, label)?;
    let text = String::from_utf8(bytes)
        .map_err(|e| journal_error(format!("{label} is not UTF-8: {e}")))?;
    decode_hex_32(text.trim(), label).map(Some)
}

fn read_node_id(dir: &Path) -> Result<Option<NodeId>> {
    match read_hex_32(&dir.join(NODE_ID_FILE), NODE_ID_FILE)? {
        Some(id) => Ok(Some(NodeId(id))),
        None => Ok(None),
    }
}

/// Bind a journal directory to exactly one derived node identity. The first
/// boot publishes the id through an exclusive create; later boots must present
/// the same id, because every CRDT dot and HLC stamp is keyed by it. A stale or
/// foreign `node.id` is an error instead of a phantom second identity.
pub fn bind_identity(dir: &Path, id: &NodeId) -> Result<()> {
    std::fs::create_dir_all(dir).map_err(|e| io_error("create journal directory", e))?;
    let path = dir.join(NODE_ID_FILE);
    let mut file = match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            let stored = read_node_id(dir)?.ok_or_else(|| {
                journal_error("node.id disappeared while binding the node identity")
            })?;
            if stored != *id {
                return Err(journal_error(
                    "node.id does not match the identity derived from the cluster secret",
                ));
            }
            return Ok(());
        }
        Err(e) => return Err(io_error("create node.id", e)),
    };
    file.write_all(hex::encode(id.as_bytes()).as_bytes())
        .map_err(|e| io_error("write node.id", e))?;
    file.sync_all().map_err(|e| io_error("sync node.id", e))?;
    drop(file);
    sync_dir(dir)
}

/// Mark that at least one durable frame or snapshot was published. A directory
/// with this marker but no surviving snapshot or frame has lost durable state;
/// recovery must refuse to rebuild it as a fresh empty view.
fn mark_committed(dir: &Path) -> Result<()> {
    let path = dir.join(COMMITTED_FILE);
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(file) => {
            drop(file);
            sync_dir(dir)
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(io_error("mark durable commit", e)),
    }
}

fn has_committed(dir: &Path) -> Result<bool> {
    match std::fs::metadata(dir.join(COMMITTED_FILE)) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
        Err(e) => Err(io_error("read durable commit marker", e)),
    }
}

/// Sync the containing directory so a rename or file creation is durable.
fn sync_dir(dir: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        let file = OpenOptions::new()
            .read(true)
            .open(dir)
            .map_err(|e| io_error("open directory for sync", e))?;
        file.sync_all()
            .map_err(|e| io_error("sync directory", e))?;
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
    }
    Ok(())
}

#[must_use]
fn snapshot_temp_path(dir: &Path, tag: &[u8; 32]) -> PathBuf {
    let sequence = SNAPSHOT_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    dir.join(format!(
        "snapshot.json.tmp-{}-{}-{}",
        std::process::id(),
        sequence,
        hex::encode(&tag[..8])
    ))
}

/// Publish bytes through a unique temporary file and an atomic rename.
fn write_snapshot_atomic(dir: &Path, bytes: &[u8], tag: &[u8; 32]) -> Result<()> {
    let tmp = snapshot_temp_path(dir, tag);
    let _guard = TempPath { path: tmp.clone() };
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
    {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            std::fs::remove_file(&tmp)
                .map_err(|e| io_error("remove stale snapshot temp", e))?;
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&tmp)
                .map_err(|e| io_error("open temporary snapshot", e))?
        }
        Err(e) => return Err(io_error("open temporary snapshot", e)),
    };
    file.write_all(bytes)
        .map_err(|e| io_error("write temporary snapshot", e))?;
    file.sync_all()
        .map_err(|e| io_error("sync temporary snapshot", e))?;
    drop(file);
    std::fs::rename(&tmp, &dir.join(SNAPSHOT_FILE))
        .map_err(|e| io_error("publish snapshot", e))?;
    sync_dir(dir)?;
    Ok(())
}

impl Journal {
    /// Open (creating when necessary) the directory and journal file.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir).map_err(|e| io_error("create journal directory", e))?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join(JOURNAL_FILE))
            .map_err(|e| io_error("create journal file", e))?;
        file.sync_all()
            .map_err(|e| io_error("sync journal file", e))?;
        drop(file);
        sync_dir(&dir)?;
        Ok(Self { dir })
    }

    fn journal_path(&self) -> PathBuf {
        self.dir.join(JOURNAL_FILE)
    }

    /// Append one checksummed frame and return its zero-based index.
    ///
    /// The record must be a canonical serialized [`View`]; empty or malformed
    /// payloads are rejected before any IO. An incomplete trailing frame is
    /// dropped under the exclusive lock. A complete frame with a bad checksum
    /// is refused, requiring an explicit `truncate_corrupt_tail` call. No
    /// valid frame is ever removed.
    pub fn append(&self, record: &[u8]) -> Result<u64> {
        if record.is_empty() {
            return Err(journal_error("journal record is empty"));
        }
        let _view = decode_view(record, "append record")?;
        let length = u64::try_from(record.len())
            .map_err(|_| journal_error("record length exceeds u64"))?;
        if length > MAX_RECORD_BYTES {
            return Err(journal_error("record exceeds the 64 MiB frame bound"));
        }
        let mut frame = Vec::with_capacity(HEADER_LEN.saturating_add(record.len()));
        frame.extend_from_slice(&length.to_be_bytes());
        frame.extend_from_slice(&digest(record));
        frame.extend_from_slice(record);

        std::fs::create_dir_all(&self.dir)
            .map_err(|e| io_error("create journal directory", e))?;
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(self.journal_path())
            .map_err(|e| io_error("open journal for append", e))?;
        file.lock()
            .map_err(|e| io_error("lock journal for append", e))?;
        file.seek(SeekFrom::Start(0))
            .map_err(|e| io_error("seek journal for append", e))?;
        let bytes = read_bounded(&mut file, MAX_JOURNAL_BYTES, "journal for append")?;
        let scan = scan_bytes(&bytes, FrameRetention::Counts)?;
        if scan.corrupt_frames > 0 || scan.invalid_frames > 0 {
            return Err(journal_error(
                "damaged journal frames detected; call truncate_corrupt_tail before append",
            ));
        }
        if scan.tail_partial {
            file.set_len(scan.tail_start)
                .map_err(|e| io_error("drop incomplete journal tail", e))?;
            file.seek(SeekFrom::End(0))
                .map_err(|e| io_error("seek journal end", e))?;
        }
        let index = scan.valid_frames;
        file.write_all(&frame)
            .map_err(|e| io_error("append journal frame", e))?;
        file.sync_all()
            .map_err(|e| io_error("sync journal frame", e))?;
        mark_committed(&self.dir)?;
        Ok(index)
    }

    /// Atomically write a checksummed, versioned [`View`] to `snapshot.json`
    /// while holding the journal's exclusive advisory lock.
    pub fn snapshot(&self, view: &View) -> Result<()> {
        let canonical = encode_view(view)?;
        let canonical_len = u64::try_from(canonical.len())
            .map_err(|_| journal_error("snapshot payload length exceeds u64"))?;
        if canonical_len > MAX_RECORD_BYTES {
            return Err(journal_error("snapshot payload exceeds the frame bound"));
        }
        let checksum = digest(&canonical);
        let state_hash = state_digest(&canonical);
        std::fs::create_dir_all(&self.dir)
            .map_err(|e| io_error("create snapshot directory", e))?;
        let mut lock_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.journal_path())
            .map_err(|e| io_error("open journal for snapshot lock", e))?;
        lock_file
            .lock()
            .map_err(|e| io_error("lock journal for snapshot", e))?;
        lock_file
            .seek(SeekFrom::Start(0))
            .map_err(|e| io_error("seek journal for snapshot", e))?;
        let journal_bytes =
            read_bounded(&mut lock_file, MAX_JOURNAL_BYTES, "journal for snapshot")?;
        let scan = scan_bytes(&journal_bytes, FrameRetention::Counts)?;
        if scan.corrupt_frames > 0 || scan.invalid_frames > 0 {
            return Err(journal_error(
                "damaged journal frames detected; call truncate_corrupt_tail before snapshot",
            ));
        }
        if scan.tail_partial {
            lock_file
                .set_len(scan.tail_start)
                .map_err(|e| io_error("drop incomplete journal tail", e))?;
            lock_file
                .sync_all()
                .map_err(|e| io_error("sync journal after tail drop", e))?;
        }
        let mut envelope = SnapshotEnvelope {
            version: SNAPSHOT_VERSION,
            payload: hex::encode(&canonical),
            checksum: hex::encode(checksum),
            state_hash: hex::encode(state_hash),
            last_applied_ms: view.last_applied_ms,
            journal_frames: scan.valid_frames,
            journal_bytes: scan.tail_start,
            integrity: String::new(),
        };
        envelope.integrity = hex::encode(envelope_integrity(&envelope));
        let envelope_bytes = serde_json::to_vec(&envelope)
            .map_err(|e| journal_error(format!("encode snapshot envelope: {e}")))?;
        write_snapshot_atomic(&self.dir, &envelope_bytes, &state_hash)?;
        mark_committed(&self.dir)?;
        Ok(())
    }

    /// Recover the durable [`View`] across a restart. Any damaged frame is
    /// surfaced as an error instead of a silently stale or empty state.
    pub fn recover(&self) -> Result<View> {
        let report = build_report(&self.dir)?;
        if report.corrupt_tail_frames > 0 {
            return Err(journal_error(format!(
                "{} damaged journal frames detected; call truncate_corrupt_tail before recovery",
                report.corrupt_tail_frames
            )));
        }
        Ok(report.view)
    }

    /// Count the checksum-verified frames and verify snapshot integrity,
    /// tolerating a corrupt tail.
    pub fn verify(&self) -> Result<u64> {
        let _snapshot = read_snapshot(&self.dir)?;
        let bytes = read_journal(&self.journal_path())?;
        let scan = scan_bytes(&bytes, FrameRetention::Counts)?;
        if scan.corrupt_frames > 0 || scan.invalid_frames > 0 {
            return Err(journal_error(
                "damaged journal frames detected; call truncate_corrupt_tail before recovery",
            ));
        }
        Ok(scan.valid_frames)
    }

    /// Rewrite the journal with only checksum-verified, View-decodable frames,
    /// dropping torn and corrupt frames; report how many were dropped.
    pub fn truncate_corrupt_tail(&self) -> Result<usize> {
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.journal_path())
        {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(0),
            Err(e) => return Err(io_error("open journal for repair", e)),
        };
        file.lock()
            .map_err(|e| io_error("lock journal for repair", e))?;
        file.seek(SeekFrom::Start(0))
            .map_err(|e| io_error("seek journal for repair", e))?;
        let bytes = read_bounded(&mut file, MAX_JOURNAL_BYTES, "journal for repair")?;
        let scan = scan_bytes(&bytes, FrameRetention::Rebuild)?;
        let dropped = scan
            .corrupt_frames
            .saturating_add(scan.invalid_frames)
            .saturating_add(scan.tail_frames);
        if dropped == 0 {
            return Ok(0);
        }
        file.set_len(0)
            .map_err(|e| io_error("clear journal for repair", e))?;
        file.seek(SeekFrom::Start(0))
            .map_err(|e| io_error("seek journal for repair write", e))?;
        file.write_all(&scan.rebuilt)
            .map_err(|e| io_error("write repaired journal", e))?;
        file.sync_all()
            .map_err(|e| io_error("sync repaired journal", e))?;
        Ok(dropped)
    }
}

/// Replay the journal and snapshot into a complete recovery report without
/// creating or modifying any file.
fn build_report(dir: &Path) -> Result<RecoveryReport> {
    let scan = scan_bytes(
        &read_journal(&dir.join(JOURNAL_FILE))?,
        FrameRetention::TrailingFrames,
    )?;
    let mut invalid_view_frames = 0usize;
    let mut journal_candidate: Option<Candidate> = None;
    for (index, frame) in &scan.frames {
        if !hash_eq(&digest(&frame.record), &frame.checksum) {
            invalid_view_frames = invalid_view_frames.saturating_add(1);
            continue;
        }
        match decode_view(&frame.record, &format!("journal frame {index}")) {
            Ok(view) => {
                let last_applied_ms = view.last_applied_ms;
                journal_candidate = Some(Candidate {
                    view,
                    expected_hash: state_digest(&frame.record),
                    last_applied_ms,
                    sequence: scan.valid_frames,
                    snapshot_frames: 0,
                    has_durable_hash: true,
                });
            }
            Err(_error) => {
                invalid_view_frames = invalid_view_frames.saturating_add(1);
            }
        }
    }
    let snapshot_candidate = read_snapshot(dir)?.map(|snapshot| {
        let last_applied_ms = snapshot.last_applied_ms;
        Candidate {
            view: snapshot.view,
            expected_hash: snapshot.state_hash,
            last_applied_ms,
            sequence: snapshot.journal_frames.saturating_add(1),
            snapshot_frames: snapshot.journal_frames,
            has_durable_hash: true,
        }
    });
    let chosen = match (snapshot_candidate, journal_candidate) {
        (Some(snapshot), Some(journal)) => {
            if journal.sequence > snapshot.snapshot_frames
                || journal.last_applied_ms > snapshot.last_applied_ms
            {
                journal
            } else {
                snapshot
            }
        }
        (Some(snapshot), None) => snapshot,
        (None, Some(journal)) => journal,
        (None, None) => {
            let self_id = read_node_id(dir)?.ok_or_else(|| {
                journal_error("node.id is missing and no snapshot or frame supplies an identity")
            })?;
            if has_committed(dir)? {
                return Err(journal_error(
                    "durable commit marker exists but no snapshot or frame survives; restore the journal before recovery",
                ));
            }
            Candidate {
                view: View::new(self_id, 0),
                expected_hash: [0u8; HASH_LEN],
                last_applied_ms: 0,
                sequence: 0,
                snapshot_frames: 0,
                has_durable_hash: false,
            }
        }
    };
    let recovered_hash = state_digest(&encode_view(&chosen.view)?);
    if chosen.has_durable_hash && !hash_eq(&recovered_hash, &chosen.expected_hash) {
        return Err(journal_error(
            "recovered state digest does not match the expected hash from the last frame or snapshot",
        ));
    }
    let corrupt_tail_frames = scan
        .tail_frames
        .saturating_add(scan.corrupt_frames)
        .saturating_add(scan.invalid_frames)
        .saturating_add(invalid_view_frames);
    Ok(RecoveryReport {
        view: chosen.view,
        records_replayed: scan.valid_frames,
        corrupt_tail_frames,
        state_hash: recovered_hash,
    })
}

/// Recover from an explicit directory without constructing a [`Journal`] or
/// performing any write IO.
pub fn recover_with_report(dir: &Path) -> Result<RecoveryReport> {
    build_report(dir)
}
