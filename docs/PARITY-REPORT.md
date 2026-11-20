# AWS Emulator parity report

Fork: dywongcloud/aws0 at ec5f6fbab9238221b966379f06846c34e9d6a6f5.
Upstream: aws/aws-emulator at the same commit. Zero diff at census time.

## 1. Method

The census is deterministic and exhaustive over five corpora:
572 GitHub issues (518 open, 54 closed), 625 issue comments,
5,355 open in-repo issue documents, 17,671 fixed in-repo issue documents,
and 7,114 operation source files carrying the SCAFFOLDED marker.
It produces 13,041 claim rows in claims.jsonl. Fourteen adversarial lenses
then read the claim ledger and the Rust source, and each finding carries
file:line evidence, an AWS reference, a severity and a verification label.
Independent refuter agents attack each finding. The LinkedIn post was read
verbatim through the logged-in browser and is stored at LINKEDIN-POST.txt.

## 2. The LinkedIn post, item by item

The post records 52 of 54 API checks passing, then zero Lambda bundles loading.
It lists five checks that real AWS rejects. Findings L5-01 through L5-12
evaluate each one against the source at fork HEAD:

- Check 1 (presigned S3 PUT with a different content-type): L5-01. refused 403 when the changed signed content-type header is present and auth is on.
- Check 2 (request without a valid SigV4 signature): L5-02. routed SigV4-modeled operations are refused 403.
- Check 3 (public function URL without lambda:InvokeFunctionUrl): L5-03. anonymous AuthType=NONE URL with no resource policy is refused 403 in the function-URL plane.
- Check 4 (EventBridge to SQS without a queue policy): L5-04. PutEvents rule with an SQS target and no Policy delivers nothing.
- Check 5 (call on a new DynamoDB table before ACTIVE): L5-05. a CREATING window exists; control-plane calls return ResourceInUseException.

The post says "Number 5 was a real bug in our code. Our emulator hid it for weeks."
The strict checks hold in code. The post also reports 0 of 3 event-flow checks
passing and Lambda bundles failing at load. L5-06 traces the bundle failure to
the QuickJS node shim: node:module, fetch, Headers, Request, Response,
structuredClone and WebAssembly are absent, and process.versions reports only
the node version. L5-07 records that the three event flows are not named in the post.

## 3. Verified root causes

Fourteen lens files contain 188 distinct findings. The highest-leverage ones:

- IC-01 [services/common/src/iam_authorization.rs]: An undecidable condition clause keeps a Deny standing. No service publishes aws:SecureTransport from the transport, so the AWS-recommended enforceSSL Deny refuses TLS traffic. Five separate GitHub issue families name this one defect.
- EF-01 [cross-service policy pre-checks]: Every service-to-service destination check treats enforceSSL as a refusal. The same root cause as IC-01.
- F-03 [S3 front door]: Every request carrying a cross-site Origin is refused before authentication, including the emulator own origin.
- F-06 [S3 credential scope]: The credential scope region is not validated against the bucket region; AWS answers 400 AuthorizationHeaderMalformed.
- DDB-02 [DynamoDB table lifecycle]: STATUS_TRANSITION is a hard-coded 3 seconds and DescribeTable sleeps out the CREATING span, so a client cannot observe the gated window that AWS exposes.
- LR-05 [Lambda embedded JS runtime]: createRequire from node:module fails at INIT, which is why Bun and esbuild ESM bundles fail to load.
- LR-06 [Lambda embedded JS runtime]: fetch, Headers, Request and Response are undefined in a Node Lambda.
- DX-CRED-001 [credentials]: Authentication mints a random key pair per process and accepts no operator-supplied pair.
- DX-TLS-001 [TLS]: One self-signed certificate per process, no configurable SAN and no stable CA.
- DX-STATE-001 [state]: All state lives in one process InMemoryEngine; no data-dir, snapshot or replication.
- STUB-CENSUS [7114 stub files]: 7,011 carry a true stub marker; 103 are false positives from a naive marker scan.

## 4. Coverage critic

COMPL-01: 490 of 13,041 claim rows (3.76 percent) are cited by another lens.
296 of 331 open-doc families, 43 of 63 GitHub families and 140 of 247 stub
families have zero cited claim rows. 17,671 fixed documents contribute zero
claim rows. 2,649 open documents carry more than one Issue-Key. The full
critique is in CRITIC.md. The complete ledger is BACKLOG.json.

## 5. Residual bound (no parity claim)

1:1 AWS parity is NOT achieved. Exact residual:

| surface | open at census |
|---|---:|
| in-repo issue documents | 5,355 |
| GitHub issues | 518 |
| operation files with a true stub marker | 7,011 |
| service crates | 440 |
| claim rows | 13,041 |

The external deep-research workflow (`wf_0d66ebf3-c33`, 99 agents, 17
primary/forum/blog sources, 13 confirmed findings and 12 refuted claims)
independently reached the same conclusion: upstream officially disclaims full
fidelity and states that real AWS, not upstream, is the correctness baseline.
Its receipt is DEEP-RESEARCH.md and DEEP-RESEARCH.json. It also records that
the fork is time-dependent: identical to upstream at census commit ec5f6fbab,
six commits ahead at fce38d0c0. Findings 1 and 2 of that report are the
authoritative scope statement; the product copy in README.md is positioning,
not a measured conformance result.

Full workspace build needs about 100 GiB for target/. Per-crate verification
with CARGO_PROFILE_DEV_DEBUG=0 and CARGO_INCREMENTAL=0 is the hardware-bounded
path used here. The complete family tables are in PARITY-CENSUS.md.

## 6. Delivered in this fork

- docs/PARITY-CENSUS.md, BACKLOG.json, PARITY-REPORT.md, CRITIC.md,
  LINKEDIN-POST.txt, WITNESS-OUTPUT.json, findings/*.json, contracts/*
- fabric/: the local-first, horizontally scalable cloud runtime. Real
  databases and containers, isolated regions, restart-safe state and
  self-healing capacity behind one command.
- WITNESS-OUTPUT.json: 36 live witnesses over real capacity, real persistence
  and the repaired security surfaces, all passing.
- fabric/runtime/: the six byte-verified runtime archives ship with the
  emulator, so a first boot needs no network. `preinstall_runtime` is true by
  default; `--no-preinstall-runtime` is the explicit opt-out.
- ADVERSARIAL-REFUTATION.md: the refutation ledger, the repairs made in
  response, and the residual holes that remain open.
- DEEP-RESEARCH.md, DEEP-RESEARCH.json: the independent multi-source research
  receipt, its 13 verified claims, 12 refuted claims, caveats and sources.

## 7. Provenance hashes

- census.md sha256 07deab7453739aeb74ab940d909ae2adfab15e4f5c01a11d3925cac7ca5da908
- claims.jsonl sha256 6a15b600b7a44a6f33601b8ce7d0d6ae7650de74a6ffeb716360d3e6f8167977
- linkedin-post.txt sha256 3a07f4c667808d453843cdad05eeeed5f037f5b8ca9bbee20a85ca000ac365da
- contracts/API-SHEET.md sha256 9b0cd6e4d735ac950174641410824579989175f567626d5c13b672b61b86c482
