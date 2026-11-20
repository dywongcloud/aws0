// SPDX-License-Identifier: CC0-1.0

//! Deterministic scheduler. `reconcile` is a least fixed point: applying it to
//! its own output produces no new effects (`f compose f == f`). Placement is
//! deterministic hashing over live capacity, so every instance independently
//! computes the same answer from the same state -- no leader, no consensus,
//! no contention.

use crate::crdt::MemberRecord;
use crate::state::View;
use crate::types::{
    Effect, Epoch, Error, NodeId, Placement, Result, WorkloadId, WorkloadSpec, WorkloadStatus,
};
use std::collections::{BTreeMap, BTreeSet};

/// Reconcile desired replicas against observed placement. Pure: returns a new
/// `View` and the effects the executor must perform.
pub fn reconcile(view: &View, at_ms: u64) -> Result<(View, Vec<Effect>)> {
    let members = view.state.live_members();
    let quorum = crate::membership::quorum(members.len());
    if members.len() < quorum {
        return Err(Error::NoQuorum { have: members.len(), need: quorum });
    }

    let mut next = view.clone();
    let mut effects = Vec::new();
    let specs: Vec<(WorkloadId, WorkloadSpec)> = next
        .state
        .workloads
        .iter()
        .filter_map(|(id, reg)| reg.get().cloned().map(|s| (id.clone(), s)))
        .collect();

    let mut load: BTreeMap<NodeId, u64> = BTreeMap::new();
    for (id, spec) in specs {
        spec.validate()?;
        let candidates = candidates_for(&members, &spec);
        if candidates.is_empty() {
            return Err(Error::NoCapacity { workload: id.to_string() });
        }
        let chosen = choose(&candidates, &spec, &load)?;
        for node in &chosen {
            let entry = load.entry(*node).or_insert(0);
            *entry = entry.saturating_add(spec.memory_mb);
        }
        let epoch = next.epoch_for(&spec.partition);
        let desired = Placement {
            workload: id.clone(),
            nodes: chosen.clone(),
            epoch,
            partition: spec.partition.clone(),
            status: WorkloadStatus::Scheduled,
        };
        let current = next.state.placement(&id).cloned();
        let unchanged = current.as_ref().is_some_and(|p| {
            p.nodes == desired.nodes
                && p.epoch == desired.epoch
                && (p.status == WorkloadStatus::Scheduled || p.status == WorkloadStatus::Running)
        });
        if unchanged {
            continue;
        }
        // Bump the partition epoch exactly once per real change, so a stale
        // replica can never win a later LWW merge.
        let bumped = epoch.next();
        let epoch_stamp = deterministic_stamp(
            &next.state,
            format!("epoch:{}", spec.partition.wire()).as_bytes(),
            next.state.epochs.get(&spec.partition).and_then(|r| r.stamp),
        );
        next.set_epoch(&spec.partition, bumped, epoch_stamp);
        let placement_stamp = deterministic_stamp(
            &next.state,
            format!("placement:{}", id.as_str()).as_bytes(),
            next.state.placements.get(&id).and_then(|r| r.stamp),
        );
        desired_with_epoch(&mut next, &id, &desired, bumped, placement_stamp);
        for node in &chosen {
            effects.push(Effect::StartWorkload(spec.clone(), *node));
        }
        if let Some(old) = &current {
            for node in &old.nodes {
                if !chosen.contains(node) {
                    effects.push(Effect::StopWorkload(id.clone(), *node));
                }
            }
        }
        effects.push(Effect::Audit {
            id: format!("place:{}", id.as_str()),
            ts_ms: at_ms,
            hash: crate::types::digest(
                format!("{}:{}:{:?}", id.as_str(), bumped.0, chosen).as_bytes(),
            ),
        });
    }

    // Self-healing: any placement naming a node no longer live is replaced.
    let live: BTreeSet<NodeId> = members.iter().map(|m| m.node).collect();
    let stale: Vec<(WorkloadId, Placement)> = next
        .state
        .placements
        .iter()
        .filter_map(|(id, reg)| reg.get().cloned().map(|p| (id.clone(), p)))
        .filter(|(_, p)| p.nodes.iter().any(|n| !live.contains(n)))
        .collect();
    for (id, p) in stale {
        let spec = match next.state.workloads.get(&id).and_then(|r| r.get()) {
            Some(s) => s.clone(),
            None => continue,
        };
        let candidates = candidates_for(&members, &spec);
        if candidates.is_empty() {
            return Err(Error::NoCapacity { workload: id.to_string() });
        }
        let chosen = choose(&candidates, &spec, &BTreeMap::new())?;
        let healed = Placement {
            workload: id.clone(),
            nodes: chosen.clone(),
            epoch: p.epoch.next(),
            partition: p.partition.clone(),
            status: WorkloadStatus::Scheduled,
        };
        let stamp = deterministic_stamp(
            &next.state,
            format!("placement:{}", id.as_str()).as_bytes(),
            next.state.placements.get(&id).and_then(|r| r.stamp),
        );
        let reg = next
            .state
            .placements
            .get(&id)
            .cloned()
            .unwrap_or_default()
            .write(healed, stamp);
        next.state.placements.insert(id.clone(), reg);
        for node in &chosen {
            effects.push(Effect::StartWorkload(spec.clone(), *node));
        }
        effects.push(Effect::Audit {
            id: format!("heal:{}", id.as_str()),
            ts_ms: at_ms,
            hash: crate::types::digest(format!("{id:?}:{:?}:{at_ms}", chosen).as_bytes()),
        });
    }
    Ok((next, effects))
}

/// Placement and epoch LWW stamps must be a pure function of the converged
/// converged state, not of the local replica's HLC. Two replicas applying the same
/// failure signal therefore write byte-identical registers, and a later join
/// cannot oscillate between two local clocks. The previous stamp supplies
/// monotonicity so `Lww::write` always accepts the new value.
fn deterministic_stamp(
    state: &crate::crdt::ClusterState,
    label: &[u8],
    previous: Option<crate::types::Hlc>,
) -> crate::types::Hlc {
    let mut buffer = b"aws0/placement-stamp/v1\0".to_vec();
    buffer.extend_from_slice(&state.digest());
    buffer.extend_from_slice(&(label.len() as u64).to_be_bytes());
    buffer.extend_from_slice(label);
    let hash = crate::types::digest(&buffer);
    let mut node_input = b"aws0/placement-node/v1\0".to_vec();
    node_input.extend_from_slice(&hash);
    let node = crate::types::NodeId(crate::types::digest(&node_input));
    let mut wall_bytes = [0u8; 8];
    wall_bytes.copy_from_slice(&hash[..8]);
    let base = u64::from_be_bytes(wall_bytes);
    let wall_ms = match previous {
        Some(p) => p.wall_ms.max(base).saturating_add(1),
        None => base,
    };
    crate::types::Hlc { wall_ms, counter: 0, node }
}

#[must_use]
fn candidates_for(members: &[MemberRecord], spec: &WorkloadSpec) -> Vec<MemberRecord> {
    let in_region: Vec<MemberRecord> = members
        .iter()
        .filter(|m| m.region == spec.partition.region)
        .cloned()
        .collect();
    // Region affinity is a preference, not a hard wall: a region with no live
    // nodes still runs, on another region's nodes, with the same contract.
    if in_region.is_empty() {
        members.to_vec()
    } else {
        in_region
    }
}

fn choose(
    candidates: &[MemberRecord],
    spec: &WorkloadSpec,
    load: &BTreeMap<NodeId, u64>,
) -> Result<Vec<NodeId>> {
    let key = {
        let mut k = spec.partition.bytes();
        k.extend_from_slice(spec.id.as_str().as_bytes());
        k
    };
    let mut ranked: Vec<([u8; 32], MemberRecord)> = candidates
        .iter()
        .cloned()
        .map(|m| (m.node.rendezvous_score(&key), m))
        .filter(|(_, m)| {
            load.get(&m.node).copied().unwrap_or(0).saturating_add(spec.memory_mb) <= m.capacity_mb
        })
        .collect();
    ranked.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.node.cmp(&b.1.node)));
    let want = spec.replicas.min(ranked.len());
    let chosen: Vec<NodeId> = ranked.iter().take(want).map(|(_, m)| m.node).collect();
    if chosen.is_empty() {
        Err(Error::NoCapacity { workload: spec.id.to_string() })
    } else {
        Ok(chosen)
    }
}

fn desired_with_epoch(
    view: &mut View,
    id: &WorkloadId,
    desired: &Placement,
    epoch: Epoch,
    stamp: crate::types::Hlc,
) {
    let mut placement = desired.clone();
    placement.epoch = epoch;
    let reg = view
        .state
        .placements
        .get(id)
        .cloned()
        .unwrap_or_default()
        .write(placement, stamp);
    view.state.placements.insert(id.clone(), reg);
}
