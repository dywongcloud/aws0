# aws0

**A true 1:1 AWS. Local-first. Horizontally scalable.**
Built to give you the democracy to run your own cloud.

One command gives you an AWS-shaped endpoint on your own machine. Real
databases and containers boot as real processes. Add machines to add capacity.
Regions stay isolated, state survives restarts, and failed capacity heals
itself. No control plane to operate, no account, no subscription, no vendor.

## Quick start

```sh
cd fabric
cargo build --bins
cargo run --bin aws0 -- up
```

The workload runtime ships in the box, so the first boot needs no network.

## Use

```sh
aws0 up                                  # start your cloud on :4566
aws0 status                              # status
aws0 health                              # health
aws0 create pg postgres:16               # start a real PostgreSQL database
aws0 scale pg 3                          # change replica count
aws0 delete pg                           # stop and remove it
aws0 regions                             # available regions
aws0 reset                               # clear derived placement
aws0 witness                             # run the live verification harness
aws0 version
```

Options: `--data-dir <path>`, `--port <u16>`, `--instances <1..64>`,
`--no-preinstall-runtime`. `AWS0_CLUSTER_SECRET` supplies the instance key;
without it one is generated into `<data-dir>/cluster.key` with owner-only
permissions, so restarts keep the same identity.

## What you get

| capability | behaviour |
|---|---|
| AWS-compatible endpoint | one endpoint per region, ordinary AWS request shapes |
| Real workloads | PostgreSQL, MySQL, MariaDB, Redis, Valkey, MongoDB, DuckDB and Kafka run as real processes |
| Horizontal scale | add capacity; placement is deterministic and self-healing |
| Region isolation | a request is served inside its own region or refused |
| Restart-safe state | a checksummed journal and versioned snapshot restore every resource |
| Zero configuration | the verified workload runtime is bundled; nothing to install or operate |

## Verification

`aws0 witness` runs 36 live checks over real capacity, real persistence and
real workloads. The receipt is `docs/WITNESS-OUTPUT.json`. Conformance status
and the measured residual are recorded in `docs/PARITY-REPORT.md`; the
engineering ledger is `docs/ADVERSARIAL-REFUTATION.md`.
