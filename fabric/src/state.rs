// SPDX-License-Identifier: CC0-1.0

//! The one state machine. Every transition is a pure morphism
//! `(view, event) -> (view', effects)`. No transition performs IO; effects are
//! interpreted by `net::Executor`. `apply` is idempotent under event replay.

use crate::crdt::{AppliedRecord, ClusterState, MemberRecord};
use crate::types::{
    digest, Capability, Effect, Epoch, Error, Hlc, NodeId, PartitionKey, Placement,
    Result, Scope, WorkloadId, WorkloadSpec, WorkloadStatus,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// Exactly-once replay window. Keys are retained in insertion order and the
/// oldest is evicted once the bound is reached, so a long-lived cluster cannot
/// grow a persisted View without bound. The bound is explicit, not a silent
/// truncation: replay beyond it is a new observation under a finite window.
pub const MAX_SEEN_EVENTS: usize = 4_096;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Event {
    NodeJoin(MemberRecord),
    NodeLeave(NodeId),
    Heartbeat { node: NodeId, incarnation: u64, at_ms: u64 },
    Suspect { node: NodeId, incarnation: u64, at_ms: u64 },
    ConfirmDead { node: NodeId, incarnation: u64, at_ms: u64 },
    WorkloadCreate(WorkloadSpec),
    WorkloadScale { id: WorkloadId, replicas: usize },
    WorkloadDelete(WorkloadId),
    PlacementSet(Placement),
    Delta(Box<ClusterState>),
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct View {
    pub state: ClusterState,
    pub self_id: NodeId,
    pub hlc: Hlc,
    pub last_applied_ms: u64,
    pub seen_events: BTreeSet<[u8; 32]>,
    pub seen_order: VecDeque<[u8; 32]>,
}

impl View {
    #[must_use]
    pub fn new(self_id: NodeId, at_ms: u64) -> Self {
        Self {
            state: ClusterState::default(),
            self_id,
            hlc: Hlc::tick(None, at_ms, self_id),
            last_applied_ms: at_ms,
            seen_events: BTreeSet::new(),
            seen_order: VecDeque::new(),
        }
    }

    /// The single transition. `auth` is checked before any state change;
    /// `now_ms` is explicit so replay is deterministic.
    pub fn apply(
        &self,
        event: &Event,
        capability: &Capability,
        secret: &[u8],
        at_ms: u64,
    ) -> Result<(Self, Vec<Effect>)> {
        capability.verify(secret, at_ms)?;
        authorize(capability, event, &self.state)?;

        let key = event_key(event)?;
        if self.seen_events.contains(&key) {
            // Exactly-once replay: no second state change, no second effect.
            return Ok((self.clone(), Vec::new()));
        }

        let mut next = self.clone();
        next.hlc = Hlc::tick(Some(self.hlc), at_ms, self.self_id);
        next.last_applied_ms = at_ms;
        next.seen_events.insert(key);
        next.seen_order.push_back(key);
        while next.seen_events.len() > MAX_SEEN_EVENTS {
            match next.seen_order.pop_front() {
                Some(oldest) => {
                    next.seen_events.remove(&oldest);
                }
                None => break,
            }
        }
        next.state.version = next.state.version.bump(self.self_id);
        next.state.counter = next.state.counter.inc(self.self_id);
        let mut effects = Vec::new();

        match event {
            Event::NodeJoin(member) => {
                let dot = (self.self_id, next.state.version.0.get(&self.self_id).copied().unwrap_or(0));
                next.state.members = next
                    .state
                    .members
                    .add(member.node.to_string(), member.clone(), dot);
                effects.push(Effect::Broadcast(serde_json::to_vec(event).map_err(|e| Error::Protocol(e.to_string()))?));
            }
            Event::NodeLeave(node) => {
                next.state.members = next.state.members.remove(&node.to_string());
                let (v, e) = crate::scheduler::reconcile(&next, at_ms)?;
                next = v;
                effects.extend(e);
            }
            Event::Heartbeat { node, incarnation, at_ms: hb } => {
                let key = node.to_string();
                if let Some(m) = next.state.members.get(&key).cloned() {
                    let mut updated = m.clone();
                    // The payload clock is never trusted past the apply clock:
                    // one future-dated heartbeat must not pin liveness forever.
                    let observed = (*hb).min(at_ms);
                    updated.last_heartbeat_ms = observed.max(updated.last_heartbeat_ms);
                    updated.incarnation = (*incarnation).max(updated.incarnation);
                    if updated != m {
                        let dot = (self.self_id, next.state.counter.value());
                        next.state.members = next.state.members.add(key, updated, dot);
                    }
                }
            }
            // A suspicion is an observation with no placement consequence. A
            // confirmed death removes the observed incarnation and re-places
            // its workloads through the deterministic scheduler: every replica
            // derives the same LWW stamp from the converged CRDT state.
            Event::Suspect { .. } => {}
            Event::ConfirmDead { node, incarnation, at_ms: dead_at } => {
                let key = node.to_string();
                let observed_dead = next.state.members.get(&key).is_some_and(|m| {
                    m.incarnation <= *incarnation && m.last_heartbeat_ms <= *dead_at
                });
                if observed_dead {
                    next.state.members = next.state.members.remove(&key);
                    let (v, e) = crate::scheduler::reconcile(&next, at_ms)?;
                    next = v;
                    effects.extend(e);
                }
            }
            Event::WorkloadCreate(spec) => {
                spec.validate()?;
                let reg = next
                    .state
                    .workloads
                    .get(&spec.id)
                    .cloned()
                    .unwrap_or_default()
                    .write(spec.clone(), next.hlc);
                next.state.workloads.insert(spec.id.clone(), reg);
                let (v, e) = crate::scheduler::reconcile(&next, at_ms)?;
                next = v;
                effects.extend(e);
            }
            Event::WorkloadScale { id, replicas } => {
                let mut spec = next
                    .state
                    .workloads
                    .get(id)
                    .and_then(|r| r.get())
                    .cloned()
                    .ok_or_else(|| Error::UnknownWorkload(id.to_string()))?;
                spec.replicas = *replicas;
                spec.validate()?;
                let reg = next.state.workloads.get(id).cloned().unwrap_or_default().write(spec, next.hlc);
                next.state.workloads.insert(id.clone(), reg);
                let (v, e) = crate::scheduler::reconcile(&next, at_ms)?;
                next = v;
                effects.extend(e);
            }
            Event::WorkloadDelete(id) => {
                if let Some(reg) = next.state.workloads.get(id).cloned() {
                    next.state.workloads.insert(id.clone(), reg.tombstone(next.hlc));
                }
                if let Some(reg) = next.state.placements.get(id).cloned() {
                    next.state.placements.insert(id.clone(), reg.tombstone(next.hlc));
                }
                effects.push(Effect::StopWorkload(id.clone(), self.self_id));
            }
            Event::PlacementSet(placement) => {
                let reg = next
                    .state
                    .placements
                    .get(&placement.workload)
                    .cloned()
                    .unwrap_or_default()
                    .write(placement.clone(), next.hlc);
                next.state.placements.insert(placement.workload.clone(), reg);
                for node in &placement.nodes {
                    if let Some(spec) = next.state.workloads.get(&placement.workload).and_then(|r| r.get()).cloned() {
                        effects.push(Effect::StartWorkload(spec, *node));
                    }
                }
            }
            Event::Delta(incoming) => {
                incoming.validate_member_clocks(at_ms)?;
                let before = next.state.digest();
                next.state = next.state.join(incoming);
                let after = next.state.digest();
                if before != after {
                    effects.push(Effect::Audit {
                        id: "delta-merge".to_owned(),
                        ts_ms: at_ms,
                        hash: after,
                    });
                }
            }
        }

        let record = serde_json::to_vec(&next)
            .map_err(|e| Error::Protocol(e.to_string()))?;
        if record.len() > crate::net::MAX_FRAME {
            return Err(Error::Protocol(
                "persisted View exceeds the 1 MiB frame bound".to_owned(),
            ));
        }
        effects.push(Effect::Persist(record));
        Ok((next, effects))
    }

    /// Merge a remote state. Pure CRDT join -- no consensus, no leader, no
    /// coordination. Byzantine-tolerant at the transport (iroh authenticates
    /// every byte to a NodeId) and at the state (join is monotone).
    #[must_use]
    pub fn converge(&self, remote: &ClusterState) -> Self {
        let mut next = self.clone();
        next.state = self.state.join(remote);
        next
    }

    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        self.state.digest()
    }

    #[must_use]
    pub fn epoch_for(&self, p: &PartitionKey) -> Epoch {
        self.state
            .epochs
            .get(p)
            .and_then(|r| r.get())
            .copied()
            .unwrap_or(Epoch(0))
    }

    pub fn set_epoch(&mut self, p: &PartitionKey, e: Epoch, stamp: Hlc) {
        let reg = self.state.epochs.get(p).cloned().unwrap_or_default().write(e, stamp);
        self.state.epochs.insert(p.clone(), reg);
    }

    pub fn record_applied(&mut self, idempotency: &[u8; 32], reply: Vec<u8>, at_ms: u64) {
        self.state
            .applied
            .insert(*idempotency, AppliedRecord { at_ms, reply });
    }

    #[must_use]
    pub fn replay(&self, idempotency: &[u8; 32]) -> Option<Vec<u8>> {
        self.state.applied.get(idempotency).map(|r| r.reply.clone())
    }

    #[must_use]
    pub fn statuses(&self) -> BTreeMap<WorkloadId, WorkloadStatus> {
        self.state
            .workloads
            .keys()
            .map(|id| (id.clone(), self.state.status(id)))
            .collect()
    }
}

/// Canonical idempotency key of one observation: a domain-separated digest of
/// the complete serialized `Event`. Every field of every variant participates --
/// the signal kind, the node, the incarnation and `at_ms` -- so a heartbeat,
/// suspicion or death observed at a later tick or for a newer incarnation is a
/// distinct event and is never deduplicated against an earlier one. Serialization
/// is a protocol boundary: an event that cannot be encoded is rejected, never
/// replaced by a shared fallback value that would collapse distinct updates.
pub fn event_key(event: &Event) -> Result<[u8; 32]> {
    const DOMAIN: &[u8] = b"aws0/event-key/v1\0";
    let payload = serde_json::to_vec(event)
        .map_err(|e| Error::Protocol(format!("event key encode: {e}")))?;
    let mut canonical = Vec::with_capacity(DOMAIN.len() + 8 + payload.len());
    canonical.extend_from_slice(DOMAIN);
    canonical.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    canonical.extend_from_slice(&payload);
    Ok(digest(&canonical))
}

/// The one pure partition check, shared by event authorization and request
/// routing. A capability covers a partition only when its scope names that
/// partition's region or account; a workload-scoped capability never covers a
/// partition, because a workload id carries no partition of its own.
pub fn authorize_partition(scope: &Scope, p: &PartitionKey) -> Result<()> {
    let allowed = match scope {
        Scope::ClusterAdmin => true,
        Scope::Region(region) => region == &p.region,
        Scope::Tenant(account) => account == &p.account,
        Scope::Workload(_) => false,
    };
    if allowed {
        Ok(())
    } else {
        Err(Error::CapabilityDenied)
    }
}

/// Zero-trust authorization. A capability scope must cover the partition of
/// the event's workload. Events without an inline partition (scale, delete)
/// resolve the partition from the replicated workload registry, never from
/// caller-supplied text; a `PlacementSet` must name the SAME partition as the
/// workload it places. Cluster-wide membership and anti-entropy events require
/// `Scope::ClusterAdmin`. A `Workload(id)` scope is exactly one workload.
pub fn authorize(cap: &Capability, event: &Event, state: &ClusterState) -> Result<()> {
    if matches!(&cap.scope, Scope::ClusterAdmin) {
        return Ok(());
    }
    match event {
        Event::WorkloadCreate(spec) => authorize_partition(&cap.scope, &spec.partition),
        Event::WorkloadScale { id, .. } | Event::WorkloadDelete(id) => {
            if matches!(&cap.scope, Scope::Workload(w) if w == id) {
                return Ok(());
            }
            let spec = state
                .workloads
                .get(id)
                .and_then(|r| r.get())
                .ok_or_else(|| Error::UnknownWorkload(id.to_string()))?;
            authorize_partition(&cap.scope, &spec.partition)
        }
        Event::PlacementSet(placement) => {
            let spec = state
                .workloads
                .get(&placement.workload)
                .and_then(|r| r.get())
                .ok_or_else(|| Error::UnknownWorkload(placement.workload.to_string()))?;
            if spec.partition != placement.partition {
                return Err(Error::CapabilityDenied);
            }
            authorize_partition(&cap.scope, &spec.partition)
        }
        Event::NodeJoin(_)
        | Event::NodeLeave(_)
        | Event::Heartbeat { .. }
        | Event::Suspect { .. }
        | Event::ConfirmDead { .. }
        | Event::Delta(_) => Err(Error::CapabilityDenied),
    }
}

