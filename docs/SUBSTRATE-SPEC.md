# SUBSTRATE-SPEC: self-hosted AWS behind one command

## Decision

Five independent designs were generated and judged. The winner is the
CRDT-first replicated state algebra, because it is the only design whose state
plane compiles against one coherent version of every iroh crate. Grafts carried
forward: laboon-first makes a real workload boot a hard pivot gate;
capability-first contributes a two-workspace compilation fence between the pure
kernel and the iroh adapter; dx-magic-first contributes the single front door.

## Contract

| element | value |
|---|---|
| transport | iroh 1.5.0 QUIC, ALPN aws0/smartops/1 |
| identity | Ed25519 endpoint key; NodeId IS the public key |
| state | ClusterState: OrSet members, Lww workloads/placements/epochs, GCounter, VersionVector, applied ledger |
| clock | Hlc(wall_ms, counter, node) total order |
| membership | phi-accrual suspicion, heartbeat, incarnation, quorum |
| scheduling | rendezvous hashing, region affinity, capacity-aware, f(f(x))=f(x) |
| routing | capability-gated, region-closed, length-prefixed keys |
| persistence | checksummed journal frames + checksummed snapshot, corrupt-tail truncation |
| runtime | laboon 0.1.0, six pinned SHA-256 artifacts, eight real database images |
| protocol | SmartOp enum, framed JSON, exactly-once idempotency ledger |
| facade | aws0 up; GET /_aws0/health, /_aws0/status; POST /_aws0/reset |

## Dependency set

iroh 1.5.0, iroh-blobs 0.103.1, iroh-docs 0.101.0, iroh-gossip 0.101.0,
quic-rpc 0.20.0 with flume-transport and iroh-transport disabled.
iroh-blobs 0.103.0 is yanked, so 0.103.1 is pinned.
iroh-roq (iroh ^0.33), iroh-dag-sync (iroh 1.0.0-rc.1) and sendme 0.37.1
(binary-only) are documented as incompatible and excluded from the build.
The full API contract sheet is docs/contracts/API-SHEET.md.

## Laboon runtime

laboon 0.1.0 was downloaded, SHA-256 verified against the release checksums.txt
(darwin_arm64 1116d234f49b3f5f3c7ce363ddb98c4d084a4e6d11342a613386a4d89203f279),
extracted and executed: ./laboon version -> laboon 0.1.0;
./laboon -image alpine:3.20 -mem 256M -no-net cat /etc/os-release returned
NAME="Alpine Linux", VERSION_ID=3.20.10. Database specs carry eight real OCI
images and readiness commands.

## Verification

25 live witnesses in docs/WITNESS-OUTPUT.json: CRDT join laws, OR-set observed
remove, LWW symmetry, HMAC zero-trust, total constructors, exactly-once replay,
scheduler fixed point, placement, journal restart, corrupt-tail recovery,
three-node convergence over real QUIC sockets, live smartops RPC, wire replay,
wire zero-trust, truncated frame handling, facade health/convergence/create,
multi-region, tenant isolation, snapshot, pinned artifacts, real databases,
preinstall idempotence and chaos p99 under 256 operations.

## Residual

The fabric is a nested workspace and does not modify the generated root
Cargo.toml, core/router or vendor/. It composes with the emulator; it does not
replace 7,011 stubbed service operations. 1:1 parity is NOT achieved; the exact
residual is enumerated in docs/BACKLOG.json.

## Machine-readable spec

docs/design/final-spec.json. Source designs: docs/design/*.json.
