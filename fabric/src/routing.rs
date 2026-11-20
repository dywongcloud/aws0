// SPDX-License-Identifier: CC0-1.0

//! Capability-gated, deterministic request routing.
//!
//! `route` is a pure morphism `(View, RouteRequest, secret, at_ms) -> Result<Route>`:
//! no IO, no ambient clock, no state mutation. The zero-trust order is fixed:
//! verify the capability MAC and expiry, authorize the partition through
//! `crate::state::authorize_partition`, validate the request lexically, then
//! resolve an owner. Ownership is rendezvous hashing over the live membership set
//! restricted to the partition's region; a region with no live owner is rejected
//! with `Error::RegionUnavailable`, never spread to a foreign region.

use crate::crdt::ClusterState;
use crate::state::View;
use crate::types::{Capability, Error, NodeId, PartitionKey, Region, Result, WorkloadId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Maximum payload of a routed request, in bytes (1 MiB). Authentication runs
/// first, then this bound is enforced before any membership state is inspected.
const MAX_PAYLOAD: usize = 1 << 20;

/// Maximum length of a service or operation label, in bytes. Bounding both
/// labels keeps the rendezvous key and every score computation bounded.
const MAX_LABEL_BYTES: usize = 128;

/// Domain separator for the injective, length-prefixed rendezvous key.
const ROUTING_KEY_DOMAIN: &[u8] = b"aws0/routing-key/v1\0";

/// A signed, partition-scoped API request. `service` and `operation` are data
/// from a restricted alphabet; they are never interpolated into a shell, path,
/// or URL. The cluster secret is supplied to `route`, never stored here.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RouteRequest {
    pub partition: PartitionKey,
    pub service: String,
    pub operation: String,
    pub payload: Vec<u8>,
    pub capability: Capability,
}

/// The resolved owner of a request. `region` is always the requested partition's
/// region; cross-region service is never fabricated.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Route {
    pub target: NodeId,
    pub local: bool,
    pub region: Region,
}

/// Lexical bound on a service or operation label: 1..=MAX_LABEL_BYTES bytes
/// from the restricted alphabet. A total predicate: every byte string is either
/// accepted or rejected, with no implicit state.
#[must_use]
fn valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= MAX_LABEL_BYTES
        && label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-' || b == b'_')
}

/// Pure request validation. Region availability is a property of `ClusterState`,
/// so it is checked by `route`, which receives the `View`; this function is
/// lexically closed over `RouteRequest` alone.
pub fn validate_request(req: &RouteRequest) -> Result<()> {
    if req.service.len() > MAX_LABEL_BYTES || req.operation.len() > MAX_LABEL_BYTES {
        return Err(Error::Protocol(format!(
            "service or operation label exceeds {} bytes",
            MAX_LABEL_BYTES
        )));
    }
    if !valid_label(&req.service) {
        return Err(Error::Protocol("invalid service label".to_owned()));
    }
    if !valid_label(&req.operation) {
        return Err(Error::Protocol("invalid operation label".to_owned()));
    }
    if req.payload.len() > MAX_PAYLOAD {
        return Err(Error::Protocol("payload exceeds 1 MiB".to_owned()));
    }
    Ok(())
}

/// Append one length-prefixed field. The prefix is an eight-byte big-endian
/// length, so no field boundary can be confused with field content.
fn append_field(key: &mut Vec<u8>, field: &[u8]) {
    key.extend_from_slice(&(field.len() as u64).to_be_bytes());
    key.extend_from_slice(field);
}

/// The rendezvous key: a fixed domain separator followed by three
/// length-prefixed fields (partition bytes, service, operation). The encoding is
/// injective, so distinct requests cannot steer the same rendezvous owner.
#[must_use]
fn routing_key(req: &RouteRequest) -> Vec<u8> {
    let partition = req.partition.bytes();
    let mut key = Vec::with_capacity(
        ROUTING_KEY_DOMAIN.len()
            + 3 * std::mem::size_of::<u64>()
            + partition.len()
            + req.service.len()
            + req.operation.len(),
    );
    key.extend_from_slice(ROUTING_KEY_DOMAIN);
    append_field(&mut key, &partition);
    append_field(&mut key, req.service.as_bytes());
    append_field(&mut key, req.operation.as_bytes());
    key
}

/// Canonical live membership: one region set per NodeId. `BTreeSet` makes the
/// index independent of the OrSet key order of duplicate live records.
#[must_use]
fn member_regions(state: &ClusterState) -> BTreeMap<NodeId, BTreeSet<Region>> {
    let mut out: BTreeMap<NodeId, BTreeSet<Region>> = BTreeMap::new();
    for member in state.live_members() {
        out.entry(member.node).or_default().insert(member.region);
    }
    out
}

/// Live membership indexed by node. Conflicting live regions for one NodeId are
/// Byzantine peer state; reject the whole index instead of choosing a region by
/// iteration order.
fn members_by_node(state: &ClusterState) -> Result<BTreeMap<NodeId, Region>> {
    let mut out = BTreeMap::new();
    for (node, regions) in member_regions(state) {
        if regions.len() != 1 {
            return Err(Error::Protocol(format!(
                "conflicting live regions for node {}",
                node
            )));
        }
        match regions.into_iter().next() {
            Some(region) => {
                out.insert(node, region);
            }
            None => return Err(Error::Protocol("empty live region set".to_owned())),
        }
    }
    Ok(out)
}

/// Highest rendezvous score over a candidate set, total for every input. Ties
/// are broken by the full `NodeId` order, so the result is a function, not a
/// race between iteration orders. Parametric in the value carried per node.
fn pick<'a, T>(candidates: &'a BTreeMap<NodeId, T>, key: &[u8]) -> Option<(NodeId, &'a T)> {
    let mut best: Option<([u8; 32], NodeId)> = None;
    for node in candidates.keys() {
        let score = node.rendezvous_score(key);
        let dominates = match best {
            None => true,
            Some((best_score, best_node)) => {
                score > best_score || (score == best_score && *node > best_node)
            }
        };
        if dominates {
            best = Some((score, *node));
        }
    }
    let (_, node) = best?;
    candidates.get(&node).map(|value| (node, value))
}

/// Resolve a request to its serving node. Verification, partition authorization
/// and lexical validation precede all state inspection; a region without a live
/// owner is an error, never a cross-region fallback.
pub fn route(view: &View, req: &RouteRequest, secret: &[u8], at_ms: u64) -> Result<Route> {
    req.capability.verify(secret, at_ms)?;
    crate::state::authorize_partition(&req.capability.scope, &req.partition)?;
    validate_request(req)?;

    let key = routing_key(req);
    let members = members_by_node(&view.state)?;
    let mut in_region: BTreeMap<NodeId, ()> = BTreeMap::new();
    for (node, region) in &members {
        if region == &req.partition.region {
            in_region.insert(*node, ());
        }
    }
    if in_region.is_empty() {
        return Err(Error::RegionUnavailable(
            req.partition.region.as_str().to_owned(),
        ));
    }
    let target = pick(&in_region, &key)
        .map(|(node, _)| node)
        .ok_or_else(|| Error::RegionUnavailable(req.partition.region.as_str().to_owned()))?;
    Ok(Route {
        target,
        local: target == view.self_id,
        region: req.partition.region.clone(),
    })
}

/// Live node identities in one region, deduplicated and ordered by `NodeId`.
/// A NodeId with conflicting region declarations is excluded; `route` rejects
/// such a state outright.
#[must_use]
pub fn region_nodes(state: &ClusterState, region: &Region) -> Vec<NodeId> {
    member_regions(state)
        .into_iter()
        .filter(|(_, regions)| regions.len() == 1 && regions.contains(region))
        .map(|(node, _)| node)
        .collect()
}

/// Owner of a placed workload for a supplied key, by rendezvous over the
/// placement's nodes. An absent placement and an empty placement are both
/// reported as errors, never as a fabricated node.
pub fn placement_owner(state: &ClusterState, workload: &WorkloadId, key: &[u8]) -> Result<NodeId> {
    let placement = state
        .placement(workload)
        .ok_or_else(|| Error::UnknownWorkload(workload.to_string()))?;
    let mut candidates: BTreeMap<NodeId, ()> = BTreeMap::new();
    for node in &placement.nodes {
        candidates.insert(*node, ());
    }
    pick(&candidates, key)
        .map(|(node, _)| node)
        .ok_or_else(|| Error::NoCapacity {
            workload: workload.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crdt::{Lww, MemberRecord};
    use crate::types::{AccountId, Epoch, Hlc, Placement, Scope, WorkloadStatus};

    fn check(condition: bool, label: &str) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(Error::Protocol(label.to_owned()))
        }
    }

    fn record(node: NodeId, region: Region) -> MemberRecord {
        MemberRecord {
            node,
            region,
            zone: "zone-a".to_owned(),
            capacity_mb: 1024,
            incarnation: 1,
            last_heartbeat_ms: 0,
            endpoint_addr: String::new(),
        }
    }

    fn with_member(
        state: ClusterState,
        key: &str,
        node: NodeId,
        region: Region,
        stamp: u64,
    ) -> ClusterState {
        let members = state
            .members
            .add(key.to_owned(), record(node, region), (node, stamp));
        ClusterState { members, ..state }
    }

    fn request(
        partition: PartitionKey,
        scope: Scope,
        secret: &[u8],
        expires_at_ms: u64,
    ) -> RouteRequest {
        RouteRequest {
            partition,
            service: "ec2.DescribeInstances".to_owned(),
            operation: "Read".to_owned(),
            payload: Vec::new(),
            capability: Capability::issue(
                NodeId([3u8; 32]),
                scope,
                expires_at_ms,
                [1u8; 16],
                secret,
            ),
        }
    }

    #[test]
    fn labels_are_restricted() -> Result<()> {
        check(valid_label("ec2.DescribeInstances"), "dotted label")?;
        check(valid_label("s3_get-object"), "underscore and hyphen")?;
        check(!valid_label(""), "empty label")?;
        check(!valid_label("ec2/Describe"), "slash")?;
        check(!valid_label("ec2 Describe"), "space")?;
        check(!valid_label("ec2\0Describe"), "NUL")?;
        let non_ascii = String::from_utf8_lossy(&[0xC3u8, 0xA9u8, b'2']).into_owned();
        check(!valid_label(&non_ascii), "non-ASCII label")?;
        let overlong = "a".repeat(MAX_LABEL_BYTES + 1);
        check(!valid_label(&overlong), "overlong label")?;
        Ok(())
    }

    #[test]
    fn validation_bounds_are_exact() -> Result<()> {
        let account = AccountId::new("123456789012")?;
        let region = Region::new("us-east-1")?;
        let secret: Vec<u8> = Vec::new();
        let mut req = request(
            PartitionKey::new(account, region),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        check(validate_request(&req).is_ok(), "valid request")?;
        req.payload = vec![0u8; MAX_PAYLOAD];
        check(validate_request(&req).is_ok(), "payload at bound")?;
        req.payload.push(0);
        check(validate_request(&req).is_err(), "payload over bound")?;
        req.payload.clear();
        req.service = "ec2/Describe".to_owned();
        check(validate_request(&req).is_err(), "invalid service")?;
        req.service = "a".repeat(MAX_LABEL_BYTES + 1);
        check(validate_request(&req).is_err(), "service over bound")?;
        req.service = "ec2".to_owned();
        req.operation = "b".repeat(MAX_LABEL_BYTES + 1);
        check(validate_request(&req).is_err(), "operation over bound")?;
        Ok(())
    }

    #[test]
    fn routing_key_is_injective() -> Result<()> {
        let account = AccountId::new("123456789012")?;
        let short_region = Region::new("us-east-1")?;
        let long_region = Region::new("us-east-1a")?;
        let secret: Vec<u8> = Vec::new();
        let mut first = request(
            PartitionKey::new(account.clone(), short_region),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        first.service = "a".to_owned();
        first.operation = "bc".to_owned();
        let mut second = request(
            PartitionKey::new(account, long_region),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        second.service = "b".to_owned();
        second.operation = "c".to_owned();
        let first_key = routing_key(&first);
        let second_key = routing_key(&second);
        check(
            first_key != second_key,
            "length prefixes separate colliding suffixes",
        )?;
        let partition = first.partition.bytes();
        let mut expected_prefix = ROUTING_KEY_DOMAIN.to_vec();
        expected_prefix.extend_from_slice(&(partition.len() as u64).to_be_bytes());
        check(
            first_key.starts_with(&expected_prefix),
            "domain and partition length prefix",
        )?;
        Ok(())
    }

    #[test]
    fn rendezvous_selects_highest_score() -> Result<()> {
        let first = NodeId([1u8; 32]);
        let second = NodeId([2u8; 32]);
        let mut nodes: BTreeMap<NodeId, ()> = BTreeMap::new();
        nodes.insert(first, ());
        nodes.insert(second, ());
        let key = b"aws0/routing/test";
        let score_first = first.rendezvous_score(key);
        let score_second = second.rendezvous_score(key);
        let expected =
            if score_first > score_second || (score_first == score_second && first > second) {
                first
            } else {
                second
            };
        check(
            pick(&nodes, key).map(|(node, _)| node) == Some(expected),
            "expected rendezvous owner",
        )?;
        let empty: BTreeMap<NodeId, ()> = BTreeMap::new();
        check(pick(&empty, key).is_none(), "empty candidate set")?;
        Ok(())
    }

    #[test]
    fn route_is_region_isolated() -> Result<()> {
        let account = AccountId::new("123456789012")?;
        let us = Region::new("us-east-1")?;
        let eu = Region::new("eu-west-1")?;
        let missing = Region::new("ap-south-1")?;
        let us_node = NodeId([1u8; 32]);
        let eu_node = NodeId([2u8; 32]);
        let state = with_member(
            with_member(ClusterState::default(), "us", us_node, us.clone(), 1),
            "eu",
            eu_node,
            eu.clone(),
            2,
        );
        let view = View::new(us_node, 0).converge(&state);
        let secret: Vec<u8> = Vec::new();
        let req = request(
            PartitionKey::new(account.clone(), eu.clone()),
            Scope::Region(eu.clone()),
            &secret,
            u64::MAX,
        );
        let expected = Route {
            target: eu_node,
            local: false,
            region: eu.clone(),
        };
        check(
            route(&view, &req, &secret, 0) == Ok(expected),
            "requested region only",
        )?;
        check(
            validate_request(&req).is_ok(),
            "lexical validation ignores cluster state",
        )?;
        check(
            region_nodes(&state, &eu) == vec![eu_node],
            "region nodes are scoped",
        )?;

        let missing_req = request(
            PartitionKey::new(account.clone(), missing.clone()),
            Scope::Region(missing.clone()),
            &secret,
            u64::MAX,
        );
        check(
            route(&view, &missing_req, &secret, 0)
                == Err(Error::RegionUnavailable(missing.as_str().to_owned())),
            "missing region is unavailable",
        )?;
        let empty_view = View::new(us_node, 0);
        check(
            route(&empty_view, &req, &secret, 0)
                == Err(Error::RegionUnavailable(eu.as_str().to_owned())),
            "empty cluster is unavailable",
        )?;

        let local_req = request(
            PartitionKey::new(account.clone(), us.clone()),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        let local_route = route(&view, &local_req, &secret, 0)?;
        check(local_route.local, "local owner flag")?;
        check(&local_route.region == &us, "requested region is reported")?;

        let expired = request(
            PartitionKey::new(account.clone(), us.clone()),
            Scope::ClusterAdmin,
            &secret,
            0,
        );
        check(
            route(&view, &expired, &secret, 0) == Err(Error::CapabilityExpired),
            "expiry is checked first",
        )?;
        let mut tampered = request(
            PartitionKey::new(account.clone(), us.clone()),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        tampered.capability.mac[0] ^= 1;
        check(
            route(&view, &tampered, &secret, 0) == Err(Error::CapabilityDenied),
            "MAC is checked first",
        )?;
        let wrong_scope = request(
            PartitionKey::new(account.clone(), us.clone()),
            Scope::Region(eu),
            &secret,
            u64::MAX,
        );
        check(
            route(&view, &wrong_scope, &secret, 0) == Err(Error::CapabilityDenied),
            "partition authorization",
        )?;
        let mut oversized = request(
            PartitionKey::new(account, us),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        oversized.service = "a".repeat(MAX_LABEL_BYTES + 1);
        check(
            route(&view, &oversized, &secret, 0).is_err(),
            "oversized service label",
        )?;
        Ok(())
    }

    #[test]
    fn conflicting_region_records_are_rejected() -> Result<()> {
        let account = AccountId::new("123456789012")?;
        let us = Region::new("us-east-1")?;
        let eu = Region::new("eu-west-1")?;
        let healthy = NodeId([1u8; 32]);
        let conflicting = NodeId([2u8; 32]);
        let state = with_member(
            with_member(
                with_member(ClusterState::default(), "healthy", healthy, us.clone(), 1),
                "conflict-a",
                conflicting,
                us.clone(),
                2,
            ),
            "conflict-b",
            conflicting,
            eu,
            3,
        );
        check(
            members_by_node(&state).is_err(),
            "conflicting NodeId is rejected",
        )?;
        check(
            region_nodes(&state, &us) == vec![healthy],
            "ambiguous NodeId is excluded",
        )?;
        let view = View::new(healthy, 0).converge(&state);
        let secret: Vec<u8> = Vec::new();
        let req = request(
            PartitionKey::new(account, us),
            Scope::ClusterAdmin,
            &secret,
            u64::MAX,
        );
        check(
            route(&view, &req, &secret, 0).is_err(),
            "route rejects ambiguous membership",
        )?;
        Ok(())
    }

    #[test]
    fn placement_owner_reports_absent_empty_and_populated() -> Result<()> {
        let account = AccountId::new("123456789012")?;
        let region = Region::new("us-east-1")?;
        let node = NodeId([1u8; 32]);
        let workload = WorkloadId::new("api")?;
        let key = b"aws0/routing/placement";
        let absent = ClusterState::default();
        check(
            placement_owner(&absent, &workload, key)
                == Err(Error::UnknownWorkload(workload.to_string())),
            "absent placement",
        )?;
        let stamp = Hlc::tick(None, 0, node);
        let empty_placement = Placement {
            workload: workload.clone(),
            nodes: Vec::new(),
            epoch: Epoch(0),
            partition: PartitionKey::new(account, region),
            status: WorkloadStatus::Pending,
        };
        let mut placements: BTreeMap<WorkloadId, Lww<Placement>> = BTreeMap::new();
        placements.insert(
            workload.clone(),
            Lww {
                stamp: Some(stamp),
                value: Some(empty_placement.clone()),
            },
        );
        let empty_state = ClusterState {
            placements,
            ..ClusterState::default()
        };
        check(
            placement_owner(&empty_state, &workload, key)
                == Err(Error::NoCapacity {
                    workload: workload.to_string(),
                }),
            "empty placement",
        )?;
        let populated_placement = Placement {
            nodes: vec![node],
            ..empty_placement
        };
        let mut placements: BTreeMap<WorkloadId, Lww<Placement>> = BTreeMap::new();
        placements.insert(
            workload.clone(),
            Lww {
                stamp: Some(stamp),
                value: Some(populated_placement),
            },
        );
        let populated_state = ClusterState {
            placements,
            ..ClusterState::default()
        };
        check(
            placement_owner(&populated_state, &workload, key) == Ok(node),
            "populated placement",
        )?;
        Ok(())
    }
}
