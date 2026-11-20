// SPDX-License-Identifier: CC0-1.0

//! Total, parameterized types. Every constructor validates; nothing partial.

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::fmt;
use std::str::FromStr;

/// A node identity: the raw 32-byte Ed25519 public key of its transport endpoint.
/// Authentication and encryption of every inter-node byte are bound to it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub [u8; 32]);

impl NodeId {
    #[must_use]
    pub fn from_public_key_bytes(bytes: &[u8; 32]) -> Self {
        Self(*bytes)
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    #[must_use]
    pub fn short(&self) -> String {
        hex::encode(&self.0[..4])
    }
    /// Deterministic, parameterised owner score. No ambient randomness:
    /// the same `(node, key)` pair always yields the same score everywhere.
    #[must_use]
    pub fn rendezvous_score(&self, key: &[u8]) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"aws0/rendezvous/v1\0");
        h.update(self.0);
        h.update((key.len() as u64).to_be_bytes());
        h.update(key);
        let out = h.finalize();
        let mut a = [0u8; 32];
        a.copy_from_slice(&out);
        a
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}
impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.short())
    }
}

/// Hex-string wire form. Serde JSON map keys must be strings; `NodeId` is used
/// as a `BTreeMap` key in the state counters and version vectors, so a byte array
/// would make the whole state (and its digest) unserializable. The encoding is
/// total and canonical: 64 lowercase hex chars, no prefix.

impl Serialize for NodeId {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&hex::encode(self.0))
    }
}
impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        let bytes = hex::decode(text.as_bytes()).map_err(|e| DeError::custom(e.to_string()))?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| DeError::custom("NodeId must be 32 bytes"))?;
        Ok(Self(arr))
    }
}

/// A 12-digit AWS account id. State is partitioned by `(account, region)`, so
/// two tenants can never observe each other's resources.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct AccountId(String);
impl AccountId {
    pub fn new(raw: &str) -> Result<Self> {
        if raw.len() == 12 && raw.bytes().all(|b| b.is_ascii_digit()) {
            Ok(Self(raw.to_owned()))
        } else {
            Err(Error::InvalidAccount(raw.to_owned()))
        }
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for AccountId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}
impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl fmt::Debug for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccountId({})", self.0)
    }
}

/// An AWS region name. Kept as data, never interpolated into a command or URL.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Region(String);
impl Region {
    pub fn new(raw: &str) -> Result<Self> {
        let ok = !raw.is_empty()
            && raw.len() <= 64
            && raw
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
        if ok {
            Ok(Self(raw.to_owned()))
        } else {
            Err(Error::InvalidRegion(raw.to_owned()))
        }
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for Region {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}
impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Region({})", self.0)
    }
}

/// A workload name: `[a-zA-Z0-9_-]`, 1..=128 bytes.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct WorkloadId(String);
impl WorkloadId {
    pub fn new(raw: &str) -> Result<Self> {
        let ok = !raw.is_empty()
            && raw.len() <= 128
            && raw
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
        if ok {
            Ok(Self(raw.to_owned()))
        } else {
            Err(Error::InvalidWorkloadId(raw.to_owned()))
        }
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl fmt::Debug for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorkloadId({})", self.0)
    }
}

/// All three wire forms decode through their validating constructor. Without
/// this, a hostile state exchange or HTTP payload could carry a 12-byte account id
/// with a NUL, a region with a fake length prefix, or a workload id that
/// escapes the routing-key partition boundary.
impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Self::new(&raw).map_err(|e| DeError::custom(e.to_string()))
    }
}
impl<'de> Deserialize<'de> for Region {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Self::new(&raw).map_err(|e| DeError::custom(e.to_string()))
    }
}
impl<'de> Deserialize<'de> for WorkloadId {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Self::new(&raw).map_err(|e| DeError::custom(e.to_string()))
    }
}

/// Disjoint state partition key. ` for all i != j: S_i intersect S_j= {} ` is enforced by using this
/// key as the sole index into every replicated map.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionKey {
    pub account: AccountId,
    pub region: Region,
}
impl PartitionKey {
    #[must_use]
    pub fn new(account: AccountId, region: Region) -> Self {
        Self { account, region }
    }
    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(12 + 1 + self.region.as_str().len());
        v.extend_from_slice(self.account.as_str().as_bytes());
        v.push(0);
        v.extend_from_slice(self.region.as_str().as_bytes());
        v
    }
    #[must_use]
    pub fn wire(&self) -> String {
        format!("{}:{}", self.account.as_str(), self.region.as_str())
    }
    pub fn from_wire(text: &str) -> Result<Self> {
        let (account, region) = text
            .split_once(':')
            .ok_or_else(|| Error::Protocol("PartitionKey must be `account:region`".to_owned()))?;
        Ok(Self::new(AccountId::new(account)?, Region::new(region)?))
    }
}
impl Serialize for PartitionKey {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(&self.wire())
    }
}
impl<'de> Deserialize<'de> for PartitionKey {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        Self::from_wire(&text).map_err(|e| DeError::custom(e.to_string()))
    }
}
impl fmt::Debug for PartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PartitionKey({})", self.wire())
    }
}

/// A hybrid logical clock: total order across nodes without trusting wall
/// clocks alone. `(wall_ms, counter, node)` is a lexicographic total order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub struct Hlc {
    pub wall_ms: u64,
    pub counter: u32,
    pub node: NodeId,
}
impl Hlc {
    #[must_use]
    pub fn tick(previous: Option<Hlc>, now_ms: u64, node: NodeId) -> Self {
        match previous {
            None => Self { wall_ms: now_ms, counter: 0, node },
            Some(p) if now_ms > p.wall_ms => Self { wall_ms: now_ms, counter: 0, node },
            Some(p) => Self {
                wall_ms: p.wall_ms,
                counter: p.counter.saturating_add(1),
                node,
            },
        }
    }
    /// Observe a remote timestamp: causal order never goes backwards.
    #[must_use]
    pub fn observe(previous: Option<Hlc>, remote: Hlc, now_ms: u64, node: NodeId) -> Self {
        let merged_wall = core::cmp::max(now_ms, remote.wall_ms);
        let counter = match previous {
            Some(p) if p.wall_ms == merged_wall => {
                core::cmp::max(p.counter, remote.counter).saturating_add(1)
            }
            _ => remote.counter.saturating_add(1),
        };
        Self { wall_ms: merged_wall, counter, node }
    }
}

/// Lamport epoch, bumped on every membership change; placements carry the epoch
/// that produced them so stale reconciliation cannot resurrect old state.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub struct Epoch(pub u64);
impl Epoch {
    #[must_use]
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Zero-trust capability: an opaque, time-bounded bearer token. The signature
/// is an HMAC-SHA256 over the canonical encoding with a cluster secret supplied
/// at boot; secrets are never literals in this crate.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Capability {
    pub subject: NodeId,
    pub scope: Scope,
    pub expires_at_ms: u64,
    pub nonce: [u8; 16],
    pub mac: [u8; 32],
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Scope {
    ClusterAdmin,
    Tenant(AccountId),
    Region(Region),
    Workload(WorkloadId),
}
impl Capability {
    /// Canonical MAC preimage. The scope is encoded directly from its
    /// validated identifier bytes, so this constructor is total: there is no
    /// fallible serialization whose failure could substitute an empty scope.
    #[must_use]
    pub fn signing_bytes(subject: &NodeId, scope: &Scope, expires_at_ms: u64, nonce: &[u8; 16]) -> Vec<u8> {
        let mut v = b"aws0/capability/v1\0".to_vec();
        v.extend_from_slice(subject.as_bytes());
        v.extend_from_slice(&expires_at_ms.to_be_bytes());
        v.extend_from_slice(nonce);
        let sc = scope_encoding(scope);
        v.extend_from_slice(&(sc.len() as u64).to_be_bytes());
        v.extend_from_slice(&sc);
        v
    }
    #[must_use]
    pub fn issue(subject: NodeId, scope: Scope, expires_at_ms: u64, nonce: [u8; 16], secret: &[u8]) -> Self {
        // HMAC-SHA256 with domain-separated, length-prefixed input. Constant-time
        // comparison lives in `verify`.
        let msg = Self::signing_bytes(&subject, &scope, expires_at_ms, &nonce);
        let mac = keyed_mac(secret, &msg);
        Self { subject, scope, expires_at_ms, nonce, mac }
    }
    pub fn verify(&self, secret: &[u8], now_ms: u64) -> Result<()> {
        if now_ms >= self.expires_at_ms {
            return Err(Error::CapabilityExpired);
        }
        let msg = Self::signing_bytes(&self.subject, &self.scope, self.expires_at_ms, &self.nonce);
        let recomputed = keyed_mac(secret, &msg);
        // Constant-time: accumulate the OR of all byte differences, no early exit,
        // no secret-dependent branch.
        let mut diff = 0u8;
        for (a, b) in recomputed.iter().zip(self.mac.iter()) {
            diff |= a ^ b;
        }
        if diff == 0 {
            Ok(())
        } else {
            Err(Error::CapabilityDenied)
        }
    }
}

/// Byte-for-byte the JSON wire form of `Scope`, built without a fallible
/// serializer. `Scope` is externally tagged and each inner newtype serializes
/// as its string, so existing capability MACs keep their preimage.
#[must_use]
fn scope_encoding(scope: &Scope) -> Vec<u8> {
    let mut out = Vec::new();
    match scope {
        Scope::ClusterAdmin => {
            out.push(b'"');
            out.extend_from_slice(b"ClusterAdmin");
            out.push(b'"');
        }
        Scope::Tenant(account) => push_json_field(&mut out, b"Tenant", account.as_str()),
        Scope::Region(region) => push_json_field(&mut out, b"Region", region.as_str()),
        Scope::Workload(workload) => push_json_field(&mut out, b"Workload", workload.as_str()),
    }
    out
}

fn push_json_field(out: &mut Vec<u8>, tag: &[u8], value: &str) {
    out.push(b'{');
    out.push(b'"');
    out.extend_from_slice(tag);
    out.push(b'"');
    out.push(b':');
    push_json_string(out, value);
    out.push(b'}');
}

/// Escapes exactly the bytes `serde_json` escapes in a string; every other
/// byte, including UTF-8 continuation bytes, is copied verbatim.
fn push_json_string(out: &mut Vec<u8>, text: &str) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    const BACKSLASH: u8 = 0x5c;
    out.push(b'"');
    for byte in text.bytes() {
        match byte {
            0x22 => {
                out.push(BACKSLASH);
                out.push(b'"');
            }
            BACKSLASH => {
                out.push(BACKSLASH);
                out.push(BACKSLASH);
            }
            0x08 => {
                out.push(BACKSLASH);
                out.push(b'b');
            }
            0x09 => {
                out.push(BACKSLASH);
                out.push(b't');
            }
            0x0a => {
                out.push(BACKSLASH);
                out.push(b'n');
            }
            0x0c => {
                out.push(BACKSLASH);
                out.push(b'f');
            }
            0x0d => {
                out.push(BACKSLASH);
                out.push(b'r');
            }
            0x00..=0x1f => {
                out.push(BACKSLASH);
                out.push(b'u');
                out.push(b'0');
                out.push(b'0');
                out.push(HEX[usize::from(byte >> 4)]);
                out.push(HEX[usize::from(byte & 0x0f)]);
            }
            _ => out.push(byte),
        }
    }
    out.push(b'"');
}

#[must_use]
pub fn keyed_mac(secret: &[u8], msg: &[u8]) -> [u8; 32] {
    // HMAC-SHA256 (RFC 2104), block size 64, implemented explicitly so the
    // crate has exactly one crypto dependency (`sha2`).
    const BLOCK: usize = 64;
    let mut key = [0u8; BLOCK];
    if secret.len() > BLOCK {
        let d = Sha256::digest(secret);
        key[..32].copy_from_slice(&d);
    } else {
        key[..secret.len()].copy_from_slice(secret);
    }
    let mut inner = Sha256::new();
    let mut outer = Sha256::new();
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for i in 0..BLOCK {
        ipad[i] ^= key[i];
        opad[i] ^= key[i];
    }
    inner.update(ipad);
    inner.update((msg.len() as u64).to_be_bytes());
    inner.update(msg);
    outer.update(opad);
    outer.update(inner.finalize());
    let out = outer.finalize();
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    a
}

/// An OCI workload. `image` and `command` are data passed as distinct argv
/// entries to laboon; they are never concatenated into a shell string.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WorkloadSpec {
    pub id: WorkloadId,
    pub image: String,
    pub command: Vec<String>,
    /// Guest environment. Keys are 1..=128 ASCII alphanumeric/underscore bytes,
    /// values are at most 8192 NUL-free bytes, and the vector holds at most 64
    /// distinct keys. `validate` enforces every bound before any child process
    /// is launched.
    pub env: Vec<(String, String)>,
    /// TCP ports to publish. Zero is not a port: `validate` rejects it, so a
    /// workload with no listener leaves this vector empty.
    pub ports: Vec<u16>,
    pub memory_mb: u64,
    pub timeout_ms: u64,
    /// Hosts a workload may reach. Empty means loopback only (laboon's default):
    /// the runtime must leave that default posture in place and must not turn an
    /// empty list into `-no-net`, which removes INET sockets entirely.
    pub egress_allowlist: Vec<String>,
    pub partition: PartitionKey,
    pub replicas: usize,
}
impl WorkloadSpec {
    pub fn validate(&self) -> Result<()> {
        if self.image.is_empty() || self.image.len() > 512 || self.image.contains('\0') {
            return Err(Error::InvalidImage(self.image.clone()));
        }
        if self.command.len() > 64 || self.command.iter().any(|s| s.contains('\0')) {
            return Err(Error::InvalidCommand);
        }
        // `Protocol(String)` is the most specific existing malformed-input
        // variant; no new `Error` variant is needed for an invalid env entry.
        if self.env.len() > 64 {
            return Err(Error::Protocol("workload env has more than 64 entries".to_owned()));
        }
        let mut seen_keys = std::collections::BTreeSet::new();
        for (key, value) in &self.env {
            let key_ok = !key.is_empty()
                && key.len() <= 128
                && !key.contains('\0')
                && !key.contains('=')
                && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
            if !key_ok {
                return Err(Error::Protocol(
                    "workload env key must be 1..=128 ASCII alphanumeric/underscore".to_owned(),
                ));
            }
            if value.contains('\0') || value.len() > 8192 {
                return Err(Error::Protocol(
                    "workload env value must be NUL-free and at most 8192 bytes".to_owned(),
                ));
            }
            if !seen_keys.insert(key.as_str()) {
                return Err(Error::Protocol("workload env has duplicate keys".to_owned()));
            }
        }
        if self.replicas == 0 || self.replicas > 16 {
            return Err(Error::InvalidReplicas(self.replicas));
        }
        if self.memory_mb == 0 || self.memory_mb > 65_536 {
            return Err(Error::InvalidMemory(self.memory_mb));
        }
        if self.timeout_ms == 0 || self.timeout_ms > 3_600_000 {
            return Err(Error::InvalidTimeout(self.timeout_ms));
        }
        if self.ports.len() > 16 || self.ports.iter().any(|p| *p == 0) {
            return Err(Error::InvalidPorts);
        }
        if self.egress_allowlist.iter().any(|s| s.contains('\0') || s.is_empty()) {
            return Err(Error::InvalidEgress);
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum WorkloadStatus {
    Pending,
    Scheduled,
    Running,
    Failed(String),
    Stopped,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Placement {
    pub workload: WorkloadId,
    pub nodes: Vec<NodeId>,
    pub epoch: Epoch,
    pub partition: PartitionKey,
    pub status: WorkloadStatus,
}

/// The single IO algebra. Pure `step` functions return these; only `net`'s
/// executor performs them. No module performs hidden IO.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Effect {
    /// Append a durable record (journal + snapshot bookkeeping).
    Persist(Vec<u8>),
    /// Disseminate a state delta to instances over internal state exchange.
    Broadcast(Vec<u8>),
    /// Start a workload through the runtime supervisor.
    StartWorkload(WorkloadSpec, NodeId),
    /// Stop a workload on a node.
    StopWorkload(WorkloadId, NodeId),
    /// Route a signed API request to the owning node.
    Forward { target: NodeId, request: Vec<u8> },
    /// Record an audit event with `(id, ts, hash)`.
    Audit { id: String, ts_ms: u64, hash: [u8; 32] },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidAccount(String),
    InvalidRegion(String),
    InvalidWorkloadId(String),
    InvalidImage(String),
    InvalidCommand,
    InvalidReplicas(usize),
    InvalidMemory(u64),
    InvalidTimeout(u64),
    InvalidPorts,
    InvalidEgress,
    CapabilityExpired,
    CapabilityDenied,
    NoQuorum { have: usize, need: usize },
    NoCapacity { workload: String },
    UnknownWorkload(String),
    UnknownNode(String),
    RegionUnavailable(String),
    Journal(String),
    Transport(String),
    Runtime(String),
    Protocol(String),
    CycleDetected,
    Replay(String),
    Timeout(String),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Monotonic millisecond clock read once per transition; `now_ms` is always an
/// explicit argument, never ambient, so transitions are deterministic and
/// replayable in the witness.
#[must_use]
pub fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

/// Unkeyed SHA-256. It detects accidental corruption only: any writer can
/// recompute this digest, so it is not an authenticity proof. Frames that must
/// resist a forging writer need a keyed MAC (`keyed_mac`) over the same bytes.
#[must_use]
pub fn digest(bytes: &[u8]) -> [u8; 32] {
    let out = Sha256::digest(bytes);
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    a
}
