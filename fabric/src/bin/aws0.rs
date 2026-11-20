// SPDX-License-Identifier: CC0-1.0

//! `aws0` -- one command. `aws0 up` is all a user needs; the P2P substrate is
//! entirely hidden. `aws0 witness` runs the live verification suite.

use aws0_fabric::facade::{simple_workload, Fabric, FabricConfig};
use aws0_fabric::smartops::SmartOp;
use aws0_fabric::types::Result;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(String::as_str).unwrap_or("up");
    match command {
        "witness" => run_witness(),
        "up" => up(&args).await,
        "status" | "health" | "reset" | "create" | "scale" | "delete" | "regions" => {
            admin(command, &args).await
        }
        "version" => {
            println!("aws0-fabric 0.1.0 / iroh 1.5.0 / laboon 0.1.0");
            Ok(())
        }
        other => {
            println!("unknown command: {other}");
            println!("usage: aws0 up | status | create <name> <image> | scale <name> <n> | delete <name> | regions | reset | witness | version");
            Ok(())
        }
    }
}

/// Run the live witness suite in-process. The witness is a library-style
/// executable in this crate; we invoke the compiled sibling by its explicit
/// path so no compile-time environment injection is needed.
fn run_witness() -> Result<()> {
    let exe = std::env::current_exe().map_err(|e| aws0_fabric::types::Error::Runtime(e.to_string()))?;
    let sibling = exe.with_file_name("aws0-fabric-witness");
    let status = std::process::Command::new(sibling)
        .status()
        .map_err(|e| aws0_fabric::types::Error::Runtime(e.to_string()))?;
    if status.success() {
        Ok(())
    } else {
        Err(aws0_fabric::types::Error::Runtime("witness failed".to_owned()))
    }
}

fn config_from_env(args: &[String]) -> Result<FabricConfig> {
    let mut c = FabricConfig::default();
    c.data_dir = args
        .iter()
        .position(|a| a == "--data-dir")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::var("AWS0_DATA_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(".aws0")));
    c.port = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(4566);
    c.nodes = args
        .iter()
        .position(|a| a == "--nodes")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(3);
    // Preinstall is the default: the pinned runtime archives ship with the
    // emulator. `--no-preinstall-runtime` is the explicit opt-out.
    c.preinstall_runtime = !args.iter().any(|a| a == "--no-preinstall-runtime");
    c.cluster_secret = cluster_secret(&c.data_dir)?;
    Ok(c)
}

/// Resolve the cluster key without ever embedding one in the binary. Precedence:
/// `AWS0_CLUSTER_SECRET` from the environment, else a 32-byte key persisted in
/// the data directory, else an OS-generated Ed25519 secret written there so a
/// restart keeps the same cluster identity. The file is owner-only on Unix.
fn cluster_secret(data_dir: &std::path::Path) -> Result<Vec<u8>> {
    let io = |e: std::io::Error| aws0_fabric::types::Error::Journal(e.to_string());
    if let Ok(value) = std::env::var("AWS0_CLUSTER_SECRET") {
        if value.len() < 16 {
            return Err(aws0_fabric::types::Error::Protocol(
                "AWS0_CLUSTER_SECRET must be at least 16 bytes".to_owned(),
            ));
        }
        return Ok(value.into_bytes());
    }
    let path = data_dir.join("cluster.key");
    if let Ok(text) = std::fs::read_to_string(&path) {
        let bytes = hex::decode(text.trim().as_bytes()).map_err(|e| {
            aws0_fabric::types::Error::Protocol(e.to_string())
        })?;
        if bytes.len() < 16 {
            return Err(aws0_fabric::types::Error::Protocol(
                "persisted cluster key is shorter than 16 bytes".to_owned(),
            ));
        }
        return Ok(bytes);
    }
    std::fs::create_dir_all(data_dir).map_err(io)?;
    let bytes = iroh::SecretKey::generate().to_bytes().to_vec();
    std::fs::write(&path, hex::encode(&bytes)).map_err(io)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).map_err(io)?;
    }
    Ok(bytes)
}

async fn up(args: &[String]) -> Result<()> {
    let config = config_from_env(args)?;
    let fabric = Fabric::boot(config).await?;
    println!("aws0 up: {} nodes across {} regions, endpoint {}", fabric.nodes.len(), fabric.config.regions.len(), fabric.endpoint_url());
    let health = fabric.health().await;
    println!("{}", serde_json::to_string_pretty(&health).map_err(|e| aws0_fabric::types::Error::Protocol(e.to_string()))?);
    fabric.serve().await
}

async fn admin(command: &str, args: &[String]) -> Result<()> {
    let config = config_from_env(args)?;
    let fabric = Fabric::boot(config).await?;
    let account = args.get(2).map(String::as_str).unwrap_or("000000000000");
    let region = args.get(3).map(String::as_str).unwrap_or("us-east-1");
    match command {
        "status" | "health" => {
            let h = fabric.health().await;
            println!("{}", serde_json::to_string_pretty(&h).map_err(|e| aws0_fabric::types::Error::Protocol(e.to_string()))?);
        }
        "regions" => {
            for r in &fabric.config.regions {
                println!("{r}");
            }
        }
        "reset" => {
            fabric.reset().await?;
            println!("{{\"reset\":\"ok\"}}");
        }
        "create" => {
            let name = args.get(2).map(String::as_str).unwrap_or("demo");
            let image = args.get(3).map(String::as_str).unwrap_or("postgres:16");
            let spec = simple_workload(name, image, "000000000000", "us-east-1", 1)?;
            let reply = fabric.smart("000000000000", "us-east-1", SmartOp::Create(spec), None).await?;
            println!("{}", serde_json::to_string_pretty(&reply).map_err(|e| aws0_fabric::types::Error::Protocol(e.to_string()))?);
        }
        "scale" => {
            let name = args.get(2).map(String::as_str).unwrap_or("demo");
            let n = args.get(3).and_then(|s| s.parse::<usize>().ok()).unwrap_or(2);
            let id = aws0_fabric::types::WorkloadId::new(name)?;
            let reply = fabric.smart(account, region, SmartOp::Scale { id, replicas: n }, None).await?;
            println!("{}", serde_json::to_string_pretty(&reply).map_err(|e| aws0_fabric::types::Error::Protocol(e.to_string()))?);
        }
        "delete" => {
            let name = args.get(2).map(String::as_str).unwrap_or("demo");
            let id = aws0_fabric::types::WorkloadId::new(name)?;
            let reply = fabric.smart(account, region, SmartOp::Delete(id), None).await?;
            println!("{}", serde_json::to_string_pretty(&reply).map_err(|e| aws0_fabric::types::Error::Protocol(e.to_string()))?);
        }
        _ => {}
    }
    fabric.shutdown().await
}
