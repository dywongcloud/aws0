// SPDX-License-Identifier: CC0-1.0

//! Smartops: a small, declarative, protocol-defined operation algebra.
//!
//! A smartop is a total function from `(View, SmartRequest)` to `(View,
//! SmartReply, Vec<Effect>)`. The protocol is fixed by [`SmartOp`]; there is no
//! plugin surface, no reflection, no dynamic dispatch, and no ambient state.
//! Idempotency keys give exactly-once semantics: replaying a request returns
//! the stored reply without re-running the transition.

use crate::crdt::MemberRecord;
use crate::state::{authorize, event_key, Event, View};
use crate::types::{
    Capability, Effect, Error, NodeId, Result, WorkloadId, WorkloadSpec,
};
use serde::{Deserialize, Serialize};

pub const ALPN: &[u8] = b"aws0/smartops/1";

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum SmartOp {
    /// Declare this node and its region/zone/capacity to the cluster.
    Join(MemberRecord),
    /// Leave the cluster; triggers self-healing re-placement.
    Leave(NodeId),
    /// Create/replace a workload. Placement is derived, never supplied.
    Create(WorkloadSpec),
    Scale { id: WorkloadId, replicas: usize },
    Delete(WorkloadId),
    /// Migrate a workload to a specific node (typestate-checked placement).
    Migrate { id: WorkloadId, to: NodeId },
    /// Drain a node: mark it leaving and re-place everything on it.
    Drain(NodeId),
    /// Anti-entropy: merge a remote CRDT state.
    Converge(Box<crate::crdt::ClusterState>),
    /// Read-only introspection.
    Status,
    Health,
    /// Durable snapshot + journal compaction.
    Snapshot,
    /// Reset derived status only; replicated state is never silently cleared.
    Replay { from_ms: u64 },
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SmartRequest {
    pub idempotency: [u8; 32],
    pub op: SmartOp,
    pub capability: Capability,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SmartReply {
    pub idempotency: [u8; 32],
    pub ok: bool,
    pub value: serde_json::Value,
    pub state_hash: [u8; 32],
    pub effects: usize,
    pub replayed: bool,
    pub errors: Vec<String>,
}

impl SmartReply {
    #[must_use]
    fn denied(idempotency: [u8; 32], why: String) -> Self {
        Self {
            idempotency,
            ok: false,
            value: serde_json::json!({ "error": why }),
            state_hash: [0u8; 32],
            effects: 0,
            replayed: false,
            errors: vec![why],
        }
    }
}

/// The outcome of one step: the new view, the reply, and the IO effects that
/// the executor must perform. `step` itself performs no IO.
#[derive(Clone, Debug)]
pub struct StepOutcome {
    pub view: View,
    pub reply: SmartReply,
    pub effects: Vec<Effect>,
}

/// The one protocol transition. Total: every input yields an outcome; a denied
/// or malformed request is a reply, never an exception escaping the boundary.
#[must_use]
pub fn step(view: &View, req: &SmartRequest, secret: &[u8], at_ms: u64) -> StepOutcome {
    if let Err(e) = req.capability.verify(secret, at_ms) {
        return StepOutcome {
            view: view.clone(),
            reply: SmartReply::denied(req.idempotency, e.to_string()),
            effects: Vec::new(),
        };
    }
    if let Some(cached) = view.replay(&req.idempotency) {
        if let Ok(reply) = serde_json::from_slice::<SmartReply>(&cached) {
            let mut replayed = reply;
            replayed.replayed = true;
            return StepOutcome { view: view.clone(), reply: replayed, effects: Vec::new() };
        }
    }

    let event = match &req.op {
        SmartOp::Join(m) => Event::NodeJoin(m.clone()),
        SmartOp::Leave(n) => Event::NodeLeave(*n),
        SmartOp::Create(s) => Event::WorkloadCreate(s.clone()),
        SmartOp::Scale { id, replicas } => Event::WorkloadScale { id: id.clone(), replicas: *replicas },
        SmartOp::Delete(id) => Event::WorkloadDelete(id.clone()),
        SmartOp::Migrate { id, to } => {
            let Some(p) = view.state.placement(id).cloned() else {
                return StepOutcome {
                    view: view.clone(),
                    reply: SmartReply::denied(
                        req.idempotency,
                        Error::UnknownWorkload(id.to_string()).to_string(),
                    ),
                    effects: Vec::new(),
                };
            };
            let mut placement = p;
            placement.nodes = vec![*to];
            placement.status = crate::types::WorkloadStatus::Scheduled;
            Event::PlacementSet(placement)
        }
        SmartOp::Drain(n) => Event::NodeLeave(*n),
        SmartOp::Converge(remote) => Event::Delta(remote.clone()),
        SmartOp::Status | SmartOp::Health | SmartOp::Snapshot | SmartOp::Replay { .. } => {
            let value = read_only(view, &req.op, at_ms);
            let reply = SmartReply {
                idempotency: req.idempotency,
                ok: true,
                value,
                state_hash: view.digest(),
                effects: 0,
                replayed: false,
                errors: Vec::new(),
            };
            let mut next = view.clone();
            if let Ok(bytes) = serde_json::to_vec(&reply) {
                next.record_applied(&req.idempotency, bytes, at_ms);
            }
            let effect = match &req.op {
                SmartOp::Snapshot => match serde_json::to_vec(&next) {
                    Ok(bytes) => Effect::Persist(bytes),
                    Err(e) => {
                        return StepOutcome {
                            view: view.clone(),
                            reply: SmartReply::denied(req.idempotency, e.to_string()),
                            effects: Vec::new(),
                        };
                    }
                },
                _ => Effect::Audit {
                    id: "read".to_owned(),
                    ts_ms: at_ms,
                    hash: view.digest(),
                },
            };
            return StepOutcome { view: next, reply, effects: vec![effect] };
        }
    };

    if let Err(e) = authorize(&req.capability, &event, &view.state) {
        return StepOutcome {
            view: view.clone(),
            reply: SmartReply::denied(req.idempotency, e.to_string()),
            effects: Vec::new(),
        };
    }

    // The canonical event key is a protocol boundary: an event that cannot be
    // encoded is rejected as a denied reply, never collapsed into a shared
    // fallback digest. `View::apply` computes the same key for deduplication.
    let key = match event_key(&event) {
        Ok(key) => key,
        Err(e) => {
            return StepOutcome {
                view: view.clone(),
                reply: SmartReply::denied(req.idempotency, e.to_string()),
                effects: Vec::new(),
            };
        }
    };

    match view.apply(&event, &req.capability, secret, at_ms) {
        Ok((mut next, effects)) => {
            let reply = SmartReply {
                idempotency: req.idempotency,
                ok: true,
                value: serde_json::json!({
                    "event": format!("{:?}", key.iter().map(|b| format!("{b:02x}")).collect::<String>()),
                    "members": next.state.live_members().len(),
                    "workloads": next.state.workloads.len(),
                    "epoch": next.hlc.wall_ms,
                }),
                state_hash: next.digest(),
                effects: effects.len(),
                replayed: false,
                errors: Vec::new(),
            };
            if let Ok(bytes) = serde_json::to_vec(&reply) {
                next.record_applied(&req.idempotency, bytes, at_ms);
            }
            StepOutcome { view: next, reply, effects }
        }
        Err(e) => StepOutcome {
            view: view.clone(),
            reply: SmartReply::denied(req.idempotency, e.to_string()),
            effects: Vec::new(),
        },
    }
}

#[must_use]
pub fn read_only_status(view: &View) -> serde_json::Value {
    read_only(view, &SmartOp::Status, view.last_applied_ms)
}

#[must_use]
fn read_only(view: &View, op: &SmartOp, at_ms: u64) -> serde_json::Value {
    match op {
        SmartOp::Status => serde_json::json!({
            "self": view.self_id.to_string(),
            "members": view.state.live_members().len(),
            "workloads": view.state.workloads.len(),
            "placements": view.state.placements.len(),
            "partitions": view.state.by_partition().len(),
            "state_hash": hex::encode(view.digest()),
            "hlc_wall_ms": view.hlc.wall_ms,
            "last_applied_ms": view.last_applied_ms,
            "statuses": view.statuses().iter().map(|(k, v)| (k.as_str().to_owned(), format!("{v:?}"))).collect::<std::collections::BTreeMap<String, String>>(),
        }),
        SmartOp::Health => serde_json::json!({
            "status": "ok",
            "at_ms": at_ms,
            "state_hash": hex::encode(view.digest()),
            "quorum": crate::membership::quorum(view.state.members.adds.keys().count()),
            "live": view.state.live_members().len(),
        }),
        SmartOp::Snapshot => serde_json::json!({
            "snapshot_at_ms": at_ms,
            "state_hash": hex::encode(view.digest()),
            "journal_effect": "Persist",
        }),
        SmartOp::Replay { from_ms } => serde_json::json!({
            "from_ms": from_ms,
            "events_seen": view.seen_events.len(),
            "state_hash": hex::encode(view.digest()),
        }),
        _ => serde_json::json!({}),
    }
}

/// The smartops protocol has no cycles: every op is a single transition with a
/// fixed dependency on exactly one view. This function checks a batch for
/// duplicate idempotency keys, which is the only structural failure mode.
pub fn validate_batch(requests: &[SmartRequest]) -> Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for r in requests {
        if !seen.insert(r.idempotency) {
            return Err(Error::CycleDetected);
        }
    }
    Ok(())
}

/// Build a `Create` request with a deterministic idempotency key.
#[must_use]
pub fn create_request(
    spec: WorkloadSpec,
    capability: Capability,
    label: &str,
    seq: u64,
) -> SmartRequest {
    SmartRequest {
        idempotency: crate::net::idempotency_key(label, seq),
        op: SmartOp::Create(spec),
        capability,
    }
}

#[must_use]
pub fn join_request(
    member: MemberRecord,
    capability: Capability,
    label: &str,
    seq: u64,
) -> SmartRequest {
    SmartRequest {
        idempotency: crate::net::idempotency_key(label, seq),
        op: SmartOp::Join(member),
        capability,
    }
}

#[must_use]
pub fn status_request(capability: Capability, label: &str, seq: u64) -> SmartRequest {
    SmartRequest {
        idempotency: crate::net::idempotency_key(label, seq),
        op: SmartOp::Status,
        capability,
    }
}

/// Protocol version. Kept explicit so a wire peer can refuse a mismatch
/// instead of silently misinterpreting bytes.
#[must_use]
pub fn protocol_version() -> serde_json::Value {
    serde_json::json!({
        "alpn": String::from_utf8_lossy(ALPN).to_string(),
        "version": 1,
        "max_frame": crate::net::MAX_FRAME,
        "ops": ["Join","Leave","Create","Scale","Delete","Migrate","Drain","Converge","Status","Health","Snapshot","Replay"],
    })
}

#[must_use]
pub fn tenant_capability(
    node: NodeId,
    scope: crate::types::Scope,
    secret: &[u8],
    at_ms: u64,
    nonce: [u8; 16],
) -> Capability {
    Capability::issue(node, scope, at_ms.saturating_add(86_400_000), nonce, secret)
}

/// A typestate witness: a request is either unverified or verified. This makes
/// the zero-trust boundary a type, not a convention.
#[derive(Clone, Debug)]
pub struct Verified<'a>(pub &'a SmartRequest, pub &'a [u8]);

impl<'a> Verified<'a> {
    pub fn verify(req: &'a SmartRequest, secret: &'a [u8], at_ms: u64) -> Result<Self> {
        req.capability.verify(secret, at_ms)?;
        Ok(Self(req, secret))
    }
    #[must_use]
    pub fn apply(&self, view: &View, at_ms: u64) -> StepOutcome {
        step(view, self.0, self.1, at_ms)
    }
}
