// SPDX-License-Identifier: CC0-1.0

//! Convergent state values. Every `join` is a total, pure join-semilattice
//! morphism: `join(x,x)=x`, `join(x,y)=join(y,x)`,
//! `join(join(x,y),z)=join(x,join(y,z))`. A value conflict is resolved by
//! `CanonicalRank`: a deterministic byte rank over every field of the
//! payload, never by a `Debug` string or by merge order. `max` over that
//! rank is symmetric, so first-writer-wins cannot survive a three-way merge.

use crate::types::{
    digest, Epoch, Hlc, NodeId, PartitionKey, Placement, Region, Result, WorkloadId,
    WorkloadSpec, WorkloadStatus,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Contract for a replicated payload's total rank. Implementors must encode
/// every semantically relevant field, with domain separation and length
/// prefixes, in a fixed order. Two rank-equal values are therefore the same
/// value; no hidden field can make `max` order-dependent. A `Serialize`-only
/// rank is not enough: `#[serde(skip)]` fields would be invisible to it.
pub trait CanonicalRank {
    fn canonical_rank(&self) -> Vec<u8>;
}

fn push_field(out: &mut Vec<u8>, field: &[u8]) {
    let length = u64::try_from(field.len()).unwrap_or(u64::MAX);
    out.extend_from_slice(&length.to_be_bytes());
    out.extend_from_slice(field);
}

fn json_rank<T: Serialize>(domain: &[u8], value: &T) -> Vec<u8> {
    let mut out = Vec::new();
    push_field(&mut out, domain);
    match serde_json::to_vec(value) {
        Ok(bytes) => {
            out.push(1u8);
            push_field(&mut out, &bytes);
        }
        Err(err) => {
            let text = err.to_string();
            out.push(0u8);
            push_field(&mut out, text.as_bytes());
        }
    }
    out
}

impl CanonicalRank for String {
    fn canonical_rank(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_field(&mut out, b"aws0/rank/string/v1\0");
        push_field(&mut out, self.as_bytes());
        out
    }
}

impl<T: CanonicalRank> CanonicalRank for Option<T> {
    fn canonical_rank(&self) -> Vec<u8> {
        match self {
            None => vec![0u8],
            Some(value) => {
                let mut out = vec![1u8];
                out.extend_from_slice(&value.canonical_rank());
                out
            }
        }
    }
}

impl CanonicalRank for Epoch {
    fn canonical_rank(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_field(&mut out, b"aws0/rank/epoch/v1\0");
        out.extend_from_slice(&self.0.to_be_bytes());
        out
    }
}

impl CanonicalRank for WorkloadSpec {
    fn canonical_rank(&self) -> Vec<u8> {
        json_rank(b"aws0/rank/workload-spec/v1\0", self)
    }
}

impl CanonicalRank for Placement {
    fn canonical_rank(&self) -> Vec<u8> {
        json_rank(b"aws0/rank/placement/v1\0", self)
    }
}

/// Membership rank is causal first: a re-join (higher incarnation) outranks an
/// older incarnation, and within one incarnation a later heartbeat outranks an
/// earlier one. Capacity is deliberately NOT the primary key -- lexicographic
/// JSON bytes made a stale 8192 MB record beat a fresh 4096 MB one.
impl CanonicalRank for MemberRecord {
    fn canonical_rank(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_field(&mut out, b"aws0/rank/member/v1\0");
        out.extend_from_slice(&self.incarnation.to_be_bytes());
        out.extend_from_slice(&self.last_heartbeat_ms.to_be_bytes());
        out.extend_from_slice(&self.capacity_mb.to_be_bytes());
        out.extend_from_slice(self.node.as_bytes());
        push_field(&mut out, self.region.as_str().as_bytes());
        push_field(&mut out, self.zone.as_bytes());
        push_field(&mut out, self.endpoint_addr.as_bytes());
        out
    }
}

impl CanonicalRank for AppliedRecord {
    fn canonical_rank(&self) -> Vec<u8> {
        json_rank(b"aws0/rank/applied-record/v1\0", self)
    }
}

/// A last-writer-wins register. Ties are broken by the rank of the full
/// `Option<T>` value, so concurrent writes converge without a coordinator.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Lww<T> {
    pub stamp: Option<Hlc>,
    pub value: Option<T>,
}
impl<T> Default for Lww<T> {
    fn default() -> Self {
        Self { stamp: None, value: None }
    }
}
impl<T: Clone + PartialEq + Serialize + DeserializeOwned + CanonicalRank> Lww<T> {
    #[must_use]
    pub fn write(&self, value: T, stamp: Hlc) -> Self {
        match self.stamp {
            Some(s) if s >= stamp => self.clone(),
            _ => Self { stamp: Some(stamp), value: Some(value) },
        }
    }
    #[must_use]
    pub fn tombstone(&self, stamp: Hlc) -> Self {
        match self.stamp {
            Some(s) if s >= stamp => self.clone(),
            _ => Self { stamp: Some(stamp), value: None },
        }
    }
    #[must_use]
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        match (self.stamp, other.stamp) {
            (None, None) => Self::default(),
            (None, Some(_)) => other.clone(),
            (Some(_), None) => self.clone(),
            (Some(a), Some(b)) => {
                if a > b {
                    return self.clone();
                }
                if a < b {
                    return other.clone();
                }
                if other.value.canonical_rank() > self.value.canonical_rank() {
                    other.clone()
                } else {
                    self.clone()
                }
            }
        }
    }
}

/// An observed-remove set with dotted adds. A remove only wins over dots the
/// remover has observed, so a concurrent add is never lost. Values rise
/// monotonically in `CanonicalRank`, which makes `add` inflationary and `join`
/// a genuine semilattice operation.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct OrSet<T> {
    pub adds: BTreeMap<String, BTreeSet<(NodeId, u64)>>,
    pub removes: BTreeSet<(NodeId, u64)>,
    pub values: BTreeMap<String, T>,
}
impl<T> Default for OrSet<T> {
    fn default() -> Self {
        Self { adds: BTreeMap::new(), removes: BTreeSet::new(), values: BTreeMap::new() }
    }
}
impl<T: Clone + PartialEq + Serialize + DeserializeOwned + CanonicalRank> OrSet<T> {
    #[must_use]
    pub fn add(&self, key: String, value: T, dot: (NodeId, u64)) -> Self {
        let mut n = self.clone();
        let wins = match n.values.get(&key) {
            Some(current) => value.canonical_rank() > current.canonical_rank(),
            None => true,
        };
        if wins {
            n.values.insert(key.clone(), value);
        }
        n.adds.entry(key).or_default().insert(dot);
        n
    }
    #[must_use]
    pub fn remove(&self, key: &str) -> Self {
        let mut n = self.clone();
        if let Some(dots) = self.adds.get(key) {
            n.removes.extend(dots.iter().copied());
        }
        n
    }
    #[must_use]
    pub fn contains(&self, key: &str) -> bool {
        self.adds
            .get(key)
            .is_some_and(|dots| dots.iter().any(|d| !self.removes.contains(d)))
    }
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&T> {
        if self.contains(key) {
            self.values.get(key)
        } else {
            None
        }
    }
    #[must_use]
    pub fn live(&self) -> Vec<(String, T)> {
        self.adds
            .keys()
            .filter(|k| self.contains(k))
            .filter_map(|k| self.values.get(k).map(|v| (k.clone(), v.clone())))
            .collect()
    }
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        let mut adds = self.adds.clone();
        for (k, dots) in &other.adds {
            adds.entry(k.clone()).or_default().extend(dots.iter().copied());
        }
        let mut removes = self.removes.clone();
        removes.extend(other.removes.iter().copied());
        let mut values = self.values.clone();
        for (k, v) in &other.values {
            let wins = match values.get(k) {
                Some(current) => v.canonical_rank() > current.canonical_rank(),
                None => true,
            };
            if wins {
                values.insert(k.clone(), v.clone());
            }
        }
        Self { adds, removes, values }
    }
}

/// Hex-string wire form for `[u8; 32]` map keys. JSON object keys must be
/// strings; the idempotency ledger is encoded through this adapter without
/// changing the in-memory type.
mod hex_key {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;

    pub fn serialize<S, V: Serialize>(
        map: &BTreeMap<[u8; 32], V>,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wire: BTreeMap<String, &V> = map.iter().map(|(k, v)| (hex::encode(*k), v)).collect();
        wire.serialize(s)
    }

    pub fn deserialize<'de, D, V: Deserialize<'de>>(
        d: D,
    ) -> Result<BTreeMap<[u8; 32], V>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as DeError;
        let wire = BTreeMap::<String, V>::deserialize(d)?;
        let mut map = BTreeMap::new();
        for (k, v) in wire {
            let bytes = hex::decode(k.as_bytes()).map_err(|e| DeError::custom(e.to_string()))?;
            let arr: [u8; 32] = bytes
                .try_into()
                .map_err(|_| DeError::custom("key must be 32 bytes"))?;
            map.insert(arr, v);
        }
        Ok(map)
    }
}

/// A grow-only counter per node. Merge is pointwise max, so it is monotone and
/// can never double-count a replicated increment.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GCounter(pub BTreeMap<NodeId, u64>);
impl GCounter {
    #[must_use]
    pub fn inc(&self, node: NodeId) -> Self {
        let mut n = self.clone();
        let e = n.0.entry(node).or_insert(0);
        *e = e.saturating_add(1);
        n
    }
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0.values().fold(0u64, |a, b| a.saturating_add(*b))
    }
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        let mut m = self.0.clone();
        for (k, v) in &other.0 {
            let e = m.entry(*k).or_insert(0);
            *e = (*e).max(*v);
        }
        Self(m)
    }
}

/// A version vector: causal context for delta delivery and duplicate detection.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct VersionVector(pub BTreeMap<NodeId, u64>);
impl VersionVector {
    #[must_use]
    pub fn bump(&self, node: NodeId) -> Self {
        let mut n = self.clone();
        let e = n.0.entry(node).or_insert(0);
        *e = e.saturating_add(1);
        n
    }
    #[must_use]
    pub fn seen(&self, node: NodeId, seq: u64) -> bool {
        match self.0.get(&node) {
            Some(v) => *v >= seq,
            None => seq == 0,
        }
    }
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        let mut m = self.0.clone();
        for (k, v) in &other.0 {
            let e = m.entry(*k).or_insert(0);
            *e = (*e).max(*v);
        }
        Self(m)
    }
}

/// The replicated cluster document. It is the ONLY replicated state; every
/// subsystem reads or writes a disjoint field of it, so no two writers touch
/// the same surface.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ClusterState {
    pub members: OrSet<MemberRecord>,
    pub workloads: BTreeMap<WorkloadId, Lww<WorkloadSpec>>,
    pub placements: BTreeMap<WorkloadId, Lww<Placement>>,
    pub epochs: BTreeMap<PartitionKey, Lww<Epoch>>,
    #[serde(with = "hex_key")]
    pub applied: BTreeMap<[u8; 32], AppliedRecord>,
    pub counter: GCounter,
    pub version: VersionVector,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MemberRecord {
    pub node: NodeId,
    pub region: Region,
    pub zone: String,
    pub capacity_mb: u64,
    pub incarnation: u64,
    pub last_heartbeat_ms: u64,
    pub endpoint_addr: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AppliedRecord {
    pub at_ms: u64,
    pub reply: Vec<u8>,
}

impl ClusterState {
    #[must_use]
    pub fn join(&self, other: &Self) -> Self {
        let mut workloads = self.workloads.clone();
        for (k, v) in &other.workloads {
            let merged = match workloads.get(k) {
                Some(a) => a.join(v),
                None => v.clone(),
            };
            workloads.insert(k.clone(), merged);
        }
        let mut placements = self.placements.clone();
        for (k, v) in &other.placements {
            let merged = match placements.get(k) {
                Some(a) => a.join(v),
                None => v.clone(),
            };
            placements.insert(k.clone(), merged);
        }
        let mut epochs = self.epochs.clone();
        for (k, v) in &other.epochs {
            let merged = match epochs.get(k) {
                Some(a) => a.join(v),
                None => v.clone(),
            };
            epochs.insert(k.clone(), merged);
        }
        let mut applied = self.applied.clone();
        for (k, v) in &other.applied {
            let wins = match applied.get(k) {
                Some(current) => v.canonical_rank() > current.canonical_rank(),
                None => true,
            };
            if wins {
                applied.insert(*k, v.clone());
            }
        }
        Self {
            members: self.members.join(&other.members),
            workloads,
            placements,
            epochs,
            applied,
            counter: self.counter.join(&other.counter),
            version: self.version.join(&other.version),
        }
    }

    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        match serde_json::to_vec(self) {
            Ok(bytes) => digest(&bytes),
            Err(err) => {
                let mut bytes = vec![0u8];
                bytes.extend_from_slice(err.to_string().as_bytes());
                bytes.extend_from_slice(format!("{self:?}").as_bytes());
                digest(&bytes)
            }
        }
    }

    /// Reject membership clocks that claim a future beyond the permitted skew.
    /// This is a transport boundary check: an untrusted instance must not be able to
    /// pin a member alive forever with `u64::MAX`.
    pub fn validate_member_clocks(&self, now_ms: u64) -> Result<()> {
        const MAX_CLOCK_SKEW_MS: u64 = 60_000;
        let limit = now_ms.saturating_add(MAX_CLOCK_SKEW_MS);
        for record in self.members.values.values() {
            if record.last_heartbeat_ms > limit {
                return Err(crate::types::Error::Protocol(format!(
                    "member {} advertises a heartbeat beyond the clock skew bound",
                    record.node
                )));
            }
        }
        Ok(())
    }

    /// Derived partition index. It is computed, never stored, so a tombstone
    /// cannot leave a dangling id and `join` stays idempotent (zero leak).
    #[must_use]
    pub fn by_partition(&self) -> BTreeMap<PartitionKey, BTreeSet<WorkloadId>> {
        let mut index: BTreeMap<PartitionKey, BTreeSet<WorkloadId>> = BTreeMap::new();
        for (id, reg) in &self.workloads {
            if let Some(spec) = reg.get() {
                index
                    .entry(spec.partition.clone())
                    .or_default()
                    .insert(id.clone());
            }
        }
        index
    }

    #[must_use]
    pub fn live_members(&self) -> Vec<MemberRecord> {
        self.members
            .live()
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }

    #[must_use]
    pub fn placement(&self, id: &WorkloadId) -> Option<&Placement> {
        self.placements.get(id).and_then(|r| r.get())
    }

    #[must_use]
    pub fn status(&self, id: &WorkloadId) -> WorkloadStatus {
        match self.placement(id) {
            Some(p) => p.status.clone(),
            None => WorkloadStatus::Pending,
        }
    }
}

/// Three states disagreeing on one idempotency key.
fn divergent_triple() -> (ClusterState, ClusterState, ClusterState) {
    let key = [0xA5u8; 32];
    let make = |at_ms: u64, reply: u8| {
        let mut state = ClusterState::default();
        state.applied.insert(key, AppliedRecord { at_ms, reply: vec![reply] });
        state
    };
    (make(1, 1), make(2, 2), make(3, 3))
}

/// Three OR-sets holding different values for the same key.
fn divergent_or_sets() -> (OrSet<String>, OrSet<String>, OrSet<String>) {
    let make = |value: &str, byte: u8| {
        OrSet::<String>::default().add(
            "conflict".to_owned(),
            value.to_owned(),
            (NodeId([byte; 32]), u64::from(byte)),
        )
    };
    (make("from-a", 1), make("from-b", 2), make("from-c", 3))
}

/// Three real `ClusterState`s whose membership documents conflict on ONE key
/// with divergent `MemberRecord` heartbeats. A first-writer-wins `OrSet` merge
/// passes the law check on disjoint keys; this is the case that catches it.
fn divergent_members() -> Result<(ClusterState, ClusterState, ClusterState)> {
    let region = Region::new("us-east-1")?;
    let make = |incarnation: u64, heartbeat: u64, capacity: u64, byte: u8| {
        let node = NodeId([byte; 32]);
        let mut state = ClusterState::default();
        state.members = state.members.add(
            "conflict".to_owned(),
            MemberRecord {
                node,
                region: region.clone(),
                zone: "us-east-1a".to_owned(),
                capacity_mb: capacity,
                incarnation,
                last_heartbeat_ms: heartbeat,
                endpoint_addr: format!("endpoint-{byte}"),
            },
            (node, incarnation),
        );
        state
    };
    Ok((
        make(1, 1_000, 8_192, 1),
        make(2, 3_000, 4_096, 2),
        make(3, 4_000, 2_048, 3),
    ))
}

/// A causal-fitness check: a fresh heartbeat with a smaller capacity must beat
/// a stale heartbeat with a larger one in BOTH merge orders.
#[must_use]
pub fn check_member_freshness() -> bool {
    let region = match Region::new("us-east-1") {
        Ok(r) => r,
        Err(_) => return false,
    };
    let record = |incarnation: u64, heartbeat: u64, capacity: u64, byte: u8| MemberRecord {
        node: NodeId([byte; 32]),
        region: region.clone(),
        zone: "us-east-1a".to_owned(),
        capacity_mb: capacity,
        incarnation,
        last_heartbeat_ms: heartbeat,
        endpoint_addr: format!("endpoint-{byte}"),
    };
    let fresh = record(12, 1_795_154_300_000, 4_096, 4);
    let stale = record(1, 1_795_154_280_000, 8_192, 5);
    let a = ClusterState::default().members.add(
        "node".to_owned(),
        fresh.clone(),
        (NodeId([4u8; 32]), 1),
    );
    let b = ClusterState::default().members.add(
        "node".to_owned(),
        stale,
        (NodeId([5u8; 32]), 1),
    );
    let mut sa = ClusterState::default();
    sa.members = a;
    let mut sb = ClusterState::default();
    sb.members = b;
    sa.join(&sb).members.get("node") == Some(&fresh)
        && sb.join(&sa).members.get("node") == Some(&fresh)
}

/// A `join` law check used by the live witness. Returns the law names that
/// hold for the supplied triple and for three constructed same-key conflicts,
/// including divergent `MemberRecord` heartbeats inside `ClusterState`.
pub fn check_join_laws(
    a: &ClusterState,
    b: &ClusterState,
    c: &ClusterState,
) -> Result<BTreeSet<&'static str>> {
    let (p, q, r) = divergent_triple();
    let (u, v, w) = divergent_or_sets();
    let (m, n, o) = divergent_members()?;
    let mut ok = BTreeSet::new();
    let triples = [
        (a, b, c),
        (&p, &q, &r),
        (&m, &n, &o),
    ];
    if triples.iter().all(|(x, y, z)| {
        x.join(x) == **x && y.join(y) == **y && z.join(z) == **z
    }) && u.join(&u) == u
        && v.join(&v) == v
        && w.join(&w) == w
    {
        ok.insert("idempotent");
    }
    if triples.iter().all(|(x, y, _)| x.join(y) == y.join(x))
        && u.join(&v) == v.join(&u)
    {
        ok.insert("commutative");
    }
    if triples.iter().all(|(x, y, z)| x.join(y).join(z) == x.join(&y.join(z)))
        && u.join(&v).join(&w) == u.join(&v.join(&w))
    {
        ok.insert("associative");
    }
    if !check_member_freshness() {
        ok.clear();
    }
    Ok(ok)
}
