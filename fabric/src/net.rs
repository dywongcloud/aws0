// SPDX-License-Identifier: CC0-1.0

//! iroh transport. Every inter-node message travels over QUIC authenticated by
//! the endpoint's Ed25519 key; a `NodeId` IS that public key, so a peer cannot
//! impersonate another node. `Effect` is interpreted only here: `Forward` opens
//! a real QUIC stream to the resolved owner, `Audit` appends a checksummed
//! durable record, and neither is ever reported as handled on failure.

use crate::persistence::Journal;
use crate::runtime::{WorkloadChild, WorkloadSupervisor};
use crate::smartops::{SmartOp, SmartRequest, SmartReply, ALPN};
use crate::state::View;
use crate::types::{
    digest, Capability, Effect, Error, NodeId, Result, Scope, WorkloadId,
};
use iroh::endpoint::{presets, Connection};
use iroh::protocol::{AcceptError, ProtocolHandler, Router};
use iroh::{Endpoint, EndpointAddr, RelayMode, SecretKey, Watcher};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A single smartops frame: 4-byte big-endian length + JSON payload. Bounded so
/// a malicious peer cannot force an unbounded allocation (p99-bounded graceful).
pub const MAX_FRAME: usize = 1 << 20;

/// Audit events are durable records, not log lines. They live in their own
/// append-only file beside the journal, so every journal frame remains exactly
/// one serialized `View` and restart recovery never has to interpret a foreign
/// record. Each audit frame carries its own SHA-256 checksum: a torn or
/// tampered event is detectable without trusting the rest of the file.
const AUDIT_FILE: &str = "aws0.audit";
const AUDIT_HEADER: usize = 4 + 32;

#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub secret: SecretKey,
    /// `true` for the offline local witness: relay disabled, one loopback UDP
    /// socket. `false` for production: n0 relay + DNS discovery + NAT traversal.
    pub loopback: bool,
    pub cluster_secret: Arc<Vec<u8>>,
}

/// The shared, explicitly-locked runtime state behind the protocol handler.
/// The mutex is the declared IO boundary; pure modules never see it.
#[derive(Debug)]
pub struct Shared {
    pub view: Mutex<View>,
    pub journal: Mutex<Journal>,
    pub supervisor: Mutex<WorkloadSupervisor>,
    pub cluster_secret: Arc<Vec<u8>>,
    pub peers: Mutex<Vec<EndpointAddr>>,
    /// Live guest processes owned by this node. Every `StartWorkload` child is
    /// retained here and reaped on `StopWorkload` or shutdown -- no zombie, no
    /// leaked handle, no orphaned sandbox (affine resource discipline).
    pub children: Mutex<Vec<(WorkloadId, NodeId, WorkloadChild)>>,
}

/// The accepted ALPN handler. It verifies the capability before touching state,
/// applies the operation as a pure transition, persists the result, and returns
/// a reply. No hidden mutation: all mutation is behind `Shared`'s mutexes.
#[derive(Debug, Clone)]
pub struct SmartHandler {
    pub shared: Arc<Shared>,
}

impl ProtocolHandler for SmartHandler {
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        let (mut send, mut recv) = connection.accept_bi().await.map_err(AcceptError::from_err)?;
        let framed = recv
            .read_to_end(MAX_FRAME)
            .await
            .map_err(AcceptError::from_err)?;
        let reply = match decode_frame(&framed) {
            Ok(request) => {
                let mut view = self.shared.view.lock().await;
                let at = crate::types::now_ms();
                let outcome = crate::smartops::step(
                    &view,
                    &request,
                    &self.shared.cluster_secret,
                    at,
                );
                let effects = outcome.effects.clone();
                *view = outcome.view;
                drop(view);
                let executed = execute(&self.shared, &effects).await;
                SmartReply {
                    idempotency: request.idempotency,
                    ok: outcome.reply.ok && executed.all_handled,
                    value: outcome.reply.value,
                    state_hash: {
                        let v = self.shared.view.lock().await;
                        v.digest()
                    },
                    effects: executed.handled.len(),
                    replayed: outcome.reply.replayed,
                    errors: executed.errors,
                }
            }
            Err(e) => SmartReply {
                idempotency: [0u8; 32],
                ok: false,
                value: serde_json::json!({ "error": e.to_string() }),
                state_hash: [0u8; 32],
                effects: 0,
                replayed: false,
                errors: vec![e.to_string()],
            },
        };
        let bytes = serde_json::to_vec(&reply).map_err(AcceptError::from_err)?;
        let frame = encode_frame(&bytes).map_err(AcceptError::from_err)?;
        send.write_all(&frame).await.map_err(AcceptError::from_err)?;
        send.finish().map_err(AcceptError::from_err)?;
        connection.closed().await;
        Ok(())
    }
}

#[must_use]
pub fn encode_frame(payload: &[u8]) -> Result<Vec<u8>> {
    if payload.len() > MAX_FRAME {
        return Err(Error::Protocol("frame exceeds 1 MiB".to_owned()));
    }
    let mut out = Vec::with_capacity(payload.len() + 4);
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(payload);
    Ok(out)
}

pub fn decode_frame(frame: &[u8]) -> Result<SmartRequest> {
    if frame.len() < 5 {
        return Err(Error::Protocol("frame shorter than header".to_owned()));
    }
    let mut len = [0u8; 4];
    len.copy_from_slice(&frame[..4]);
    let declared = u32::from_be_bytes(len) as usize;
    if declared > MAX_FRAME {
        return Err(Error::Protocol("declared frame length exceeds 1 MiB".to_owned()));
    }
    let body = &frame[4..];
    if body.len() != declared {
        return Err(Error::Protocol(format!(
            "truncated frame: declared {declared}, received {}",
            body.len()
        )));
    }
    serde_json::from_slice(body).map_err(|e| Error::Protocol(e.to_string()))
}

/// One durable audit event. `hash` comes from the pure transition that emitted
/// the effect; the executor never invents or rewrites it.
#[derive(serde::Serialize, serde::Deserialize)]
struct AuditRecord {
    id: String,
    ts_ms: u64,
    hash: [u8; 32],
}

#[must_use]
fn encode_audit_frame(record: &AuditRecord) -> Result<Vec<u8>> {
    let body = serde_json::to_vec(record)
        .map_err(|e| Error::Protocol(format!("audit encode: {e}")))?;
    if body.len() > MAX_FRAME {
        return Err(Error::Protocol("audit record exceeds 1 MiB".to_owned()));
    }
    let mut out = Vec::with_capacity(AUDIT_HEADER.saturating_add(body.len()));
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    out.extend_from_slice(&digest(&body));
    out.extend_from_slice(&body);
    Ok(out)
}

fn append_audit_frame(dir: &Path, frame: &[u8]) -> Result<()> {
    std::fs::create_dir_all(dir)
        .map_err(|e| Error::Journal(format!("audit directory: {e}")))?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join(AUDIT_FILE))
        .map_err(|e| Error::Journal(format!("open audit log: {e}")))?;
    file.write_all(frame)
        .map_err(|e| Error::Journal(format!("write audit log: {e}")))?;
    file.sync_all()
        .map_err(|e| Error::Journal(format!("sync audit log: {e}")))?;
    Ok(())
}

/// Verify every independently checksummed audit frame. A torn or tampered frame
/// is an error, never a silently skipped event.
#[must_use]
pub fn verify_audit_log(dir: &Path) -> Result<u64> {
    let bytes = match std::fs::read(dir.join(AUDIT_FILE)) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(Error::Journal(format!("read audit log: {e}"))),
    };
    let mut offset = 0usize;
    let mut count = 0u64;
    while offset < bytes.len() {
        if bytes.len().saturating_sub(offset) < AUDIT_HEADER {
            return Err(Error::Journal("audit log has a torn frame header".to_owned()));
        }
        let mut len = [0u8; 4];
        len.copy_from_slice(&bytes[offset..offset + 4]);
        let declared = u32::from_be_bytes(len) as usize;
        if declared > MAX_FRAME {
            return Err(Error::Journal("audit record exceeds 1 MiB".to_owned()));
        }
        let frame_end = offset
            .saturating_add(AUDIT_HEADER)
            .saturating_add(declared);
        if frame_end > bytes.len() {
            return Err(Error::Journal("audit log has a torn frame body".to_owned()));
        }
        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&bytes[offset + 4..offset + AUDIT_HEADER]);
        let body = &bytes[offset + AUDIT_HEADER..frame_end];
        if digest(body) != checksum {
            return Err(Error::Journal("audit frame failed its checksum".to_owned()));
        }
        if serde_json::from_slice::<AuditRecord>(body).is_err() {
            return Err(Error::Journal("audit frame is not an audit record".to_owned()));
        }
        offset = frame_end;
        count = count.saturating_add(1);
    }
    Ok(count)
}

/// A journal frame is exactly one serialized `View`. Refuse empty or non-View
/// records at the executor boundary: appending them makes restart recovery fail
/// permanently with no recoverable signal.
fn validate_persist_record(bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() {
        return Err(Error::Protocol("persist record is empty".to_owned()));
    }
    serde_json::from_slice::<View>(bytes)
        .map(|_| ())
        .map_err(|e| Error::Protocol(format!("persist record is not a View: {e}")))
}

/// Resolve a `NodeId` to a usable `EndpointAddr`. `peers` is the boot-time
/// address table; the membership view is the fallback because each member
/// carries its own endpoint address. No address is ever fabricated.
async fn resolve_forward_addr(shared: &Arc<Shared>, target: &NodeId) -> Result<EndpointAddr> {
    {
        let peers = shared.peers.lock().await;
        if let Some(addr) = peers
            .iter()
            .find(|addr| addr.id.as_bytes() == target.as_bytes())
            .cloned()
        {
            return Ok(addr);
        }
    }
    let mut bad_address: Option<String> = None;
    {
        let view = shared.view.lock().await;
        for member in view.state.live_members() {
            if member.node != *target {
                continue;
            }
            match serde_json::from_str::<EndpointAddr>(&member.endpoint_addr) {
                Ok(addr) => {
                    if addr.id.as_bytes() != target.as_bytes() {
                        return Err(Error::Transport(format!(
                            "forward address for {} names a different endpoint",
                            target.short()
                        )));
                    }
                    return Ok(addr);
                }
                Err(e) => bad_address = Some(e.to_string()),
            }
        }
    }
    Err(Error::Transport(match bad_address {
        Some(e) => format!(
            "no usable endpoint address for {}: {e}",
            target.short()
        ),
        None => format!(
            "no endpoint address for {} in peers or membership",
            target.short()
        ),
    }))
}

/// A real point-to-point forward: the framed smartops request travels over iroh
/// QUIC to the resolved target, and success is recorded only after the remote
/// node reports `ok`. Every failure is an error in the report, never a silent
/// success, and a missing address is reported as `Error::Transport`.
async fn forward_request(
    shared: &Arc<Shared>,
    target: &NodeId,
    request: &[u8],
) -> Result<()> {
    if request.is_empty() {
        return Err(Error::Protocol("forward request is empty".to_owned()));
    }
    if request.len() > MAX_FRAME {
        return Err(Error::Protocol("forward request exceeds 1 MiB".to_owned()));
    }
    let addr = resolve_forward_addr(shared, target).await?;
    let response = send_delta(addr, request).await?;
    let reply_bytes = decode_reply_frame(&response)?;
    let reply: SmartReply = serde_json::from_slice(&reply_bytes)
        .map_err(|e| Error::Protocol(format!("forward reply decode: {e}")))?;
    if reply.ok {
        Ok(())
    } else {
        Err(Error::Transport(format!(
            "forward to {} was refused: {}",
            target.short(),
            reply.errors.join("; ")
        )))
    }
}

/// Result of interpreting a batch of effects. `all_handled` is false if any
/// effect failed; failures are returned as data, never silently dropped.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionReport {
    pub handled: Vec<String>,
    pub errors: Vec<String>,
    pub all_handled: bool,
}

pub async fn execute(shared: &Arc<Shared>, effects: &[Effect]) -> ExecutionReport {
    let mut handled = Vec::new();
    let mut errors = Vec::new();
    for effect in effects {
        match effect {
            Effect::Persist(bytes) => match validate_persist_record(bytes) {
                Ok(()) => {
                    let j = shared.journal.lock().await;
                    match j.append(bytes) {
                        Ok(seq) => handled.push(format!("persist:{seq}")),
                        Err(e) => errors.push(format!("persist: {e}")),
                    }
                }
                Err(e) => errors.push(format!("persist: {e}")),
            },
            Effect::Broadcast(_) => {
                // Anti-entropy: exchange the whole CRDT state with every peer
                // as a framed smartops `Converge` request. A failed peer is
                // recorded and skipped; a join is idempotent, so retrying a
                // delta can never double-apply it.
                let peers = shared.peers.lock().await.clone();
                let (self_id, state) = {
                    let view = shared.view.lock().await;
                    (view.self_id, view.state.clone())
                };
                // One idempotency key per broadcast. A constant key would make
                // the receiver replay the first Converge reply forever, so no
                // heartbeat or placement delta would ever reach a peer.
                let request = SmartRequest {
                    idempotency: idempotency_key("gossip", crate::types::now_ms()),
                    op: SmartOp::Converge(Box::new(state)),
                    capability: Capability::issue(
                        self_id,
                        Scope::ClusterAdmin,
                        u64::MAX,
                        [0u8; 16],
                        &shared.cluster_secret,
                    ),
                };
                let payload = match serde_json::to_vec(&request) {
                    Ok(p) => p,
                    Err(e) => {
                        errors.push(format!("broadcast encode: {e}"));
                        Vec::new()
                    }
                };
                if !payload.is_empty() {
                    for peer in peers {
                        match send_delta(peer, &payload).await {
                            Ok(response) => match decode_reply_frame(&response)
                                .and_then(|b| serde_json::from_slice::<SmartReply>(&b)
                                    .map_err(|e| Error::Protocol(e.to_string())),
                            ) {
                                Ok(reply) if reply.ok => handled.push("broadcast".to_owned()),
                                Ok(reply) => errors.push(format!("broadcast: {}", reply.errors.join("; "))),
                                Err(e) => errors.push(format!("broadcast decode: {e}")),
                            },
                            Err(e) => errors.push(format!("broadcast: {e}")),
                        }
                    }
                }
            }
            Effect::StartWorkload(spec, node) => {
                let child = {
                    let sup = shared.supervisor.lock().await;
                    sup.start(spec)
                };
                match child {
                    Ok(child) => {
                        shared.children.lock().await.push((spec.id.clone(), *node, child));
                        handled.push(format!("start:{}@{}", spec.id.as_str(), node.short()));
                    }
                    Err(e) => errors.push(format!("start: {e}")),
                }
            }
            Effect::StopWorkload(id, node) => {
                let mut target = None;
                {
                    let mut children = shared.children.lock().await;
                    let mut kept = Vec::with_capacity(children.len());
                    for entry in children.drain(..) {
                        if entry.0 == *id && entry.1 == *node {
                            target = Some(entry.2);
                        } else {
                            kept.push(entry);
                        }
                    }
                    *children = kept;
                }
                if let Some(mut child) = target {
                    let sup = shared.supervisor.lock().await;
                    match sup.stop(&mut child) {
                        Ok(()) => handled.push(format!("stop:{}@{}", id.as_str(), node.short())),
                        Err(e) => errors.push(format!("stop: {e}")),
                    }
                }
            }
            Effect::Forward { target, request } => {
                match forward_request(shared, target, request).await {
                    Ok(()) => handled.push(format!(
                        "forward:{}:{}",
                        target.short(),
                        request.len()
                    )),
                    Err(e) => errors.push(format!("forward: {e}")),
                }
            }
            Effect::Audit { id, ts_ms, hash } => {
                let frame = encode_audit_frame(&AuditRecord {
                    id: id.as_str().to_owned(),
                    ts_ms: *ts_ms,
                    hash: *hash,
                });
                match frame {
                    Err(e) => errors.push(format!("audit: {e}")),
                    Ok(frame) => {
                        // The journal lock is the declared IO boundary; holding it
                        // serialises the audit append against journal writes.
                        let j = shared.journal.lock().await;
                        match append_audit_frame(&j.dir, &frame) {
                            Ok(()) => handled.push(format!(
                                "audit:{id}:{ts_ms}:{}",
                                hex::encode(&hash[..4])
                            )),
                            Err(e) => errors.push(format!("audit: {e}")),
                        }
                    }
                }
            }
        }
    }
    ExecutionReport {
        all_handled: errors.is_empty(),
        handled,
        errors,
    }
}

/// A point-to-point smartops RPC to a peer. Live QUIC, no mocks.
pub async fn send_delta(
    peer: EndpointAddr,
    payload: &[u8],
) -> Result<Vec<u8>> {
    let endpoint = Endpoint::builder(presets::Minimal)
        .relay_mode(RelayMode::Disabled)
        .clear_ip_transports()
        .bind_addr("127.0.0.1:0")
        .map_err(|e| Error::Transport(e.to_string()))?
        .bind()
        .await
        .map_err(|e| Error::Transport(e.to_string()))?;
    let conn = endpoint
        .connect(peer, ALPN)
        .await
        .map_err(|e| Error::Transport(e.to_string()))?;
    let (mut send, mut recv) = conn
        .open_bi()
        .await
        .map_err(|e| Error::Transport(e.to_string()))?;
    let frame = encode_frame(payload)?;
    send.write_all(&frame)
        .await
        .map_err(|e| Error::Transport(e.to_string()))?;
    send.finish()
        .map_err(|e| Error::Transport(e.to_string()))?;
    let response = recv
        .read_to_end(MAX_FRAME)
        .await
        .map_err(|e| Error::Transport(e.to_string()))?;
    conn.close(0u32.into(), b"done");
    endpoint.close().await;
    Ok(response)
}

/// A running node: endpoint + router + shared state.
#[derive(Debug)]
pub struct Node {
    pub endpoint: Endpoint,
    pub router: Router,
    pub shared: Arc<Shared>,
    pub self_id: NodeId,
}

impl Node {
    pub async fn open(config: NodeConfig, shared: Arc<Shared>) -> Result<Self> {
        let mut builder = Endpoint::builder(presets::Minimal).secret_key(config.secret);
        if config.loopback {
            builder = builder
                .relay_mode(RelayMode::Disabled)
                .clear_ip_transports()
                .bind_addr("127.0.0.1:0")
                .map_err(|e| Error::Transport(e.to_string()))?;
        }
        builder = builder.alpns(vec![ALPN.to_vec()]);
        let endpoint = builder
            .bind()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        let self_id = NodeId::from_public_key_bytes(endpoint.id().as_bytes());
        let router = Router::builder(endpoint.clone())
            .accept(ALPN, SmartHandler { shared: shared.clone() })
            .spawn();
        Ok(Self { endpoint, router, shared, self_id })
    }

    #[must_use]
    pub fn addr(&self) -> EndpointAddr {
        self.endpoint.watch_addr().get()
    }

    #[must_use]
    pub fn addr_json(&self) -> String {
        serde_json::to_string(&self.addr()).unwrap_or_else(|_| "{}".to_owned())
    }

    pub async fn call(&self, peer: &EndpointAddr, request: &SmartRequest) -> Result<SmartReply> {
        let payload = serde_json::to_vec(request).map_err(|e| Error::Protocol(e.to_string()))?;
        let frame = encode_frame(&payload)?;
        let conn = self
            .endpoint
            .connect(peer.clone(), ALPN)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        let (mut send, mut recv) = conn
            .open_bi()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        send.write_all(&frame)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        send.finish()
            .map_err(|e| Error::Transport(e.to_string()))?;
        let response = recv
            .read_to_end(MAX_FRAME)
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        conn.close(0u32.into(), b"done");
        let reply_bytes = decode_reply_frame(&response)?;
        serde_json::from_slice(&reply_bytes).map_err(|e| Error::Protocol(e.to_string()))
    }

    pub async fn shutdown(self) -> Result<()> {
        self.router
            .shutdown()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        self.endpoint.close().await;
        Ok(())
    }
}

#[must_use]
pub fn decode_reply_frame(frame: &[u8]) -> Result<Vec<u8>> {
    if frame.len() < 5 {
        return Err(Error::Protocol("reply frame shorter than header".to_owned()));
    }
    let mut len = [0u8; 4];
    len.copy_from_slice(&frame[..4]);
    let declared = u32::from_be_bytes(len) as usize;
    if declared > MAX_FRAME {
        return Err(Error::Protocol("reply frame exceeds 1 MiB".to_owned()));
    }
    if frame.len() != 4 + declared {
        return Err(Error::Protocol(format!(
            "reply frame length mismatch: declared {declared}, received {}",
            frame.len()
        )));
    }
    Ok(frame[4..4 + declared].to_vec())
}

/// Issue a cluster-admin capability for a node. Used by the CLI and witness;
/// the secret comes from configuration, never from a literal.
#[must_use]
pub fn admin_capability(node: NodeId, secret: &[u8], at_ms: u64, nonce: [u8; 16]) -> Capability {
    Capability::issue(node, Scope::ClusterAdmin, at_ms.saturating_add(86_400_000), nonce, secret)
}

/// Deterministic, parameterised idempotency key for a labelled operation.
#[must_use]
pub fn idempotency_key(label: &str, seq: u64) -> [u8; 32] {
    let mut bytes = Vec::with_capacity(label.len() + 8);
    bytes.extend_from_slice(b"aws0/idem/v1\0");
    bytes.extend_from_slice(label.as_bytes());
    bytes.extend_from_slice(&seq.to_be_bytes());
    digest(&bytes)
}
