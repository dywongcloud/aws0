# Adversarial refutation and repair ledger

Status: 36 of 36 live witnesses pass. This document records what the
adversarial refuters found, what was actually repaired, and what remains open.
It does not claim 1:1 AWS parity. See PARITY-REPORT.md section 5 for the exact
residual bound.

## 1. Refutation run

Workflow `wf_c9b24e53-637` (17 agents, 2,859,847 tokens) refuted the first
substrate implementation. Its receipt is `w4puejyqn.output`. It closed 3 of 3
CRDT claims as NOT closed, 3 of 4 state claims, 8 of 9 routing claims, 9 of
12 persistence claims, 6 of 9 runtime claims and 0 of the facade claims. A
second pass produced 15 new regressions from the attempted fixes themselves.

## 2. Regressions found in the attempted fixes, and their repairs

| # | defect | repair |
|---:|---|---|
| 1 | `authorize()` denied every event without an inline partition, so tenant scale/delete were dead | partition is now resolved from the replicated workload registry; `Scope::Workload(id)` covers exactly that workload |
| 2 | `PlacementSet` trusted a caller-supplied partition and could place tenant B's workload on an attacker-chosen node | the placement partition must equal the registered workload's partition |
| 3 | `Suspect`/`ConfirmDead` never re-placed workloads; `membership::tick` had zero call sites | a real background heartbeat loop applies the failure detector; confirmed deaths call the deterministic scheduler |
| 4 | `seen_events` grew one permanent key per heartbeat and was serialized into every snapshot | bounded to 4,096 keys with an insertion-order queue; the persisted View is checked against the 1 MiB frame bound |
| 5 | a late `ConfirmDead` removed a re-joined incarnation | death signals carry the observed incarnation and are applied only when the live record is that incarnation or older |
| 6 | a future-dated heartbeat pinned liveness forever | heartbeat payloads are clamped to the apply clock; incoming gossip clocks beyond a 60 s skew are refused |
| 7 | event-key format changed without a migration | format is unchanged; the doc now states that every failure signal carries an incarnation, so distinct observations get distinct keys |
| 8 | `OrSet`/`Lww` ties still resolved by merge order | a `CanonicalRank` trait ranks every field; `max` over that rank is symmetric |
| 9 | lexicographic JSON bytes let a stale 8192 MB member beat a fresh 4096 MB one | `MemberRecord` ranks incarnation and heartbeat before capacity; `check_member_freshness` witnesses both merge orders |
| 10 | `OrSet::add` overwrote values unconditionally, so anti-entropy oscillated | `add` is inflationary: dots always grow, values only rise in rank |
| 11 | the law check never built conflicting `MemberRecord` values inside `ClusterState` | `divergent_members()` builds exactly that triple and `check_join_laws` checks all three laws on it |
| 12 | a `T: Debug` bound broke non-Debug payloads and made every join O(total state) | the bound is gone; ranks are computed only for conflicting keys |
| 13 | fresh boot failed because no code wrote `node.id` | `bind_identity` publishes the derived id on first boot and rejects a foreign id later |
| 14 | an operator-supplied `node.id` became a phantom second identity | `Fabric::boot` refuses a recovered view whose `self_id` differs from the derived id |
| 15 | admin smartops were unreachable because the facade minted only `Tenant` caps | scopes are minted internally from the operation class; callers never choose a scope |

## 3. Original defects also closed in this pass

- Snapshot envelope metadata (`journal_frames`, `journal_bytes`,
  `last_applied_ms`) is now covered by a domain-separated envelope hash.
  Tampering with `journal_frames` to make a stale snapshot outrank newer
  frames is refused. Witness: `persistence-integrity`.
- Every complete journal frame is decoded as a canonical `View`, not just the
  trailing 32. An old checksum-valid malformed frame aborts recovery instead
  of being counted as durable state. Witness: `persistence-integrity`.
- `AccountId`, `Region` and `WorkloadId` deserialize through their validating
  constructors, closing the NUL-injection path into `routing_key`.
- The 1 MiB output cap is a typed refusal, not a silent truncation; the
  combined stdout plus stderr total is enforced. Witness:
  `runtime-output-cap-typed`.
- The executed laboon binary is a private copy whose bytes are hashed through
  the same handle used to write it and re-hashed immediately before spawn. A
  writer that replaces `bin/laboon` after verification cannot change the bytes
  that run. Witness: `runtime-verified-exec`.
- Archive hashing, member listing and extraction all read from ONE open file
  descriptor passed to bsdtar on standard input. No tool re-opens the mutable
  pathname. Witness: `runtime-bundled-archive`.
- Identity, recovery and authorization witnesses: `fresh-boot-identity`,
  `membership-incarnation-guard`, `seen-events-bounded`,
  `facade-tenant-scale`, `facade-tenant-delete`, `facade-admin-join`.

## 4. Residual holes (open, not claimed closed)

- `build_report`'s recovered-state digest comparison is tautological: it
  compares a digest of the durable bytes with a digest derived from the same
  bytes after a canonical round-trip. A single-node journal has no external
  root of trust to attest a different expected hash. Closing this requires an
  operator-supplied root or a quorum-signed head, not another local file.
- A same-UID attacker with `chmod` rights can still alter the private exec
  directory; the remaining window is an OS privilege boundary, not something
  this crate can close with a path name.
- `Fabric::smart` is an in-process trusted entry point. Any code inside the
  process can name any configured account. The HTTP front door exposes only
  health, status and reset, so no remote caller mints capabilities today; a
  full SigV4-authenticated front door is still required for zero-trust at the
  network boundary.
- The event-key replay window is finite (4,096 events). Exactly-once holds
  within that window; a replay older than the window is treated as a new
  observation.
- 1:1 AWS parity is NOT achieved: 5,355 open in-repo issue documents, 518 open
  GitHub issues and 7,011 true marker stubs remain. Only the SNS empty
  `NextToken` defect (GH #479) has a live-witnessed upstream code fix in this
  fork. The LinkedIn post's five checks are documented in PARITY-REPORT.md,
  not all implemented.

## 5. Witness receipt

`WITNESS-OUTPUT.json` carries the 36 `(id, ts, hash)` records from a single
live run of `aws0-fabric-witness`: real iroh QUIC endpoints, real journal and
snapshot I/O, real laboon OCI workloads, and the adversarial cases above.
`cargo check --offline --all-targets` and `cargo build --offline --bins` both
exit 0. The witness binary is a live harness, not a test suite on disk.

## 6. Supply-chain check

gm `scan_deps {"full":true}` returns `ok:true` with `filesScanned:0` for this
checkout, including when scoped to `fabric` and `docs`. A scan of zero files is
not evidence, so the two structural checks were run independently over
`git ls-files`: 91,184 tracked paths, 91,178 decoded as text. Zero dense
unicode-escape runs decoding to an identifier shape (the HiddenSpawn
corroborator). 462 files trip the size-to-line ratio heuristic alone; all are
single-line minified JSON data with no escape-density corroboration, which is
the documented benign case. `blockedCount` and `failCount` are both zero. The
`filesScanned:0` behaviour of the served verb is reported as a tooling gap,
not treated as a passed gate. Re-probed after the disk-full interruption with
142 GiB free: `ok:true`, `filesScanned:0`, `failCount:0`, `blockedCount:0`,
`nodeModulesPresent:false` (dispatch `1795216036293-375-e499b16ca95a5705`).
The zero-file result is therefore persistent, not an ENOSPC artifact.
