# aws0-fabric

The self-hosted substrate behind the emulator. One command boots a multi-node,
multi-region cluster that runs real OCI workloads through [laboon](https://github.com/samyfodil/laboon),
replicates state over [iroh](https://github.com/n0-computer/iroh) QUIC, and
hides every peer-to-peer detail behind one ordinary HTTP endpoint.

There is no Kubernetes, no Docker daemon, no relay dependency for local mode,
and no user-visible P2P vocabulary.

## Build

`fabric/` is its own Cargo workspace. The root `aws0` workspace is generated
and must not be edited.

```sh
cd fabric
cargo build --bins
cargo run --bin aws0 -- up
```

The first boot needs no network: all six byte-verified laboon v0.1.0 release
archives are vendored in `fabric/runtime/`. `preinstall_runtime` defaults to
`true`; pass `--no-preinstall-runtime` to skip the install.

## Use

```sh
aws0 up                                  # boot the cluster and serve :4566
aws0 status                              # cluster status
aws0 health                              # GET /_aws0/health equivalent
aws0 create pg postgres:16               # start a real PostgreSQL guest
aws0 scale pg 3                          # change replica count
aws0 delete pg                           # stop and remove it
aws0 regions                             # configured regions
aws0 reset                               # clear derived placement only
aws0 witness                             # run the live verification harness
aws0 version
```

Options: `--data-dir <path>`, `--port <u16>`, `--nodes <1..64>`,
`--no-preinstall-runtime`. `AWS0_CLUSTER_SECRET` supplies the cluster key;
without it a 32-byte Ed25519 secret is generated into `<data-dir>/cluster.key`
with owner-only permissions, so a restart keeps the same node identities.

## What is inside

| module | responsibility |
|---|---|
| `types.rs` | validated identifiers, capabilities, HLC, the effect algebra |
| `crdt.rs` | `Lww`, `OrSet`, `GCounter`, version vectors, `ClusterState` |
| `state.rs` | the one pure `(view, event) -> (view', effects)` morphism |
| `membership.rs` | quorum, phi-accrual failure signals, gossip validation |
| `scheduler.rs` | rendezvous placement, deterministic self-healing |
| `routing.rs` | region isolation and capability-checked ownership |
| `persistence.rs` | checksummed journal, versioned snapshot, identity binding |
| `net.rs` | iroh QUIC endpoints, framed smartops, the IO executor |
| `runtime.rs` | laboon supervision, pinned artifacts, real database engines |
| `smartops.rs` | the closed, versioned smartop protocol |
| `facade.rs` | the one public surface; no node or QUIC vocabulary leaks |

## Verification

`aws0-fabric-witness` is a live harness. It opens real QUIC sockets, appends
real journal frames, extracts the pinned laboon archive, starts real OCI
workloads and checks 36 falsifiable properties. Its output is
`docs/WITNESS-OUTPUT.json`. `docs/ADVERSARIAL-REFUTATION.md` records the
refutation ledger and the residual holes; `docs/PARITY-REPORT.md` records the
exact residual bound for upstream AWS parity, which is NOT claimed complete.
