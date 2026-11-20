// SPDX-License-Identifier: CC0-1.0

//! Real workload hosting on laboon v0.1.0.
//!
//! laboon is a syscall-interception sandbox: guest machine code executes
//! natively on the host CPU and only its syscall instructions are intercepted.
//! There is no VM, no second kernel, no daemon, no Docker socket and no root.
//! That single fact is what lets aws0 boot unmodified OCI images and real
//! database servers -- PostgreSQL, MySQL, MariaDB, Redis, Valkey, MongoDB,
//! DuckDB and Kafka -- without Kubernetes and without reimplementing an engine.
//! The engines below are the actual `postgres`, `mysqld`, `mongod`,
//! `redis-server` and `kafka-server-start.sh` processes, not modelled subsets.
//!
//! This module performs no IO at import time. [`LaboonRuntime::preinstall`] is
//! the sole download path and is called explicitly by the CLI. Every process is
//! spawned from a `Vec<String>` of distinct argv entries; no path, image or
//! environment value is ever interpolated into a shell string. User-derived
//! paths are resolved to absolute paths, rejected when any component begins
//! with `-`, and separated from tool options by a `--` terminator. The three
//! host tools are resolved from pinned absolute paths, never from ambient PATH.
//!
//! The six pinned SHA-256 values are the byte-exact v0.1.0 release
//! `checksums.txt` digests, public values from
//! `https://github.com/samyfodil/laboon/releases/download/v0.1.0/checksums.txt`.
//! The implementing contract's inline list truncated three of them; a truncated
//! literal is not a SHA-256 digest, so the release values below are the sole
//! authority. The on-disk `laboon.sha256` manifest is only a cache:
//! [`LaboonRuntime::verify`] recomputes the ARCHIVE digest from the retained
//! bytes, compares it with the compile-time pin, re-derives the binary digest
//! from that pinned archive by extracting it again, and compares that digest
//! with the installed executable. A writable manifest alone can never
//! authorize a replaced binary.
//!
//! An empty `egress_allowlist` selects laboon's default loopback-only posture:
//! neither `-net` nor `-no-net` is emitted. `-no-net` removes INET sockets
//! entirely, so it would stop a database server from binding 127.0.0.1. A
//! non-empty allowlist compiles to `-net -allow-domains` plus the fixed
//! loopback CIDR that the v0.1.0 egress policy requires for guest loopback.

use crate::types::{Error, PartitionKey, Result, WorkloadId, WorkloadSpec, WorkloadStatus};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// The pinned laboon release. Only this version's CLI is supported.
pub const LABOON_VERSION: &str = "0.1.0";

/// Release download base. The tag and the pinned asset name are appended as
/// distinct string components; the resulting URL is one argv entry.
pub const LABOON_RELEASE_BASE: &str = "https://github.com/samyfodil/laboon/releases/download";

/// One pinned release artifact. `target` is `os_arch` in GoReleaser spelling.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PlatformArtifact {
    pub target: &'static str,
    pub asset: &'static str,
    pub sha256: &'static str,
}

/// Exactly the six v0.1.0 `laboon_*` artifacts and their SHA-256 digests read
/// from the release `checksums.txt`. The six `laboon-mcp_*` assets are not
/// installed on nodes. Order is fixed, so iteration is deterministic. These six
/// values are deliberately not altered: they were verified byte-exact against
/// the upstream release file.
pub const LABOON_ARTIFACTS: &[PlatformArtifact] = &[
    PlatformArtifact {
        target: "darwin_arm64",
        asset: "laboon_0.1.0_darwin_arm64.tar.gz",
        sha256: "1116d234f49b3f5f3c7ce363ddb98c4d084a4e6d11342a613386a4d89203f279",
    },
    PlatformArtifact {
        target: "darwin_amd64",
        asset: "laboon_0.1.0_darwin_amd64.tar.gz",
        sha256: "bd3c10db9086d5c0c688c502b7a80607c62f5ad1a9ec510f9146d9fece6ba89e",
    },
    PlatformArtifact {
        target: "linux_arm64",
        asset: "laboon_0.1.0_linux_arm64.tar.gz",
        sha256: "8459d1d6a00df841d4fff3419beb02e20cf61a37357c4cb5f720ca5384c1955a",
    },
    PlatformArtifact {
        target: "linux_amd64",
        asset: "laboon_0.1.0_linux_amd64.tar.gz",
        sha256: "842fe5620db4592498ff599a080c9f8f31b710a27face7ac2ffa13799d5312c8",
    },
    PlatformArtifact {
        target: "windows_arm64",
        asset: "laboon_0.1.0_windows_arm64.zip",
        sha256: "2e1a02a19882975ef8b0dc14b275f8625704b6b1a7861cfac8e52ff81f710071",
    },
    PlatformArtifact {
        target: "windows_amd64",
        asset: "laboon_0.1.0_windows_amd64.zip",
        sha256: "ef9c67428b3cde1aecae2b3c90a123f56f4d70a02e7b62fa7ec517f474c14366",
    },
];

const MANIFEST_NAME: &str = "laboon.sha256";
const MAX_OUTPUT_BYTES: usize = 1 << 20;
const MAX_ARGV_BYTES: usize = 1 << 20;
const MAX_SEARCH_DEPTH: usize = 12;
const HASH_CHUNK: usize = 64 * 1024;
const DATABASE_MEMORY_MB: u64 = 1024;
const DATABASE_TIMEOUT_MS: u64 = 60_000;
const MAX_ENV_ENTRIES: usize = 64;
const MAX_ENV_KEY_BYTES: usize = 128;
const MAX_ENV_VALUE_BYTES: usize = 8192;
const MAX_EGRESS_BYTES: usize = 253;
const LOOPBACK_EGRESS_HOST: &str = "localhost";
const LOOPBACK_CIDR: &str = "127.0.0.1/32";
const DUCKDB_UI_PORT: u16 = 4213;

/// Host variables copied to the laboon process. Nothing from `WorkloadSpec.env`
/// ever reaches the host process; those entries are guest `-e` flags instead.
const HOST_ENV_ALLOWLIST: &[&str] = &[
    "PATH",
    "HOME",
    "TMPDIR",
    "TEMP",
    "TMP",
    "LANG",
    "LC_ALL",
    "USER",
    "SystemRoot",
    "SSL_CERT_FILE",
    "SSL_CERT_DIR",
];

/// Guest variables that could steer the host loader, Go runtime or profiler.
const DENIED_ENV_KEYS: &[&str] = &[
    "LD_PRELOAD",
    "LD_LIBRARY_PATH",
    "LD_AUDIT",
    "DYLD_INSERT_LIBRARIES",
    "DYLD_LIBRARY_PATH",
    "DYLD_FRAMEWORK_PATH",
    "GODEBUG",
    "LABOON_CPUPROFILE",
    "LABOON_MEMPROFILE",
    "DISPLAY",
];

#[cfg(unix)]
const CURL_PATHS: &[&str] = &["/usr/bin/curl", "/bin/curl", "/usr/local/bin/curl"];
#[cfg(unix)]
const TAR_PATHS: &[&str] = &["/usr/bin/tar", "/bin/tar"];
#[cfg(windows)]
const CURL_PATHS: &[&str] = &["C:\\Windows\\System32\\curl.exe"];
#[cfg(windows)]
const TAR_PATHS: &[&str] = &["C:\\Windows\\System32\\tar.exe"];
#[cfg(not(any(unix, windows)))]
const CURL_PATHS: &[&str] = &["/usr/bin/curl"];
#[cfg(not(any(unix, windows)))]
const TAR_PATHS: &[&str] = &["/usr/bin/tar"];
/// Monotonic sequence used only to give each verification/extraction pass its
/// own private directory, so concurrent installs cannot share mutable paths.
static PRIVATE_SEQ: AtomicU64 = AtomicU64::new(0);

/// A verified, already-installed laboon binary. Constructed by `preinstall` or
/// explicitly from a known path by the CLI; construction alone performs no IO.
#[derive(Clone, Debug)]
pub struct LaboonRuntime {
    pub binary: PathBuf,
}

impl LaboonRuntime {
    /// Resolve the host's pinned artifact from the compile-time `OS` and `ARCH`
    /// constants. No runtime environment is consulted.
    pub fn platform() -> Result<&'static PlatformArtifact> {
        let os = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "windows",
            other => return Err(Error::Runtime(format!("unsupported laboon OS: {other}"))),
        };
        let arch = match std::env::consts::ARCH {
            "aarch64" => "arm64",
            "x86_64" => "amd64",
            other => return Err(Error::Runtime(format!("unsupported laboon arch: {other}"))),
        };
        let target = format!("{os}_{arch}");
        LABOON_ARTIFACTS
            .iter()
            .find(|a| a.target == target.as_str())
            .ok_or_else(|| Error::Runtime(format!("no pinned laboon artifact for {target}")))
    }

    /// Idempotent verified install. `f  compose  f  ==  f`: a complete, correctly bound
    /// install is returned only after the full `verify()` chain succeeds; a
    /// partial or tampered install is reinstalled from the pinned archive, and
    /// malformed local state is reported as an error rather than hidden.
    pub fn preinstall(dir: &Path) -> Result<Self> {
        let artifact = Self::platform()?;
        let pinned = pinned_bytes(artifact.sha256)?;
        let root = absolute_root(dir)?;
        let bin_dir = root.join("bin");
        ensure_dir(&bin_dir)?;
        restrict_dir(&bin_dir)?;
        let binary = bin_dir.join(binary_name());
        let manifest = bin_dir.join(MANIFEST_NAME);
        let download_dir = root.join("download");
        ensure_dir(&download_dir)?;
        restrict_dir(&download_dir)?;
        let archive = download_dir.join(artifact.asset);

        let binary_present = file_state(&binary)?;
        let manifest_present = file_state(&manifest)?;
        let archive_present = file_state(&archive)?;
        if binary_present && manifest_present && archive_present {
            let runtime = Self {
                binary: binary.clone(),
            };
            runtime.verify()?;
            return Ok(runtime);
        }

        if !archive_present {
            // The six release archives are bundled with the emulator. A first
            // boot therefore never needs the network; the bytes are verified
            // against the compile-time pin before they are written.
            match bundled_archive() {
                Some(bytes) => install_bundled(&archive, bytes, &pinned)?,
                None => download_archive(&archive, artifact)?,
            }
        }
        let stage_dir = private_dir(&root, "install-stage")?;
        let staged = stage_dir.join(artifact.asset);
        let outcome = (|| -> Result<[u8; 32]> {
            stage_archive(&archive, &pinned, &staged)?;
            let extract_dir = private_dir(&root, "install-extract")?;
            let result = install_from_extract(&extract_dir, &staged, artifact, &binary, &pinned);
            let cleanup = remove_tree(&extract_dir);
            join_result_and_cleanup(result, cleanup)
        })();
        let cleanup = remove_tree(&stage_dir);
        let derived = join_result_and_cleanup(outcome, cleanup)?;
        let installed = sha256_file(&binary)?;
        if !ct_eq(&derived, &installed) {
            return Err(Error::Runtime(
                "laboon installed binary differs from the pinned archive".to_owned(),
            ));
        }
        write_manifest(&manifest, artifact.sha256, &installed)?;
        // The retained archive is read-only: verify() re-hashes these bytes on
        // every check, so this copy is the immutable evidence for the install.
        set_read_only(&archive)?;
        let runtime = Self { binary };
        runtime.verify()?;
        Ok(runtime)
    }

    /// Re-check the installed binary against its recorded digest AND against a
    /// fresh derivation from the pinned archive. No network access. The local
    /// manifest is a cache, never an authority: the archive digest is computed
    /// again from retained bytes and compared with the compile-time pin, and
    /// the binary digest is derived again by extracting that archive.
    pub fn verify(&self) -> Result<()> {
        let artifact = Self::platform()?;
        let pinned = pinned_bytes(artifact.sha256)?;
        validate_components(&self.binary)?;
        let meta = std::fs::metadata(&self.binary)
            .map_err(|e| Error::Runtime(format!("laboon metadata: {e}")))?;
        if !meta.is_file() {
            return Err(Error::Runtime(
                "laboon binary is not a regular file".to_owned(),
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if (meta.permissions().mode() & 0o111) == 0 {
                return Err(Error::Runtime("laboon binary is not executable".to_owned()));
            }
        }
        let installed = sha256_file(&self.binary)?;
        let (recorded_archive, recorded_binary) = read_manifest(&manifest_path(&self.binary)?)?;
        if !ct_eq(&recorded_archive, &pinned) {
            return Err(Error::Runtime("laboon archive pin is stale".to_owned()));
        }
        if !ct_eq(&recorded_binary, &installed) {
            return Err(Error::Runtime("laboon binary digest mismatch".to_owned()));
        }

        let root = data_root(&self.binary)?;
        let archive = root.join("download").join(artifact.asset);
        let stage_dir = private_dir(&root, "verify-stage")?;
        let staged = stage_dir.join(artifact.asset);
        let outcome = (|| -> Result<[u8; 32]> {
            // Recompute the archive hash from the retained bytes themselves.
            stage_archive(&archive, &pinned, &staged)?;
            derive_binary(&root, &staged, artifact, &pinned)
        })();
        let cleanup = remove_tree(&stage_dir);
        let derived = join_result_and_cleanup(outcome, cleanup)?;
        if !ct_eq(&derived, &installed) {
            return Err(Error::Runtime(
                "laboon binary is not the binary in the pinned archive".to_owned(),
            ));
        }
        Ok(())
    }

    /// Return the exact bytes that will be executed. The installed path is
    /// verified against the pinned archive, then copied into a private
    /// directory while hashing through the same open write handle. `start`
    /// re-hashes that private path immediately before spawn. A local writer
    /// that replaces `bin/laboon` after `verify` therefore cannot change the
    /// bytes this process runs: the copy's digest must equal the digest the
    /// pinned archive derived.
    pub fn verified_executable(&self) -> Result<(PathBuf, [u8; 32])> {
        self.verify()?;
        let (_recorded_archive, recorded_binary) = read_manifest(&manifest_path(&self.binary)?)?;
        let root = data_root(&self.binary)?;
        let exec_dir = root.join("private").join("exec");
        ensure_dir(&exec_dir)?;
        restrict_dir(&exec_dir)?;
        let exec_path = exec_dir.join(format!("laboon-{}", hex::encode(&recorded_binary)));
        if !file_state(&exec_path)? {
            let mut source = std::fs::File::open(&self.binary)
                .map_err(|e| Error::Runtime(format!("open verified laboon: {e}")))?;
            let mut dest = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&exec_path)
                .map_err(|e| Error::Runtime(format!("create private laboon: {e}")))?;
            let mut hasher = Sha256::new();
            let mut buf = [0u8; HASH_CHUNK];
            loop {
                let n = source
                    .read(&mut buf)
                    .map_err(|e| Error::Runtime(format!("read verified laboon: {e}")))?;
                if n == 0 {
                    break;
                }
                let chunk = match buf.get(..n) {
                    Some(part) => part,
                    None => return Err(Error::Runtime("read exceeded buffer".to_owned())),
                };
                hasher.update(chunk);
                dest.write_all(chunk)
                    .map_err(|e| Error::Runtime(format!("write private laboon: {e}")))?;
            }
            dest.sync_all()
                .map_err(|e| Error::Runtime(format!("sync private laboon: {e}")))?;
            drop(dest);
            let out = hasher.finalize();
            let mut copied = [0u8; 32];
            copied.copy_from_slice(&out);
            if !ct_eq(&copied, &recorded_binary) {
                return Err(Error::Runtime(
                    "verified laboon copy differs from the pinned archive".to_owned(),
                ));
            }
            set_executable(&exec_path)?;
        }
        if !ct_eq(&sha256_file(&exec_path)?, &recorded_binary) {
            return Err(Error::Runtime(
                "private laboon executable digest mismatch".to_owned(),
            ));
        }
        Ok((exec_path, recorded_binary))
    }
}

/// Database engines booted as real OCI server processes inside laboon.
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Debug)]
pub enum DatabaseKind {
    Postgres,
    Mysql,
    MariaDb,
    Redis,
    Valkey,
    MongoDb,
    DuckDb,
    Kafka,
}

impl DatabaseKind {
    /// Real OCI reference, pinned to a major version where one exists.
    #[must_use]
    pub fn image(self) -> &'static str {
        match self {
            Self::Postgres => "postgres:16",
            Self::Mysql => "mysql:8.4",
            Self::MariaDb => "mariadb:11",
            Self::Redis => "redis:7",
            Self::Valkey => "valkey/valkey:8",
            Self::MongoDb => "mongo:7",
            Self::DuckDb => "duckdb/duckdb:latest",
            Self::Kafka => "apache/kafka:3.8.0",
        }
    }

    /// A real, non-zero listening port. DuckDB's own local UI listener uses
    /// 4213, so its CLI-only image still declares a valid engine port rather
    /// than a zero placeholder that `WorkloadSpec::validate` would reject.
    #[must_use]
    pub fn port(self) -> u16 {
        match self {
            Self::Postgres => 5432,
            Self::Mysql | Self::MariaDb => 3306,
            Self::Redis | Self::Valkey => 6379,
            Self::MongoDb => 27017,
            Self::DuckDb => DUCKDB_UI_PORT,
            Self::Kafka => 9092,
        }
    }

    /// A real readiness probe for the engine's own client tools. This string
    /// is data for the caller; it is never handed to a shell by this module.
    #[must_use]
    pub fn readiness(self) -> &'static str {
        match self {
            Self::Postgres => "pg_isready -h 127.0.0.1 -p 5432 -U postgres",
            Self::Mysql => "mysqladmin ping -h 127.0.0.1 -P 3306 -u root",
            Self::MariaDb => "mariadb-admin ping -h 127.0.0.1 -P 3306 -u root",
            Self::Redis => "redis-cli -h 127.0.0.1 -p 6379 ping",
            Self::Valkey => "valkey-cli -h 127.0.0.1 -p 6379 ping",
            Self::MongoDb => "mongosh --quiet --host 127.0.0.1 --port 27017 --eval \"db.adminCommand({ping:1})\"",
            Self::DuckDb => "duckdb -c \"SELECT 1\"",
            Self::Kafka => "/opt/kafka/bin/kafka-broker-api-versions.sh --bootstrap-server 127.0.0.1:9092",
        }
    }
}

/// Build a validated workload that runs the engine's real server argv. Egress
/// is an explicit loopback policy, never the empty list that used to compile to
/// `-no-net`: a database must be able to bind 127.0.0.1 and serve its client.
pub fn database_spec(
    kind: DatabaseKind,
    name: &WorkloadId,
    partition: PartitionKey,
) -> Result<WorkloadSpec> {
    let command = match kind {
        DatabaseKind::Postgres => strings(&[
            "postgres",
            "-c",
            "listen_addresses=127.0.0.1",
            "-c",
            "port=5432",
        ]),
        DatabaseKind::Mysql => strings(&["mysqld", "--bind-address=127.0.0.1", "--port=3306"]),
        DatabaseKind::MariaDb => strings(&["mariadbd", "--bind-address=127.0.0.1", "--port=3306"]),
        DatabaseKind::Redis => strings(&["redis-server", "--bind", "127.0.0.1", "--port", "6379"]),
        DatabaseKind::Valkey => strings(&["valkey-server", "--bind", "127.0.0.1", "--port", "6379"]),
        DatabaseKind::MongoDb => strings(&["mongod", "--bind_ip", "127.0.0.1", "--port", "27017"]),
        DatabaseKind::DuckDb => strings(&["duckdb", "-c", "SELECT 1"]),
        DatabaseKind::Kafka => strings(&[
            "/opt/kafka/bin/kafka-server-start.sh",
            "/opt/kafka/config/kraft/server.properties",
        ]),
    };
    let spec = WorkloadSpec {
        id: name.clone(),
        image: kind.image().to_owned(),
        command,
        env: Vec::new(),
        ports: vec![kind.port()],
        memory_mb: DATABASE_MEMORY_MB,
        timeout_ms: DATABASE_TIMEOUT_MS,
        egress_allowlist: vec![LOOPBACK_EGRESS_HOST.to_owned()],
        partition,
        replicas: 1,
    };
    spec.validate()?;
    Ok(spec)
}

/// A pure plan for one workload: the distinct laboon argv plus the two bounds
/// the supervisor enforces. Constructing it performs no IO.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorkloadSpecRun {
    pub spec: WorkloadSpec,
    pub argv: Vec<String>,
    pub timeout_ms: u64,
    pub max_output_bytes: usize,
}

/// Compile a workload into laboon's verified v0.1.0 CLI. Every value is a
/// distinct argv entry. `--` terminates laboon's flags before the guest
/// command. Guest environment entries become repeatable `-e KEY=VALUE` pairs
/// and are never applied to the host laboon process.
pub fn laboon_argv(spec: &WorkloadSpec, runtime: &LaboonRuntime) -> Result<WorkloadSpecRun> {
    spec.validate()?;
    validate_env(spec)?;
    if runtime.binary.as_os_str().is_empty() {
        return Err(Error::Runtime("laboon binary path is empty".to_owned()));
    }
    validate_components(&runtime.binary)?;
    if spec.image.starts_with('-') {
        return Err(Error::InvalidImage(spec.image.clone()));
    }
    let mut argv = Vec::with_capacity(10usize.saturating_add(spec.command.len()));
    argv.push("-image".to_owned());
    argv.push(spec.image.clone());
    argv.push("-mem".to_owned());
    argv.push(format!("{}M", spec.memory_mb));
    for (key, value) in &spec.env {
        argv.push("-e".to_owned());
        argv.push(format!("{key}={value}"));
    }
    egress_flags(spec, &mut argv)?;
    if !spec.command.is_empty() {
        argv.push("--".to_owned());
        argv.extend(spec.command.iter().cloned());
    }
    Ok(WorkloadSpecRun {
        spec: spec.clone(),
        argv,
        timeout_ms: spec.timeout_ms,
        max_output_bytes: MAX_OUTPUT_BYTES,
    })
}

/// A live guest with its two bounded output drainers and its deadline
/// watchdog. The `Child` is shared through a mutex so the watchdog can call
/// `Child::kill()` directly: no external signal utility and no pid-reuse
/// window. Every spawned thread is joined before the value is released.
/// Callers that retain spawned guests (`Shared.children` in net.rs) must use
/// this type in place of a bare `std::process::Child`.
#[derive(Debug)]
pub struct WorkloadChild {
    child: Arc<Mutex<Child>>,
    started_ms: u64,
    timeout_ms: u64,
    max_output_bytes: usize,
    timed_out: Arc<AtomicBool>,
    drains: Vec<JoinHandle<Result<Vec<u8>>>>,
    watchdog: Option<JoinHandle<Result<()>>>,
    done: Option<Sender<()>>,
}

impl WorkloadChild {
    /// OS process id of the guest, for diagnostics.
    pub fn id(&self) -> Result<u32> {
        let guard = self
            .child
            .lock()
            .map_err(|e| Error::Runtime(format!("laboon child lock: {e}")))?;
        Ok(guard.id())
    }

    #[must_use]
    pub fn started_ms(&self) -> u64 {
        self.started_ms
    }

    #[must_use]
    pub fn timeout_ms(&self) -> u64 {
        self.timeout_ms
    }

    /// Liveness only. A successful exit is never re-stopped or re-reported as
    /// a timeout; the deadline is examined only after this returns `None`.
    fn liveness(&self) -> Result<Option<ExitStatus>> {
        let mut guard = self
            .child
            .lock()
            .map_err(|e| Error::Runtime(format!("laboon child lock: {e}")))?;
        guard
            .try_wait()
            .map_err(|e| Error::Runtime(format!("laboon try_wait: {e}")))
    }

    fn join_threads(&mut self) -> Result<()> {
        let mut failure: Option<Error> = None;
        let mut total_output = 0usize;
        if let Some(handle) = self.watchdog.take() {
            match handle.join() {
                Ok(Ok(())) => {}
                Ok(Err(e)) => failure = Some(e),
                Err(_) => {
                    failure = Some(Error::Runtime("laboon watchdog thread failed".to_owned()));
                }
            }
        }
        for handle in std::mem::take(&mut self.drains) {
            match handle.join() {
                Ok(Ok(bytes)) => {
                    total_output = total_output.saturating_add(bytes.len());
                    if total_output > self.max_output_bytes && failure.is_none() {
                        failure = Some(Error::Runtime(
                            "laboon output exceeded its cap".to_owned(),
                        ));
                    }
                }
                Ok(Err(e)) => {
                    if failure.is_none() {
                        failure = Some(e);
                    }
                }
                Err(_) => {
                    if failure.is_none() {
                        failure = Some(Error::Runtime(
                            "laboon output drainer thread failed".to_owned(),
                        ));
                    }
                }
            }
        }
        match failure {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Signal the watchdog, terminate and reap a still-running guest, then join
    /// every thread. Calling this twice is harmless: an exited child is
    /// accepted and no thread is joined twice.
    fn release(&mut self) -> Result<()> {
        drop(self.done.take());
        {
            let mut guard = self
                .child
                .lock()
                .map_err(|e| Error::Runtime(format!("laboon child lock: {e}")))?;
            let live = guard
                .try_wait()
                .map_err(|e| Error::Runtime(format!("laboon try_wait: {e}")))?;
            if live.is_none() {
                guard
                    .kill()
                    .map_err(|e| Error::Runtime(format!("laboon kill: {e}")))?;
                guard
                    .wait()
                    .map_err(|e| Error::Runtime(format!("laboon wait: {e}")))?;
            }
        }
        self.join_threads()
    }
}

impl Drop for WorkloadChild {
    fn drop(&mut self) {
        if let Err(e) = self.release() {
            // A destructor cannot return an error; report it once instead of
            // silently abandoning the guest or its threads.
            eprintln!("aws0: laboon workload cleanup: {e}");
        }
    }
}

/// Hosts real guest processes. `dir` is the explicit working directory; the
/// runtime is passed in, and only a fixed allowlist of the node's own host
/// variables is copied to the laboon process.
#[derive(Clone, Debug)]
pub struct WorkloadSupervisor {
    pub runtime: LaboonRuntime,
    pub dir: PathBuf,
}

impl WorkloadSupervisor {
    #[must_use]
    pub fn new(runtime: LaboonRuntime, dir: PathBuf) -> Self {
        Self { runtime, dir }
    }

    /// Validate, compile and spawn laboon with piped stdout and stderr. The
    /// 1 MiB output cap is enforced by two drainer threads that call
    /// [`read_bounded`] with `run.max_output_bytes`; the workload timeout is
    /// enforced by a watchdog thread that kills the shared `Child` at the
    /// recorded deadline. A failed spawn terminates the child and joins every
    /// thread already started.
    pub fn start(&self, spec: &WorkloadSpec) -> Result<WorkloadChild> {
        spec.validate()?;
        self.runtime.verify()?;
        validate_components(&self.dir)?;
        let run = laboon_argv(spec, &self.runtime)?;
        enforce_bounds(&run)?;
        ensure_dir(&self.dir)?;
        let timeout_ms = run.timeout_ms;
        let cap = run.max_output_bytes;

        // Execute the verified private copy, not the mutable install path. The
        // digest is checked again here, immediately before the spawn.
        let (executable, expected) = self.runtime.verified_executable()?;
        if !ct_eq(&sha256_file(&executable)?, &expected) {
            return Err(Error::Runtime(
                "laboon executable changed between verification and spawn".to_owned(),
            ));
        }
        let mut command = Command::new(executable.as_os_str());
        command.args(run.argv.iter().map(String::as_str));
        command.env_clear();
        for &key in HOST_ENV_ALLOWLIST {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        command
            .current_dir(&self.dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command
            .spawn()
            .map_err(|e| Error::Runtime(format!("laboon spawn: {e}")))?;

        let stdout = match child.stdout.take() {
            Some(pipe) => pipe,
            None => {
                return Err(abandon_child(
                    child,
                    Vec::new(),
                    Error::Runtime("laboon stdout pipe is missing".to_owned()),
                ))
            }
        };
        let stderr = match child.stderr.take() {
            Some(pipe) => pipe,
            None => {
                return Err(abandon_child(
                    child,
                    Vec::new(),
                    Error::Runtime("laboon stderr pipe is missing".to_owned()),
                ))
            }
        };
        let mut drains = Vec::with_capacity(2);
        match spawn_drainer(stdout, cap, "aws0-laboon-stdout") {
            Ok(handle) => drains.push(handle),
            Err(e) => return Err(abandon_child(child, drains, e)),
        }
        match spawn_drainer(stderr, cap, "aws0-laboon-stderr") {
            Ok(handle) => drains.push(handle),
            Err(e) => return Err(abandon_child(child, drains, e)),
        }

        let shared = Arc::new(Mutex::new(child));
        let timed_out = Arc::new(AtomicBool::new(false));
        let (done, signal) = mpsc::channel();
        let watchdog_child = Arc::clone(&shared);
        let watchdog_flag = Arc::clone(&timed_out);
        let watchdog = match std::thread::Builder::new()
            .name("aws0-laboon-deadline".to_owned())
            .spawn(move || watchdog(watchdog_child, watchdog_flag, timeout_ms, signal))
        {
            Ok(handle) => handle,
            Err(e) => {
                return Err(abandon_shared(
                    shared,
                    drains,
                    Error::Runtime(format!("laboon watchdog spawn: {e}")),
                ))
            }
        };
        Ok(WorkloadChild {
            child: shared,
            started_ms: crate::types::now_ms(),
            timeout_ms,
            max_output_bytes: cap,
            timed_out,
            drains,
            watchdog: Some(watchdog),
            done: Some(done),
        })
    }

    /// Kill the guest, reap it and join the bounded drainers. An already-exited
    /// child is accepted, so `stop` is idempotent and total.
    pub fn stop(&self, child: &mut WorkloadChild) -> Result<()> {
        child.release()
    }

    /// Non-blocking liveness observation with the real clock. Liveness is
    /// checked BEFORE the deadline: a child that exited successfully is always
    /// `Stopped`, never re-stopped and never reported as a timeout.
    pub fn health(&self, child: &mut WorkloadChild) -> Result<WorkloadStatus> {
        if let Some(status) = child.liveness()? {
            let expired = child.timed_out.load(Ordering::Relaxed);
            child.release()?;
            if expired {
                return Ok(timeout_status(child.timeout_ms));
            }
            return Ok(status_of(status));
        }
        let now_ms = crate::types::now_ms();
        if now_ms.saturating_sub(child.started_ms) >= child.timeout_ms {
            child.timed_out.store(true, Ordering::Relaxed);
            child.release()?;
            return Ok(timeout_status(child.timeout_ms));
        }
        Ok(WorkloadStatus::Running)
    }

    /// Bounded liveness with an explicit clock. `started_ms` and `now_ms` are
    /// supplied by the caller, so the timeout decision is deterministic under
    /// replay. Liveness is checked BEFORE the deadline; only a still-running
    /// child past its deadline is stopped and failed.
    pub fn status_at(
        &self,
        child: &mut WorkloadChild,
        started_ms: u64,
        now_ms: u64,
        timeout_ms: u64,
    ) -> Result<WorkloadStatus> {
        if timeout_ms == 0 {
            return Err(Error::InvalidTimeout(0));
        }
        if let Some(status) = child.liveness()? {
            let expired = child.timed_out.load(Ordering::Relaxed);
            child.release()?;
            if expired {
                return Ok(timeout_status(timeout_ms));
            }
            return Ok(status_of(status));
        }
        if now_ms.saturating_sub(started_ms) >= timeout_ms {
            child.timed_out.store(true, Ordering::Relaxed);
            child.release()?;
            return Ok(timeout_status(timeout_ms));
        }
        Ok(WorkloadStatus::Running)
    }
}

/// Drain a piped stream to EOF while retaining at most `cap` bytes. The stream
/// is always drained so a verbose guest cannot block on a full pipe, but
/// exceeding the cap is a typed refusal, never a silent truncation.
pub fn read_bounded<R: Read>(reader: &mut R, cap: usize) -> Result<Vec<u8>> {
    let mut retained = Vec::new();
    let mut total = 0usize;
    let mut scratch = [0u8; HASH_CHUNK];
    loop {
        let n = reader
            .read(&mut scratch)
            .map_err(|e| Error::Runtime(format!("laboon output read: {e}")))?;
        if n == 0 {
            break;
        }
        let chunk = match scratch.get(..n) {
            Some(part) => part,
            None => return Err(Error::Runtime("read exceeded buffer".to_owned())),
        };
        total = total.saturating_add(chunk.len());
        let room = cap.saturating_sub(retained.len());
        if room > 0 {
            let take = room.min(chunk.len());
            let part = match chunk.get(..take) {
                Some(bytes) => bytes,
                None => return Err(Error::Runtime("read exceeded buffer".to_owned())),
            };
            retained.extend_from_slice(part);
        }
    }
    if total > cap {
        return Err(Error::Runtime("laboon output exceeded its cap".to_owned()));
    }
    Ok(retained)
}

fn enforce_bounds(run: &WorkloadSpecRun) -> Result<()> {
    if run.timeout_ms == 0 || run.timeout_ms > 3_600_000 {
        return Err(Error::InvalidTimeout(run.timeout_ms));
    }
    if run.max_output_bytes != MAX_OUTPUT_BYTES || run.max_output_bytes == 0 {
        return Err(Error::Runtime("laboon output cap must be 1 MiB".to_owned()));
    }
    let mut total = 0usize;
    for entry in &run.argv {
        if entry.contains('\0') {
            return Err(Error::InvalidCommand);
        }
        total = total
            .checked_add(entry.len())
            .ok_or_else(|| Error::Runtime("laboon argv length overflow".to_owned()))?;
    }
    if total > MAX_ARGV_BYTES {
        return Err(Error::Runtime("laboon argv exceeds 1 MiB".to_owned()));
    }
    Ok(())
}

/// Validate every guest environment entry: ASCII, bounded, no NUL, no `=` in
/// keys (the key grammar forbids it), no duplicate keys and no loader keys.
fn validate_env(spec: &WorkloadSpec) -> Result<()> {
    if spec.env.len() > MAX_ENV_ENTRIES {
        return Err(Error::InvalidCommand);
    }
    let mut seen = BTreeSet::new();
    for (key, value) in &spec.env {
        if key.is_empty()
            || key.len() > MAX_ENV_KEY_BYTES
            || !key.is_ascii()
            || value.len() > MAX_ENV_VALUE_BYTES
            || !value.is_ascii()
        {
            return Err(Error::InvalidCommand);
        }
        let first = match key.as_bytes().first() {
            Some(byte) => *byte,
            None => return Err(Error::InvalidCommand),
        };
        if !first.is_ascii_alphabetic() && first != b'_' {
            return Err(Error::InvalidCommand);
        }
        if !key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(Error::InvalidCommand);
        }
        if value
            .bytes()
            .any(|byte| byte == 0 || (byte < 0x20 && byte != b'\t'))
        {
            return Err(Error::InvalidCommand);
        }
        if DENIED_ENV_KEYS.contains(&key.as_str()) {
            return Err(Error::InvalidCommand);
        }
        if !seen.insert(key.as_str()) {
            return Err(Error::InvalidCommand);
        }
    }
    Ok(())
}

fn egress_flags(spec: &WorkloadSpec, argv: &mut Vec<String>) -> Result<()> {
    if spec.egress_allowlist.is_empty() {
        // laboon's default is loopback only. `-no-net` would remove INET
        // sockets entirely and is never emitted here.
        return Ok(());
    }
    for host in &spec.egress_allowlist {
        let valid = !host.is_empty()
            && host.len() <= MAX_EGRESS_BYTES
            && host.is_ascii()
            && !host.contains(',')
            && !host.contains('\0')
            && !host.starts_with('-')
            && host
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'-' || byte == b'_');
        if !valid {
            return Err(Error::InvalidEgress);
        }
    }
    argv.push("-net".to_owned());
    argv.push("-allow-domains".to_owned());
    argv.push(spec.egress_allowlist.join(","));
    // The v0.1.0 egress policy blocks loopback/private addresses unless an
    // explicit CIDR is granted; a database listens on its own guest loopback.
    argv.push("-allow-cidrs".to_owned());
    argv.push(LOOPBACK_CIDR.to_owned());
    Ok(())
}

fn spawn_drainer<R>(
    mut reader: R,
    cap: usize,
    label: &'static str,
) -> Result<JoinHandle<Result<Vec<u8>>>>
where
    R: Read + Send + 'static,
{
    std::thread::Builder::new()
        .name(label.to_owned())
        .spawn(move || read_bounded(&mut reader, cap))
        .map_err(|e| Error::Runtime(format!("{label} spawn: {e}")))
}

/// Sleep until the deadline or until the owner releases the child, then kill
/// the still-running guest through the shared `Child` handle.
fn watchdog(
    child: Arc<Mutex<Child>>,
    flag: Arc<AtomicBool>,
    timeout_ms: u64,
    done: Receiver<()>,
) -> Result<()> {
    let start = Instant::now();
    let limit = Duration::from_millis(timeout_ms);
    loop {
        let remaining = match limit.checked_sub(start.elapsed()) {
            Some(duration) => duration,
            None => Duration::from_millis(0),
        };
        match done.recv_timeout(remaining) {
            Ok(()) | Err(RecvTimeoutError::Disconnected) => return Ok(()),
            Err(RecvTimeoutError::Timeout) => {
                if remaining == Duration::from_millis(0) {
                    let mut guard = child
                        .lock()
                        .map_err(|e| Error::Runtime(format!("laboon watchdog lock: {e}")))?;
                    return match guard.try_wait() {
                        Ok(Some(_)) => Ok(()),
                        Ok(None) => {
                            flag.store(true, Ordering::Relaxed);
                            guard
                                .kill()
                                .map_err(|e| Error::Runtime(format!("laboon watchdog kill: {e}")))
                        }
                        Err(e) => Err(Error::Runtime(format!("laboon watchdog try_wait: {e}"))),
                    };
                }
            }
        }
    }
}

fn abandon_child(
    mut child: Child,
    drains: Vec<JoinHandle<Result<Vec<u8>>>>,
    primary: Error,
) -> Error {
    let mut notes = Vec::new();
    match child.kill() {
        Ok(()) => {}
        Err(e) => notes.push(format!("kill: {e}")),
    }
    match child.wait() {
        Ok(_) => {}
        Err(e) => notes.push(format!("wait: {e}")),
    }
    collect_drains(drains, &mut notes);
    combine_error(primary, notes)
}

fn abandon_shared(
    shared: Arc<Mutex<Child>>,
    drains: Vec<JoinHandle<Result<Vec<u8>>>>,
    primary: Error,
) -> Error {
    let mut notes = Vec::new();
    match shared.lock() {
        Ok(mut guard) => {
            if let Err(e) = guard.kill() {
                notes.push(format!("kill: {e}"));
            }
            if let Err(e) = guard.wait() {
                notes.push(format!("wait: {e}"));
            }
        }
        Err(e) => notes.push(format!("lock: {e}")),
    }
    collect_drains(drains, &mut notes);
    combine_error(primary, notes)
}

fn collect_drains(drains: Vec<JoinHandle<Result<Vec<u8>>>>, notes: &mut Vec<String>) {
    for handle in drains {
        match handle.join() {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => notes.push(format!("drainer: {e}")),
            Err(_) => notes.push("drainer thread failed".to_owned()),
        }
    }
}

fn combine_error(primary: Error, notes: Vec<String>) -> Error {
    if notes.is_empty() {
        primary
    } else {
        Error::Runtime(format!("{primary}; cleanup: {}", notes.join("; ")))
    }
}

fn status_of(status: ExitStatus) -> WorkloadStatus {
    if status.success() {
        WorkloadStatus::Stopped
    } else {
        WorkloadStatus::Failed(format!("laboon exited: {status}"))
    }
}

fn timeout_status(timeout_ms: u64) -> WorkloadStatus {
    WorkloadStatus::Failed(format!("laboon timed out after {timeout_ms} ms"))
}

fn strings(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| (*part).to_owned()).collect()
}

fn binary_name() -> &'static str {
    if cfg!(windows) {
        "laboon.exe"
    } else {
        "laboon"
    }
}

/// Resolve a user-supplied data directory to an absolute path exactly once and
/// reject any component that a child tool could read as an option.
fn absolute_root(dir: &Path) -> Result<PathBuf> {
    let root = if dir.is_absolute() {
        dir.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| Error::Runtime(format!("current directory: {e}")))?
            .join(dir)
    };
    validate_components(&root)?;
    Ok(root)
}

fn validate_components(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(Error::Runtime("laboon path is empty".to_owned()));
    }
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let text = match part.to_str() {
                    Some(value) => value,
                    None => {
                        return Err(Error::Runtime(format!(
                            "path is not UTF-8: {}",
                            path.display()
                        )))
                    }
                };
                if text.starts_with('-') || text.contains('\0') {
                    return Err(Error::Runtime(format!(
                        "path component may begin an option: {}",
                        path.display()
                    )));
                }
            }
            Component::ParentDir => {
                return Err(Error::Runtime(format!(
                    "path contains a parent component: {}",
                    path.display()
                )))
            }
            _ => {}
        }
    }
    Ok(())
}

fn path_arg(path: &Path) -> Result<String> {
    if !path.is_absolute() {
        return Err(Error::Runtime(format!(
            "path is not absolute: {}",
            path.display()
        )));
    }
    validate_components(path)?;
    path.to_str().map(|s| s.to_owned()).ok_or_else(|| {
        Error::Runtime(format!("path is not UTF-8: {}", path.display()))
    })
}

fn ensure_dir(dir: &Path) -> Result<()> {
    std::fs::create_dir_all(dir)
        .map_err(|e| Error::Runtime(format!("create {}: {e}", dir.display())))
}

fn remove_tree(path: &Path) -> Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(meta) => {
            if meta.file_type().is_symlink() || !meta.is_dir() {
                std::fs::remove_file(path)
                    .map_err(|e| Error::Runtime(format!("clear {}: {e}", path.display())))
            } else {
                std::fs::remove_dir_all(path)
                    .map_err(|e| Error::Runtime(format!("clear {}: {e}", path.display())))
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(Error::Runtime(format!("stat {}: {e}", path.display()))),
    }
}

fn reset_dir(dir: &Path) -> Result<()> {
    remove_tree(dir)?;
    ensure_dir(dir)
}

fn private_dir(root: &Path, label: &str) -> Result<PathBuf> {
    let seq = PRIVATE_SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = root
        .join("private")
        .join(format!("{}-{}-{}", label, std::process::id(), seq));
    validate_components(&dir)?;
    reset_dir(&dir)?;
    restrict_dir(&dir)?;
    Ok(dir)
}

fn join_result_and_cleanup<T>(result: Result<T>, cleanup: Result<()>) -> Result<T> {
    match (result, cleanup) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(e), _) => Err(e),
        (Ok(_), Err(e)) => Err(e),
    }
}

fn file_state(path: &Path) -> Result<bool> {
    match std::fs::metadata(path) {
        Ok(meta) => Ok(meta.is_file()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(Error::Runtime(format!("stat {}: {e}", path.display()))),
    }
}

fn data_root(binary: &Path) -> Result<PathBuf> {
    let bin_dir = binary
        .parent()
        .ok_or_else(|| Error::Runtime("laboon binary has no parent directory".to_owned()))?;
    let root = bin_dir
        .parent()
        .ok_or_else(|| Error::Runtime("laboon binary has no data directory".to_owned()))?;
    if root.as_os_str().is_empty() {
        return Err(Error::Runtime(
            "laboon binary has no data directory".to_owned(),
        ));
    }
    Ok(root.to_path_buf())
}

fn manifest_path(binary: &Path) -> Result<PathBuf> {
    let parent = binary
        .parent()
        .ok_or_else(|| Error::Runtime("laboon binary has no parent directory".to_owned()))?;
    Ok(parent.join(MANIFEST_NAME))
}

fn pinned_bytes(text: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(text)
        .map_err(|e| Error::Runtime(format!("pinned digest is not hex: {e}")))?;
    let array: [u8; 32] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| Error::Runtime("pinned digest is not 32 bytes".to_owned()))?;
    Ok(array)
}

/// Constant-time equality over exactly 32 bytes: no early exit, no branch on
/// a differing byte, no secret-dependent memory access.
fn ct_eq(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn sha256_file(path: &Path) -> Result<[u8; 32]> {
    let mut file = std::fs::File::open(path)
        .map_err(|e| Error::Runtime(format!("open {}: {e}", path.display())))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; HASH_CHUNK];
    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| Error::Runtime(format!("read {}: {e}", path.display())))?;
        if n == 0 {
            break;
        }
        let chunk = match buf.get(..n) {
            Some(part) => part,
            None => return Err(Error::Runtime("read exceeded buffer".to_owned())),
        };
        hasher.update(chunk);
    }
    let out = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(&out);
    Ok(result)
}

/// Open the archive exactly once: every byte read from that handle is hashed
/// and is also written to a private staged copy. The extractor reads only that
/// staged copy, so no second open of the mutable download path can introduce
/// a different archive (no time-of-check/time-of-use split).
fn stage_archive(source: &Path, pinned: &[u8; 32], staged: &Path) -> Result<[u8; 32]> {
    let mut input = std::fs::File::open(source)
        .map_err(|e| Error::Runtime(format!("open {}: {e}", source.display())))?;
    let mut output = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(staged)
        .map_err(|e| Error::Runtime(format!("create {}: {e}", staged.display())))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; HASH_CHUNK];
    loop {
        let n = input
            .read(&mut buf)
            .map_err(|e| Error::Runtime(format!("read {}: {e}", source.display())))?;
        if n == 0 {
            break;
        }
        let chunk = match buf.get(..n) {
            Some(part) => part,
            None => return Err(Error::Runtime("read exceeded buffer".to_owned())),
        };
        hasher.update(chunk);
        output
            .write_all(chunk)
            .map_err(|e| Error::Runtime(format!("stage {}: {e}", staged.display())))?;
    }
    output
        .flush()
        .map_err(|e| Error::Runtime(format!("stage {}: {e}", staged.display())))?;
    output
        .sync_all()
        .map_err(|e| Error::Runtime(format!("stage {}: {e}", staged.display())))?;
    drop(output);
    let out = hasher.finalize();
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&out);
    set_read_only(staged)?;
    if !ct_eq(&digest, pinned) {
        return match remove_tree(staged) {
            Ok(()) => Err(Error::Runtime(
                "laboon archive digest does not match the pinned release".to_owned(),
            )),
            Err(e) => Err(e),
        };
    }
    Ok(digest)
}

fn install_from_extract(
    extract_dir: &Path,
    staged: &Path,
    artifact: &PlatformArtifact,
    binary: &Path,
    pinned: &[u8; 32],
) -> Result<[u8; 32]> {
    extract_archive(staged, artifact, extract_dir, pinned)?;
    let found = locate_binary(extract_dir)?;
    let derived = sha256_file(&found)?;
    std::fs::copy(&found, binary)
        .map_err(|e| Error::Runtime(format!("laboon install: {e}")))?;
    set_executable(binary)?;
    Ok(derived)
}

fn derive_binary(
    root: &Path,
    staged: &Path,
    artifact: &PlatformArtifact,
    pinned: &[u8; 32],
) -> Result<[u8; 32]> {
    let extract_dir = private_dir(root, "verify-extract")?;
    let result = (|| -> Result<[u8; 32]> {
        extract_archive(staged, artifact, &extract_dir, pinned)?;
        let found = locate_binary(&extract_dir)?;
        sha256_file(&found)
    })();
    let cleanup = remove_tree(&extract_dir);
    join_result_and_cleanup(result, cleanup)
}

fn read_manifest(path: &Path) -> Result<([u8; 32], [u8; 32])> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| Error::Runtime(format!("laboon manifest: {e}")))?;
    let mut parts = text.split_whitespace();
    let archive = parts
        .next()
        .ok_or_else(|| Error::Runtime("laboon manifest is empty".to_owned()))?;
    let binary = parts
        .next()
        .ok_or_else(|| Error::Runtime("laboon manifest has one field".to_owned()))?;
    if parts.next().is_some() {
        return Err(Error::Runtime("laboon manifest has extra fields".to_owned()));
    }
    Ok((pinned_bytes(archive)?, pinned_bytes(binary)?))
}

/// Atomic manifest write: a fresh temporary file is synced and renamed, so a
/// torn manifest can never be observed. The manifest remains a cache only.
fn write_manifest(path: &Path, archive_hex: &str, binary: &[u8; 32]) -> Result<()> {
    let line = format!("{} {}\n", archive_hex, hex::encode(&binary[..]));
    let tmp = path.with_extension("tmp");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&tmp)
        .map_err(|e| Error::Runtime(format!("laboon manifest write: {e}")))?;
    file.write_all(line.as_bytes())
        .map_err(|e| Error::Runtime(format!("laboon manifest write: {e}")))?;
    file.sync_all()
        .map_err(|e| Error::Runtime(format!("laboon manifest sync: {e}")))?;
    drop(file);
    std::fs::rename(&tmp, path)
        .map_err(|e| Error::Runtime(format!("laboon manifest rename: {e}")))
}

fn resolve_tool(candidates: &[&str], name: &str) -> Result<PathBuf> {
    for &candidate in candidates {
        let path = Path::new(candidate);
        match std::fs::metadata(path) {
            Ok(meta) if meta.is_file() => return Ok(path.to_path_buf()),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(Error::Runtime(format!("stat {candidate}: {e}"))),
        }
    }
    Err(Error::Runtime(format!(
        "{name} is not available at its pinned absolute paths"
    )))
}

fn run_tool(program: &Path, args: &[String], label: &str) -> Result<()> {
    let out = Command::new(program)
        .args(args.iter().map(String::as_str))
        .output()
        .map_err(|e| Error::Runtime(format!("{label} spawn: {e}")))?;
    if !out.status.success() {
        let stderr = out.stderr.as_slice();
        let shown = match stderr.get(..4096) {
            Some(part) => part,
            None => stderr,
        };
        return Err(Error::Runtime(format!(
            "{label} failed ({}): {}",
            out.status,
            String::from_utf8_lossy(shown)
        )));
    }
    Ok(())
}

/// The byte-exact v0.1.0 release archives, selected by the compile-time host
/// target. All six are present in the repository, so an installed emulator is
/// never forced to fetch a runtime from the network.
fn bundled_archive() -> Option<&'static [u8]> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => {
            Some(include_bytes!("../runtime/laboon_0.1.0_darwin_arm64.tar.gz"))
        }
        ("macos", "x86_64") => {
            Some(include_bytes!("../runtime/laboon_0.1.0_darwin_amd64.tar.gz"))
        }
        ("linux", "aarch64") => {
            Some(include_bytes!("../runtime/laboon_0.1.0_linux_arm64.tar.gz"))
        }
        ("linux", "x86_64") => {
            Some(include_bytes!("../runtime/laboon_0.1.0_linux_amd64.tar.gz"))
        }
        ("windows", "aarch64") => {
            Some(include_bytes!("../runtime/laboon_0.1.0_windows_arm64.zip"))
        }
        ("windows", "x86_64") => {
            Some(include_bytes!("../runtime/laboon_0.1.0_windows_amd64.zip"))
        }
        _ => None,
    }
}

/// Write a bundled archive to the data directory only after its bytes match the
/// compile-time release pin. `create_new` refuses to overwrite an existing file.
fn install_bundled(dest: &Path, bytes: &'static [u8], pinned: &[u8; 32]) -> Result<()> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&hasher.finalize());
    if !ct_eq(&digest, pinned) {
        return Err(Error::Runtime(
            "bundled laboon archive does not match the pinned release".to_owned(),
        ));
    }
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dest)
        .map_err(|e| Error::Runtime(format!("create bundled laboon archive: {e}")))?;
    file.write_all(bytes)
        .map_err(|e| Error::Runtime(format!("write bundled laboon archive: {e}")))?;
    file.sync_all()
        .map_err(|e| Error::Runtime(format!("sync bundled laboon archive: {e}")))?;
    drop(file);
    set_read_only(dest)
}

fn download_archive(dest: &Path, artifact: &PlatformArtifact) -> Result<()> {
    let url = format!(
        "{}/v{}/{}",
        LABOON_RELEASE_BASE, LABOON_VERSION, artifact.asset
    );
    let part = dest.with_extension("part");
    let args: Vec<String> = vec![
        "-fsSL".to_owned(),
        "--max-time".to_owned(),
        "60".to_owned(),
        "-o".to_owned(),
        path_arg(&part)?,
        "--".to_owned(),
        url,
    ];
    let program = resolve_tool(CURL_PATHS, "curl")?;
    let outcome = run_tool(&program, &args, "curl");
    let cleanup = match outcome {
        Ok(()) => Ok(()),
        Err(e) => match remove_tree(&part) {
            Ok(()) => Err(e),
            Err(cleanup_error) => Err(Error::Runtime(format!("{e}; cleanup: {cleanup_error}"))),
        },
    };
    cleanup?;
    std::fs::rename(&part, dest)
        .map_err(|e| Error::Runtime(format!("laboon archive install: {e}")))
}

/// Run a pinned tool with the verified archive as its standard input. `File`
/// is cloned from the SAME open handle that was hashed, so the child reads the
/// exact bytes that were authenticated; no pathname is re-opened.
fn run_tool_with_stdin(
    program: &Path,
    args: &[String],
    label: &str,
    file: &std::fs::File,
) -> Result<Vec<u8>> {
    let stdin = file
        .try_clone()
        .map_err(|e| Error::Runtime(format!("{label} stdin clone: {e}")))?;
    let out = Command::new(program)
        .args(args.iter().map(String::as_str))
        .stdin(Stdio::from(stdin))
        .output()
        .map_err(|e| Error::Runtime(format!("{label} spawn: {e}")))?;
    if !out.status.success() {
        let shown = match out.stderr.get(..4096) {
            Some(part) => part,
            None => out.stderr.as_slice(),
        };
        return Err(Error::Runtime(format!(
            "{label} failed ({}): {}",
            out.status,
            String::from_utf8_lossy(shown)
        )));
    }
    Ok(out.stdout)
}

/// List archive member names from the authenticated open handle. The listing
/// is bounded, so a hostile archive cannot force an unbounded allocation.
fn list_members_from_fd(program: &Path, file: &std::fs::File) -> Result<Vec<String>> {
    let args = vec!["-tf".to_owned(), "-".to_owned(), "--".to_owned()];
    let stdout = run_tool_with_stdin(program, &args, "tar member list", file)?;
    if stdout.len() > MAX_ARGV_BYTES {
        return Err(Error::Runtime(
            "laboon archive member list exceeds 1 MiB".to_owned(),
        ));
    }
    let text = String::from_utf8_lossy(&stdout);
    Ok(text
        .lines()
        .map(|line| line.trim_end_matches('\r').to_owned())
        .collect())
}

/// Hash a stream while reading it to EOF. Used on the single open archive
/// handle that is also handed to the extractor.
fn sha256_reader<R: Read>(reader: &mut R) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut buf = [0u8; HASH_CHUNK];
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| Error::Runtime(format!("laboon archive read: {e}")))?;
        if n == 0 {
            break;
        }
        let chunk = match buf.get(..n) {
            Some(part) => part,
            None => return Err(Error::Runtime("read exceeded buffer".to_owned())),
        };
        hasher.update(chunk);
    }
    let out = hasher.finalize();
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&out);
    Ok(digest)
}

/// Refuse absolute, drive-letter and parent-traversal members before any
/// extraction command is allowed to write inside the private directory.
fn member_confined(name: &str) -> bool {
    if name.is_empty()
        || name.starts_with('/')
        || name.starts_with('\\')
        || name.contains(':')
        || name.contains('\0')
    {
        return false;
    }
    !name
        .split(|c| c == '/' || c == '\\')
        .any(|part| part == "..")
}

fn extract_archive(
    archive: &Path,
    artifact: &PlatformArtifact,
    dest: &Path,
    pinned: &[u8; 32],
) -> Result<()> {
    if !(artifact.asset.ends_with(".tar.gz") || artifact.asset.ends_with(".zip")) {
        return Err(Error::Runtime(format!(
            "unknown laboon archive format: {}",
            artifact.asset
        )));
    }
    // One open handle serves hashing, member listing and extraction. bsdtar
    // reads both release formats from standard input, so no tool re-opens the
    // mutable pathname: the bytes authenticated are the bytes extracted.
    let mut file = std::fs::File::open(archive)
        .map_err(|e| Error::Runtime(format!("open staged laboon archive: {e}")))?;
    let digest = sha256_reader(&mut file)?;
    if !ct_eq(&digest, pinned) {
        return Err(Error::Runtime(
            "staged laboon archive changed after verification".to_owned(),
        ));
    }
    let program = resolve_tool(TAR_PATHS, "tar")?;
    file.seek(SeekFrom::Start(0))
        .map_err(|e| Error::Runtime(format!("rewind staged archive: {e}")))?;
    for member in list_members_from_fd(&program, &file)? {
        if !member_confined(&member) {
            return Err(Error::Runtime(format!(
                "laboon archive member escapes the extraction root: {member}"
            )));
        }
    }
    file.seek(SeekFrom::Start(0))
        .map_err(|e| Error::Runtime(format!("rewind staged archive: {e}")))?;
    let dest_arg = path_arg(dest)?;
    let args = vec![
        "-xf".to_owned(),
        "-".to_owned(),
        "-C".to_owned(),
        dest_arg,
        "--".to_owned(),
    ];
    let _ = run_tool_with_stdin(&program, &args, "tar", &file)?;
    Ok(())
}

fn sorted_entries(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    let reader = std::fs::read_dir(dir)
        .map_err(|e| Error::Runtime(format!("read {}: {e}", dir.display())))?;
    for item in reader {
        let entry = item.map_err(|e| Error::Runtime(format!("read {}: {e}", dir.display())))?;
        entries.push(entry.path());
    }
    entries.sort();
    Ok(entries)
}

/// Depth-bounded, sorted walk of the freshly extracted directory. Symlinks are
/// never followed, so a pinned archive cannot steer the search outside its
/// sandbox. Only the exact host binary name is accepted: a Windows
/// `laboon.exe` can never be installed as the Unix `laboon` binary.
fn locate_binary(root: &Path) -> Result<PathBuf> {
    let expected = binary_name();
    let mut stack: Vec<(PathBuf, usize)> = vec![(root.to_path_buf(), 0)];
    while let Some((dir, depth)) = stack.pop() {
        if depth > MAX_SEARCH_DEPTH {
            continue;
        }
        for path in sorted_entries(&dir)? {
            let meta = std::fs::symlink_metadata(&path)
                .map_err(|e| Error::Runtime(format!("stat {}: {e}", path.display())))?;
            let kind = meta.file_type();
            if kind.is_symlink() {
                continue;
            }
            let name = match path.file_name() {
                Some(n) => n.to_string_lossy().into_owned(),
                None => continue,
            };
            if kind.is_file() && name == expected {
                return Ok(path);
            }
            if kind.is_dir() {
                stack.push((path, depth.saturating_add(1)));
            }
        }
    }
    Err(Error::Runtime(
        "verified laboon archive contains no laboon binary for this host".to_owned(),
    ))
}

#[cfg(unix)]
fn set_read_only(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o400))
        .map_err(|e| Error::Runtime(format!("chmod {}: {e}", path.display())))
}

#[cfg(not(unix))]
fn set_read_only(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(unix)]
fn restrict_dir(dir: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
        .map_err(|e| Error::Runtime(format!("chmod {}: {e}", dir.display())))
}

#[cfg(not(unix))]
fn restrict_dir(_dir: &Path) -> Result<()> {
    Ok(())
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .map_err(|e| Error::Runtime(format!("laboon metadata: {e}")))?
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
        .map_err(|e| Error::Runtime(format!("laboon chmod: {e}")))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> Result<()> {
    Ok(())
}
