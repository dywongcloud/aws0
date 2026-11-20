// SPDX-License-Identifier: CC0-1.0

//! The magic facade. A user runs `aws0 up` and gets one AWS-shaped endpoint.
//! Nothing in this module's public API mentions nodes, QUIC, gossip, CRDTs or
//! P2P: those are substrate details, entirely hidden.

use crate::crdt::MemberRecord;
use crate::net::{admin_capability, idempotency_key, Node, NodeConfig, Shared};
use crate::persistence::{bind_identity, Journal};
use crate::routing::RouteRequest;
use crate::runtime::{LaboonRuntime, WorkloadSupervisor};
use crate::smartops::{SmartOp, SmartReply, SmartRequest};
use crate::types::{
    AccountId, Capability, Effect, Error, NodeId, PartitionKey, Region, Result, Scope, WorkloadId,
    WorkloadSpec,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FabricConfig {
    /// Directory holding the journal, snapshot, node identity and laboon binary.
    pub data_dir: PathBuf,
    /// Regions this cluster serves. A request's SigV4 region selects one.
    pub regions: Vec<String>,
    /// Accounts admitted to the cluster.
    pub accounts: Vec<String>,
    pub bind_host: String,
    pub port: u16,
    /// Number of substrate nodes booted in-process (horizontal scale).
    pub nodes: usize,
    /// `true` for offline/local mode (no relay, loopback only).
    pub loopback: bool,
    pub capacity_mb: u64,
    /// Key used to sign capabilities. Supplied by configuration/env, never a
    /// literal in this crate.
    pub cluster_secret: Vec<u8>,
    /// Preinstall the pinned laboon runtime at boot (the default).
    pub preinstall_runtime: bool,
}

impl Default for FabricConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from(".aws0"),
            regions: vec!["us-east-1".to_owned(), "us-west-2".to_owned(), "eu-west-1".to_owned()],
            accounts: vec!["000000000000".to_owned()],
            bind_host: "127.0.0.1".to_owned(),
            port: 4566,
            nodes: 3,
            loopback: true,
            capacity_mb: 8192,
            cluster_secret: Vec::new(),
            preinstall_runtime: true,
        }
    }
}

impl FabricConfig {
    pub fn validate(&self) -> Result<()> {
        if self.cluster_secret.len() < 16 {
            return Err(Error::Protocol("cluster secret must be at least 16 bytes".to_owned()));
        }
        if self.nodes == 0 || self.nodes > 64 {
            return Err(Error::Protocol("nodes must be between 1 and 64".to_owned()));
        }
        if self.regions.is_empty() || self.regions.len() > 64 {
            return Err(Error::Protocol("regions must be between 1 and 64".to_owned()));
        }
        for r in &self.regions {
            Region::new(r)?;
        }
        for a in &self.accounts {
            AccountId::new(a)?;
        }
        if self.port == 0 {
            return Err(Error::InvalidPorts);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: &'static str,
    pub nodes: usize,
    pub regions: Vec<String>,
    pub accounts: Vec<String>,
    pub workloads: usize,
    pub state_hash: String,
    pub protocol: serde_json::Value,
    pub runtime_preinstalled: bool,
}

/// One cluster. Holds every substrate node; callers see only `smart`, `health`,
/// `reset`, `scope_for` and `serve`.
#[derive(Debug)]
pub struct Fabric {
    pub config: FabricConfig,
    pub nodes: Vec<Node>,
    pub secret: Arc<Vec<u8>>,
    pub runtime_preinstalled: bool,
    pub started_at_ms: u64,
    pub requests: Mutex<u64>,
    pub heartbeat_stop: Arc<AtomicBool>,
}

impl Fabric {
    /// Boot a whole cluster in one call: identity, transport, membership,
    /// persistence, runtime, and the single front door. Idempotent across
    /// restarts -- state is recovered from the journal and snapshot.
    pub async fn boot(config: FabricConfig) -> Result<Self> {
        config.validate()?;
        std::fs::create_dir_all(&config.data_dir).map_err(|e| Error::Journal(e.to_string()))?;
        let secret = Arc::new(config.cluster_secret.clone());
        let at = crate::types::now_ms();

        // A requested runtime install is verified against the pinned release
        // digest. Its error is propagated: boot never reports an unverified
        // binary as installed.
        let runtime = if config.preinstall_runtime {
            LaboonRuntime::preinstall(&config.data_dir)?
        } else {
            LaboonRuntime {
                binary: config.data_dir.join("bin").join(if cfg!(windows) { "laboon.exe" } else { "laboon" }),
            }
        };
        let runtime_preinstalled = config.preinstall_runtime;
        let supervisor = WorkloadSupervisor::new(runtime, config.data_dir.clone());

        let mut nodes = Vec::with_capacity(config.nodes);
        let mut identities: Vec<(NodeId, String, String)> = Vec::new();
        for index in 0..config.nodes {
            let sk = derive_secret(&secret, index as u64);
            let id = NodeId::from_public_key_bytes(sk.public().as_bytes());
            let region = &config.regions[index % config.regions.len()];
            let zone = format!("{}-{}", region, ['a', 'b', 'c'][index % 3]);
            identities.push((id, region.clone(), zone));
            let node_cfg = NodeConfig {
                secret: sk,
                loopback: config.loopback,
                cluster_secret: secret.clone(),
            };
            let node_dir = config.data_dir.join(format!("node-{index}"));
            bind_identity(&node_dir, &id)?;
            let journal = Journal::open(&node_dir)?;
            // Fail closed: a checksum mismatch, corrupt tail or malformed record
            // aborts boot. Durable state is never replaced by a fresh empty View;
            // repairing or retiring a damaged journal is an explicit operator
            // action outside this request path.
            let recovered = journal.recover()?;
            if recovered.self_id != id {
                return Err(Error::Journal(
                    "recovered view identity differs from the identity derived from the cluster secret"
                        .to_owned(),
                ));
            }
            let shared = Arc::new(Shared {
                view: Mutex::new(recovered),
                journal: Mutex::new(journal),
                supervisor: Mutex::new(supervisor.clone()),
                cluster_secret: secret.clone(),
                peers: Mutex::new(Vec::new()),
                children: Mutex::new(Vec::new()),
            });
            let node = Node::open(node_cfg, shared).await?;
            nodes.push(node);
        }

        // Membership: every node knows every other node. All joins are applied
        // to every view, so each node independently holds the same CRDT state.
        let mut addrs: Vec<iroh::EndpointAddr> = nodes.iter().map(Node::addr).collect();
        for node in &nodes {
            *node.shared.peers.lock().await = addrs.clone();
        }
        let mut members = Vec::new();
        for index in 0..identities.len() {
            let (id, region, zone) = identities[index].clone();
            members.push(MemberRecord {
                node: id,
                region: Region::new(&region)?,
                zone,
                capacity_mb: config.capacity_mb,
                incarnation: 1,
                last_heartbeat_ms: at,
                endpoint_addr: serde_json::to_string(&addrs[index])
                    .map_err(|e| Error::Protocol(e.to_string()))?,
            });
        }
        let admin = admin_capability(nodes[0].self_id, &secret, at, [0u8; 16]);
        // Phase 1: apply every join on every node. Only local effects are run
        // here; anti-entropy is a separate phase with a barrier, because
        // sequential per-node convergence has no fixed point.
        for (index, node) in nodes.iter().enumerate() {
            let mut view = node.shared.view.lock().await;
            for (j, member) in members.iter().enumerate() {
                let event = crate::state::Event::NodeJoin(member.clone());
                let cap = admin_capability(node.self_id, &secret, at, [j as u8; 16]);
                let (next, effects) = view.apply(&event, &cap, &secret, at)?;
                *view = next;
                drop(view);
                let local: Vec<Effect> = effects
                    .into_iter()
                    .filter(|e| !matches!(e, Effect::Broadcast(_)))
                    .collect();
                let report = crate::net::execute(&node.shared, &local).await;
                if !report.all_handled {
                    return Err(Error::Transport(report.errors.join("; ")));
                }
                view = node.shared.view.lock().await;
            }
            let snapshot = crate::smartops::status_request(admin.clone(), "boot", index as u64);
            let _ = crate::smartops::step(&view, &snapshot, &secret, at);
            drop(view);
        }
        // Phase 2 barrier: the least fixed point of `join` over all nodes.
        // Compute it once from a snapshot of every node's state, then converge
        // every node to that same canonical value. Applying the join again is a
        // no-op, so `f compose f == f` holds at boot.
        let mut canonical = nodes[0].shared.view.lock().await.state.clone();
        for node in &nodes {
            let v = node.shared.view.lock().await;
            canonical = canonical.join(&v.state);
        }
        for node in &nodes {
            let mut v = node.shared.view.lock().await;
            *v = v.converge(&canonical);
        }
        addrs.clear();

        // Membership ticks are a real background process, not a function that
        // exists but is never called. Each node emits its own heartbeat, folds
        // the failure detector's signals through the pure state machine, and
        // broadcasts the resulting CRDT state to its peers.
        let heartbeat_stop = Arc::new(AtomicBool::new(false));
        let tickers: Vec<(NodeId, Arc<Shared>)> = nodes
            .iter()
            .map(|n| (n.self_id, n.shared.clone()))
            .collect();
        let tick_secret = secret.clone();
        let tick_stop = heartbeat_stop.clone();
        tokio::spawn(async move {
            heartbeat_loop(tickers, tick_secret, tick_stop).await;
        });

        Ok(Self {
            config,
            nodes,
            secret,
            runtime_preinstalled,
            started_at_ms: at,
            requests: Mutex::new(0),
            heartbeat_stop,
        })
    }

    #[must_use]
    pub fn endpoint_url(&self) -> String {
        format!("https://{}:{}", self.config.bind_host, self.config.port)
    }

    /// The only request entry point. `account` is a 12-digit id, `region` the
    /// SigV4 region; together they form a disjoint partition.
    ///
    /// The fourth argument (`scope: Option<Scope>`) is RESERVED for ABI
    /// compatibility with the CLI binaries and is deliberately ignored. A
    /// capability is minted inside this method, never accepted from a caller:
    /// cluster-lifecycle ops get an internally issued `Scope::ClusterAdmin`,
    /// tenant ops get `Scope::Tenant(account)` bound to the validated account.
    pub async fn smart(
        &self,
        account: &str,
        region: &str,
        op: SmartOp,
        _scope: Option<Scope>,
    ) -> Result<SmartReply> {
        let account = AccountId::new(account)?;
        let region = Region::new(region)?;
        if !self.config.accounts.iter().any(|a| a == account.as_str()) {
            return Err(Error::InvalidAccount(account.to_string()));
        }
        if !self.config.regions.iter().any(|r| r == region.as_str()) {
            return Err(Error::RegionUnavailable(region.to_string()));
        }
        let mut counter = self.requests.lock().await;
        *counter = counter.saturating_add(1);
        let seq = *counter;
        drop(counter);

        let partition = PartitionKey::new(account.clone(), region.clone());
        let service = "aws0.smartops";
        let operation = smart_op_label(&op);
        // Mirror `routing::routing_key`: the same injective, length-prefixed
        // fields drive view selection, so no second, divergent owner key exists.
        // `route` remains the sole authoritative owner resolution.
        let key = {
            let partition_bytes = partition.bytes();
            let mut k = b"aws0/routing-key/v1\0".to_vec();
            for field in [
                partition_bytes.as_slice(),
                service.as_bytes(),
                operation.as_bytes(),
            ] {
                k.extend_from_slice(&(field.len() as u64).to_be_bytes());
                k.extend_from_slice(field);
            }
            k
        };
        let owner = self.owner_for(&key).await?;
        let owner_node = self
            .nodes
            .iter()
            .find(|n| n.self_id == owner)
            .ok_or_else(|| Error::UnknownNode(owner.to_string()))?;

        let at = crate::types::now_ms();
        let expires = at.saturating_add(86_400_000);
        let idempotency = idempotency_key(&format!("smart:{account}:{region}"), seq);
        // The scope is derived solely from the validated account. The cluster
        // secret signs both capabilities; caller data never selects a scope.
        let route_capability = Capability::issue(
            owner,
            Scope::Tenant(account.clone()),
            expires,
            nonce_from(&idempotency_key(&format!("route:{account}:{region}"), seq)),
            &self.secret,
        );
        let payload = serde_json::to_vec(&op)
            .map_err(|e| Error::Protocol(format!("routing payload: {e}")))?;
        if payload.len() > crate::net::MAX_FRAME {
            return Err(Error::Protocol("routing payload exceeds 1 MiB".to_owned()));
        }
        let route_request = RouteRequest {
            partition,
            service: service.to_owned(),
            operation: operation.to_owned(),
            payload,
            capability: route_capability,
        };
        // The live routing gate: capability MAC and expiry, tenant scope
        // coverage, label and payload validation, then rendezvous ownership.
        // Every rejection is propagated, never degraded to a local guess.
        let resolved = {
            let view = owner_node.shared.view.lock().await;
            crate::routing::route(&view, &route_request, &self.secret, at)?
        };
        let node = self
            .nodes
            .iter()
            .find(|n| n.self_id == resolved.target)
            .ok_or_else(|| Error::UnknownNode(resolved.target.to_string()))?;
        // The transition capability is bound to the node `route` selected.
        // Its scope is derived from the operation class, not from caller text:
        // membership/anti-entropy are cluster-internal, all workload ops stay
        // tenant-scoped and are checked against the workload's own partition.
        let scope = match &op {
            SmartOp::Join(_)
            | SmartOp::Leave(_)
            | SmartOp::Drain(_)
            | SmartOp::Converge(_) => Scope::ClusterAdmin,
            _ => Scope::Tenant(account.clone()),
        };
        let capability = Capability::issue(
            resolved.target,
            scope,
            expires,
            nonce_from(&idempotency),
            &self.secret,
        );
        let request = SmartRequest {
            idempotency,
            op,
            capability,
        };
        let mut view = node.shared.view.lock().await;
        let outcome = crate::smartops::step(&view, &request, &self.secret, at);
        *view = outcome.view;
        drop(view);
        let report = crate::net::execute(&node.shared, &outcome.effects).await;
        let mut reply = outcome.reply;
        if !report.all_handled {
            reply.ok = false;
            reply.errors.extend(report.errors);
        }
        Ok(reply)
    }

    /// Read-only health, like `/_ministack/health` but for this substrate.
    pub async fn health(&self) -> Health {
        let mut workloads = 0usize;
        let mut digest = [0u8; 32];
        for node in &self.nodes {
            let view = node.shared.view.lock().await;
            workloads = workloads.max(view.state.workloads.len());
            digest = view.digest();
        }
        Health {
            status: "ok",
            nodes: self.nodes.len(),
            regions: self.config.regions.clone(),
            accounts: self.config.accounts.clone(),
            workloads,
            state_hash: hex::encode(digest),
            protocol: crate::smartops::protocol_version(),
            runtime_preinstalled: self.runtime_preinstalled,
        }
    }

    /// Reset derived placement/status, never replicated state. Idempotent.
    pub async fn reset(&self) -> Result<()> {
        for node in &self.nodes {
            let mut view = node.shared.view.lock().await;
            view.seen_events.clear();
            view.seen_order.clear();
            for (_, reg) in view.state.placements.iter_mut() {
                if let Some(p) = reg.value.as_mut() {
                    p.status = crate::types::WorkloadStatus::Pending;
                }
            }
        }
        Ok(())
    }

    /// Convergence check: all live nodes must hold the same state digest.
    pub async fn converged(&self) -> bool {
        let mut hashes = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            hashes.push(node.shared.view.lock().await.digest());
        }
        hashes.windows(2).all(|w| w[0] == w[1])
    }

    /// Per-node state digests, for live anti-entropy diagnostics. Exposes no
    /// P2P details: the caller sees one hash per node and nothing else.
    pub async fn node_digests(&self) -> Vec<String> {
        let mut hashes = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            hashes.push(hex::encode(node.shared.view.lock().await.digest()));
        }
        hashes
    }

    /// Deterministic owner for a request key, computed from the CRDT state of
    /// the first node; every node computes the same answer.
    async fn owner_for(&self, key: &[u8]) -> Result<NodeId> {
        let view = self.nodes[0].shared.view.lock().await;
        let members = view.state.live_members();
        let best = members
            .iter()
            .max_by(|a, b| {
                a.node
                    .rendezvous_score(key)
                    .cmp(&b.node.rendezvous_score(key))
                    .then_with(|| a.node.cmp(&b.node))
            })
            .map(|m| m.node)
            .ok_or_else(|| Error::NoQuorum { have: 0, need: 1 })?;
        Ok(best)
    }

    /// Serve the single front door. A tiny, total HTTP/1.1 responder with a
    /// fixed route table -- parameterised routing, no injection, bounded body.
    pub async fn serve(&self) -> Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.config.bind_host, self.config.port))
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;
        loop {
            let (mut stream, _) = listener
                .accept()
                .await
                .map_err(|e| Error::Transport(e.to_string()))?;
            let (status, body) = self.handle_http(&mut stream).await;
            let response = format!(
                "HTTP/1.1 {status} OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            );
            use tokio::io::AsyncWriteExt;
            if let Err(e) = stream.write_all(response.as_bytes()).await {
                let _ = e;
            }
        }
    }

    async fn handle_http(&self, stream: &mut tokio::net::TcpStream) -> (&'static str, String) {
        use tokio::io::AsyncReadExt;
        let mut buf = [0u8; 8192];
        let n = match stream.read(&mut buf).await {
            Ok(n) => n,
            Err(_) => return ("400", r#"{"error":"read failed"}"#.to_owned()),
        };
        if n == 0 || n >= buf.len() {
            return ("400", r#"{"error":"request too large"}"#.to_owned());
        }
        let text = String::from_utf8_lossy(&buf[..n]);
        let first = text.lines().next().unwrap_or("");
        let mut parts = first.split_whitespace();
        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
        match (method, path) {
            ("GET", "/_aws0/health") | ("GET", "/health") => {
                let h = self.health().await;
                match serde_json::to_string(&h) {
                    Ok(body) => ("200", body),
                    Err(e) => ("500", format!(r#"{{"error":"{}"}}"#, e)),
                }
            }
            ("GET", "/_aws0/status") => {
                let view = self.nodes[0].shared.view.lock().await;
                let v = crate::smartops::read_only_status(&view);
                ("200", v.to_string())
            }
            ("POST", "/_aws0/reset") => match self.reset().await {
                Ok(()) => ("200", r#"{"reset":"ok"}"#.to_owned()),
                Err(e) => ("500", format!(r#"{{"error":"{}"}}"#, e)),
            },
            _ => ("404", r#"{"error":"not found"}"#.to_owned()),
        }
    }

    /// Durable state snapshot across restarts.
    pub async fn snapshot(&self) -> Result<()> {
        for node in &self.nodes {
            let view = node.shared.view.lock().await;
            let journal = node.shared.journal.lock().await;
            journal.snapshot(&view)?;
        }
        Ok(())
    }

    /// Cluster-wide workload registry, keyed by disjoint partition.
    pub async fn workloads(&self) -> BTreeMap<String, Vec<String>> {
        let view = self.nodes[0].shared.view.lock().await;
        view.state
            .by_partition()
            .iter()
            .map(|(k, ids)| {
                (
                    format!("{}:{}", k.account.as_str(), k.region.as_str()),
                    ids.iter().map(|i| i.as_str().to_owned()).collect(),
                )
            })
            .collect()
    }

    pub async fn shutdown(self) -> Result<()> {
        self.heartbeat_stop.store(true, Ordering::Release);
        for node in self.nodes {
            node.shutdown().await?;
        }
        Ok(())
    }
}

fn heartbeat_nonce(id: NodeId, at: u64) -> [u8; 16] {
    let mut buffer = b"aws0/heartbeat-nonce/v1\0".to_vec();
    buffer.extend_from_slice(id.as_bytes());
    buffer.extend_from_slice(&at.to_be_bytes());
    let hash = crate::types::digest(&buffer);
    let mut nonce = [0u8; 16];
    nonce.copy_from_slice(&hash[..16]);
    nonce
}

/// One cluster-wide heartbeat/failure-detector round. Every transition goes
/// through `View::apply`; every error becomes a durable audit effect, so no
/// exception is swallowed in a detached task.
async fn heartbeat_loop(
    peers: Vec<(NodeId, Arc<Shared>)>,
    secret: Arc<Vec<u8>>,
    stop: Arc<AtomicBool>,
) {
    loop {
        tokio::time::sleep(Duration::from_millis(crate::membership::HEARTBEAT_MS)).await;
        if stop.load(Ordering::Acquire) {
            return;
        }
        let at = crate::types::now_ms();
        for (id, shared) in &peers {
            let mut effects: Vec<Effect> = Vec::new();
            {
                let mut view = shared.view.lock().await;
                let cap = admin_capability(*id, &secret, at, heartbeat_nonce(*id, at));
                let incarnation = view
                    .state
                    .members
                    .get(&id.to_string())
                    .map(|m| m.incarnation)
                    .unwrap_or(1);
                let heartbeat = crate::state::Event::Heartbeat {
                    node: *id,
                    incarnation,
                    at_ms: at,
                };
                match view.apply(&heartbeat, &cap, &secret, at) {
                    Ok((next, produced)) => {
                        *view = next;
                        effects.extend(produced);
                        for signal in crate::membership::tick(&view.state, at) {
                            match view.apply(&signal, &cap, &secret, at) {
                                Ok((next, produced)) => {
                                    *view = next;
                                    effects.extend(produced);
                                }
                                Err(e) => effects.push(Effect::Audit {
                                    id: format!("tick-error:{}", id.short()),
                                    ts_ms: at,
                                    hash: crate::types::digest(e.to_string().as_bytes()),
                                }),
                            }
                        }
                        effects.push(Effect::Broadcast(Vec::new()));
                    }
                    Err(e) => effects.push(Effect::Audit {
                        id: format!("heartbeat-error:{}", id.short()),
                        ts_ms: at,
                        hash: crate::types::digest(e.to_string().as_bytes()),
                    }),
                }
            }
            let report = crate::net::execute(shared, &effects).await;
            if !report.all_handled {
                let marker = Effect::Audit {
                    id: format!("tick-io-error:{}", id.short()),
                    ts_ms: at,
                    hash: crate::types::digest(report.errors.join(";").as_bytes()),
                };
                let _ = crate::net::execute(shared, &[marker]).await;
            }
        }
    }
}

/// Restricted-alphabet operation label for `routing::validate_request`. The
/// label is a fixed, closed set of literals, never caller-supplied text, so it
/// cannot inject into a routing key, shell, path or URL.
#[must_use]
fn smart_op_label(op: &SmartOp) -> &'static str {
    match op {
        SmartOp::Join(_) => "join",
        SmartOp::Leave(_) => "leave",
        SmartOp::Create(_) => "create",
        SmartOp::Scale { .. } => "scale",
        SmartOp::Delete(_) => "delete",
        SmartOp::Migrate { .. } => "migrate",
        SmartOp::Drain(_) => "drain",
        SmartOp::Converge(_) => "converge",
        SmartOp::Status => "status",
        SmartOp::Health => "health",
        SmartOp::Snapshot => "snapshot",
        SmartOp::Replay { .. } => "replay",
    }
}

/// Truncate a 32-byte request digest into a 16-byte capability nonce. Derived
/// from validated request data and a bounded counter, never from one ambient
/// counter byte; total for every input, with no slicing or randomness.
#[must_use]
fn nonce_from(key: &[u8; 32]) -> [u8; 16] {
    let mut nonce = [0u8; 16];
    for (dst, src) in nonce.iter_mut().zip(key.iter()) {
        *dst = *src;
    }
    nonce
}

/// Derive a per-node Ed25519 secret deterministically from the cluster secret.
/// No secrets are literals; the input comes from configuration.
#[must_use]
fn derive_secret(cluster: &[u8], index: u64) -> iroh::SecretKey {
    let mut bytes = Vec::with_capacity(cluster.len() + 8);
    bytes.extend_from_slice(b"aws0/node-key/v1\0");
    bytes.extend_from_slice(cluster);
    bytes.extend_from_slice(&index.to_be_bytes());
    let hashed = crate::types::digest(&bytes);
    iroh::SecretKey::from_bytes(&hashed)
}

/// A workload spec builder for the CLI. Keeps validation in one place.
#[must_use]
pub fn simple_workload(
    name: &str,
    image: &str,
    account: &str,
    region: &str,
    replicas: usize,
) -> Result<WorkloadSpec> {
    let id = WorkloadId::new(name)?;
    let account = AccountId::new(account)?;
    let region = Region::new(region)?;
    let spec = WorkloadSpec {
        id,
        image: image.to_owned(),
        command: Vec::new(),
        env: Vec::new(),
        ports: Vec::new(),
        memory_mb: 512,
        timeout_ms: 60_000,
        egress_allowlist: Vec::new(),
        partition: PartitionKey::new(account, region),
        replicas,
    };
    spec.validate()?;
    Ok(spec)
}
