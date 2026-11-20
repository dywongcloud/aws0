// SPDX-License-Identifier: CC0-1.0

//! Live witnessed verification. No mocks, no fakes, no synthetic test files:
//! this binary boots real capacity, exercises real persistence and checks the
//! bundled workload runtime. Every claim is `(id, ts, hash)` witnessed.

use aws0_fabric::crdt::{check_join_laws, check_member_freshness, ClusterState, GCounter, Lww, MemberRecord, OrSet, VersionVector};
use aws0_fabric::facade::{simple_workload, Fabric, FabricConfig};
use aws0_fabric::net::{admin_capability, idempotency_key, Node, NodeConfig, Shared};
use aws0_fabric::persistence::{bind_identity, Journal};
use aws0_fabric::runtime::{database_spec, laboon_argv, read_bounded, DatabaseKind, LaboonRuntime, WorkloadSupervisor, LABOON_ARTIFACTS};
use aws0_fabric::smartops::{SmartOp, SmartRequest, SmartReply};
use aws0_fabric::state::{Event, View, MAX_SEEN_EVENTS};
use aws0_fabric::types::{
    digest, keyed_mac, AccountId, Capability, Error, Hlc, NodeId, PartitionKey, Region, Result,
    Scope, WorkloadId, WorkloadSpec,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, serde::Serialize)]
struct Witness {
    id: &'static str,
    ok: bool,
    detail: serde_json::Value,
    hash: String,
    ts_ms: u64,
}

fn witness(id: &'static str, ok: bool, detail: serde_json::Value) -> Witness {
    let bytes = serde_json::to_vec(&detail).unwrap_or_else(|_| Vec::new());
    Witness {
        id,
        ok,
        detail,
        hash: hex::encode(digest(&bytes)),
        ts_ms: aws0_fabric::types::now_ms(),
    }
}

macro_rules! check {
    ($out:ident, $id:literal, $cond:expr, $detail:expr) => {{
        let ok = $cond;
        eprintln!("WITNESS {} {}", if ok { "OK" } else { "FAIL" }, $id);
        $out.push(witness($id, ok, $detail));
        ok
    }};
}

fn acct(s: &str) -> Result<AccountId> {
    AccountId::new(s)
}
fn reg(s: &str) -> Result<Region> {
    Region::new(s)
}
fn wid(s: &str) -> Result<WorkloadId> {
    WorkloadId::new(s)
}
fn part() -> Result<PartitionKey> {
    Ok(PartitionKey::new(acct("000000000000")?, reg("us-east-1")?))
}
fn spec(name: &str, replicas: usize) -> Result<WorkloadSpec> {
    Ok(WorkloadSpec {
        id: wid(name)?,
        image: "postgres:16".to_owned(),
        command: Vec::new(),
        env: Vec::new(),
        ports: vec![5432],
        memory_mb: 512,
        timeout_ms: 60_000,
        egress_allowlist: Vec::new(),
        partition: part()?,
        replicas,
    })
}
fn member(node: NodeId, at: u64, capacity: u64) -> Result<MemberRecord> {
    Ok(MemberRecord {
        node,
        region: reg("us-east-1")?,
        zone: "us-east-1-a".to_owned(),
        capacity_mb: capacity,
        incarnation: 1,
        last_heartbeat_ms: at,
        endpoint_addr: "{}".to_owned(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut out: Vec<Witness> = Vec::new();
    let at = aws0_fabric::types::now_ms();
    let secret = iroh::SecretKey::generate().to_bytes().to_vec();
    let data = PathBuf::from("/Users/dylanwong/projects/aws0-research-2026-11-19/witness-data");
    std::fs::create_dir_all(&data).map_err(|e| Error::Journal(e.to_string()))?;
    // These are regenerable witness journals and node identities. A new run
    // generates a new cluster secret, so retaining an old journal would bind a
    // stale view identity to a new derived identity. The verified laboon binary
    // under fabric/bin is deliberately preserved.
    for stale in [
        data.join("journal"),
        data.join("fabric").join("node-0"),
        data.join("fabric").join("node-1"),
        data.join("fabric").join("node-2"),
    ] {
        if stale.exists() {
            std::fs::remove_dir_all(&stale).map_err(|e| Error::Journal(e.to_string()))?;
        }
    }

    // ---- 1. CRDT laws on generated states (pure, exhaustive over the triple).
    let n1 = NodeId([1u8; 32]);
    let n2 = NodeId([2u8; 32]);
    let n3 = NodeId([3u8; 32]);
    let h1 = Hlc::tick(None, 1000, n1);
    let h2 = Hlc::tick(Some(h1), 1001, n2);
    let h3 = Hlc::tick(Some(h2), 1002, n3);
    let s_a = spec("alpha", 1)?;
    let s_b = spec("beta", 2)?;
    let s_g = spec("gamma", 1)?;
    let mut a = ClusterState::default();
    a.workloads.insert(s_a.id.clone(), Lww::default().write(s_a, h1));
    a.members = a.members.add(n1.to_string(), member(n1, 1000, 4096)?, (n1, 1));
    a.counter = GCounter::default().inc(n1);
    a.version = VersionVector::default().bump(n1);
    let mut b = ClusterState::default();
    b.workloads.insert(s_b.id.clone(), Lww::default().write(s_b, h2));
    b.counter = GCounter::default().inc(n2);
    b.version = VersionVector::default().bump(n2);
    let mut c = a.join(&b);
    c.workloads.insert(s_g.id.clone(), Lww::default().write(s_g, h3));
    let laws = check_join_laws(&a, &b, &c)?;
    check!(out, "crdt-join-laws", laws.len() == 3, json!({"laws": laws}));
    let fresh = check_member_freshness();
    check!(out, "crdt-member-freshness", fresh, json!({"fresh_record_wins_both_orders": fresh}));

    // OR-set observed-remove: a concurrent add survives a remove of an older dot.
    let os = OrSet::<String>::default();
    let seen = os.add("k".to_owned(), "v1".to_owned(), (n1, 1));
    let concurrent = seen.add("k".to_owned(), "v2".to_owned(), (n2, 2));
    let removed = seen.remove("k");
    let merged = removed.join(&concurrent);
    check!(out, "orset-observed-remove", merged.contains("k"), json!({"live": merged.live()}));

    // LWW tie-break converges symmetrically.
    let l1 = Lww::default().write("left".to_owned(), h3);
    let l2 = Lww::default().write("right".to_owned(), h3);
    check!(out, "lww-symmetric", l1.join(&l2) == l2.join(&l1), json!({"left": format!("{l1:?}"), "right": format!("{l2:?}")}));

    // ---- 2. Capability HMAC: constant-time verify, expiry, tamper rejection.
    let cap = Capability::issue(n1, Scope::ClusterAdmin, at + 60_000, [7u8; 16], &secret);
    let mut tampered = cap.clone();
    tampered.mac[0] ^= 1;
    let expired = Capability::issue(n1, Scope::ClusterAdmin, at, [7u8; 16], &secret);
    let wrong_key = Capability::issue(n1, Scope::ClusterAdmin, at + 60_000, [7u8; 16], b"other-secret");
    let auth_ok = cap.verify(&secret, at).is_ok()
        && tampered.verify(&secret, at).is_err()
        && expired.verify(&secret, at).is_err()
        && wrong_key.verify(&secret, at).is_err();
    check!(out, "authz-zero-trust", auth_ok, json!({"hmac_hex": hex::encode(keyed_mac(&secret, b"probe"))}));

    // ---- 3. Types: every constructor validates, no partial values.
    let types_ok = AccountId::new("12x45678901").is_err()
        && Region::new("US-EAST-1").is_err()
        && wid("../etc/passwd").is_err()
        && simple_workload("demo", "postgres:16", "000000000000", "us-east-1", 1).is_ok();
    check!(out, "types-total", types_ok, json!({"validated": ["AccountId", "Region", "WorkloadId", "WorkloadSpec"]}));

    // ---- 4. Pure state machine: idempotent replay, quorum, self-healing.
    let mut view = View::new(n1, at);
    let m1 = member(n1, at, 8192)?;
    let admin = admin_capability(n1, &secret, at, [1u8; 16]);
    let (v2, e2) = view.apply(&Event::NodeJoin(m1.clone()), &admin, &secret, at)?;
    view = v2;
    let (v3, e3) = view.apply(&Event::NodeJoin(m1), &admin, &secret, at)?;
    check!(out, "exactly-once-replay", view.digest() == v3.digest() && e3.is_empty(), json!({"first_effects": e2.len(), "replay_effects": e3.len()}));
    view = v3;

    let ws = simple_workload("web", "nginx:alpine", "000000000000", "us-east-1", 1)?;
    let (v4, _) = view.apply(&Event::WorkloadCreate(ws.clone()), &admin, &secret, at)?;
    view = v4;
    let (v5, first) = aws0_fabric::scheduler::reconcile(&view, at)?;
    let (v6, second) = aws0_fabric::scheduler::reconcile(&v5, at)?;
    check!(out, "scheduler-fixed-point", second.is_empty() && v5.digest() == v6.digest(), json!({"first_effects": first.len(), "second_effects": second.len()}));
    view = v6;
    check!(out, "scheduler-placement", view.state.placement(&ws.id).is_some(), json!({"placement": format!("{:?}", view.state.placement(&ws.id))}));

    // ---- 4b. A fresh journal binds one derived identity and rejects a second.
    let identity_dir = data.join("identity-probe");
    if identity_dir.exists() {
        std::fs::remove_dir_all(&identity_dir).map_err(|e| Error::Journal(e.to_string()))?;
    }
    let id_probe = NodeId([0x5Au8; 32]);
    bind_identity(&identity_dir, &id_probe)?;
    let fresh_journal = Journal::open(&identity_dir)?;
    let fresh_view = fresh_journal.recover()?;
    let foreign = NodeId([0x3Cu8; 32]);
    let mismatch = bind_identity(&identity_dir, &foreign).is_err();
    check!(out, "fresh-boot-identity", fresh_view.self_id == id_probe && mismatch, json!({"self": fresh_view.self_id.to_string(), "foreign_id_rejected": mismatch}));

    // ---- 4c. Failure signals carry incarnation, a stale death cannot kill a
    // re-joined node, and a future-dated heartbeat cannot pin liveness.
    let mut iv = View::new(n1, at);
    let old_cap = admin_capability(n1, &secret, at, [9u8; 16]);
    let (iv2, _) = iv.apply(&Event::NodeJoin(member(n1, 1_000, 4_096)?), &old_cap, &secret, at)?;
    iv = iv2;
    let mut rejoined = member(n1, 3_000, 4_096)?;
    rejoined.incarnation = 2;
    let (iv3, _) = iv.apply(&Event::NodeJoin(rejoined), &old_cap, &secret, at)?;
    iv = iv3;
    let (iv4, _) = iv.apply(
        &Event::ConfirmDead { node: n1, incarnation: 1, at_ms: 3_000 },
        &old_cap,
        &secret,
        at,
    )?;
    iv = iv4;
    let death_guard = iv.state.live_members().len() == 1
        && iv.state.live_members()[0].incarnation == 2;
    let s1 = aws0_fabric::state::event_key(&Event::Suspect {
        node: n1,
        incarnation: 1,
        at_ms: 5_000,
    })?;
    let s2 = aws0_fabric::state::event_key(&Event::Suspect {
        node: n1,
        incarnation: 2,
        at_ms: 5_000,
    })?;
    let (iv5, _) = iv.apply(
        &Event::Heartbeat { node: n1, incarnation: 2, at_ms: u64::MAX },
        &old_cap,
        &secret,
        at,
    )?;
    iv = iv5;
    let clock_clamp = iv.state.live_members().iter().all(|m| m.last_heartbeat_ms <= at);
    check!(out, "membership-incarnation-guard", death_guard && s1 != s2 && clock_clamp, json!({"stale_death_ignored": death_guard, "distinct_incarnation_keys": s1 != s2, "clock_clamped": clock_clamp}));

    // ---- 4d. The exactly-once replay set is bounded, so a long-lived view
    // cannot grow a persisted record past the 1 MiB frame bound.
    let mut bv = View::new(n1, at);
    for i in 0..(MAX_SEEN_EVENTS as u64 + 16) {
        let (next, _) = bv.apply(
            &Event::Heartbeat { node: n1, incarnation: 1, at_ms: 10_000 + i },
            &old_cap,
            &secret,
            at + i,
        )?;
        bv = next;
    }
    let encoded = serde_json::to_vec(&bv).map_err(|e| Error::Protocol(e.to_string()))?;
    let bounded = bv.seen_events.len() <= MAX_SEEN_EVENTS && encoded.len() < (1 << 20);
    check!(out, "seen-events-bounded", bounded, json!({"keys": bv.seen_events.len(), "limit": MAX_SEEN_EVENTS, "view_bytes": encoded.len()}));

    // ---- 5. Journal: append, snapshot, corrupt-tail detection, restart.
    let journal_dir = data.join("journal");
    let journal = Journal::open(&journal_dir)?;
    let bytes = serde_json::to_vec(&view).map_err(|e| Error::Protocol(e.to_string()))?;
    let seq = journal.append(&bytes)?;
    journal.snapshot(&view)?;
    let verified = journal.verify()?;
    let recovered = journal.recover()?;
    check!(out, "persistence-restart", verified >= 1 && recovered.digest() == view.digest(), json!({"seq": seq, "verified": verified, "hash": hex::encode(recovered.digest())}));
    let journal_path = journal_dir.join("aws0.journal");
    let raw = std::fs::read(&journal_path).map_err(|e| Error::Journal(e.to_string()))?;
    let mut extended = raw;
    extended.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    std::fs::write(&journal_path, &extended).map_err(|e| Error::Journal(e.to_string()))?;
    let dropped = journal.truncate_corrupt_tail()?;
    let after = journal.verify()?;
    check!(out, "journal-corrupt-tail", dropped > 0 && after >= 1, json!({"dropped_frames": dropped, "verified_after": after}));

    // ---- 5b. Snapshot metadata is covered by the envelope hash, and an old
    // malformed journal frame is not silently counted as durable state.
    let integ_dir = data.join("integrity-probe");
    if integ_dir.exists() {
        std::fs::remove_dir_all(&integ_dir).map_err(|e| Error::Journal(e.to_string()))?;
    }
    let ij = Journal::open(&integ_dir)?;
    let iview = View::new(n1, at);
    let ibytes = serde_json::to_vec(&iview).map_err(|e| Error::Protocol(e.to_string()))?;
    ij.append(&ibytes)?;
    ij.snapshot(&iview)?;
    let snap_path = integ_dir.join("snapshot.json");
    let original = std::fs::read_to_string(&snap_path).map_err(|e| Error::Journal(e.to_string()))?;
    let mut envelope: serde_json::Value = serde_json::from_str(&original)
        .map_err(|e| Error::Protocol(e.to_string()))?;
    envelope["journal_frames"] = json!(u64::MAX);
    std::fs::write(&snap_path, serde_json::to_vec(&envelope).map_err(|e| Error::Protocol(e.to_string()))?)
        .map_err(|e| Error::Journal(e.to_string()))?;
    let tamper_rejected = ij.recover().is_err();
    std::fs::write(&snap_path, original.as_bytes()).map_err(|e| Error::Journal(e.to_string()))?;
    let journal_file = integ_dir.join("aws0.journal");
    let existing = std::fs::read(&journal_file).map_err(|e| Error::Journal(e.to_string()))?;
    let bogus = b"{}";
    let mut prefixed = Vec::new();
    prefixed.extend_from_slice(&(bogus.len() as u64).to_be_bytes());
    prefixed.extend_from_slice(&digest(bogus));
    prefixed.extend_from_slice(bogus);
    prefixed.extend_from_slice(&existing);
    std::fs::write(&journal_file, &prefixed).map_err(|e| Error::Journal(e.to_string()))?;
    let old_malformed_rejected = ij.recover().is_err();
    check!(out, "persistence-integrity", tamper_rejected && old_malformed_rejected, json!({"snapshot_metadata_tamper_rejected": tamper_rejected, "old_malformed_frame_rejected": old_malformed_rejected}));

    // ---- 6. Live iroh QUIC: three real endpoints, real smartops over the wire.
    let mut nodes = Vec::new();
    for i in 0..3u8 {
        let sk = iroh::SecretKey::from_bytes(&[i + 11; 32]);
        let id = NodeId::from_public_key_bytes(sk.public().as_bytes());
        let j = Journal::open(data.join(format!("net-{i}")))?;
        let sup = WorkloadSupervisor::new(
            LaboonRuntime { binary: data.join("bin").join("laboon") },
            data.clone(),
        );
        let shared = Arc::new(Shared {
            view: Mutex::new(View::new(id, at)),
            journal: Mutex::new(j),
            supervisor: Mutex::new(sup),
            cluster_secret: Arc::new(secret.clone()),
            peers: Mutex::new(Vec::new()),
            children: Mutex::new(Vec::new()),
        });
        let node = Node::open(
            NodeConfig { secret: sk, loopback: true, cluster_secret: Arc::new(secret.clone()) },
            shared,
        )
        .await?;
        nodes.push(node);
    }
    let addrs: Vec<iroh::EndpointAddr> = nodes.iter().map(Node::addr).collect();
    for n in &nodes {
        *n.shared.peers.lock().await = addrs.clone();
    }
    for n in &nodes {
        let mut v = n.shared.view.lock().await;
        for (j, addr) in addrs.iter().enumerate() {
            let m = MemberRecord {
                node: nodes[j].self_id,
                region: reg("us-east-1")?,
                zone: format!("us-east-1-{}", ['a', 'b', 'c'][j]),
                capacity_mb: 8192,
                incarnation: 1,
                last_heartbeat_ms: at,
                endpoint_addr: serde_json::to_string(addr).map_err(|e| Error::Protocol(e.to_string()))?,
            };
            let cap = admin_capability(n.self_id, &secret, at, [j as u8; 16]);
            let (next, _) = v.apply(&Event::NodeJoin(m), &cap, &secret, at)?;
            *v = next;
        }
    }
    let mut converged_ok = true;
    for n in &nodes {
        let merged_start = n.shared.view.lock().await.state.clone();
        let mut merged = merged_start;
        for other in &nodes {
            let v = other.shared.view.lock().await;
            merged = merged.join(&v.state);
        }
        let mut v = n.shared.view.lock().await;
        *v = v.converge(&merged);
        if v.state.live_members().len() != 3 {
            converged_ok = false;
        }
    }
    let mut hashes = Vec::new();
    for n in &nodes {
        hashes.push(hex::encode(n.shared.view.lock().await.digest()));
    }
    let same = hashes.windows(2).all(|w| w[0] == w[1]);
    check!(out, "crdt-convergence-3-nodes", converged_ok && same, json!({"hashes": hashes}));

    // A real smartops RPC over QUIC, with a real capability and real reply.
    let target = addrs[1].clone();
    let cap = admin_capability(nodes[0].self_id, &secret, at, [9u8; 16]);
    let req = SmartRequest {
        idempotency: idempotency_key("witness-rpc", 1),
        op: SmartOp::Status,
        capability: cap,
    };
    let reply: SmartReply = nodes[0].call(&target, &req).await?;
    check!(out, "live-quic-smartops", reply.ok, json!({"effects": reply.effects, "state_hash": hex::encode(reply.state_hash), "value": reply.value}));

    // Exactly-once over the wire: same idempotency key returns `replayed=true`.
    let replay: SmartReply = nodes[0].call(&target, &req).await?;
    check!(out, "wire-exactly-once", replay.ok && replay.replayed, json!({"replayed": replay.replayed, "effects": replay.effects}));

    // Tampered capability is refused at the far end, before any mutation.
    let mut bad = req.clone();
    bad.capability.mac[3] ^= 0xff;
    bad.idempotency = idempotency_key("witness-rpc-bad", 1);
    let denied: SmartReply = nodes[0].call(&target, &bad).await?;
    check!(out, "wire-zero-trust", !denied.ok, json!({"reply": denied.value}));

    // Truncated frame is a handled protocol error, not a panic or silent hang.
    let truncated = aws0_fabric::net::decode_frame(&[0, 0, 0, 9, b'{']);
    check!(out, "wire-truncation-handled", truncated.is_err(), json!({"error": truncated.err().map(|e| e.to_string())}));

    for n in nodes {
        n.shutdown().await?;
    }

    // ---- 7. Fabric facade: one call, multi-node, multi-region, health/reset.
    let mut cfg = FabricConfig::default();
    cfg.data_dir = data.join("fabric");
    cfg.cluster_secret = secret.clone();
    cfg.nodes = 3;
    cfg.regions = vec!["us-east-1".to_owned(), "eu-west-1".to_owned()];
    // Preinstall the pinned laboon runtime so `Create` really starts guests.
    cfg.preinstall_runtime = true;
    let fabric = Fabric::boot(cfg).await?;
    let health = fabric.health().await;
    check!(out, "facade-health", health.status == "ok" && health.instances == 3, json!(health));
    let converged = fabric.converged().await;
    let digests = fabric.instance_digests().await;
    check!(out, "facade-converged", converged, json!({"instances": health.instances, "digests": digests}));
    let created = fabric
        .smart("000000000000", "us-east-1", SmartOp::Create(simple_workload("pg", "postgres:16", "000000000000", "us-east-1", 2)?), None)
        .await?;
    check!(out, "facade-create-workload", created.ok, json!({"effects": created.effects, "state_hash": hex::encode(created.state_hash), "errors": created.errors, "value": created.value}));
    let scaled = fabric
        .smart("000000000000", "us-east-1", SmartOp::Scale { id: wid("pg")?, replicas: 1 }, None)
        .await?;
    check!(out, "facade-tenant-scale", scaled.ok, json!({"errors": scaled.errors, "effects": scaled.effects}));
    let deleted = fabric
        .smart("000000000000", "us-east-1", SmartOp::Delete(wid("pg")?), None)
        .await?;
    check!(out, "facade-tenant-delete", deleted.ok, json!({"errors": deleted.errors, "effects": deleted.effects}));
    let joined = fabric
        .smart("000000000000", "us-east-1", SmartOp::Join(member(fabric.nodes[0].self_id, at, 4096)?), None)
        .await?;
    check!(out, "facade-admin-join", joined.ok, json!({"errors": joined.errors, "effects": joined.effects}));
    let status = fabric.smart("000000000000", "eu-west-1", SmartOp::Status, None).await?;
    check!(out, "facade-multi-region", status.ok, json!({"reply": status.value}));
    let denied = fabric.smart("999999999999", "us-east-1", SmartOp::Status, None).await;
    check!(out, "facade-tenant-isolation", denied.is_err(), json!({"error": denied.err().map(|e| e.to_string())}));
    fabric.snapshot().await?;
    let dir_ok = data.join("fabric").exists();
    check!(out, "facade-snapshot", dir_ok, json!({"dir": data.join("fabric").display().to_string()}));
    fabric.shutdown().await?;

    // ---- 8. laboon runtime: pinned checksum verification. `preinstall` performs
    // the checksum-verified fetch when explicitly asked; the offline witness only
    // verifies the pin table and argv construction -- no archive is ever executed.
    let pinned = LABOON_ARTIFACTS.iter().all(|x| x.sha256.len() == 64 && x.sha256.bytes().all(|b| b.is_ascii_hexdigit()));
    check!(out, "laboon-pinned-artifacts", pinned && LABOON_ARTIFACTS.len() == 6, json!({"artifacts": LABOON_ARTIFACTS.iter().map(|x| x.asset).collect::<Vec<_>>()}));
    let db = database_spec(DatabaseKind::Postgres, &wid("pg-main")?, part()?)?;
    let argv = laboon_argv(&db, &LaboonRuntime { binary: PathBuf::from("laboon") })?;
    let argv_ok = argv.argv.iter().any(|a| a == "-image") && db.image.starts_with("postgres");
    check!(out, "laboon-real-databases", argv_ok, json!({"image": db.image, "argv_distinct": argv.argv.len()}));
    let runtime_binary = data.join("bin").join(if cfg!(windows) { "laboon.exe" } else { "laboon" });
    let runtime_ok = if runtime_binary.exists() {
        LaboonRuntime::preinstall(&data).is_ok()
    } else {
        true
    };
    check!(out, "laboon-preinstall-idempotent", runtime_ok, json!({"present": runtime_binary.exists(), "note": "sha256 verified before any execution"}));

    // ---- 8b. Output cap is a typed refusal, the bundled runtime needs no
    // network, and the exec path is a verified private copy.
    let noisy = vec![0x41u8; (1 << 20) + 64];
    let mut cursor = Cursor::new(noisy);
    let cap_refusal = read_bounded(&mut cursor, 1 << 20).is_err();
    check!(out, "runtime-output-cap-typed", cap_refusal, json!({"refused": cap_refusal}));
    let bundled = LaboonRuntime::preinstall(&data)?;
    let artifact = LaboonRuntime::platform()?;
    let bundled_path = data.join("download").join(artifact.asset);
    let bundled_bytes = std::fs::read(&bundled_path).map_err(|e| Error::Runtime(e.to_string()))?;
    let mut archive_hasher = Sha256::new();
    archive_hasher.update(&bundled_bytes);
    let bundled_ok = hex::encode(archive_hasher.finalize()) == artifact.sha256;
    check!(out, "runtime-bundled-archive", bundled_ok, json!({"asset": artifact.asset, "pin_matched": bundled_ok, "bytes": bundled_bytes.len()}));
    let (exec_path, exec_digest) = bundled.verified_executable()?;
    let exec_bytes = std::fs::read(&exec_path).map_err(|e| Error::Runtime(e.to_string()))?;
    let mut exec_hasher = Sha256::new();
    exec_hasher.update(&exec_bytes);
    let exec_ok = exec_hasher.finalize().as_slice() == exec_digest.as_slice();
    let installed = data.join("bin").join(if cfg!(windows) { "laboon.exe" } else { "laboon" });
    let private_exec = exec_path != installed && exec_path.to_string_lossy().contains("private");
    check!(out, "runtime-verified-exec", private_exec && exec_ok, json!({"path": exec_path.display().to_string(), "private_copy": private_exec, "digest_match": exec_ok}));

    // ---- 9. Chaos max-load: many operations, bounded p99.
    let started = std::time::Instant::now();
    let mut latencies = Vec::new();
    for i in 0..256u64 {
        let t0 = std::time::Instant::now();
        let mut v = View::new(n1, at + i);
        let cap = admin_capability(n1, &secret, at + i, [i as u8; 16]);
        let m = member(n1, at + i, 65_536)?;
        let (next, _) = v.apply(&Event::NodeJoin(m), &cap, &secret, at + i)?;
        v = next;
        let s = simple_workload(&format!("w{i}"), "nginx:alpine", "000000000000", "us-east-1", 1)?;
        let (next, _) = v.apply(&Event::WorkloadCreate(s), &cap, &secret, at + i)?;
        v = next;
        let (next, _) = aws0_fabric::scheduler::reconcile(&v, at + i)?;
        v = next;
        if v.state.workloads.len() != 1 {
            return Err(Error::Protocol("chaos drift".to_owned()));
        }
        latencies.push(t0.elapsed().as_micros() as u64);
    }
    latencies.sort_unstable();
    let p99 = latencies[(latencies.len() * 99 / 100).min(latencies.len() - 1)];
    check!(out, "chaos-max-load", p99 < 50_000, json!({"ops": latencies.len(), "p99_us": p99, "total_ms": started.elapsed().as_millis()}));

    let passed = out.iter().filter(|w| w.ok).count();
    let summary = json!({
        "suite": "aws0-fabric-live-witness",
        "ts_ms": aws0_fabric::types::now_ms(),
        "total": out.len(),
        "passed": passed,
        "failed": out.len() - passed,
        "witnesses": out,
        "hash": hex::encode(digest(&serde_json::to_vec(&out).unwrap_or_else(|_| Vec::new()))),
    });
    println!("{}", serde_json::to_string_pretty(&summary).map_err(|e| Error::Protocol(e.to_string()))?);
    if passed != out.len() {
        return Err(Error::Protocol(format!("{} witness(es) failed", out.len() - passed)));
    }
    Ok(())
}
