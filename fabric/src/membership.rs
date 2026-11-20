// SPDX-License-Identifier: CC0-1.0

//! Capacity health: majority quorum, failure signals and a bounded
//! phi-accrual detector. Every function is a pure morphism over an explicit
//! `(state, now_ms)` pair; clocks and secrets are caller-owned parameters,
//! never ambient state in this module.

use crate::crdt::{AppliedRecord, ClusterState, GCounter, Lww, MemberRecord, OrSet, VersionVector};
use crate::state::Event;
use crate::types::{
    Epoch, Error, NodeId, PartitionKey, Placement, Region, Result, WorkloadId, WorkloadSpec,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Heartbeat period in milliseconds.
pub const HEARTBEAT_MS: u64 = 1_000;
/// Silence strictly beyond this window produces a `Suspect` signal.
pub const SUSPECT_AFTER_MS: u64 = 5 * HEARTBEAT_MS;
/// Silence strictly beyond this window produces a `ConfirmDead` signal.
pub const DEAD_AFTER_MS: u64 = 15 * HEARTBEAT_MS;

/// Bounded per-node arrival window: O(1) memory per member.
const PHI_SAMPLES: usize = 16;
/// Fixed-point numerator scale: one factor of 1000 for milli-phi units and
/// one factor of 1000 for the fixed-point logarithm below.
const PHI_SCALE: u128 = 1_000_000;
/// Fixed-point `ln(10) * 1000`, rounded to the nearest integer.
const PHI_LN10_MILLI: u128 = 2_303;

/// Strict majority of `total`; an empty cluster requires no votes.
#[must_use]
pub fn quorum(total: usize) -> usize {
    if total == 0 {
        0
    } else {
        (total / 2).saturating_add(1)
    }
}

/// Heartbeat freshness is a separate predicate, deliberately outside the
/// quorum decision. A live member with a stale heartbeat is still a voter;
/// [`tick`] turns this predicate into `Suspect` / `ConfirmDead` signals, while
/// [`has_quorum`] reports membership counts only. Nothing here redefines
/// [`Error::NoQuorum`] as a clock-derived verdict.
#[must_use]
pub fn has_fresh_heartbeat(member: &MemberRecord, now_ms: u64) -> bool {
    now_ms.saturating_sub(member.last_heartbeat_ms) <= SUSPECT_AFTER_MS
}

/// Compacted live voting set: one vote per distinct node identity among live
/// records. Tombstoned keys and adds without a record carry no vote, so a
/// death or a re-join cannot inflate the denominator and an authenticated instance
/// cannot buy votes with fabricated keys. Derived, never stored, so compaction
/// cannot leave a dangling voter behind. All membership reads go through the
/// canonical [`ClusterState::live_members`] accessor rather than a second,
/// divergent index; convergence of that view is a property of `OrSet::join`.
#[must_use]
fn compacted_voters(records: &[MemberRecord]) -> BTreeSet<NodeId> {
    records.iter().map(|member| member.node).collect()
}

/// Quorum check over the compacted membership document. `have` is exactly the
/// live-member count, with no heartbeat-freshness filter; `need` is the strict
/// majority of the compacted live voting set. Every voter is by construction a
/// live record, so `have >= need` holds for every document admitted by
/// the state-exchange merge; documents with phantom adds or conflicting duplicate
/// identities are rejected there instead of silently lowering the bar.
/// `now_ms` is retained for call-site compatibility and is intentionally not
/// part of the decision: freshness is [`has_fresh_heartbeat`].
pub fn has_quorum(state: &ClusterState, now_ms: u64) -> Result<()> {
    let _ = now_ms;
    let live = state.live_members();
    let have = live.len();
    let need = quorum(compacted_voters(&live).len());
    if have >= need {
        Ok(())
    } else {
        Err(Error::NoQuorum { have, need })
    }
}

/// Exactly one failure signal per live member per tick: the stronger signal
/// subsumes the weaker. The caller emits its own heartbeat, never this module.
/// A repeated tick at a fixed `now_ms` yields the same vector, `View::apply`
/// deduplicates each signal by `event_key`, and a removed key is never read.
#[must_use]
pub fn tick(state: &ClusterState, now_ms: u64) -> Vec<Event> {
    let mut events = Vec::new();
    for member in state.live_members() {
        let silent_for = now_ms.saturating_sub(member.last_heartbeat_ms);
        if silent_for > DEAD_AFTER_MS {
            events.push(Event::ConfirmDead {
                node: member.node,
                incarnation: member.incarnation,
                at_ms: now_ms,
            });
        } else if !has_fresh_heartbeat(&member, now_ms) {
            events.push(Event::Suspect {
                node: member.node,
                incarnation: member.incarnation,
                at_ms: now_ms,
            });
        }
    }
    events
}

/// Canonical bytes of the full state document. Plain `ClusterState` JSON is
/// used whenever its map keys are JSON strings; otherwise the pair-list frame
/// below carries the same seven fields without losing a join operand.
pub fn gossip_payload(state: &ClusterState) -> Result<Vec<u8>> {
    match serde_json::to_vec(state) {
        Ok(bytes) => Ok(bytes),
        Err(plain_err) => {
            let frame = GossipFrame::State(wire_from_state(state));
            serde_json::to_vec(&frame).map_err(|frame_err| {
                Error::Protocol(format!(
                    "gossip encode: plain={plain_err}; frame={frame_err}"
                ))
            })
        }
    }
}

/// Deserialize a remote document and join it. Both wire shapes are accepted;
/// every remote membership document is validated before any join operand is
/// dropped, and a payload that is neither shape, that carries a phantom member
/// key, or whose pair lists are not canonical is rejected as a protocol error.
pub fn merge_gossip(state: &ClusterState, payload: &[u8]) -> Result<ClusterState> {
    let remote = match serde_json::from_slice::<ClusterState>(payload) {
        Ok(plain) => {
            validate_members(&plain.members)?;
            plain
        }
        Err(plain_err) => {
            let frame = match serde_json::from_slice::<GossipFrame>(payload) {
                Ok(frame) => frame,
                Err(frame_err) => {
                    return Err(Error::Protocol(format!(
                        "gossip decode: plain={plain_err}; frame={frame_err}"
                    )))
                }
            };
            match frame {
                GossipFrame::State(wire) => state_from_wire(wire)?,
            }
        }
    };
    Ok(state.join(&remote))
}

/// Greatest live incarnation observed for a node; zero for an unknown node.
#[must_use]
pub fn incarnation_for(state: &ClusterState, node: NodeId) -> u64 {
    state
        .live_members()
        .into_iter()
        .filter(|member| member.node == node)
        .fold(0u64, |acc, member| acc.max(member.incarnation))
}

/// Construct the first membership record of a node. The address is opaque
/// JSON data, never a command string interpolated into a shell.
#[must_use]
pub fn bootstrap_member(
    node: NodeId,
    region: Region,
    zone: &str,
    capacity_mb: u64,
    addr_json: &str,
    now_ms: u64,
) -> MemberRecord {
    MemberRecord {
        node,
        region,
        zone: zone.to_owned(),
        capacity_mb,
        incarnation: 0,
        last_heartbeat_ms: now_ms,
        endpoint_addr: addr_json.to_owned(),
    }
}

/// Bounded, deterministic Phi-accrual detector with integer fixed-point
/// arithmetic. `history` holds at most 16 arrival timestamps per node.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct PhiAccrual {
    pub history: BTreeMap<NodeId, Vec<u64>>,
}

impl PhiAccrual {
    #[must_use]
    pub fn new() -> Self {
        Self {
            history: BTreeMap::new(),
        }
    }

    /// Record one arrival into a new detector; the receiver is unchanged and
    /// the oldest samples are dropped once the window is full.
    #[must_use]
    pub fn observe(&self, node: NodeId, at_ms: u64) -> Self {
        let mut history = self.history.clone();
        let samples = history.entry(node).or_default();
        samples.push(at_ms);
        if samples.len() > PHI_SAMPLES {
            let mut tail: Vec<u64> = samples.iter().rev().take(PHI_SAMPLES).copied().collect();
            tail.reverse();
            *samples = tail;
        }
        Self { history }
    }

    /// Suspicion in milli-phi units: `phi * 1000 = elapsed_ms * 10^6 /
    /// (mean_ms * 2303)`, where `2303/1000` is fixed-point `ln(10)`. Zero for
    /// an unobserved node: no sample means no evidence to accrue.
    #[must_use]
    pub fn phi(&self, node: NodeId, now_ms: u64) -> u64 {
        let samples = match self.history.get(&node) {
            Some(s) => s,
            None => return 0,
        };
        let elapsed = match samples.last() {
            Some(last) => now_ms.saturating_sub(*last),
            None => return 0,
        };
        suspicion(samples, elapsed)
    }

    /// Live members whose accrued suspicion is at most `threshold`, in the
    /// same milli-phi units as `phi`. The detector has exactly one evidence
    /// function: `self.phi(member.node, now_ms)`. The liveness heartbeat is never
    /// blended into a silence estimate, so `phi` and this predicate cannot
    /// disagree about the same node. The unconditional `DEAD_AFTER_MS`
    /// fail-closed bound belongs to [`tick`], which emits `ConfirmDead` from
    /// liveness-heartbeat silence; it is not a second detector here.
    #[must_use]
    pub fn alive_nodes(&self, state: &ClusterState, now_ms: u64, threshold: u64) -> Vec<NodeId> {
        state
            .live_members()
            .into_iter()
            .filter(|member| self.phi(member.node, now_ms) <= threshold)
            .map(|member| member.node)
            .collect()
    }
}

/// Mean inter-arrival in milliseconds, with a one-heartbeat prior when fewer
/// than two samples exist. Out-of-order timestamps contribute zero, keeping
/// the estimate total for every input.
fn mean_interval(samples: &[u64]) -> u64 {
    let mut previous: Option<u64> = None;
    let mut sum: u128 = 0;
    let mut count: u128 = 0;
    for &sample in samples {
        if let Some(p) = previous {
            sum = sum.saturating_add(u128::from(sample.saturating_sub(p)));
            count = count.saturating_add(1);
        }
        previous = Some(sample);
    }
    if count == 0 {
        return HEARTBEAT_MS;
    }
    let mean = sum / count;
    if mean > u128::from(u64::MAX) {
        u64::MAX
    } else {
        (mean as u64).max(1)
    }
}

/// Integer-only, saturating suspicion. No floats and no division by zero: the
/// mean is clamped to at least one millisecond and the denominator to one.
fn suspicion(samples: &[u64], elapsed_ms: u64) -> u64 {
    let mean = u128::from(mean_interval(samples));
    let numerator = u128::from(elapsed_ms).saturating_mul(PHI_SCALE);
    let denominator = mean.saturating_mul(PHI_LN10_MILLI).max(1);
    let score = numerator / denominator;
    if score > u128::from(u64::MAX) {
        u64::MAX
    } else {
        score as u64
    }
}

/// Wire image of a `ClusterState`. Maps whose keys are not JSON strings are
/// encoded as pair lists in sorted `BTreeMap` order; every field is carried,
/// so the remote join is over the whole document.
#[derive(Serialize, Deserialize)]
struct GossipState {
    members: OrSet<MemberRecord>,
    workloads: Vec<(WorkloadId, Lww<WorkloadSpec>)>,
    placements: Vec<(WorkloadId, Lww<Placement>)>,
    epochs: Vec<(PartitionKey, Lww<Epoch>)>,
    applied: Vec<([u8; 32], AppliedRecord)>,
    counter: Vec<(NodeId, u64)>,
    version: Vec<(NodeId, u64)>,
}

/// Externally tagged frame, unambiguous against a bare `ClusterState` object.
#[derive(Serialize, Deserialize)]
enum GossipFrame {
    State(GossipState),
}

/// Validate a remote membership OR-Set before it is joined. An add key without
/// a record is a phantom voter: it can never emit a heartbeat, and admitting it
/// would inflate the voting set with an identity that has no member. A record
/// without an add key is equally malformed. Two live records for one node
/// identity that declare different regions are rejected too, so a crafted
/// OR-Set key order cannot steer region selection; tombstoned keys carry no
/// vote and are ignored for that check, so a re-join is not a false conflict.
fn validate_members(members: &OrSet<MemberRecord>) -> Result<()> {
    for key in members.adds.keys() {
        if !members.values.contains_key(key) {
            return Err(Error::Protocol(
                "gossip frame: phantom member add key has no matching record".to_owned(),
            ));
        }
    }
    for key in members.values.keys() {
        if !members.adds.contains_key(key) {
            return Err(Error::Protocol(
                "gossip frame: orphan member record has no add key".to_owned(),
            ));
        }
    }
    let mut regions: BTreeMap<NodeId, &Region> = BTreeMap::new();
    for key in members.adds.keys() {
        if !members.contains(key) {
            continue;
        }
        let record = match members.values.get(key) {
            Some(record) => record,
            None => {
                return Err(Error::Protocol(
                    "gossip frame: phantom live member key has no matching record".to_owned(),
                ))
            }
        };
        match regions.get(&record.node) {
            Some(previous) if *previous != &record.region => {
                return Err(Error::Protocol(format!(
                    "gossip frame: conflicting live regions for node {}",
                    record.node
                )));
            }
            Some(_) => {}
            None => {
                regions.insert(record.node, &record.region);
            }
        }
    }
    Ok(())
}

/// Collect a pair list only when its keys are strictly increasing in
/// `BTreeMap` order. `collect()` on a `Vec` silently keeps the last duplicate
/// and accepts unsorted frames, which drops a join operand; a frame that is
/// not the exact image of a `BTreeMap` is a protocol error instead.
fn checked_pairs<K: Ord, V>(field: &str, pairs: Vec<(K, V)>) -> Result<BTreeMap<K, V>> {
    if !pairs.windows(2).all(|pair| pair[0].0 < pair[1].0) {
        return Err(Error::Protocol(format!(
            "gossip frame: {field} pair list is duplicated or out of canonical BTreeMap order"
        )));
    }
    Ok(pairs.into_iter().collect())
}

fn wire_from_state(state: &ClusterState) -> GossipState {
    GossipState {
        members: state.members.clone(),
        workloads: state
            .workloads
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        placements: state
            .placements
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        epochs: state
            .epochs
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        applied: state.applied.iter().map(|(k, v)| (*k, v.clone())).collect(),
        counter: state.counter.0.iter().map(|(k, v)| (*k, *v)).collect(),
        version: state.version.0.iter().map(|(k, v)| (*k, *v)).collect(),
    }
}

fn state_from_wire(wire: GossipState) -> Result<ClusterState> {
    let GossipState {
        members,
        workloads,
        placements,
        epochs,
        applied,
        counter,
        version,
    } = wire;
    validate_members(&members)?;
    Ok(ClusterState {
        members,
        workloads: checked_pairs("workloads", workloads)?,
        placements: checked_pairs("placements", placements)?,
        epochs: checked_pairs("epochs", epochs)?,
        applied: checked_pairs("applied", applied)?,
        counter: GCounter(checked_pairs("counter", counter)?),
        version: VersionVector(checked_pairs("version", version)?),
    })
}
