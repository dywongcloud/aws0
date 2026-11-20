# Authoritative API Contract Sheet -- iroh P2P substrate

Generated 2026-11-19 from the `aws0-substrate-api-contracts` workflow (`Extract` -> `Refute` -> `Compile`).

> **BLUF.** All 14 extracted contracts survived adversarial source refutation (`refuted=false`, `wrong_signatures=[]`), giving **800 source-verified signatures** and **73 unverified items**. There are **zero rejected contracts**. The smallest five-crate set that actually builds with ONE version of each crate is `iroh =1.5.0`, `iroh-blobs =0.103.1`, `iroh-docs =0.101.0`, `iroh-gossip =0.101.0`, `quic-rpc =0.20.0` with `iroh-transport` disabled; it was witnessed with `cargo check --offline` (exit 0). Using the sheet's contracted `iroh-blobs 0.105.0` / `iroh-gossip 0.103.0` together with `iroh-docs 0.101.0` is a **hard blocker** (duplicate, type-incompatible version universes).

## 0. Provenance, method, and scope

- Workflow script: `/Users/dylanwong/.tclaude/projects/-Users-dylanwong/1fbb9576-9f61-4529-ac97-efd40ba18afd/workflows/scripts/aws0-substrate-api-contracts-wf_f0a101ff-744.js`
- Structured workflow journal (all `extract:*` and `refute:*` results): `/Users/dylanwong/.tclaude/projects/-Users-dylanwong/1fbb9576-9f61-4529-ac97-efd40ba18afd/subagents/workflows/wf_f0a101ff-744/journal.jsonl`
- On-disk contract files: `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/<crate>.json` (14 files)
- The compile prompt embedded only the first 60,000 characters of the surviving-contract JSON array (`JSON.stringify(...).slice(0, 60000)`), which truncates mid-way through the second contract. The complete surviving set below was recovered from the workflow journal's structured results and cross-checked against all 14 on-disk contract files: exact_version, signature count and unverified count match for every crate.
- Counts: 14 surviving contracts, 14 refutation verdicts, 800 signatures, 73 unverified items, 0 signatures with `verified=false`, 293 distinct source URLs.

| crate | on-disk file matched journal | signatures (journal / disk) |
|---|---|---|
| `net-tools` | True | 8 / 8 |
| `tokio-rustls-acme` | True | 80 / 80 |
| `iroh-gossip` | True | 48 / 48 |
| `sendme` | True | 35 / 35 |
| `iroh-dag-sync` | True | 59 / 59 |
| `iroh-blobs` | True | 90 / 90 |
| `n0-dns-resolver` | True | 140 / 140 |
| `iroh-docs` | True | 73 / 73 |
| `iroh-roq` | True | 24 / 24 |
| `ministack` | True | 42 / 42 |
| `iroh-smol-kv` | True | 57 / 57 |
| `laboon-cli` | True | 26 / 26 |
| `quic-rpc` | True | 37 / 37 |
| `iroh` | True | 81 / 81 |

Every signature below is reproduced **verbatim** from its contract's `signature` field, with its `source` field (local path, docs.rs URL, GitHub tag path) reproduced verbatim. Contract-level `source_refs` are reproduced in full. No signature was inferred from memory.

## 1. SURVIVING CONTRACTS (authoritative)

### 1.1 `net-tools` -- exact version `0.20.2`

- **License:** MIT OR Apache-2.0 (net-tools workspace, portmapper, netwatch, nested_enum_utils); netdev 0.47.0 is MIT
- **MSRV / Go:** 1.91 (portmapper 0.20.2 and netwatch 0.20.2; crates.io index rust_version=1.91); netdev 0.47.0 and nested_enum_utils 0.2.3 declare no rust_version
- **Signatures:** 8; **unverified items:** 4; **source refs:** 63

**Feature flags / version gates**

- `portmapper 0.20.2: default = ["metrics"]; metrics = ["iroh-metrics/metrics"]`
- `netwatch 0.20.2: no features declared in crates.io index`
- `netdev 0.47.0: default = ["gateway", "apple-system-configuration-extra", "android-extra"]; optional features gateway, serde, apple-wifi-extra, apple-system-configuration-extra, android-extra`
- `nested_enum_utils 0.2.3: no features declared in crates.io index`

**Integration notes (verbatim)**

````text
n0-computer/net-tools is a Cargo workspace, not a published crate: https://index.crates.io/ne/t-/net-tools returned HTTP 404. HEAD is 40fc25830973a357d99be2b1d5ccc01c3c9f3558 (2026-11-19T23:02:44Z); tags portmapper-v0.20.2 and netwatch-v0.20.2 both point to commit 334793cfdaf3487db0c721b9312179a8ae3b0dcb. Root Cargo.toml at that tag lists exactly two workspace members: portmapper and netwatch. Exact crates.io sparse-index versions: portmapper 0.20.2 (cksum d36ce759df600731dd2511cd2d640bd551086b2f7b68bceb9bbcfd8c7ff86053), netwatch 0.20.2 (cksum 2acf7a1ca9205d7c1fc9a1595193a8cbaa7537b4a07dca93c6ae3de7c8d0e597), netdev 0.47.0 (cksum 98a4d13f8061600697d721c5fd45262aba533a7d4a7670e7c64fd02d68e4a9fb), nested_enum_utils 0.2.3 (cksum b1d5475271bdd36a4a2769eac1ef88df0f99428ea43e52dfd8b0ee5cb674695f). All four tarballs were downloaded from static.crates.io and their SHA-256 sums matched the index checksums; portmapper/netwatch source files at the GitHub tag are byte-identical to the published crate source files. Subcrate classification: portmapper 0.20.2 and netwatch 0.20.2 are workspace members; netdev 0.47.0 is an external dependency of netwatch (locked in Cargo.lock; repository https://github.com/shellrow/netdev) and NOT a workspace member; nested_enum_utils 0.2.3 is a separate n0-computer/nested-enum-utils proc-macro crate and is NOT a workspace member, NOT in Cargo.lock, and has 0 code-search hits in net-tools. No netdev/Cargo.toml or nested_enum_utils/Cargo.toml path exists in repository commit history; tag portmapper-v0.1.0 also lists only portmapper and netwatch. Integration: use portmapper = \"0.20.2\" (default feature metrics; disable with default-features = false) for UPnP/PCP/NAT-PMP traversal and netwatch = \"0.20.2\" for interface/route discovery; use netdev = \"0.47.0\" only if raw interface enumeration is needed; nested_enum_utils is not needed for NAT/interface discovery. The example portmapper/examples/monitor_and_probe.rs wires netwatch::netmon::Monitor::interface_state() to portmapper::Client::probe(). Contract written to /Users/dylanwong/projects/aws0-research-2026-11-19/contracts/net-tools.json. gm terminal gate surfaced: session owner 1fbb9576-9f61-4529-ac97-efd40ba18afd, phase JTBD, foreign PRD row aws0-parity-census pending; left pending, not resolved.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `portmapper::Client::new` | `pub fn new(config: Config) -> Self` | `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb (lines 163-167); https://docs.rs/portmapper/0.20.2/portmapper/struct.Client.html; /tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/src/lib.rs` | True |
| 2 | `portmapper::Client::probe` | `pub fn probe(&self) -> oneshot::Receiver<Result<ProbeOutput, ProbeError>>` | `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb (lines 187-211); https://docs.rs/portmapper/0.20.2/portmapper/struct.Client.html; /tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/src/lib.rs` | True |
| 3 | `portmapper::Client::watch_external_address` | `pub fn watch_external_address(&self) -> watch::Receiver<Option<SocketAddrV4>>` | `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb (lines 246-249); /tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/src/lib.rs` | True |
| 4 | `netwatch::interfaces::State::new` | `pub async fn new() -> Self` | `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/interfaces.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb (lines 294-300); https://docs.rs/netwatch/0.20.2/netwatch/interfaces/struct.State.html; /tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/interfaces.rs` | True |
| 5 | `netwatch::interfaces::HomeRouter::new` | `pub fn new() -> Option<Self>` | `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/interfaces.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb (lines 403-411); /tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/interfaces.rs` | True |
| 6 | `netwatch::netmon::Monitor::new` | `pub async fn new() -> Result<Self, Error>` | `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/netmon.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb (lines 55-71); /tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/netmon.rs` | True |
| 7 | `netdev::interface::get_interfaces (re-exported as netdev::get_interfaces)` | `pub fn get_interfaces() -> Vec<Interface>` | `https://static.crates.io/crates/netdev/netdev-0.47.0.crate (extracted src/interface/mod.rs lines 24-29); https://docs.rs/netdev/0.47.0/netdev/interface/fn.get_interfaces.html; /tmp/net-tools-research-20261119/crates/netdev-0.47.0/netdev-0.47.0/src/interface/mod.rs` | True |
| 8 | `nested_enum_utils::enum_conversions (NOT a net-tools subcrate; no NAT/interface-discovery API exists in this crate)` | `#[proc_macro_attribute] pub fn enum_conversions(attr: TokenStream, item: TokenStream) -> TokenStream` | `https://static.crates.io/crates/nested_enum_utils/nested_enum_utils-0.2.3.crate (extracted src/lib.rs lines 205-206); https://docs.rs/nested_enum_utils/0.2.3/nested_enum_utils/attr.enum_conversions.html; /tmp/net-tools-research-20261119/crates/nested_enum_utils-0.2.3/nested_enum_utils-0.2.3/src/lib.rs` | True |

**Contract source refs**

- `https://api.github.com/repos/n0-computer/net-tools/git/trees/HEAD?recursive=1`
- `https://api.github.com/repos/n0-computer/net-tools/commits/HEAD`
- `https://api.github.com/repos/n0-computer/net-tools/tags`
- `https://api.github.com/repos/n0-computer/net-tools/contents/Cargo.toml`
- `https://api.github.com/repos/n0-computer/net-tools/contents/README.md`
- `https://api.github.com/repos/n0-computer/net-tools/contents/Cargo.toml?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/README.md?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/Cargo.lock?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/Cargo.toml?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/README.md?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/examples/monitor_and_probe.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/Cargo.toml?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/README.md?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/interfaces.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/netmon.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/interfaces/netdev_impl.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb`
- `https://api.github.com/repos/n0-computer/net-tools/commits?path=netdev/Cargo.toml&per_page=5`
- `https://api.github.com/repos/n0-computer/net-tools/commits?path=nested_enum_utils/Cargo.toml&per_page=5`
- `https://api.github.com/repos/n0-computer/net-tools/git/trees/89782fb542d217fd0d59975e13c828f4a1adf0ae?recursive=1`
- `https://api.github.com/search/code?q=repo%3An0-computer%2Fnet-tools+nested_enum_utils`
- `https://api.github.com/search/code?q=repo%3An0-computer%2Fnet-tools+netdev`
- `https://index.crates.io/po/rt/portmapper`
- `https://index.crates.io/ne/tw/netwatch`
- `https://index.crates.io/ne/td/netdev`
- `https://index.crates.io/ne/st/nested_enum_utils`
- `https://index.crates.io/ne/t-/net-tools (HTTP 404)`
- `https://static.crates.io/crates/portmapper/portmapper-0.20.2.crate`
- `https://static.crates.io/crates/netwatch/netwatch-0.20.2.crate`
- `https://static.crates.io/crates/netdev/netdev-0.47.0.crate`
- `https://static.crates.io/crates/nested_enum_utils/nested_enum_utils-0.2.3.crate`
- `https://docs.rs/portmapper/0.20.2/portmapper/`
- `https://docs.rs/portmapper/0.20.2/portmapper/struct.Client.html`
- `https://docs.rs/netwatch/0.20.2/netwatch/`
- `https://docs.rs/netwatch/0.20.2/netwatch/interfaces/struct.State.html`
- `https://docs.rs/netdev/0.47.0/netdev/`
- `https://docs.rs/netdev/0.47.0/netdev/interface/fn.get_interfaces.html`
- `https://docs.rs/nested_enum_utils/0.2.3/nested_enum_utils/`
- `https://docs.rs/nested_enum_utils/0.2.3/nested_enum_utils/attr.enum_conversions.html`
- `/tmp/net-tools-research-20261119/tree.json`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/Cargo.toml`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/README.md`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/Cargo.lock`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/Cargo.toml`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/README.md`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/src/lib.rs`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/portmapper/examples/monitor_and_probe.rs`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/Cargo.toml`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/README.md`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/lib.rs`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/interfaces.rs`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/netmon.rs`
- `/tmp/net-tools-research-20261119/sources/tag-v0.20.2/netwatch/src/interfaces/netdev_impl.rs`
- `/tmp/net-tools-research-20261119/crates/portmapper-0.20.2/portmapper-0.20.2/src/lib.rs`
- `/tmp/net-tools-research-20261119/crates/netwatch-0.20.2/netwatch-0.20.2/src/interfaces.rs`
- `/tmp/net-tools-research-20261119/crates/netwatch-0.20.2/netwatch-0.20.2/src/netmon.rs`
- `/tmp/net-tools-research-20261119/crates/netdev-0.47.0/netdev-0.47.0/Cargo.toml`
- `/tmp/net-tools-research-20261119/crates/netdev-0.47.0/netdev-0.47.0/src/lib.rs`
- `/tmp/net-tools-research-20261119/crates/netdev-0.47.0/netdev-0.47.0/src/interface/mod.rs`
- `/tmp/net-tools-research-20261119/crates/nested_enum_utils-0.2.3/nested_enum_utils-0.2.3/Cargo.toml`
- `/tmp/net-tools-research-20261119/crates/nested_enum_utils-0.2.3/nested_enum_utils-0.2.3/src/lib.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/net-tools.json`

### 1.2 `tokio-rustls-acme` -- exact version `0.10.0`

- **License:** Apache-2.0 OR MIT
- **MSRV / Go:** No declared MSRV: Cargo.toml at tag v0.10.0 has no rust-version key and https://crates.io/api/v1/crates/tokio-rustls-acme/0.10.0 reports rust_version: null; edition = "2024" (Cargo.toml:8). Go version: n/a (Rust crate). No concrete minimum rustc version is asserted.
- **Signatures:** 80; **unverified items:** 4; **source refs:** 28

**Feature flags / version gates**

- `default`
- `axum`
- `tls-webpki-roots`
- `tls-ring`
- `tls-aws-lc-rs`

**Integration notes (verbatim)**

````text
PROVENANCE. crates.io index (https://index.crates.io/to/ki/tokio-rustls-acme) lists 0.10.0 as the newest of 14 versions, yanked=false, cksum=14303ebe76ff697adc251341c800c56e7a6a15c53dcd6e45986dbc291e2ae9a4, pubtime 2026-11-19T02:01:43Z; crates.io API reports license="Apache-2.0 OR MIT", repository=https://github.com/n0-computer/tokio-rustls-acme, rust_version=null, edition=2024. The downloaded .crate sha256 equals that cksum; its .cargo_vcs_info.json says git.sha1=4017266013e0bd00bae83e9edc410f3624f80815, which is exactly tag v0.10.0 AND the current main HEAD. All 23 compared src/, examples/ and README.md files are byte-identical between the published crate and the GitHub tag; only Cargo.toml differs, by Cargo's publish-time normalization (Cargo.toml.orig retains the original). Every `source` path in `signatures` is an absolute local path of a base64-decoded `gh api repos/n0-computer/tokio-rustls-acme/contents/<path>?ref=v0.10.0` blob; the corresponding https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/<path> was fetched and byte-compared for all 16 key files. No signature comes from docs.rs (see UNVERIFIED).

FEATURES (Cargo.toml:92-104 and index features2, exact). default = ["tls-webpki-roots","tls-ring"]; axum = ["dep:axum-server"]; tls-webpki-roots = ["dep:webpki-roots"]; tls-ring = ["dep:ring","rcgen/ring","rustls/ring"]; tls-aws-lc-rs = ["dep:aws-lc-rs","rcgen/aws_lc_rs","rustls/aws-lc-rs"]. src/crypto.rs:11-12 has compile_error! when neither tls-ring nor tls-aws-lc-rs is enabled; with both, aws-lc-rs wins. AcmeConfig::new and AcmeConfig::new_with_crypto_provider are #[cfg(feature = "tls-webpki-roots")]; AcmeConfig::new_with_client_tls_config is unconditional. The `axum` module, AxumAcceptor/AxumAccept, and AcmeState::axum_acceptor are #[cfg(feature = "axum")]. Dev-dependency axum 0.8 backs the axum example; the library's optional integration is axum-server 0.8 with feature "tokio-rustls". The crate uses one rustls::crypto::CryptoProvider end-to-end taken from the ACME ClientConfig; it never reads/installs the process-wide default (src/lib.rs:14-38).

DIRECTORIES / DOMAINS / CONTACTS. new_with_client_tls_config defaults directory_url to LETS_ENCRYPT_STAGING_DIRECTORY (src/config.rs:117, src/acme.rs:24-27); all examples use staging unless `--prod` calls directory_lets_encrypt(true). Switch with directory(url) or directory_lets_encrypt(bool). Domains come from new(domains)/domains(iter)/domains_push(str); they become SANs via CertificateParams::new(config.domains.clone()) (src/state.rs:279) and each pending authorization is solved with tls-alpn-01 (src/state.rs:333-353). Note the source names the `domains`/`domains_push` parameter `contact` (cosmetic only). Contacts come from contact(iter)/contact_push(str) and MUST include a `mailto:` prefix (src/config.rs:162-176); examples map emails with format!("mailto:{e}"). External account binding: external_account_binding(kid, key) -> acme::ExternalAccountKey::new(kid: String, key: &[u8]).

CACHE DIR. caches::DirCache::new("./rustls_acme_cache") (doc example src/lib.rs:66-69) or cache_option(Some(DirCache::new(path_buf))) as in examples/high_level.rs:45; also cache(C), cache_compose(cert_cache, account_cache), cache_with_boxed_err(C) -> AcmeConfig<Box<dyn Debug>> to erase cache error types. DirCache::new takes any P: AsRef<Path> + Send + Sync; both cache error types are std::io::Error. It writes cached_account_<base64url-nopad SHA256(contact, directory_url)> and cached_cert_<...domains, directory_url>, calls tokio::fs::create_dir_all on write, maps NotFound on read to Ok(None) (src/caches/dir.rs:18-60, 63-106). NoCache is the default (src/config.rs:120): production needs both account and cert caching to avoid Let's Encrypt rate limits (src/lib.rs:104-110). Custom caches implement Cache (blanket impl) or CertCache + AccountCache, which are #[async_trait] traits (src/cache.rs).

HIGH-LEVEL TOKIO. AcmeConfig::new(...).incoming(TcpListenerStream::new(listener), Vec::new()) yields Incoming, a Stream + FusedStream of Result<tokio_rustls::server::TlsStream<TCP>, ETCP> (src/incoming.rs:84-153). alpn_protocols: Vec<Vec<u8>> is the APPLICATION ALPN list, most preferred first; empty Vec disables ALPN (src/config.rs:214-216, src/incoming.rs:72-74). Polling Incoming also polls AcmeState, so certificate acquisition/renewal is folded into the accept loop; no persistent tasks are spawned (src/lib.rs:8-9). TLS handshake errors are only logged and dropped (src/incoming.rs:109-111, 117-119), so the Item error type ETCP is the TCP-stream error only. For a custom rustls ServerConfig use incoming_with_server(tcp_incoming, ServerConfig) or Incoming::new_with_server.

LOW-LEVEL TOKIO ACCEPTOR. config.state() -> AcmeState; build the application ServerConfig with ServerConfig::builder_with_provider(state.crypto_provider()).with_protocol_versions(DEFAULT_VERSIONS).unwrap().with_no_client_auth().with_cert_resolver(state.resolver()); get AcmeAcceptor via state.acceptor(); poll state.next() in a spawned loop (examples/low_level.rs:39-60). AcmeAcceptor::new is pub(crate) (src/acceptor.rs:36), so state.acceptor() is the only public way to obtain one. AcmeAcceptor is Clone and accept<IO: AsyncRead + AsyncWrite + Unpin>(&self, io: IO) -> AcmeAccept<IO>, a Future with Output = io::Result<Option<tokio_rustls::StartHandshake<IO>>>: Ok(None) means the tls-alpn-01 validation was already answered internally; Ok(Some(sh)) must be finished with sh.into_stream(rustls_config).await (examples/low_level.rs:70-80). AcmeState is an infinite Stream<Item = Event<EC, EA>>; renewal is scheduled at not_after - (not_after - not_before)/3 (src/state.rs:242-247).

AXUM. Enable feature "axum". let acceptor = state.axum_acceptor(Arc::new(rustls_config)); axum_server::bind(addr).acceptor(acceptor).serve(app.into_make_service()).await (examples/low_level_axum.rs:44-67). AxumAcceptor implements axum_server::accept::Accept<I, S> with Stream = tokio_rustls::server::TlsStream<I>, Future = AxumAccept<I, S>, Output = io::Result<(TlsStream<I>, S)>; a validation connection deliberately returns Err(io::Error::other("TLS-ALPN-01 validation request")) (src/axum.rs:76-78), so expect one logged accept error per challenge. The AcmeState still has to be polled separately (the example spawns a loop over state.next()). AxumAcceptor::new is public but requires an AcmeAcceptor, which itself comes from AcmeState::acceptor(). high_level_warp.rs shows the alternative warp path: warp::serve(route).run_incoming(tls_incoming).await.

TLS-ALPN-01. acme::ACME_TLS_ALPN_NAME = b"acme-tls/1" (src/acme.rs:28). AcmeAcceptor::new pushes it into the validation ServerConfig's alpn_protocols (src/acceptor.rs:40-41). AcmeAccept::poll classifies a ClientHello as validation iff its ALPN list equals exactly [b"acme-tls/1"] (src/acceptor.rs:89-95); ResolvesServerCertAcme::resolve uses the same test plus SNI to serve the per-domain auth keys (src/resolver.rs:39-59). acme::Account::tls_alpn_01(&self, &CryptoProvider, &[Challenge], String) -> Result<(&Challenge, CertifiedKey), AcmeError> builds that self-signed acmeIdentifier certificate (src/acme.rs:185-215); AcmeState::authorize registers it and calls the challenge URL (src/state.rs:333-353). ChallengeType::TlsAlpn01 is the serde "tls-alpn-01" variant; AcmeError::NoTlsAlpn01Challenge is returned if the ACME server offers no such challenge. src/lib.rs:120-124 states pub mod acme is incomplete and NOT covered by any stability promises.

LICENSE / DOCS. Dual licensed "Apache-2.0 OR MIT" (Cargo.toml:10 and crates.io API); LICENSE-APACHE and LICENSE-MIT are in the tag tree (their text was not read for this contract). GitHub repo API reports license.spdx_id = Apache-2.0 for the primary detected file. docs.rs failed to build 0.10.0, so no rustdoc API index is available; use the pinned source above.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `tokio_rustls_acme::tokio_rustls (re-export)` | `pub use tokio_rustls;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/lib.rs:158` | True |
| 2 | `crate-root modules` | `pub mod acme; pub mod caches; #[cfg(feature = "axum")] pub mod axum; // acceptor, cache, config, crypto, https_helper, incoming, jose, resolver, state are private modules` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/lib.rs:144-156` | True |
| 3 | `crate-root re-exports` | `pub use acceptor::*; pub use cache::*; pub use config::*; pub use incoming::*; pub use resolver::*; pub use state::*;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/lib.rs:160-165` | True |
| 4 | `AcmeConfig (struct)` | `pub struct AcmeConfig<EC: Debug, EA: Debug = EC> { /* all fields pub(crate): */ client_config: Arc<ClientConfig>, directory_url: String, domains: Vec<String>, contact: Vec<String>, cache: Box<dyn Cache<EC = EC, EA = EA>>, eab: Option<ExternalAccountKey> }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:20-27` | True |
| 5 | `AcmeConfig generic impl bounds` | `impl<EC: 'static + Debug, EA: 'static + Debug> AcmeConfig<EC, EA> { ... }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:126` | True |
| 6 | `AcmeConfig::new` | `#[cfg(feature = "tls-webpki-roots")] pub fn new(domains: impl IntoIterator<Item = impl AsRef<str>>) -> Self // impl AcmeConfig<Infallible, Infallible>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:29-59` | True |
| 7 | `AcmeConfig::new_with_crypto_provider` | `#[cfg(feature = "tls-webpki-roots")] pub fn new_with_crypto_provider(domains: impl IntoIterator<Item = impl AsRef<str>>, crypto_provider: Arc<CryptoProvider>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:74-86` | True |
| 8 | `AcmeConfig::new_with_client_tls_config` | `pub fn new_with_client_tls_config(domains: impl IntoIterator<Item = impl AsRef<str>>, client_config: Arc<ClientConfig>) -> Self // impl AcmeConfig<Infallible, Infallible>; no feature gate` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:111-123` | True |
| 9 | `AcmeConfig::client_tls_config` | `pub fn client_tls_config(mut self, client_config: Arc<ClientConfig>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:132-135` | True |
| 10 | `AcmeConfig::directory` | `pub fn directory(mut self, directory_url: impl AsRef<str>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:136-139` | True |
| 11 | `AcmeConfig::directory_lets_encrypt` | `pub fn directory_lets_encrypt(mut self, production: bool) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:140-147` | True |
| 12 | `AcmeConfig::domains` | `pub fn domains(mut self, contact: impl IntoIterator<Item = impl AsRef<str>>) -> Self // NOTE: source names this parameter &#96;contact&#96;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:148-151` | True |
| 13 | `AcmeConfig::domains_push` | `pub fn domains_push(mut self, contact: impl AsRef<str>) -> Self // NOTE: source names this parameter &#96;contact&#96;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:152-155` | True |
| 14 | `AcmeConfig::external_account_binding` | `pub fn external_account_binding(mut self, kid: impl AsRef<str>, key: impl AsRef<[u8]>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:157-160` | True |
| 15 | `AcmeConfig::contact` | `pub fn contact(mut self, contact: impl IntoIterator<Item = impl AsRef<str>>) -> Self // email addresses must include a &#96;mailto:&#96; prefix` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:162-168` | True |
| 16 | `AcmeConfig::contact_push` | `pub fn contact_push(mut self, contact: impl AsRef<str>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:170-176` | True |
| 17 | `AcmeConfig::cache` | `pub fn cache<C: 'static + Cache>(self, cache: C) -> AcmeConfig<C::EC, C::EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:178-187` | True |
| 18 | `AcmeConfig::cache_compose` | `pub fn cache_compose<CC: 'static + CertCache, CA: 'static + AccountCache>(self, cert_cache: CC, account_cache: CA) -> AcmeConfig<CC::EC, CA::EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:188-194` | True |
| 19 | `AcmeConfig::cache_with_boxed_err` | `pub fn cache_with_boxed_err<C: 'static + Cache>(self, cache: C) -> AcmeConfig<Box<dyn Debug>>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:195-197` | True |
| 20 | `AcmeConfig::cache_option` | `pub fn cache_option<C: 'static + Cache>(self, cache: Option<C>) -> AcmeConfig<C::EC, C::EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:198-203` | True |
| 21 | `AcmeConfig::state` | `pub fn state(self) -> AcmeState<EC, EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:210-213` | True |
| 22 | `AcmeConfig::incoming` | `pub fn incoming<TCP: AsyncRead + AsyncWrite + Unpin, ETCP, ITCP: Stream<Item = Result<TCP, ETCP>> + Unpin>(self, tcp_incoming: ITCP, alpn_protocols: Vec<Vec<u8>>) -> Incoming<TCP, ETCP, ITCP, EC, EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:214-227` | True |
| 23 | `AcmeConfig::incoming_with_server` | `pub fn incoming_with_server<TCP: AsyncRead + AsyncWrite + Unpin, ETCP, ITCP: Stream<Item = Result<TCP, ETCP>> + Unpin>(self, tcp_incoming: ITCP, server_config: ServerConfig) -> Incoming<TCP, ETCP, ITCP, EC, EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/config.rs:228-240` | True |
| 24 | `AcmeAcceptor (struct)` | `#[derive(Clone)] pub struct AcmeAcceptor { config: Arc<ServerConfig> /* private */ }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acceptor.rs:28-33` | True |
| 25 | `AcmeAcceptor::new (NOT public)` | `pub(crate) fn new(resolver: Arc<ResolvesServerCertAcme>, crypto_provider: Arc<CryptoProvider>) -> Self // consumers must obtain an AcmeAcceptor via AcmeState::acceptor()` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acceptor.rs:35-45` | True |
| 26 | `AcmeAcceptor::accept` | `pub fn accept<IO: AsyncRead + AsyncWrite + Unpin>(&self, io: IO) -> AcmeAccept<IO>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acceptor.rs:47-54` | True |
| 27 | `AcmeAccept (struct)` | `pub struct AcmeAccept<IO: AsyncRead + AsyncWrite + Unpin> { /* private: LazyConfigAcceptor<IO>, Arc<ServerConfig>, Option<Accept<IO>> */ }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acceptor.rs:57-62` | True |
| 28 | `AcmeAccept: Future` | `impl<IO: AsyncRead + AsyncWrite + Unpin> Future for AcmeAccept<IO> { type Output = io::Result<Option<StartHandshake<IO>>>; } // Ok(None) means the tls-alpn-01 validation was already handled internally` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acceptor.rs:74-105` | True |
| 29 | `Incoming (struct)` | `pub struct Incoming<TCP: AsyncRead + AsyncWrite + Unpin, ETCP, ITCP: Stream<Item = Result<TCP, ETCP>> + Unpin, EC: Debug + 'static, EA: Debug + 'static> { /* private fields */ }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/incoming.rs:13-30` | True |
| 30 | `Incoming: Unpin` | `impl<TCP: AsyncRead + AsyncWrite + Unpin, ETCP, ITCP: Stream<Item = Result<TCP, ETCP>> + Unpin, EC: Debug + 'static, EA: Debug + 'static> Unpin for Incoming<TCP, ETCP, ITCP, EC, EA> {}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/incoming.rs:32-40` | True |
| 31 | `Incoming::new_with_server` | `pub fn new_with_server(tcp_incoming: ITCP, state: AcmeState<EC, EA>, acceptor: AcmeAcceptor, server_config: ServerConfig) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/incoming.rs:50-64` | True |
| 32 | `Incoming::new` | `pub fn new(tcp_incoming: ITCP, state: AcmeState<EC, EA>, acceptor: AcmeAcceptor, alpn_protocols: Vec<Vec<u8>>) -> Self // sets server_config.alpn_protocols = alpn_protocols` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/incoming.rs:66-75` | True |
| 33 | `Incoming: Stream` | `impl<...> Stream for Incoming<TCP, ETCP, ITCP, EC, EA> { type Item = Result<TlsStream<TCP>, ETCP>; fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> } // TlsStream = tokio_rustls::server::TlsStream` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/incoming.rs:78-137` | True |
| 34 | `Incoming: FusedStream` | `impl<...> FusedStream for Incoming<TCP, ETCP, ITCP, EC, EA> { fn is_terminated(&self) -> bool }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/incoming.rs:140-153` | True |
| 35 | `after` | `pub fn after(d: std::time::Duration) -> Timer // Timer = std::pin::Pin<Box<tokio::time::Sleep>> (private type alias)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:28-33` | True |
| 36 | `AcmeState (struct)` | `pub struct AcmeState<EC: Debug = Infallible, EA: Debug = EC> { /* private fields */ }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:35-51` | True |
| 37 | `Event (type alias)` | `pub type Event<EC, EA> = Result<EventOk, EventError<EC, EA>>;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:53` | True |
| 38 | `EventOk (enum)` | `#[derive(Debug)] pub enum EventOk { DeployedCachedCert, DeployedNewCert, CertCacheStore, AccountCacheStore }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:55-61` | True |
| 39 | `EventError (enum)` | `#[derive(Error, Debug)] pub enum EventError<EC: Debug, EA: Debug> { CertCacheLoad(EC), AccountCacheLoad(EA), CertCacheStore(EC), AccountCacheStore(EA), CachedCertParse(CertParseError), Order(OrderError), NewCertParse(CertParseError) }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:63-79` | True |
| 40 | `OrderError / CertParseError (enums)` | `pub enum OrderError { Acme(#[from] AcmeError), Rcgen(#[from] RcgenError), BadOrder(Order), BadAuth(Auth), TooManyAttemptsAuth(String), ProcessingTimeout(Order) }; pub enum CertParseError { X509(#[from] x509_parser::nom::Err<x509_parser::error::X509Error>), Pem(#[from] pem::PemError), TooFewPem(usize), InvalidPrivateKey }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:81-107` | True |
| 41 | `AcmeState::incoming` | `pub fn incoming<TCP: AsyncRead + AsyncWrite + Unpin, ETCP, ITCP: Stream<Item = Result<TCP, ETCP>> + Unpin>(self, tcp_incoming: ITCP, alpn_protocols: Vec<Vec<u8>>) -> Incoming<TCP, ETCP, ITCP, EC, EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:109-126` | True |
| 42 | `AcmeState::incoming_with_server` | `pub fn incoming_with_server<TCP: AsyncRead + AsyncWrite + Unpin, ETCP, ITCP: Stream<Item = Result<TCP, ETCP>> + Unpin>(self, tcp_incoming: ITCP, server_config: ServerConfig) -> Incoming<TCP, ETCP, ITCP, EC, EA>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:128-144` | True |
| 43 | `AcmeState::acceptor` | `pub fn acceptor(&self) -> AcmeAcceptor` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:146-149` | True |
| 44 | `AcmeState::crypto_provider` | `pub fn crypto_provider(&self) -> Arc<CryptoProvider>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:151-154` | True |
| 45 | `AcmeState::axum_acceptor` | `#[cfg(feature = "axum")] pub fn axum_acceptor(&self, rustls_config: Arc<rustls::ServerConfig>) -> crate::axum::AxumAcceptor` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:156-162` | True |
| 46 | `AcmeState::resolver` | `pub fn resolver(&self) -> Arc<ResolvesServerCertAcme>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:163-166` | True |
| 47 | `AcmeState::new` | `pub fn new(config: AcmeConfig<EC, EA>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:168-199` | True |
| 48 | `AcmeState: Stream` | `impl<EC: 'static + Debug, EA: 'static + Debug> Stream for AcmeState<EC, EA> { type Item = Event<EC, EA>; } // infinite stream: poll to drive issuance and renewal` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/state.rs:463-469` | True |
| 49 | `AxumAcceptor (struct)` | `#[derive(Clone)] pub struct AxumAcceptor { /* private: AcmeAcceptor, Arc<ServerConfig> */ } // reached as tokio_rustls_acme::axum::AxumAcceptor under #[cfg(feature = "axum")]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/axum.rs:11-15` | True |
| 50 | `AxumAcceptor::new` | `pub fn new(acme_acceptor: AcmeAcceptor, config: Arc<ServerConfig>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/axum.rs:17-24` | True |
| 51 | `AxumAcceptor: axum_server::accept::Accept` | `impl<I: AsyncRead + AsyncWrite + Unpin + Send + 'static, S: Send + 'static> axum_server::accept::Accept<I, S> for AxumAcceptor { type Stream = tokio_rustls::server::TlsStream<I>; type Service = S; type Future = AxumAccept<I, S>; fn accept(&self, stream: I, service: S) -> Self::Future }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/axum.rs:26-42` | True |
| 52 | `AxumAccept (struct)` | `pub struct AxumAccept<I: AsyncRead + AsyncWrite + Unpin + Send + 'static, S: Send + 'static> { /* private */ } // also impl Unpin` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/axum.rs:44-54` | True |
| 53 | `AxumAccept: Future` | `impl<I: AsyncRead + AsyncWrite + Unpin + Send + 'static, S: Send + 'static> Future for AxumAccept<I, S> { type Output = io::Result<(tokio_rustls::server::TlsStream<I>, S)>; } // validation connections yield Err(io::Error::other("TLS-ALPN-01 validation request"))` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/axum.rs:56-84` | True |
| 54 | `Cache (trait)` | `pub trait Cache: CertCache + AccountCache {}; impl<T> Cache for T where T: CertCache + AccountCache {}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/cache.rs:5-7` | True |
| 55 | `CertCache (trait)` | `#[async_trait] pub trait CertCache: Send + Sync { type EC: Debug; async fn load_cert(&self, domains: &[String], directory_url: &str) -> Result<Option<Vec<u8>>, Self::EC>; async fn store_cert(&self, domains: &[String], directory_url: &str, cert: &[u8]) -> Result<(), Self::EC>; }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/cache.rs:9-23` | True |
| 56 | `AccountCache (trait)` | `#[async_trait] pub trait AccountCache: Send + Sync { type EA: Debug; async fn load_account(&self, contact: &[String], directory_url: &str) -> Result<Option<Vec<u8>>, Self::EA>; async fn store_account(&self, contact: &[String], directory_url: &str, account: &[u8]) -> Result<(), Self::EA>; }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/cache.rs:25-39` | True |
| 57 | `caches module re-exports` | `pub use boxed::*; pub use composite::*; pub use dir::*; pub use no::*; pub use test::*;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/mod.rs:1-11` | True |
| 58 | `DirCache (struct)` | `pub struct DirCache<P: AsRef<Path> + Send + Sync> { inner: P /* private */ }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/dir.rs:10-17` | True |
| 59 | `DirCache::new` | `pub fn new(dir: P) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/dir.rs:14-17` | True |
| 60 | `DirCache: CertCache/AccountCache` | `impl<P: AsRef<Path> + Send + Sync> CertCache for DirCache<P> { type EC = std::io::Error; ... }; impl<P: AsRef<Path> + Send + Sync> AccountCache for DirCache<P> { type EA = std::io::Error; ... }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/dir.rs:63-106` | True |
| 61 | `DirCache file naming (private helpers)` | `// PRIVATE helpers (not callable): fn cached_account_file_name(contact: &[String], directory_url: impl AsRef<str>) -> String and fn cached_cert_file_name(domains: &[String], directory_url: impl AsRef<str>) -> String; names are "cached_account_" / "cached_cert_" + URL_SAFE_NO_PAD(SHA256(each element + 0x00 terminator, then directory_url bytes))` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/dir.rs:41-60` | True |
| 62 | `NoCache` | `#[derive(Copy, Clone)] pub struct NoCache<EC: Debug = Infallible, EA: Debug = Infallible>; impl<EC: Debug, EA: Debug> NoCache<EC, EA> { pub fn new() -> Self } // Default` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/no.rs:15-34` | True |
| 63 | `CompositeCache` | `pub struct CompositeCache<C: CertCache + Send + Sync, A: AccountCache + Send + Sync> { pub cert_cache: C, pub account_cache: A }; pub fn new(cert_cache: C, account_cache: A) -> Self; pub fn into_inner(self) -> (C, A)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/composite.rs:4-19` | True |
| 64 | `BoxedErrCache` | `pub struct BoxedErrCache<T: Send + Sync> { /* private */ }; pub fn new(inner: T) -> Self; pub fn into_inner(self) -> T // CertCache::EC = AccountCache::EA = Box<dyn Debug>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/boxed.rs:5-27` | True |
| 65 | `TestCache` | `#[derive(Clone)] pub struct TestCache<EC: Debug = std::io::Error, EA: Debug = std::io::Error>; pub fn new() -> Self; pub fn ca_pem(&self) -> &str` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/caches/test.rs:23-67` | True |
| 66 | `ResolvesServerCertAcme` | `#[derive(Debug)] pub struct ResolvesServerCertAcme { /* private Mutex<Inner> */ }; impl ResolvesServerCert for ResolvesServerCertAcme { fn resolve(&self, client_hello: ClientHello) -> Option<Arc<CertifiedKey>> } // new/set_cert/set_auth_key are pub(crate)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/resolver.rs:8-61` | True |
| 67 | `LETS_ENCRYPT_STAGING_DIRECTORY` | `pub const LETS_ENCRYPT_STAGING_DIRECTORY: &str = "https://acme-staging-v02.api.letsencrypt.org/directory";` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:24-25` | True |
| 68 | `LETS_ENCRYPT_PRODUCTION_DIRECTORY` | `pub const LETS_ENCRYPT_PRODUCTION_DIRECTORY: &str = "https://acme-v02.api.letsencrypt.org/directory";` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:26-27` | True |
| 69 | `ACME_TLS_ALPN_NAME` | `pub const ACME_TLS_ALPN_NAME: &[u8] = b"acme-tls/1";` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:28` | True |
| 70 | `Account::tls_alpn_01` | `pub fn tls_alpn_01<'a>(&self, crypto_provider: &CryptoProvider, challenges: &'a [Challenge], domain: String) -> Result<(&'a Challenge, CertifiedKey), AcmeError> // selects ChallengeType::TlsAlpn01, builds a self-signed cert with CustomExtension::new_acme_identifier(SHA-256(keyAuthorization))` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:185-215` | True |
| 71 | `ChallengeType` | `#[derive(Debug, Deserialize, Eq, PartialEq)] pub enum ChallengeType { #[serde(rename = "http-01")] Http01, #[serde(rename = "dns-01")] Dns01, #[serde(rename = "tls-alpn-01")] TlsAlpn01, #[serde(other)] Unknown }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:257-267` | True |
| 72 | `Challenge` | `pub struct Challenge { #[serde(rename = "type")] pub typ: ChallengeType, #[serde(default)] pub url: String, #[serde(default)] pub token: String, pub error: Option<Problem> }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:314-323` | True |
| 73 | `ExternalAccountKey` | `pub struct ExternalAccountKey { pub kid: String, /* pub(crate) key: hmac::Key */ }; impl ExternalAccountKey { pub fn new(kid: String, key: &[u8]) -> Self }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:242-255` | True |
| 74 | `AcmeError` | `#[derive(Error, Debug)] pub enum AcmeError { Io(#[from] std::io::Error), Rcgen(#[from] RcgenError), Jose(#[from] JoseError), Json(#[from] serde_json::Error), HttpRequest(#[from] HttpsRequestError), KeyRejected(CryptoError), Crypto(#[from] CryptoError), MissingHeader(&'static str), NoTlsAlpn01Challenge }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/acme.rs:333-353` | True |
| 75 | `CryptoError` | `#[derive(Debug, thiserror::Error)] #[error("{0}")] pub struct CryptoError(String); // re-exported as tokio_rustls_acme::acme::CryptoError via &#96;pub use crate::crypto::CryptoError;&#96; (src/acme.rs:3)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/crypto.rs:38-40` | True |
| 76 | `example: doc high-level stream` | `let mut tls_incoming = AcmeConfig::new(["example.com"]).contact_push("mailto:admin@example.com").cache(DirCache::new("./rustls_acme_cache")).incoming(tcp_incoming, Vec::new()); while let Some(tls) = tls_incoming.next().await { ... }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/src/lib.rs:53-77` | True |
| 77 | `example: high_level.rs` | `AcmeConfig::new(args.domains).contact(args.email.iter().map(\|e\| format!("mailto:{e}"))).cache_option(args.cache.clone().map(DirCache::new)).directory_lets_encrypt(args.prod).incoming(tcp_incoming, Vec::new())` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/examples/high_level.rs:43-47` | True |
| 78 | `example: low_level.rs (tokio acceptor)` | `let mut state = AcmeConfig::new(args.domains)...state(); let rustls_config = ServerConfig::builder_with_provider(state.crypto_provider()).with_protocol_versions(DEFAULT_VERSIONS).unwrap().with_no_client_auth().with_cert_resolver(state.resolver()); let acceptor = state.acceptor(); tokio::spawn(async move { loop { state.next().await ... } }); match acceptor.accept(tcp).await.unwrap() { None => /* tls-alpn-01 handled */, Some(start_handshake) => start_handshake.into_stream(rustls_config).await.unwrap() }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/examples/low_level.rs:39-82` | True |
| 79 | `example: low_level_axum.rs` | `let acceptor = state.axum_acceptor(Arc::new(rustls_config)); axum_server::bind(addr).acceptor(acceptor).serve(app.into_make_service()).await.unwrap();` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/examples/low_level_axum.rs:39-67` | True |
| 80 | `example: high_level_warp.rs` | `warp::serve(route).run_incoming(tls_incoming).await;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0/examples/high_level_warp.rs:42-49` | True |

**Contract source refs**

- `https://index.crates.io/to/ki/tokio-rustls-acme (14 versions; 0.10.0 newest, cksum 14303ebe76ff697adc251341c800c56e7a6a15c53dcd6e45986dbc291e2ae9a4, yanked=false, features2)`
- `https://crates.io/api/v1/crates/tokio-rustls-acme/0.10.0 (license, rust_version=null, edition=2024, repository, created_at)`
- `https://static.crates.io/crates/tokio-rustls-acme/tokio-rustls-acme-0.10.0.crate (downloaded, sha256 verified; .cargo_vcs_info.json git.sha1=4017266013e0bd00bae83e9edc410f3624f80815)`
- `https://github.com/n0-computer/tokio-rustls-acme (gh api repos/n0-computer/tokio-rustls-acme: default_branch=main, license.spdx_id=Apache-2.0)`
- `https://api.github.com/repos/n0-computer/tokio-rustls-acme/tags?per_page=100 via &#96;gh api repos/n0-computer/tokio-rustls-acme/tags?per_page=100&#96; (v0.10.0 -> 4017266013e0bd00bae83e9edc410f3624f80815)`
- `https://api.github.com/repos/n0-computer/tokio-rustls-acme/commits/main via &#96;gh api repos/n0-computer/tokio-rustls-acme/commits/main&#96; (sha=4017266013e0bd00bae83e9edc410f3624f80815)`
- `&#96;gh api 'repos/n0-computer/tokio-rustls-acme/git/trees/HEAD?recursive=1'&#96; and &#96;gh api 'repos/n0-computer/tokio-rustls-acme/git/trees/4017266013e0bd00bae83e9edc410f3624f80815?recursive=1'&#96; (identical 40-blob trees)`
- `&#96;gh api 'repos/n0-computer/tokio-rustls-acme/contents/<path>?ref=v0.10.0'&#96; base64-decoded for all 23 src/, examples/, Cargo.toml, README.md blobs plus tests/incoming.rs, tests/crypto_provider.rs, tests/pebble.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/lib.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/config.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/acceptor.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/incoming.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/axum.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/state.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/resolver.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/cache.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/caches/dir.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/acme.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/Cargo.toml`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/README.md`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/high_level.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/low_level.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/low_level_axum.rs`
- `https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/high_level_warp.rs`
- `https://docs.rs/tokio-rustls-acme/0.10.0/tokio_rustls_acme/index.html (WebFetch: docs.rs failed-to-build landing page, no rustdoc API index)`
- `https://docs.rs/crate/tokio-rustls-acme/0.10.0/builds (WebFetch: All builds failed; rustc 1.102.0-nightly 65ce8611f 2026-11-18)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_src/tokio-rustls-acme-v0.10.0 (absolute local directory of the base64-decoded tag v0.10.0 source; every file also byte-compared with the published .crate)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_crate/tokio-rustls-acme-0.10.0/tokio-rustls-acme-0.10.0 (absolute local directory of the extracted published crate)`

### 1.3 `iroh-gossip` -- exact version `0.103.0`

- **License:** MIT/Apache-2.0 (dual-licensed MIT OR Apache-2.0, at your option; LICENSE-MIT and LICENSE-APACHE ship in the published crate)
- **MSRV / Go:** Rust 1.91 (rust-version = "1.91" in Cargo.toml at tag v0.103.0; crates.io sparse index rust_version = "1.91"; edition 2021)
- **Signatures:** 48; **unverified items:** 0; **source refs:** 45

**Feature flags / version gates**

- `default = ["net", "metrics"]`
- `net = ["dep:iroh", "dep:tokio", "dep:tokio-util"] -- gates Gossip, Builder, QueueOptions, GossipTopic, GossipSender, GossipReceiver, JoinOptions, Event, Message, DeliveryScope, Error and ALPN`
- `metrics = ["iroh-metrics/metrics"] (enabled by default)`
- `experimental-proto-internals = [] -- makes &#96;pub mod proto&#96; available; explicitly unstable`
- `test-utils = ["experimental-proto-internals", "rand/chacha", "dep:humantime-serde"]`
- `simulator = ["test-utils", "dep:tracing-subscriber", "dep:toml", "dep:clap", "dep:serde_json", "dep:rayon", "dep:comfy-table"]`
- `examples = ["net"] -- required to build examples/chat.rs and examples/setup.rs`
- `docs.rs metadata: features = ["net", "metrics"], rustdoc-args = ["--cfg", "iroh_docsrs"]`
- `TopicId crate-root re-export gate: cfg(any(feature = "net", feature = "experimental-proto-internals"))`
- `0.103.0 has NO rpc, rpc-tls-ring or rpc-tls-aws-lc-rs features; those existed in 0.102.0 (crates.io index comparison). iroh-gossip 0.103.0 has no irpc/noq dependencies.`

**Integration notes (verbatim)**

````text
REPO VERDICT: 0.103.0 source is in n0-computer/iroh-gossip, NOT n0-computer/iroh. The published .crate's .cargo_vcs_info.json names commit e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106; the PGP-signed annotated tag v0.103.0 in n0-computer/iroh-gossip resolves to exactly that commit ('chore: Release iroh-gossip version 0.103.0'), Cargo.toml declares repository = https://github.com/n0-computer/iroh-gossip, and every checked src/*.rs, examples/*.rs and README.md fetched via gh api contents?ref=v0.103.0 is byte-identical to the published tarball (Cargo.toml.orig is byte-identical to the tag's Cargo.toml; the crate's generated Cargo.toml differs by cargo publish normalization). Conversely n0-computer/iroh HEAD has 0 of 294 tree paths matching 'gossip' (truncated=false), no iroh-gossip tag, and its commits endpoint returns 422 'No commit found' for e135cf36. iroh ^1.4 is only an optional dependency of iroh-gossip behind the net feature. Crate checksum 74c4170847b5603f489c4d7771f1e7b263fa63a2521dbbb747601931987aa777 from https://index.crates.io/ir/oh/iroh-gossip matches shasum of the downloaded .crate. Published 2026-11-16T19:54:59Z, not yanked. MSRV Rust 1.91; edition 2021; crate-type = ["lib", "cdylib"]; dual MIT/Apache-2.0.

SUBSCRIBE/BROADCAST: there is NO GossipTopic::subscribe method in 0.103.0. Subscription lives on Gossip: Gossip::subscribe(topic_id, bootstrap) / subscribe_with_opts(topic_id, JoinOptions) return Result<GossipTopic, Error> and register the topic without waiting for a neighbor; subscribe_and_join(topic_id, bootstrap) additionally awaits GossipTopic::joined() and has no timeout. GossipTopic is a n0_future::Stream<Item = Result<Event, Error>> with broadcast(&mut self, Bytes) and broadcast_neighbors(&mut self, Bytes). GossipTopic::split(self) -> (GossipSender, GossipReceiver) moves broadcast to GossipSender::broadcast(&self, Bytes) (no &mut) and events to GossipReceiver. Dropping the last topic handle leaves the topic. Ok from broadcast means local acceptance only, not peer delivery; own broadcast is not echoed as Event::Received; Error::TooLarge is raised above GossipSender::max_content_size() (default Builder::max_message_size = 4096 encoded bytes, ~40 bytes of encoding overhead). Identical payloads are deduplicated by content hash for BroadcastConfig::message_id_retention (default 90 s), so add a nonce for repeated sends.

PASS-THROUGH ALPN: iroh_gossip::ALPN is the byte string b"/iroh-gossip/1" at src/net.rs:36, re-exported at crate root under feature net (src/lib.rs:35-40). Gossip does not accept connections itself. Gossip implements iroh::protocol::ProtocolHandler (accept + shutdown, src/net.rs:105-118), so the ordinary pass-through registration is Router::builder(endpoint.clone()).accept(ALPN, gossip.clone()).spawn() -- verbatim in examples/setup.rs:15-17, examples/chat.rs:136-138 and README.md:38-40. For a hand-written accept loop, check the incoming connection's ALPN yourself and forward it with Gossip::handle_connection(&self, conn: Connection) -> Result<(), Error>. Builder::alpn(impl AsRef<[u8]>) overrides the ALPN used for outbound dials (ActorConfig::alpn defaults to ALPN.to_vec().into() at src/net/actor.rs:53-61; Dialer::queue_dial calls endpoint.connect(endpoint_id, &alpn) at src/net/dialer.rs:28-43). A custom ALPN MUST also be passed to RouterBuilder::accept and be identical on every peer; the custom-ALPN test at src/net/actor/tests/gossip.rs:635-644 demonstrates both sides. Gossip owns any connection handed to it and closes it after 10 s with no streams.

TOPICID: 32-byte newtype pub struct TopicId([u8; 32]); re-exported at crate root under any(net, experimental-proto-internals) via src/lib.rs:28-34. Construct with TopicId::from_bytes([u8; 32]) (const fn), [u8; 32].into(), or "64-hex-chars".parse::<TopicId>() (Err = ParseIdError); as_bytes() -> &[u8; 32], fmt_short() -> String (first 5 bytes as lowercase hex), Display = full lowercase hex, plus Debug, AsRef<[u8]>, AsRef<[u8; 32]>, Serialize/Deserialize, Eq/Ord/Hash/Copy.

TYPE PROVENANCE: Bytes = bytes::Bytes (bytes 1.12); EndpointId/PublicKey come from iroh-base 1 / iroh 1.4; Error is the crate's n0_error::stack_error enum with Closed and TooLarge; Stream is n0_future::Stream (re-export of futures_core::Stream). Event/Message/DeliveryScope/JoinOptions are #[non_exhaustive], so match with a wildcard arm. 0.103.0 removed the 0.102.0 rpc/rpc-tls-* features and irpc/noq dependencies; code written against older gossip RPC APIs will not apply.

VERIFICATION: every signature above was read from the exact published 0.103.0 tarball at the absolute paths in `source`, cross-checked byte-for-byte against gh api contents at tag v0.103.0, and (where a docs.rs URL is attached) cross-checked against https://docs.rs/iroh-gossip/0.103.0/. No requested item is unverified.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `iroh_gossip::ALPN` | `pub const ALPN: &[u8] = b"/iroh-gossip/1";` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:35-36; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/constant.ALPN.html` | True |
| 2 | `iroh_gossip (crate-root re-exports, net)` | `pub use self::{api::{DeliveryScope, Event, GossipReceiver, GossipSender, GossipTopic, JoinOptions, Message}, error::Error, net::{Builder, Gossip, QueueOptions, ALPN}};` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/lib.rs:35-40; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/` | True |
| 3 | `iroh_gossip (crate-root re-exports, any(net, experimental-proto-internals))` | `pub use self::proto::{BroadcastConfig, MembershipConfig, ParseIdError, TopicId, DEFAULT_MAX_MESSAGE_SIZE, MIN_MAX_MESSAGE_SIZE};` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/lib.rs:28-34; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/` | True |
| 4 | `iroh_gossip::Gossip::builder` | `pub fn builder() -> Builder` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:309-314; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 5 | `iroh_gossip::Builder::spawn` | `pub fn spawn(self, endpoint: Endpoint) -> Gossip` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:201-229; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 6 | `iroh_gossip::Builder::alpn` | `pub fn alpn(mut self, alpn: impl AsRef<[u8]>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:162-172; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 7 | `iroh_gossip::Builder::max_message_size` | `pub fn max_message_size(mut self, size: usize) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:127-140; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 8 | `iroh_gossip::Builder::membership_config` | `pub fn membership_config(mut self, config: MembershipConfig) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:142-150; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 9 | `iroh_gossip::Builder::broadcast_config` | `pub fn broadcast_config(mut self, config: BroadcastConfig) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:152-160; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 10 | `iroh_gossip::Builder::send_queue` | `pub fn send_queue(mut self, options: QueueOptions) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:174-178; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 11 | `iroh_gossip::Builder::max_peer_streams` | `pub fn max_peer_streams(mut self, max: u32) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:180-199; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html` | True |
| 12 | `iroh_gossip::QueueOptions::size` | `pub fn size(mut self, size: usize) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:275-287` | True |
| 13 | `iroh_gossip::Gossip::subscribe` | `pub async fn subscribe(&self, topic_id: TopicId, bootstrap: Vec<EndpointId>) -> Result<GossipTopic, Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:378-393; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 14 | `iroh_gossip::Gossip::subscribe_with_opts` | `pub async fn subscribe_with_opts(&self, topic_id: TopicId, opts: JoinOptions) -> Result<GossipTopic, Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:316-360; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 15 | `iroh_gossip::Gossip::subscribe_and_join` | `pub async fn subscribe_and_join(&self, topic_id: TopicId, bootstrap: Vec<EndpointId>) -> Result<GossipTopic, Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:362-376; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 16 | `iroh_gossip::Gossip::handle_connection` | `pub async fn handle_connection(&self, conn: Connection) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:405-423; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 17 | `iroh_gossip::Gossip::shutdown` | `pub async fn shutdown(&self) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:425-438; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 18 | `iroh_gossip::Gossip::max_message_size` | `pub fn max_message_size(&self) -> usize` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:395-403; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 19 | `iroh_gossip::Gossip::metrics` | `pub fn metrics(&self) -> &Arc<Metrics>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:440-443; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html` | True |
| 20 | `impl iroh::protocol::ProtocolHandler for iroh_gossip::Gossip` | `async fn accept(&self, connection: Connection) -> Result<(), AcceptError>; async fn shutdown(&self);` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs:105-118` | True |
| 21 | `iroh_gossip::GossipTopic::broadcast` | `pub async fn broadcast(&mut self, message: Bytes) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:168-175; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 22 | `iroh_gossip::GossipTopic::broadcast_neighbors` | `pub async fn broadcast_neighbors(&mut self, message: Bytes) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:177-184; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 23 | `iroh_gossip::GossipTopic::split` | `pub fn split(self) -> (GossipSender, GossipReceiver)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:163-166; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 24 | `iroh_gossip::GossipTopic::max_content_size` | `pub fn max_content_size(&self) -> usize` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:156-161; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 25 | `iroh_gossip::GossipTopic::neighbors` | `pub fn neighbors(&self) -> impl Iterator<Item = EndpointId> + '_` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:186-189; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 26 | `iroh_gossip::GossipTopic::joined` | `pub async fn joined(&mut self) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:191-196; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 27 | `iroh_gossip::GossipTopic::is_joined` | `pub fn is_joined(&self) -> bool` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:198-201; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 28 | `impl n0_future::Stream for GossipTopic` | `type Item = Result<Event, Error>; fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:204-210; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html` | True |
| 29 | `iroh_gossip::GossipSender::broadcast` | `pub async fn broadcast(&self, message: Bytes) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:71-91; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipSender.html` | True |
| 30 | `iroh_gossip::GossipSender::broadcast_neighbors` | `pub async fn broadcast_neighbors(&self, message: Bytes) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:93-105; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipSender.html` | True |
| 31 | `iroh_gossip::GossipSender::join_peers` | `pub async fn join_peers(&self, peers: Vec<EndpointId>) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:107-119; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipSender.html` | True |
| 32 | `iroh_gossip::GossipSender::max_content_size` | `pub fn max_content_size(&self) -> usize` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:50-58; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipSender.html` | True |
| 33 | `iroh_gossip::GossipReceiver::joined` | `pub async fn joined(&mut self) -> Result<(), Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:238-250` | True |
| 34 | `iroh_gossip::GossipReceiver::is_joined` | `pub fn is_joined(&self) -> bool` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:252-255` | True |
| 35 | `iroh_gossip::GossipReceiver::neighbors` | `pub fn neighbors(&self) -> impl Iterator<Item = EndpointId> + '_` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:229-236` | True |
| 36 | `impl n0_future::Stream for GossipReceiver` | `type Item = Result<Event, Error>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:258-276` | True |
| 37 | `iroh_gossip::TopicId (type)` | `pub struct TopicId([u8; 32]); // #[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Ord, PartialOrd, Deserialize)] + idbytes_impls!(TopicId, "TopicId")` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/state.rs:17-20; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html` | True |
| 38 | `iroh_gossip::TopicId::from_bytes` | `pub const fn from_bytes(bytes: [u8; 32]) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/util.rs:17-21; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html` | True |
| 39 | `iroh_gossip::TopicId::as_bytes` | `pub fn as_bytes(&self) -> &[u8; 32]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/util.rs:23-26; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html` | True |
| 40 | `iroh_gossip::TopicId::fmt_short` | `pub fn fmt_short(&self) -> String` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/state.rs:22-27; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html` | True |
| 41 | `iroh_gossip::TopicId (macro-generated impls)` | `impl<T: Into<[u8; 32]>> From<T> for TopicId; impl FromStr for TopicId { type Err = ParseIdError; fn from_str(s: &str) -> Result<Self, Self::Err> }; impl AsRef<[u8]> for TopicId; impl AsRef<[u8; 32]> for TopicId; impl Display for TopicId; impl Debug for TopicId` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/util.rs:29-67; https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html` | True |
| 42 | `iroh_gossip::JoinOptions` | `pub struct JoinOptions { pub bootstrap: BTreeSet<EndpointId>, pub subscription_capacity: usize } // #[non_exhaustive]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:375-393` | True |
| 43 | `iroh_gossip::JoinOptions::with_bootstrap` | `pub fn with_bootstrap(endpoints: impl IntoIterator<Item = EndpointId>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:402-409` | True |
| 44 | `iroh_gossip::JoinOptions::with_subscription_capacity` | `pub fn with_subscription_capacity(mut self, capacity: usize) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:411-416` | True |
| 45 | `iroh_gossip::Error` | `pub enum Error { Closed, TooLarge { size: usize, max_content: usize } } // #[non_exhaustive], #[n0_error::stack_error(derive, add_meta)]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/error.rs:3-29` | True |
| 46 | `iroh_gossip::Event` | `pub enum Event { NeighborUp(EndpointId), NeighborDown(EndpointId), Received(Message), Lagged } // #[non_exhaustive]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:278-296` | True |
| 47 | `iroh_gossip::Message` | `pub struct Message { pub content: Bytes, pub scope: DeliveryScope, pub delivered_from: EndpointId } // #[non_exhaustive]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:338-355` | True |
| 48 | `iroh_gossip::DeliveryScope` | `pub enum DeliveryScope { Swarm(u16), Neighbors } // #[non_exhaustive]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs:298-313` | True |

**Contract source refs**

- `https://index.crates.io/ir/oh/iroh-gossip (sparse-index row for 0.103.0: cksum 74c4170847b5603f489c4d7771f1e7b263fa63a2521dbbb747601931987aa777, rust_version 1.91, pubtime 2026-11-16T19:54:59Z, yanked false, features/features2)`
- `https://crates.io/api/v1/crates/iroh-gossip/0.103.0 (num 0.103.0, created_at 2026-11-16T19:54:59.627222Z, license MIT/Apache-2.0, rust_version 1.91, yanked false)`
- `https://static.crates.io/crates/iroh-gossip/iroh-gossip-0.103.0.crate (downloaded; shasum -a 256 equals the index cksum)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/.cargo_vcs_info.json (git.sha1 = e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106; path_in_vcs = "")`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/Cargo.toml (generated by cargo publish; repository = https://github.com/n0-computer/iroh-gossip)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/Cargo.toml.orig (byte-identical to GitHub tag Cargo.toml; sha256 0fbcee174ccd2ca3...)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/README.md`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/lib.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/api.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/error.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/state.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/proto/util.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net/actor.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/src/net/dialer.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/examples/chat.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_work/iroh-gossip-0.103.0-extracted/iroh-gossip-0.103.0/examples/setup.rs`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/Cargo.toml?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/README.md?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/lib.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/api.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/net.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/error.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/proto.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/proto/state.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/proto/util.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/examples/chat.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/contents/examples/setup.rs?ref=v0.103.0 (gh api, base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/git/trees/v0.103.0?recursive=1 (77 entries, truncated=false, 31 .rs files under src/)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/git/ref/tags/v0.103.0 (annotated tag dd1c197f85385d6e0f534791d81ca17cf16a4c76)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/git/tags/dd1c197f85385d6e0f534791d81ca17cf16a4c76 (message: chore: Release iroh-gossip version 0.103.0; object = e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106)`
- `https://api.github.com/repos/n0-computer/iroh-gossip/commits/e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106 (2026-11-16T19:54:52Z)`
- `https://github.com/n0-computer/iroh-gossip/tree/v0.103.0 (repository declared in Cargo.toml; tag v0.103.0)`
- `https://api.github.com/repos/n0-computer/iroh/git/trees/HEAD?recursive=1 (294 entries, truncated=false, ZERO paths matching 'gossip')`
- `https://api.github.com/repos/n0-computer/iroh/git/matching-refs/tags/iroh-gossip (no matching tags)`
- `https://api.github.com/repos/n0-computer/iroh/commits/e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106 (HTTP 422: No commit found for SHA)`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/ (crate-root public item list)`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipSender.html`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html`
- `https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/constant.ALPN.html`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-gossip.json (written contract)`

### 1.4 `sendme` -- exact version `0.37.1`

- **License:** Apache-2.0 OR MIT (Cargo.toml license field and README: either license at your option; GitHub repo API reports primary SPDX Apache-2.0). Transitive: iroh 1.5.0, iroh-blobs 0.103.1 and iroh-tickets 1.0.0 all declare license = "MIT OR Apache-2.0" in their published Cargo.toml.orig.
- **MSRV / Go:** Rust 1.91: sendme Cargo.toml rust-version = "1.91" and CI .github/workflows/ci.yml env MSRV: "1.91"; crates.io index entries for sendme 0.37.1, iroh 1.0.0/1.3.0/1.5.0 and iroh-blobs 0.103.1 all declare rust_version 1.91. sendme edition 2021; iroh 1.5.0 edition 2024. Not a Go module (no Go version).
- **Signatures:** 35; **unverified items:** 5; **source refs:** 28

**Feature flags / version gates**

- `sendme 0.37.1: default = ["clipboard"]`
- `sendme 0.37.1: clipboard = ["dep:crossterm", "dep:windows-sys", "dep:libc"]`
- `sendme 0.37.1: optional deps gated by clipboard: crossterm 0.29.0 with features ["event-stream", "osc52"], libc 0.2.174 (unix), windows-sys 0.61 with Win32_System_Console (windows)`
- `iroh = "1.0.0" with default features; iroh 1.5.0 default = ["metrics", "fast-apple-datapath", "portmapper", "tls-ring"]`
- `iroh-blobs = "0.103.1" with default features; iroh-blobs 0.103.1 default = ["hide-proto-docs", "fs-store", "rpc"]; fs-store = ["dep:redb", "dep:reflink-copy", "bao-tree/fs"], rpc = ["dep:noq", "irpc/rpc", "irpc/noq_endpoint_setup"]`
- `clap = { version = "4.4.10", features = ["derive"] }`
- `derive_more = { version = "2.0.1", features = ["display", "from_str"] }`
- `tokio = { version = "1.34.0", features = ["full"] }`
- `tracing-subscriber = { version = "0.3.20", features = ["env-filter"] }`

**Integration notes (verbatim)**

````text
sendme is binary-only (HEAD tree has src/main.rs, no src/lib.rs): its public surface is the clap CLI plus the iroh/iroh-blobs integration. GitHub HEAD = fe29e3454999794bc5ea686cef1ad53d99124a53 (tree cb2f429439ecd0d473d0bfd0b6449ab315c8bbe9, 2026-11-19, 'deps: upgrade iroh and others to get security fixes (#164)'); published crate/tag v0.37.1 = commit 21881bd4b527f55533633623cf84770215ba27d4 (2026-10-21). HEAD Cargo.toml, src/main.rs, README.md are byte-identical to the published 0.37.1 crate; only Cargo.lock differs.

VERSIONS: Cargo.toml pins iroh = "1.0.0" (^1.0.0) and iroh-blobs = { version = "0.103.1" } (^0.103.1), with no features= and no default-features=false. curl of https://index.crates.io/ir/oh/iroh, /ir/oh/iroh-blobs and /se/nd/sendme confirms sendme 0.37.1, iroh 1.0.0, iroh-blobs 0.103.1 are published, unyanked, rust_version 1.91 (cksums 1dee2428dae3fceabe093d64f65038104a576dc77edcffac16fc530c1eeb52d9, 6435544bb3a5c4e6ff7affaa0c0aa0d1bca45bd700226329d5059d3eb54f9dff, a3148ad01585eb6b7cfb641e4100564f4f56db644f4cbed8c10ab2478663161f). HEAD Cargo.lock resolves iroh 1.5.0 + iroh-blobs 0.103.1 + iroh-base 1.5.0 + iroh-tickets 1.0.0 + irpc 0.17.0; published 0.37.1's Cargo.lock resolves iroh 1.3.0 + iroh-blobs 0.103.1. iroh-blobs 0.104.0/0.104.1/0.105.0 exist but ^0.103.1 excludes them; ^1.0.0 admits iroh 1.5.0. iroh-blobs 0.103.1's own manifest declares iroh = { version = "1.0.0", default-features = false }, iroh-tickets = "1.0.0", postcard 1.1.1 (experimental-derive, use-std), data-encoding 2.8.0.

ENDPOINT + ALPN: send side (src/main.rs 652-743) gets SecretKey from ambient IROH_SECRET or SecretKey::generate(), then Endpoint::builder(presets::N0).alpns(vec![iroh_blobs::protocol::ALPN.to_vec()]).secret_key(secret_key).relay_mode(relay_mode.clone()); --ticket-type=Id adds .address_lookup(PkarrPublisher::n0_dns()); --magic-ipv4-addr/--magic-ipv6-addr add .bind_addr(...); builder.bind().await?. presets::N0 installs PkarrPublisher::n0_dns, PkarrResolver::n0_dns, DnsAddressLookup::n0_dns and default relay mode. Data lives in a fresh .sendme-send-<hex16> dir under cwd via FsStore::load. Handler = BlobsProtocol::new(&store, Some(EventSender::new(progress_tx, EventMask { connected: ConnectMode::Notify, get: RequestMode::NotifyLog, ..EventMask::DEFAULT }))) registered with iroh::protocol::Router::builder(endpoint).accept(iroh_blobs::ALPN, blobs.clone()).spawn(); spawn pushes accepted ALPNs onto the endpoint. Before ticketing, send waits <=30 s on router.endpoint().online() unless RelayMode::Disabled; Ctrl-C drops the TempTag, awaits Router::shutdown() with 2 s timeout, removes the temp dir. ALPN is literally pub const ALPN: &[u8] = b"/iroh-bytes/4"; (iroh-blobs src/protocol.rs:406), re-exported as iroh_blobs::ALPN at src/lib.rs:58 -- the identifier says 'bytes' though the crate is 'blobs'. The provider registers exactly that one ALPN; the receiver builds .alpns(vec![]) and calls endpoint.connect(addr, iroh_blobs::protocol::ALPN).await?, adding DnsAddressLookup::n0_dns() only when the ticket's EndpointAddr has neither relay_urls() nor ip_addrs().

DATA PLANE: each file is imported with Store::add_path_with_opts(AddPathOptions { path, mode: ImportMode::TryReference, format: BlobFormat::Raw }); sorted (name, hash) pairs form a Collection; Collection::store(db).await? yields a HashSeq root whose TempTag keeps blobs live. Export uses export_with_opts(ExportOptions { hash, target, mode: ExportMode::Copy }) after Collection::load. Receiver uses get_hash_seq_and_sizes(&connection, &hash, 32*1024*1024, None), db.remote().local(hash_and_format), db.remote().execute_get(connection, local.missing()), and GetProgressItem::{Progress, Done, Error} from get.stream().

TICKET FORMAT: BlobTicket { addr: EndpointAddr, format: BlobFormat, hash: Hash }; sendme builds BlobTicket::new(addr, hash, BlobFormat::HashSeq) with hash = collection TempTag::hash() and addr = router.endpoint().addr() filtered by apply_options (Id clears addr.addrs; Relay keeps only TransportAddr::Relay; Addresses keeps only TransportAddr::Ip; RelayAndAddresses is the CLI default). impl Ticket for BlobTicket sets KIND = "blob". encode_bytes postcard-encodes TicketWireFormat::Variant0(Variant0BlobTicket { node: Variant0NodeAddr { endpoint_id, info: Variant0AddrInfo { relay_url: Option<RelayUrl> (FIRST relay_urls() only), direct_addresses: BTreeSet<SocketAddr> } }, format, hash }); Ticket::encode_string = lowercase "blob" + BASE32_NOPAD(bytes), lowercased; decode_string strips "blob", uppercases, BASE32_NOPAD-decodes, postcard-decodes. Display = Ticket::encode_string, FromStr = Ticket::decode_string, serde human-readable = that string. test_ticket_base32 fixes the empty-address Raw layout: 00 variant0, 32-byte endpoint id, 00 relay None, 00 address count, 00 BlobFormat::Raw, 32-byte blake3 hash. tests/cli.rs round-trips the printed ticket via BlobTicket::from_str/to_string.

CAVEATS (read, not inferred): ambient IROH_SECRET, std::env::current_dir(), .sendme-send-* / .sendme-recv-* dirs in cwd, std::process::exit(1|130) branches, unsafe { raise(SIGINT) } in unix clipboard handling, ProgressStyle unwraps, and payload_bytes = 0 with a todo comment when a download is already complete. Upstream BlobTicket::encode_bytes uses postcard::to_stdvec(&data).expect("postcard serialization failed") (a panic path). iroh-blobs src/protocol.rs:395-396 comments claim MAX_MESSAGE_SIZE is 100MiB while the constant is 1024*1024 (1 MiB). No build, no binary execution, no live network witness; all claims are static source reads.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `sendme::Args` | `#[derive(Parser, Debug)]<br>#[command(<br>    version,<br>    about,<br>    after_help = "Run &#96;sendme <COMMAND> --help&#96; to see all options for a command."<br>)]<br>pub struct Args {<br>    #[clap(subcommand)]<br>    pub command: Commands,<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 62-71)` | True |
| 2 | `sendme::Commands` | `#[derive(Subcommand, Debug)]<br>pub enum Commands {<br>    /// Send a file or directory.<br>    Send(SendArgs),<br><br>    /// Receive a file or directory.<br>    #[clap(visible_alias = "recv")]<br>    Receive(ReceiveArgs),<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 108-116)` | True |
| 3 | `sendme::SendArgs` | `#[derive(Parser, Debug)]<br>pub struct SendArgs {<br>    /// Path to the file or directory to send.<br>    ///<br>    /// The last component of the path will be used as the name of the data<br>    /// being shared.<br>    pub path: PathBuf,<br><br>    /// What type of ticket to use.<br>    ///<br>    /// Use "id" for the shortest type only including the endpoint ID,<br>    /// "addresses" to only add IP addresses without a relay url,<br>    /// "relay" to only add a relay address, and leave the option out<br>    /// to use the biggest type of ticket that includes both relay and<br>    /// address information.<br>    ///<br>    /// Generally, the more information the higher the likelihood of<br>    /// a successful connection, but also the bigger a ticket to connect.<br>    ///<br>    /// This is most useful for debugging which methods of connection<br>    /// establishment work well.<br>    #[clap(long, default_value_t = AddrInfoOptions::RelayAndAddresses)]<br>    pub ticket_type: AddrInfoOptions,<br><br>    #[clap(flatten)]<br>    pub common: CommonArgs,<br><br>    /// Store the receive command in the clipboard.<br>    #[cfg(feature = "clipboard")]<br>    #[clap(short = 'c', long)]<br>    pub clipboard: bool,<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 204-235)` | True |
| 4 | `sendme::ReceiveArgs` | `#[derive(Parser, Debug)]<br>pub struct ReceiveArgs {<br>    /// The ticket to use to connect to the sender.<br>    pub ticket: BlobTicket,<br><br>    #[clap(flatten)]<br>    pub common: CommonArgs,<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 237-244)` | True |
| 5 | `sendme::AddrInfoOptions` | `#[derive(<br>    Copy,<br>    Clone,<br>    PartialEq,<br>    Eq,<br>    Default,<br>    Debug,<br>    derive_more::Display,<br>    derive_more::FromStr,<br>    Serialize,<br>    Deserialize,<br>)]<br>pub enum AddrInfoOptions {<br>    #[default]<br>    Id,<br>    RelayAndAddresses,<br>    Relay,<br>    Addresses,<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 246-271)` | True |
| 6 | `sendme::apply_options` | `fn apply_options(addr: &mut EndpointAddr, opts: AddrInfoOptions) {<br>    match opts {<br>        AddrInfoOptions::Id => {<br>            addr.addrs = Default::default();<br>        }<br>        AddrInfoOptions::RelayAndAddresses => {<br>            // nothing to do<br>        }<br>        AddrInfoOptions::Relay => {<br>            addr.addrs = addr<br>                .addrs<br>                .iter()<br>                .filter(\|addr\| matches!(addr, TransportAddr::Relay(_)))<br>                .cloned()<br>                .collect();<br>        }<br>        AddrInfoOptions::Addresses => {<br>            addr.addrs = addr<br>                .addrs<br>                .iter()<br>                .filter(\|addr\| matches!(addr, TransportAddr::Ip(_)))<br>                .cloned()<br>                .collect();<br>        }<br>    }<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 273-298)` | True |
| 7 | `sendme::send` | `async fn send(args: SendArgs) -> anyhow::Result<()> {` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 652)` | True |
| 8 | `sendme::receive` | `async fn receive(args: ReceiveArgs) -> anyhow::Result<()> {` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 1018)` | True |
| 9 | `sendme::main` | `#[tokio::main]<br>async fn main() -> anyhow::Result<()> {` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 1189-1190)` | True |
| 10 | `sendme::send::endpoint_builder` | `let relay_mode: RelayMode = args.common.relay.into();<br>let mut builder = Endpoint::builder(presets::N0)<br>    .alpns(vec![iroh_blobs::protocol::ALPN.to_vec()])<br>    .secret_key(secret_key)<br>    .relay_mode(relay_mode.clone());<br>if args.ticket_type == AddrInfoOptions::Id {<br>    builder = builder.address_lookup(PkarrPublisher::n0_dns());<br>}<br>if let Some(addr) = args.common.magic_ipv4_addr {<br>    builder = builder.bind_addr(addr)?;<br>}<br>if let Some(addr) = args.common.magic_ipv6_addr {<br>    builder = builder.bind_addr(addr)?;<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 658-672)` | True |
| 11 | `sendme::send::bind_and_protocol` | `let endpoint = builder.bind().await?;<br>let store = FsStore::load(&blobs_data_dir2).await?;<br>let blobs = BlobsProtocol::new(<br>    &store,<br>    Some(EventSender::new(<br>        progress_tx,<br>        EventMask {<br>            connected: ConnectMode::Notify,<br>            get: provider::events::RequestMode::NotifyLog,<br>            ..EventMask::DEFAULT<br>        },<br>    )),<br>);<br>let import_result = import(path2, blobs.store(), &mut mp, args.common.jobs).await?;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 706-726 (contiguous statements; intervening draw-target lines 707-712 omitted))` | True |
| 12 | `sendme::send::router_accept` | `let router = iroh::protocol::Router::builder(endpoint)<br>    .accept(iroh_blobs::ALPN, blobs.clone())<br>    .spawn();` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 729-731)` | True |
| 13 | `sendme::send::make_ticket` | `let hash = temp_tag.hash();<br>let mut addr = router.endpoint().addr();<br>apply_options(&mut addr, args.ticket_type);<br>let ticket = BlobTicket::new(addr, hash, BlobFormat::HashSeq);` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 750-755)` | True |
| 14 | `sendme::receive::endpoint_builder` | `let addr = ticket.addr().clone();<br>let secret_key = get_or_create_secret(args.common.verbose > 0)?;<br>let mut builder = Endpoint::builder(presets::N0)<br>    .alpns(vec![])<br>    .secret_key(secret_key)<br>    .relay_mode(args.common.relay.into());<br><br>if ticket.addr().relay_urls().next().is_none() && ticket.addr().ip_addrs().next().is_none() {<br>    builder = builder.address_lookup(DnsAddressLookup::n0_dns());<br>}<br>let endpoint = builder.bind().await?;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 1020-1036 (bind_addr lines 1030-1035 omitted))` | True |
| 15 | `sendme::receive::connect_and_get` | `let connection = endpoint.connect(addr, iroh_blobs::protocol::ALPN).await?;<br>let (_hash_seq, sizes) =<br>    get_hash_seq_and_sizes(&connection, &hash_and_format.hash, 1024 * 1024 * 32, None)<br>        .await<br>        .map_err(show_get_error)?;<br>let get = db.remote().execute_get(connection, local.missing());` | `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs (gh api repos/n0-computer/sendme/contents/src/main.rs, blob a88269e861f37be3fc049ebebf5e4169324193ce, lines 1058, 1084-1087, 1108)` | True |
| 16 | `iroh::Endpoint::builder` | `pub fn builder(preset: impl Preset) -> Builder` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/endpoint.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 962)` | True |
| 17 | `iroh::endpoint::Builder::alpns` | `pub fn alpns(mut self, alpn_protocols: Vec<Vec<u8>>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/endpoint.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 544)` | True |
| 18 | `iroh::endpoint::Builder::bind` | `pub async fn bind(self) -> Result<Endpoint, BindError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/endpoint.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 228)` | True |
| 19 | `iroh::Endpoint::connect` | `pub async fn connect(<br>    &self,<br>    endpoint_addr: impl Into<EndpointAddr>,<br>    alpn: &[u8],<br>) -> Result<Connection, ConnectError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/endpoint.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 1066-1070)` | True |
| 20 | `iroh::protocol::Router::builder` | `pub fn builder(endpoint: Endpoint) -> RouterBuilder` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/protocol.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 412)` | True |
| 21 | `iroh::protocol::RouterBuilder::accept` | `pub fn accept(<br>    mut self,<br>    alpn: impl AsRef<[u8]>,<br>    handler: impl Into<Box<dyn DynProtocolHandler>>,<br>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/protocol.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 495-499) ; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 22 | `iroh::protocol::RouterBuilder::spawn` | `pub fn spawn(self) -> Router` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/protocol.rs (extracted from https://crates.io/api/v1/crates/iroh/1.5.0/download, lines 512)` | True |
| 23 | `iroh_blobs::protocol::ALPN` | `pub const ALPN: &[u8] = b"/iroh-bytes/4";` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/protocol.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 405-406) ; https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/protocol/constant.ALPN.html` | True |
| 24 | `iroh_blobs::ALPN (crate-root re-export)` | `pub use protocol::ALPN;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/lib.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 58) ; https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/index.html` | True |
| 25 | `iroh_blobs::BlobsProtocol::new` | `pub fn new(store: &Store, events: Option<EventSender>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/net_protocol.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 71-79)` | True |
| 26 | `iroh_blobs::store::fs::FsStore::load` | `pub async fn load(root: impl AsRef<Path>) -> Result<Self>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/store/fs.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 1389-1390)` | True |
| 27 | `iroh_blobs::BlobFormat` | `pub enum BlobFormat {<br>    /// Raw blob<br>    #[default]<br>    Raw,<br>    /// A sequence of BLAKE3 hashes<br>    HashSeq,<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/hash.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 223-229)` | True |
| 28 | `iroh_blobs::ticket::BlobTicket (struct + Display/FromStr)` | `#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]<br>#[display("{}", Ticket::encode_string(self))]<br>pub struct BlobTicket {<br>    /// The provider to get a file from.<br>    addr: EndpointAddr,<br>    /// The format of the blob.<br>    format: BlobFormat,<br>    /// The hash to retrieve.<br>    hash: Hash,<br>}<br><br>impl FromStr for BlobTicket {<br>    type Err = ParseError;<br>    fn from_str(s: &str) -> Result<Self, Self::Err> {<br>        Ticket::decode_string(s)<br>    }<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/ticket.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 14-23, 96-102) ; https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/ticket/struct.BlobTicket.html` | True |
| 29 | `iroh_blobs::ticket::BlobTicket::new and accessors` | `pub fn new(addr: EndpointAddr, hash: Hash, format: BlobFormat) -> Self<br>pub fn hash(&self) -> Hash<br>pub fn addr(&self) -> &EndpointAddr<br>pub fn format(&self) -> BlobFormat<br>pub fn hash_and_format(&self) -> HashAndFormat<br>pub fn recursive(&self) -> bool<br>pub fn into_parts(self) -> (EndpointAddr, Hash, BlobFormat)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/ticket.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 104-142) ; https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/ticket/struct.BlobTicket.html` | True |
| 30 | `iroh_blobs::ticket::TicketWireFormat (wire shape)` | `enum TicketWireFormat {<br>    Variant0(Variant0BlobTicket),<br>}<br><br>struct Variant0BlobTicket {<br>    node: Variant0NodeAddr,<br>    format: BlobFormat,<br>    hash: Hash,<br>}<br><br>struct Variant0NodeAddr {<br>    endpoint_id: EndpointId,<br>    info: Variant0AddrInfo,<br>}<br><br>struct Variant0AddrInfo {<br>    relay_url: Option<RelayUrl>,<br>    direct_addresses: BTreeSet<SocketAddr>,<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/ticket.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 34-62)` | True |
| 31 | `impl Ticket for BlobTicket (KIND + encode/decode_bytes)` | `impl Ticket for BlobTicket {<br>    const KIND: &'static str = "blob";<br><br>    fn encode_bytes(&self) -> Vec<u8> {<br>        let data = TicketWireFormat::Variant0(Variant0BlobTicket {<br>            node: Variant0NodeAddr {<br>                endpoint_id: self.addr.id,<br>                info: Variant0AddrInfo {<br>                    relay_url: self.addr.relay_urls().next().cloned(),<br>                    direct_addresses: self.addr.ip_addrs().cloned().collect(),<br>                },<br>            },<br>            format: self.format,<br>            hash: self.hash,<br>        });<br>        postcard::to_stdvec(&data).expect("postcard serialization failed")<br>    }<br><br>    fn decode_bytes(bytes: &[u8]) -> std::result::Result<Self, ParseError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/ticket.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 64-93)` | True |
| 32 | `iroh_tickets::Ticket::KIND / encode_string / decode_string` | `pub trait Ticket: Sized {<br>    const KIND: &'static str;<br>    fn encode_bytes(&self) -> Vec<u8>;<br>    fn decode_bytes(bytes: &[u8]) -> Result<Self, ParseError>;<br><br>    fn encode_string(&self) -> String {<br>        let mut out = Self::KIND.to_string();<br>        data_encoding::BASE32_NOPAD.encode_append(&self.encode_bytes(), &mut out);<br>        out.make_ascii_lowercase();<br>        out<br>    }<br><br>    fn decode_string(s: &str) -> Result<Self, ParseError> {<br>        let expected = Self::KIND;<br>        let Some(rest) = s.strip_prefix(expected) else {<br>            return Err(e!(ParseError::Kind { expected }));<br>        };<br>        let bytes = data_encoding::BASE32_NOPAD.decode(rest.to_ascii_uppercase().as_bytes())?;<br>        Self::decode_bytes(&bytes)<br>    }<br>}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-tickets-1.0.0/iroh-tickets-1.0.0/src/lib.rs (extracted from https://crates.io/api/v1/crates/iroh-tickets/1.0.0/download, lines 26-65)` | True |
| 33 | `iroh_blobs::ticket::test_ticket_base32 (fixed wire hexdump)` | `let encoded = ticket.to_string();<br>let stripped = encoded.strip_prefix("blob").unwrap();<br>let base32 = data_encoding::BASE32_NOPAD<br>    .decode(stripped.to_ascii_uppercase().as_bytes())<br>    .unwrap();` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/ticket.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 208-235 (hexdump: 00 discriminator, 32-byte endpoint id, 00 relay None, 00 zero direct addresses, 00 BlobFormat::Raw, 32-byte hash))` | True |
| 34 | `iroh_base::EndpointAddr / TransportAddr` | `pub struct EndpointAddr {<br>    pub id: EndpointId,<br>    pub addrs: BTreeSet<TransportAddr>,<br>}<br><br>#[non_exhaustive]<br>pub enum TransportAddr {<br>    Relay(RelayUrl),<br>    Ip(SocketAddr),<br>    Custom(CustomAddr),<br>}<br><br>pub fn ip_addrs(&self) -> impl Iterator<Item = &SocketAddr><br>pub fn relay_urls(&self) -> impl Iterator<Item = &RelayUrl>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-base-1.5.0/iroh-base-1.5.0/src/endpoint_addr.rs (extracted from https://crates.io/api/v1/crates/iroh-base/1.5.0/download, lines 41-62, 137-152)` | True |
| 35 | `iroh_blobs::get::request::get_hash_seq_and_sizes` | `pub async fn get_hash_seq_and_sizes(<br>    connection: &Connection,<br>    hash: &Hash,<br>    max_size: u64,<br>    _progress: Option<mpsc::Sender<u64>>,<br>) -> GetResult<(HashSeq, Arc<[u64]>)>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/get/request.rs (extracted from https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download, lines 222-227)` | True |

**Contract source refs**

- `gh api repos/n0-computer/sendme (default_branch main, license Apache-2.0)`
- `gh api repos/n0-computer/sendme/git/trees/HEAD?recursive=1 (15 blobs; no src/lib.rs)`
- `gh api repos/n0-computer/sendme/git/trees/cb2f429439ecd0d473d0bfd0b6449ab315c8bbe9?recursive=1`
- `gh api repos/n0-computer/sendme/commits/HEAD (fe29e3454999794bc5ea686cef1ad53d99124a53)`
- `gh api repos/n0-computer/sendme/tags and /releases/latest (v0.37.1 -> 21881bd4b527f55533633623cf84770215ba27d4)`
- `gh api repos/n0-computer/sendme/contents/Cargo.toml (blob d1bd4ac76825eb16c032b65df64d8500b190e10b)`
- `gh api repos/n0-computer/sendme/contents/Cargo.lock (blob 2e8e34e0adb45a4da3adfae96896a3710b314942)`
- `gh api repos/n0-computer/sendme/contents/src/main.rs (blob a88269e861f37be3fc049ebebf5e4169324193ce)`
- `gh api repos/n0-computer/sendme/contents/README.md, /tests/cli.rs, /.github/workflows/ci.yml`
- `curl https://index.crates.io/ir/oh/iroh (75 versions, latest 1.5.0)`
- `curl https://index.crates.io/ir/oh/iroh-blobs (47 versions, latest 0.105.0)`
- `curl https://index.crates.io/se/nd/sendme (41 versions, latest 0.37.1)`
- `https://crates.io/api/v1/crates/sendme/0.37.1/download (extracted; VCS sha1 21881bd4b527f55533633623cf84770215ba27d4)`
- `https://crates.io/api/v1/crates/iroh/1.5.0/download (extracted; VCS sha1 ea3bbb6ab41fa23816d7f9e3220ab77d444435b2)`
- `https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download (extracted; VCS sha1 7a4c53a45ecd2bbbb5dd86b3639271004ae89d39)`
- `https://crates.io/api/v1/crates/iroh-tickets/1.0.0/download and /iroh-base/1.5.0/download (extracted)`
- `https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/protocol/constant.ALPN.html`
- `https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/ticket/struct.BlobTicket.html`
- `https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/index.html`
- `https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/src__main.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/Cargo.toml`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/src-sendme/Cargo.lock`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/ticket.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-blobs-0.103.1/iroh-blobs-0.103.1/src/protocol.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-tickets-1.0.0/iroh-tickets-1.0.0/src/lib.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/endpoint.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/crates/iroh-1.5.0/iroh-1.5.0/src/protocol.rs`

### 1.5 `iroh-dag-sync` -- exact version `0.1.0`

- **License:** MIT OR Apache-2.0
- **MSRV / Go:** No rust-version in Cargo.toml and no rust-toolchain file in the repo tree; edition 2021. CI (.github/workflows/ci.yml) declares env MSRV: "1.75" but never references it (job matrix toolchain = stable only). In practice rustc >= 1.75 is required: `pub trait Traversal::next` uses return-position impl Trait in traits (`-> impl Future<Output = ...>`) and the impls use `async fn` in traits (stabilized 1.75); the code also uses let-else and `#[derive(Default)]` with `#[default]`. Not verified by a build. Go version: not applicable (Rust crate).
- **Signatures:** 59; **unverified items:** 9; **source refs:** 25

**Feature flags / version gates**


**Integration notes (verbatim)**

````text
SOURCE / PIN: n0-computer/iroh-experiments@41ebc455678d7789459add9c2218d463bc74866a (branch main, pushed 2026-11-05T10:45:39Z, repo description 'a place for things that aren't in iroh, but are useful nonetheless'). Last commit touching this crate: b66d8b861235d7b5123c62491cb9fafab950c0f4 (2026-06-01, 'feat: update to iroh@1.0.0-rc.1 (#47)'). Every file was fetched with `gh api repos/n0-computer/iroh-experiments/contents/<path>` and base64-decoded; absolute local copies were read with the Read tool from /Users/dylanwong/projects/aws0-deps-2026-11-19/probe/iroh-dag-sync-src/<path>. Each signatures[].source is a repo-relative path with a line number at that pinned commit.

VERSION / AVAILABILITY: Cargo.toml declares name = "iroh-dag-sync", version = "0.1.0", edition = "2021", license = "MIT OR Apache-2.0". There is NO published crate and therefore no exact published version to pin: https://index.crates.io/ir/oh/iroh-dag-sync -> HTTP 404 NoSuchKey; https://crates.io/api/v1/crates/iroh-dag-sync -> HTTP 404 with body {"errors":[{"detail":"crate `iroh-dag-sync` does not exist"}]}; https://docs.rs/iroh-dag-sync/ and https://docs.rs/iroh-dag-sync/latest/iroh_dag_sync/ -> HTTP 404; `gh api repos/n0-computer/iroh-experiments/tags?per_page=100` -> [] (zero tags). exact_version 0.1.0 is the source manifest value only. Cargo.toml does NOT set publish = false, but nothing was released. No Cargo.lock exists for this crate (only content-discovery/Cargo.lock is in the repo tree).

LIBRARY vs EXPERIMENT: ONLY AN EXPERIMENT / EXAMPLE BINARY, NOT A LIBRARY. `git/trees/HEAD?recursive=1` and `contents/iroh-dag-sync/src` list exactly args.rs, main.rs, protocol.rs, sync.rs, tables.rs, traversal.rs, util.rs plus Cargo.toml, README.md, .gitignore. There is NO src/lib.rs (also https://raw.githubusercontent.com/n0-computer/iroh-experiments/main/iroh-dag-sync/src/lib.rs -> 404), no [lib] section, no [[bin]] section, no [features] section, no rust-version. main.rs declares `mod args; mod protocol; mod sync; mod tables; mod traversal; mod util;` as private modules, so all `pub` items in signatures are binary-crate-internal: a downstream crate cannot `use iroh_dag_sync::*`. Root README: 'This is for experiments with iroh by the n0 team. Things in here can be very low level and unpolished... most will not [make it into iroh]', and for this directory: 'An experiment trying to bridge the gap between iroh-blobs and IPFS, allowing you to sync IPFS DAGs with non-BLAKE3 CIDs via iroh-blobs.' Crate README opens 'Example how to use iroh protocols such as gossip and iroh-bytes...'. To reuse, vendor the six modules and add your own src/lib.rs (or depend on a git/path fork that converts it); a package with only a bin target exposes no linkable library. CI (.github/workflows/ci.yml) only runs `cargo check --all-features`, `cargo fmt --all -- --check`, `cargo clippy` for each example dir; it declares env MSRV: "1.75" but never uses that variable (toolchain matrix is stable only).

CRDT / MERGE SEMANTICS: NONE. A case-insensitive grep across all eight crate files for crdt|merge|causal|vector clock|version vector|anti-entropy|conflict|tombstone|lamport|happens-before|gossip finds only `iroh-gossip = "0.100"` in Cargo.toml and the word 'gossip' in the README's first sentence; the gossip dependency is never referenced in any source file. There is no merge function, no state-based or op-based CRDT, no vector/version clock, no operation log, no causal delivery, no conflict resolution, no anti-entropy or rumor layer, no tombstones/GC, and no multi-writer replica API. The 'DAG' is an IPLD link graph of immutable CID-addressed blocks, NOT a causality graph of operations; 'causal DAG sync' would be a misnomer. Convergence is by content addressing: handle_sync_response recomputes the CID's non-BLAKE3 multihash (`hasher.digest(&buffer)` vs `cid.hash()`) and checks the blake3 tag returned by `store.add_bytes(data.clone()).with_tag().await?`, so repeated/parallel syncs are idempotent key-value inserts into two redb tables. `FullTraversalOpts.visited: Option<BTreeSet<Cid>>` is a delta-sync hint (abort at an already-known CID), not a version vector. The only writable state operation is Tables::insert_links.

SYNC ALGORITHM (README 'Sync algorithm' + traversal.rs + sync.rs): Bob (receiver) sends postcard-encoded `Request::Sync(SyncRequest { traversal, inline })`; both peers call `get_traversal(opts, db)` and execute the SAME lazy deterministic traversal concurrently, so the n-th response frame corresponds to the n-th CID of the receiver's own traversal. `FullTraversal<D>` is depth-first pre-order left-to-right: a `Vec<Cid>` stack, `HashSet<Cid>` visited used only for membership (never iterated, so no randomness in order), and deferred link expansion via `prev: Option<Cid>` so a CID is yielded BEFORE its links are read; links come from `ReadableTables::links`, pushed with `links.into_iter().rev()`, and raw codec 0x55 nodes are never expanded. A traversal stops as soon as a block that could contain links is absent locally. `TraversalOrder` has exactly one variant and `get_traversal` ignores the `order` field (destructured with `..`). `TraversalFilter` = All | NoRaw | JustRaw | Excude(BTreeSet<u64>) and `InlineOpts` = All | NoRaw | Excude(BTreeSet<u64>) | None (note the source's misspelling 'Excude'); get_inline compiles them into `InlineCb = Box<dyn Fn(&Cid) -> bool>`. `SequenceTraversalOpts(Vec<Cid>)` gives individual/unrelated CIDs; `Traversal::filter` and `Traversal::boxed` allow composition and type erasure. Sender side (write_sync_response, private): iterates traversal, looks up blake3_hash, writes a 33-byte header then `blobs.export_bao(hash, ChunkRanges::all())` for inlined CIDs.

WIRE PROTOCOL / STORAGE: QUIC bidirectional stream on ALPN b"DAG_SYNC/1" (private const, main.rs:30), one request/response pair. Request read with `recv.read_to_end(MAX_REQUEST_SIZE)` where MAX_REQUEST_SIZE = 1024*1024*16 (private const, sync.rs:15) and decoded with `postcard::from_bytes::<Request>`. CLI parses RON via ron_parser() = `ron::Options::default()` + IMPLICIT_SOME + UNWRAP_NEWTYPES + UNWRAP_VARIANT_NEWTYPES. Response is repeated 33-byte frames from `SyncResponseHeader::as_bytes()` (postcard::to_slice into [u8;33] with unwrap()/assert!()) with variants Hash(Hash)/Data(Hash); for Data the stream carries a LE u64 size plus bao4 chunks (chunk group 2^4 => 16 KiB per README), decoded by the receiver with `EmptyOutboard { tree: BaoTree::new(size, IROH_BLOCK_SIZE), root: blake3_hash.into() }` and `bao_tree::io::fsm::decode_ranges`; each verified block is added to the FsStore and `insert_links` updates the receiver's link table so its own traversal can continue. `SyncResponseHeader::from_stream` reads exactly 33 bytes and maps UnexpectedEof to end-of-stream. Local state: FsStore::load("blobs.db") + redb::Database::create("dag.db") with two tables (tables.rs:8,12): HASH_TO_BLAKE3: (u64, &[u8]) -> Hash keyed by (multihash code, digest); DATA_TO_LINKS: (u64, Hash) -> Vec<u8> keyed by (IPLD codec, blake3) holding DAG-CBOR Vec<Cid>, only for blocks with links. Links are extracted with `serde_ipld_dagcbor::codec::DagCborCodec::links(data)`.

CLI (src/args.rs, clap derive): `import <PATH>` (CAR -> blobs.db + dag.db, prints root CID), `export [CID] [--traversal RON] [--target PATH]`, `endpoint` (serve, prints EndpointId + EndpointTicket), `sync [--from ENDPOINT_ID] [ROOT] [--traversal RON] [--inline RON] [--iroh-ipv4-addr] [--iroh-ipv6-addr]`. README examples say `node` but the actual subcommand is Endpoint -- documentation drift. .gitignore covers **/*.db, **/*.car, **/secret.key.

KNOWN PARTIALITY / HAZARDS (source-verified): (1) handle_sync_response on `SyncResponseHeader::Hash` only prints 'just got hash mapping' and `continue`s with `// todo: get the data via another request`; non-inlined blobs are never fetched, so InlineOpts::None/NoRaw/Excude yield an incomplete local DAG. (2) The README's security note says a cid->blake3 mapping for non-inlined data MUST be validated once the data arrives by another means; no such validation exists in code. (3) as_bytes() uses unwrap()+assert! (panic path), `impl Display for TraversalOpts` uses unwrap(), and main.rs does `postcard::from_bytes(&request).unwrap()` for a debug roundtrip. (4) No authn/authz beyond iroh EndpointId, no bound on response size or traversal depth, 16 MiB request cap only. (5) `TraversalOrder` / FullTraversalOpts.order is dead config. (6) util::get_or_create_secret stores secret.key in cwd. (7) iroh-gossip 0.100 is declared but unused. (8) Only tests are four postcard/RON roundtrip unit tests in protocol.rs (cid_json_roundtrip, cid_postcard_roundtrip, opts_postcard_roundtrip, request_postcard_roundtrip); no algorithm, CRDT, or network tests.

DEPENDENCY PINS (Cargo.toml, exact requirement strings; no [features] section so feature_flags is empty and CI's --all-features is a no-op): iroh-blobs 0.102; iroh-gossip 0.100; iroh 1.0.0-rc.1; iroh-tickets 1.0.0-rc.1; iroh-car 0.5; redb 4.1; clap 4.5.7 features=["derive"]; tracing-subscriber 0.3.18; anyhow 1.0.86; tokio 1.38.0 features=["full"]; postcard 1.0.8; serde 1.0.203 features=["derive"]; futures-lite 2.3.0; iroh-io 0.6; tokio-util 0.7.11 features=["rt"]; bao-tree 0.16.0; genawaiter 0.99.1; bytes 1.6.0; hex 0.4.3; ron 0.11; rand 0.9.2; tracing 0.1.40; serde_bytes 0.11.14; ipld-core 0.4.1; multihash 0.19.1; cid 0.11.1 features=["serde"]; multihash-codetable 0.2; serde_ipld_dagcbor 0.6.1.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `Request` | `pub enum Request { Sync(SyncRequest), }` | `iroh-dag-sync/src/protocol.rs:8` | True |
| 2 | `Cid` | `pub struct Cid(cid::Cid);` | `iroh-dag-sync/src/protocol.rs:17` | True |
| 3 | `impl Serialize for Cid::serialize` | `impl Serialize for Cid { fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> }` | `iroh-dag-sync/src/protocol.rs:19` | True |
| 4 | `impl<'de> Deserialize<'de> for Cid::deserialize` | `impl<'de> Deserialize<'de> for Cid { fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> }` | `iroh-dag-sync/src/protocol.rs:29` | True |
| 5 | `impl From<cid::Cid> for Cid::from` | `impl From<cid::Cid> for Cid { fn from(cid: cid::Cid) -> Self }` | `iroh-dag-sync/src/protocol.rs:43` | True |
| 6 | `impl From<Cid> for cid::Cid::from` | `impl From<Cid> for cid::Cid { fn from(scid: Cid) -> Self }` | `iroh-dag-sync/src/protocol.rs:49` | True |
| 7 | `impl Deref for Cid` | `impl Deref for Cid { type Target = cid::Cid; fn deref(&self) -> &Self::Target }` | `iroh-dag-sync/src/protocol.rs:55` | True |
| 8 | `impl Display for Cid::fmt` | `impl std::fmt::Display for Cid { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result }` | `iroh-dag-sync/src/protocol.rs:63` | True |
| 9 | `impl FromStr for Cid` | `impl std::str::FromStr for Cid { type Err = cid::Error; fn from_str(s: &str) -> Result<Self, Self::Err> }` | `iroh-dag-sync/src/protocol.rs:69` | True |
| 10 | `SyncRequest` | `pub struct SyncRequest { pub traversal: TraversalOpts, pub inline: InlineOpts }` | `iroh-dag-sync/src/protocol.rs:84` | True |
| 11 | `SyncResponseHeader` | `pub enum SyncResponseHeader { Hash(Hash), Data(Hash) }` | `iroh-dag-sync/src/protocol.rs:92` | True |
| 12 | `SyncResponseHeader::as_bytes` | `pub fn as_bytes(&self) -> [u8; 33]` | `iroh-dag-sync/src/protocol.rs:98` | True |
| 13 | `SyncResponseHeader::from_stream` | `pub async fn from_stream(mut stream: impl AsyncRead + Unpin) -> anyhow::Result<Option<Self>>` | `iroh-dag-sync/src/protocol.rs:105` | True |
| 14 | `TraversalOpts` | `pub enum TraversalOpts { Sequence(SequenceTraversalOpts), Full(FullTraversalOpts) }` | `iroh-dag-sync/src/protocol.rs:126` | True |
| 15 | `SequenceTraversalOpts` | `pub struct SequenceTraversalOpts(pub Vec<Cid>);` | `iroh-dag-sync/src/protocol.rs:138` | True |
| 16 | `FullTraversalOpts` | `pub struct FullTraversalOpts { pub root: Cid, pub visited: Option<BTreeSet<Cid>>, #[serde(default)] pub order: Option<TraversalOrder>, #[serde(default)] pub filter: Option<TraversalFilter> }` | `iroh-dag-sync/src/protocol.rs:144` | True |
| 17 | `TraversalOrder` | `#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, Clone)] pub enum TraversalOrder { #[default] DepthFirstPreOrderLeftToRight }` | `iroh-dag-sync/src/protocol.rs:167` | True |
| 18 | `TraversalFilter` | `pub enum TraversalFilter { #[default] All, NoRaw, JustRaw, Excude(BTreeSet<u64>) }` | `iroh-dag-sync/src/protocol.rs:174` | True |
| 19 | `InlineOpts` | `pub enum InlineOpts { #[default] All, NoRaw, Excude(BTreeSet<u64>), None }` | `iroh-dag-sync/src/protocol.rs:187` | True |
| 20 | `InlineOpts::from_args` | `pub fn from_args(arg: &Option<String>) -> anyhow::Result<Self>` | `iroh-dag-sync/src/protocol.rs:200` | True |
| 21 | `ron_parser` | `pub fn ron_parser() -> ::ron::Options` | `iroh-dag-sync/src/protocol.rs:208` | True |
| 22 | `impl Display for TraversalOpts::fmt` | `impl Display for TraversalOpts { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result }` | `iroh-dag-sync/src/protocol.rs:215` | True |
| 23 | `impl FromStr for TraversalOpts` | `impl FromStr for TraversalOpts { type Err = ron::de::SpannedError; fn from_str(s: &str) -> Result<Self, Self::Err> }` | `iroh-dag-sync/src/protocol.rs:222` | True |
| 24 | `TraversalOpts::from_args` | `pub fn from_args(root: &Option<Cid>, traversal: &Option<String>) -> anyhow::Result<Self>` | `iroh-dag-sync/src/protocol.rs:231` | True |
| 25 | `Traversal` | `pub trait Traversal { type Db; fn roots(&self) -> Vec<Cid>; fn next(&mut self) -> impl Future<Output = anyhow::Result<Option<Cid>>>; fn db_mut(&mut self) -> &mut Self::Db; fn filter<F: Fn(&Cid) -> bool>(self, f: F) -> Filtered<Self, F> where Self: Sized; fn boxed<'a>(self) -> BoxedTraversal<'a, Self::Db> where Self: Sized + Unpin + 'a; }` | `iroh-dag-sync/src/traversal.rs:16` | True |
| 26 | `BoxedTraversal` | `pub struct BoxedTraversal<'a, D>(Pin<Box<dyn BoxableTraversal<D> + Unpin + 'a>>);` | `iroh-dag-sync/src/traversal.rs:43` | True |
| 27 | `impl<D> Traversal for BoxedTraversal<'_, D>` | `impl<D> Traversal for BoxedTraversal<'_, D> { type Db = D; fn roots(&self) -> Vec<Cid>; async fn next(&mut self) -> anyhow::Result<Option<Cid>>; fn db_mut(&mut self) -> &mut D }` | `iroh-dag-sync/src/traversal.rs:45` | True |
| 28 | `Filtered` | `pub struct Filtered<T, F> { inner: T, filter: F }` | `iroh-dag-sync/src/traversal.rs:85` | True |
| 29 | `impl<T, F> Traversal for Filtered<T, F>` | `impl<T: Traversal, F: Fn(&Cid) -> bool> Traversal for Filtered<T, F> { type Db = T::Db; async fn next(&mut self) -> anyhow::Result<Option<Cid>>; fn db_mut(&mut self) -> &mut Self::Db; fn roots(&self) -> Vec<Cid> }` | `iroh-dag-sync/src/traversal.rs:90` | True |
| 30 | `SequenceTraversal` | `pub struct SequenceTraversal<T> { cids: std::vec::IntoIter<Cid>, db: T }` | `iroh-dag-sync/src/traversal.rs:111` | True |
| 31 | `SequenceTraversal::new` | `impl<D> SequenceTraversal<D> { pub fn new(db: D, cids: Vec<Cid>) -> Self }` | `iroh-dag-sync/src/traversal.rs:116` | True |
| 32 | `impl<D> Traversal for SequenceTraversal<D>` | `impl<D> Traversal for SequenceTraversal<D> { type Db = D; async fn next(&mut self) -> anyhow::Result<Option<Cid>>; fn db_mut(&mut self) -> &mut D; fn roots(&self) -> Vec<Cid> }` | `iroh-dag-sync/src/traversal.rs:125` | True |
| 33 | `FullTraversal` | `pub struct FullTraversal<D> { prev: Option<Cid>, stack: Vec<Cid>, visited: HashSet<Cid>, db: D }` | `iroh-dag-sync/src/traversal.rs:141` | True |
| 34 | `FullTraversal::new` | `impl<D> FullTraversal<D> { pub fn new(db: D, root: Cid, visited: HashSet<Cid>) -> Self }` | `iroh-dag-sync/src/traversal.rs:148` | True |
| 35 | `impl<D: ReadableTables> Traversal for FullTraversal<D>` | `impl<D: ReadableTables> Traversal for FullTraversal<D> { type Db = D; fn roots(&self) -> Vec<Cid>; async fn next(&mut self) -> anyhow::Result<Option<Cid>>; fn db_mut(&mut self) -> &mut D }` | `iroh-dag-sync/src/traversal.rs:160` | True |
| 36 | `get_traversal` | `pub fn get_traversal<'a, D: ReadableTables + Unpin + 'a>(opts: TraversalOpts, db: D) -> anyhow::Result<BoxedTraversal<'a, D>>` | `iroh-dag-sync/src/traversal.rs:200` | True |
| 37 | `InlineCb` | `pub type InlineCb = Box<dyn Fn(&Cid) -> bool>;` | `iroh-dag-sync/src/traversal.rs:232` | True |
| 38 | `get_inline` | `pub fn get_inline(inline: InlineOpts) -> anyhow::Result<InlineCb>` | `iroh-dag-sync/src/traversal.rs:234` | True |
| 39 | `ReadableTables` | `pub trait ReadableTables { fn hash_to_blake3(&self) -> &impl redb::ReadableTable<(u64, &'static [u8]), Hash>; fn data_to_links(&self) -> &impl redb::ReadableTable<(u64, Hash), Vec<u8>>; #[allow(dead_code)] fn has_links(&self, cid: &cid::Cid) -> anyhow::Result<bool>; fn links(&self, cid: &cid::Cid) -> anyhow::Result<Option<Vec<cid::Cid>>>; fn blake3_hash<const N: usize>(&self, hash: &Multihash<N>) -> anyhow::Result<Option<Hash>>; }` | `iroh-dag-sync/src/tables.rs:14` | True |
| 40 | `impl<T: ReadableTables> ReadableTables for &T` | `impl<T: ReadableTables> ReadableTables for &T { fn hash_to_blake3(&self) -> &impl redb::ReadableTable<(u64, &'static [u8]), Hash>; fn data_to_links(&self) -> &impl redb::ReadableTable<(u64, Hash), Vec<u8>> }` | `iroh-dag-sync/src/tables.rs:51` | True |
| 41 | `impl<T: ReadableTables> ReadableTables for &mut T` | `impl<T: ReadableTables> ReadableTables for &mut T { fn hash_to_blake3(&self) -> &impl redb::ReadableTable<(u64, &'static [u8]), Hash>; fn data_to_links(&self) -> &impl redb::ReadableTable<(u64, Hash), Vec<u8>> }` | `iroh-dag-sync/src/tables.rs:61` | True |
| 42 | `Tables<'tx>` | `pub struct Tables<'tx> { pub hash_to_blake3: redb::Table<'tx, (u64, &'static [u8]), Hash>, pub data_to_links: redb::Table<'tx, (u64, Hash), Vec<u8>> }` | `iroh-dag-sync/src/tables.rs:71` | True |
| 43 | `Tables::new` | `pub fn new(tx: &'tx redb::WriteTransaction) -> std::result::Result<Self, redb::TableError>` | `iroh-dag-sync/src/tables.rs:77` | True |
| 44 | `Tables::insert_links` | `pub fn insert_links(&mut self, cid: &cid::Cid, hash: Hash, data: &[u8]) -> anyhow::Result<()>` | `iroh-dag-sync/src/tables.rs:84` | True |
| 45 | `impl ReadableTables for Tables<'_>` | `impl ReadableTables for Tables<'_> { fn hash_to_blake3(&self) -> &impl redb::ReadableTable<(u64, &'static [u8]), Hash>; fn data_to_links(&self) -> &impl redb::ReadableTable<(u64, Hash), Vec<u8>> }` | `iroh-dag-sync/src/tables.rs:96` | True |
| 46 | `ReadOnlyTables` | `pub struct ReadOnlyTables { pub hash_to_blake3: redb::ReadOnlyTable<(u64, &'static [u8]), Hash>, pub data_to_links: redb::ReadOnlyTable<(u64, Hash), Vec<u8>> }` | `iroh-dag-sync/src/tables.rs:106` | True |
| 47 | `ReadOnlyTables::new` | `pub fn new(tx: &redb::ReadTransaction) -> std::result::Result<Self, redb::TableError>` | `iroh-dag-sync/src/tables.rs:112` | True |
| 48 | `impl ReadableTables for ReadOnlyTables` | `impl ReadableTables for ReadOnlyTables { fn hash_to_blake3(&self) -> &impl redb::ReadableTable<(u64, &'static [u8]), Hash>; fn data_to_links(&self) -> &impl redb::ReadableTable<(u64, Hash), Vec<u8>> }` | `iroh-dag-sync/src/tables.rs:120` | True |
| 49 | `handle_request` | `pub async fn handle_request(mut accepting: Accepting, tables: &impl ReadableTables, blobs: &FsStore) -> anyhow::Result<()>` | `iroh-dag-sync/src/sync.rs:17` | True |
| 50 | `handle_sync_request` | `pub async fn handle_sync_request(send: &mut SendStream, request: SyncRequest, tables: &impl ReadableTables, blobs: &FsStore) -> anyhow::Result<()>` | `iroh-dag-sync/src/sync.rs:42` | True |
| 51 | `handle_sync_response` | `pub async fn handle_sync_response(recv: RecvStream, tables: &mut Tables<'_>, store: &FsStore, traversal: TraversalOpts) -> anyhow::Result<()>` | `iroh-dag-sync/src/sync.rs:89` | True |
| 52 | `Args` | `pub struct Args { #[clap(subcommand)] pub cmd: SubCommand }` | `iroh-dag-sync/src/args.rs:12` | True |
| 53 | `SubCommand` | `pub enum SubCommand { Import(ImportArgs), Export(ExportArgs), Endpoint(EndpointArgs), Sync(SyncArgs) }` | `iroh-dag-sync/src/args.rs:18` | True |
| 54 | `ImportArgs` | `pub struct ImportArgs { pub path: PathBuf }` | `iroh-dag-sync/src/args.rs:26` | True |
| 55 | `ExportArgs` | `pub struct ExportArgs { pub cid: Option<Cid>, #[clap(long)] pub traversal: Option<String>, #[clap(long)] pub target: Option<PathBuf> }` | `iroh-dag-sync/src/args.rs:32` | True |
| 56 | `EndpointArgs` | `pub struct EndpointArgs { #[clap(flatten)] pub net: NetArgs }` | `iroh-dag-sync/src/args.rs:44` | True |
| 57 | `NetArgs` | `pub struct NetArgs { #[clap(long)] pub iroh_ipv4_addr: Option<SocketAddrV4>, #[clap(long)] pub iroh_ipv6_addr: Option<SocketAddrV6> }` | `iroh-dag-sync/src/args.rs:50` | True |
| 58 | `SyncArgs` | `pub struct SyncArgs { #[clap(flatten)] pub net: NetArgs, pub root: Option<Cid>, #[clap(long)] pub traversal: Option<String>, #[clap(long)] pub inline: Option<String>, #[clap(long)] pub from: EndpointId }` | `iroh-dag-sync/src/args.rs:60` | True |
| 59 | `get_or_create_secret` | `pub fn get_or_create_secret() -> anyhow::Result<SecretKey>` | `iroh-dag-sync/src/util.rs:5` | True |

**Contract source refs**

- `https://api.github.com/repos/n0-computer/iroh-experiments (gh api repos/n0-computer/iroh-experiments; default_branch main, archived false, pushed_at 2026-11-05T10:45:39Z)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/git/trees/HEAD?recursive=1 (complete path list; no src/lib.rs, no Cargo.lock in iroh-dag-sync)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src (lists args.rs, main.rs, protocol.rs, sync.rs, tables.rs, traversal.rs, util.rs)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/Cargo.toml (base64-decoded; package metadata and dependency pins)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/README.md (base64-decoded; sync algorithm, traversals, network protocol, security note, license)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/args.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/main.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/protocol.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/sync.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/tables.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/traversal.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/util.rs (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/.gitignore (base64-decoded)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/README.md (base64-decoded; repo root experiment status, license)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/contents/.github/workflows/ci.yml (base64-decoded; MSRV env, RS_EXAMPLES_LIST, check/fmt/clippy jobs)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/commits/main -> 41ebc455678d7789459add9c2218d463bc74866a`
- `https://api.github.com/repos/n0-computer/iroh-experiments/commits?path=iroh-dag-sync&per_page=3 -> b66d8b861235d7b5123c62491cb9fafab950c0f4 (2026-06-01), 04fe7df262f248b33d77de2fab26fcd03b746e42 (2026-01-30), c461e1e474703ba3e21b8b12bc8f3fc447faf370 (2025-11-06)`
- `https://api.github.com/repos/n0-computer/iroh-experiments/tags?per_page=100 -> [] (no tags/releases)`
- `https://github.com/n0-computer/iroh-experiments/blob/41ebc455678d7789459add9c2218d463bc74866a/iroh-dag-sync/ (pinned canonical URLs for every file above)`
- `https://index.crates.io/ir/oh/iroh-dag-sync -> HTTP 404 NoSuchKey (crates.io sparse index; crate not published, zero published versions)`
- `https://crates.io/api/v1/crates/iroh-dag-sync -> HTTP 404 {"errors":[{"detail":"crate &#96;iroh-dag-sync&#96; does not exist"}]}`
- `https://docs.rs/iroh-dag-sync/ -> HTTP 404; https://docs.rs/iroh-dag-sync/latest/iroh_dag_sync/ -> HTTP 404 (no docs.rs signatures exist)`
- `https://raw.githubusercontent.com/n0-computer/iroh-experiments/main/iroh-dag-sync/src/lib.rs -> HTTP 404 (no library target)`
- `/Users/dylanwong/projects/aws0-deps-2026-11-19/probe/iroh-dag-sync-src/ (absolute local directory holding the base64-decoded files actually read; signatures[].source paths are repo-relative at commit 41ebc455)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-dag-sync.json (this contract)`

### 1.6 `iroh-blobs` -- exact version `0.105.0`

- **License:** MIT OR Apache-2.0
- **MSRV / Go:** Rust 1.91 (Cargo.toml rust-version, index.crates.io rust_version and crates.io API all say 1.91)
- **Signatures:** 90; **unverified items:** 5; **source refs:** 30

**Feature flags / version gates**

- `default = ["hide-proto-docs", "fs-store", "rpc"]`
- `hide-proto-docs = [] (doc(hidden) only; does not remove API)`
- `fs-store = ["dep:redb", "dep:reflink-copy", "dep:libc", "dep:windows-sys", "bao-tree/fs", "tokio/rt-multi-thread"] (gates store::fs / FsStore)`
- `rpc = ["dep:noq", "irpc/rpc", "irpc/noq_endpoint_setup"] (gates api::Store::connect and api::Store::listen)`
- `metrics = [] (no cfg(feature = "metrics") in src/ of 0.105.0; no-op here)`
- `unstable-util = [] (gates doc(hidden) test module and get::request helpers; outside semver guarantees)`

**Integration notes (verbatim)**

````text
PROVENANCE: 0.105.0 published 2026-11-13T09:43:19Z, yanked=false. crates.io sparse index, crates.io API and the downloaded .crate agree; sha256 of the .crate equals the index cksum and eight checked source files are byte-identical to GitHub tag v0.105.0 (7a7c410f31c9da90854099f4a5d5fdf20da840c3). Every signature below is exact source text read via gh api contents (base64-decoded), not from memory. README.md line 3 warns this release is NOT production quality and recommends iroh-blobs 0.35 for production. Cargo.toml: MIT OR Apache-2.0, rust-version 1.91, edition 2021, iroh 1.0.0 (default-features=false), iroh-base 1.0.0, iroh-tickets 1.0.0, irpc 1.0.0, bao-tree 0.17, redb 4.1.0 behind fs-store. A 157-signature extended extraction of the same sources is at /Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-blobs.full.json.

NAMES THAT DO NOT EXIST IN 0.105.0 (do not generate code with them): (1) BlobStore - no type/trait named BlobStore in any of the 58 src/*.rs files of tag v0.105.0 (full recursive tree + grep), docs.rs crate root lists no such item, and there is no `pub trait Store`. (2) add_from_path - absent; path import is Blobs::add_path / Blobs::add_path_with_opts. (3) store.get / store.read / store.download - absent; local reads are get_bytes, reader, export_ranges, export_bao; network download is Store::downloader(&endpoint).download(...) or Store::remote().fetch(...). get::request::get_blob is a low-level connection-based function.

ARCHITECTURE: api::Store is a #[repr(transparent)] struct wrapping irpc::Client<proto::Request>, NOT a trait. MemStore, FsStore, ReadonlyMemStore and BlobsProtocol all implement Deref<Target = api::Store> (plus AsRef and From), and api::Store implements Deref<Target = api::blobs::Blobs>; store.add_slice(b"...").await? works through two deref steps. Pass api::Store as the capability value. BlobsProtocol::new(&store, None) plus Router::builder(endpoint).accept(iroh_blobs::ALPN, blobs).spawn() serves it (src/net_protocol.rs example lines 7-38).

PERSISTENCE ACROSS RESTARTS: there is no store trait to implement for persistence - persistence is a backend choice. Use FsStore::load(root).await? (creates <root>/blobs.db redb plus <root>/data and <root>/temp; src/store/fs.rs:1492-1497, src/store/fs/options.rs:20-26), or FsStore::load_with_opts(db_path, Options). Re-opening the same root recovers blobs AND named tags (src/store/fs.rs lines 20-25: the database actor stores metadata and tags). MemStore and ReadonlyMemStore are volatile. Durability: await Store::shutdown().await (or Store::sync_db().await) before dropping FsStore - src/store/fs.rs lines 58-71 say dropping without shutdown closes the db in the background, can leave it locked for an immediate reload, can lose the last seconds of writes, and defers partial-blob recovery to next startup; BlobsProtocol::shutdown calls Store::shutdown when the iroh Router shuts down. Options.crash_consistency is CrashConsistency::ProcessCrash (default) or PowerLoss (fsyncs data files before dependent metadata); src/store/fs/options.rs:104-141 says a write is durable only after Store::sync_db or a clean shutdown, and only against the chosen crash class. Named tags are the persistent GC roots: awaiting AddProgress calls with_tag() -> RequestResult<TagInfo> (auto-named persistent tag); with_named_tag(name) -> RequestResult<HashAndFormat>; Tags::set(name, haf) also persists. AddProgress::temp_tag()/TempTag protect only while alive and do NOT survive restart. GC is opt-in via mem::Options.gc_config or fs::options::Options.gc and sweeps everything not reachable from temp/named tags and HashSeq children. Hash::EMPTY is always present (client answers its reads locally).

CUSTOM STORE / PERSISTENCE CONTRACT: Store::from_client(client: impl Into<irpc::Client<proto::Request>>) is the extension point. src/api/proto.rs lines 1-9 define the contract: a store is an actor handling proto::Request / the macro-generated proto::Command (#[rpc_requests(message = Command, alias = "Msg", rpc_feature = "rpc")]); unsupported commands may be answered with an error. src/api.rs lines 288-306 say the minimum commands for BlobsProtocol are ExportBao (get / get-many / HashSeq reads), Observe, and ImportBao (push). The only public trait resembling a store is format::collection::SimpleStore { fn load(&self, hash: Hash) -> impl Future<Output = Result<Bytes>> + Send + '_ }, implemented for api::Store; it is a collection loader, not a persistence trait.

ADD / READ / DOWNLOAD / EXPORT / IMPORT SEMANTICS: add_slice, add_bytes, add_bytes_with_opts, add_path, add_path_with_opts, add_stream return AddProgress<'_> (IntoFuture -> RequestResult<TagInfo>); TagInfo has name: Tag, hash: Hash, format: BlobFormat, so the ticket example uses tag.hash / tag.format. get_bytes materializes the whole blob in memory (explicit OOM warning). reader(hash) returns BlobReader implementing tokio::io::AsyncRead + AsyncSeek. export_ranges returns ExportRangesProgress; export_bao returns ExportBaoProgress with data_to_bytes / data_to_vec / write / stream. export(hash, target) writes to a path on the STORE's filesystem (important for rpc-connected remote stores). add_path defaults to ImportMode::Copy and BlobFormat::Raw; ImportMode::TryReference / ExportMode::TryReference are hints stores may ignore. import_bao, import_bao_with_opts, import_bao_reader, import_bao_bytes ARE public but #[cfg_attr(feature = "hide-proto-docs", doc(hidden))] with default features. Downloader::download returns DownloadProgress (IntoFuture -> Result<()>); providers can be any Vec<EndpointId> via the blanket ContentDiscovery impl. Remote::fetch(sp, content) is the single-connection alternative.

HASH / FORMAT / TICKET: Hash is a 32-byte BLAKE3 hash (new, from_bytes, EMPTY, as_bytes, to_hex, fmt_short; hex or base32 FromStr with HexOrBase32ParseError; Display is 64-char lowercase hex). BlobFormat { Raw, HashSeq } defaults to Raw with u64 discriminants 0/1 and is_raw/is_hash_seq. HashAndFormat { hash, format } has raw(hash), hash_seq(hash) and an 's'-prefixed 65-char hex Display for HashSeq. BlobTicket::new(addr: EndpointAddr, hash: Hash, format: BlobFormat) implements iroh_tickets::Ticket with KIND = "blob" (postcard/base32 wire format Variant0), FromStr<Err = ParseError>, Display, human-readable Serialize/Deserialize and From<BlobTicket> for HashAndFormat; accessors hash(), addr(), format(), hash_and_format(), recursive() (true iff HashSeq), into_parts().

FEATURES: default = hide-proto-docs + fs-store + rpc; disabling fs-store removes store::fs/FsStore, disabling rpc removes Store::connect/Store::listen. hide-proto-docs only hides docs. metrics is an empty feature with no cfg(feature = "metrics") in src/, so it has no observable effect here. unstable-util is outside semver guarantees. IROH_BLOCK_SIZE = BlockSize::from_chunk_log(4) = 16 KiB (src/store/mod.rs:17-18).
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `iroh_blobs::Hash/BlobFormat/HashAndFormat re-export` | `pub use hash::{BlobFormat, Hash, HashAndFormat};` | `/tmp/iroh-blobs-v0.105.0-src/src/lib.rs:37-37 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 2 | `iroh_blobs::BlobsProtocol re-export` | `pub use net_protocol::BlobsProtocol;` | `/tmp/iroh-blobs-v0.105.0-src/src/lib.rs:45-45 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 3 | `iroh_blobs::ALPN` | `pub use protocol::ALPN;` | `/tmp/iroh-blobs-v0.105.0-src/src/lib.rs:61-61 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 4 | `api::Store struct (not a trait)` | `#[derive(Debug, Clone, ref_cast::RefCast)]<br>#[repr(transparent)]<br>pub struct Store {<br>    client: ApiClient,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:203-207 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 5 | `impl Deref for api::Store -> api::blobs::Blobs` | `impl Deref for Store {<br>    type Target = blobs::Blobs;<br><br>    fn deref(&self) -> &Self::Target {<br>        blobs::Blobs::ref_from_sender(&self.client)<br>    }<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:209-215 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 6 | `api::Store::blobs` | `    pub fn blobs(&self) -> &blobs::Blobs {<br>        blobs::Blobs::ref_from_sender(&self.client)<br>    }` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:224-226 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 7 | `api::Store::downloader` | `    pub fn downloader(&self, endpoint: &Endpoint) -> downloader::Downloader {<br>        downloader::Downloader::new(self, endpoint)<br>    }` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:238-240 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 8 | `api::Store::connect (feature=rpc)` | `    #[cfg(feature = "rpc")]<br>    #[cfg_attr(iroh_blobs_docsrs, doc(cfg(feature = "rpc")))]<br>    pub fn connect(endpoint: noq::Endpoint, addr: std::net::SocketAddr) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:243-245 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 9 | `api::Store::listen (feature=rpc)` | `    #[cfg(feature = "rpc")]<br>    #[cfg_attr(iroh_blobs_docsrs, doc(cfg(feature = "rpc")))]<br>    pub async fn listen(self, endpoint: noq::Endpoint) {` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:251-253 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 10 | `api::Store::sync_db` | `    pub async fn sync_db(&self) -> RequestResult<()> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:259-259 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 11 | `api::Store::shutdown` | `    pub async fn shutdown(&self) -> irpc::Result<()> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:265-265 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 12 | `api::Store::from_client (custom store actor)` | `    pub fn from_client(client: impl Into<irpc::Client<proto::Request>>) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:327-327 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 13 | `api::RequestResult type alias` | `pub type RequestResult<T> = std::result::Result<T, RequestError>;` | `/tmp/iroh-blobs-v0.105.0-src/src/api.rs:74-74 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 14 | `api::blobs::Blobs struct` | `#[derive(Debug, Clone, ref_cast::RefCast)]<br>#[repr(transparent)]<br>pub struct Blobs {<br>    client: ApiClient,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:87-91 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 15 | `api::blobs::AddBytesOptions` | `#[derive(Debug)]<br>pub struct AddBytesOptions {<br>    pub data: Bytes,<br>    pub format: BlobFormat,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:70-74 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 16 | `api::blobs::AddPathOptions` | `#[derive(Debug)]<br>pub struct AddPathOptions {<br>    pub path: PathBuf,<br>    pub format: BlobFormat,<br>    pub mode: ImportMode,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:688-693 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 17 | `api::blobs::Blobs::add_slice` | `    pub fn add_slice(&self, data: impl AsRef<[u8]>) -> AddProgress<'_> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:183-183 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 18 | `api::blobs::Blobs::add_bytes` | `    pub fn add_bytes(&self, data: impl Into<bytes::Bytes>) -> AddProgress<'_> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:192-192 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 19 | `api::blobs::Blobs::add_bytes_with_opts` | `    pub fn add_bytes_with_opts(&self, options: impl Into<AddBytesOptions>) -> AddProgress<'_> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:201-201 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 20 | `api::blobs::Blobs::add_path` | `    pub fn add_path(&self, path: impl AsRef<Path>) -> AddProgress<'_> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:271-271 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 21 | `api::blobs::Blobs::add_path_with_opts` | `    pub fn add_path_with_opts(&self, options: impl Into<AddPathOptions>) -> AddProgress<'_> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:236-236 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 22 | `api::blobs::Blobs::add_stream` | `    pub async fn add_stream(<br>        &self,<br>        data: impl Stream<Item = io::Result<Bytes>> + Send + 'static,<br>    ) -> AddProgress<'_> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:279-282 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 23 | `impl IntoFuture for AddProgress -> RequestResult<TagInfo>` | `impl<'a> IntoFuture for AddProgress<'a> {<br>    type Output = RequestResult<TagInfo>;` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:716-717 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 24 | `api::blobs::AddProgress::with_named_tag` | `    pub async fn with_named_tag(self, name: impl AsRef<[u8]>) -> RequestResult<HashAndFormat> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:771-771 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 25 | `api::blobs::AddProgress::with_tag` | `    pub async fn with_tag(self) -> RequestResult<TagInfo> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:788-788 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 26 | `api::blobs::AddProgress::temp_tag` | `    pub async fn temp_tag(self) -> RequestResult<TempTag> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:755-755 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 27 | `api::blobs::Blobs::get_bytes` | `    pub async fn get_bytes(&self, hash: impl Into<Hash>) -> super::ExportBaoResult<Bytes> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:416-416 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 28 | `api::blobs::Blobs::reader` | `    pub fn reader(&self, hash: impl Into<Hash>) -> BlobReader {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:144-144 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 29 | `impl tokio::io::AsyncRead for BlobReader` | `impl tokio::io::AsyncRead for BlobReader {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs/reader.rs:56-56 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 30 | `impl tokio::io::AsyncSeek for BlobReader` | `impl tokio::io::AsyncSeek for BlobReader {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs/reader.rs:154-154 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 31 | `api::blobs::Blobs::export_ranges` | `    pub fn export_ranges(<br>        &self,<br>        hash: impl Into<Hash>,<br>        ranges: impl Into<RangeSet2<u64>>,<br>    ) -> ExportRangesProgress {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:337-341 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 32 | `api::blobs::Blobs::export_bao` | `    pub fn export_bao(<br>        &self,<br>        hash: impl Into<Hash>,<br>        ranges: impl Into<ChunkRanges>,<br>    ) -> ExportBaoProgress {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:378-382 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 33 | `api::blobs::Blobs::export` | `    pub fn export(&self, hash: impl Into<Hash>, target: impl AsRef<Path>) -> ExportProgress {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:442-442 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 34 | `api::blobs::Blobs::import_bao` | `    #[cfg_attr(feature = "hide-proto-docs", doc(hidden))]<br>    pub async fn import_bao(<br>        &self,<br>        hash: impl Into<Hash>,<br>        size: NonZeroU64,<br>        local_update_cap: usize,<br>    ) -> irpc::Result<ImportBaoHandle> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:454-460 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 35 | `api::blobs::Blobs::import_bao_bytes` | `    #[cfg_attr(feature = "hide-proto-docs", doc(hidden))]<br>    pub async fn import_bao_bytes(<br>        &self,<br>        hash: Hash,<br>        ranges: ChunkRanges,<br>        data: impl Into<Bytes>,<br>    ) -> RequestResult<()> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:523-529 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 36 | `api::blobs::Blobs::status` | `    pub async fn status(&self, hash: impl Into<Hash>) -> irpc::Result<BlobStatus> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:540-540 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 37 | `api::blobs::Blobs::has` | `    pub async fn has(&self, hash: impl Into<Hash>) -> irpc::Result<bool> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs:550-550 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 38 | `api::proto::TagInfo` | `#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]<br>pub struct TagInfo {<br>    /// Name of the tag<br>    pub name: Tag,<br>    /// Format of the data<br>    pub format: BlobFormat,<br>    /// Hash of the data<br>    pub hash: Hash,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/proto.rs:448-456 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 39 | `api::proto::ImportMode` | `#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]<br>pub enum ImportMode {<br>    /// This mode will copy the file into the database before hashing.<br>    ///<br>    /// This is the safe default because the file can not be accidentally modified<br>    /// after it has been imported.<br>    #[default]<br>    Copy,<br>    /// This mode will try to reference the file in place and assume it is unchanged after import.<br>    ///<br>    /// This has a large performance and storage benefit, but it is less safe since<br>    /// the file might be modified after it has been imported.<br>    ///<br>    /// Stores are allowed to ignore this mode and always copy the file, e.g.<br>    /// if the file is very small or if the store does not support referencing files.<br>    TryReference,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/proto.rs:729-745 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 40 | `api::proto::ExportMode` | `#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]<br>pub enum ExportMode {<br>    /// This mode will copy the file to the target directory.<br>    ///<br>    /// This is the safe default because the file can not be accidentally modified<br>    /// after it has been exported.<br>    #[default]<br>    Copy,<br>    /// This mode will try to move the file to the target directory and then reference it from<br>    /// the database.<br>    ///<br>    /// This has a large performance and storage benefit, but it is less safe since<br>    /// the file might be modified in the target directory after it has been exported.<br>    ///<br>    /// Stores are allowed to ignore this mode and always copy the file, e.g.<br>    /// if the file is very small or if the store does not support referencing files.<br>    TryReference,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/proto.rs:753-770 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 41 | `api::proto::Request (store actor contract -> Command)` | `#[rpc_requests(message = Command, alias = "Msg", rpc_feature = "rpc")]<br>#[derive(Debug, Serialize, Deserialize)]<br>pub enum Request {<br>    #[rpc(tx = mpsc::Sender<ListBlobsItem>)]<br>    ListBlobs(ListRequest),<br>    #[rpc(tx = oneshot::Sender<Scope>, rx = mpsc::Receiver<BatchResponse>)]<br>    Batch(BatchRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<()>>)]<br>    DeleteBlobs(BlobDeleteRequest),<br>    #[rpc(rx = mpsc::Receiver<BaoContentItem>, tx = oneshot::Sender<super::Result<()>>)]<br>    ImportBao(ImportBaoRequest),<br>    #[rpc(tx = mpsc::Sender<EncodedItem>)]<br>    ExportBao(ExportBaoRequest),<br>    #[rpc(tx = mpsc::Sender<ExportRangesItem>)]<br>    ExportRanges(ExportRangesRequest),<br>    #[rpc(tx = mpsc::Sender<Bitfield>)]<br>    Observe(ObserveRequest),<br>    #[rpc(tx = oneshot::Sender<BlobStatus>)]<br>    BlobStatus(BlobStatusRequest),<br>    #[rpc(tx = mpsc::Sender<AddProgressItem>)]<br>    ImportBytes(ImportBytesRequest),<br>    #[rpc(rx = mpsc::Receiver<ImportByteStreamUpdate>, tx = mpsc::Sender<AddProgressItem>)]<br>    ImportByteStream(ImportByteStreamRequest),<br>    #[rpc(tx = mpsc::Sender<AddProgressItem>)]<br>    ImportPath(ImportPathRequest),<br>    #[rpc(tx = mpsc::Sender<ExportProgressItem>)]<br>    ExportPath(ExportPathRequest),<br>    #[rpc(tx = mpsc::Sender<ListTagsItem>)]<br>    ListTags(ListTagsRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<()>>)]<br>    SetTag(SetTagRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<u64>>)]<br>    DeleteTags(DeleteTagsRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<()>>)]<br>    RenameTag(RenameTagRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<Tag>>)]<br>    CreateTag(CreateTagRequest),<br>    #[rpc(tx = mpsc::Sender<ListTempTagsItem>)]<br>    ListTempTags(ListTempTagsRequest),<br>    #[rpc(tx = oneshot::Sender<TempTag>)]<br>    CreateTempTag(CreateTempTagRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<()>>)]<br>    SyncDb(SyncDbRequest),<br>    #[rpc(tx = oneshot::Sender<()>)]<br>    WaitIdle(WaitIdleRequest),<br>    #[rpc(tx = oneshot::Sender<()>)]<br>    Shutdown(ShutdownRequest),<br>    #[rpc(tx = oneshot::Sender<super::Result<()>>)]<br>    ClearProtected(ClearProtectedRequest),<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/proto.rs:112-161 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 42 | `api::downloader::Downloader::new` | `    pub fn new(store: &Store, endpoint: &Endpoint) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/downloader.rs:429-429 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 43 | `api::downloader::Downloader::download` | `    pub fn download(<br>        &self,<br>        request: impl SupportedRequest,<br>        providers: impl ContentDiscovery,<br>    ) -> DownloadProgress {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/downloader.rs:454-458 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 44 | `api::downloader::Downloader::download_with_opts` | `    pub fn download_with_opts(&self, options: DownloadOptions) -> DownloadProgress {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/downloader.rs:468-468 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 45 | `impl IntoFuture for DownloadProgress -> Result<()>` | `impl IntoFuture for DownloadProgress {<br>    type Output = Result<()>;<br>    type IntoFuture = future::Boxed<Self::Output>;` | `/tmp/iroh-blobs-v0.105.0-src/src/api/downloader.rs:380-382 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 46 | `api::downloader::ContentDiscovery` | `pub trait ContentDiscovery: Debug + Send + Sync + 'static {<br>    fn find_providers(&self, hash: HashAndFormat) -> n0_future::stream::Boxed<EndpointId>;<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/api/downloader.rs:608-610 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 47 | `api::remote::Remote::fetch` | `    pub fn fetch(<br>        &self,<br>        sp: impl GetStreamPair + 'static,<br>        content: impl Into<HashAndFormat>,<br>    ) -> GetProgress {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/remote.rs:492-496 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 48 | `api::Tags::set (persistent named tag)` | `    pub async fn set(<br>        &self,<br>        name: impl AsRef<[u8]>,<br>        value: impl Into<HashAndFormat>,<br>    ) -> super::RequestResult<()> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/tags.rs:145-149 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 49 | `api::Tags::get` | `    pub async fn get(&self, name: impl AsRef<[u8]>) -> super::RequestResult<Option<TagInfo>> {` | `/tmp/iroh-blobs-v0.105.0-src/src/api/tags.rs:132-132 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 50 | `store::IROH_BLOCK_SIZE` | `/// Block size used by iroh, 2^4*1024 = 16KiB<br>pub const IROH_BLOCK_SIZE: BlockSize = BlockSize::from_chunk_log(4);` | `/tmp/iroh-blobs-v0.105.0-src/src/store/mod.rs:17-18 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 51 | `store::mem::MemStore struct` | `#[derive(Debug, Clone)]<br>#[repr(transparent)]<br>pub struct MemStore {<br>    client: ApiClient,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/mem.rs:78-82 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 52 | `store::mem::MemStore::new` | `    pub fn new() -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/mem.rs:118-118 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 53 | `store::mem::MemStore::new_with_opts` | `    pub fn new_with_opts(opts: Options) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/mem.rs:122-122 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 54 | `impl Deref for MemStore -> api::Store` | `impl Deref for MemStore {<br>    type Target = crate::api::Store;<br><br>    fn deref(&self) -> &Self::Target {<br>        crate::api::Store::ref_from_sender(&self.client)<br>    }<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/mem.rs:96-102 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 55 | `store::fs::FsStore::load` | `    pub async fn load(root: impl AsRef<Path>) -> Result<Self> {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs:1492-1492 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 56 | `store::fs::FsStore::load_with_opts` | `    pub async fn load_with_opts(db_path: PathBuf, options: Options) -> Result<FsStore> {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs:1500-1500 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 57 | `store::fs::FsStore struct` | `#[derive(Debug, Clone)]<br>pub struct FsStore {<br>    sender: ApiClient,<br>    db: tokio::sync::mpsc::Sender<InternalCommand>,<br>    #[cfg(test)]<br>    threads: Arc<AtomicU64>,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs:1567-1573 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 58 | `impl Deref for FsStore -> api::Store` | `impl Deref for FsStore {<br>    type Target = Store;<br><br>    fn deref(&self) -> &Self::Target {<br>        Store::ref_from_sender(&self.sender)<br>    }<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs:1581-1587 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 59 | `store::fs::FsStore::dump` | `    pub async fn dump(&self) -> Result<()> {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs:1608-1608 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 60 | `store::fs shutdown/durability contract (module docs)` | `//! Dropping the store shuts down the runtime and closes the database in the<br>//! background, without waiting for either to finish. The database stays locked<br>//! until it is closed, so loading the store again right after dropping it will<br>//! usually fail. Also, some parts of the persistent state won't make it to<br>//! disk, so operations involving large partial blobs will have a large initial<br>//! delay on the next startup.<br>//!<br>//! It is also not guaranteed that all write operations will make it to disk.<br>//! The on-disk store will be in a consistent state, but might miss some writes<br>//! in the last seconds before shutdown.<br>//!<br>//! To avoid this, await [&#96;crate::api::Store::shutdown&#96;] before dropping the<br>//! store. It returns once the database is closed and ephemeral state is saved<br>//! to disk, so the store can be loaded again right away.` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs:58-71 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 61 | `store::fs::options::Options` | `#[derive(Debug, Clone)]<br>pub struct Options {<br>    /// Path options.<br>    pub path: PathOptions,<br>    /// Inline storage options.<br>    pub inline: InlineOptions,<br>    /// Transaction batching options.<br>    pub batch: BatchOptions,<br>    /// Gc configuration.<br>    pub gc: Option<GcConfig>,<br>    /// What kind of crash the store stays consistent through.<br>    pub crash_consistency: CrashConsistency,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs/options.rs:144-156 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 62 | `store::fs::options::Options::new` | `    pub fn new(root: &Path) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs/options.rs:160-160 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 63 | `store::fs::options::CrashConsistency` | `#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]<br>#[non_exhaustive]<br>pub enum CrashConsistency {<br>    /// Consistent after a process crash; recovers from an OS crash or power loss on the next open.<br>    ///<br>    /// No data files are synced, so an OS crash or power loss can leave entries<br>    /// that claim data that never reached the disk. The store records the boot<br>    /// id while it is open, and if it finds one from another boot session on<br>    /// open, every entry with files checks them again when it is next loaded.<br>    /// Until then it may report less data than it has, never more.<br>    ///<br>    /// A clean shutdown does not sync, so an OS crash within seconds after it,<br>    /// before the OS wrote out its cache, goes unnoticed. Where there is no boot<br>    /// id, every unclean shutdown counts as a possible OS crash.<br>    #[default]<br>    ProcessCrash,<br>    /// Consistent after an OS crash or power loss too, at the cost of fsyncs.<br>    ///<br>    /// Data files and the data directory are synced before the metadata that<br>    /// depends on them is recorded, for imports as well as downloads. That<br>    /// costs a few fsyncs for every blob above the inline threshold, which<br>    /// slows down imports and downloads of many small blobs considerably.<br>    /// On Linux, Android, macOS and iOS, opening the store also flushes the data<br>    /// filesystem after recovery, before recording the new run. This includes<br>    /// writes inherited from an earlier run, but may wait for unrelated writes<br>    /// on the same filesystem. If the flush fails, opening the store returns an<br>    /// error and leaves the previous run record in place for the next attempt.<br>    /// Other platforms do not yet perform this startup flush, so switching from<br>    /// &#96;ProcessCrash&#96; can leave earlier writes unsynced.<br>    PowerLoss,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/fs/options.rs:111-141 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 64 | `store::GcConfig` | `#[derive(derive_more::Debug, Clone)]<br>pub struct GcConfig {<br>    /// Interval in which to run garbage collection.<br>    pub interval: Duration,<br>    /// Optional callback to manually add protected blobs.<br>    ///<br>    /// The callback is called before each garbage collection run. It gets a &#96;&mut HashSet<Hash>&#96;<br>    /// and returns a future that returns [&#96;ProtectOutcome&#96;]. All hashes that are added to the<br>    /// [&#96;HashSet&#96;] will be protected from garbage collection during this run.<br>    ///<br>    /// In normal operation, return [&#96;ProtectOutcome::Continue&#96;] from the callback. If you return<br>    /// [&#96;ProtectOutcome::Abort&#96;], the garbage collection run will be aborted.Use this if your<br>    /// source of hashes to protect returned an error, and thus garbage collection should be skipped<br>    /// completely to not unintentionally delete blobs that should be protected.<br>    #[debug("ProtectCallback")]<br>    pub add_protected: Option<ProtectCb>,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/store/gc.rs:156-172 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 65 | `store::readonly_mem::ReadonlyMemStore::new` | `    pub fn new(items: impl IntoIterator<Item = impl AsRef<[u8]>>) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/store/readonly_mem.rs:376-376 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 66 | `format::collection::SimpleStore (only public store-ish trait)` | `pub trait SimpleStore {<br>    /// Load a blob from the store<br>    fn load(&self, hash: Hash) -> impl Future<Output = Result<Bytes>> + Send + '_;<br>}<br><br>impl SimpleStore for crate::api::Store {<br>    async fn load(&self, hash: Hash) -> Result<Bytes> {<br>        Ok(self.get_bytes(hash).await?)<br>    }<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/format/collection.rs:68-77 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 67 | `BlobsProtocol::new` | `    pub fn new(store: &Store, events: Option<EventSender>) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/net_protocol.rs:72-72 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 68 | `Hash struct` | `#[derive(PartialEq, Eq, Copy, Clone, Hash)]<br>pub struct Hash(blake3::Hash);` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:13-14 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 69 | `Hash::EMPTY` | `    pub const EMPTY: Hash = Hash::from_bytes([<br>        175, 19, 73, 185, 245, 249, 161, 166, 160, 64, 77, 234, 54, 220, 201, 73, 155, 203, 37,<br>        201, 173, 193, 18, 183, 204, 154, 147, 202, 228, 31, 50, 98,<br>    ]);` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:27-30 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 70 | `Hash::new` | `    pub fn new(buf: impl AsRef<[u8]>) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:33-33 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 71 | `Hash::as_bytes` | `    pub fn as_bytes(&self) -> &[u8; 32] {` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:39-39 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 72 | `Hash::from_bytes` | `    pub const fn from_bytes(bytes: [u8; 32]) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:44-44 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 73 | `Hash::to_hex` | `    pub fn to_hex(&self) -> String {` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:49-49 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 74 | `impl FromStr for Hash` | `impl FromStr for Hash {<br>    type Err = HexOrBase32ParseError;` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:163-164 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 75 | `BlobFormat enum` | `pub enum BlobFormat {<br>    /// Raw blob<br>    #[default]<br>    Raw,<br>    /// A sequence of BLAKE3 hashes<br>    HashSeq,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:243-249 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 76 | `BlobFormat::is_raw / is_hash_seq` | `impl BlobFormat {<br>    /// Is raw format<br>    pub const fn is_raw(&self) -> bool {<br>        matches!(self, BlobFormat::Raw)<br>    }<br><br>    /// Is hash seq format<br>    pub const fn is_hash_seq(&self) -> bool {<br>        matches!(self, BlobFormat::HashSeq)<br>    }<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:260-270 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 77 | `HashAndFormat struct` | `#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, MaxSize, Hash)]<br>pub struct HashAndFormat {<br>    /// The hash<br>    pub hash: Hash,<br>    /// The format<br>    pub format: BlobFormat,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:273-279 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 78 | `HashAndFormat::raw` | `    pub fn raw(hash: Hash) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:385-385 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 79 | `HashAndFormat::hash_seq` | `    pub fn hash_seq(hash: Hash) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs:393-393 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 80 | `ticket::BlobTicket struct` | `#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]<br>#[display("{}", Ticket::encode_string(self))]<br>pub struct BlobTicket {<br>    /// The provider to get a file from.<br>    addr: EndpointAddr,<br>    /// The format of the blob.<br>    format: BlobFormat,<br>    /// The hash to retrieve.<br>    hash: Hash,<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:14-23 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 81 | `impl iroh_tickets::Ticket for BlobTicket (KIND)` | `impl Ticket for BlobTicket {<br>    const KIND: &'static str = "blob";` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:64-65 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 82 | `impl FromStr for BlobTicket` | `impl FromStr for BlobTicket {<br>    type Err = ParseError;` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:96-97 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 83 | `ticket::BlobTicket::new` | `    pub fn new(addr: EndpointAddr, hash: Hash, format: BlobFormat) -> Self {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:106-106 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 84 | `ticket::BlobTicket::hash` | `    pub fn hash(&self) -> Hash {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:111-111 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 85 | `ticket::BlobTicket::addr` | `    pub fn addr(&self) -> &EndpointAddr {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:116-116 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 86 | `ticket::BlobTicket::format` | `    pub fn format(&self) -> BlobFormat {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:121-121 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 87 | `ticket::BlobTicket::hash_and_format` | `    pub fn hash_and_format(&self) -> HashAndFormat {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:125-125 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 88 | `ticket::BlobTicket::recursive` | `    pub fn recursive(&self) -> bool {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:133-133 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 89 | `ticket::BlobTicket::into_parts` | `    pub fn into_parts(self) -> (EndpointAddr, Hash, BlobFormat) {` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:138-138 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |
| 90 | `impl From<BlobTicket> for HashAndFormat` | `impl From<BlobTicket> for HashAndFormat {<br>    fn from(val: BlobTicket) -> Self {<br>        HashAndFormat {<br>            hash: val.hash,<br>            format: val.format,<br>        }<br>    }<br>}` | `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs:25-32 @ tag v0.105.0 (gh api contents, base64-decoded)` | True |

**Contract source refs**

- `https://index.crates.io/ir/oh/iroh-blobs (0.105.0: cksum b9ef502f80b48a0d87bc6b4212a9140ecc2915e2683cb3f2606b9241fec4349a, rust_version 1.91, yanked false, pubtime 2026-11-13T09:43:19Z, features + features2)`
- `https://crates.io/api/v1/crates/iroh-blobs/0.105.0 (num 0.105.0, rust_version 1.91, license MIT OR Apache-2.0, yanked false, same checksum)`
- `https://static.crates.io/crates/iroh-blobs/iroh-blobs-0.105.0.crate (downloaded; sha256 == index cksum; src/lib.rs, src/api.rs, src/api/blobs.rs, src/store/fs.rs, src/store/mem.rs, src/store/fs/options.rs, src/ticket.rs, src/hash.rs byte-identical to tag v0.105.0)`
- `https://api.github.com/repos/n0-computer/iroh-blobs/tags (v0.105.0 -> 7a7c410f31c9da90854099f4a5d5fdf20da840c3)`
- `https://api.github.com/repos/n0-computer/iroh-blobs/git/trees/HEAD?recursive=1 (108 blobs)`
- `https://api.github.com/repos/n0-computer/iroh-blobs/git/trees/v0.105.0?recursive=1 (tree sha 7a7c410f31c9da90854099f4a5d5fdf20da840c3, truncated false, 107 blobs)`
- `https://api.github.com/repos/n0-computer/iroh-blobs/contents/<path>?ref=v0.105.0 for all 58 src/*.rs files + Cargo.toml + README.md (base64-decoded to /tmp/iroh-blobs-v0.105.0-src/)`
- `/tmp/iroh-blobs-v0.105.0-src/src/lib.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/api.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/api/blobs.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/api/proto.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/api/downloader.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/api/remote.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/store/mod.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/store/mem.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/store/fs.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/store/fs/options.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/ticket.rs`
- `/tmp/iroh-blobs-v0.105.0-src/src/hash.rs`
- `/tmp/iroh-blobs-v0.105.0-src/README.md`
- `/tmp/iroh-blobs-v0.105.0-src/Cargo.toml`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/ (no BlobStore, no public traits)`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/api/struct.Store.html (Deref target Blobs)`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/api/blobs/struct.Blobs.html`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/store/fs/struct.FsStore.html`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/store/mem/struct.MemStore.html`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/ticket/struct.BlobTicket.html`
- `https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/enum.BlobFormat.html`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-blobs.json (this contract)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-blobs.full.json (157-signature extended extraction from the same sources)`

### 1.7 `n0-dns-resolver` -- exact version `0.1.1`

- **License:** MIT OR Apache-2.0 (dual licensed; LICENSE-MIT and LICENSE-APACHE present in the published crate and tag)
- **MSRV / Go:** Rust 1.91 (rust-version = "1.91"); edition 2024. Crates.io index record also has rust_version = "1.91".
- **Signatures:** 140; **unverified items:** 6; **source refs:** 44

**Feature flags / version gates**

- `default = ["transport-tls", "transport-https", "tls-ring"]`
- `transport-tls = ["dep:rustls", "dep:tokio-rustls", "dep:webpki-roots"]`
- `transport-https = ["dep:reqwest", "dep:rustls", "dep:webpki-roots"]`
- `tls-ring = ["rustls?/ring"]`
- `tls-aws-lc-rs = ["rustls?/aws-lc-rs"]`
- `iroh-dns 1.5.0 consumes it with default-features = false, features = ["transport-tls", "transport-https"], and forwards its own tls-ring / tls-aws-lc-rs features to n0-dns-resolver/tls-ring / n0-dns-resolver/tls-aws-lc-rs`

**Integration notes (verbatim)**

````text
VERSION PROVENANCE: https://index.crates.io/n0/-d/n0-dns-resolver lists exactly 0.1.0 and 0.1.1, both unyanked; 0.1.1 has rust_version 1.91, pubtime 2026-11-18T16:33:19Z, and cksum 0381996935dc55768ae8b13e7db69d3a90f13afb51dde80f870484027968bcd2. The downloaded https://static.crates.io/crates/n0-dns-resolver/n0-dns-resolver-0.1.1.crate has that exact SHA-256; its .cargo_vcs_info.json records git sha1 357c7e7b7f2da00d35b47c782721554f418fe880, which is exactly the commit of tag v0.1.1 (gh api repos/n0-computer/n0-dns-resolver/tags). Every src/*.rs file in the published tarball is byte-identical to the tag contents fetched with gh api repos/n0-computer/n0-dns-resolver/contents/<path>?ref=v0.1.1; Cargo.toml.orig is byte-identical to the tag Cargo.toml, while the published Cargo.toml is cargo's normalized rewrite. docs.rs https://docs.rs/n0-dns-resolver/0.1.1/n0_dns_resolver/struct.DnsResolver.html independently displays the same 14 DnsResolver declarations.

RESOLVER TRAIT / API: n0-dns-resolver 0.1.1 itself declares NO public Rust trait (grep 'pub trait ' over /tmp/n0-dns-v011-src/src returns nothing; docs.rs index lists no trait item). Its public surface is the concrete struct DnsResolver plus Builder/Nameserver/DnsProtocol/record/error types listed above. The trait asked about is iroh_dns::dns::Resolver (iroh repo tag v1.5.0, /tmp/iroh-v150-src/iroh-dns/src/dns.rs:104-126): lookup_ipv4, lookup_ipv6, lookup_txt, clear_cache, reset. iroh-dns deliberately keeps n0-dns-resolver an internal detail: private DefaultResolver(Arc<n0_dns_resolver::DnsResolver>) implements that trait (dns.rs:556-598), converting TXT via TxtRecordData::into_boxed_slices and mapping n0_dns_resolver::Error with private map_resolve_error (dns.rs:600-624). iroh_dns::dns::Builder::into_resolver_builder (dns.rs:519-553) maps its config onto n0_dns_resolver::DnsResolver::builder() + .use_system_config() + .fallback_mode() + .default_fallback_nameservers()/.fallback_nameservers() + .tls_client_config(). iroh-dns 1.5.0 declares n0-dns-resolver = { version = "0.1", default-features = false, features = ["transport-tls", "transport-https"] } (iroh-dns/Cargo.toml:30) and iroh tag v1.5.0 Cargo.lock pins version 0.1.1 with the index checksum; iroh/src/lib.rs:290 re-exports `pub use iroh_dns::dns;`.

N0_DNS / CUSTOM DNS SERVER: There is no N0_DNS_* constant inside n0-dns-resolver 0.1.1. The constants live in iroh: iroh_dns::dns::N0_DNS_ENDPOINT_ORIGIN_PROD = "dns.iroh.link." and N0_DNS_ENDPOINT_ORIGIN_STAGING = "staging-dns.iroh.link." (iroh-dns/src/dns.rs:35-38), iroh::address_lookup::pkarr::N0_DNS_PKARR_RELAY_PROD = "https://dns.iroh.link/pkarr" and N0_DNS_PKARR_RELAY_STAGING = "https://staging-dns.iroh.link/pkarr" (pkarr.rs:127,134). iroh::endpoint::ENV_FORCE_STAGING_RELAYS = "IROH_FORCE_STAGING_RELAYS" plus force_staging_infra() (endpoint.rs:2043-2049) selects the staging endpoints; DnsAddressLookup::n0_dns() uses it (address_lookup/dns.rs:85-98). For a custom server, iroh_dns::dns::DnsResolver::with_nameserver(SocketAddr) builds a single UDP nameserver (dns.rs:752-759), DnsResolver::custom(impl Resolver) accepts a fully custom implementation (dns.rs:771-775), and iroh::endpoint::Builder::dns_resolver(DnsResolver) injects it (endpoint.rs:683-696). Concretely, iroh/examples/transfer.rs:303-305 declares --dns-server, lines 527-536 call builder.dns_resolver(DnsResolver::with_nameserver(addr)) (or the dev default 127.0.0.1:5300), and iroh-dns-server/examples/resolve.rs:9-10,51-65 uses DEV_DNS_SERVER = "127.0.0.1:5300" with DEV_DNS_ORIGIN_DOMAIN = "irohdns.example". iroh test utilities run a local DNS + pkarr server and hand back DnsResolver::with_nameserver(nameserver) (iroh/src/test_utils.rs:118-167). At the dependency level the equivalent is n0_dns_resolver::DnsResolver::builder().nameserver(Nameserver::new(addr, DnsProtocol::Udp)).build() (README.md:50-53, src/builder.rs:79-83).

HOW IROH USES IT FOR RELAY AND NODE DISCOVERY: (1) Relay hostname resolution. iroh::defaults::prod declares the four production relay hostnames (defaults.rs:26-43); the endpoint builder's default resolver is DnsResolver::builder().with_system_defaults().tls_client_config(tls_config.clone()).build() (endpoint.rs:263-269). iroh-relay/src/client/tls.rs:250-258 calls DnsResolver::resolve_host_all(url, DNS_TIMEOUT) and happy-eyeballs dials the resulting IPs; iroh/src/util.rs:34-70 implements reqwest::dns::Resolve via lookup_ipv4_ipv6_staggered(name, DNS_TIMEOUT, DNS_STAGGERING_MS) for HTTPS requests (including pkarr relay publishing); net_report uses the same DnsResolver for relay QUIC probes. (2) Node discovery. DnsAddressLookup::n0_dns() selects the prod/staging origin; DnsAddressLookup::resolve (address_lookup/dns.rs:113-135) calls lookup_endpoint_by_id_staggered(&endpoint_id, &origin_domain, DNS_STAGGERING_MS). DnsResolver::lookup_endpoint_by_id (iroh-dns/src/dns.rs:1057-1066) builds the name _iroh.<z32-endpoint-id>.<origin>, calls lookup_txt(name, DNS_TIMEOUT), and parses it with EndpointInfo::from_txt_lookup; each TXT character string carries `<name>=<value>` attributes including relay=<url> (address_lookup/dns.rs:24-42), which yields the peer's home relay URL. Publishing goes to the pkarr relay at N0_DNS_PKARR_RELAY_PROD/STAGING, whose HTTP URL is itself resolved through the same DnsResolver. n0-dns-resolver's own example /tmp/n0-dns-v011-src/examples/compare_iroh_lookups.rs:1-4,40-49,74-94 states this explicitly: iroh resolves relay hostnames (A/AAAA) and `_iroh.<z32>.<origin>` TXT records at dns.iroh.link. (3) Network change: iroh_dns::dns::DnsResolver::reset() atomically swaps in Resolver::reset(), which for DefaultResolver calls n0_dns_resolver::DnsResolver::reset() (carrying the cache across, n0-dns-resolver/src/resolver.rs:1300-1308). Android requires install_android_jni_context, re-exported from both crates. The dependency is a stub resolver: it does not perform recursive resolution and does not validate DNSSEC (README.md:6-9,33).
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `n0_dns_resolver::public_resolvers` | `pub mod public_resolvers;` | `/tmp/n0-dns-v011-src/src/lib.rs:43` | True |
| 2 | `n0_dns_resolver (root re-exports)` | `pub use self::{builder::{Builder, DnsProtocol, FallbackMode, Nameserver, interleave_nameservers}, error::{Error, ResponseCode}, records::{CaaRecordData, HttpsRecordData, MxRecordData, Record, RecordKind, SrvRecordData, SvcbRecordData, TxtRecordData}, resolver::{DnsResolver, TransportError}};` | `/tmp/n0-dns-v011-src/src/lib.rs:53-61` | True |
| 3 | `n0_dns_resolver::install_android_jni_context (cfg-gated re-export)` | `#[cfg(any(target_os = "android", doc))] pub use self::system_config::install_android_jni_context;` | `/tmp/n0-dns-v011-src/src/lib.rs:51-52` | True |
| 4 | `n0_dns_resolver::install_android_jni_context` | `pub unsafe fn install_android_jni_context(java_vm: *mut c_void, application_context: *mut c_void)` | `/tmp/n0-dns-v011-src/src/system_config/android.rs:198-202 and /tmp/n0-dns-v011-src/src/lib.rs:51-52` | True |
| 5 | `n0_dns_resolver::DnsResolver` | `#[derive(Debug)] pub struct DnsResolver { /* private fields: tls_config (cfg with_rustls), https_client (cfg transport_https), conn_pool, cache, builder, state */ }` | `/tmp/n0-dns-v011-src/src/resolver.rs:204-233` | True |
| 6 | `DnsResolver::lookup_record` | `pub fn lookup_record(&self, name: impl Into<String>, kind: RecordKind) -> Pin<Box<dyn Future<Output = Result<Vec<Record>, Error>> + Send + '_>>` | `/tmp/n0-dns-v011-src/src/resolver.rs:317-326` | True |
| 7 | `DnsResolver::system_with_fallback` | `pub fn system_with_fallback() -> Self` | `/tmp/n0-dns-v011-src/src/resolver.rs:338-343` | True |
| 8 | `DnsResolver::builder` | `pub fn builder() -> Builder` | `/tmp/n0-dns-v011-src/src/resolver.rs:349-351` | True |
| 9 | `DnsResolver::lookup_ipv4` | `pub async fn lookup_ipv4(&self, name: impl Into<String>) -> Result<Vec<Ipv4Addr>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1105-1130` | True |
| 10 | `DnsResolver::lookup_ipv6` | `pub async fn lookup_ipv6(&self, name: impl Into<String>) -> Result<Vec<Ipv6Addr>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1133-1158` | True |
| 11 | `DnsResolver::lookup_txt` | `pub async fn lookup_txt(&self, name: impl Into<String>) -> Result<Vec<TxtRecordData>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1161-1173` | True |
| 12 | `DnsResolver::lookup_mx` | `pub async fn lookup_mx(&self, name: impl Into<String>) -> Result<Vec<MxRecordData>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1176-1188` | True |
| 13 | `DnsResolver::lookup_svcb` | `pub async fn lookup_svcb(&self, name: impl Into<String>) -> Result<Vec<SvcbRecordData>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1191-1203` | True |
| 14 | `DnsResolver::lookup_https` | `pub async fn lookup_https(&self, name: impl Into<String>) -> Result<Vec<HttpsRecordData>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1210-1225` | True |
| 15 | `DnsResolver::lookup_ns` | `pub async fn lookup_ns(&self, name: impl Into<String>) -> Result<Vec<String>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1230-1242` | True |
| 16 | `DnsResolver::lookup_srv` | `pub async fn lookup_srv(&self, name: impl Into<String>) -> Result<Vec<SrvRecordData>, Error>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1245-1257` | True |
| 17 | `DnsResolver::clear_cache` | `pub fn clear_cache(&self)` | `/tmp/n0-dns-v011-src/src/resolver.rs:1260-1262` | True |
| 18 | `DnsResolver::configured_nameservers` | `pub fn configured_nameservers(&self) -> Vec<Nameserver>` | `/tmp/n0-dns-v011-src/src/resolver.rs:1271-1273` | True |
| 19 | `DnsResolver::reset` | `pub fn reset(&self) -> Self` | `/tmp/n0-dns-v011-src/src/resolver.rs:1306-1308` | True |
| 20 | `n0_dns_resolver::Builder` | `#[derive(Debug, Clone, Default)] pub struct Builder { /* private fields: use_system_config, nameservers, fallback_nameservers, fallback, serve_stale, cache_min_ttl, negative_max_ttl, tls_client_config (cfg with_rustls) */ }` | `/tmp/n0-dns-v011-src/src/builder.rs:38-60` | True |
| 21 | `Builder::use_system_config` | `#[must_use] pub fn use_system_config(mut self) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:68-72` | True |
| 22 | `Builder::nameserver` | `#[must_use] pub fn nameserver(mut self, nameserver: Nameserver) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:79-83` | True |
| 23 | `Builder::nameservers` | `#[must_use] pub fn nameservers(mut self, nameservers: impl IntoIterator<Item = Nameserver>) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:91-95` | True |
| 24 | `Builder::fallback_mode` | `#[must_use] pub fn fallback_mode(mut self, mode: FallbackMode) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:102-106` | True |
| 25 | `Builder::fallback_nameservers` | `#[must_use] pub fn fallback_nameservers(mut self, nameservers: impl IntoIterator<Item = Nameserver>) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:117-124` | True |
| 26 | `Builder::default_fallback_nameservers` | `#[must_use] pub fn default_fallback_nameservers(self) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:134-139` | True |
| 27 | `Builder::tls_client_config` | `#[cfg(with_rustls)] #[must_use] pub fn tls_client_config(mut self, config: rustls::ClientConfig) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:146-151` | True |
| 28 | `Builder::serve_stale` | `#[must_use] pub fn serve_stale(mut self, max_age: Duration) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:160-164` | True |
| 29 | `Builder::cache_min_ttl` | `#[must_use] pub fn cache_min_ttl(mut self, min_ttl: Duration) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:172-176` | True |
| 30 | `Builder::negative_max_ttl` | `#[must_use] pub fn negative_max_ttl(mut self, max_ttl: Duration) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:183-187` | True |
| 31 | `Builder::build` | `#[must_use] pub fn build(self) -> DnsResolver` | `/tmp/n0-dns-v011-src/src/builder.rs:190-193` | True |
| 32 | `n0_dns_resolver::FallbackMode` | `#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)] #[non_exhaustive] pub enum FallbackMode { Eager, IfSystemEmpty, #[default] Deferred }` | `/tmp/n0-dns-v011-src/src/builder.rs:203-224` | True |
| 33 | `n0_dns_resolver::interleave_nameservers` | `pub fn interleave_nameservers<L>(lists: impl IntoIterator<Item = L>) -> Vec<Nameserver> where L: IntoIterator<Item = Nameserver>` | `/tmp/n0-dns-v011-src/src/builder.rs:246-264` | True |
| 34 | `n0_dns_resolver::Nameserver` | `#[derive(Debug, Clone, Eq, PartialEq, Hash)] pub struct Nameserver { /* private fields: addr: SocketAddr, protocol: DnsProtocol, server_name: Option<String> (cfg with_rustls/doc) */ }` | `/tmp/n0-dns-v011-src/src/builder.rs:272-283` | True |
| 35 | `Nameserver::new` | `pub const fn new(addr: SocketAddr, protocol: DnsProtocol) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:287-294` | True |
| 36 | `Nameserver::addr` | `pub fn addr(&self) -> SocketAddr` | `/tmp/n0-dns-v011-src/src/builder.rs:297-299` | True |
| 37 | `Nameserver::protocol` | `pub fn protocol(&self) -> DnsProtocol` | `/tmp/n0-dns-v011-src/src/builder.rs:302-304` | True |
| 38 | `Nameserver::server_name` | `#[cfg(any(with_rustls, doc))] pub fn server_name(&self) -> Option<&str>` | `/tmp/n0-dns-v011-src/src/builder.rs:310-313` | True |
| 39 | `Nameserver::with_server_name` | `#[cfg(any(with_rustls, doc))] pub fn with_server_name(addr: SocketAddr, protocol: DnsProtocol, server_name: impl Into<String>) -> Self` | `/tmp/n0-dns-v011-src/src/builder.rs:320-331` | True |
| 40 | `n0_dns_resolver::DnsProtocol` | `#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)] #[non_exhaustive] pub enum DnsProtocol { #[default] Udp, Tcp, #[cfg(transport_tls)] Tls, #[cfg(transport_https)] Https }` | `/tmp/n0-dns-v011-src/src/builder.rs:335-361` | True |
| 41 | `DnsProtocol::port` | `pub const fn port(self) -> u16` | `/tmp/n0-dns-v011-src/src/builder.rs:370-381` | True |
| 42 | `n0_dns_resolver::RecordKind` | `#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)] #[non_exhaustive] pub enum RecordKind { A, Aaaa, Txt, Ns, Srv, Mx, Caa, Svcb, Https }` | `/tmp/n0-dns-v011-src/src/records.rs:21-42` | True |
| 43 | `n0_dns_resolver::Record` | `#[derive(Debug, Clone, PartialEq, Eq)] #[non_exhaustive] pub enum Record { A(Ipv4Addr), Aaaa(Ipv6Addr), Txt(TxtRecordData), Ns(String), Srv(SrvRecordData), Mx(MxRecordData), Caa(CaaRecordData), Svcb(SvcbRecordData), Https(HttpsRecordData) }` | `/tmp/n0-dns-v011-src/src/records.rs:53-74` | True |
| 44 | `n0_dns_resolver::SrvRecordData` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct SrvRecordData { pub priority: u16, pub weight: u16, pub port: u16, pub target: String }` | `/tmp/n0-dns-v011-src/src/records.rs:79-89` | True |
| 45 | `n0_dns_resolver::MxRecordData` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct MxRecordData { pub preference: u16, pub exchange: String }` | `/tmp/n0-dns-v011-src/src/records.rs:94-100` | True |
| 46 | `n0_dns_resolver::CaaRecordData` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct CaaRecordData { pub flag: u8, pub tag: String, pub value: Box<[u8]> }` | `/tmp/n0-dns-v011-src/src/records.rs:109-116` | True |
| 47 | `n0_dns_resolver::SvcbRecordData` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct SvcbRecordData(SVCB<'static>);  /* private field */` | `/tmp/n0-dns-v011-src/src/records.rs:126-127` | True |
| 48 | `SvcbRecordData::priority` | `pub fn priority(&self) -> u16` | `/tmp/n0-dns-v011-src/src/records.rs:138-140` | True |
| 49 | `SvcbRecordData::target` | `pub fn target(&self) -> String` | `/tmp/n0-dns-v011-src/src/records.rs:146-148` | True |
| 50 | `SvcbRecordData::alpn` | `pub fn alpn(&self) -> impl Iterator<Item = String> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:155-163` | True |
| 51 | `SvcbRecordData::port` | `pub fn port(&self) -> Option<u16>` | `/tmp/n0-dns-v011-src/src/records.rs:166-171` | True |
| 52 | `SvcbRecordData::ipv4hint` | `pub fn ipv4hint(&self) -> impl Iterator<Item = Ipv4Addr> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:176-184` | True |
| 53 | `SvcbRecordData::ipv6hint` | `pub fn ipv6hint(&self) -> impl Iterator<Item = Ipv6Addr> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:189-197` | True |
| 54 | `SvcbRecordData::ech` | `pub fn ech(&self) -> Option<&[u8]>` | `/tmp/n0-dns-v011-src/src/records.rs:204-209` | True |
| 55 | `SvcbRecordData::mandatory` | `pub fn mandatory(&self) -> impl Iterator<Item = u16> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:216-224` | True |
| 56 | `SvcbRecordData::no_default_alpn` | `pub fn no_default_alpn(&self) -> bool` | `/tmp/n0-dns-v011-src/src/records.rs:230-234` | True |
| 57 | `n0_dns_resolver::HttpsRecordData` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct HttpsRecordData { /* private: owner: String, data: SvcbRecordData */ }` | `/tmp/n0-dns-v011-src/src/records.rs:249-258` | True |
| 58 | `HttpsRecordData::svcb` | `pub fn svcb(&self) -> &SvcbRecordData` | `/tmp/n0-dns-v011-src/src/records.rs:267-269` | True |
| 59 | `HttpsRecordData::priority` | `pub fn priority(&self) -> u16` | `/tmp/n0-dns-v011-src/src/records.rs:275-277` | True |
| 60 | `HttpsRecordData::is_alias` | `pub fn is_alias(&self) -> bool` | `/tmp/n0-dns-v011-src/src/records.rs:284-286` | True |
| 61 | `HttpsRecordData::is_service` | `pub fn is_service(&self) -> bool` | `/tmp/n0-dns-v011-src/src/records.rs:292-294` | True |
| 62 | `HttpsRecordData::target` | `pub fn target(&self) -> String` | `/tmp/n0-dns-v011-src/src/records.rs:300-302` | True |
| 63 | `HttpsRecordData::effective_target` | `pub fn effective_target(&self) -> String` | `/tmp/n0-dns-v011-src/src/records.rs:311-318` | True |
| 64 | `HttpsRecordData::alpn` | `pub fn alpn(&self) -> impl Iterator<Item = String> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:325-327` | True |
| 65 | `HttpsRecordData::effective_alpn` | `pub fn effective_alpn(&self) -> Vec<String>` | `/tmp/n0-dns-v011-src/src/records.rs:340-349` | True |
| 66 | `HttpsRecordData::supports_http2` | `pub fn supports_http2(&self) -> bool` | `/tmp/n0-dns-v011-src/src/records.rs:357-359` | True |
| 67 | `HttpsRecordData::supports_http3` | `pub fn supports_http3(&self) -> bool` | `/tmp/n0-dns-v011-src/src/records.rs:364-366` | True |
| 68 | `HttpsRecordData::port` | `pub fn port(&self) -> Option<u16>` | `/tmp/n0-dns-v011-src/src/records.rs:369-371` | True |
| 69 | `HttpsRecordData::ipv4hint` | `pub fn ipv4hint(&self) -> impl Iterator<Item = Ipv4Addr> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:374-376` | True |
| 70 | `HttpsRecordData::ipv6hint` | `pub fn ipv6hint(&self) -> impl Iterator<Item = Ipv6Addr> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:379-381` | True |
| 71 | `HttpsRecordData::ech` | `pub fn ech(&self) -> Option<&[u8]>` | `/tmp/n0-dns-v011-src/src/records.rs:384-386` | True |
| 72 | `HttpsRecordData::mandatory` | `pub fn mandatory(&self) -> impl Iterator<Item = u16> + '_` | `/tmp/n0-dns-v011-src/src/records.rs:391-393` | True |
| 73 | `HttpsRecordData::no_default_alpn` | `pub fn no_default_alpn(&self) -> bool` | `/tmp/n0-dns-v011-src/src/records.rs:396-398` | True |
| 74 | `n0_dns_resolver::TxtRecordData` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct TxtRecordData(Box<[Box<[u8]>]>);  /* private field */` | `/tmp/n0-dns-v011-src/src/records.rs:412-413` | True |
| 75 | `TxtRecordData::iter` | `pub fn iter(&self) -> impl Iterator<Item = &[u8]>` | `/tmp/n0-dns-v011-src/src/records.rs:417-419` | True |
| 76 | `TxtRecordData::into_boxed_slices` | `pub fn into_boxed_slices(self) -> Box<[Box<[u8]>]>` | `/tmp/n0-dns-v011-src/src/records.rs:426-428` | True |
| 77 | `impl IntoIterator for &TxtRecordData` | `impl<'a> IntoIterator for &'a TxtRecordData { type Item = &'a [u8]; type IntoIter = std::iter::Map<std::slice::Iter<'a, Box<[u8]>>, fn(&'a Box<[u8]>) -> &'a [u8]>; }` | `/tmp/n0-dns-v011-src/src/records.rs:431-438` | True |
| 78 | `impl fmt::Display for TxtRecordData` | `impl fmt::Display for TxtRecordData { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result }` | `/tmp/n0-dns-v011-src/src/records.rs:440-447` | True |
| 79 | `TxtRecordData conversions` | `impl FromIterator<Box<[u8]>> for TxtRecordData; impl From<Vec<Box<[u8]>>> for TxtRecordData; impl From<Vec<String>> for TxtRecordData` | `/tmp/n0-dns-v011-src/src/records.rs:449-470` | True |
| 80 | `n0_dns_resolver::Error` | `#[stack_error(derive, add_meta, std_sources)] #[non_exhaustive] pub enum Error { NoNameservers {}, Timeout {}, NoResponse {}, NxDomain {}, ServerError { code: ResponseCode }, InvalidResponse {}, InvalidName { name: String }, MissingTlsConfig {}, Transport { source: TransportError } }  /* every variant also #[non_exhaustive] */` | `/tmp/n0-dns-v011-src/src/error.rs:14-65` | True |
| 81 | `n0_dns_resolver::ResponseCode` | `#[derive(Debug, Clone, Copy, PartialEq, Eq)] #[non_exhaustive] pub enum ResponseCode { ServerFailure, Refused, FormatError, NotImplemented, Other }` | `/tmp/n0-dns-v011-src/src/error.rs:72-85` | True |
| 82 | `impl fmt::Display for ResponseCode` | `impl fmt::Display for ResponseCode { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result }` | `/tmp/n0-dns-v011-src/src/error.rs:87-98` | True |
| 83 | `n0_dns_resolver::TransportError (re-exported from resolver.rs:39)` | `#[stack_error(derive, add_meta, std_sources)] #[non_exhaustive] pub enum TransportError { Io { source: io::Error }, QueryTooLarge {}, UnexpectedResponse {}, #[cfg(transport_tls)] InvalidServerName { name: String }, #[cfg(transport_https)] Http { source: reqwest::Error }, #[cfg(transport_https)] BuildClient { source: reqwest::Error }, #[cfg(transport_https)] ResponseTooLarge {}, #[cfg(transport_https)] UnexpectedStatus { status: u16 } }` | `/tmp/n0-dns-v011-src/src/resolver/transport.rs:16-70 and /tmp/n0-dns-v011-src/src/resolver.rs:39` | True |
| 84 | `n0_dns_resolver::public_resolvers::Provider` | `#[derive(Debug, Copy, Clone, Eq, PartialEq)] #[non_exhaustive] pub enum Provider { Cloudflare, Google, Quad9 }` | `/tmp/n0-dns-v011-src/src/public_resolvers.rs:40-49` | True |
| 85 | `public_resolvers::Provider::ALL` | `pub const ALL: &'static [Provider] = &[Provider::Cloudflare, Provider::Google, Provider::Quad9];` | `/tmp/n0-dns-v011-src/src/public_resolvers.rs:51-53` | True |
| 86 | `public_resolvers::Provider::nameservers` | `pub fn nameservers(self, protocol: DnsProtocol) -> Vec<Nameserver>` | `/tmp/n0-dns-v011-src/src/public_resolvers.rs:82-94` | True |
| 87 | `n0_dns_resolver::public_resolvers::default_order` | `pub fn default_order(providers: Vec<Provider>) -> Vec<Nameserver>` | `/tmp/n0-dns-v011-src/src/public_resolvers.rs:97-137` | True |
| 88 | `iroh_dns::dns::DNS_TIMEOUT` | `pub const DNS_TIMEOUT: Duration = Duration::from_secs(3);` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:32-33` | True |
| 89 | `iroh_dns::dns::N0_DNS_ENDPOINT_ORIGIN_PROD` | `pub const N0_DNS_ENDPOINT_ORIGIN_PROD: &str = "dns.iroh.link.";` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:35-36` | True |
| 90 | `iroh_dns::dns::N0_DNS_ENDPOINT_ORIGIN_STAGING` | `pub const N0_DNS_ENDPOINT_ORIGIN_STAGING: &str = "staging-dns.iroh.link.";` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:37-38` | True |
| 91 | `iroh_dns::dns::install_android_jni_context (Android/doc)` | `#[cfg(any(target_os = "android", doc))] pub unsafe fn install_android_jni_context(java_vm: *mut std::ffi::c_void, application_context: *mut std::ffi::c_void)` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:88-99` | True |
| 92 | `iroh_dns::dns::Resolver (trait)` | `pub trait Resolver: fmt::Debug + Send + Sync + 'static { fn lookup_ipv4(&self, host: String) -> BoxFuture<Result<BoxIter<Ipv4Addr>, DnsError>>; fn lookup_ipv6(&self, host: String) -> BoxFuture<Result<BoxIter<Ipv6Addr>, DnsError>>; fn lookup_txt(&self, host: String) -> BoxFuture<Result<BoxIter<TxtRecordData>, DnsError>>; fn clear_cache(&self); fn reset(&self) -> Box<dyn Resolver>; }` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:104-126` | True |
| 93 | `iroh_dns::dns::BoxIter` | `pub type BoxIter<T> = Box<dyn Iterator<Item = T> + Send + 'static>;` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:128-131` | True |
| 94 | `iroh_dns::dns::DefaultResolver (private adapter)` | `#[derive(Debug)] struct DefaultResolver(Arc<n0_dns_resolver::DnsResolver>);` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:556-561` | True |
| 95 | `impl iroh_dns::dns::Resolver for DefaultResolver` | `impl Resolver for DefaultResolver { fn lookup_ipv4(&self, host: String) -> BoxFuture<Result<BoxIter<Ipv4Addr>, DnsError>>; fn lookup_ipv6(&self, host: String) -> BoxFuture<Result<BoxIter<Ipv6Addr>, DnsError>>; fn lookup_txt(&self, host: String) -> BoxFuture<Result<BoxIter<TxtRecordData>, DnsError>>; fn clear_cache(&self); fn reset(&self) -> Box<dyn Resolver>; }` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:563-598` | True |
| 96 | `iroh_dns::dns::map_resolve_error (private)` | `fn map_resolve_error(err: n0_dns_resolver::Error) -> DnsError` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:600-616` | True |
| 97 | `iroh_dns::dns::convert_txt (private)` | `fn convert_txt(txt: n0_dns_resolver::TxtRecordData) -> TxtRecordData` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:618-624` | True |
| 98 | `iroh_dns::dns::Builder::into_resolver_builder (private)` | `fn into_resolver_builder(self) -> n0_dns_resolver::Builder` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:519-553` | True |
| 99 | `iroh_dns::dns::Builder::build` | `pub fn build(self) -> DnsResolver` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:512-517` | True |
| 100 | `iroh_dns::dns::DnsResolver` | `#[derive(Debug, Clone)] pub struct DnsResolver { /* private: inner: Arc<Inner> */ }` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:626-655` | True |
| 101 | `iroh_dns::dns::DnsResolver::new` | `pub fn new() -> Self` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:748-750` | True |
| 102 | `iroh_dns::dns::DnsResolver::with_nameserver` | `pub fn with_nameserver(nameserver: SocketAddr) -> Self` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:752-759` | True |
| 103 | `iroh_dns::dns::DnsResolver::builder` | `pub fn builder() -> Builder` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:762-764` | True |
| 104 | `iroh_dns::dns::DnsResolver::custom` | `pub fn custom(resolver: impl Resolver) -> Self` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:771-775` | True |
| 105 | `iroh_dns::dns::DnsResolver::clear_cache` | `pub fn clear_cache(&self)` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:778-780` | True |
| 106 | `iroh_dns::dns::DnsResolver::reset` | `pub fn reset(&self)` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:787-789` | True |
| 107 | `iroh_dns::dns::DnsResolver::lookup_txt` | `pub async fn lookup_txt<T: ToString>(&self, host: T, timeout: Duration) -> Result<impl Iterator<Item = TxtRecordData>, DnsError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:791-803` | True |
| 108 | `iroh_dns::dns::DnsResolver::lookup_ipv4` | `pub async fn lookup_ipv4<T: ToString>(&self, host: T, timeout: Duration) -> Result<impl Iterator<Item = IpAddr> + use<T>, DnsError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:805-817` | True |
| 109 | `iroh_dns::dns::DnsResolver::lookup_ipv6` | `pub async fn lookup_ipv6<T: ToString>(&self, host: T, timeout: Duration) -> Result<impl Iterator<Item = IpAddr> + use<T>, DnsError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:819-831` | True |
| 110 | `iroh_dns::dns::DnsResolver::lookup_ipv4_ipv6` | `pub async fn lookup_ipv4_ipv6<T: ToString>(&self, host: T, timeout: Duration) -> Result<impl Iterator<Item = IpAddr> + use<T>, DnsError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:833-858` | True |
| 111 | `iroh_dns::dns::DnsResolver::resolve_host` | `pub async fn resolve_host(&self, url: &Url, prefer_ipv6: bool, timeout: Duration) -> Result<IpAddr, DnsError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:860-895` | True |
| 112 | `iroh_dns::dns::DnsResolver::resolve_host_all` | `pub fn resolve_host_all<'a>(&'a self, url: &Url, timeout: Duration) -> impl Stream<Item = Result<IpAddr, DnsError>> + Send + 'a` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:897-912` | True |
| 113 | `iroh_dns::dns::DnsResolver::lookup_ipv4_ipv6_staggered` | `pub async fn lookup_ipv4_ipv6_staggered(&self, host: impl ToString, timeout: Duration, delays_ms: &[u64]) -> Result<impl Iterator<Item = IpAddr>, StaggeredError<DnsError>>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:1035-1051` | True |
| 114 | `iroh_dns::dns::DnsResolver::lookup_endpoint_by_id` | `pub async fn lookup_endpoint_by_id(&self, endpoint_id: &EndpointId, origin: &str) -> Result<EndpointInfo, LookupError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:1053-1066` | True |
| 115 | `iroh_dns::dns::DnsResolver::lookup_endpoint_by_domain_name` | `pub async fn lookup_endpoint_by_domain_name(&self, name: &str) -> Result<EndpointInfo, LookupError>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:1068-1081` | True |
| 116 | `iroh_dns::dns::DnsResolver::lookup_endpoint_by_id_staggered` | `pub async fn lookup_endpoint_by_id_staggered(&self, endpoint_id: &EndpointId, origin: &str, delays_ms: &[u64]) -> Result<EndpointInfo, StaggeredError<LookupError>>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:1098-1112` | True |
| 117 | `iroh_dns::dns::DnsResolver::lookup_endpoint_by_domain_name_staggered` | `pub async fn lookup_endpoint_by_domain_name_staggered(&self, name: &str, delays_ms: &[u64]) -> Result<EndpointInfo, StaggeredError<LookupError>>` | `/tmp/iroh-v150-src/iroh-dns/src/dns.rs:1083-1096` | True |
| 118 | `iroh::address_lookup::dns::DNS_STAGGERING_MS (pub(crate))` | `pub(crate) const DNS_STAGGERING_MS: &[u64] = &[200, 300, 600, 1000, 2000, 3000];` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:18-22` | True |
| 119 | `iroh::address_lookup::dns::DnsAddressLookup` | `#[derive(Debug)] pub struct DnsAddressLookup { /* private: origin_domain: String, dns_resolver: DnsResolver */ }` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:45-49` | True |
| 120 | `iroh::address_lookup::dns::DnsAddressLookupBuilder::dns_resolver` | `pub fn dns_resolver(mut self, dns_resolver: DnsResolver) -> Self` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:60-65` | True |
| 121 | `iroh::address_lookup::dns::DnsAddressLookupBuilder::build` | `pub fn build(self) -> DnsAddressLookup` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:67-73` | True |
| 122 | `iroh::address_lookup::dns::DnsAddressLookup::builder` | `pub fn builder(origin_domain: String) -> DnsAddressLookupBuilder` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:76-83` | True |
| 123 | `iroh::address_lookup::dns::DnsAddressLookup::n0_dns` | `pub fn n0_dns() -> DnsAddressLookupBuilder` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:85-98` | True |
| 124 | `iroh::address_lookup::dns::DnsAddressLookup::resolve (trait impl)` | `fn resolve(&self, endpoint_id: EndpointId) -> Option<BoxStream<Result<AddressLookupItem, AddressLookupError>>>` | `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs:113-135` | True |
| 125 | `iroh::address_lookup::pkarr::N0_DNS_PKARR_RELAY_PROD` | `pub const N0_DNS_PKARR_RELAY_PROD: &str = "https://dns.iroh.link/pkarr";` | `/tmp/iroh-v150-src/iroh/src/address_lookup/pkarr.rs:127` | True |
| 126 | `iroh::address_lookup::pkarr::N0_DNS_PKARR_RELAY_STAGING` | `pub const N0_DNS_PKARR_RELAY_STAGING: &str = "https://staging-dns.iroh.link/pkarr";` | `/tmp/iroh-v150-src/iroh/src/address_lookup/pkarr.rs:128-134` | True |
| 127 | `iroh::address_lookup::pkarr::DEFAULT_PKARR_TTL` | `pub const DEFAULT_PKARR_TTL: u32 = 30;` | `/tmp/iroh-v150-src/iroh/src/address_lookup/pkarr.rs:136-143` | True |
| 128 | `iroh::address_lookup::pkarr::PkarrPublisherBuilder::dns_resolver` | `#[cfg(not(wasm_browser))] pub fn dns_resolver(mut self, dns_resolver: DnsResolver) -> Self` | `/tmp/iroh-v150-src/iroh/src/address_lookup/pkarr.rs:206-211` | True |
| 129 | `iroh::endpoint::Builder::dns_resolver` | `#[cfg(not(wasm_browser))] pub fn dns_resolver(mut self, dns_resolver: DnsResolver) -> Self` | `/tmp/iroh-v150-src/iroh/src/endpoint.rs:683-696` | True |
| 130 | `iroh::endpoint::ENV_FORCE_STAGING_RELAYS` | `pub const ENV_FORCE_STAGING_RELAYS: &str = "IROH_FORCE_STAGING_RELAYS";` | `/tmp/iroh-v150-src/iroh/src/endpoint.rs:2043-2044` | True |
| 131 | `iroh::endpoint::force_staging_infra` | `pub fn force_staging_infra() -> bool` | `/tmp/iroh-v150-src/iroh/src/endpoint.rs:2046-2049` | True |
| 132 | `iroh::Endpoint::dns_resolver` | `pub fn dns_resolver(&self) -> Result<&DnsResolver, EndpointError>` | `/tmp/iroh-v150-src/iroh/src/endpoint.rs:1482-1489` | True |
| 133 | `iroh::defaults::prod relay hostnames` | `pub const NA_EAST_RELAY_HOSTNAME: &str = "use1-1.relay.n0.iroh.link."; pub const NA_WEST_RELAY_HOSTNAME: &str = "usw1-1.relay.n0.iroh.link."; pub const EU_RELAY_HOSTNAME: &str = "euc1-1.relay.n0.iroh.link."; pub const AP_RELAY_HOSTNAME: &str = "aps1-1.relay.n0.iroh.link.";` | `/tmp/iroh-v150-src/iroh/src/defaults.rs:26-33` | True |
| 134 | `iroh::defaults::prod::default_relay_map` | `pub fn default_relay_map() -> RelayMap` | `/tmp/iroh-v150-src/iroh/src/defaults.rs:35-43` | True |
| 135 | `iroh::defaults::staging relay hostnames` | `pub const NA_EAST_RELAY_HOSTNAME: &str = "use1-1.staging-relay.n0.iroh.link."; pub const EU_RELAY_HOSTNAME: &str = "euc1-1.staging-relay.n0.iroh.link.";` | `/tmp/iroh-v150-src/iroh/src/defaults.rs:93-96` | True |
| 136 | `iroh::util::reqwest_client_builder (pub(crate))` | `#[cfg(not(wasm_browser))] pub(crate) fn reqwest_client_builder(tls_client_config: rustls::ClientConfig, dns_resolver: crate::dns::DnsResolver) -> reqwest::ClientBuilder` | `/tmp/iroh-v150-src/iroh/src/util.rs:9-19` | True |
| 137 | `iroh::util::ReqwestDnsResolver (pub(super)) implements reqwest::dns::Resolve` | `impl reqwest::dns::Resolve for ReqwestDnsResolver { fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving }` | `/tmp/iroh-v150-src/iroh/src/util.rs:34-70` | True |
| 138 | `iroh_relay::client::tls::dial_happy_eyeballs (private)` | `async fn dial_happy_eyeballs(dns_resolver: &DnsResolver, url: &Url, prefer_ipv6: bool) -> Result<TcpStream, DialError>` | `/tmp/iroh-v150-src/iroh-relay/src/client/tls.rs:250-258` | True |
| 139 | `iroh::test_utils::DnsPkarrServer::run_with_origin` | `pub async fn run_with_origin(endpoint_origin: String) -> std::io::Result<Self>` | `/tmp/iroh-v150-src/iroh/src/test_utils.rs:124-137` | True |
| 140 | `iroh::test_utils::DnsPkarrServer::dns_resolver` | `pub fn dns_resolver(&self) -> DnsResolver` | `/tmp/iroh-v150-src/iroh/src/test_utils.rs:164-167` | True |

**Contract source refs**

- `https://index.crates.io/n0/-d/n0-dns-resolver`
- `https://static.crates.io/crates/n0-dns-resolver/n0-dns-resolver-0.1.1.crate`
- `/tmp/n0-dns-crate-v011/extracted/n0-dns-resolver-0.1.1/.cargo_vcs_info.json`
- `/tmp/n0-dns-crate-v011/extracted/n0-dns-resolver-0.1.1/Cargo.toml`
- `/tmp/n0-dns-crate-v011/extracted/n0-dns-resolver-0.1.1/Cargo.toml.orig`
- `/tmp/n0-dns-crate-v011/extracted/n0-dns-resolver-0.1.1/Cargo.toml`
- `gh api repos/n0-computer/n0-dns-resolver/git/trees/HEAD?recursive=1`
- `gh api repos/n0-computer/n0-dns-resolver/git/trees/v0.1.1?recursive=1`
- `gh api repos/n0-computer/n0-dns-resolver/tags?per_page=100`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/lib.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/builder.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/resolver.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/records.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/error.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/public_resolvers.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/resolver/transport.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/src/system_config/android.rs?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/README.md?ref=v0.1.1`
- `gh api repos/n0-computer/n0-dns-resolver/contents/Cargo.toml?ref=v0.1.1`
- `https://docs.rs/n0-dns-resolver/0.1.1/n0_dns_resolver/`
- `https://docs.rs/n0-dns-resolver/0.1.1/n0_dns_resolver/struct.DnsResolver.html`
- `/tmp/n0-dns-v011-src/examples/compare_iroh_lookups.rs`
- `gh api search/code?q=n0_dns_resolver%20repo:n0-computer/iroh`
- `gh api search/code?q=N0_DNS%20repo:n0-computer/iroh`
- `gh api repos/n0-computer/iroh/tags?per_page=20`
- `gh api repos/n0-computer/iroh/git/trees/main?recursive=1`
- `gh api repos/n0-computer/iroh/contents/Cargo.lock?ref=v1.5.0`
- `/tmp/iroh-v150-src/Cargo.lock`
- `/tmp/iroh-v150-src/iroh-dns/Cargo.toml`
- `/tmp/iroh-v150-src/iroh-dns/src/lib.rs`
- `/tmp/iroh-v150-src/iroh-dns/src/dns.rs`
- `/tmp/iroh-v150-src/iroh/src/lib.rs`
- `/tmp/iroh-v150-src/iroh/src/address_lookup.rs`
- `/tmp/iroh-v150-src/iroh/src/address_lookup/dns.rs`
- `/tmp/iroh-v150-src/iroh/src/address_lookup/pkarr.rs`
- `/tmp/iroh-v150-src/iroh/src/defaults.rs`
- `/tmp/iroh-v150-src/iroh/src/endpoint.rs`
- `/tmp/iroh-v150-src/iroh/src/endpoint/presets.rs`
- `/tmp/iroh-v150-src/iroh/src/util.rs`
- `/tmp/iroh-v150-src/iroh/src/test_utils.rs`
- `/tmp/iroh-v150-src/iroh-relay/src/client/tls.rs`
- `/tmp/iroh-v150-src/iroh/examples/transfer.rs`
- `/tmp/iroh-v150-src/iroh-dns-server/examples/resolve.rs`
- `/tmp/iroh-v150-src/iroh/Cargo.toml`

### 1.8 `iroh-docs` -- exact version `0.101.0`

- **License:** MIT/Apache-2.0 (dual license, at your option; Cargo.toml:7, README.md:92-103; LICENSE-MIT and LICENSE-APACHE present in published crate)
- **MSRV / Go:** Rust 1.91 (rust-version = "1.91" in tag Cargo.toml and published Cargo.toml; crates.io index rust_version = "1.91"). Edition 2021. Not a Go dependency.
- **Signatures:** 73; **unverified items:** 8; **source refs:** 32

**Feature flags / version gates**

- `default = ["metrics", "rpc", "fs-store", "redb-v2-migration"]`
- `metrics = ["iroh-metrics/metrics", "iroh/metrics"]`
- `rpc = ["dep:noq", "irpc/rpc", "iroh-blobs/rpc"]`
- `fs-store = ["iroh-blobs/fs-store"]`
- `redb-v2-migration = ["dep:redb_v3"]`

**Integration notes (verbatim)**

````text
Verified provenance: crates.io index https://index.crates.io/ir/oh/iroh-docs contains exactly one 0.101.0 record (not yanked, rust_version 1.91, cksum 8fd1bd5e39d0321a3c4a2bcef9650476c076e2df41a0e84577eca23d6de6c8ab). The downloaded .crate SHA-256 equals that cksum; every src/*.rs, README.md, CHANGELOG.md and license file fetched from GitHub tag v0.101.0 (commit 091e8cac47bbc49cdb84b0bfed227cc163b61dfe) is byte-identical to the published crate; only Cargo.toml differs because Cargo normalizes it at publish time. The main branch HEAD (8cfeacb087b4b195b1930683aa4448e990da0659) is NOT the tag; all signatures below are from the tag/published crate.

Shape of the 0.101.0 public API: the entry point is iroh_docs::protocol::Docs (a ProtocolHandler), which owns an Arc<Engine> and a DocsApi. Docs implements Deref<Target = DocsApi>, so docs.create(), docs.open(), docs.list(), docs.author_*() are DocsApi methods reached through Deref, not inherent Docs methods. iroh_docs::api::DocsApi is the node-wide client; iroh_docs::api::Doc is a per-namespace handle. Docs::new exists but takes an already-spawned Engine, not a storage path. Root re-exports (src/lib.rs:57) are `pub use self::{heads::*, keys::*, sync::*, ticket::DocTicket};`, so Entry, AuthorId, NamespaceId, ContentStatus, Capability, Replica, etc. are available at the crate root; Docs is NOT root-re-exported (use iroh_docs::protocol::Docs). ALPN is b"/iroh-sync/1".

Requested-name reconciliation (exhaustive grep of the published crate plus docs.rs). In 0.101.0 there is no Docs::author, Doc::insert, Doc::keys, Doc::delete, Doc::import_export, or Doc::content_status method. The real equivalents are: authors -> DocsApi::author_create/author_default/author_set_default/author_list/author_export/author_import/author_delete; insert -> Doc::set_bytes (writes value via iroh-blobs then inserts its Hash) or Doc::set_hash (inserts a pre-existing Hash + size), with low-level Replica::insert / Replica::hash_and_insert / SyncHandle::insert_local; keys -> Doc::get_many(impl Into<Query>) / Doc::get_one with Query::all / Query::key_prefix / Query::key_exact (there is no dedicated keys() method); delete -> Doc::del(author_id, prefix) -> usize, which inserts an empty prefix tombstone rather than removing rows (low-level Replica::delete_prefix); import/export -> Doc::import_file / Doc::export_file plus DocsApi::import / import_namespace / import_and_subscribe and DocsApi::author_import / author_export; content status -> the ContentStatus enum {Complete, Incomplete, Missing}, the ContentStatusCallback type alias, LiveEvent::InsertRemote { content_status, .. } and LiveEvent::ContentReady { hash }. Doc::status() returns actor::OpenState { sync: bool, subscribers: usize, handles: usize } and contains NO content-status field. Engine::entry_to_content_status (src/engine.rs:258-269) is a private helper mapping irpc::Result<iroh_blobs::api::blobs::BlobStatus> to ContentStatus (errors map to Missing).

Sync start: Doc::start_sync(&self, peers: Vec<EndpointAddr>) -> Result<()> sends StartSyncRequest { doc_id, peers } to the RPC actor, which calls Engine::start_sync(doc_id, peers) (src/api/actor.rs:465-474). Engine::start_sync (src/engine.rs:167-182) forwards ToLiveActor::StartSync; a non-empty peers list triggers initial range-based set reconciliation with each peer AND joins an iroh-gossip swarm. `engine::live::ToLiveActor` lives in a private module and is not public API. DocsApi::import(ticket) calls doc.start_sync(ticket.nodes) automatically; DocsApi::import_namespace(capability) explicitly does NOT start sync; DocsApi::import_and_subscribe creates the subscription stream before starting sync so the first events are not missed. Doc::share(mode, addr_options) also internally calls start_sync(doc_id, vec![]) (src/api/actor.rs:385-412), builds Capability::Read(doc_id) or Capability::Write(secret) and returns DocTicket { capability, nodes: vec![local_endpoint_addr] }. Doc::leave() stops live sync. Doc::subscribe() yields Result<LiveEvent> with Send + Unpin + 'static; DocsApi::list() and DocsApi::author_list() also return streams, not Vecs.

Documents are multi-dimensional key-value CRDTs keyed by (NamespaceId, AuthorId, key); the value is a 32-byte BLAKE3 content hash plus content length and a timestamp. Content bytes are NOT stored in docs: they live in iroh-blobs (src/lib.rs:4-8, README.md:13-17). Doc::get_exact(author, key, include_empty) has an explicit include_empty flag for deletion markers; Doc::del returns the number of entries removed and Doc::set_bytes returns the resulting Hash. `Doc::import_file` returns api::ImportFileProgress, which implements both `Stream<Item = ImportFileProgressItem>` and `Future<Output = Result<ImportFileOutcome>>` (src/api.rs:576-686); `Doc::export_file` returns iroh_blobs::api::blobs::ExportProgress. Doc::close() sets an AtomicBool so subsequent operations fail with "document is closed" (src/api.rs:291-307). DocsApi::open() returns Result<Option<Doc>> but its implementation always returns Some(Doc) after a successful RPC; a missing document surfaces as an RPC error (src/api.rs:259-265, src/api/actor.rs:358-364).

Setup (verified in README.md:50-90 and examples/setup.rs:1-37): build iroh_blobs and iroh_gossip protocols, then `Docs::memory().spawn(endpoint.clone(), (*blobs).clone(), gossip.clone()).await?`; register with `Router::builder(endpoint).accept(BLOBS_ALPN, ...).accept(GOSSIP_ALPN, gossip).accept(DOCS_ALPN, docs).spawn()`. `Docs::persistent(PathBuf)` is gated on feature `fs-store` and stores docs.redb plus a default-author file in that directory (src/protocol.rs:107-118). Feature-gated items: `DocsApi::connect`/`listen` require `rpc`; `DefaultAuthorStorage::Persistent` requires `fs-store`. Default features are metrics, rpc, fs-store, redb-v2-migration. Dependency co-versions in Cargo.toml: iroh 1 (default-features=false), iroh-blobs 0.103, iroh-gossip 0.101.0, iroh-metrics 1, iroh-tickets 1, irpc 0.17, n0-future 0.3.1, redb 4.1; edition 2021, rust-version 1.91. `Result` in src/api.rs is anyhow::Result; `Stream` is n0_future::Stream; `Hash` is iroh_blobs::Hash; `ExportMode`/`ExportProgress`/`ImportMode` come from iroh_blobs::api::blobs. License is dual MIT/Apache-2.0 at your option (Cargo.toml:7, README.md:92-103).
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `iroh_docs::protocol::Docs` | `#[derive(Debug, Clone)] pub struct Docs { engine: Arc<Engine>, api: DocsApi }  // fields private` | `src/protocol.rs:24-29 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html` | True |
| 2 | `iroh_docs::protocol::Docs::memory` | `pub fn memory() -> Builder` | `src/protocol.rs:31-35 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html` | True |
| 3 | `iroh_docs::protocol::Docs::persistent` | `#[cfg(feature = "fs-store")] pub fn persistent(path: std::path::PathBuf) -> Builder` | `src/protocol.rs:37-45 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html` | True |
| 4 | `iroh_docs::protocol::Docs::new` | `pub fn new(engine: Engine) -> Self` | `src/protocol.rs:47-52 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html` | True |
| 5 | `iroh_docs::protocol::Docs::api` | `pub fn api(&self) -> &DocsApi` | `src/protocol.rs:54-57 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html` | True |
| 6 | `impl std::ops::Deref for iroh_docs::protocol::Docs` | `impl std::ops::Deref for Docs { type Target = DocsApi; fn deref(&self) -> &Self::Target }` | `src/protocol.rs:60-66 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html` | True |
| 7 | `iroh_docs::protocol::Builder::protect_handler` | `pub fn protect_handler(mut self, protect_handler: ProtectCallbackHandler) -> Self` | `src/protocol.rs:91-98 @ v0.101.0` | True |
| 8 | `iroh_docs::protocol::Builder::spawn` | `pub async fn spawn(self, endpoint: Endpoint, blobs: BlobsStore, gossip: Gossip) -> anyhow::Result<Docs>` | `src/protocol.rs:100-131 @ v0.101.0` | True |
| 9 | `iroh_docs::api::DocsApi` | `#[derive(Debug, Clone)] pub struct DocsApi { pub(crate) inner: Client }  // Client = irpc::Client<DocsProtocol>` | `src/api.rs:50-56 @ v0.101.0` | True |
| 10 | `iroh_docs::api::DocsApi::spawn` | `pub fn spawn(engine: Arc<Engine>) -> Self` | `src/api.rs:58-62 @ v0.101.0` | True |
| 11 | `iroh_docs::api::DocsApi::connect` | `#[cfg(feature = "rpc")] pub fn connect(endpoint: noq::Endpoint, addr: std::net::SocketAddr) -> Result<DocsApi>` | `src/api.rs:64-70 @ v0.101.0` | True |
| 12 | `iroh_docs::api::DocsApi::listen` | `#[cfg(feature = "rpc")] pub fn listen(&self, endpoint: noq::Endpoint) -> Result<n0_future::task::AbortOnDropHandle<()>>` | `src/api.rs:72-120 @ v0.101.0` | True |
| 13 | `iroh_docs::api::DocsApi::author_create` | `pub async fn author_create(&self) -> Result<AuthorId>` | `src/api.rs:122-131 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html` | True |
| 14 | `iroh_docs::api::DocsApi::author_default` | `pub async fn author_default(&self) -> Result<AuthorId>` | `src/api.rs:133-142 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html` | True |
| 15 | `iroh_docs::api::DocsApi::author_set_default` | `pub async fn author_set_default(&self, author_id: AuthorId) -> Result<()>` | `src/api.rs:144-155 @ v0.101.0` | True |
| 16 | `iroh_docs::api::DocsApi::author_list` | `pub async fn author_list(&self) -> Result<impl Stream<Item = Result<AuthorId>>>` | `src/api.rs:157-167 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html` | True |
| 17 | `iroh_docs::api::DocsApi::author_export` | `pub async fn author_export(&self, author: AuthorId) -> Result<Option<Author>>` | `src/api.rs:169-175 @ v0.101.0` | True |
| 18 | `iroh_docs::api::DocsApi::author_import` | `pub async fn author_import(&self, author: Author) -> Result<()>` | `src/api.rs:177-183 @ v0.101.0` | True |
| 19 | `iroh_docs::api::DocsApi::author_delete` | `pub async fn author_delete(&self, author: AuthorId) -> Result<()>` | `src/api.rs:185-193 @ v0.101.0` | True |
| 20 | `iroh_docs::api::DocsApi::create` | `pub async fn create(&self) -> Result<Doc>` | `src/api.rs:195-199 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html` | True |
| 21 | `iroh_docs::api::DocsApi::drop_doc` | `pub async fn drop_doc(&self, doc_id: NamespaceId) -> Result<()>` | `src/api.rs:201-209 @ v0.101.0` | True |
| 22 | `iroh_docs::api::DocsApi::import_namespace` | `pub async fn import_namespace(&self, capability: Capability) -> Result<Doc>` | `src/api.rs:211-217 @ v0.101.0` | True |
| 23 | `iroh_docs::api::DocsApi::import` | `pub async fn import(&self, ticket: DocTicket) -> Result<Doc>` | `src/api.rs:219-225 @ v0.101.0` | True |
| 24 | `iroh_docs::api::DocsApi::import_and_subscribe` | `pub async fn import_and_subscribe(&self, ticket: DocTicket) -> Result<(Doc, impl Stream<Item = Result<LiveEvent>>)>` | `src/api.rs:227-243 @ v0.101.0` | True |
| 25 | `iroh_docs::api::DocsApi::list` | `pub async fn list(&self) -> Result<impl Stream<Item = Result<(NamespaceId, CapabilityKind)>> + Unpin + Send + 'static>` | `src/api.rs:245-257 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html` | True |
| 26 | `iroh_docs::api::DocsApi::open` | `pub async fn open(&self, id: NamespaceId) -> Result<Option<Doc>>` | `src/api.rs:259-265 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html` | True |
| 27 | `iroh_docs::api::Doc` | `#[derive(Debug, Clone)] pub struct Doc { inner: Client, namespace_id: NamespaceId, closed: Arc<AtomicBool> }  // fields private` | `src/api.rs:268-274 @ v0.101.0` | True |
| 28 | `iroh_docs::api::Doc::id` | `pub fn id(&self) -> NamespaceId` | `src/api.rs:285-288 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 29 | `iroh_docs::api::Doc::close` | `pub async fn close(&self) -> Result<()>` | `src/api.rs:290-299 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 30 | `iroh_docs::api::Doc::set_bytes` | `pub async fn set_bytes(&self, author_id: AuthorId, key: impl Into<Bytes>, value: impl Into<Bytes>) -> Result<Hash>` | `src/api.rs:309-327 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 31 | `iroh_docs::api::Doc::set_hash` | `pub async fn set_hash(&self, author_id: AuthorId, key: impl Into<Bytes>, hash: Hash, size: u64) -> Result<()>` | `src/api.rs:329-348 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 32 | `iroh_docs::api::Doc::del` | `pub async fn del(&self, author_id: AuthorId, prefix: impl Into<Bytes>) -> Result<usize>` | `src/api.rs:350-367 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 33 | `iroh_docs::api::Doc::get_exact` | `pub async fn get_exact(&self, author: AuthorId, key: impl AsRef<[u8]>, include_empty: bool) -> Result<Option<Entry>>` | `src/api.rs:369-389 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 34 | `iroh_docs::api::Doc::get_many` | `pub async fn get_many(&self, query: impl Into<Query>) -> Result<impl Stream<Item = Result<Entry>>>` | `src/api.rs:391-412 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 35 | `iroh_docs::api::Doc::get_one` | `pub async fn get_one(&self, query: impl Into<Query>) -> Result<Option<Entry>>` | `src/api.rs:414-420 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 36 | `iroh_docs::api::Doc::share` | `pub async fn share(&self, mode: ShareMode, addr_options: AddrInfoOptions) -> Result<DocTicket>` | `src/api.rs:422-434 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 37 | `iroh_docs::api::Doc::start_sync` | `pub async fn start_sync(&self, peers: Vec<EndpointAddr>) -> Result<()>` | `src/api.rs:436-446 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 38 | `iroh_docs::api::Doc::leave` | `pub async fn leave(&self) -> Result<()>` | `src/api.rs:448-457 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 39 | `iroh_docs::api::Doc::subscribe` | `pub async fn subscribe(&self) -> Result<impl Stream<Item = Result<LiveEvent>> + Send + Unpin + 'static>` | `src/api.rs:459-478 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 40 | `iroh_docs::api::Doc::status` | `pub async fn status(&self) -> Result<OpenState>` | `src/api.rs:480-490 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html` | True |
| 41 | `iroh_docs::api::Doc::import_file` | `pub async fn import_file(&self, blobs: &iroh_blobs::api::Store, author: AuthorId, key: Bytes, path: impl AsRef<Path>, import_mode: iroh_blobs::api::blobs::ImportMode) -> Result<ImportFileProgress>` | `src/api.rs:528-555 @ v0.101.0` | True |
| 42 | `iroh_docs::api::Doc::export_file` | `pub async fn export_file(&self, blobs: &iroh_blobs::api::Store, entry: Entry, path: impl AsRef<Path>, mode: ExportMode) -> Result<ExportProgress>` | `src/api.rs:557-573 @ v0.101.0` | True |
| 43 | `iroh_docs::api::ImportFileOutcome` | `#[derive(Debug, Clone, PartialEq, Eq)] pub struct ImportFileOutcome { pub hash: Hash, pub size: u64, pub key: Bytes }` | `src/api.rs:688-697 @ v0.101.0` | True |
| 44 | `iroh_docs::engine::Engine::spawn` | `pub async fn spawn(endpoint: Endpoint, gossip: Gossip, replica_store: crate::store::Store, bao_store: iroh_blobs::api::Store, downloader: Downloader, default_author_storage: DefaultAuthorStorage, protect_cb: Option<ProtectCallbackHandler>) -> anyhow::Result<Self>` | `src/engine.rs:58-155 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/engine/struct.Engine.html` | True |
| 45 | `iroh_docs::engine::Engine::start_sync` | `pub async fn start_sync(&self, namespace: NamespaceId, peers: Vec<EndpointAddr>) -> Result<()>` | `src/engine.rs:167-182 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/engine/struct.Engine.html` | True |
| 46 | `iroh_docs::engine::Engine::subscribe` | `pub async fn subscribe(&self, namespace: NamespaceId) -> Result<impl Stream<Item = Result<LiveEvent>> + Unpin + 'static>` | `src/engine.rs:201-237 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/engine/struct.Engine.html` | True |
| 47 | `iroh_docs::engine::LiveEvent` | `#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, strum::Display)] pub enum LiveEvent { InsertLocal { entry: Entry }, InsertRemote { from: PublicKey, entry: Entry, content_status: ContentStatus }, ContentReady { hash: Hash }, PendingContentReady, NeighborUp(PublicKey), NeighborDown(PublicKey), SyncFinished(SyncEvent) }` | `src/engine.rs:271-309 @ v0.101.0` | True |
| 48 | `iroh_docs::engine::SyncEvent (re-export of engine::live::SyncEvent)` | `#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)] pub struct SyncEvent { pub peer: PublicKey, pub origin: Origin, pub finished: SystemTime, pub started: SystemTime, pub result: std::result::Result<SyncDetails, String> }` | `src/engine/live.rs:831-844 @ v0.101.0` | True |
| 49 | `iroh_docs::ALPN` | `pub const ALPN: &[u8] = b"/iroh-sync/1";` | `src/net.rs:17-18 @ v0.101.0` | True |
| 50 | `iroh_docs::ContentStatus` | `#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)] pub enum ContentStatus { Complete, Incomplete, Missing }` | `src/sync.rs:97-106 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/enum.ContentStatus.html` | True |
| 51 | `iroh_docs::ContentStatusCallback` | `pub type ContentStatusCallback = Arc<dyn Fn(Hash) -> n0_future::boxed::BoxFuture<ContentStatus> + Send + Sync + 'static>;` | `src/sync.rs:50-52 @ v0.101.0` | True |
| 52 | `iroh_docs::actor::OpenState` | `#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)] pub struct OpenState { pub sync: bool, pub subscribers: usize, pub handles: usize }` | `src/actor.rs:202-211 @ v0.101.0` | True |
| 53 | `iroh_docs::Entry` | `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)] pub struct Entry { id: RecordIdentifier, record: Record }  // fields private` | `src/sync.rs:908-917 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 54 | `iroh_docs::Entry::content_hash` | `pub fn content_hash(&self) -> Hash` | `src/sync.rs:968-971 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 55 | `iroh_docs::Entry::content_len` | `pub fn content_len(&self) -> u64` | `src/sync.rs:973-976 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 56 | `iroh_docs::Entry::key` | `pub fn key(&self) -> &[u8]` | `src/sync.rs:958-961 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 57 | `iroh_docs::Entry::author` | `pub fn author(&self) -> AuthorId` | `src/sync.rs:953-956 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 58 | `iroh_docs::Entry::namespace` | `pub fn namespace(&self) -> NamespaceId` | `src/sync.rs:948-951 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 59 | `iroh_docs::Entry::timestamp` | `pub fn timestamp(&self) -> u64` | `src/sync.rs:978-981 @ v0.101.0 \| https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html` | True |
| 60 | `iroh_docs::AuthorId` | `#[derive(Default, Clone, Copy, PartialOrd, Ord, Eq, PartialEq, Hash, derive_more::From, derive_more::Into, derive_more::AsRef, Serialize, Deserialize)] pub struct AuthorId([u8; 32]);` | `src/keys.rs:351-367 @ v0.101.0` | True |
| 61 | `iroh_docs::NamespaceId` | `#[derive(Default, Clone, Copy, PartialOrd, Ord, Eq, PartialEq, Hash, derive_more::From, derive_more::Into, derive_more::AsRef, Serialize, Deserialize)] pub struct NamespaceId([u8; 32]);` | `src/keys.rs:333-349 @ v0.101.0` | True |
| 62 | `iroh_docs::DocTicket` | `#[derive(Serialize, Deserialize, Clone, Debug, derive_more::Display)] pub struct DocTicket { pub capability: Capability, pub nodes: Vec<EndpointAddr> }` | `src/ticket.rs:9-17 @ v0.101.0` | True |
| 63 | `iroh_docs::DocTicket::new` | `pub fn new(capability: Capability, peers: Vec<EndpointAddr>) -> Self` | `src/ticket.rs:49-57 @ v0.101.0` | True |
| 64 | `iroh_docs::Capability` | `#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)] pub enum Capability { Write(NamespaceSecret), Read(NamespaceId) }` | `src/sync.rs:184-191 @ v0.101.0` | True |
| 65 | `iroh_docs::api::protocol::ShareMode` | `#[derive(Debug, Clone, Serialize, Deserialize)] pub enum ShareMode { Read, Write }` | `src/api/protocol.rs:36-43 @ v0.101.0` | True |
| 66 | `iroh_docs::api::protocol::AddrInfoOptions` | `pub enum AddrInfoOptions { Id, RelayAndAddresses, Relay, Addresses }` | `src/api/protocol.rs:368-393 @ v0.101.0` | True |
| 67 | `iroh_docs::api::protocol::StartSyncRequest` | `#[derive(Debug, Serialize, Deserialize, Clone)] pub struct StartSyncRequest { pub doc_id: NamespaceId, pub peers: Vec<EndpointAddr> }` | `src/api/protocol.rs:179-183 @ v0.101.0` | True |
| 68 | `iroh_docs::api::protocol::GetExactRequest` | `#[derive(Debug, Serialize, Deserialize, Clone)] pub struct GetExactRequest { pub doc_id: NamespaceId, pub key: Bytes, pub author: AuthorId, pub include_empty: bool }` | `src/api/protocol.rs:138-144 @ v0.101.0` | True |
| 69 | `iroh_docs::store::Query::all` | `pub fn all() -> QueryBuilder<FlatQuery>` | `src/store.rs:292-296 @ v0.101.0` | True |
| 70 | `iroh_docs::store::Query::key_prefix` | `pub fn key_prefix(prefix: impl AsRef<[u8]>) -> QueryBuilder<FlatQuery>` | `src/store.rs:313-316 @ v0.101.0` | True |
| 71 | `iroh_docs::Replica::insert` | `pub async fn insert(&mut self, key: impl AsRef<[u8]>, author: &Author, hash: Hash, len: u64) -> Result<usize, InsertError>` | `src/sync.rs:375-399 @ v0.101.0` | True |
| 72 | `iroh_docs::Replica::delete_prefix` | `pub async fn delete_prefix(&mut self, prefix: impl AsRef<[u8]>, author: &Author) -> Result<usize, InsertError>` | `src/sync.rs:401-417 @ v0.101.0` | True |
| 73 | `iroh_docs::actor::SyncHandle::insert_local` | `pub async fn insert_local(&self, namespace: NamespaceId, author: AuthorId, key: Bytes, hash: Hash, len: u64) -> Result<()>` | `src/actor.rs:357-375 @ v0.101.0` | True |

**Contract source refs**

- `https://index.crates.io/ir/oh/iroh-docs (curl; exact 0.101.0 record: cksum 8fd1bd5e39d0321a3c4a2bcef9650476c076e2df41a0e84577eca23d6de6c8ab, rust_version 1.91, yanked false, pubtime 2026-06-15T14:33:45Z, features/features2)`
- `https://static.crates.io/crates/iroh-docs/iroh-docs-0.101.0.crate (curl; sha256 verified equal to index cksum)`
- `https://api.github.com/repos/n0-computer/iroh-docs/git/trees/HEAD?recursive=1 (gh api)`
- `https://api.github.com/repos/n0-computer/iroh-docs/git/trees/v0.101.0?recursive=1 (gh api; 59 blobs, truncated=false)`
- `https://api.github.com/repos/n0-computer/iroh-docs/tags?per_page=100 (gh api; v0.101.0 -> 091e8cac47bbc49cdb84b0bfed227cc163b61dfe)`
- `https://api.github.com/repos/n0-computer/iroh-docs/git/tags/39453262e639cb1004d493e4a82f9995bc4bb604 (gh api; annotated tag dereferences to commit 091e8cac47bbc49cdb84b0bfed227cc163b61dfe)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/lib.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/api.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/actor.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/engine.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/api/protocol.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/api/actor.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/keys.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/sync.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/protocol.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/ticket.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/store.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/src/net.rs?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/Cargo.toml?ref=v0.101.0 (gh api; base64 decoded)`
- `https://api.github.com/repos/n0-computer/iroh-docs/contents/README.md?ref=v0.101.0 (gh api; base64 decoded)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/index.html (WebFetch; crate version 0.101.0, modules and re-exports)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html (WebFetch; verbatim signatures)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html (WebFetch; verbatim signatures)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html (WebFetch; verbatim signatures and Deref)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/engine/struct.Engine.html (WebFetch; verbatim signatures)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html (WebFetch; verbatim signatures)`
- `https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/enum.ContentStatus.html (WebFetch; variants)`
- `Decoded v0.101.0 source files (repo path -> absolute local path actually read): Cargo.toml -> /tmp/iroh-docs-research-20261119/blobs/000/Cargo.toml; README.md -> /tmp/iroh-docs-research-20261119/blobs/001/README.md; src/lib.rs -> /tmp/iroh-docs-research-20261119/blobs/005/src__lib.rs; src/api.rs -> /tmp/iroh-docs-research-20261119/blobs/006/src__api.rs; src/api/actor.rs -> /tmp/iroh-docs-research-20261119/blobs/007/src__api__actor.rs; src/api/protocol.rs -> /tmp/iroh-docs-research-20261119/blobs/008/src__api__protocol.rs; src/actor.rs -> /tmp/iroh-docs-research-20261119/blobs/009/src__actor.rs; src/engine.rs -> /tmp/iroh-docs-research-20261119/blobs/010/src__engine.rs; src/engine/live.rs -> /tmp/iroh-docs-research-20261119/blobs/011/src__engine__live.rs; src/engine/state.rs -> /tmp/iroh-docs-research-20261119/blobs/012/src__engine__state.rs; src/store.rs -> /tmp/iroh-docs-research-20261119/blobs/014/src__store.rs; src/keys.rs -> /tmp/iroh-docs-research-20261119/blobs/016/src__keys.rs; src/ticket.rs -> /tmp/iroh-docs-research-20261119/blobs/017/src__ticket.rs; src/net.rs -> /tmp/iroh-docs-research-20261119/blobs/018/src__net.rs; src/protocol.rs -> /tmp/iroh-docs-research-20261119/blobs/019/src__protocol.rs; src/sync.rs -> /tmp/iroh-docs-research-20261119/blobs/021/src__sync.rs; examples/setup.rs -> /tmp/iroh-docs-research-20261119/crate-extracted/iroh-docs-0.101.0/examples/setup.rs; Cargo.toml (published) -> /tmp/iroh-docs-research-20261119/crate-extracted/iroh-docs-0.101.0/Cargo.toml`
- `/tmp/iroh-docs-research-20261119/scripts/manifest.json (local index of decoded v0.101.0 files)`
- `/tmp/iroh-docs-research-20261119/blobs/ (one isolated directory per decoded source file)`
- `/tmp/iroh-docs-research-20261119/crate-extracted/iroh-docs-0.101.0/ (safely extracted published crate; examples/setup.rs and tests/client.rs read)`
- `/tmp/iroh-docs-research-20261119/crate-extracted/iroh-docs-0.101.0/Cargo.toml (published, cargo-normalized; version 0.101.0, rust-version 1.91, license MIT/Apache-2.0)`

### 1.9 `iroh-roq` -- exact version `0.1.0 (only published release; crates.io index pubtime 2025-02-26T13:07:02Z, non-yanked, cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09). git tag v0.1.0 = 389c51e1207d1936103293461bcfc6bb7f5b2691. HEAD main fe00827e5fc0749386fc208d07254725ee16fd7d (2025-06-18) still declares version 0.1.0 but is UNPUBLISHED and has different dependency pins (iroh 0.35 / rtp 0.13 / webrtc-util 0.11).`

- **License:** MIT OR Apache-2.0 (Cargo.toml [package].license; README.md License section; LICENSE-MIT and LICENSE-APACHE both present, Copyright 2025 N0, INC.; README contribution clause re-states dual licensing)
- **MSRV / Go:** Rust 1.81 (rust-version = "1.81" in published and HEAD Cargo.toml; crates.io index rust_version = "1.81"; .github/workflows/ci.yml env MSRV: "1.81" with a dedicated msrv job running cargo +$MSRV check). edition = "2021". No Go component.
- **Signatures:** 24; **unverified items:** 5; **source refs:** 19

**Feature flags / version gates**


**Integration notes (verbatim)**

````text
WHAT IT ACTUALLY IS: iroh-roq is NOT an iroh `ProtocolHandler`. It is a thin RTP-over-QUIC framing shim: one ALPN constant `ALPN: &[u8] = b"/iroh/roq/1"` plus a `Session` that wraps an already-accepted `iroh::endpoint::Connection`. No `impl ProtocolHandler`, no `Router`, no `protocol::` import exists in any of its 5 src files. The internal `Session::run` loop selects on `conn.accept_uni()`, `conn.read_datagram()`, a JoinSet, and a private CancellationToken; it demultiplexes by a QUIC `VarInt` flow id, delivering `Bytes` into per-flow `mpsc::channel(64)` receivers that `ReceiveFlow::read_rtp` unmarshals into `rtp::packet::Packet`. Wire formats (from source): DATAGRAM = `VarInt(flow_id) ++ marshaled RTP`; UNI STREAM = `VarInt(flow_id)` once, lazily on first packet, then per packet `VarInt(packet_len) ++ marshaled RTP`. Marshaling uses `webrtc_util::marshal::{Marshal, MarshalSize}` / `Unmarshal`. Cancellation is via `tokio_util::sync::CancellationToken` child tokens; `SendFlow`/`ReceiveFlow`/`SendStream::close()` take `&self` and just cancel a token.

HOW A SERVICE WIRES IT (verified from src/session.rs tests and tests/opus.rs): build endpoints with `Endpoint::builder().bind_addr_v4("127.0.0.1:0".parse().unwrap()).alpns(vec![ALPN.to_vec()]).bind().await?`; connect with `ep1.connect(ep2.node_addr().await?, ALPN).await?`; on the acceptor side loop `while let Some(incoming) = ep2.accept().await { if let Ok(connection) = incoming.await { assert_eq!(connection.alpn().unwrap(), ALPN, ...); let session = Session::new(connection); ... } }`. Then `session.new_send_flow(VarInt::from_u32(0))` / `session.new_receive_flow(id)` and `send_flow.send_rtp(&packet)` (sync, datagram) or `send_flow.new_send_stream().await?` + `send_stream.send_rtp(&packet).await` (async, stream); receive with `recv_flow.read_rtp().await`. If your service uses `iroh::protocol::Router`, you must write the handler yourself: registering `RouterBuilder::accept(ALPN, H)` where `H::accept(&self, connection: Connection) -> BoxFuture<Result<()>>` just calls `Session::new(connection)` and drives flows. The crate itself does not do this and offers no 0-RTT `on_connecting` hook.

VERSION PIN IS THE BLOCKER: the only published crate, 0.1.0 (crates.io 2025-02-26), requires `iroh = { version = "0.33", default-features = false }` (index `^0.33`, lock 0.33.0, rtp 0.12.0, webrtc-util 0.10.0). Unreleased `main` (fe00827e, 2025-06-18) bumped to `iroh = "0.35"` (lock 0.35.0), rtp 0.13, webrtc-util 0.11, but still declares version 0.1.0 and has never been published -- so `iroh-roq = "0.1"` and a git dependency on main are DIFFERENT dependency surfaces. The current latest non-yanked iroh on index.crates.io is 1.5.0. Since `Session::new` accepts the concrete `iroh::endpoint::Connection` from the crate's own iroh major, a service on iroh 1.x cannot hand its connection to this crate without a port: Cargo would resolve two distinct iroh majors and the types would not match. Either pin the whole service to iroh 0.33/0.35, or vendor/fork and port. Also note `default-features = false` on iroh -- the caller must enable the iroh features it needs separately. No `[features]` block exists in the crate (index `features: {}`), so feature_flags is empty; MSRV Rust 1.81, dual MIT/Apache-2.0.

COULD A MEDIA/DATA-PLANE SERVICE USE IT? Conditionally yes as a prototype or reference implementation, NO as a drop-in production SFU/data plane. Useful: both QUIC DATAGRAM (unreliable, low-latency) and uni-stream (ordered, reliable) RoQ paths; VarInt flow multiplexing; opaque `rtp::Packet` in/out so it is codec-agnostic (Opus appears only as a dev-dependency in tests/opus.rs); async per-flow receive; per-flow cancellation; small surface (5 src files) that is feasible to vendor and port. Missing/unfit for production: no RTCP, no SRTP/key management (encryption is only QUIC/TLS as provided by iroh), no jitter buffer, PLC, NACK, FEC, RTX, congestion/ bitrate adaptation beyond QUIC, no payload-type/SDP negotiation, no simulcast, no SFU routing or mixing; no public flow-discovery/enumeration API (undiscovered incoming flows are held in a private `incoming_flow: Option<ReceiveFlow>` map, but `new_receive_flow(id)` requires the caller to already know the id); no public `Session::close()`/shutdown (session ends on connection loss or when all `Session` clones drop their `Arc<AbortOnDropHandle<()>>`); `SendFlow::close()` leaves the entry in the private `send_flows` map, so re-creating the same id bails `duplicated flow ID`. Concrete robustness concerns read from source: `const RECV_FLOW_BUFFER: usize = 64` and the run loop holds `flows.lock().await` while awaiting `sender.send(bytes).await`, so one slow/unread flow can backpressure and head-of-line-block all other flows; `const MAX_PACKET_SIZE: u64 = 1024 * 1024 * 64; // TODO: what should this be?` followed by `BytesMut::zeroed(len as usize)` on a peer-supplied VarInt permits up to a 64 MiB allocation per packet from an untrusted peer; `s.send(bytes).await.expect("just created")` in the datagram branch is a panic path; the crate logs packet contents at trace/debug in places. README points to the still-draft IETF RoQ spec (draft-ietf-avtcore-rtp-over-quic) and a Go implementation; interop with either was not verified here. Repo has exactly one tag/release and no commits after 2025-06-18, so it reads as an early experiment, not a maintained media stack. Recommendation: use only if both ends are your own code and you accept pinning to iroh 0.35; otherwise vendor it, port `Session::new` to your service's iroh version, add a `ProtocolHandler` wrapper, add flow discovery, cap the peer-controlled allocation, replace the shared-mutex backpressure, and supply jitter/RTCP/security yourself.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `ALPN` | `pub const ALPN: &[u8] = b"/iroh/roq/1";` | `https://github.com/n0-computer/iroh-roq/blob/main/src/lib.rs (gh api contents, base64-decoded); https://github.com/n0-computer/iroh-roq/blob/v0.1.0/src/lib.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/lib.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/constant.ALPN.html` | True |
| 2 | `crate root re-exports (published 0.1.0)` | `pub use self::receive_flow::ReceiveFlow; pub use self::send_flow::SendFlow; pub use self::send_stream::SendStream; pub use self::session::Session; pub use iroh_quinn_proto::VarInt; pub use rtp; pub use rtp::packet::Packet as RtpPacket;` | `https://github.com/n0-computer/iroh-roq/blob/v0.1.0/src/lib.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/lib.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/index.html (Re-exports: rtp; Structs: ReceiveFlow, RtpPacket, SendFlow, SendStream, Session, VarInt)` | True |
| 3 | `crate root re-exports (HEAD main, unpublished)` | `pub use self::{receive_flow::ReceiveFlow, send_flow::SendFlow, send_stream::SendStream, session::Session}; pub use iroh_quinn_proto::VarInt; pub use rtp::{self, packet::Packet as RtpPacket};` | `https://github.com/n0-computer/iroh-roq/blob/main/src/lib.rs (gh api contents at main fe00827e5fc0749386fc208d07254725ee16fd7d)` | True |
| 4 | `Session (struct declaration)` | `#[derive(Debug, Clone)] pub struct Session { /* private: conn: Connection, cancel_token: CancellationToken, send_flows: Arc<Mutex<HashMap<VarInt, SendFlow>>>, receive_flows: Arc<Mutex<HashMap<VarInt, ReceiveFlowSender>>>, _task: Arc<AbortOnDropHandle<()>> */ }` | `https://github.com/n0-computer/iroh-roq/blob/main/src/session.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/session.rs` | True |
| 5 | `Session::new` | `pub fn new(conn: iroh::endpoint::Connection) -> Self` | `https://github.com/n0-computer/iroh-roq/blob/main/src/session.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/session.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.Session.html` | True |
| 6 | `Session::new_send_flow` | `pub async fn new_send_flow(&self, id: VarInt) -> anyhow::Result<SendFlow>` | `https://github.com/n0-computer/iroh-roq/blob/main/src/session.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/session.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.Session.html` | True |
| 7 | `Session::new_receive_flow` | `pub async fn new_receive_flow(&self, id: VarInt) -> anyhow::Result<ReceiveFlow>` | `https://github.com/n0-computer/iroh-roq/blob/main/src/session.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/session.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.Session.html` | True |
| 8 | `SendFlow (struct declaration)` | `#[derive(Clone, Debug)] pub struct SendFlow { /* private: id: VarInt, conn: Connection, cancel_token: CancellationToken */ }` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_flow.rs` | True |
| 9 | `SendFlow::flow_id` | `pub fn flow_id(&self) -> VarInt` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendFlow.html` | True |
| 10 | `SendFlow::send_rtp (QUIC DATAGRAM path)` | `pub fn send_rtp(&self, packet: &RtpPacket) -> anyhow::Result<()>  // marshal: VarInt(flow id) ++ RTP packet bytes -> conn.send_datagram(..)` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendFlow.html (raw HTML code-header: &#96;pub fn send_rtp(&self, packet: &RtpPacket) -> Result<()>&#96;)` | True |
| 11 | `SendFlow::new_send_stream (QUIC uni-stream path)` | `pub async fn new_send_stream(&self) -> anyhow::Result<SendStream>  // conn.open_uni().await` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendFlow.html` | True |
| 12 | `SendFlow::close / SendFlow::is_closed` | `pub fn close(&self); pub fn is_closed(&self) -> bool` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendFlow.html` | True |
| 13 | `ReceiveFlow (struct declaration)` | `#[derive(Debug)] pub struct ReceiveFlow { /* private: id: VarInt, cancel_token: CancellationToken, datagram_receiver: mpsc::Receiver<Bytes> */ }` | `https://github.com/n0-computer/iroh-roq/blob/main/src/receive_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/receive_flow.rs` | True |
| 14 | `ReceiveFlow::flow_id` | `pub fn flow_id(&self) -> VarInt` | `https://github.com/n0-computer/iroh-roq/blob/main/src/receive_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/receive_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.ReceiveFlow.html` | True |
| 15 | `ReceiveFlow::read_rtp` | `pub async fn read_rtp(&mut self) -> anyhow::Result<RtpPacket>  // RtpPacket::unmarshal from Bytes` | `https://github.com/n0-computer/iroh-roq/blob/main/src/receive_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/receive_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.ReceiveFlow.html` | True |
| 16 | `ReceiveFlow::close / ReceiveFlow::is_closed` | `pub fn close(&self); pub fn is_closed(&self) -> bool` | `https://github.com/n0-computer/iroh-roq/blob/main/src/receive_flow.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/receive_flow.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.ReceiveFlow.html` | True |
| 17 | `SendStream (struct declaration)` | `#[derive(Debug)] pub struct SendStream { /* private: id: VarInt, stream: iroh::endpoint::SendStream, cancel_token: CancellationToken, sent_flow_id: bool */ }` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_stream.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_stream.rs` | True |
| 18 | `SendStream::flow_id` | `pub fn flow_id(&self) -> VarInt` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_stream.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_stream.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendStream.html` | True |
| 19 | `SendStream::send_rtp (QUIC uni-stream path)` | `pub async fn send_rtp(&mut self, packet: &RtpPacket) -> anyhow::Result<()>  // first write: VarInt(flow id); every write: VarInt(marshal_size) ++ RTP bytes -> stream.write_all(..).await` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_stream.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_stream.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendStream.html (raw HTML code-header: &#96;pub async fn send_rtp(&mut self, packet: &RtpPacket) -> Result<()>&#96;)` | True |
| 20 | `SendStream::close / SendStream::is_closed` | `pub fn close(&self); pub fn is_closed(&self) -> bool` | `https://github.com/n0-computer/iroh-roq/blob/main/src/send_stream.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/send_stream.rs; https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendStream.html` | True |
| 21 | `flow constructors are NOT public (negative result)` | `SendFlow::new, ReceiveFlow::new, SendStream::new are all &#96;pub(crate) fn new(..) -> Self&#96;. The only public entry point is &#96;Session::new(conn)&#96;; all other constructors are reached via Session / SendFlow methods.` | `https://github.com/n0-computer/iroh-roq/blob/main/src/{send_flow,receive_flow,send_stream}.rs; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/src/{send_flow,receive_flow,send_stream}.rs` | True |
| 22 | `ProtocolHandler implementation ABSENT (negative result)` | `There is NO &#96;impl iroh::protocol::ProtocolHandler&#96; and no &#96;iroh::protocol::*&#96; / &#96;iroh::protocol::Router&#96; usage anywhere in iroh-roq src. The crate exports only the ALPN byte string; the caller must do its own accept loop or write its own handler. iroh 0.35's trait (read for context) is: &#96;pub trait ProtocolHandler: Send + Sync + std::fmt::Debug + 'static { fn on_connecting(&self, connecting: Connecting) -> BoxFuture<Result<Connection>> {..} fn accept(&self, connection: Connection) -> BoxFuture<Result<()>>; fn shutdown(&self) -> BoxFuture<()> {..} }&#96;, registered with &#96;RouterBuilder::accept(alpn, handler)&#96;.` | `gh api 'repos/n0-computer/iroh-roq/git/trees/HEAD?recursive=1' (only 5 src files); https://github.com/n0-computer/iroh-roq/blob/main/src/{lib,session,send_flow,receive_flow,send_stream}.rs (grep for ProtocolHandler\|Router\|protocol:: found none); https://github.com/n0-computer/iroh/blob/v0.35.0/iroh/src/protocol.rs lines 106-132, 244` | True |
| 23 | `iroh version pin (published 0.1.0)` | `iroh = { version = "0.33", default-features = false }  // crates.io index req "^0.33"; published Cargo.lock resolves iroh 0.33.0, iroh-quinn-proto 0.13.0, rtp 0.12.0, webrtc-util 0.10.0, n0-future 0.1.2, tokio 1.41.1, tokio-util 0.7.12, anyhow 1.0.93, tracing 0.1.41` | `https://index.crates.io/ir/oh/iroh-roq; https://github.com/n0-computer/iroh-roq/blob/v0.1.0/Cargo.toml; static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate (downloaded, verified cksum 8ba28cc2a62f5f2ce520c60563d6868d75c3e29742e73d1c56ccd5ec2d830a09)/Cargo.toml.orig and /Cargo.lock` | True |
| 24 | `iroh version pin (HEAD main, unpublished)` | `iroh = { version = "0.35", default-features = false }  // HEAD Cargo.lock resolves iroh 0.35.0; also bumps rtp = "0.13", webrtc-util = "0.11", anyhow "1.0.98", tokio "1.45.1", tokio-util "0.7.15", n0-future "0.1.3"` | `https://github.com/n0-computer/iroh-roq/blob/main/Cargo.toml; gh api repos/n0-computer/iroh-roq/contents/Cargo.lock (grep &#96;name = "iroh"&#96; -> 0.35.0); https://index.crates.io/ir/oh/iroh (only 0.35.0 published in the 0.35 line)` | True |

**Contract source refs**

- `gh api repos/n0-computer/iroh-roq (metadata: main, non-archived, description 'RTP over QUIC implemented as iroh protocol')`
- `gh api 'repos/n0-computer/iroh-roq/git/trees/HEAD?recursive=1' (complete file list: .github/workflows/ci.yml, .gitignore, Cargo.lock, Cargo.toml, LICENSE-APACHE, LICENSE-MIT, Makefile.toml, README.md, deny.toml, src/lib.rs, src/receive_flow.rs, src/send_flow.rs, src/send_stream.rs, src/session.rs, tests/opus.rs)`
- `gh api repos/n0-computer/iroh-roq/contents/{Cargo.toml,README.md,src/lib.rs,src/receive_flow.rs,src/send_flow.rs,src/send_stream.rs,src/session.rs,tests/opus.rs,Cargo.lock,.github/workflows/ci.yml,LICENSE-MIT,LICENSE-APACHE} --jq .content | base64 -D`
- `gh api 'repos/n0-computer/iroh-roq/contents/Cargo.toml?ref=v0.1.0' (base64-decoded; matches published Cargo.toml.orig)`
- `gh api 'repos/n0-computer/iroh-roq/tags?per_page=100' -> only tag v0.1.0 = 389c51e1207d1936103293461bcfc6bb7f5b2691`
- `gh api repos/n0-computer/iroh-roq/commits/main -> fe00827e5fc0749386fc208d07254725ee16fd7d 2025-06-18T16:56:25Z 'chore: try and make deny happier'`
- `gh api 'repos/n0-computer/iroh-roq/compare/v0.1.0...main' -> ahead_by 4 (feat: ci, dial down logging, rtp crate type mismatch fix, deny happier)`
- `https://index.crates.io/ir/oh/iroh-roq (one version: 0.1.0, full deps/features/cksum/pubtime/rust_version)`
- `https://index.crates.io/ir/oh/iroh (0.35.0 published, not yanked, rust_version 1.81; latest non-yanked stable 1.5.0)`
- `https://static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate -> /Users/dylanwong/projects/aws0-deps-2026-11-19/probe/iroh-roq-0.1.0-crate/iroh-roq-0.1.0/ (Cargo.toml, Cargo.toml.orig, Cargo.lock, README.md, src/*.rs, tests/opus.rs, LICENSE-*)`
- `https://docs.rs/iroh-roq/0.1.0/iroh_roq/ (index and all.html item list)`
- `https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.Session.html`
- `https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendFlow.html`
- `https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.ReceiveFlow.html`
- `https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendStream.html`
- `https://docs.rs/iroh-roq/0.1.0/iroh_roq/constant.ALPN.html`
- `https://docs.rs/crate/iroh-roq/0.1.0/source/src/lib.rs`
- `https://github.com/n0-computer/iroh/blob/v0.35.0/iroh/src/protocol.rs (ProtocolHandler trait, RouterBuilder::accept) via gh api contents?ref=v0.35.0`
- `https://github.com/n0-computer/iroh-roq/blob/main/README.md (RoQ draft link, Go impl link, licensing)`

### 1.10 `ministack` -- exact version `1.5.43`

- **License:** MIT (LICENSE: 'MIT License', Copyright (c) 2026 MiniStack Contributors; GitHub repo license.spdx_id=MIT; PyPI info.license='MIT License')
- **MSRV / Go:** No Rust MSRV / no Go version: Python package with requires-python >=3.10 (pyproject.toml:11, PyPI info.requires_python); classifiers 3.10-3.13, CI uses 3.13, Docker images use python:3.13-alpine (light) and python:3.13-slim (full).
- **Signatures:** 42; **unverified items:** 7; **source refs:** 19

**Feature flags / version gates**

- `full`
- `test`
- `dev`
- `AUTH`
- `USE_SSL`
- `MINISTACK_EDITION`
- `GATEWAY_PORT`
- `EDGE_PORT`
- `BIND_HOST`
- `MINISTACK_HOST`
- `DOCKER_HOST`
- `DOCKER_NETWORK`
- `MINISTACK_IMAGE_PREFIX`
- `MINISTACK_DOCKER_TIMEOUT`
- `RDS_PERSIST`
- `RDS_BASE_PORT`
- `RDS_TMPFS_SIZE`
- `MINISTACK_RDS_PUBLIC_ENDPOINT`
- `MINISTACK_RDS_PG_CLUSTER_REPLICATION`
- `DSQL_STRICT`
- `DSQL_BASE_PORT`
- `DSQL_PERSIST`
- `DSQL_PG_IMAGE`
- `DOCDB_BASE_PORT`
- `DOCDB_PERSIST`
- `DOCDB_TMPFS_SIZE`
- `ELASTICACHE_BASE_PORT`
- `ELASTICACHE_CLUSTER_MODE_REAL`
- `REDIS_HOST`
- `REDIS_PORT`
- `OPENSEARCH_DATAPLANE`
- `OPENSEARCH_BASE_PORT`
- `OPENSEARCH_IMAGE`
- `OPENSEARCH_DASHBOARDS`
- `OPENSEARCH_DASHBOARDS_BASE_PORT`
- `OPENSEARCH_DASHBOARDS_IMAGE`
- `MINISTACK_OPENSEARCH_ENDPOINT`
- `LAMBDA_EXECUTOR`
- `LAMBDA_STRICT`
- `LAMBDA_DOCKER_FLAGS`
- `LAMBDA_WARM_TTL_SECONDS`
- `LAMBDA_ACCOUNT_CONCURRENCY`
- `CODEBUILD_DOCKER_FLAGS`
- `MINISTACK_CODEBUILD_EXECUTE`
- `ATHENA_ENGINE`
- `GLUE_DOCKER_IMAGE`
- `EKS_K3S_IMAGE`
- `MWAA_AIRFLOW_IMAGE`
- `PERSIST_STATE`
- `STATE_DIR`
- `S3_PERSIST`
- `S3_DATA_DIR`
- `SFTP_PORT`
- `SFTP_PORT_PER_SERVER`
- `IOT_MTLS_ENABLED`
- `IOT_MTLS_PORT`
- `MINISTACK_IMDS_V2_REQUIRED`
- `MINISTACK_BEDROCK_PROXY_URL`
- `MINISTACK_MSK_BOOTSTRAP`
- `SMTP_HOST`
- `TRANSLATE_JOB_RUN_SECONDS`

**Integration notes (verbatim)**

````text
PROVENANCE / EXACT VERSION. Registry of record is PyPI, not crates.io: curl https://index.crates.io/mi/ni/ministack returned HTTP 404 (no Rust crate, so no docs.rs signature set exists for this dependency). curl https://pypi.org/pypi/ministack/1.5.43/json gives version 1.5.43, requires_python >=3.10, license 'MIT License', wheel ministack-1.5.43-py3-none-any.whl uploaded 2026-11-19T21:00:24Z. GitHub release v1.5.43 published 2026-11-19T21:00:22Z; HEAD of main is 5080d4b7141e7030e8cc865a4832e8d0c9870e0c (2026-11-19T21:47:51Z), and pyproject.toml:7 declares version = "1.5.43", matching the published artifact. All source was fetched with `gh api repos/ministackorg/ministack/contents/<path>` and base64-decoded; the full file list came from `gh api 'repos/ministackorg/ministack/git/trees/HEAD?recursive=1'`.

GATEWAY / PACKAGING / DX. MiniStack is a single-port ASGI app (`ministack.app:app`) served by Hypercorn on port 4566; `_resolve_port()` resolves GATEWAY_PORT > EDGE_PORT > 4566, and BIND_HOST defaults to 0.0.0.0 (AGENTS.md recommends `python -m hypercorn ministack.app:app --bind 0.0.0.0:4566`). Docker images EXPOSE 4566 2222, set ENTRYPOINT ["python", "-m", "ministack"], and ship a pure-Python HEALTHCHECK that GETs /_ministack/health on GATEWAY_PORT or EDGE_PORT with USE_SSL-aware http/https. `pip install ministack` installs the `ministack` console script (`ministack.app:main`, with -d/--detach and --stop); `ministack[full]` adds duckdb, docker, cryptography, psycopg2-binary, pymysql, asyncssh, aws-sam-translator. Dockerfile builds the light Alpine/python:3.13 image (~110 MB per README) from a full-image plugin donor; Dockerfile.full builds the Debian/glibc python:3.13-slim variant with DuckDB and native PG/MySQL drivers and sets MINISTACK_EDITION=full (health reports edition light/full). docker-compose.yml mounts ./data/s3 and /var/run/docker.sock, publishes 4566:4566, depends_on a healthy redis:7-alpine sidecar at 127.0.0.1:6379, and healthchecks /_ministack/health. `bin/awslocal` wraps the bundled AWS CLI with --endpoint-url and dummy test/test keys. LocalStack-compatible init-script volumes (/docker-entrypoint-initaws.d, /etc/localstack/init/{boot.d,ready.d}) are declared as VOLUMEs and both phases are merged, deduplicated by filename, run alphabetically, with AWS_* and MINISTACK_INIT_* env injected. Testcontainers examples ship for Python/Java/Go/.NET; the Python one polls /_ministack/health for a 30s readiness window.

BOOT SEQUENCE (app.py:3705-3765). ASGI lifespan.startup: set a 64-worker default executor (MINISTACK_WORKER_THREADS), probe the Docker socket (installing a Docker Hub fallback when present), run boot.d scripts, start the periodic container_reaper, reap orphaned MiniStack containers with a 10s boot deadline (never blocking the gateway bind), load PERSIST_STATE snapshots, start the Transfer Family SFTP (:2222) and IoT mTLS (:8883) listeners, then run ready.d scripts; GET /_ministack/ready returns 503 until they finish. Every registered service module implements get_state()/load_persisted_state(data)/reset(), enforced by the registry and tests/test_persistence.py. SERVICE_REGISTRY has 91 entries (counted by ast.literal_eval on app.py:435-541).

DOCKER SOCKET USAGE. Docker is opt-in and strictly capability-detected: `-v /var/run/docker.sock:/var/run/docker.sock` (README Quick Start option 2b, docker-compose.yml:20-22, lambda/README:1097, EKS/README:1256). `DOCKER_HOST` overrides the default `unix:///var/run/docker.sock`; `_docker_socket_present()` and dsql.py `_docker_available()` skip importing docker-py when the socket is absent, so each service degrades to a metadata-only stub (EKS falls back to a static mock; DSQL default DSQL_STRICT=0 is control-plane only; OpenSearch default OPENSEARCH_DATAPLANE=0 returns a stub endpoint). Clients are `docker.from_env(timeout=_DOCKER_REAP_TIMEOUT)` / per-service `docker.from_env(timeout=_DOCKER_TIMEOUT)` with MINISTACK_DOCKER_TIMEOUT default 10s; container creation is `containers.run(**run_kwargs)` or `create`+`start`, retried on host-port collisions via container_reaper.run_on_free_port. Every spawned container is stamped by `own_labels(service, **extra)` with `ministack=<service>`, `ministack.instance`, `ministack.boot` plus inherited Testcontainers/Ryuk labels so Ryuk cleans up nested containers when the test JVM dies; reap_all at boot/shutdown removes all of this instance's containers and non-persisted named volumes, while reap_abandoned is the conservative live-process pass. DOCKER_NETWORK attaches nested containers to the Compose network; MINISTACK_IMAGE_PREFIX rewrites all nested image refs for mirrored registries; host.docker.internal:host-gateway is added so nested containers can call back into the gateway. EKS k3s is launched --privileged (README states the security trade-off); CodeBuild's local agent gets /var/run/docker.sock bind-mounted rw.

SERVICE -> REAL ENGINE MAP (source-verified image literals). RDS CreateDBInstance/CreateDBCluster: postgres:{major}-alpine (:5432) for postgres/aurora-postgresql, mysql:{5.6|5.7|8.0|8.4} (:3306) for mysql/aurora-mysql, mariadb:latest (:3306); host ports from RDS_BASE_PORT=15432; supports RDS_PERSIST named volumes vs tmpfs, IAM auth plugin compiled for MySQL 8.0/8.4 in Dockerfile.full, Data API via pymysql/psycopg2. ElastiCache CreateCacheCluster/CreateServerlessCache: redis:<major>-alpine, valkey/valkey:<7.2|8.0|8.1>-alpine (:6379), memcached:<version>-alpine (:11211), ports from ELASTICACHE_BASE_PORT=16379, TLS serverless CA at /_ministack/elasticache/ca.pem; a redis:7-alpine sidecar is always present under Compose. DocumentDB: ghcr.io/documentdb/documentdb/documentdb-local:latest (:27017, DOCDB_BASE_PORT=27117) speaking MongoDB over TLS. Aurora DSQL: DSQL_STRICT=1 starts postgres:16-alpine per cluster (DSQL_PG_IMAGE), capped at 30 ports from DSQL_BASE_PORT=25432, behind a validating in-process Postgres wire proxy. OpenSearch: OPENSEARCH_DATAPLANE=1 starts opensearchproject/opensearch:2.15.0 per domain/collection (OPENSEARCH_BASE_PORT=14571) and optional opensearchproject/opensearch-dashboards:2.15.0 (:15601). EKS: rancher/k3s:v1.31.4-k3s1 (privileged) giving a real Kubernetes API server. ECS RunTask and EC2 RunInstances (when an AMI is registered from a container reference) run user-supplied images. Lambda: public.ecr.aws/lambda/{python,nodejs,java,dotnet,ruby,provided}:<version> per _RUNTIME_IMAGE_MAP, or local subprocess/warm pool by default. Glue Spark jobs: amazon/aws-glue-libs:{5, glue_libs_4.0.0_image_01, glue_libs_3.0.0_image_01}. Managed Flink: flink:1.19.1-java11 / flink:1.20.5-java11 JobManager+TaskManager. CodeBuild: public.ecr.aws/codebuild/local-builds:latest with MINISTACK_CODEBUILD_EXECUTE=1. MWAA: apache/airflow:3.0.6. Athena: in-process DuckDB (full image; `SELECT 1+1` returns 2, the light image's mock returns 1). Bedrock proxies to an OpenAI-compatible MINISTACK_BEDROCK_PROXY_URL (Ollama/llama.cpp/vLLM). MSK is BYO-broker via MINISTACK_MSK_BOOTSTRAP; AmazonMQ explicitly has no real container support. Transfer Family runs a real SFTP server on :2222 backed by local S3.

HEALTH / RESET / INSPECTION ENDPOINTS. GET /_ministack/health returns {services (map to "available"), auth (enabled/disabled), edition (light/full), version, iot_mtls, ready_scripts}; /_localstack/health and /health are aliases; GET /_ministack/ready gates on ready.d + mTLS startup. POST /_ministack/reset wipes every registered module's state, persisted STATE_DIR/*.json, and S3_PERSIST data, returns {"reset": "ok"}, and is serialized under an asyncio reset lock that blocks other requests mid-reset; POST /_ministack/reset?init=1 additionally re-runs boot.d/ready.d (the CI-recommended setUp/beforeEach pattern; tests/conftest.py coordinates a single reset across xdist workers). POST /_ministack/config applies a strict whitelist of module.VAR runtime settings and returns {"applied": {...}}. Inspection endpoints: /_ministack/ses/messages, /_ministack/sqs/messages, /_ministack/sns/sms-messages (filterable by 12-digit account, phoneNumber, QueueUrl, region), plus CA/key endpoints /_ministack/{elasticache,rds,iot}/ca.pem, /_ministack/elb/public-keys/<kid>, /_ministack/transfer/sftp-ports.

MULTI-ACCOUNT / MULTI-REGION STATE SCOPING. No config needed: a 12-digit numeric AWS_ACCESS_KEY_ID becomes the account ID (`_12_DIGIT_RE = re.compile(r"^\d{12}$")` in responses.py; non-numeric keys -> MINISTACK_ACCOUNT_ID or "000000000000"; ASIA* session keys resolve to the assumed-role ARN's account). The region comes from the SigV4 `Credential=<akid>/<date>/<region>/<service>/aws4_request` scope (or presigned X-Amz-Credential / SigV4a X-Amz-Region-Set), extracted by router.extract_signed_region and falling back to MINISTACK_REGION/us-east-1; app.py:3577-3605 pins both into contextvars for each request. Stores use AccountScopedDict (account, key) and AccountRegionScopedDict (account, region, key) so identical resource names in two accounts/regions never collide; request_scope(account_id, region) pins scope for background/cross-service work; cross-resource references resolve in the referenced ARN's own account and region. Per README:224-228, since 1.5.3 every regional service is region-isolated except S3 and Aurora DSQL; IAM/STS, CloudFront, Route 53 and Organizations are correctly global. PERSIST_STATE=1 writes atomic, version-stamped (v2/v3) state to STATE_DIR and migrates legacy records by recovering region from stored ARNs; a newer-format file is refused rather than mis-parsed; /_ministack/reset deletes all tenants' persisted state, not just the current request's.

DX BENCHMARK IMPLICATIONS. To match or exceed MiniStack, a substrate needs: one gateway port with AWS-SDK-compatible SigV4 routing; real engine containers behind a detected, optional Docker socket with graceful metadata-only fallback and Ryuk/Testcontainers label inheritance; a reset endpoint that wipes all tenants plus an init=1 re-seed path for CI; a health endpoint plus a readiness endpoint distinct from health (init scripts and secondary listeners); per-request account+region isolation with 12-digit access keys and SigV4 credential-scope regions; LocalStack-compatible health aliases, init-script paths and awslocal; light/full packaging with clear edition reporting; and four-language Testcontainers examples. Explicitly unverified in this session: no Docker daemon was contacted and no container was actually started -- engine boot behaviour is established from source literals and README, not a live witness; README marketing figures (~270 MB image, ~30 MB idle RAM, <2 s startup) were not measured; Docker Hub tag digests were not read from a registry manifest; mq.py, msk.py, opensearchserverless.py internals were fetched/grepped only partially and are not asserted here beyond the README statements; docs.rs is inapplicable (Python package, crates.io 404).
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `GET /_ministack/health` | `GET /_ministack/health -> 200 application/json {"services": {<name>: "available", ...}, "auth": "enabled"\|"disabled", "edition": "light"\|"full", "version": str, "iot_mtls": <state>, "ready_scripts": {"status","total","completed","failed"}}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:945-965; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:76-77,133-138` | True |
| 2 | `GET /_localstack/health and GET /health (LocalStack aliases)` | `_HEALTH_PATHS = ("/_ministack/health", "/_localstack/health", "/health")` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:169; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:133-138; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/tests_test_health.py:15-30` | True |
| 3 | `GET /_ministack/ready` | `GET /_ministack/ready -> 200 {"status","total","completed","failed","iot_mtls"} when ready_scripts.status=="completed" and iot_mtls!="starting", else 503` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:968-986` | True |
| 4 | `POST /_ministack/reset` | `POST /_ministack/reset -> 200 {"reset": "ok"}; POST /_ministack/reset?init=1 also re-runs boot.d + ready.d init scripts` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:1078-1091,4090-4138; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:79-83,110; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/tests_conftest.py:583-612` | True |
| 5 | `POST /_ministack/config` | `POST /_ministack/config {"<module>.<VAR>": value} -> 200 {"applied": {...}}; whitelist = athena.ATHENA_ENGINE, athena.ATHENA_DATA_DIR, stepfunctions._sfn_mock_config, stepfunctions._SFN_WAIT_SCALE, translate._JOB_RUN_SECONDS, transcribe._JOB_RUN_SECONDS, lambda_svc.LAMBDA_EXECUTOR, cloudtrail._recording_enabled, alb.TARGET_CONNECT_TIMEOUT, alb.TARGET_IDLE_TIMEOUT` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:1558-1613; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:85-88,112-120` | True |
| 6 | `GET /_ministack/ses/messages, /_ministack/sqs/messages, /_ministack/sns/sms-messages` | `GET /_ministack/ses/messages?account=<12-digit>; GET /_ministack/sqs/messages?account=<12-digit>&QueueUrl=<url>; GET /_ministack/sns/sms-messages?phoneNumber=...&account=...&region=... -> grouped JSON` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:1094-1097,1178-1180,1273-1276; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:90-107` | True |
| 7 | `GET /_ministack/{elasticache,rds,iot}/ca.pem, /_ministack/elb/public-keys/<kid>, /_ministack/transfer/sftp-ports` | `GET /_ministack/elasticache/ca.pem \| /_ministack/rds/ca.pem \| /_ministack/iot/ca.pem \| /_ministack/elb/public-keys/<kid> \| /_ministack/transfer/sftp-ports -> PEM / JSON` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:1415-1418,1433-1437,1461-1464,1483-1489,1517-1526; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:766-776` | True |
| 8 | `ministack.app:app (ASGI application)` | `async def app(scope, receive, send)  # served by hypercorn on 0.0.0.0:4566; python -m hypercorn ministack.app:app --bind 0.0.0.0:4566` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:3462,4145-4150,4195-4206; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/AGENTS.md:59-61; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/pyproject.toml:94-95` | True |
| 9 | `ministack.app:main (console script)` | `def main():  # argparse -d/--detach and --stop; bind_host=BIND_HOST or 0.0.0.0; port=_resolve_port()` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:4145-4158,4250-4262; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/pyproject.toml:94-95` | True |
| 10 | `ministack.app:_resolve_port (gateway port)` | `def _resolve_port(): return os.environ.get("GATEWAY_PORT") or os.environ.get("EDGE_PORT") or "4566"` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:5,30-31,579-581; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile:80,98; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/docker-compose.yml:6-10; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:45,899` | True |
| 11 | `Dockerfile / Dockerfile.full image contract` | `ENV GATEWAY_PORT=4566 ...; EXPOSE 4566 2222; HEALTHCHECK ... GET /_ministack/health; ENTRYPOINT ["python", "-m", "ministack"]; Dockerfile.full adds ENV MINISTACK_EDITION=full` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile:78-104; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile.full:209-236` | True |
| 12 | `docker-compose.yml docker socket mount` | `volumes: - ./data/s3:/tmp/ministack-data/s3; - /var/run/docker.sock:/var/run/docker.sock; healthcheck GET http://localhost:4566/_ministack/health; depends_on redis:7-alpine (service_healthy)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/docker-compose.yml:1-44; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:50-51,1077,1097,1256` | True |
| 13 | `pyproject.toml packaging` | `[project] name="ministack"; version="1.5.43"; requires-python=">=3.10"; dependencies=[hypercorn>=0.18.0, pyyaml>=6.0, defusedxml>=0.7, botocore==1.43.124, jsonata-python>=0.7.0, graphql-core==3.2.12]; [project.scripts] ministack = "ministack.app:main"; [project.optional-dependencies] full / test / dev` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/pyproject.toml:1-99; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/requirements.txt:1-6` | True |
| 14 | `bin/awslocal CLI wrapper` | `ENDPOINT_URL="${MINISTACK_ENDPOINT:-http://localhost:4566}"; exec aws --endpoint-url="$ENDPOINT_URL" "$@"` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/bin_awslocal:7-27; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile:63-64` | True |
| 15 | `SERVICE_REGISTRY + per-service state contract` | `SERVICE_REGISTRY = {...}  # dict, 91 entries (ast.literal_eval count); values {"module": str, optional "aliases": tuple, optional "sub_modules": tuple}; every registered module must define get_state(), load_persisted_state(data), reset()` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:435-541,4090-4111; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/AGENTS.md:41-50` | True |
| 16 | `ministack.core.responses.set_request_account_id` | `def set_request_account_id(access_key_id: str) -> None  # _12_DIGIT_RE = re.compile(r"^\d{12}$"); 12-digit key becomes account, else STS-session ARN account, else MINISTACK_ACCOUNT_ID or "000000000000"` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_responses.py:23-30,33-75; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:142-169; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/tests_test_multitenancy.py:34-60` | True |
| 17 | `ministack.core.responses.get_account_id` | `def get_account_id() -> str  # returns _request_account_id ContextVar` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_responses.py:78-80; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_router.py:1477-1482` | True |
| 18 | `ministack.core.responses.request-scoped region` | `def set_request_region(region: str \| None) -> None; def get_region() -> str  # default MINISTACK_REGION or "us-east-1"` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_responses.py:83-103` | True |
| 19 | `ministack.core.responses.request_scope` | `@contextlib.contextmanager def request_scope(account_id: str, region: str)  # pins both ContextVars for cross-service/background work` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_responses.py:123-139` | True |
| 20 | `ministack.core.responses.AccountRegionScopedDict` | `class AccountRegionScopedDict: def _scoped(self, key): return (get_account_id(), get_region(), key); def get_scoped/set_scoped/contains_scoped/pop_scoped(self, account_id, region, key, ...)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_responses.py:257-280,315-346; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:192-230` | True |
| 21 | `ministack.core.router.extract_signed_region` | `def extract_signed_region(headers: dict, query_params=None) -> str \| None  # SigV4 Credential=[^/]+/\d{8}/([^/]+)/[^/]+/aws4_request; also presigned X-Amz-Credential and SigV4a X-Amz-Region-Set` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_router.py:1389-1442; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:192-205` | True |
| 22 | `ministack.core.router.extract_region / extract_access_key_id / detect_service` | `def extract_region(headers: dict, query_params=None) -> str; def extract_access_key_id(headers: dict, query_params=None) -> str; def detect_service(method: str, path: str, headers: dict, query_params: dict) -> str` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_router.py:702,1445-1474; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:1346-1359` | True |
| 23 | `app.py per-request tenant wiring` | `set_request_account_id(_request_account_scope(_access_key) if AUTH else _access_key); set_request_region(extract_region(headers, query_params))` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:3577-3605; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:192-230` | True |
| 24 | `ministack.app:_docker_socket_present / _reaper_docker_client` | `def _docker_socket_present() -> bool  # sock = os.environ.get("DOCKER_HOST") or "unix:///var/run/docker.sock"; def _reaper_docker_client(): return docker.from_env(timeout=_DOCKER_REAP_TIMEOUT) or None` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:3870-3890; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_dsql.py:197-208` | True |
| 25 | `ministack.app:_handle_lifespan boot order` | `async def _handle_lifespan(scope, receive, send)  # 64-thread executor -> docker-socket check -> boot.d scripts -> container_reaper.start(_reaper_docker_client) -> boot orphan reap (10s deadline) -> PERSIST_STATE load -> SFTP/mTLS listeners -> ready.d scripts` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:3705-3765,3870-3914,3968-4058; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:985-1030` | True |
| 26 | `ministack.core.container_reaper ownership API` | `def own_labels(service: str, **extra) -> dict  # {"ministack": service, "ministack.instance": id, "ministack.boot": nonce, + inherited Ryuk labels}; def register_live_ids(label: str, provider) -> None; def start(get_docker) -> None; def reap_all(docker_client, stop_timeout: int = 2, include_unlabelled: bool = False) -> int; def reap_volumes(docker_client, service: str) -> int` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_container_reaper.py:50-51,73-89,146-170,268,286-299,348-354` | True |
| 27 | `ministack.services.rds._docker_image_for_engine (RDS -> real DB)` | `def _docker_image_for_engine(engine, engine_version, user, password, db_name): -> (image, env, container_port, data_path)  # postgres/aurora-postgresql -> postgres:<major>-alpine:5432; mysql/aurora-mysql AURORA_MYSQL_IMAGE_MAP {5.6:mysql:5.6,5.7:mysql:5.7,8.0:mysql:8.0,8.4:mysql:8.4}:3306; mariadb -> mariadb:latest:3306` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_rds.py:506-511,11588-11660,1356-1398,2795; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:638-640,677-709; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile:86,89` | True |
| 28 | `ministack.services.elasticache._engine_image_and_port (ElastiCache -> real cache)` | `def _engine_image_and_port(engine, engine_version): -> (image, port)  # valkey -> valkey/valkey:<major.minor>-alpine:6379; redis -> redis:<major>-alpine:6379; else memcached:<version>-alpine:11211` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_elasticache.py:525-538,640-645,2633; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:640-642,740-776; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/docker-compose.yml:34-44` | True |
| 29 | `ministack.services.documentdb._docker_image_for_docdb` | `def _docker_image_for_docdb(engine_version, user, password, db_name=""): -> ("ghcr.io/documentdb/documentdb/documentdb-local:latest", {"USERNAME","PASSWORD","DOCUMENTDB_PORT":"27017"}, 27017, "/data")` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_documentdb.py:3,733-746; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:640,919-921` | True |
| 30 | `ministack.services.dsql real Postgres backend` | `PG_IMAGE = os.environ.get("DSQL_PG_IMAGE", "postgres:16-alpine"); _BACKEND_PORT_WINDOW = 30; DSQL_STRICT = os.environ.get("DSQL_STRICT", "0").lower() in ("1","true","yes"); def _backends_enabled(): return DSQL_STRICT and _docker_available(); endpoint localhost:<DSQL_BASE_PORT+n> (25432)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_dsql.py:43-55,197-208,293; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:641,724-736,916-918` | True |
| 31 | `ministack.services.opensearch real OpenSearch` | `DATAPLANE_ENABLED = os.environ.get("OPENSEARCH_DATAPLANE","0") == "1"; BASE_PORT = int(os.environ.get("OPENSEARCH_BASE_PORT","14571")); DEFAULT_IMAGE = os.environ.get("OPENSEARCH_IMAGE","opensearchproject/opensearch:2.15.0"); DASHBOARDS_IMAGE = os.environ.get("OPENSEARCH_DASHBOARDS_IMAGE","opensearchproject/opensearch-dashboards:2.15.0")` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_opensearch.py:54-63,361,449,533; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:667-668,926-932` | True |
| 32 | `ministack.services.eks real k3s` | `EKS_K3S_IMAGE = os.environ.get("EKS_K3S_IMAGE", "rancher/k3s:v1.31.4-k3s1")  # CreateCluster -> _k3s_run_kwargs(privileged=True) -> client.containers.create/run; without Docker falls back to static EKS mock` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_eks.py:7-18,73,438-499,603-640,783-813; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:666,1177-1258` | True |
| 33 | `ministack.services.ecs RunTask real Docker containers` | `RunTask -> docker.from_env(timeout=_DOCKER_TIMEOUT) -> docker_client.containers.run(create_image, **run_kwargs)  # injects ECS_CONTAINER_METADATA_URI_V4, AWS_CONTAINER_CREDENTIALS_FULL_URI, AWS_ENDPOINT_URL; StopTask removes container` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_ecs.py:338,1819,2427,2459; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:638,841-886` | True |
| 34 | `ministack.services.lambda_svc runtime images` | `LAMBDA_EXECUTOR = os.environ.get("LAMBDA_EXECUTOR", "local").lower(); LAMBDA_STRICT = os.environ.get("LAMBDA_STRICT","0").lower() in ("1","true","yes"); _RUNTIME_IMAGE_MAP: dict[str, str] = {"python3.8": "public.ecr.aws/lambda/python:3.8", ..., "provided.al2023": "public.ecr.aws/lambda/provided:al2023"}` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_lambda_svc.py:257-269,3418-3448,5051-5057; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:935-938,1061-1079` | True |
| 35 | `ministack.services.glue / kinesisanalyticsv2_flink / codebuild / mwaa images` | `_GLUE_VERSION_IMAGES = {"5.0":"amazon/aws-glue-libs:5","4.0":"amazon/aws-glue-libs:glue_libs_4.0.0_image_01","3.0":"amazon/aws-glue-libs:glue_libs_3.0.0_image_01"}; _RUNTIME_IMAGES = {"FLINK-1_20":"flink:1.20.5-java11","FLINK-1_19":"flink:1.19.1-java11"}; AGENT_IMAGE = "public.ecr.aws/codebuild/local-builds:latest"; DEFAULT_AIRFLOW_IMAGE = os.environ.get("MWAA_AIRFLOW_IMAGE", "apache/airflow:3.0.6")` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_glue.py:61-66,2936; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_kinesisanalyticsv2_flink.py:29-31,422,490,504; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_codebuild.py:233-241,471,490; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_mwaa.py:46,337,416; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:643,659,889-891` | True |
| 36 | `ministack.services.ec2 RegisterImage container-backed instances` | `RegisterImage(ImageLocation=<container ref>) -> RunInstances -> client.containers.run(ref, **kwargs)  # instance-store backed, no EBS root; Stop/Start refused; unregistered AMIs stay metadata-only` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_ec2.py:2196,2528-2542,2652; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:648,1113-1175` | True |
| 37 | `ministack.services.athena DuckDB engine` | `ATHENA_ENGINE = os.environ.get("ATHENA_ENGINE", "auto")  # "auto"\|"duckdb"\|"mock"; async def _run_duckdb(query, database, catalog=None):  # duckdb.connect(":memory:") inside` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_athena.py:6,50,72,225-229,928-971; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:645,781-813,1032-1045,944` | True |
| 38 | `ministack.services.rds_data real SQL drivers` | `def _connect(instance, engine, database=None, password=None, username=None):  # pymysql.connect(...) for MySQL/Aurora MySQL, psycopg2.connect(...) for PostgreSQL/Aurora PostgreSQL, against Docker-backed RDS endpoints` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_rds_data.py:455-502; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:656; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:169,1872-1883` | True |
| 39 | `ministack.services.transfer real SFTP listener` | `def _port_per_server() -> bool; def _shared_port() -> int  # int(os.environ.get("SFTP_PORT", "2222")); SFTP_PORT_PER_SERVER=1 allocates per-server ports from SFTP_BASE_PORT (2300); GET /_ministack/transfer/sftp-ports` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_services_transfer.py:14-15,557-561,1261; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:661; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile:98; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:1517-1526` | True |
| 40 | `ministack.core.persistence state API` | `PERSIST_STATE = os.environ.get("PERSIST_STATE","0") == "1"; STATE_FORMAT_VERSION = 2; def save_state(service: str, data: dict) -> None; def load_state(service: str) -> dict \| None; def save_all(services: dict) -> None  # atomic tmp+rename; refuses newer-format files; region recovered from stored ARNs` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_core_persistence.py:5-33,144-201; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:3916-3937,4113-4136; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:230,1047-1059,933-934` | True |
| 41 | `Init scripts (LocalStack-compatible paths)` | `def _collect_scripts(*dirs):; def _run_init_scripts():  # /docker-entrypoint-initaws.d + /etc/localstack/init/boot.d; async def _run_ready_scripts():  # /docker-entrypoint-initaws.d/ready.d + /etc/localstack/init/ready.d; env AWS_* + MINISTACK_INIT_* injected` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ministack_app.py:3968-4058; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:985-1030; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile:71-76; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile.full:154-157` | True |
| 42 | `Testcontainers Python readiness contract` | `DockerContainer("ministackorg/ministack:latest").with_exposed_ports(4566); poll GET {endpoint}/_ministack/health until 200 (30s deadline); boto3.client(service, endpoint_url=endpoint, aws_access_key_id="test", ...)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Testcontainers_python-testcontainers_test_ministack.py:10-46; /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md:1571-1573` | True |

**Contract source refs**

- `gh api repos/ministackorg/ministack (metadata: default_branch=main, license=MIT, language=Python)`
- `gh api 'repos/ministackorg/ministack/git/trees/HEAD?recursive=1' (full blob path listing)`
- `gh api repos/ministackorg/ministack/commits/main -> sha 5080d4b7141e7030e8cc865a4832e8d0c9870e0c (2026-11-19T21:47:51Z)`
- `gh api repos/ministackorg/ministack/contents/README.md (base64-decoded to /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/README.md)`
- `gh api repos/ministackorg/ministack/contents/pyproject.toml (base64-decoded to /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/pyproject.toml)`
- `gh api repos/ministackorg/ministack/contents/Dockerfile and /Dockerfile.full (base64-decoded to /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile, /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/Dockerfile.full)`
- `gh api repos/ministackorg/ministack/contents/docker-compose.yml (base64-decoded to /Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/docker-compose.yml)`
- `gh api repos/ministackorg/ministack/contents/requirements.txt, /bin/awslocal, /AGENTS.md, /LICENSE (base64-decoded)`
- `gh api repos/ministackorg/ministack/contents/ministack/app.py, /ministack/__main__.py, /ministack/__init__.py, /ministack/services/__init__.py (base64-decoded)`
- `gh api repos/ministackorg/ministack/contents/ministack/core/{router,responses,sigv4,persistence,container_reaper,hypercorn_conf,hypercorn_compat,pgproxy,lambda_runtime}.py (base64-decoded)`
- `gh api repos/ministackorg/ministack/contents/ministack/services/{rds,elasticache,documentdb,dsql,opensearch,opensearchserverless,ecs,eks,athena,glue,kinesisanalyticsv2_flink,codebuild,ec2,lambda_svc,mwaa,transfer,rds_data,s3}.py (base64-decoded)`
- `gh api repos/ministackorg/ministack/contents/tests/{test_health,test_multitenancy,test_ministack,conftest,test_persistence,test_sigv4,test_router,test_init_scripts}.py (base64-decoded)`
- `gh api repos/ministackorg/ministack/contents/Testcontainers/python-testcontainers/test_ministack.py (and java/go/dotnet equivalents)`
- `gh api repos/ministackorg/ministack/releases/latest -> tag_name v1.5.43, published_at 2026-11-19T21:00:22Z`
- `https://pypi.org/pypi/ministack/json (curl; info.version=1.5.43, requires_python=>=3.10, license=MIT License, 224 releases)`
- `https://pypi.org/pypi/ministack/1.5.43/json (curl; wheel + sdist uploaded 2026-11-19T21:00:24Z/27Z; requires_dist with full/test/dev extras)`
- `https://index.crates.io/mi/ni/ministack (curl; HTTP 404 -- no Rust crate published)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/ministack-src/ is the local decoded-copy directory for every source path cited above`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/ministack.json (this contract)`

### 1.11 `iroh-smol-kv` -- exact version `0.5.0`

- **License:** MIT OR Apache-2.0 (Cargo.toml:6; published Cargo.toml:34; Cargo.toml.orig; LICENSE-MIT and LICENSE-APACHE are present in both the HEAD tree and the published .crate; crates.io version records list license = "MIT OR Apache-2.0"; docs.rs renders "MIT OR Apache-2.0"; the GitHub repo API reports primary SPDX Apache-2.0 and the crates.io crate-level license field is null)
- **MSRV / Go:** Not declared. Cargo.toml/Cargo.toml.orig/published Cargo.toml contain no rust-version; all six sparse-index rows and all crates.io version records have rust_version = null; the HEAD tree has no .github/ CI MSRV file. edition = "2024" (Cargo.toml:4; published Cargo.toml:13). This is a Rust crate (crate-type ["rlib", "cdylib"]), not a Go module; uniffi-example/go/go.mod exists only as a separate example directory and was not read.
- **Signatures:** 57; **unverified items:** 5; **source refs:** 41

**Feature flags / version gates**

- `default = ["filter-parser"] (Cargo.toml:37; published Cargo.toml:38; sparse-index features2 for all six versions)`
- `filter-parser = ["dep:chrono", "dep:peg"] (Cargo.toml:36; optional chrono ^0.4.45, peg ^0.8.6)`
- `#[cfg(feature = "filter-parser")] gates private mod peg_parser (src/lib.rs:997) supplying impl Display for Filter and impl FromStr for Filter (type Err = String); disabling it removes those two public trait impls and the optional deps. docs.rs/crate/iroh-smol-kv/0.5.0/features reports exactly 1 feature flag, enabled by default.`
- `No other crate features: 0.5.0 index row has features = {} and features2 = {default, filter-parser} only. Dependency-level settings: iroh 1.4 default-features=false; irpc 1.0 default-features=false with features derive/rpc/stream; iroh-gossip 0.103.0 default features; bytes 1.12 serde; rpds 1.2.1 serde.`

**Integration notes (verbatim)**

````text
CRATE STATUS: PUBLISHED, NOT GIT-ONLY. exact_version = 0.5.0. https://index.crates.io/ir/oh/iroh-smol-kv has six rows 0.1.0, 0.2.0, 0.3.0, 0.3.1, 0.4.0, 0.5.0; all yanked=false, rust_version=null, features2 = {default, filter-parser}; 0.5.0 cksum f25caa3f604a0ebacf2c3e72bf1289caaee749c40a9259a2b38945df97f4de3d. crates.io API max_version/newest_version 0.5.0, created 2025-09-22T15:38:04Z, 0.5.0 published 2026-11-19T15:53:24Z by rklaehn, not yanked. GitHub HEAD = 3007ed0b84f718958e386f03a7bef0f4cccc48a8 ('chore: Release', 2026-11-19T15:52:54Z); tag v0.5.0 is that commit; releases list empty. The downloaded https://static.crates.io/crates/iroh-smol-kv/iroh-smol-kv-0.5.0.crate has sha256 equal to the index cksum; .cargo_vcs_info.json names that same sha1; src/lib.rs, README.md, Cargo.toml.orig and both examples are byte-identical to GitHub HEAD. docs.rs/0.5.0 is built. Companion workspace member iroh-smol-kv-uniffi 0.4.0 (depends on iroh-smol-kv 0.5.0 path = \"..\") is GIT-ONLY: sparse index and crates.io API both 404.

API SHAPE: crate docs say 'A small, gossip-based, eventually consistent key-value store.' README.md says 'A tiny in memory replicated key value store that syncs via an iroh-gossip topic.' Two-level map: scope = iroh::PublicKey -> bytes::Bytes key -> SignedValue { timestamp: u64 nanoseconds since epoch, value: Bytes, signature: [u8; 64] }. The signed payload is SigningData<'a> { key: &'a [u8], timestamp: u64, value: &'a [u8] } (src/lib.rs:87-91), so wire order is (key, timestamp, value); README.md line 9 says '(key, value, timestamp)' loosely. Remote inserts verify scope.verify(&signing_data_bytes, &Signature::from_bytes(&value.signature)) (src/lib.rs:496-511); local writes go through the deliberately private Client::put, whose comment says 'This isn't public because it does not verify the signature on the value', and are inserted with insert_signed_value_unverified (src/lib.rs:405-413, 745-747). Client::local(topic: GossipTopic, config: Config) -> Self consumes the GossipTopic, splits it into GossipSender/GossipReceiver (src/lib.rs:588-600), calls tokio::spawn(actor.run()), and therefore requires an ambient Tokio runtime; Client is Clone. Client::write(&self, secret: SecretKey) -> WriteScope; WriteScope::scope() returns secret.public(); WriteScope::put() signs (key, timestamp, value). WriteScope derives Clone, so the write capability is duplicable, not affine/linear. Client::get -> Result<Option<Bytes>, irpc::Error> discards timestamp and signature; use subscribe().stream() (yields (PublicKey, Bytes, SignedValue)) or stream_raw() (yields SubscribeItem) to see metadata. Public reads/observations: subscribe, subscribe_with_opts, iter, iter_with_opts, IterResult::collect, SubscribeResponse::stream_raw, SubscribeResponse::stream. Filter has public scope/key/timestamp bounds, nine builders, and ALL/EMPTY constants; with default feature filter-parser it also implements Display and FromStr (type Err = String). Config has anti_entropy_interval, fast_anti_entropy_interval, expiry: Option<ExpiryConfig>; ExpiryConfig has horizon and check_interval. join_peers is the only peer-management call. InsertError and proto::GossipMessage are public types but no public method returns or consumes them: InsertError is used only in private State::insert_signed_value*, GossipMessage is the internal postcard wire message. The only root re-export is pub use proto::SignedValue; public modules are proto and util. peg_parser::parse_filter is pub inside a private #[cfg(feature = \"filter-parser\")] module and is NOT exported (docs.rs fn.parse_filter.html returns 404; all.html lists only the six util functions).

INTEGRATION: examples/test.rs:44-67 builds iroh_gossip::Gossip::builder().spawn(endpoint.clone()), TopicId::from_bytes([0; 32]), registers iroh_gossip::ALPN with iroh::protocol::Router, then gossip.subscribe(topic, bootstrap_ids).await or subscribe_and_join(...) to obtain GossipTopic; then Client::local(topic, Config::DEBUG), api.write(key.clone()), ws.put(\"hello\", \"world\").await, api.get(node_id, \"hello\").await -> Some(\"world\".into()), api.iter().collect::<Vec<_>>().await, api.subscribe().stream(). examples/cli.rs:108-119 uses Config::default(), Filter::from_str for /iter and /subscribe, subscribe_with_opts(Subscribe { filter, mode: SubscribeMode::Both }), and join_peers(ids). Published Cargo.toml resolves iroh 1.4 default-features=false, iroh-gossip 0.103.0, irpc 1.0 (derive/rpc/stream, default-features=false), n0-future 0.3.2, n0-error 1, tokio 1.54, bytes 1.12 serde, rpds 1.2.1 serde, postcard 1.1.3, rand 0.10, sync_wrapper 1, ed25519-dalek 3, hex 0.4.3, serde-big-array 0.5.1, tracing 0.1; published Cargo.lock pins iroh 1.4.1, iroh-gossip 0.103.0, irpc 1.0.0, n0-error 1.0.3, n0-future 0.3.2, tokio 1.54.1.

SUITABILITY FOR MEMBERSHIP STATE: VERDICT = NO as an authoritative membership view, quorum, leader election, failure detector, or source of truth; LIMITED YES only as an advisory, best-effort presence/heartbeat hint that is continuously refreshed and checked against an external authority. Grounds, all from 0.5.0 source: (1) No durability: State is an in-memory rpds::HashTrieMapSync<PublicKey, RedBlackTreeMapSync<Bytes, SignedValue>> (src/lib.rs:482-485); src/lib.rs contains 0 occurrences of std::fs, tokio::fs, File::, write_all, sled, redb, sqlite. A restart yields an empty store repopulated only by periodic anti-entropy from peers (Config::default every 300 s; fast path every 10 s on Event::NeighborUp, src/lib.rs:635-667, 797-800, 835-842); simultaneous restarts lose all membership state. (2) Built-in expiry and no delete/tombstone/leave: Config::default sets expiry = Some(ExpiryConfig { horizon: 24 h, check_interval: 1 h }); Actor::horizon() computes now - horizon and apply_horizon() removes value.timestamp < horizon (src/lib.rs:602-633); remote values below horizon are ignored (src/lib.rs:819-824); SubscribeItem::Expired((PublicKey, Bytes, u64)) is emitted only on the local broadcast channel (src/lib.rs:843-847), never over gossip. Config::expiry = None disables expiry but leaves unbounded growth. (3) ExpiryConfig::check_interval is a DEAD public field: declared at src/lib.rs:871, set to 10 s in Config::DEBUG and 3600 s in Default, but Actor::run hardcodes horizon_period = Duration::from_secs(30) whenever expiry is Some (src/lib.rs:731-735, 843-850); the Default comment 'purge every hour' does not match the implementation. (4) Merge is wall-clock last-write-wins, not consensus: insert_signed_value_unverified rejects existing.timestamp >= value.timestamp and otherwise overwrites (src/lib.rs:513-537), so equal-timestamp concurrent writes are first-arrival-wins per peer. Timestamps come from ambient SystemTime::now() via util::current_timestamp() (src/lib.rs:379, 918-928), not a logical/version clock; there is no vector clock, CAS, transaction, term/epoch, quorum, read repair, or upper freshness bound, so a signer can put an arbitrary future timestamp and permanently win; clock skew changes the winner; util::to_nanos panics on pre-epoch times and u64 overflow (documented year 2554). (5) Authenticity is not authority: scope is a PublicKey verified remotely, but any node can generate a new SecretKey and claim a new member scope; there is no ACL, admission control, quorum, revocation list, or capability set. Filter.scope restricts only the local subscriber view; it does not restrict what gossip peers receive. GossipMessage is postcard-serialized and broadcast with no KV-layer encryption; all topic peers observe every scope/key/value (src/lib.rs:655-663, 748-751, 811-832). (6) Observation is lossy and partial: Client::local uses tokio::sync::mpsc::channel(32) (src/lib.rs:399), Actor::new uses broadcast::channel(32) (src/lib.rs:591), subscribe_with_opts/iter_with_opts call server_streaming(..., 32) (src/lib.rs:447, 455); handle_subscribe treats any Err from broadcast_rx.recv() (including Lagged) as break (src/lib.rs:703-708), silently terminating a watcher after >32 queued updates with no resync marker. SubscribeResponse::stream() drops CurrentDone and Expired (src/lib.rs:343-349), so expiry notifications require stream_raw(); IterResult::collect() stops at the first non-Entry item and yields only (PublicKey, Bytes, Bytes), stripping timestamp/signature (src/lib.rs:305-314); Client::get() likewise returns bare Bytes. (7) join_peers answers Ok(()) before checking the gossip result; on error the actor logs and breaks (src/lib.rs:768-774), so later calls fail with irpc::Error rather than a diagnostic; there is no explicit sync trigger, and Event::NeighborUp/NeighborDown are handled internally only to restart fast anti-entropy (src/lib.rs:795-810), not exposed as membership events. (8) Not total / not verified: src/lib.rs has no unsafe blocks but has 8 .expect(...) calls and 1 .unwrap() (src/lib.rs:385, 507, 523, 623, 747, 920, 923, 936, 1173); there is no #[cfg(test)] module and no tests/ directory in the HEAD tree; no formal liveness, convergence, or race proof was read. The feature-gated Filter::from_str grammar silently drops invalid scope public keys (src/lib.rs:1117-1123) and turns an unparsable RFC3339 timestamp into Bound::Unbounded (src/lib.rs:1185-1190, 1146-1152), which can broaden a time filter rather than reject it. RECOMMENDED USE IF ACCEPTED AS A HINT ONLY: keep a durable authoritative membership/quorum store elsewhere; encode each member's presence under its own PublicKey scope with a small signed value; re-put on a lease interval strictly shorter than Config::expiry.horizon (24 h default; Config::DEBUG 30 s) and explicitly disable expiry or re-derive from the authoritative store on restart; use subscribe_with_opts(Subscribe { mode: SubscribeMode::Both, filter }) plus stream_raw() to observe CurrentDone and Expired, and treat any stream end or lag as 'state unknown, re-bootstrap' rather than 'member left'; never use get()/iter().collect() as an authoritative read because they discard timestamp and signature; do not treat WriteScope as exclusive (it is Clone); pass an explicit Tokio runtime and supply a GossipTopic from iroh-gossip with the same ALPN/router wiring as the examples. For actual swarm membership, use iroh-gossip's own neighbor events rather than reconstructing membership from this KV. This is a static source-level assessment, not a runtime or formal-verification result.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `iroh_smol_kv::SignedValue (root re-export)` | `pub use proto::SignedValue;` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:33; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/index.html` | True |
| 2 | `iroh_smol_kv::proto (public module)` | `pub mod proto { /* Gossip protocol messages and helpers */ }` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:44-92; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/index.html` | True |
| 3 | `iroh_smol_kv::util (public module)` | `pub mod util { /* Utility functions for working with strings and nanosecond timestamps. */ }` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:901-995; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 4 | `iroh_smol_kv::proto::SignedValue` | `#[derive(Clone, Serialize, Deserialize)]<br>pub struct SignedValue {<br>    /// Timestamp in nanoseconds since epoch<br>    pub timestamp: u64,<br>    /// The actual value<br>    pub value: Bytes,<br>    /// Signature over (key, timestamp, value)<br>    #[serde(with = "BigArray")]<br>    pub signature: [u8; 64],<br>}<br>// custom impl std::fmt::Debug for SignedValue (src/lib.rs:65-73)` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:54-73; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/struct.SignedValue.html` | True |
| 5 | `iroh_smol_kv::proto::SignedValue::system_time` | `pub fn system_time(&self) -> SystemTime` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:75-79; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/struct.SignedValue.html` | True |
| 6 | `iroh_smol_kv::proto::GossipMessage` | `#[derive(Debug, Serialize, Deserialize)]<br>pub enum GossipMessage {<br>    SignedValue(PublicKey, Bytes, SignedValue),<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:81-84; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/enum.GossipMessage.html` | True |
| 7 | `iroh_smol_kv::InsertError` | `#[stack_error(derive, add_meta, from_sources)]<br>pub enum InsertError {<br>    #[error(transparent)]<br>    Signature { source: iroh::SignatureError },<br>    #[error("Value too old: existing timestamp {}, new timestamp {}", old, new)]<br>    ValueTooOld { old: u64, new: u64 },<br>}<br>// rustdoc (0.5.0) renders the macro-injected fields:<br>// Signature { source: SignatureError, meta: Meta },<br>// ValueTooOld { old: u64, new: u64, meta: Meta }` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:94-100; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.InsertError.html` | True |
| 8 | `iroh_smol_kv::SubscribeMode` | `#[derive(Debug, Serialize, Deserialize)]<br>pub enum SubscribeMode {<br>    Current,<br>    Future,<br>    Both,<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:115-123; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.SubscribeMode.html` | True |
| 9 | `iroh_smol_kv::SubscribeItem` | `#[derive(Debug, Serialize, Deserialize, Clone)]<br>pub enum SubscribeItem {<br>    /// A matching entry (scope, key, value)<br>    Entry(Entry),<br>    /// Marker that all current entries have been sent, and future entries will follow.<br>    /// You will get at most one CurrentDone message per subscription.<br>    CurrentDone,<br>    /// An entry that has expired (removed). The u64 is the timestamp when it was last modified.<br>    Expired((PublicKey, Bytes, u64)),<br>}<br>// rustdoc expands Entry(Entry) as Entry((PublicKey, Bytes, SignedValue))` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:125-135; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.SubscribeItem.html` | True |
| 10 | `iroh_smol_kv::Entry (PRIVATE alias, appears in public signatures)` | `type Entry = (PublicKey, Bytes, SignedValue);` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:42; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.SubscribeResponse.html` | True |
| 11 | `iroh_smol_kv::Subscribe` | `#[derive(Debug, Serialize, Deserialize)]<br>pub struct Subscribe {<br>    pub mode: SubscribeMode,<br>    pub filter: Filter,<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:165-169; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Subscribe.html` | True |
| 12 | `iroh_smol_kv::Filter` | `#[derive(Debug, Clone, Serialize, Deserialize)]<br>pub struct Filter {<br>    /// None means no filtering by scope<br>    pub scope: Option<HashSet<PublicKey>>,<br>    /// Range of keys to include<br>    pub key: (Bound<Bytes>, Bound<Bytes>),<br>    /// Range of timestamps (in nanoseconds since epoch) to include<br>    pub timestamp: (Bound<u64>, Bound<u64>),<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:194-202; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 13 | `iroh_smol_kv::Filter::ALL` | `pub const ALL: Self = Self {<br>    scope: None,<br>    key: (Bound::Unbounded, Bound::Unbounded),<br>    timestamp: (Bound::Unbounded, Bound::Unbounded),<br>};` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:205-209; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 14 | `iroh_smol_kv::Filter::EMPTY` | `pub const EMPTY: Self = Self {<br>    scope: None,<br>    key: (Bound::Unbounded, Bound::Excluded(Bytes::new())),<br>    timestamp: (Bound::Unbounded, Bound::Excluded(0)),<br>};` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:210-214; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 15 | `iroh_smol_kv::Filter::scope` | `pub fn scope(self, scope: PublicKey) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:216-218; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 16 | `iroh_smol_kv::Filter::scopes` | `pub fn scopes(self, scope: impl IntoIterator<Item = PublicKey>) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:220-227; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 17 | `iroh_smol_kv::Filter::key` | `pub fn key(self, key: impl Into<Bytes>) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:229-236; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 18 | `iroh_smol_kv::Filter::keys` | `pub fn keys<I, V>(self, range: I) -> Self<br>where<br>    I: RangeBounds<V>,<br>    V: Clone + Into<Bytes>,` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:238-250; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 19 | `iroh_smol_kv::Filter::key_prefix` | `pub fn key_prefix(self, prefix: impl Into<Bytes>) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:252-266; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 20 | `iroh_smol_kv::Filter::timestamps` | `pub fn timestamps(self, range: impl RangeBounds<SystemTime>) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:269-273; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 21 | `iroh_smol_kv::Filter::timestamps_nanos` | `pub fn timestamps_nanos(self, range: impl RangeBounds<u64>) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:276-284; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 22 | `iroh_smol_kv::Filter::contains` | `pub fn contains(&self, scope: &PublicKey, key: &[u8], timestamp: u64) -> bool` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:287-289; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 23 | `iroh_smol_kv::Filter::contains_key` | `pub fn contains_key(&self, scope: &PublicKey, key: &[u8]) -> bool` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:292-299; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html` | True |
| 24 | `iroh_smol_kv::Filter: impl Display (feature = "filter-parser")` | `impl Display for Filter {<br>    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { ... }<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:997-1088; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html; https://docs.rs/crate/iroh-smol-kv/0.5.0/features` | True |
| 25 | `iroh_smol_kv::Filter: impl FromStr (feature = "filter-parser")` | `impl FromStr for Filter {<br>    type Err = String;<br>    fn from_str(s: &str) -> Result<Self, Self::Err> { parse_filter(s) }<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:1090-1096; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html; https://docs.rs/crate/iroh-smol-kv/0.5.0/features` | True |
| 26 | `iroh_smol_kv::IterResult` | `pub struct IterResult(BoxFuture<Result<mpsc::Receiver<SubscribeItem>, irpc::Error>>);<br>// docs.rs renders the tuple field as private: pub struct IterResult(/* private fields */);` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:302; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.IterResult.html` | True |
| 27 | `iroh_smol_kv::IterResult::collect` | `pub async fn collect<C: Default + Extend<(PublicKey, Bytes, Bytes)>>(<br>        self,<br>    ) -> Result<C, irpc::Error>` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:304-315; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.IterResult.html` | True |
| 28 | `iroh_smol_kv::SubscribeResponse` | `pub struct SubscribeResponse(BoxFuture<Result<mpsc::Receiver<SubscribeItem>, irpc::Error>>);<br>// docs.rs renders the tuple field as private: pub struct SubscribeResponse(/* private fields */);` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:317; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.SubscribeResponse.html` | True |
| 29 | `iroh_smol_kv::SubscribeResponse::stream_raw` | `pub fn stream_raw(<br>        self,<br>    ) -> impl n0_future::Stream<Item = Result<SubscribeItem, irpc::Error>> + Send + Sync + 'static` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:319-332; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.SubscribeResponse.html` | True |
| 30 | `iroh_smol_kv::SubscribeResponse::stream` | `pub fn stream(<br>        self,<br>    ) -> impl n0_future::Stream<Item = Result<Entry, irpc::Error>> + Send + Sync + 'static<br>// rustdoc expands Entry as (PublicKey, Bytes, SignedValue);<br>// it filters out SubscribeItem::CurrentDone and SubscribeItem::Expired` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:335-354; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.SubscribeResponse.html` | True |
| 31 | `iroh_smol_kv::Client` | `#[derive(Debug, Clone)]<br>pub struct Client(irpc::Client<Proto>);<br>// tuple field and Proto are private` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:357-358; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 32 | `iroh_smol_kv::Client::local` | `/// Create a local client. This requires a tokio runtime.<br>pub fn local(topic: GossipTopic, config: Config) -> Self` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:397-403; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 33 | `PRIVATE (not public API): Client::put` | `/// This isn't public because it does not verify the signature on the value.<br>async fn put(<br>        &self,<br>        scope: PublicKey,<br>        key: Bytes,<br>        value: SignedValue,<br>    ) -> Result<(), irpc::Error>` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:405-413` | True |
| 34 | `iroh_smol_kv::Client::write` | `/// Create a write scope that can put values signed by the given secret key.<br>pub fn write(&self, secret: SecretKey) -> WriteScope` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:415-422; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 35 | `iroh_smol_kv::Client::get` | `pub async fn get(<br>        &self,<br>        scope: PublicKey,<br>        key: impl Into<Bytes>,<br>    ) -> Result<Option<Bytes>, irpc::Error>` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:424-437; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 36 | `iroh_smol_kv::Client::subscribe` | `pub fn subscribe(&self) -> SubscribeResponse<br>// internally SubscribeMode::Both + Filter::ALL` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:439-444; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 37 | `iroh_smol_kv::Client::subscribe_with_opts` | `pub fn subscribe_with_opts(&self, subscribe: Subscribe) -> SubscribeResponse` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:446-448; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 38 | `iroh_smol_kv::Client::iter_with_opts` | `pub fn iter_with_opts(&self, filter: Filter) -> IterResult` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:450-456; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 39 | `iroh_smol_kv::Client::iter` | `pub fn iter(&self) -> IterResult` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:458-464; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 40 | `iroh_smol_kv::Client::join_peers` | `pub fn join_peers(<br>        &self,<br>        peers: impl IntoIterator<Item = EndpointId>,<br>    ) -> impl n0_future::Future<Output = Result<(), irpc::Error>>` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:466-474; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 41 | `iroh_smol_kv::Client::shutdown` | `pub async fn shutdown(&self) -> Result<(), irpc::Error>` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:476-479; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html` | True |
| 42 | `iroh_smol_kv::WriteScope` | `#[derive(Debug, Clone)]<br>pub struct WriteScope {<br>    api: Client,<br>    secret: SecretKey,<br>    public: PublicKey,<br>}<br>// all fields private; docs.rs: pub struct WriteScope { /* private fields */ }` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:360-365; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.WriteScope.html` | True |
| 43 | `iroh_smol_kv::WriteScope::scope` | `pub fn scope(&self) -> PublicKey` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:367-370; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.WriteScope.html` | True |
| 44 | `iroh_smol_kv::WriteScope::put` | `pub async fn put(<br>        &self,<br>        key: impl Into<Bytes>,<br>        value: impl Into<Bytes>,<br>    ) -> Result<(), irpc::Error>` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:372-393; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.WriteScope.html` | True |
| 45 | `iroh_smol_kv::Config` | `#[derive(Debug, Clone)]<br>pub struct Config {<br>    pub anti_entropy_interval: Duration,<br>    pub fast_anti_entropy_interval: Duration,<br>    /// Optional horizon duration. Values older than now - horizon are removed,<br>    /// and will not be re-added.<br>    pub expiry: Option<ExpiryConfig>,<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:857-864; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Config.html` | True |
| 46 | `iroh_smol_kv::Config::DEBUG` | `pub const DEBUG: Self = Self {<br>    anti_entropy_interval: Duration::from_secs(30),<br>    fast_anti_entropy_interval: Duration::from_secs(5),<br>    expiry: Some(ExpiryConfig {<br>        horizon: Duration::from_secs(30),<br>        check_interval: Duration::from_secs(10),<br>    }),<br>};` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:874-883; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Config.html` | True |
| 47 | `iroh_smol_kv::Config: impl Default` | `impl Default for Config {<br>    fn default() -> Self {<br>        Self {<br>            // republish all known values every 5 minutes<br>            anti_entropy_interval: Duration::from_secs(300),<br>            // republish all known values every 10 seconds when we get a new peer<br>            fast_anti_entropy_interval: Duration::from_secs(10),<br>            // keep values for 24 hours, purge every hour<br>            expiry: Some(ExpiryConfig {<br>                horizon: Duration::from_secs(3600 * 24),<br>                check_interval: Duration::from_secs(3600),<br>            }),<br>        }<br>    }<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:885-899; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Config.html` | True |
| 48 | `iroh_smol_kv::ExpiryConfig` | `#[derive(Debug, Clone)]<br>pub struct ExpiryConfig {<br>    /// Duration after which values expire.<br>    pub horizon: Duration,<br>    /// How often to check for expired values.<br>    pub check_interval: Duration,<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:866-872; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.ExpiryConfig.html` | True |
| 49 | `iroh_smol_kv::util::to_nanos` | `/// We use nanoseconds since epoch for timestamps.<br>/// This will overflow in the year 2554. ...<br>pub fn to_nanos(t: &SystemTime) -> u64` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:911-924; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 50 | `iroh_smol_kv::util::current_timestamp` | `pub fn current_timestamp() -> u64` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:926-928; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 51 | `iroh_smol_kv::util::from_nanos` | `pub fn from_nanos(nanos: u64) -> SystemTime` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:930-932; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 52 | `iroh_smol_kv::util::next_prefix` | `pub fn next_prefix(bytes: &mut [u8]) -> bool` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:950-959; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 53 | `iroh_smol_kv::util::format_bytes` | `pub fn format_bytes(bytes: &[u8]) -> String` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:961-974; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 54 | `iroh_smol_kv::util::escape_string` | `pub fn escape_string(s: &str) -> String` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:976-987; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html` | True |
| 55 | `NOT EXPORTED: peg_parser::parse_filter (private module)` | `pub fn parse_filter(input: &str) -> Result<Filter, String><br>// inside #[cfg(feature = "filter-parser")] mod peg_parser (private); docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/fn.parse_filter.html returns 404` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:997-998, 1207-1209; https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/fn.parse_filter.html` | True |
| 56 | `PRIVATE (not public API): Proto RPC enum` | `#[derive(Debug, Serialize, Deserialize)]<br>#[rpc_requests(message = Message)]<br>enum Proto {<br>    #[rpc(tx = oneshot::Sender<()>)]<br>    Put(Put),<br>    #[rpc(tx = oneshot::Sender<Option<SignedValue>>)]<br>    Get(Get),<br>    #[rpc(tx = mpsc::Sender<SubscribeItem>)]<br>    Subscribe(Subscribe),<br>    #[rpc(tx = oneshot::Sender<()>)]<br>    JoinPeers(JoinPeers),<br>    #[rpc(tx = oneshot::Sender<()>)]<br>    Shutdown(Shutdown),<br>}` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:179-192` | True |
| 57 | `PRIVATE (not public API): internal structs` | `#[derive(Debug, Serialize, Deserialize)]<br>struct Put { pub scope: PublicKey, pub key: Bytes, pub value: SignedValue }<br>#[derive(Debug, Serialize, Deserialize)]<br>struct Get { pub scope: PublicKey, pub key: Bytes }<br>#[derive(Debug, Serialize, Deserialize, Clone)]<br>pub(crate) enum BroadcastItem { Entry(Entry), Expired((PublicKey, Bytes, u64)) }<br>#[derive(Debug, Serialize, Deserialize)]<br>pub(crate) struct JoinPeers { pub peers: Vec<EndpointId> }<br>#[derive(Debug, Serialize, Deserialize)]<br>pub(crate) struct Shutdown;<br>#[derive(Debug, Clone, Serialize, Deserialize, Default)]<br>struct State { current: rpds::HashTrieMapSync<PublicKey, rpds::RedBlackTreeMapSync<Bytes, SignedValue>> }<br>struct Actor { /* GossipSender, GossipReceiver, mpsc::Receiver<Message>, Config, State, broadcast::Sender<BroadcastItem> */ }<br>#[derive(Debug, Clone, Serialize, Deserialize)]<br>pub(crate) struct SigningData<'a> { pub key: &'a [u8], pub timestamp: u64, pub value: &'a [u8] }` | `/tmp/iroh-smol-kv-src-20261119/src__lib.rs:87-91, 102-113, 137-143, 171-177, 482-486, 579-586` | True |

**Contract source refs**

- `gh api repos/n0-computer/iroh-smol-kv (repo metadata: default_branch main, license Apache-2.0, archived false, pushed_at 2026-11-19T15:53:29Z)`
- `gh api repos/n0-computer/iroh-smol-kv/git/trees/HEAD?recursive=1 (tree sha 3007ed0b84f718958e386f03a7bef0f4cccc48a8, truncated=false; lists README.md, Cargo.toml, src/lib.rs, examples/, iroh-smol-kv-uniffi/, uniffi-example/, LICENSE-MIT, LICENSE-APACHE)`
- `gh api repos/n0-computer/iroh-smol-kv/commits/HEAD (sha 3007ed0b84f718958e386f03a7bef0f4cccc48a8, 2026-11-19T15:52:54Z, 'chore: Release')`
- `gh api repos/n0-computer/iroh-smol-kv/tags (v0.5.0 -> 3007ed0b84f718958e386f03a7bef0f4cccc48a8)`
- `gh api repos/n0-computer/iroh-smol-kv/releases (empty array)`
- `gh api repos/n0-computer/iroh-smol-kv/contents/README.md?ref=HEAD (base64-decoded to /tmp/iroh-smol-kv-src-20261119/README.md; 634 bytes)`
- `gh api repos/n0-computer/iroh-smol-kv/contents/Cargo.toml?ref=HEAD (base64-decoded to /tmp/iroh-smol-kv-src-20261119/Cargo.toml; 1652 bytes)`
- `gh api repos/n0-computer/iroh-smol-kv/contents/src/lib.rs?ref=HEAD (base64-decoded to /tmp/iroh-smol-kv-src-20261119/src__lib.rs; 41488 bytes; read in full, lines 1-1210)`
- `gh api repos/n0-computer/iroh-smol-kv/contents/examples/cli.rs?ref=HEAD (base64-decoded to /tmp/iroh-smol-kv-src-20261119/examples__cli.rs; read in full, lines 1-366)`
- `gh api repos/n0-computer/iroh-smol-kv/contents/examples/test.rs?ref=HEAD (base64-decoded to /tmp/iroh-smol-kv-src-20261119/examples__test.rs; read in full, lines 1-86)`
- `gh api repos/n0-computer/iroh-smol-kv/contents/iroh-smol-kv-uniffi/Cargo.toml?ref=HEAD (version 0.4.0, depends on iroh-smol-kv 0.5.0 path = "..")`
- `gh api repos/n0-computer/iroh-smol-kv/contents/iroh-smol-kv-uniffi/README.md?ref=HEAD ('Uniffi bindings for iroh-smol-kv')`
- `https://index.crates.io/ir/oh/iroh-smol-kv (downloaded to /tmp/iroh-smol-kv-index.json; six JSONL rows 0.1.0-0.5.0 with deps, features2, yanked, rust_version, cksum)`
- `https://index.crates.io/ir/oh/iroh-smol-kv-uniffi (HTTP 404 NoSuchKey; companion crate not published)`
- `https://crates.io/api/v1/crates/iroh-smol-kv (downloaded to /tmp/iroh-smol-kv-cratesio.json; max_version 0.5.0, created_at 2025-09-22T15:38:04Z, updated_at 2026-11-19T15:53:24Z, all six versions with created_at/rust_version/license)`
- `https://crates.io/api/v1/crates/iroh-smol-kv-uniffi (HTTP 404)`
- `https://static.crates.io/crates/iroh-smol-kv/iroh-smol-kv-0.5.0.crate (downloaded to /tmp/iroh-smol-kv-050-publish/iroh-smol-kv-0.5.0.crate; sha256 f25caa3f604a0ebacf2c3e72bf1289caaee749c40a9259a2b38945df97f4de3d equals index cksum)`
- `/tmp/iroh-smol-kv-050-publish/extracted/iroh-smol-kv-0.5.0/.cargo_vcs_info.json (git.sha1 = 3007ed0b84f718958e386f03a7bef0f4cccc48a8, path_in_vcs = "")`
- `/tmp/iroh-smol-kv-src-20261119/README.md (GitHub HEAD README.md lines 1-17; two-level map, signatures, filters)`
- `/tmp/iroh-smol-kv-src-20261119/Cargo.toml (GitHub HEAD Cargo.toml lines 1-56; version 0.5.0, edition 2024, license, features, workspace deps)`
- `/tmp/iroh-smol-kv-src-20261119/src__lib.rs (GitHub HEAD src/lib.rs lines 1-1210; primary source for every signature)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/ (root index: re-export SignedValue, modules proto/util, 8 structs, 4 enums; WebFetch loaded, not building)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/all.html (all-items index; 8 root structs, 4 enums, 6 util functions, proto::SignedValue, proto::GossipMessage)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html (nine public method signatures)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.WriteScope.html (scope + put)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html (fields, ALL/EMPTY, nine methods, Display/FromStr impls)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Config.html (fields, DEBUG, Default)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.ExpiryConfig.html (fields)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.IterResult.html (collect)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.SubscribeResponse.html (stream_raw, stream with expanded Entry tuple)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Subscribe.html (fields)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.SubscribeItem.html (expanded Entry tuple)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.SubscribeMode.html (variants)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.InsertError.html (macro-injected meta: Meta fields and trait impls)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/struct.SignedValue.html (fields, system_time)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/enum.GossipMessage.html (variant)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html (six public function names)`
- `https://docs.rs/crate/iroh-smol-kv/0.5.0/features (1 feature flag; default -> filter-parser -> dep:chrono, dep:peg)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/fn.parse_filter.html (HTTP 404; parse_filter is not exported)`
- `https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/ (curl -L returned HTTP 200, /tmp/docsrs-iroh-smol-kv-050.html)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-smol-kv.json (this contract, written with python3 -I after mkdir -p)`

### 1.12 `laboon-cli` -- exact version `v0.1.0`

- **License:** Apache-2.0 (LICENSE: Apache License Version 2.0; COPYRIGHT: Copyright 2026 Bunshin Labs LLC - bunshinlabs.com; GitHub API license.spdx_id Apache-2.0)
- **MSRV / Go:** Go 1.27.1 (go 1.27.1 in root go.mod, go.work and all four cmd/*/go.mod at both v0.1.0 and HEAD; README.md: Go 1.27.1 or newer)
- **Signatures:** 26; **unverified items:** 6; **source refs:** 47

**Feature flags / version gates**


**Integration notes (verbatim)**

````text
IDENTITY: NAME=laboon-cli is a harness label, not a Cargo crate. index.crates.io/la/bo/laboon -> 404, index.crates.io/la/bo/laboon-cli -> 404, docs.rs/laboon -> 404, crates.io/api/v1/crates/laboon -> 403. The real dependency is Go module github.com/samyfodil/laboon, CLI package main in cmd/laboon (separate module github.com/samyfodil/laboon/cmd/laboon). feature_flags=[] is therefore correct: no Cargo crate, no features.
VERSION: exact_version v0.1.0 is the only published version: proxy.golang.org/github.com/samyfodil/laboon/@v/list = ["v0.1.0"]; /@v/v0.1.0.info = {Version:v0.1.0, Time:2026-11-17T15:23:02Z, Origin.Hash:72b4fb91209d8d9576749da097c3fda4f9365f6f, Ref:refs/tags/v0.1.0}. GitHub has one release: tag v0.1.0, name "laboon 0.1.0", published 2026-11-17T15:25:44Z, 13 assets, annotated tag object 049f6c2a764f0e51556a560b575120b5f0f122e0. HEAD of main at read time = ad7a15ba1997d6fb3ec24b1970a75015f40285ce (2026-11-19T23:04:37Z). In signatures, source URLs pinned to 72b4fb91... are the v0.1.0 tag; ad7a15ba... is HEAD. Every signature was read via gh api repos/samyfodil/laboon/contents/<path>?ref=<sha> (base64-decoded); decoded copies are at /tmp/laboon-work/data/tag/ and /tmp/laboon-work/data/head/ (flat names, / -> __). L<n> prefixes are exact line numbers in those files.
CLI: main flag block and `build` flag block are byte-identical between v0.1.0 and HEAD (verified by exact block diff), so the CLI is stable even though the library is not. Invocation: `laboon [-root dir] program [args...]` or `laboon -image ref [command [args...]]`; subcommands build, version, -version, --version. Only -runtime=standalone is wired: docker|laboon fail with "-runtime %s submits the run to a daemon, not yet wired in this build; run laboond (it serves the Docker API) and point a docker client at it"; any other value fails. Env read: LABOON_CPUPROFILE (pprof), DISPLAY (only with -x11). Exit codes: guest status normally, 2 missing program, 125 fail(err), 1 build error. -image uses laboon.RunImage(ctx, *image, opts...), otherwise laboon.Run(ctx, flag.Arg(0), append(opts, laboon.WithArgs(flag.Args()[1:]...))...). -allow-ports uses strconv.Atoi per comma item; -mem accepts K/M/G; -mount requires HOST:GUEST; -env requires KEY=VALUE.
LIBRARY BREAK: v0.1.0 has WithRoot(dir string), WithMount(host, guest string), WithMountReadOnly(host, guest string), WithX11() and internal/oci. HEAD removes all four (GitHub code search repo:samyfodil/laboon total_count=0 for WithRoot, WithMountReadOnly, WithX11), changes WithMount to func WithMount(guest string, fs FileSystem) Option, adds fs.go (FileSystem, Node/File/Attr aliases, Kind*, NewMemFS, NewDirFS, NewDiskFS, Overlay, ReadOnly, Bind, HardenFS, HardenOption, WithNoHardLink/WithNoSymLink/WithNoDevice/WithNoSocket/WithNoFifo/WithConfinedLinks), moves internal/oci -> oci, and inlines -x11 in the CLI. Run/RunImage/Start/StartImage/Process and all other With* options keep the same signatures; imageRun's error text changed from "WithRoot does not apply to an image" to 'a root filesystem (WithMount "/") does not apply to an image'. STALE DOCS: docs/permissions-and-limits.md is byte-identical at v0.1.0 and HEAD but documents WithMount(host, guest) and WithMountReadOnly(host, guest), which no longer exist at HEAD; HEAD README.md still mentions WithRoot and WithMountReadOnly. At HEAD compose read-only binds as laboon.WithMount(guest, laboon.ReadOnly(hostfs.Dir(host))) and roots as laboon.WithMount("/", hostfs.Dir(root)). hostfs.Dir is a confined writable FileSystem (symlink resolution stays inside the subtree; at "/" it is a read-only image root with an ephemeral writable upper) with AF_UNIX socket forwarding.
EGRESS + CONFINED FS: WithNet() enables egress (default loopback-only), WithNoNet() removes INET sockets, WithAllowDomains(...string) permits exact domain or subdomain via supervisor-answered DNS, WithAllowCIDRs(...string) adds directly reachable networks (needed for private/link-local/metadata targets), WithAllowPorts(...int) restricts destination ports. docs/permissions-and-limits.md line 10: "enabling networking alone does not create a restrictive allowlist" -- empty allowlist = all. internal/sandbox/egress.go: pinTTL=2min, domainAllowed exact-or-subdomain, internalIP blocks loopback/private/link-local/unspecified unless explicit CIDR, connectAllowed never permits host loopback. Filesystem: no host path without an explicit grant; v0.1.0 Config.Root (read-only root string) + Mount{Host,Guest,ReadOnly}; HEAD Config.RootFS core.FileSystem + Mount{Guest,FS}. Library defaults: stdout/stderr discarded without WithStdio/WithStdout/WithStderr; env PATH=/usr/local/bin:/usr/bin:/bin, HOME=/; no memory cap; RWX allowed; images cached offline unless WithPull(); first pull needs host registry access (separate from guest net perms); Run/RunImage return (int, error); signal-killed guest exits 128+signum; cancelling ctx kills it.
RELEASE: .goreleaser.yaml builds laboon (cmd/laboon) and laboon-mcp (cmd/laboon-mcp) for goos [linux,darwin,windows] x goarch [amd64,arm64], CGO_ENABLED=0, -trimpath, -X main.version; archives include README.md/LICENSE/COPYRIGHT; checksum name_template checksums.txt, sha256. 13 assets: checksums.txt + 6 laboon_* + 6 laboon-mcp_*; laboond and laboon-runtime are NOT shipped. Download URL pattern https://github.com/samyfodil/laboon/releases/download/v0.1.0/<asset>. Full checksums.txt is reproduced verbatim below; its own sha256 is ed592f16f49c3c8b74f44dc97401b640258f86dd7d8ffef78b8d17c60442abe8.
--- BEGIN checksums.txt ---
1fb59221e4f2aab4ad5e19693bb28cecde9cabaf430aa81c97cb3d42e9a5590b  laboon-mcp_0.1.0_darwin_amd64.tar.gz
192a474acfc6f93ce77c7f132427bedba73f19b4afeb9b39a365053ca9fbaf88  laboon-mcp_0.1.0_darwin_arm64.tar.gz
ada057ea8fdaac624914ead08923caeea62dafd4b8397c19d87b56409765b11a  laboon-mcp_0.1.0_linux_amd64.tar.gz
c94c25c51788b15f298696a972656a10f66026a6c26c29403ed539e86e7fa728  laboon-mcp_0.1.0_linux_arm64.tar.gz
924d133ec38dba0eea580a1b0ff2ee89c7b41d815809ce6fc651455c85348d7a  laboon-mcp_0.1.0_windows_amd64.zip
43482e94b1563932591fbef1b5f3239885c8e02ddbc0403ec4d3131f70d6613d  laboon-mcp_0.1.0_windows_arm64.zip
bd3c10db9086d5c0c688c502b7a80607c62f5ad1a9ec510f9146d9fece6ba89e  laboon_0.1.0_darwin_amd64.tar.gz
1116d234f49b3f5f3c7ce363ddb98c4d084a4e6d11342a613386a4d89203f279  laboon_0.1.0_darwin_arm64.tar.gz
842fe5620db4592498ff599a080c9f8f31b710a27face7ac2ffa13799d5312c8  laboon_0.1.0_linux_amd64.tar.gz
8459d1d6a00df841d4fff3419beb02e20cf61a37357c4cb5f720ca5384c1955a  laboon_0.1.0_linux_arm64.tar.gz
ef9c67428b3cde1aecae2b3c90a123f56f4d70a02e7b62fa7ec517f474c14366  laboon_0.1.0_windows_amd64.zip
2e1a02a19882975ef8b0dc14b275f8625704b6b1a7861cfac8e52ff81f710071  laboon_0.1.0_windows_arm64.zip
--- END checksums.txt ---
PLATFORM: the requested 'platform support matrix from docs/permissions-and-limits.md' DOES NOT EXIST in that file (34 lines / 1457 bytes, no OS x arch table). It has only: (1) 'The current Windows exception is that guest AF_INET sockets require WithNet, even for loopback'; (2) 'WithMaxMemory limits mapped address space on Linux and stub-process committed memory on Windows; **macOS does not enforce it yet**'; (3) 'laboon is confined by default; every capability below is an explicit opt-in'. The real matrix is linux/darwin/windows x amd64/arm64 from .goreleaser.yaml, README.md badges and line 56, AGENTS.md line 89 ('Platforms are linux, darwin, windows x amd64, arm64'), and the 12 archives. Host backstops: seccomp (Linux), Seatbelt (macOS), AppContainer (Windows). README: native execution needs a matching CPU arch, no amd64<->arm64 translation.
GO VERSION: root go.mod line 3 = `go 1.27.1` at both v0.1.0 and HEAD; go.work and all four cmd/*/go.mod declare go 1.27.1; README: 'To build from source, you need Go 1.27.1 or newer on Linux, macOS, or Windows, on amd64 or arm64.'; CI/release use actions/setup-go@v5 with go-version-file: go.mod. The harness note 'Go 1.27.1 is required to build from source' is CONFIRMED as a declaration, not by compiling.
laboond: separate module, `laboond -host unix:///run/laboon/laboond.sock [-net]`, http.ServeMux routes in daemon.go L38-51, strips optional /v1.xx prefix, sets Api-Version: 1.43 / Ostype: linux; constants apiVersion 1.43, minAPIVersion 1.24, laboonVersion 0.1.0. GET|HEAD /_ping, GET /version, GET /info, POST /images/create, POST /containers/create, GET /containers/json, GET /containers/{id}/json, POST /containers/{id}/start, POST /containers/{id}/wait, POST /containers/{id}/attach, DELETE /containers/{id}. attach hijacks the connection (101 UPGRADED with Upgrade: tcp, else 200 OK) and emits Docker stdcopy frames; start waits up to 2s for a pending attach; wait flushes 200 headers before blocking. cmd/laboond/container.go runs each container with laboon.RunImage(context.Background(), c.image, opts...). handlers.go and daemon.go are byte-identical at v0.1.0 and HEAD.
INTEGRATION: do not add this to Cargo.toml. Consume as a prebuilt Go CLI binary (matching OS/arch asset, verify sha256 against checksums.txt) or import Go module github.com/samyfodil/laboon@v0.1.0. Pin v0.1.0 for the released API; if using HEAD, re-read fs.go and options.go because the docs are stale.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `v0.1.0/laboon.go (10 signatures)` | `L31  v0.1.0 laboon.ErrKilled :: var ErrKilled = sandbox.ErrKilled<br>L38  v0.1.0 laboon.Run :: func Run(ctx context.Context, program string, opts ...Option) (int, error)<br>L51  v0.1.0 laboon.RunImage :: func RunImage(ctx context.Context, ref string, opts ...Option) (int, error)<br>L60  v0.1.0 laboon.Process :: type Process struct { cancel context.CancelFunc; control *sandbox.Control; done chan struct{}; code int; err error }<br>L69  v0.1.0 laboon.Start :: func Start(ctx context.Context, program string, opts ...Option) (*Process, error)<br>L80  v0.1.0 laboon.StartImage :: func StartImage(ctx context.Context, ref string, opts ...Option) (*Process, error)<br>L102  v0.1.0 (*laboon.Process).Wait :: func (p *Process) Wait() (int, error)<br>L109  v0.1.0 (*laboon.Process).Kill :: func (p *Process) Kill()<br>L117  v0.1.0 (*laboon.Process).Pid :: func (p *Process) Pid() int<br>L126  v0.1.0 (*laboon.Process).Signal :: func (p *Process) Signal(sig os.Signal) error` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/laboon.go` | True |
| 2 | `v0.1.0/options.go (40 signatures)` | `L14  v0.1.0 laboon.Option :: type Option func(*options)<br>L42  v0.1.0 laboon.WithArgs :: func WithArgs(args ...string) Option<br>L50  v0.1.0 laboon.WithEntrypoint :: func WithEntrypoint(entrypoint ...string) Option<br>L57  v0.1.0 laboon.WithEnv :: func WithEnv(env ...string) Option<br>L63  v0.1.0 laboon.WithEnvVar :: func WithEnvVar(key, value string) Option<br>L69  v0.1.0 laboon.WithStdin :: func WithStdin(r io.Reader) Option<br>L75  v0.1.0 laboon.WithStdout :: func WithStdout(w io.Writer) Option<br>L81  v0.1.0 laboon.WithStderr :: func WithStderr(w io.Writer) Option<br>L86  v0.1.0 laboon.WithStdio :: func WithStdio(in io.Reader, out, err io.Writer) Option<br>L92  v0.1.0 laboon.WithDiscardStdin :: func WithDiscardStdin() Option<br>L99  v0.1.0 laboon.WithDiscardStdout :: func WithDiscardStdout() Option<br>L104  v0.1.0 laboon.WithDiscardStderr :: func WithDiscardStderr() Option<br>L110  v0.1.0 laboon.WithDiscardStdio :: func WithDiscardStdio() Option<br>L116  v0.1.0 laboon.WithDir :: func WithDir(dir string) Option<br>L124  v0.1.0 laboon.WithRoot :: func WithRoot(dir string) Option  // v0.1.0 ONLY; removed at HEAD<br>L129  v0.1.0 laboon.WithTrace :: func WithTrace(w io.Writer) Option<br>L135  v0.1.0 laboon.WithRejectRWX :: func WithRejectRWX() Option<br>L143  v0.1.0 laboon.WithMaxMemory :: func WithMaxMemory(bytes uint64) Option<br>L149  v0.1.0 laboon.WithNet :: func WithNet() Option<br>L155  v0.1.0 laboon.WithNoNet :: func WithNoNet() Option<br>L165  v0.1.0 laboon.WithMount :: func WithMount(host, guest string) Option  // v0.1.0 signature; CHANGED at HEAD<br>L179  v0.1.0 laboon.WithMountReadOnly :: func WithMountReadOnly(host, guest string) Option  // v0.1.0 ONLY; removed at HEAD<br>L187  v0.1.0 laboon.Terminal / laboon.WinSize :: type ( Terminal = sandbox.Terminal; WinSize = sandbox.WinSize )<br>L197  v0.1.0 laboon.WithTerminal :: func WithTerminal(t sandbox.Terminal) Option<br>L203  v0.1.0 laboon.Device / laboon.DeviceKind :: type ( Device = sandbox.Device; DeviceKind = sandbox.DeviceKind )<br>L209  v0.1.0 laboon.DeviceSerial :: const DeviceSerial = sandbox.DeviceSerial<br>L214  v0.1.0 laboon.WithDevice :: func WithDevice(d *sandbox.Device) Option<br>L221  v0.1.0 laboon.WithUser :: func WithUser(spec string) Option<br>L227  v0.1.0 laboon.WithX11 :: func WithX11() Option  // v0.1.0 ONLY; removed at HEAD<br>L239  v0.1.0 laboon.WithAllowDomains :: func WithAllowDomains(domains ...string) Option<br>L245  v0.1.0 laboon.WithAllowCIDRs :: func WithAllowCIDRs(cidrs ...string) Option<br>L251  v0.1.0 laboon.WithAllowPorts :: func WithAllowPorts(ports ...int) Option<br>L257  v0.1.0 laboon.WithPull :: func WithPull() Option<br>L263  v0.1.0 laboon.WithPlatform :: func WithPlatform(platform string) Option<br>L269  v0.1.0 laboon.WithImageCacheDir :: func WithImageCacheDir(dir string) Option<br>L274  v0.1.0 laboon.WithMaxImageSize :: func WithMaxImageSize(bytes uint64) Option<br>L280  v0.1.0 laboon.WithCache :: func WithCache(c *Cache) Option<br>L286  v0.1.0 laboon.Cache :: type Cache = rewrite.Cache<br>L289  v0.1.0 laboon.DefaultCache :: func DefaultCache() *Cache<br>L293  v0.1.0 laboon.WithNoRewrite :: func WithNoRewrite() Option` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/options.go` | True |
| 3 | `HEAD/options.go (1 signatures)` | `L159  HEAD laboon.WithMount :: func WithMount(guest string, fs FileSystem) Option  // breaking: was WithMount(host, guest string) in v0.1.0` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/options.go` | True |
| 4 | `HEAD/cmd/laboon/main.go (3 signatures)` | `L207  HEAD laboon.WithRoot :: ABSENT at HEAD; replaced by WithMount("/", hostfs.Dir(dir))<br>L197  HEAD laboon.WithX11 :: ABSENT at HEAD; the -x11 CLI flag now inlines WithMount + WithEnvVar<br>L199  cli.env.DISPLAY :: read only when -x11 is set (v0.1.0 via WithX11 at options.go:230; HEAD inlined at cmd/laboon/main.go:199)` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/cmd/laboon/main.go` | True |
| 5 | `HEAD/fs.go (18 signatures)` | `L86  HEAD laboon.WithMountReadOnly :: ABSENT at HEAD; replaced by laboon.ReadOnly(hostfs.Dir(dir))<br>L25  HEAD laboon.FileSystem :: type FileSystem = core.FileSystem<br>L28  HEAD laboon core type aliases :: type ( Node = core.Node; File = core.File; Attr = core.Attr; AttrUpdate = core.AttrUpdate; CreateSpec = core.CreateSpec; Dirent = core.Dirent; Kind = core.Kind; NodeID = core.NodeID; StatFS = core.StatFS; StatFSer = core.StatFSer; WriteFileSystem = core.WriteFileSystem )<br>L54  HEAD laboon Kind constants :: const ( KindReg = core.Reg; KindDir = core.Dir; KindSymlink = core.Symlink; KindFifo = core.Fifo; KindChardev = core.Chardev; KindBlockdev = core.Blockdev; KindSocket = core.Socket )<br>L66  HEAD laboon.NewMemFS :: func NewMemFS() FileSystem<br>L72  HEAD laboon.NewDirFS :: func NewDirFS(root string) FileSystem<br>L77  HEAD laboon.NewDiskFS :: func NewDiskFS(root string) FileSystem<br>L82  HEAD laboon.Overlay :: func Overlay(layers ...FileSystem) FileSystem<br>L86  HEAD laboon.ReadOnly :: func ReadOnly(fs FileSystem) FileSystem<br>L90  HEAD laboon.Bind :: func Bind(fs FileSystem, root Node) FileSystem<br>L97  HEAD laboon.HardenFS :: func HardenFS(fs FileSystem, opts ...HardenOption) FileSystem<br>L100  HEAD laboon.HardenOption :: type HardenOption = harden.HardenOption<br>L103  HEAD laboon.WithNoHardLink :: func WithNoHardLink() HardenOption<br>L106  HEAD laboon.WithNoSymLink :: func WithNoSymLink() HardenOption<br>L109  HEAD laboon.WithNoDevice :: func WithNoDevice() HardenOption<br>L112  HEAD laboon.WithNoSocket :: func WithNoSocket() HardenOption<br>L115  HEAD laboon.WithNoFifo :: func WithNoFifo() HardenOption<br>L122  HEAD laboon.WithConfinedLinks :: func WithConfinedLinks() HardenOption` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/fs.go` | True |
| 6 | `HEAD/filesystems/hostfs/hostdirfs.go (1 signatures)` | `L37  HEAD hostfs.Dir (confined filesystem backend) :: func Dir(root string) core.FileSystem // confined, writable; at "/" a read-only image root with ephemeral writable upper; wrap with ReadOnly for a read-only bind` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/filesystems/hostfs/hostdirfs.go` | True |
| 7 | `HEAD/laboon.go (1 signatures)` | `L151  HEAD Run/RunImage/Start/StartImage/Process :: signatures unchanged from v0.1.0; imageRun error text changed to "a root filesystem (WithMount \"/\") does not apply to an image"; oci import moved internal/oci -> oci` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/laboon.go` | True |
| 8 | `v0.1.0/cmd/laboon/main.go (43 signatures)` | `L5  cli.usage :: laboon [-root dir] program [args...] \| laboon -image ref [command [args...]]<br>L141  cli.usage.flag.Usage :: usage: %s [flags] program [args...] / %s -image ref [command [args...]]<br>L105  cli.subcommand.build :: os.Args[1] == "build" -> buildCmd(os.Args[2:]); usage: laboon build [-t name:tag] [-f Dockerfile] [-o image.tar] [--platform P] [--target stage] [--net] [--build-arg NAME=VALUE] [--no-cache] [context]<br>L109  cli.subcommand.version :: os.Args[1] in {version, -version, --version} -> fmt.Println("laboon", version); version = "dev" unless set by -ldflags -X main.version<br>L113  cli.flag.-root :: root := flag.String("root", "", "directory the guest sees as / (read-only); the program path is inside it")<br>L114  cli.flag.-image :: image := flag.String("image", "", "OCI image to run, e.g. alpine:3.20; arguments replace its CMD")<br>L115  cli.flag.-entrypoint :: entrypoint := flag.String("entrypoint", "", "with -image: replace the image's ENTRYPOINT and drop its CMD, as docker run --entrypoint (empty clears it)")<br>L116  cli.flag.-pull :: pull := flag.Bool("pull", false, "with -image: check the registry for a newer image")<br>L117  cli.flag.-platform :: platform := flag.String("platform", "", "with -image: the image's platform, linux/amd64 or linux/arm64 (default: the host's architecture)")<br>L118  cli.flag.-trace :: trace := flag.Bool("trace", false, "log guest syscalls to stderr")<br>L119  cli.flag.-reject-rwx :: rejectRWX := flag.Bool("reject-rwx", false, "refuse writable+executable memory (W^X); JIT engines like V8 need it allowed")<br>L120  cli.flag.-mem :: maxMem := flag.String("mem", "", "cap guest address space, e.g. 256M, 1G (0 = unlimited)")<br>L121  cli.flag.-net :: net := flag.Bool("net", false, "enable networking (proxy guest sockets to host); without it the guest has loopback only")<br>L122  cli.flag.-no-net :: noNet := flag.Bool("no-net", false, "no network at all, not even loopback (without -net)")<br>L123  cli.flag.-runtime :: runtime := flag.String("runtime", "standalone", "where to run: standalone (in-process, no daemon), laboon (a laboon daemon over ConnectRPC), or docker (a Docker API endpoint)")<br>L124  cli.flag.-x11 :: x11 := flag.Bool("x11", false, "sugar for -mount /tmp/.X11-unix:/tmp/.X11-unix and passing DISPLAY, so GUI programs display on the host X server")<br>L125  cli.flag.-user :: user := flag.String("user", "", "run as this user (uid, uid:gid, name or name:group), overriding the image's USER, as docker --user")<br>L126  cli.flag.-u :: flag.StringVar(user, "u", "", "alias for -user")<br>L128  cli.flag.-mount :: flag.Var(&mounts, "mount", "bind a host path into the guest as HOST:GUEST (repeatable); GUEST=/ is the rootfs (-root), others are socket binds")<br>L129  cli.flag.-v :: flag.Var(&mounts, "v", "alias for -mount")<br>L131  cli.flag.-env :: flag.Var(&envVars, "env", "set a guest environment variable as KEY=VALUE (repeatable), on top of the image's environment")<br>L132  cli.flag.-e :: flag.Var(&envVars, "e", "alias for -env")<br>L133  cli.flag.-allow-domains :: allowDomains := flag.String("allow-domains", "", "with -net: comma-separated egress allowlist; the sandbox resolves only these domains (and subdomains) and lets the guest connect only to the addresses it resolved")<br>L134  cli.flag.-allow-cidrs :: allowCIDRs := flag.String("allow-cidrs", "", "with -net: comma-separated CIDR networks the guest may also connect to directly")<br>L135  cli.flag.-allow-ports :: allowPorts := flag.String("allow-ports", "", "with -net: comma-separated destination ports the guest may connect to (others refused)")<br>L136  cli.flag.-no-rewrite :: noRewrite := flag.Bool("no-rewrite", false, "skip syscall rewriting (seccomp should kill the guest)")<br>L137  cli.flag.-q :: quiet := flag.Bool("q", false, "discard the guest's stdout and stderr")<br>L138  cli.flag.-discard-stdout :: discardOut := flag.Bool("discard-stdout", false, "discard the guest's stdout")<br>L139  cli.flag.-discard-stderr :: discardErr := flag.Bool("discard-stderr", false, "discard the guest's stderr")<br>L32  cli.build.flag.-t :: fs.String("t", "laboon-build:latest", "name:tag for the built image")<br>L33  cli.build.flag.-f :: fs.String("f", "", "Dockerfile to use (default <context>/Dockerfile)")<br>L34  cli.build.flag.-o :: fs.String("o", "image.tar", "write the image here as a docker-load tarball")<br>L35  cli.build.flag.-platform :: fs.String("platform", "", "image platform, linux/amd64 or linux/arm64 (default: the host's)")<br>L36  cli.build.flag.-target :: fs.String("target", "", "build only up to this named stage (default: the last stage)")<br>L37  cli.build.flag.-net :: fs.Bool("net", false, "allow RUN steps to use the network")<br>L39  cli.build.flag.-build-arg :: fs.Var(&buildArgs, "build-arg", "set a build-time ARG as NAME=VALUE (repeatable)")<br>L40  cli.build.flag.-no-cache :: fs.Bool("no-cache", false, "do not reuse cached layers from previous builds")<br>L41  cli.build.flag.-cache :: fs.String("cache", "", "build layer cache directory (default: <user cache>/laboon/build)")<br>L47  cli.build.context :: ctxDir := fs.Arg(0); default "."; Dockerfile default <context>/Dockerfile<br>L153  cli.env.LABOON_CPUPROFILE :: os.Getenv("LABOON_CPUPROFILE") -> pprof.StartCPUProfile / StopCPUProfile<br>L229  cli.exit-codes :: os.Exit(code) = guest exit status; os.Exit(2) when no program given; os.Exit(125) from fail(err); os.Exit(1) on build errors<br>L145  cli.runtime.restriction :: switch *runtime: "standalone" proceeds; "docker"\|"laboon" calls fail("-runtime %s submits the run to a daemon, not yet wired in this build; run laboond (it serves the Docker API) and point a docker client at it"); default fails with "unknown -runtime %q: want standalone, laboon or docker"<br>L225  cli.dispatch :: with -image -> laboon.RunImage(ctx, *image, opts...), else laboon.Run(ctx, flag.Arg(0), append(opts, laboon.WithArgs(flag.Args()[1:]...))...)` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboon/main.go` | True |
| 9 | `v0.1.0/cmd/laboond/daemon.go (14 signatures)` | `L13  laboond.api.constants :: const ( apiVersion = "1.43"; minAPIVersion = "1.24"; laboonVersion = "0.1.0" )<br>L32  laboond.versionPrefix :: var versionPrefix = regexp.MustCompile(&#96;^/v[0-9]+\.[0-9]+&#96;); router strips it and sets headers Api-Version: 1.43, Ostype: linux<br>L38  laboond.route ping.GET :: mux.HandleFunc("GET /_ping", d.ping)<br>L39  laboond.route ping.HEAD :: mux.HandleFunc("HEAD /_ping", d.ping)<br>L40  laboond.route version :: mux.HandleFunc("GET /version", d.version)<br>L41  laboond.route info :: mux.HandleFunc("GET /info", d.info)<br>L43  laboond.route images/create :: mux.HandleFunc("POST /images/create", d.imageCreate)<br>L45  laboond.route containers/create :: mux.HandleFunc("POST /containers/create", d.containerCreate)<br>L46  laboond.route containers/json :: mux.HandleFunc("GET /containers/json", d.containerList)<br>L47  laboond.route containers/{id}/json :: mux.HandleFunc("GET /containers/{id}/json", d.containerInspect)<br>L48  laboond.route containers/{id}/start :: mux.HandleFunc("POST /containers/{id}/start", d.containerStart)<br>L49  laboond.route containers/{id}/wait :: mux.HandleFunc("POST /containers/{id}/wait", d.containerWait)<br>L50  laboond.route containers/{id}/attach :: mux.HandleFunc("POST /containers/{id}/attach", d.containerAttach)<br>L51  laboond.route containers/{id} :: mux.HandleFunc("DELETE /containers/{id}", d.containerDelete)` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/daemon.go` | True |
| 10 | `v0.1.0/cmd/laboond/main.go (1 signatures)` | `L20  laboond.flags :: flag.String("host", "unix:///run/laboon/laboond.sock", "address to serve the Docker API on (unix://path or tcp://host:port)"); flag.Bool("net", false, "allow containers to use the network")` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/main.go` | True |
| 11 | `v0.1.0/cmd/laboond/handlers.go (10 signatures)` | `L13  laboond.handler.imageCreate :: func (d *daemon) imageCreate(w http.ResponseWriter, r *http.Request) // query fromImage, tag; streams {"status": ...}; images are pulled lazily on run<br>L28  laboond.handler.createBody :: type createBody struct { Image string; Cmd []string; Entrypoint []string; Env []string; WorkingDir string; User string; Tty bool; AttachStdout bool; AttachStderr bool }<br>L40  laboond.handler.containerCreate :: func (d *daemon) containerCreate(w http.ResponseWriter, r *http.Request) // 201 {"Id":...,"Warnings":[]}<br>L60  laboond.handler.lookup :: func (d *daemon) lookup(r *http.Request) *container // r.PathValue("id"); unique-id prefix, exact name or "/"+name<br>L75  laboond.handler.containerStart :: func (d *daemon) containerStart(w http.ResponseWriter, r *http.Request) // 204; 304 if already started; 404 if unknown<br>L98  laboond.handler.containerWait :: func (d *daemon) containerWait(w http.ResponseWriter, r *http.Request) // 200 headers flushed, then {"StatusCode": n}<br>L118  laboond.handler.containerAttach :: func (d *daemon) containerAttach(w http.ResponseWriter, r *http.Request) // hijack, 101 UPGRADED or 200, raw-stream or multiplexed-stream, Docker stdcopy frames<br>L157  laboond.handler.containerDelete :: func (d *daemon) containerDelete(w http.ResponseWriter, r *http.Request) // 204; 409 if running unless force=1\|true<br>L173  laboond.handler.containerInspect :: func (d *daemon) containerInspect(w http.ResponseWriter, r *http.Request) // 200 {Id,Name,Created,Image,State{Status,Running,ExitCode},Config{...}}<br>L202  laboond.handler.containerList :: func (d *daemon) containerList(w http.ResponseWriter, r *http.Request) // query all=1\|true; 200 JSON array` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/handlers.go` | True |
| 12 | `v0.1.0/cmd/laboond/handshake.go (3 signatures)` | `L9  laboond.handshake.ping :: func (d *daemon) ping(w http.ResponseWriter, r *http.Request) // Builder-Version: 1, Docker-Experimental: false, body "OK"<br>L22  laboond.handshake.version :: func (d *daemon) version(w http.ResponseWriter, r *http.Request) // GoVersion: runtime.Version(), Os: linux, KernelVersion: laboon<br>L42  laboond.handshake.info :: func (d *daemon) info(w http.ResponseWriter, r *http.Request) // Driver: laboon-vfs, DefaultRuntime: laboon` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/handshake.go` | True |
| 13 | `v0.1.0/go.mod (2 signatures)` | `L3  go.version :: go 1.27.1 (root go.mod line 3; identical at v0.1.0 and HEAD)<br>L1  module.path :: module github.com/samyfodil/laboon` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/go.mod` | True |
| 14 | `HEAD/go.work (1 signatures)` | `L1  go.work :: go 1.27.1; use ( . ./cmd/laboon ./cmd/laboond ./cmd/laboon-runtime ./cmd/laboon-mcp )` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/go.work` | True |
| 15 | `v0.1.0/README.md (1 signatures)` | `L56  readme.build-requirement :: "To build from source, you need Go 1.27.1 or newer on Linux, macOS, or Windows,\non amd64 or arm64." (README.md lines 56-57)` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/README.md` | True |
| 16 | `HEAD/.github/workflows/release.yml (1 signatures)` | `L1  ci.go-version :: actions/setup-go@v5 with go-version-file: go.mod (release.yml and ci.yml)` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/.github/workflows/release.yml` | True |
| 17 | `v0.1.0/docs/permissions-and-limits.md (4 signatures)` | `L7  docs/permissions-and-limits.md networking-default :: Networking is off by default: the guest has loopback only, like &#96;docker --network none&#96;. &#96;WithNet()&#96; enables egress through supervisor-proxied host sockets. Add &#96;WithAllowDomains&#96;, &#96;WithAllowCIDRs&#96;, and &#96;WithAllowPorts&#96; to bound access; enabling networking alone does not create a restrictive allowlist.<br>L14  docs/permissions-and-limits.md windows-af_inet-exception :: The current Windows exception is that guest &#96;AF_INET&#96; sockets require &#96;WithNet&#96;, even for loopback.<br>L19  docs/permissions-and-limits.md filesystem-confined :: &#96;WithMount(host, guest)&#96; exposes a host directory read-write, confined to that subtree; &#96;WithMountReadOnly(host, guest)&#96; exposes it read-only (create, modify, delete and rename all fail with &#96;EROFS&#96;). A mount can also expose host Unix sockets beneath it, so a mount is an explicit grant of access.<br>L26  docs/permissions-and-limits.md memory-platform-limits :: Writable-plus-executable guest memory is allowed by default for JITs such as V8; &#96;WithRejectRWX()&#96; refuses it. There is no default memory cap. &#96;WithMaxMemory&#96; limits mapped address space on Linux and stub-process committed memory on Windows; **macOS does not enforce it yet**.` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/docs/permissions-and-limits.md` | True |
| 18 | `v0.1.0/.goreleaser.yaml (1 signatures)` | `L16  goreleaser.platform-matrix :: builds: goos [linux, darwin, windows] x goarch [amd64, arm64] for laboon and laboon-mcp; CGO_ENABLED=0; archives name_template laboon_{{ .Version }}_{{ .Os }}_{{ .Arch }} (tar.gz, zip on windows)` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/.goreleaser.yaml` | True |
| 19 | `HEAD/AGENTS.md (1 signatures)` | `L89  AGENTS.md.platform-matrix :: "Platforms are &#96;linux&#96;, &#96;darwin&#96;, &#96;windows&#96; x &#96;amd64&#96;, &#96;arm64&#96;."` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/AGENTS.md` | True |
| 20 | `release v0.1.0 assets (13 signatures)` | `release.v0.1.0.asset checksums.txt :: checksums.txt<br>release.v0.1.0.asset laboon-mcp_0.1.0_darwin_amd64.tar.gz :: laboon-mcp_0.1.0_darwin_amd64.tar.gz<br>release.v0.1.0.asset laboon-mcp_0.1.0_darwin_arm64.tar.gz :: laboon-mcp_0.1.0_darwin_arm64.tar.gz<br>release.v0.1.0.asset laboon-mcp_0.1.0_linux_amd64.tar.gz :: laboon-mcp_0.1.0_linux_amd64.tar.gz<br>release.v0.1.0.asset laboon-mcp_0.1.0_linux_arm64.tar.gz :: laboon-mcp_0.1.0_linux_arm64.tar.gz<br>release.v0.1.0.asset laboon-mcp_0.1.0_windows_amd64.zip :: laboon-mcp_0.1.0_windows_amd64.zip<br>release.v0.1.0.asset laboon-mcp_0.1.0_windows_arm64.zip :: laboon-mcp_0.1.0_windows_arm64.zip<br>release.v0.1.0.asset laboon_0.1.0_darwin_amd64.tar.gz :: laboon_0.1.0_darwin_amd64.tar.gz<br>release.v0.1.0.asset laboon_0.1.0_darwin_arm64.tar.gz :: laboon_0.1.0_darwin_arm64.tar.gz<br>release.v0.1.0.asset laboon_0.1.0_linux_amd64.tar.gz :: laboon_0.1.0_linux_amd64.tar.gz<br>release.v0.1.0.asset laboon_0.1.0_linux_arm64.tar.gz :: laboon_0.1.0_linux_arm64.tar.gz<br>release.v0.1.0.asset laboon_0.1.0_windows_amd64.zip :: laboon_0.1.0_windows_amd64.zip<br>release.v0.1.0.asset laboon_0.1.0_windows_arm64.zip :: laboon_0.1.0_windows_arm64.zip` | `https://api.github.com/repos/samyfodil/laboon/releases/latest (gh api .assets[].name)` | True |
| 21 | `release v0.1.0 checksums.txt (13 signatures)` | `release.v0.1.0.sha256 laboon-mcp_0.1.0_darwin_amd64.tar.gz :: sha256  1fb59221e4f2aab4ad5e19693bb28cecde9cabaf430aa81c97cb3d42e9a5590b  laboon-mcp_0.1.0_darwin_amd64.tar.gz<br>release.v0.1.0.sha256 laboon-mcp_0.1.0_darwin_arm64.tar.gz :: sha256  192a474acfc6f93ce77c7f132427bedba73f19b4afeb9b39a365053ca9fbaf88  laboon-mcp_0.1.0_darwin_arm64.tar.gz<br>release.v0.1.0.sha256 laboon-mcp_0.1.0_linux_amd64.tar.gz :: sha256  ada057ea8fdaac624914ead08923caeea62dafd4b8397c19d87b56409765b11a  laboon-mcp_0.1.0_linux_amd64.tar.gz<br>release.v0.1.0.sha256 laboon-mcp_0.1.0_linux_arm64.tar.gz :: sha256  c94c25c51788b15f298696a972656a10f66026a6c26c29403ed539e86e7fa728  laboon-mcp_0.1.0_linux_arm64.tar.gz<br>release.v0.1.0.sha256 laboon-mcp_0.1.0_windows_amd64.zip :: sha256  924d133ec38dba0eea580a1b0ff2ee89c7b41d815809ce6fc651455c85348d7a  laboon-mcp_0.1.0_windows_amd64.zip<br>release.v0.1.0.sha256 laboon-mcp_0.1.0_windows_arm64.zip :: sha256  43482e94b1563932591fbef1b5f3239885c8e02ddbc0403ec4d3131f70d6613d  laboon-mcp_0.1.0_windows_arm64.zip<br>release.v0.1.0.sha256 laboon_0.1.0_darwin_amd64.tar.gz :: sha256  bd3c10db9086d5c0c688c502b7a80607c62f5ad1a9ec510f9146d9fece6ba89e  laboon_0.1.0_darwin_amd64.tar.gz<br>release.v0.1.0.sha256 laboon_0.1.0_darwin_arm64.tar.gz :: sha256  1116d234f49b3f5f3c7ce363ddb98c4d084a4e6d11342a613386a4d89203f279  laboon_0.1.0_darwin_arm64.tar.gz<br>release.v0.1.0.sha256 laboon_0.1.0_linux_amd64.tar.gz :: sha256  842fe5620db4592498ff599a080c9f8f31b710a27face7ac2ffa13799d5312c8  laboon_0.1.0_linux_amd64.tar.gz<br>release.v0.1.0.sha256 laboon_0.1.0_linux_arm64.tar.gz :: sha256  8459d1d6a00df841d4fff3419beb02e20cf61a37357c4cb5f720ca5384c1955a  laboon_0.1.0_linux_arm64.tar.gz<br>release.v0.1.0.sha256 laboon_0.1.0_windows_amd64.zip :: sha256  ef9c67428b3cde1aecae2b3c90a123f56f4d70a02e7b62fa7ec517f474c14366  laboon_0.1.0_windows_amd64.zip<br>release.v0.1.0.sha256 laboon_0.1.0_windows_arm64.zip :: sha256  2e1a02a19882975ef8b0dc14b275f8625704b6b1a7861cfac8e52ff81f710071  laboon_0.1.0_windows_arm64.zip<br>release.v0.1.0.checksums.txt.sha256 :: sha256  ed592f16f49c3c8b74f44dc97401b640258f86dd7d8ffef78b8d17c60442abe8  checksums.txt (computed locally with shasum -a 256)` | `https://github.com/samyfodil/laboon/releases/download/v0.1.0/checksums.txt (curl, 1200 bytes, 12 lines)` | True |
| 22 | `v0.1.0/LICENSE (1 signatures)` | `L2  license.file :: Apache License, Version 2.0, January 2004; COPYRIGHT: Copyright 2026 Bunshin Labs LLC - bunshinlabs.com` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/LICENSE` | True |
| 23 | `v0.1.0/example_test.go (1 signatures)` | `L14  example.RunImage :: code, err := laboon.RunImage(context.Background(), "alpine:3.20", laboon.WithArgs("wget", "-qO-", "https://example.com"), laboon.WithStdio(os.Stdin, os.Stdout, os.Stderr), laboon.WithNet(), laboon.WithAllowDomains("example.com"), laboon.WithMaxMemory(512<<20))` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/example_test.go` | True |
| 24 | `HEAD/internal/sandbox/egress.go (1 signatures)` | `L15  egress.DNS-policy :: egress: allowDomains []string (empty = all), allowCIDRs []*net.IPNet, allowPorts []int; pinTTL = 2 * time.Minute; supervisor answers :53 DNS; domainAllowed exact-or-subdomain; internalIP blocks loopback/private/link-local/unspecified unless explicit CIDR; connectAllowed never permits host loopback` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/internal/sandbox/egress.go` | True |
| 25 | `HEAD/internal/sandbox/config.go (1 signatures)` | `L32  egress.Config-fields :: type Config struct { ... Net bool; NoNet bool; AllowDomains []string; AllowCIDRs []string; AllowPorts []int; MaxBytes uint64; RootFS core.FileSystem; Mounts []Mount ... }` | `https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/internal/sandbox/config.go` | True |
| 26 | `v0.1.0/internal/sandbox/config.go (2 signatures)` | `L125  v0.1.0.Mount :: type Mount struct { Host, Guest string; ReadOnly bool } // bind the host directory read-only (EROFS on any change)<br>L46  v0.1.0.Config.Root :: Root string // "Root is the directory the guest sees as \"/\", read-only"` | `https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/internal/sandbox/config.go` | True |

**Contract source refs**

- `https://api.github.com/repos/samyfodil/laboon (gh api; license Apache-2.0, default_branch main)`
- `https://api.github.com/repos/samyfodil/laboon/git/trees/HEAD?recursive=1 (751 blobs, tree sha ad7a15ba1997d6fb3ec24b1970a75015f40285ce)`
- `https://api.github.com/repos/samyfodil/laboon/git/trees/72b4fb91209d8d9576749da097c3fda4f9365f6f?recursive=1 (v0.1.0 tree)`
- `https://api.github.com/repos/samyfodil/laboon/commits/main (HEAD commit ad7a15ba1997d6fb3ec24b1970a75015f40285ce)`
- `https://api.github.com/repos/samyfodil/laboon/git/ref/tags/v0.1.0 and /git/tags/049f6c2a764f0e51556a560b575120b5f0f122e0`
- `https://api.github.com/repos/samyfodil/laboon/releases and /releases/latest (one release, v0.1.0, 13 assets)`
- `https://github.com/samyfodil/laboon/releases/download/v0.1.0/checksums.txt (curl; local copy /tmp/laboon-work/data/checksums.txt, sha256 ed592f16f49c3c8b74f44dc97401b640258f86dd7d8ffef78b8d17c60442abe8)`
- `https://proxy.golang.org/github.com/samyfodil/laboon/@v/list and /@v/v0.1.0.info`
- `https://index.crates.io/la/bo/laboon (404), https://index.crates.io/la/bo/laboon-cli (404), https://docs.rs/laboon/latest/laboon/ (404)`
- `gh api repos/samyfodil/laboon/contents/go.mod?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/go.mod?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/go.work?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/options.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/options.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/laboon.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/laboon.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/fs.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/example_test.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/README.md?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/README.md?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/llms.txt?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/AGENTS.md?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/LICENSE?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/COPYRIGHT?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/docs/permissions-and-limits.md?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/docs/permissions-and-limits.md?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/.goreleaser.yaml?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/.github/workflows/release.yml?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/.github/workflows/ci.yml?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboon/main.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboon/main.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboon/go.mod?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboond/main.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboond/daemon.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboond/handlers.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboond/handshake.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboond/container.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboond/go.mod?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboon-mcp/go.mod?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/cmd/laboon-runtime/go.mod?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/internal/sandbox/config.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/internal/sandbox/config.go?ref=72b4fb91209d8d9576749da097c3fda4f9365f6f (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/internal/sandbox/egress.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/filesystems/hostfs/hostdirfs.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `gh api repos/samyfodil/laboon/contents/filesystems/hostfs/dir.go?ref=ad7a15ba1997d6fb3ec24b1970a75015f40285ce (base64-decoded)`
- `https://api.github.com/search/code?q=WithMountReadOnly+repo:samyfodil/laboon and ?q=WithRoot and ?q=WithX11 (all total_count=0 at HEAD)`
- `/tmp/laboon-work/data/head/ (decoded HEAD sources), /tmp/laboon-work/data/tag/ (decoded v0.1.0 sources), /tmp/laboon-work/scripts/fetch.sh (fetch helper)`

### 1.13 `quic-rpc` -- exact version `0.20.0`

- **License:** Apache-2.0/MIT -- Cargo.toml line 8 `license = "Apache-2.0/MIT"`; crates.io API reports `license = "Apache-2.0/MIT"`; dual LICENSE-APACHE + LICENSE-MIT present in the tag; GitHub license detection returns NOASSERTION (slash form is not a valid SPDX expression).
- **MSRV / Go:** Rust only. rust-version = "1.76" (MSRV, Cargo.toml:13 and crates.io index rust_version=1.76), edition = "2021". No go.mod / Go version. Uses RPITIT (`impl Future` in traits), stable since 1.75.
- **Signatures:** 37; **unverified items:** 0; **source refs:** 15

**Feature flags / version gates**

- `default = ["flume-transport"]`
- `flume-transport = ["dep:flume"]`
- `quinn-transport = ["dep:flume", "dep:quinn", "dep:postcard", "dep:bytes", "dep:tokio-serde", "tokio-util/codec"]`
- `hyper-transport = ["dep:flume", "dep:hyper", "dep:postcard", "dep:bytes", "dep:tokio-serde", "tokio-util/codec"]`
- `iroh-transport = ["dep:iroh", "dep:flume", "dep:postcard", "dep:tokio-serde", "tokio-util/codec"]`
- `macros = []`
- `test-utils = ["dep:rcgen", "dep:rustls"]`

**Integration notes (verbatim)**

````text
Source: all signatures read from GitHub tag v0.20.0 (5ed68d4323daa5cfb29539e47f981f0fab80c233) via gh api contents?ref=v0.20.0 (base64 decoded, each blob SHA-1 checked against the tree); the published quic-rpc-0.20.0.crate has SHA-256 18bad98bd048264ceb1361ff9d77a031535d8c1e3fe8f12c6966ec825bf68eb7 matching https://index.crates.io/qu/ic/quic-rpc (yanked=false, rust_version=1.76, pubtime=2025-05-12T16:51:10Z), and all 21 src/**/*.rs are byte-identical to the tag. signature.source = <tag>:<path>:<lines>, resolved against /Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/ and https://github.com/n0-computer/quic-rpc/blob/v0.20.0/. docs.rs/0.20.0 pages in source_refs confirm RpcClient/RpcServer/RpcChannel/Service/Msg/QuinnListener/QuinnConnector. No `Message` trait exists: use Service, RpcMessage, message::Msg<S> + InteractionPattern, and RpcMsg/ServerStreamingMsg/ClientStreamingMsg/BidiStreamingMsg/TryServerStreamingMsg (only RpcMsg has a blanket Msg impl). The `quinn` dep is package="iroh-quinn", version="0.13" renamed to quinn -- endpoints MUST be iroh_quinn::Endpoint, not upstream quinn. Server: make_server_endpoint(bind_addr) [test-utils] -> QuinnListener::<In,Out>::new(endpoint) -> RpcServer::new -> accept()/read_first() or accept_loop/spawn_accept_loop/run_server_loop. Client: make_client_endpoint(bind_addr,&[&cert_der]) -> QuinnConnector::<In,Out>::new(endpoint,addr,"localhost".into()) -> RpcClient::new; or from_connection(quinn::Connection). quinn_channel() is gated on test-utils only but needs quinn-transport too (lib.rs:199-219): enable both. Wire: length-delimited postcard, private 16 MiB MAX_FRAME_LENGTH, io::Error send/recv, quinn::ConnectionError open/accept; into_inner() escapes to raw streams. macros caveat: __derive_create_client is never invoked and __rpc_method! expands to stale client::* error names absent in 0.20.0; prefer hand-written impls. Typed RPC: RpcClient::rpc(&self) / RpcChannel::rpc(self,req,target,f); streaming: server_streaming/client_streaming/bidi/try_server_streaming with UpdateSink/UpdateStream/BoxStreamSync/StreamCreated. Limits: source-level verification, no cargo build/test; nothing from memory or newer HEAD.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `quic_rpc::Service` | `pub trait Service: Send + Sync + Debug + Clone + 'static {<br>    type Req: RpcMessage;<br>    type Res: RpcMessage;<br>}` | `v0.20.0:src/lib.rs:161-166` | True |
| 2 | `quic_rpc::RpcMessage` | `pub trait RpcMessage: Debug + Serialize + DeserializeOwned + Send + Sync + Unpin + 'static {}<br><br>impl<T> RpcMessage for T where<br>    T: Debug + Serialize + DeserializeOwned + Send + Sync + Unpin + 'static<br>{<br>}` | `v0.20.0:src/lib.rs:120-125` | True |
| 3 | `quic_rpc::message::Msg<S>` | `pub trait Msg<S: Service>: Into<S::Req> + TryFrom<S::Req> + Send + 'static {<br>    type Pattern: InteractionPattern;<br>}` | `v0.20.0:src/message.rs:17-20` | True |
| 4 | `quic_rpc::message::InteractionPattern` | `pub trait InteractionPattern: Debug + Clone + Send + Sync + 'static {}` | `v0.20.0:src/message.rs:31-31` | True |
| 5 | `quic_rpc::Connector<S>` | `pub trait Connector<S: Service>: transport::Connector<In = S::Res, Out = S::Req> {}<br><br>impl<T: transport::Connector<In = S::Res, Out = S::Req>, S: Service> Connector<S> for T {}` | `v0.20.0:src/lib.rs:173-175` | True |
| 6 | `quic_rpc::Listener<S>` | `pub trait Listener<S: Service>: transport::Listener<In = S::Req, Out = S::Res> {}<br><br>impl<T: transport::Listener<In = S::Req, Out = S::Res>, S: Service> Listener<S> for T {}` | `v0.20.0:src/lib.rs:182-184` | True |
| 7 | `quic_rpc::pattern::rpc::RpcMsg<S>` | `pub trait RpcMsg<S: Service>: Msg<S, Pattern = Rpc> {<br>    type Response: Into<S::Res> + TryFrom<S::Res> + Send + 'static;<br>}<br><br>impl<T: RpcMsg<S>, S: Service> Msg<S> for T {<br>    type Pattern = Rpc;<br>}` | `v0.20.0:src/pattern/rpc.rs:26-41` | True |
| 8 | `quic_rpc::pattern::server_streaming::ServerStreamingMsg<S>` | `pub trait ServerStreamingMsg<S: Service>: Msg<S, Pattern = ServerStreaming> {<br>    type Response: Into<S::Res> + TryFrom<S::Res> + Send + 'static;<br>}` | `v0.20.0:src/pattern/server_streaming.rs:27-33` | True |
| 9 | `quic_rpc::pattern::client_streaming::ClientStreamingMsg<S>` | `pub trait ClientStreamingMsg<S: Service>: Msg<S, Pattern = ClientStreaming> {<br>    type Update: Into<S::Req> + TryFrom<S::Req> + Send + 'static;<br>    type Response: Into<S::Res> + TryFrom<S::Res> + Send + 'static;<br>}` | `v0.20.0:src/pattern/client_streaming.rs:28-40` | True |
| 10 | `quic_rpc::pattern::bidi_streaming::BidiStreamingMsg<S>` | `pub trait BidiStreamingMsg<S: Service>: Msg<S, Pattern = BidiStreaming> {<br>    type Update: Into<S::Req> + TryFrom<S::Req> + Send + 'static;<br>    type Response: Into<S::Res> + TryFrom<S::Res> + Send + 'static;<br>}` | `v0.20.0:src/pattern/bidi_streaming.rs:28-40` | True |
| 11 | `quic_rpc::client::RpcClient` | `#[derive(Debug)]<br>pub struct RpcClient<S, C = BoxedConnector<S>> {<br>    pub(crate) source: C,<br>    pub(crate) _p: PhantomData<S>,<br>}<br><br>impl<S, C: Clone> Clone for RpcClient<S, C> { ... }` | `v0.20.0:src/client.rs:60-73` | True |
| 12 | `quic_rpc::client::RpcClient::new` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S, C> RpcClient<S, C> where S: Service, C: Connector<S><br>pub fn new(source: C) -> Self` | `v0.20.0:src/client.rs:119-136` | True |
| 13 | `quic_rpc::client::RpcClient::rpc` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S, C> RpcClient<S, C> where S: Service, C: Connector<S><br>pub async fn rpc<M>(&self, msg: M) -> result::Result<M::Response, Error<C>><br>where<br>    M: RpcMsg<S>,` | `v0.20.0:src/pattern/rpc.rs:65-87` | True |
| 14 | `quic_rpc::client::RpcClient::server_streaming` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S, C> RpcClient<S, C> where C: crate::Connector<S>, S: Service<br>pub async fn server_streaming<M>(<br>    &self,<br>    msg: M,<br>) -> result::Result<BoxStreamSync<'static, result::Result<M::Response, ItemError<C>>>, Error<C>><br>where<br>    M: ServerStreamingMsg<S>,` | `v0.20.0:src/pattern/server_streaming.rs:69-93` | True |
| 15 | `quic_rpc::client::RpcClient::client_streaming` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S, C> RpcClient<S, C> where S: Service, C: Connector<S><br>pub async fn client_streaming<M>(<br>    &self,<br>    msg: M,<br>) -> result::Result<<br>    (<br>        UpdateSink<C, M::Update>,<br>        Boxed<result::Result<M::Response, ItemError<C>>>,<br>    ),<br>    Error<C>,<br>><br>where<br>    M: ClientStreamingMsg<S>,` | `v0.20.0:src/pattern/client_streaming.rs:78-112` | True |
| 16 | `quic_rpc::client::RpcClient::bidi` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S, C> RpcClient<S, C> where S: Service, C: Connector<In = S::Res, Out = S::Req><br>pub async fn bidi<M>(<br>    &self,<br>    msg: M,<br>) -> result::Result<<br>    (<br>        UpdateSink<C, M::Update>,<br>        BoxStreamSync<'static, result::Result<M::Response, ItemError<C>>>,<br>    ),<br>    Error<C>,<br>><br>where<br>    M: BidiStreamingMsg<S>,` | `v0.20.0:src/pattern/bidi_streaming.rs:76-105` | True |
| 17 | `quic_rpc::server::RpcServer` | `#[derive(Debug)]<br>pub struct RpcServer<S, C = BoxedListener<S>> {<br>    source: C,<br>    _p: PhantomData<S>,<br>}<br><br>impl<S, C: Clone> Clone for RpcServer<S, C> { ... }` | `v0.20.0:src/server.rs:81-98` | True |
| 18 | `quic_rpc::server::RpcServer::new` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S: Service, C: Listener<S>> RpcServer<S, C><br>pub fn new(source: C) -> Self` | `v0.20.0:src/server.rs:100-110` | True |
| 19 | `quic_rpc::server::RpcServer::accept` | `pub async fn accept(&self) -> result::Result<Accepting<S, C>, RpcServerError<C>>` | `v0.20.0:src/server.rs:225-235` | True |
| 20 | `quic_rpc::server::RpcServer::accept_loop` | `pub async fn accept_loop<Fun, Fut, E>(self, handler: Fun)<br>where<br>    S: Service,<br>    C: Listener<S>,<br>    Fun: Fn(S::Req, RpcChannel<S, C>) -> Fut + Send + Sync + 'static,<br>    Fut: Future<Output = Result<(), E>> + Send + 'static,<br>    E: Into<anyhow::Error> + 'static,` | `v0.20.0:src/server.rs:242-254` | True |
| 21 | `quic_rpc::server::Accepting<S, C>` | `pub struct Accepting<S: Service, C: Listener<S>> {<br>    send: C::SendSink,<br>    recv: C::RecvStream,<br>    _p: PhantomData<S>,<br>}<br><br>impl<S: Service, C: Listener<S>> Accepting<S, C> {<br>    pub async fn read_first(self) -> result::Result<(S::Req, RpcChannel<S, C>), RpcServerError<C>><br>}` | `v0.20.0:src/server.rs:194-223` | True |
| 22 | `quic_rpc::server::RpcChannel<S, C>` | `#[derive(Debug)]<br>pub struct RpcChannel<S: Service, C: ChannelTypes<S> = BoxedChannelTypes<S>> {<br>    pub send: C::SendSink,<br>    pub recv: C::RecvStream,<br>    pub(crate) _p: PhantomData<S>,<br>}` | `v0.20.0:src/server.rs:132-144` | True |
| 23 | `quic_rpc::server::RpcChannel::rpc` | `// context (source writes this as a multi-line impl/where clause)<br>impl<S, C> RpcChannel<S, C> where S: Service, C: StreamTypes<In = S::Req, Out = S::Res><br>pub async fn rpc<M, F, Fut, T>(<br>    self,<br>    req: M,<br>    target: T,<br>    f: F,<br>) -> result::Result<(), RpcServerError<C>><br>where<br>    M: RpcMsg<S>,<br>    F: FnOnce(T, M) -> Fut,<br>    Fut: Future<Output = M::Response>,<br>    T: Send + 'static,` | `v0.20.0:src/pattern/rpc.rs:89-126` | True |
| 24 | `quic_rpc::server::RpcChannel::server_streaming` | `pub async fn server_streaming<M, F, Str, T>(<br>    self,<br>    req: M,<br>    target: T,<br>    f: F,<br>) -> result::Result<(), RpcServerError<C>><br>where<br>    M: ServerStreamingMsg<S>,<br>    F: FnOnce(T, M) -> Str + Send + 'static,<br>    Str: Stream<Item = M::Response> + Send + 'static,<br>    T: Send + 'static,` | `v0.20.0:src/pattern/server_streaming.rs:95-139` | True |
| 25 | `quic_rpc::server::RpcChannel::client_streaming` | `pub async fn client_streaming<M, F, Fut, T>(<br>    self,<br>    req: M,<br>    target: T,<br>    f: F,<br>) -> result::Result<(), RpcServerError<C>><br>where<br>    M: ClientStreamingMsg<S>,<br>    F: FnOnce(T, M, UpdateStream<C, M::Update>) -> Fut + Send + 'static,<br>    Fut: Future<Output = M::Response> + Send + 'static,<br>    T: Send + 'static,` | `v0.20.0:src/pattern/client_streaming.rs:114-146` | True |
| 26 | `quic_rpc::server::RpcChannel::bidi_streaming` | `pub async fn bidi_streaming<M, F, Str, T>(<br>    self,<br>    req: M,<br>    target: T,<br>    f: F,<br>) -> result::Result<(), RpcServerError<C>><br>where<br>    M: BidiStreamingMsg<S>,<br>    F: FnOnce(T, M, UpdateStream<C, M::Update>) -> Str + Send + 'static,<br>    Str: Stream<Item = M::Response> + Send + 'static,<br>    T: Send + 'static,` | `v0.20.0:src/pattern/bidi_streaming.rs:107-146` | True |
| 27 | `quic_rpc::server::run_server_loop` | `pub async fn run_server_loop<S, C, T, F, Fut>(<br>    _service_type: S,<br>    conn: C,<br>    target: T,<br>    mut handler: F,<br>) -> Result<(), RpcServerError<C>><br>where<br>    S: Service,<br>    C: Listener<S>,<br>    T: Clone + Send + 'static,<br>    F: FnMut(RpcChannel<S, C>, S::Req, T) -> Fut + Send + 'static,<br>    Fut: Future<Output = Result<(), RpcServerError<C>>> + Send + 'static,` | `v0.20.0:src/server.rs:474-496` | True |
| 28 | `quic_rpc::server::RpcServerError<C>` | `pub enum RpcServerError<C: ConnectionErrors> {<br>    Accept(C::AcceptError),<br>    EarlyClose,<br>    UnexpectedStartMessage,<br>    RecvError(C::RecvError),<br>    SendError(C::SendError),<br>    UnexpectedUpdateMessage,<br>}` | `v0.20.0:src/server.rs:375-389` | True |
| 29 | `quic_rpc::transport::Connector` | `pub trait Connector: StreamTypes {<br>    fn open(<br>        &self,<br>    ) -> impl Future<Output = Result<(Self::SendSink, Self::RecvStream), Self::OpenError>> + Send;<br><br>    fn map<In1, Out1>(self) -> MappedConnector<In1, Out1, Self><br>    where<br>        In1: TryFrom<Self::In>,<br>        Self::Out: From<Out1>,<br>    { MappedConnector::new(self) }<br><br>    fn boxed(self) -> BoxedConnector<Self::In, Self::Out><br>    where<br>        Self: BoxableConnector<Self::In, Self::Out> + Sized + 'static,<br>    { self::BoxedConnector::new(self) }<br>}` | `v0.20.0:src/transport/mod.rs:86-111` | True |
| 30 | `quic_rpc::transport::Listener` | `pub trait Listener: StreamTypes {<br>    fn accept(<br>        &self,<br>    ) -> impl Future<Output = Result<(Self::SendSink, Self::RecvStream), Self::AcceptError>> + Send;<br><br>    fn local_addr(&self) -> &[LocalAddr];<br><br>    fn boxed(self) -> BoxedListener<Self::In, Self::Out><br>    where<br>        Self: BoxableListener<Self::In, Self::Out> + Sized + 'static,<br>    { BoxedListener::new(self) }<br>}` | `v0.20.0:src/transport/mod.rs:113-134` | True |
| 31 | `quic_rpc::transport::StreamTypes` | `pub trait StreamTypes: ConnectionErrors {<br>    type In: RpcMessage;<br>    type Out: RpcMessage;<br>    type RecvStream: Stream<Item = Result<Self::In, Self::RecvError>><br>        + Send<br>        + Sync<br>        + Unpin<br>        + 'static;<br>    type SendSink: Sink<Self::Out, Error = Self::SendError> + Send + Sync + Unpin + 'static;<br>}` | `v0.20.0:src/transport/mod.rs:68-84` | True |
| 32 | `quic_rpc::transport::quinn::QuinnListener::new` | `// context (source writes this as a multi-line impl/where clause)<br>impl<In: RpcMessage, Out: RpcMessage> QuinnListener<In, Out><br>pub fn new(endpoint: quinn::Endpoint) -> io::Result<Self>` | `v0.20.0:src/transport/quinn.rs:119-138` | True |
| 33 | `quic_rpc::transport::quinn::QuinnConnector::new` | `pub fn new(endpoint: quinn::Endpoint, addr: SocketAddr, name: String) -> Self` | `v0.20.0:src/transport/quinn.rs:454-471` | True |
| 34 | `quic_rpc::transport::quinn::QuinnConnector::from_connection` | `pub fn from_connection(connection: quinn::Connection) -> Self` | `v0.20.0:src/transport/quinn.rs:440-452` | True |
| 35 | `quic_rpc::transport::quinn test-utils helpers` | `#[cfg(feature = "test-utils")]<br>pub fn configure_client(server_certs: &[&[u8]]) -> Result<ClientConfig><br>#[cfg(feature = "test-utils")]<br>pub fn make_client_endpoint(bind_addr: SocketAddr, server_certs: &[&[u8]]) -> Result<Endpoint><br>#[cfg(feature = "test-utils")]<br>pub fn make_server_endpoint(bind_addr: SocketAddr) -> Result<(Endpoint, Vec<u8>)><br>#[cfg(feature = "test-utils")]<br>pub fn configure_server() -> anyhow::Result<(ServerConfig, Vec<u8>)><br>#[cfg(feature = "test-utils")]<br>pub fn make_insecure_client_endpoint(bind_addr: SocketAddr) -> Result<Endpoint><br>#[cfg(feature = "test-utils")]<br>pub use quinn_setup_utils::*;` | `v0.20.0:src/transport/quinn.rs:807-948` | True |
| 36 | `quic_rpc::flume_channel` | `#[cfg(feature = "flume-transport")]<br>pub fn flume_channel<S: Service>(<br>    size: usize,<br>) -> (<br>    RpcServer<S, server::FlumeListener<S>>,<br>    RpcClient<S, client::FlumeConnector<S>>,<br>)` | `v0.20.0:src/lib.rs:186-197` | True |
| 37 | `quic_rpc::quinn_channel` | `#[cfg(feature = "test-utils")]<br>#[allow(clippy::type_complexity)]<br>pub fn quinn_channel<S: Service>() -> anyhow::Result<(<br>    RpcServer<S, server::QuinnListener<S>>,<br>    RpcClient<S, client::QuinnConnector<S>>,<br>)>` | `v0.20.0:src/lib.rs:199-219` | True |

**Contract source refs**

- `signature.source = <tag>:<repo-relative path>:<line range>; absolute mirror root /Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/ and canonical https://github.com/n0-computer/quic-rpc/blob/v0.20.0/<path>#L<a>-L<b>`
- `https://index.crates.io/qu/ic/quic-rpc (curl; full 0.20.0 index line, features2, rust_version, cksum, pubtime)`
- `https://static.crates.io/crates/quic-rpc/quic-rpc-0.20.0.crate (curl; sha256 18bad98bd048264ceb1361ff9d77a031535d8c1e3fe8f12c6966ec825bf68eb7 matches index cksum)`
- `gh api repos/n0-computer/quic-rpc/git/trees/v0.20.0?recursive=1 (64 blobs, truncated=false)`
- `gh api repos/n0-computer/quic-rpc/contents/<path>?ref=v0.20.0 for all 64 paths (base64 decoded; each content SHA-1 checked against its git tree blob sha)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/lib.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/client.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/server.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/message.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/macros.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/pattern/{mod,rpc,server_streaming,client_streaming,bidi_streaming,try_server_streaming}.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/quic-rpc-src-v0.20.0/src/transport/{mod,quinn,flume,boxed,mapped,combined,misc/mod}.rs`
- `https://docs.rs/quic-rpc/0.20.0/quic_rpc/client/struct.RpcClient.html`
- `https://docs.rs/quic-rpc/0.20.0/quic_rpc/server/struct.RpcServer.html`
- `https://docs.rs/quic-rpc/0.20.0/quic_rpc/transport/quinn/struct.QuinnListener.html`

### 1.14 `iroh` -- exact version `1.5.0`

- **License:** MIT OR Apache-2.0
- **MSRV / Go:** Rust 1.91 (rust-version = "1.91" in iroh/Cargo.toml at tag v1.5.0; crates.io index rust_version = "1.91"; iroh-published-1.5.0 Cargo.toml edition = "2024"). Rust crate only, no Go version. Companion crates also MSRV 1.91: iroh-base 1.5.0, iroh-tickets 1.0.0, n0-watcher 1.0.0/1.0.1.
- **Signatures:** 81; **unverified items:** 9; **source refs:** 36

**Feature flags / version gates**

- `default = ["metrics", "fast-apple-datapath", "portmapper", "tls-ring"]`
- `metrics`
- `fast-apple-datapath`
- `portmapper`
- `platform-verifier`
- `qlog`
- `tls-ring`
- `tls-aws-lc-rs`
- `test-utils`
- `unstable-custom-transports`
- `unstable-net-report`

**Integration notes (verbatim)**

````text
VERSION/PROVENANCE: exact published version is iroh 1.5.0 (crates.io index line, not yanked, sha256 40ac5d8330439176254581b6db4bc6f1d947d7cc741fda4e04b96d758addf1db). GitHub tag v1.5.0 = commit ea3bbb6ab41fa23816d7f9e3220ab77d444435b2. The published tarball's src/lib.rs, src/endpoint.rs, src/protocol.rs, src/endpoint/connection.rs, src/endpoint/presets.rs and src/address_lookup.rs are byte-identical to the tag files fetched via gh api contents, so every signature below is the published API, not just HEAD. License MIT OR Apache-2.0; edition 2024; rust-version 1.91 (Rust 1.91+ required). Cargo.toml default features = [metrics, fast-apple-datapath, portmapper, tls-ring]; index splits these across `features` and `features2` (default/portmapper/test-utils live in features2). Declare `iroh = "=1.5.0"` (or "1.5.0") plus `n0-error` (errors/Result), `n0-future` (StreamExt for Watcher streams), `tokio` (runtime/io) and `rustls` if you must supply your own crypto provider.

CRITICAL API DRIFT vs the requested names: iroh 1.5.0 is a 1.x API that renamed the whole Node vocabulary in v0.94/1.0. `NodeId` -> `EndpointId` (= `pub type EndpointId = PublicKey`), `NodeAddr` -> `EndpointAddr`, `NodeTicket` -> `iroh_tickets::endpoint::EndpointTicket` (moved OUT of this crate into the separate `iroh-tickets` 1.0.0 crate), and `Discovery` -> `iroh::address_lookup::AddressLookup` / `AddressLookupBuilder`. CHANGELOG.md:727, :826, :828 at tag v1.5.0 record all three breaking changes. docs.rs/iroh/1.5.0/iroh/index.html lists no ticket module and no NodeAddr/NodeId/NodeTicket/Discovery/Router at the crate root; `Router` is `iroh::protocol::Router`. The full recursive tree at v1.5.0 contains no ticket.rs anywhere.

BUILDER: `Endpoint::builder()` with zero arguments DOES NOT EXIST. It is `Endpoint::builder(preset: impl Preset) -> Builder`, and `Endpoint::bind(preset: impl Preset) -> Result<Self, BindError>` is an async one-shot constructor. Use `presets::N0` (n0 DNS/Pkarr address lookup + production relays + crypto provider), `presets::Minimal`, `presets::N0DisableRelay`, or `presets::Empty` (which requires an explicit `Builder::crypto_provider(Arc<rustls::crypto::CryptoProvider>)` or `bind()` fails with `BindError::InvalidCryptoProvider`, endpoint.rs:235-241). Minimal/N0/N0DisableRelay are `#[cfg(with_crypto_provider)]`, i.e. only compiled when `tls-ring` or `tls-aws-lc-rs` is enabled; default features include `tls-ring`. `relay_mode` and `alpns` are Builder methods (`Builder::relay_mode(RelayMode)`, `Builder::alpns(Vec<Vec<u8>>)`), not Endpoint methods; on a live Endpoint use `Endpoint::set_alpns(&self, Vec<Vec<u8>>)`. `Builder::bind_addr` is cfg(not(wasm_browser)).

ROUTER: `Router::builder(endpoint).accept(ALPN, handler).spawn()` is exactly right (protocol.rs:412, :495, :512), but: `accept` takes `alpn: impl AsRef<[u8]>` and `handler: impl Into<Box<dyn DynProtocolHandler>>` (a blanket `From<T: ProtocolHandler>` exists); ALPN registration order is preference order; `spawn()` itself calls `Endpoint::set_alpns(...)` with the registered ALPNs, so do not also set them manually unless you know you are overriding them; `Router` aborts its accept loop when dropped, so keep it and `await router.shutdown()` (returns `Result<(), n0_future::task::JoinError>`). `RouterBuilder::incoming_filter` (new in this line) can Accept/Retry/Reject/Ignore pre-handshake.

PROTOCOL HANDLER: the trait has exactly three methods: required `accept(&self, Connection) -> impl Future<Output = Result<(), AcceptError>> + Send`, plus provided `on_accepting(&self, Accepting) -> impl Future<Output = Result<Connection, AcceptError>> + Send` (default just awaits `Accepting`, used for 0-RTT via `Accepting::into_0rtt`) and `shutdown(&self) -> impl Future<Output = ()> + Send`. `handle_connection` is NOT a trait method: it is a private, non-pub free function `async fn handle_connection(incoming: Incoming, protocols: Arc<ProtocolMap>)` at protocol.rs:636. `DynProtocolHandler` is the boxed-future variant; `Arc<T>` and `Box<T>` implement ProtocolHandler. Call `connection.close(code, reason)` inside `accept` to send an error code to the peer; returning Err only drops/logs.

CONNECTION/STREAMS: `Connection<State: ConnectionState = HandshakeCompleted>`; `open_bi()`/`accept_bi()` are NOT async fns, they return the futures `OpenBi<'_>`/`AcceptBi<'_>` (re-exported from noq 1.5.0 via src/endpoint/quic.rs). Awaiting yields `Result<(SendStream, RecvStream), ConnectionError>` (docs.rs/noq/1.5.0/noq/struct.OpenBi.html and struct.AcceptBi.html). Streams are lazy: the sender MUST write data before the peer's `accept_bi()` resolves (lib.rs:150-157, connection.rs:850-852). On `Connection<HandshakeCompleted>`: `alpn(&self) -> &[u8]`, `remote_id(&self) -> EndpointId`, `close(&self, VarInt, &[u8])`, `closed().await -> ConnectionError`.

ACCEPT FLOW without Router: `Endpoint::accept()` -> `Accept<'_>` (Future<Output = Option<Incoming>>) -> `Incoming::accept(self) -> Result<Accepting, ConnectionError>` -> await `Accepting` (Future<Output = Result<Connection, ConnectingError>>). With Router this chain is handled for you. `Endpoint::connect(addr, alpn)` takes `impl Into<EndpointAddr>`; because `impl From<EndpointId> for EndpointAddr` exists, you may dial with just an `EndpointId` when address lookup is configured (e.g. presets::N0).

ADDRESSING/TICKETS: `EndpointAddr { pub id: EndpointId, pub addrs: BTreeSet<TransportAddr> }` with `new`, `from_parts`, `with_relay_url`, `with_ip_addr`, `with_addrs`, `is_empty`, `ip_addrs`, `relay_urls`. It derives Serialize/Deserialize but has NO Display and NO FromStr in iroh-base 1.5.0. For a shareable string, add the separate crate `iroh-tickets = "1.0.0"` (depends on `iroh-base ^1.0.0`, which unifies with iroh 1.5.0's re-exported iroh-base 1.5.0): `EndpointTicket::new(endpoint.addr())`, `ticket.encode_string()` (via `use iroh_tickets::Ticket;`, string is lowercase `endpoint` prefix + unpadded base32), `EndpointTicket::from_str(s)?`, `EndpointAddr::from(ticket)` -> `endpoint.connect(addr, ALPN)`. `iroh::Watcher` is a re-export of `n0_watcher::Watcher` (iroh 1.5.0 declares `n0-watcher = "1.0.0"`, caret; I read both published 1.0.0 and 1.0.1 tarballs and their Watcher trait blocks are identical). `Endpoint::watch_addr().get()` needs `use iroh::Watcher;` in scope; `watch_addr().stream()` needs `n0_future::StreamExt` for `.next()`. Other Watcher-returning Endpoint methods: `home_relay_status()` -> `Watcher<Value = Vec<RelayStatus>>`, `net_report()` -> `Watcher<Value = Option<NetReport>>` (feature `unstable-net-report` only). `Endpoint::online().await` waits for one connected home relay and can hang forever when no relays are configured.

CANONICAL SHAPE (from iroh/examples/echo.rs at v1.5.0): `use iroh::{Endpoint, EndpointAddr, endpoint::{Connection, presets}, protocol::{AcceptError, ProtocolHandler, Router}};` -> `let endpoint = Endpoint::bind(presets::N0).await?;` -> `let router = Router::builder(endpoint).accept(ALPN, Echo).spawn();` -> `router.endpoint().online().await;` -> `let conn = Endpoint::bind(presets::N0).await?.connect(router.endpoint().addr(), ALPN).await?;` -> `let (mut send, mut recv) = conn.open_bi().await?;` -> in the handler `let (mut send, mut recv) = connection.accept_bi().await?;` -> `router.shutdown().await?`.

UNVERIFIED/NEGATIVE items are listed in `unverified`; they are absent from 1.5.0, not merely unread. Every positive signature above was read from a named source path/URL, and all but the companion-crate entries were cross-checked against docs.rs 1.5.0.
````

**Exact signatures and sources**

| # | name | exact signature | source | verified |
|---|---|---|---|---|
| 1 | `iroh::modules` | `mod socket; pub mod tls; pub mod address_lookup; pub mod defaults; pub mod endpoint; pub mod metrics; mod net_report; pub mod protocol; #[cfg(any(test, feature = "test-utils"))] pub mod test_utils; #[cfg(feature = "unstable-net-report")] pub mod unstable_net_report; #[cfg(not(wasm_browser))] pub use iroh_dns::dns; pub use iroh_dns::endpoint_info;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/lib.rs:270-306; https://docs.rs/iroh/1.5.0/iroh/index.html` | True |
| 2 | `iroh::Endpoint / iroh::RelayMode (re-exports)` | `pub use endpoint::{Endpoint, RelayMode};` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/lib.rs:284; https://docs.rs/iroh/1.5.0/iroh/index.html` | True |
| 3 | `iroh::EndpointAddr / EndpointId / keys (re-exports)` | `pub use iroh_base::{EndpointAddr, EndpointId, KeyParsingError, PublicKey, RelayUrl, RelayUrlParseError, SecretKey, Signature, SignatureError, TransportAddr};` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/lib.rs:285-288; https://docs.rs/iroh/1.5.0/iroh/index.html` | True |
| 4 | `iroh::RelayConfig / RelayMap / NetReportConfig / NET_REPORT_TIMEOUT (re-exports)` | `pub use iroh_relay::{RelayConfig, RelayMap}; pub use net_report::{NetReportConfig, TIMEOUT as NET_REPORT_TIMEOUT};` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/lib.rs:292-294; https://docs.rs/iroh/1.5.0/iroh/index.html` | True |
| 5 | `iroh::Watcher (re-export)` | `pub use n0_watcher::Watcher;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/lib.rs:293; https://docs.rs/iroh/1.5.0/iroh/index.html` | True |
| 6 | `endpoint::presets::Preset` | `pub trait Preset { fn apply(self, builder: Builder) -> Builder; }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/presets.rs:20-24; https://docs.rs/iroh/1.5.0/iroh/endpoint/presets/index.html (module named in lib.rs:89)` | True |
| 7 | `endpoint::presets::Empty / Minimal / N0 / N0DisableRelay` | `pub struct Empty; #[cfg(with_crypto_provider)] pub struct Minimal; #[cfg(with_crypto_provider)] pub struct N0; #[cfg(with_crypto_provider)] pub struct N0DisableRelay; (all derive Debug, Copy, Clone, Default)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/presets.rs:36-37, 57-59, 111-113, 173-175; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/examples/echo.rs:9-13` | True |
| 8 | `Endpoint::builder` | `pub fn builder(preset: impl Preset) -> Builder` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:962-964; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-published-1.5.0/iroh-1.5.0/src/endpoint.rs:962 (published tarball, byte-identical)` | True |
| 9 | `Endpoint::bind` | `pub async fn bind(preset: impl Preset) -> Result<Self, BindError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:967-969; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 10 | `endpoint::Builder (struct)` | `pub struct Builder { /* private fields */ } // derive(Debug)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:130-153; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 11 | `Builder::new` | `pub fn new(preset: impl Preset) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:182-184; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 12 | `Builder::empty` | `pub fn empty() -> Self // no address lookup and RelayMode::Disabled; needs crypto_provider before bind` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:192-222; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 13 | `Builder::preset` | `pub fn preset(self, preset: impl Preset) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:187-190; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 14 | `Builder::bind` | `pub async fn bind(self) -> Result<Endpoint, BindError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:228; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 15 | `Builder::alpns` | `pub fn alpns(self, alpn_protocols: Vec<Vec<u8>>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:544-547; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 16 | `Builder::relay_mode` | `pub fn relay_mode(self, relay_mode: RelayMode) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:566-586; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 17 | `Builder::address_lookup` | `pub fn address_lookup(self, address_lookup: impl AddressLookupBuilder) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:614-617; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 18 | `Builder::clear_address_lookup` | `pub fn clear_address_lookup(self) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:594-597; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 19 | `Builder::secret_key` | `pub fn secret_key(self, secret_key: SecretKey) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:529-532; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 20 | `Builder::crypto_provider` | `pub fn crypto_provider(self, crypto_provider: Arc<rustls::crypto::CryptoProvider>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:770; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/presets.rs:45-79` | True |
| 21 | `Builder::bind_addr` | `pub fn bind_addr<A>(self, addr: A) -> Result<Self, InvalidSocketAddr> where A: ToSocketAddr, <A as ToSocketAddr>::Err: Into<InvalidSocketAddr>  // #[cfg(not(wasm_browser))]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:367-374; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html` | True |
| 22 | `Endpoint::set_alpns` | `pub fn set_alpns(&self, alpns: Vec<Vec<u8>>)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:982-991; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 23 | `Endpoint::connect` | `pub async fn connect(&self, endpoint_addr: impl Into<EndpointAddr>, alpn: &[u8]) -> Result<Connection, ConnectError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1066-1085; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 24 | `Endpoint::connect_with_opts` | `pub async fn connect_with_opts(&self, endpoint_addr: impl Into<EndpointAddr>, alpn: &[u8], options: ConnectOptions) -> Result<Connecting, ConnectWithOptsError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1106-1111; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 25 | `Endpoint::accept` | `pub fn accept(&self) -> Accept<'_>   // Future for Accept<'_>: type Output = Option<Incoming>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1179-1184; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:99-128; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 26 | `Endpoint::id` | `pub fn id(&self) -> EndpointId` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1197-1199; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 27 | `Endpoint::addr` | `pub fn addr(&self) -> EndpointAddr   // implemented as self.watch_addr().get()` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1213-1215; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 28 | `Endpoint::watch_addr` | `pub fn watch_addr(&self) -> impl n0_watcher::Watcher<Value = EndpointAddr> + use<>   // #[cfg(not(wasm_browser))]; #[cfg(wasm_browser)] variant has the same signature` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1287-1301 and 1314-1323; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 29 | `Endpoint::online` | `pub async fn online(&self)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1372-1387; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 30 | `Endpoint::home_relay_status` | `pub fn home_relay_status(&self) -> impl Watcher<Value = Vec<RelayStatus>> + use<>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1406-1408; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 31 | `Endpoint::net_report` | `pub fn net_report(&self) -> impl Watcher<Value = Option<NetReport>> + use<>   // #[cfg(feature = "unstable-net-report")]` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1454-1457; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/lib.rs:296-303` | True |
| 32 | `Endpoint::close / is_closed / closed` | `pub async fn close(&self); pub fn is_closed(&self) -> bool; pub fn closed(&self) -> EndpointClosed` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1733, 1738, 1762; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html` | True |
| 33 | `Endpoint::insert_relay / remove_relay` | `pub async fn insert_relay(&self, relay: RelayUrl, config: Arc<RelayConfig>) -> Option<Arc<RelayConfig>>; pub async fn remove_relay(&self, relay: &RelayUrl) -> Option<Arc<RelayConfig>>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:998-1017` | True |
| 34 | `endpoint::RelayMode` | `pub enum RelayMode { Disabled, Default, Staging, Custom(RelayMap) } // derive(Debug, Clone, PartialEq, Eq)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1997-2011; https://docs.rs/iroh/1.5.0/iroh/endpoint/enum.RelayMode.html` | True |
| 35 | `RelayMode::relay_map / RelayMode::custom` | `pub fn relay_map(&self) -> RelayMap; pub fn custom(map: impl IntoIterator<Item = RelayUrl>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:2015-2022, 2037-2040; https://docs.rs/iroh/1.5.0/iroh/endpoint/enum.RelayMode.html` | True |
| 36 | `endpoint::default_relay_mode / force_staging_infra / ENV_FORCE_STAGING_RELAYS` | `pub const ENV_FORCE_STAGING_RELAYS: &str = "IROH_FORCE_STAGING_RELAYS"; pub fn force_staging_infra() -> bool; pub fn default_relay_mode() -> RelayMode` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:2043-2060` | True |
| 37 | `endpoint::RelayStatus` | `pub struct RelayStatus { /* private */ } // pub fn url(&self) -> &RelayUrl; pub fn is_connected(&self) -> bool; pub fn last_error(&self) -> Option<&AnyError>; pub fn auth_denied_reason(&self) -> Option<&str>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:1918-1995` | True |
| 38 | `protocol::Router (struct)` | `pub struct Router { /* private */ } // derive(Clone, Debug); aborts accept task on drop` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:96-102; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.Router.html` | True |
| 39 | `Router::builder` | `pub fn builder(endpoint: Endpoint) -> RouterBuilder` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:412-414; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.Router.html` | True |
| 40 | `Router::endpoint / is_shutdown` | `pub fn endpoint(&self) -> &Endpoint; pub fn is_shutdown(&self) -> bool` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:417-424; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.Router.html` | True |
| 41 | `Router::shutdown` | `pub async fn shutdown(&self) -> Result<(), n0_future::task::JoinError>  // docs.rs renders Result<(), JoinError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:435-452; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.Router.html` | True |
| 42 | `protocol::RouterBuilder (struct)` | `pub struct RouterBuilder { /* private */ }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:104-111; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 43 | `RouterBuilder::new` | `pub fn new(endpoint: Endpoint) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:457-463; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 44 | `RouterBuilder::accept` | `pub fn accept(self, alpn: impl AsRef<[u8]>, handler: impl Into<Box<dyn DynProtocolHandler>>) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:495-503; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 45 | `RouterBuilder::incoming_filter` | `pub fn incoming_filter(self, filter: IncomingFilter) -> Self` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:478-481; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 46 | `RouterBuilder::endpoint` | `pub fn endpoint(&self) -> &Endpoint` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:506-508; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 47 | `RouterBuilder::spawn` | `#[must_use] pub fn spawn(self) -> Router  // calls self.endpoint.set_alpns(alpns) with ALPNs in registration order` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:510-522; https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html` | True |
| 48 | `protocol::ProtocolHandler (trait)` | `pub trait ProtocolHandler: Send + Sync + std::fmt::Debug + 'static { fn on_accepting(&self, accepting: Accepting) -> impl Future<Output = Result<Connection, AcceptError>> + Send; fn accept(&self, connection: Connection) -> impl Future<Output = Result<(), AcceptError>> + Send; fn shutdown(&self) -> impl Future<Output = ()> + Send; }  // only accept is required; on_accepting and shutdown have default bodies` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:228-287; https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html` | True |
| 49 | `ProtocolHandler::accept (required)` | `fn accept(&self, connection: Connection) -> impl Future<Output = Result<(), AcceptError>> + Send;  // may be written: async fn accept(&self, connection: Connection) -> Result<(), AcceptError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:273-276; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/examples/echo.rs:83-111; https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html` | True |
| 50 | `ProtocolHandler::on_accepting (provided)` | `fn on_accepting(&self, accepting: Accepting) -> impl Future<Output = Result<Connection, AcceptError>> + Send { async move { let conn = accepting.await?; Ok(conn) } }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:248-256; https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html` | True |
| 51 | `ProtocolHandler::shutdown (provided)` | `fn shutdown(&self) -> impl Future<Output = ()> + Send { async move {} }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:284-286; https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html` | True |
| 52 | `ProtocolHandler blanket impls` | `impl<T: ProtocolHandler> ProtocolHandler for Arc<T>; impl<T: ProtocolHandler> ProtocolHandler for Box<T>; impl<T: ProtocolHandler> From<T> for Box<dyn DynProtocolHandler>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:289-321` | True |
| 53 | `protocol::DynProtocolHandler (trait)` | `pub trait DynProtocolHandler: Send + Sync + std::fmt::Debug + 'static { fn on_accepting(&self, accepting: Accepting) -> Pin<Box<dyn Future<Output = Result<Connection, AcceptError>> + Send + '_>>; fn accept(&self, connection: Connection) -> Pin<Box<dyn Future<Output = Result<(), AcceptError>> + Send + '_>>; fn shutdown(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>; } // impl<P: ProtocolHandler> DynProtocolHandler for P exists` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:331-373` | True |
| 54 | `protocol::AcceptError` | `#[non_exhaustive] pub enum AcceptError { Connecting { source: ConnectingError }, Connection { source: ConnectionError }, MissingRemoteEndpointId { source: RemoteEndpointIdError }, NotAllowed {}, User { source: AnyError } } // pub fn from_err<T: std::error::Error + Send + Sync + 'static>(value: T) -> Self; pub fn from_boxed(value: Box<dyn std::error::Error + Send + Sync>) -> Self; impl From<std::io::Error>, impl From<quic::ClosedStream>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:113-162` | True |
| 55 | `protocol::IncomingFilter / IncomingFilterOutcome` | `pub type IncomingFilter = Arc<dyn Fn(&crate::endpoint::Incoming) -> IncomingFilterOutcome + Send + Sync + 'static>; #[non_exhaustive] pub enum IncomingFilterOutcome { Accept, Retry, Reject, Ignore }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:164-215` | True |
| 56 | `protocol::handle_connection (NOT part of ProtocolHandler)` | `async fn handle_connection(incoming: crate::endpoint::Incoming, protocols: Arc<ProtocolMap>)  // private free function, not pub, not a trait method` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/protocol.rs:636-672; https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html` | True |
| 57 | `endpoint::Connection (struct)` | `pub struct Connection<State: ConnectionState = HandshakeCompleted> { /* private */ } // derive(Debug, Clone)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:747-752; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html` | True |
| 58 | `Connection::open_bi` | `pub fn open_bi(&self) -> OpenBi<'_>  // await yields Result<(SendStream, RecvStream), ConnectionError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:896-898; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/quic.rs:15-26 (pub use noq::{OpenBi, ...}); https://docs.rs/noq/1.5.0/noq/struct.OpenBi.html; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html` | True |
| 59 | `Connection::accept_bi` | `pub fn accept_bi(&self) -> AcceptBi<'_>  // await yields Result<(SendStream, RecvStream), ConnectionError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:912-914; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/quic.rs:15-26 (pub use noq::{AcceptBi, ...}); https://docs.rs/noq/1.5.0/noq/struct.AcceptBi.html; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html` | True |
| 60 | `Connection::open_uni / accept_uni` | `pub fn open_uni(&self) -> OpenUni<'_>; pub fn accept_uni(&self) -> AcceptUni<'_>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:885-887, 904-906; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html` | True |
| 61 | `Connection::close / closed / close_reason` | `pub fn close(&self, error_code: VarInt, reason: &[u8]); pub async fn closed(&self) -> ConnectionError; pub fn close_reason(&self) -> Option<ConnectionError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:943-953, 985-987; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html` | True |
| 62 | `Connection<HandshakeCompleted>::alpn / remote_id` | `pub fn alpn(&self) -> &[u8]; pub fn remote_id(&self) -> EndpointId` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:1155, 1167; https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html` | True |
| 63 | `endpoint::Incoming::accept` | `pub fn accept(self) -> Result<Accepting, ConnectionError>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:148-152` | True |
| 64 | `endpoint::Accepting / Connecting futures` | `impl Future for Accepting { type Output = Result<Connection, ConnectingError>; }  // impl Future for Connecting { type Output = Result<Connection, ConnectingError>; }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/connection.rs:574-575, 671-672` | True |
| 65 | `endpoint::ConnectError / ConnectWithOptsError` | `#[non_exhaustive] pub enum ConnectError { Connect { source: ConnectWithOptsError }, Connecting { source: ConnectingError }, Connection { source: ConnectionError } }; #[non_exhaustive] pub enum ConnectWithOptsError { SelfConnect, NoAddress { source: AddressLookupFailed }, Noq { source: QuicConnectError }, InternalConsistencyError, LocallyRejected, EndpointClosed, InvalidAlpn }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint.rs:912-952` | True |
| 66 | `EndpointId (type alias)` | `pub type EndpointId = PublicKey;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh-base/src/key.rs:70; https://docs.rs/iroh/1.5.0/iroh/type.EndpointId.html` | True |
| 67 | `EndpointAddr (struct)` | `pub struct EndpointAddr { pub id: EndpointId, pub addrs: BTreeSet<TransportAddr> } // derives Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh-base/src/endpoint_addr.rs:41-47; https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html` | True |
| 68 | `EndpointAddr methods` | `pub fn new(id: PublicKey) -> Self; pub fn from_parts(id: PublicKey, addrs: impl IntoIterator<Item = TransportAddr>) -> Self; pub fn with_relay_url(self, relay_url: RelayUrl) -> Self; pub fn with_ip_addr(self, addr: SocketAddr) -> Self; pub fn with_addrs(self, addrs: impl IntoIterator<Item = TransportAddr>) -> Self; pub fn is_empty(&self) -> bool; pub fn ip_addrs(&self) -> impl Iterator<Item = &SocketAddr>; pub fn relay_urls(&self) -> impl Iterator<Item = &RelayUrl>` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh-base/src/endpoint_addr.rs:91-153; https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html` | True |
| 69 | `EndpointAddr conversions` | `impl From<EndpointId> for EndpointAddr { fn from(endpoint_id: EndpointId) -> Self { EndpointAddr::new(endpoint_id) } }  // no Display and no FromStr impl for EndpointAddr in iroh-base 1.5.0` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh-base/src/endpoint_addr.rs:155-159; https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html` | True |
| 70 | `TransportAddr (enum)` | `#[non_exhaustive] pub enum TransportAddr { Relay(RelayUrl), Ip(SocketAddr), Custom(CustomAddr) } // pub fn is_relay(&self) -> bool; pub fn is_ip(&self) -> bool; pub fn is_custom(&self) -> bool; impl fmt::Display` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh-base/src/endpoint_addr.rs:49-89; https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html` | True |
| 71 | `PublicKey string addressing` | `pub fn fmt_short(&self) -> impl Display + Copy + 'static; pub fn to_z32(&self) -> String; pub fn from_z32(s: &str) -> Result<Self, KeyParsingError>; impl Display for PublicKey (lowercase hex); impl FromStr for PublicKey { type Err = KeyParsingError }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh-base/src/key.rs:142-148, 164-174, 221-257` | True |
| 72 | `address_lookup::AddressLookup (replaces Discovery)` | `pub trait AddressLookup: std::fmt::Debug + Send + Sync + 'static { fn publish(&self, _data: &EndpointData) {} fn resolve(&self, _endpoint_id: EndpointId) -> Option<BoxStream<Result<Item, Error>>> { None } }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/address_lookup.rs:335-352; https://docs.rs/iroh/1.5.0/iroh/address_lookup/trait.AddressLookup.html` | True |
| 73 | `address_lookup::AddressLookupBuilder` | `pub trait AddressLookupBuilder: Send + Sync + std::fmt::Debug + 'static { fn into_address_lookup(self, endpoint: &Endpoint) -> Result<impl AddressLookup, AddressLookupBuilderError>; }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/address_lookup.rs:144-152; https://docs.rs/iroh/1.5.0/iroh/address_lookup/trait.AddressLookupBuilder.html` | True |
| 74 | `address_lookup::AddressLookupServices` | `pub struct AddressLookupServices; pub fn set_addr_filter(&self, filter: AddrFilter); pub fn add(&self, service: impl AddressLookup + 'static); pub fn add_boxed(&self, service: Box<dyn AddressLookup>); pub fn is_empty(&self) -> bool; pub fn len(&self) -> usize; pub fn clear(&self)` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/address_lookup.rs:464-523` | True |
| 75 | `address_lookup concrete services` | `pub struct DnsAddressLookup; pub fn DnsAddressLookup::builder(origin_domain: String) -> DnsAddressLookupBuilder; pub fn DnsAddressLookup::n0_dns() -> DnsAddressLookupBuilder; pub struct MemoryLookup; pub fn MemoryLookup::new() -> Self; pub fn MemoryLookup::add_endpoint_info(&self, endpoint_info: impl Into<EndpointInfo>); pub struct PkarrPublisher; pub fn PkarrPublisher::n0_dns() -> PkarrPublisherBuilder; pub struct PkarrResolver; pub fn PkarrResolver::n0_dns() -> PkarrResolverBuilder` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/address_lookup/dns.rs:46-92; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/address_lookup/memory.rs:75-211; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/address_lookup/pkarr.rs:288-355, 519-547; /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/iroh/src/endpoint/presets.rs:116-139` | True |
| 76 | `iroh_tickets::Ticket (separate crate iroh-tickets 1.0.0)` | `pub trait Ticket: Sized { const KIND: &'static str; fn encode_bytes(&self) -> Vec<u8>; fn decode_bytes(bytes: &[u8]) -> Result<Self, ParseError>; fn encode_string(&self) -> String; fn decode_string(s: &str) -> Result<Self, ParseError>; }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-tickets-src-1.0.0/iroh-tickets-1.0.0/src/lib.rs:26-65; https://docs.rs/iroh-tickets/1.0.0/iroh_tickets/trait.Ticket.html (crate README in downloaded tarball)` | True |
| 77 | `iroh_tickets::endpoint::EndpointTicket` | `pub struct EndpointTicket { /* private */ } // impl Ticket for EndpointTicket { const KIND: &'static str = "endpoint"; ... }; pub fn new(addr: EndpointAddr) -> Self; pub fn endpoint_addr(&self) -> &EndpointAddr; impl From<EndpointAddr> for EndpointTicket; impl From<EndpointTicket> for EndpointAddr; impl Display; impl FromStr { type Err = ParseError }; impl Serialize/Deserialize` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-tickets-src-1.0.0/iroh-tickets-1.0.0/src/endpoint.rs:28-104, 106-130; https://docs.rs/iroh-tickets/1.0.0/iroh_tickets/endpoint/struct.EndpointTicket.html` | True |
| 78 | `n0_watcher::Watcher (trait declaration)` | `pub trait Watcher: Clone { type Value: Clone + Eq; ... }` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.0/n0-watcher-1.0.0/src/lib.rs:230-239; /Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.1/n0-watcher-1.0.1/src/lib.rs:230-239 (1.0.1 trait block is byte-identical); https://docs.rs/n0-watcher/1.0.0/n0_watcher/trait.Watcher.html` | True |
| 79 | `Watcher required methods` | `fn update(&mut self) -> bool; fn peek(&self) -> &Self::Value; fn is_connected(&self) -> bool; fn poll_updated(&mut self, cx: &mut task::Context<'_>) -> Poll<Result<(), Disconnected>>;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.0/n0-watcher-1.0.0/src/lib.rs:260-284; https://docs.rs/n0-watcher/1.0.0/n0_watcher/trait.Watcher.html` | True |
| 80 | `Watcher provided methods` | `fn get(&mut self) -> Self::Value; fn updated(&mut self) -> NextFut<'_, Self>; fn initialized<T, W>(&mut self) -> InitializedFut<'_, T, W, Self> where W: Nullable<T> + Clone, Self: Watcher<Value = W>; fn stream(self) -> Stream<Self> where Self: Unpin; fn stream_updates_only(self) -> Stream<Self> where Self: Unpin; fn map<T: Clone + Eq>(self, map: impl Fn(Self::Value) -> T + Send + Sync + 'static) -> Map<Self, T>; fn or<W: Watcher>(self, other: W) -> Tuple<Self, W>;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.0/n0-watcher-1.0.0/src/lib.rs:255-258, 292-383; https://docs.rs/n0-watcher/1.0.0/n0_watcher/trait.Watcher.html` | True |
| 81 | `n0_watcher::Watchable / Direct / Disconnected` | `#[derive(Debug, Default)] pub struct Watchable<T>; impl<T: Clone + Eq> Watchable<T> { pub fn new(value: T) -> Self; pub fn set(&self, value: T) -> Result<T, T>; pub fn watch(&self) -> Direct<T>; pub fn get(&self) -> T; pub fn has_watchers(&self) -> bool; } pub struct Direct<T>; pub struct Disconnected;` | `/Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.0/n0-watcher-1.0.0/src/lib.rs:96-192, 390, 756; https://docs.rs/n0-watcher/1.0.0/n0_watcher/trait.Watcher.html` | True |

**Contract source refs**

- `https://index.crates.io/ir/oh/iroh (line for 1.5.0: vers=1.5.0, cksum=40ac5d8330439176254581b6db4bc6f1d947d7cc741fda4e04b96d758addf1db, rust_version=1.91, yanked=false, features + features2)`
- `https://static.crates.io/crates/iroh/iroh-1.5.0.crate (download; sha256 verified equal to index cksum)`
- `https://index.crates.io/ir/oh/iroh-base (1.5.0 published, rust_version 1.91)`
- `https://index.crates.io/n0/-w/n0-watcher (1.0.0 and 1.0.1 published, rust_version 1.91)`
- `https://static.crates.io/crates/n0-watcher/n0-watcher-1.0.0.crate and https://static.crates.io/crates/n0-watcher/n0-watcher-1.0.1.crate (sha256 verified against index)`
- `https://index.crates.io/ir/oh/iroh-tickets (1.0.0 published, rust_version 1.91); https://static.crates.io/crates/iroh-tickets/iroh-tickets-1.0.0.crate (sha256 verified)`
- `https://index.crates.io/ir/oh/iroh-ticket (HTTP 404: no such crate)`
- `https://index.crates.io/3/n/noq (noq 1.5.0 published, rust_version 1.88)`
- `gh api repos/n0-computer/iroh/tags?per_page=100 (tag v1.5.0 -> commit ea3bbb6ab41fa23816d7f9e3220ab77d444435b2)`
- `gh api repos/n0-computer/iroh/git/trees/ea3bbb6ab41fa23816d7f9e3220ab77d444435b2?recursive=1 (294 entries, truncated=false; no ticket.rs, no NodeId/NodeAddr/Discovery modules)`
- `gh api repos/n0-computer/iroh/contents/<path>?ref=v1.5.0 (base64-decoded; files actually inspected for signatures are named in each signature's source field): Cargo.toml, iroh/Cargo.toml, iroh/README.md, CHANGELOG.md, iroh/src/lib.rs, iroh/src/protocol.rs, iroh/src/endpoint.rs, iroh/src/endpoint/connection.rs, iroh/src/endpoint/presets.rs, iroh/src/endpoint/quic.rs, iroh/src/address_lookup.rs + dns.rs/memory.rs/pkarr.rs, iroh-base/src/lib.rs, iroh-base/src/endpoint_addr.rs, iroh-base/src/key.rs, iroh/examples/echo.rs; also decoded but only used as cross-checks: iroh/src/endpoint/bind.rs, hooks.rs, defaults.rs, iroh-base/src/relay_url.rs, iroh/examples/connect.rs, listen.rs, echo-no-router.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/ (decoded GitHub tag source tree; byte-identical to published crate for lib.rs, endpoint.rs, protocol.rs, endpoint/connection.rs, endpoint/presets.rs, address_lookup.rs)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-published-1.5.0/iroh-1.5.0/ (extracted published iroh-1.5.0.crate; Cargo.toml features and src/*.rs read)`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.0/n0-watcher-1.0.0/src/lib.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_n0watcher-src-1.0.1/n0-watcher-1.0.1/src/lib.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-tickets-src-1.0.0/iroh-tickets-1.0.0/src/lib.rs and /Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-tickets-src-1.0.0/iroh-tickets-1.0.0/src/endpoint.rs`
- `/Users/dylanwong/projects/aws0-research-2026-11-19/_iroh-src-1.5.0/CHANGELOG.md:727 (breaking: Rename &#96;Discovery&#96; to &#96;AddressLookup&#96;), :826 (breaking: Move tickets into their own crate), :828 (breaking: Rename Node to Endpoint in all cases)`
- `https://docs.rs/iroh/1.5.0/iroh/index.html`
- `https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html`
- `https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html`
- `https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html`
- `https://docs.rs/iroh/1.5.0/iroh/endpoint/enum.RelayMode.html`
- `https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html`
- `https://docs.rs/iroh/1.5.0/iroh/protocol/struct.Router.html`
- `https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html`
- `https://docs.rs/iroh/1.5.0/iroh/address_lookup/trait.AddressLookup.html`
- `https://docs.rs/iroh/1.5.0/iroh/address_lookup/trait.AddressLookupBuilder.html`
- `https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html`
- `https://docs.rs/iroh/1.5.0/iroh/type.EndpointId.html`
- `https://docs.rs/iroh/1.5.0/iroh/endpoint/presets/index.html`
- `https://docs.rs/iroh-tickets/1.0.0/iroh_tickets/trait.Ticket.html`
- `https://docs.rs/n0-watcher/1.0.0/n0_watcher/trait.Watcher.html`
- `https://docs.rs/iroh-tickets/1.0.0/iroh_tickets/endpoint/struct.EndpointTicket.html`
- `https://docs.rs/noq/1.5.0/noq/struct.OpenBi.html`
- `https://docs.rs/noq/1.5.0/noq/struct.AcceptBi.html`
- `https://crates.io/api/v1/crates/iroh/1.5.0 (HTTP 403 from this host; NOT used as a source. Exact version data comes from index.crates.io and the checksum-verified published crate.)`

## 2. REJECTED CONTRACTS -- CLEARLY MARKED SECTION

### 2.1 Rejected contracts: NONE

The adversarial `Refute` phase ran one independent agent per contract. **All 14 verdicts returned `refuted=false` with an empty `wrong_signatures` array**, so the workflow's survivor filter admitted all 14 contracts. There is therefore no rejected contract and no rejection reason to reproduce. The verdicts and their full reasons are preserved verbatim here (and in `api-sheet.json` under `refutation_verdicts`) so this claim is falsifiable against `/Users/dylanwong/.tclaude/projects/-Users-dylanwong/1fbb9576-9f61-4529-ac97-efd40ba18afd/subagents/workflows/wf_f0a101ff-744/journal.jsonl`.

| refute label | refuted | wrong_signatures | reason (verbatim, full text in JSON) |
|---|---|---|---|
| `refute:iroh-gossip` | False | 0 | Could not refute; the contract is accurate for iroh-gossip 0.103.0. No invented, renamed, deprecated, or version-mismatched signature found.<br><br>Verification performed (all with curl / gh api against live sources, not just local files):<br><br>1) Authenticity of the cited tarball. Downloaded https://static.crates.io/crates/iroh-gossip/iroh-gossip-0.103.0.crate fresh; sha256 = 74c4170847b5603f489c4d7771f1e7 ... |
| `refute:iroh-blobs` | False | 0 | Could not refute: the contract survives direct, independent source checks. I re-fetched everything from live sources rather than trusting the contract's /tmp mirror: &#96;gh api repos/n0-computer/iroh-blobs/git/ref/tags/v0.105.0&#96; -> annotated tag -> commit 7a7c410f31c9da90854099f4a5d5fdf20da840c3; &#96;git/trees/v0.105.0?recursive=1&#96; -> sha 7a7c410f..., truncated=false, 107 blobs, 58 src/*.rs (HEAD tree 108 ... |
| `refute:tokio-rustls-acme` | False | 0 | Could not refute: every claim checks out against the live crates.io index/API, the published .crate, and the GitHub tag v0.10.0 source fetched directly with &#96;gh api&#96; and curl.<br><br>Provenance (checked, not trusted): /Users/dylanwong/projects/aws0-research-2026-11-19/contracts/tokio-rustls-acme.json is the contract (80 signatures, all &#96;verified:true&#96;). The sparse index https://index.crates.io/to/ki/tok ... |
| `refute:iroh-docs` | False | 0 | Could not refute: the contract is accurate for iroh-docs 0.101.0. Direct source checks performed: (1) curl to https://index.crates.io/ir/oh/iroh-docs shows exactly one 0.101.0 record, yanked=false, rust_version=1.91, cksum 8fd1bd5e39d0321a3c4a2bcef9650476c076e2df41a0e84577eca23d6de6c8ab, pubtime 2026-06-15T14:33:45Z, features/features2 exactly matching all 5 claimed feature_flags (default, metrics ... |
| `refute:iroh-smol-kv` | False | 0 | REFUTATION FAILED -- the 0.5.0 contract survives direct source checks. No invented, renamed, deprecated, or version-mismatched signature was found. Evidence (all files under /Users/dylanwong/projects/aws0-research-2026-11-19/findings/.verify-iroh-smol-kv/):<br><br>1) Registry/GitHub identity: index.txt (curl https://index.crates.io/ir/oh/iroh-smol-kv) has exactly six rows 0.1.0-0.5.0, all yanked=false, r ... |
| `refute:quic-rpc` | False | 0 | REFUTATION FAILED (refuted=false): the quic-rpc 0.20.0 contract survives direct, independent source checks; no invented, renamed, deprecated, or version-mismatched signature found.<br><br>Provenance: gh api repos/n0-computer/quic-rpc/git/ref/tags/v0.20.0 -> annotated tag cb8b1ced25b80594c339c80c08c044cf7fc4dccf -> commit 5ed68d4323daa5cfb29539e47f981f0fab80c233, also the sha1 in the published crate's .c ... |
| `refute:sendme` | False | 0 | COULD NOT REFUTE the API contract. I re-fetched every cited primary source into /Users/dylanwong/projects/aws0-research-2026-11-19/_refute_sendme_dl: &#96;gh api repos/n0-computer/sendme&#96; (default_branch main, license Apache-2.0), &#96;.../commits/HEAD&#96; = fe29e3454999794bc5ea686cef1ad53d99124a53 (tree cb2f429439ecd0d473d0bfd0b6449ab315c8bbe9, 2026-11-19), &#96;.../tags&#96; v0.37.1 -> 21881bd4b527f55533633623cf84 ... |
| `refute:iroh` | False | 0 | REFUTATION FAILED -- the contract in /Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh.json is accurate, so refuted=false and wrong_signatures is empty.<br><br>Provenance (gh api / curl, not the local caches): annotated tag v1.5.0 = fefd9168... -> commit ea3bbb6ab41fa23816d7f9e3220ab77d444435b2; recursive tree at that commit returns 294 entries with truncated=false and zero paths containin ... |
| `refute:net-tools` | False | 0 | REFUTED=false (contract survives); wrong_signatures is empty. I could not refute any signature, version, feature, license, MSRV, or checksum claim by direct gh api / curl checks.<br><br>ALL 8 SIGNATURES VERIFIED VERBATIM at the cited line ranges:<br>- /tmp/net-tools-adversarial-20261119/gh-tag/portmapper/src/lib.rs (ref 334793cfdaf3487db0c721b9312179a8ae3b0dcb): :165 &#96;pub fn new(config: Config) -> Self&#96;, : ... |
| `refute:n0-dns-resolver` | False | 0 | Could not refute: every check against live sources (gh api / curl) confirms the contract. Provenance: https://index.crates.io/n0/-d/n0-dns-resolver has exactly 0.1.0/0.1.1, both unyanked; 0.1.1 rust_version 1.91, pubtime 2026-11-18T16:33:19Z, cksum 0381996935dc55768ae8b13e7db69d3a90f13afb51dde80f870484027968bcd2, and the downloaded https://static.crates.io/crates/n0-dns-resolver/n0-dns-resolver-0. ... |
| `refute:iroh-dag-sync` | False | 0 | No refutation: every cited signature checks out against the pinned upstream source. I re-fetched all crate files with &#96;gh api repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/<path>?ref=41ebc455678d7789459add9c2218d463bc74866a&#96; (base64-decoded) into /Users/dylanwong/projects/aws0-research-2026-11-19/findings/.verify-iroh-dag-sync/, plus the root README and CI workflow. &#96;commits/main&#96; = 41 ... |
| `refute:iroh-roq` | False | 0 | NOT REFUTED: all 24 &#96;signatures&#96; entries in /Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-roq.json match the primary sources. Verification performed with curl (index.crates.io, static.crates.io, docs.rs) and authenticated &#96;gh api&#96; (unauthenticated curl to api.github.com is rate-limited; gh CLI works).<br><br>Primary evidence (artifacts in /Users/dylanwong/projects/aws0-research-2026- ... |
| `refute:laboon-cli` | False | 0 | NOT REFUTED. I re-fetched every cited source fresh with &#96;gh api repos/samyfodil/laboon/contents/<path>?ref=<sha>&#96; (base64-decoded into /tmp/laboon-refute/data/tag/ and /tmp/laboon-refute/data/head/) and re-checked external facts with curl. The full 26-group/188-signature contract at /Users/dylanwong/projects/aws0-research-2026-11-19/contracts/laboon-cli.json matches the task text and is byte-ident ... |
| `refute:ministack` | False | 0 | Adversarial pass found no invented, renamed, deprecated or version-mismatched signature, so the contract survives. Direct evidence: (1) Provenance all matches -- curl https://index.crates.io/mi/ni/ministack = 404 and https://crates.io/api/v1/crates/ministack = 404 {"errors":[{"detail":"crate &#96;ministack&#96; does not exist"}]}; https://pypi.org/pypi/ministack/1.5.43/json gives version 1.5.43, requires_p ... |

### 2.2 Refuted API names and negative findings (not rejected contracts)

The refute phase did not reject any whole contract, but the surviving contracts themselves **refute specific requested API names**. These are negative, source-verified findings -- do not generate code with them. They are a subset of the unverified items listed in section 3; the full text of each is in `api-sheet.json` under `refuted_api_claims`.

- **net-tools** (unverified #1): No published crate named net-tools exists: https://index.crates.io/ne/t-/net-tools returned HTTP 404; exact_version 0.20.2 is the common version of the two workspace members, not a net-tools crate release.
- **net-tools** (unverified #3): nested_enum_utils 0.2.3 has no NAT traversal or interface discovery API; its only public items are the proc-macro attributes enum_conversions and common_fields, so the requested one useful API per subcrate cannot be satisfied for it.
- **tokio-rustls-acme** (unverified #1): docs.rs rustdoc signatures for tokio-rustls-acme 0.10.0: https://docs.rs/tokio-rustls-acme/0.10.0/tokio_rustls_acme/index.html is a 'failed to build' landing page and https://docs.rs/crate/tokio-rustls-acme/0.10.0/builds says 'All builds failed' (rustc 1.102.0-nightly, 2026-11-18). No docs.rs-derived signature is asserted; all signatures come from the pinned GitHub tag and the published .crate.
- **tokio-rustls-acme** (unverified #3): No concrete minimum supported Rust version is asserted (no rust-version in Cargo.toml; crates.io rust_version is null). The docs.rs nightly compiler version is a build-host fact, not an MSRV.
- **sendme** (unverified #5): No build, no cargo metadata feature-unification run, no live send/receive execution: feature_flags are manifest declarations, not a resolved feature graph; NAT traversal, ALPN negotiation and ticket round-trip were not witnessed.
- **iroh-dag-sync** (unverified #1): No published version exists, so exact_version 0.1.0 could not be cross-checked against a crates.io release artifact; it is only the Cargo.toml value. There are zero published versions.
- **iroh-dag-sync** (unverified #2): docs.rs returns 404, so no signature was cross-checked against generated rustdoc; all signatures come from source files only.
- **iroh-dag-sync** (unverified #3): No cargo build/check/test was run here; compilability, runtime behavior, and protocol interop are unverified.
- **iroh-dag-sync** (unverified #4): No Cargo.lock exists for iroh-dag-sync, so exact resolved versions of direct and transitive dependencies (and their real APIs) are unverified; the listed versions are manifest requirements only.
- **iroh-dag-sync** (unverified #5): The rustc >= 1.75 requirement is inferred from language features and an unused CI MSRV variable; no build on 1.75 or any other toolchain was performed.
- **n0-dns-resolver** (unverified #2): No compilation against Rust 1.91 was performed; MSRV/edition are taken from Cargo.toml (rust-version = "1.91", edition = "2024") and the crates.io index record (rust_version = "1.91").
- **n0-dns-resolver** (unverified #4): cfg-dependent items are reported from source attributes only; no wasm_browser, Windows, Apple, Android, TLS-feature or no-default-features build was compiled or tested.
- **n0-dns-resolver** (unverified #5): No live DNS queries, relay connections, or pkarr publishes were executed; all iroh relay/node-discovery claims are read from iroh tag v1.5.0 source and the n0-dns-resolver v0.1.1 example documentation.
- **iroh-docs** (unverified #1): Docs::author: no such public method in iroh-docs 0.101.0. Verified absent by exhaustive grep of the published crate and tag; no signature supplied. Use DocsApi::author_create/author_default/author_list/author_set_default/author_export/author_import/author_delete instead.
- **iroh-docs** (unverified #2): Doc::insert: no such public method in 0.101.0. Verified absent; use Doc::set_bytes or Doc::set_hash (or low-level Replica::insert / Replica::hash_and_insert).
- **iroh-docs** (unverified #3): Doc::keys: no such public method in 0.101.0. Verified absent; use Doc::get_many(impl Into<Query>) / Doc::get_one with Query::all / Query::key_prefix / Query::key_exact.
- **iroh-docs** (unverified #4): Doc::delete: no such public method in 0.101.0. Verified absent; use Doc::del(author_id, prefix) -> Result<usize> (prefix tombstone) or Replica::delete_prefix.
- **iroh-docs** (unverified #5): Doc::import_export: no such public method in 0.101.0. Verified absent; use Doc::import_file / Doc::export_file, DocsApi::import / import_namespace / import_and_subscribe, and DocsApi::author_import / author_export.
- **iroh-docs** (unverified #6): Doc::content_status (or Docs::content_status): no such public method in 0.101.0. Verified absent; content status is exposed as ContentStatus, ContentStatusCallback, LiveEvent::InsertRemote.content_status and LiveEvent::ContentReady. Doc::status() returns OpenState { sync, subscribers, handles }.
- **iroh-docs** (unverified #7): Docs::new is NOT a storage-path constructor: the only exact signature is &#96;pub fn new(engine: Engine) -> Self&#96; (src/protocol.rs:48); construct through Docs::memory()/Docs::persistent(path) -> Builder::spawn(endpoint, blobs, gossip) or DocsApi::spawn(Arc<Engine>). Docs::open/create/list are inherited through Deref<Target = DocsApi>, not inherent methods.
- **iroh-docs** (unverified #8): WebFetch of https://docs.rs/iroh-docs/0.101.0/iroh_docs/ (bare directory URL) returned 'No response from model'; the equivalent https://docs.rs/iroh-docs/0.101.0/iroh_docs/index.html page was fetched successfully and no API claim relies on the failed fetch.
- **iroh-roq** (unverified #1): https://crates.io/api/v1/crates/iroh-roq returned HTTP 403; no metadata from that endpoint was used. All version data came from https://index.crates.io/ir/oh/iroh-roq and the downloaded .crate instead.
- **iroh-roq** (unverified #2): Compatibility of iroh-roq with modern iroh (latest non-yanked 1.5.0) was NOT source-verified: I did not read iroh 1.x sources to check whether &#96;iroh::endpoint::Connection::{send_datagram, read_datagram, open_uni, accept_uni}&#96;, &#96;Endpoint::builder().alpns(..)&#96;, &#96;Connection::alpn()&#96;, or &#96;Endpoint::connect(node_addr, ALPN)&#96; still exist with these signatures. The type-mismatch conclusion follows only from the semver-disti ...
- **iroh-roq** (unverified #4): No build, test, fuzz, or interoperability run was performed; runtime behavior, RoQ draft conformance, QUIC datagram MTU limits, and interop with the Go RoQ implementation named in README are unverified.
- **iroh-roq** (unverified #5): No signature is asserted for &#96;rtp::packet::Packet&#96;, &#96;iroh_quinn_proto::VarInt&#96;, or &#96;iroh::endpoint::Connection&#96; themselves -- those are re-exported/third-party types whose own sources were not read for this contract.
- **ministack** (unverified #1): No live Docker daemon was contacted and no container was started in this session: the real-engine boot behaviour is sourced from code literals + README, not witnessed at runtime.
- **ministack** (unverified #2): crates.io/docs.rs verification path is inapplicable: https://index.crates.io/mi/ni/ministack returns HTTP 404; ministack is a Python/PyPI package, so there is no Rust crate version or docs.rs signature page.
- **ministack** (unverified #6): ministack/services/opensearchserverless.py and core/pgproxy.py were fetched but only grep-level evidence is cited; no exhaustive API extraction was done for them.
- **ministack** (unverified #7): The gm skill was loaded as required by CLAUDE.md, but its spool daemon is not present in this working directory (one instruction dispatch returned no response); the extraction was performed directly with gh api/curl per the task's explicit evidence rules.
- **iroh-smol-kv** (unverified #2): No declared MSRV: no rust-version in Cargo.toml, Cargo.toml.orig or published Cargo.toml; rust_version=null in all index rows and crates.io records; no CI MSRV file in HEAD tree. edition=2024; an edition-2024 toolchain floor was not measured against a compiler.
- **iroh-smol-kv** (unverified #5): Companion APIs not extracted: iroh-smol-kv-uniffi/src/code/*.rs and uniffi-example/src/lib.rs were listed but not read; iroh-smol-kv-uniffi 0.4.0 is git-only (index/API 404); uniffi-example/go/go.mod was not read. No runtime test, formal liveness/consensus proof, race analysis or gossip-convergence fuzz was performed; the verdict is static source analysis.
- **laboon-cli** (unverified #1): No crates.io crate exists for this dependency: curl https://index.crates.io/la/bo/laboon -> 404 and https://index.crates.io/la/bo/laboon-cli -> 404; https://docs.rs/laboon/latest/laboon/ -> 404; https://crates.io/api/v1/crates/laboon -> 403. No published Rust version or docs.rs signature can be reported.
- **laboon-cli** (unverified #2): docs/permissions-and-limits.md contains no platform support matrix (only three per-platform caveats), so the requested matrix cannot be extracted from it; the OS x arch matrix in this contract is derived from .goreleaser.yaml, README.md, AGENTS.md and the release asset names instead.
- **laboon-cli** (unverified #4): Go 1.27.1 as a build requirement is verified from go.mod/go.work/README.md/CI, not by running &#96;go build&#96; or &#96;go test&#96; in this environment; no toolchain availability check was performed.
- **iroh** (unverified #1): NodeId: ABSENT in iroh 1.5.0 (source checked: full recursive git tree at v1.5.0, iroh/src/lib.rs:285-288 re-exports, iroh-base/src/key.rs:70, docs.rs/iroh/1.5.0/iroh/index.html). Use &#96;iroh::EndpointId&#96; = &#96;pub type EndpointId = PublicKey&#96;.
- **iroh** (unverified #2): NodeAddr: ABSENT in iroh 1.5.0 (same sources; CHANGELOG.md:828 'Rename Node to Endpoint in all cases'). Use &#96;iroh::EndpointAddr&#96;.
- **iroh** (unverified #3): Discovery / Discovery builder / &#96;Builder::discovery(...)&#96;: ABSENT in iroh 1.5.0 (iroh/src/address_lookup.rs:144,335; CHANGELOG.md:727 'Rename &#96;Discovery&#96; to &#96;AddressLookup&#96;'). Use &#96;iroh::address_lookup::{AddressLookup, AddressLookupBuilder}&#96; and &#96;Builder::address_lookup(...)&#96;; the &#96;presets::N0&#96; preset installs DnsAddressLookup + PkarrPublisher + PkarrResolver.
- **iroh** (unverified #4): NodeTicket / iroh::ticket module / iroh::NodeTicket: ABSENT in iroh 1.5.0 (the v1.5.0 recursive tree has zero paths containing 'ticket'; docs.rs root has no ticket module; CHANGELOG.md:826 'Move tickets into their own crate'). Use the separate crate &#96;iroh-tickets = "1.0.0"&#96;, type &#96;iroh_tickets::endpoint::EndpointTicket&#96;. No crate named &#96;iroh-ticket&#96; exists on the crates.io index (HTTP 404 for https://index.crates.io/ ...
- **iroh** (unverified #5): ProtocolHandler::handle_connection: ABSENT as a trait method (docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html shows only accept/on_accepting/shutdown; iroh/src/protocol.rs:636 defines a PRIVATE free fn). Do not implement it.
- **iroh** (unverified #6): Endpoint::builder() (zero-argument), Endpoint::relay_mode(...), Endpoint::alpns(...), Endpoint::discovery(...): ABSENT in 1.5.0. &#96;Endpoint::builder&#96; requires &#96;preset: impl Preset&#96;; relay_mode/alpns live on &#96;endpoint::Builder&#96;; &#96;Endpoint::set_alpns&#96; is the live-Endpoint setter.
- **iroh** (unverified #7): https://crates.io/api/v1/crates/iroh/1.5.0 returned HTTP 403 from this host and was NOT used as evidence; exact published version, checksum, yanked status, rust_version and features came from https://index.crates.io/ir/oh/iroh plus the checksum-verified published crate tarball. No signature depends on the 403'd endpoint.
- **iroh** (unverified #8): docs.rs/iroh-base/1.5.0/iroh_base/struct.EndpointAddr.html returned no model response; EndpointAddr signatures were instead read from iroh-base/src/endpoint_addr.rs at tag v1.5.0 and cross-checked at https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html.
- **iroh** (unverified #9): EndpointAddr Display / FromStr: NOT implemented in iroh-base 1.5.0 (verified by the full impl list in iroh-base/src/endpoint_addr.rs and docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html). Do not assume a string form; use serde or iroh-tickets.

## 3. EVERY UNVERIFIED ITEM

Total: **73** unverified items from the 14 surviving contracts. Additionally, **0** of the 800 signature rows carry `verified=false`. Each item is reproduced verbatim.

### 3.1 `net-tools` -- 4 unverified items

**U1. `net-tools`**

````text
No published crate named net-tools exists: https://index.crates.io/ne/t-/net-tools returned HTTP 404; exact_version 0.20.2 is the common version of the two workspace members, not a net-tools crate release.
````

**U2. `net-tools`**

````text
netdev 0.47.0 and nested_enum_utils 0.2.3 are not subcrates of n0-computer/net-tools; netdev is an external dependency, nested_enum_utils is unrelated. The task's suggested subcrate list is corrected rather than reproduced.
````

**U3. `net-tools`**

````text
nested_enum_utils 0.2.3 has no NAT traversal or interface discovery API; its only public items are the proc-macro attributes enum_conversions and common_fields, so the requested one useful API per subcrate cannot be satisfied for it.
````

**U4. `net-tools`**

````text
docs.rs crate-level index pages did not include full declarations; all listed signatures were verified against extracted crate sources and GitHub tagged sources instead.
````

### 3.2 `tokio-rustls-acme` -- 4 unverified items

**U1. `tokio-rustls-acme`**

````text
docs.rs rustdoc signatures for tokio-rustls-acme 0.10.0: https://docs.rs/tokio-rustls-acme/0.10.0/tokio_rustls_acme/index.html is a 'failed to build' landing page and https://docs.rs/crate/tokio-rustls-acme/0.10.0/builds says 'All builds failed' (rustc 1.102.0-nightly, 2026-11-18). No docs.rs-derived signature is asserted; all signatures come from the pinned GitHub tag and the published .crate.
````

**U2. `tokio-rustls-acme`**

````text
External, non-tokio-rustls-acme signatures used by the examples were not read from their own crates' sources: tokio_rustls::StartHandshake::into_stream, rustls::ServerConfig::builder_with_provider / with_protocol_versions / with_no_client_auth / with_cert_resolver, axum_server::bind(...).acceptor(...).serve(...), and std::io::Error::other. Only their call sites inside this crate's examples are verified.
````

**U3. `tokio-rustls-acme`**

````text
No concrete minimum supported Rust version is asserted (no rust-version in Cargo.toml; crates.io rust_version is null). The docs.rs nightly compiler version is a build-host fact, not an MSRV.
````

**U4. `tokio-rustls-acme`**

````text
LICENSE-APACHE and LICENSE-MIT file contents were not read; only their paths in the tag tree and the Cargo.toml / crates.io dual-license expression were read.
````

### 3.3 `iroh-gossip` -- 0 unverified items

- (none)
### 3.4 `sendme` -- 5 unverified items

**U1. `sendme`**

````text
Exact accepted CLI spellings for --ticket-type: AddrInfoOptions derives derive_more::FromStr (src/main.rs:246-271); derive_more 2.0.1 source was not read and the binary was not run. The doc comment names "id", "addresses", "relay" and states RelayAndAddresses is the default.
````

**U2. `sendme`**

````text
https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html: WebFetch failed (32k-token output cap); iroh builder signatures come from the published iroh 1.5.0 crate source instead.
````

**U3. `sendme`**

````text
iroh 1.0.0 and 1.3.0 source not read (only crates.io index metadata and Cargo.lock entries). All iroh signatures here are verified against 1.5.0; compatibility with 1.0.0/1.3.0 is inferred from ^1.0.0, not witnessed.
````

**U4. `sendme`**

````text
Exact postcard byte layout for sendme's BlobFormat::HashSeq ticket with non-empty relay_url / direct_addresses is not hexdumped in source read; test_ticket_base32 verifies only the Raw / no-relay / zero-address case (variant0 discriminator 0x00).
````

**U5. `sendme`**

````text
No build, no cargo metadata feature-unification run, no live send/receive execution: feature_flags are manifest declarations, not a resolved feature graph; NAT traversal, ALPN negotiation and ticket round-trip were not witnessed.
````

### 3.5 `iroh-dag-sync` -- 9 unverified items

**U1. `iroh-dag-sync`**

````text
No published version exists, so exact_version 0.1.0 could not be cross-checked against a crates.io release artifact; it is only the Cargo.toml value. There are zero published versions.
````

**U2. `iroh-dag-sync`**

````text
docs.rs returns 404, so no signature was cross-checked against generated rustdoc; all signatures come from source files only.
````

**U3. `iroh-dag-sync`**

````text
No cargo build/check/test was run here; compilability, runtime behavior, and protocol interop are unverified.
````

**U4. `iroh-dag-sync`**

````text
No Cargo.lock exists for iroh-dag-sync, so exact resolved versions of direct and transitive dependencies (and their real APIs) are unverified; the listed versions are manifest requirements only.
````

**U5. `iroh-dag-sync`**

````text
The rustc >= 1.75 requirement is inferred from language features and an unused CI MSRV variable; no build on 1.75 or any other toolchain was performed.
````

**U6. `iroh-dag-sync`**

````text
The exact on-wire byte layout of Request/SyncResponseHeader postcard encoding (including the 33-byte frame) is inferred from as_bytes()/from_stream() and the README, not from a captured packet or a dedicated header serialization test.
````

**U7. `iroh-dag-sync`**

````text
LICENSE-MIT and LICENSE-APACHE exist at the repo root (confirmed via contents listing) but their text was not read; the license value is taken from Cargo.toml and both READMEs.
````

**U8. `iroh-dag-sync`**

````text
CI workflow run results were not inspected; only the workflow definition was read.
````

**U9. `iroh-dag-sync`**

````text
The claim that iroh-gossip is unused is based on a full-text grep of all eight crate files, not on a build/link analysis.
````

### 3.6 `iroh-blobs` -- 5 unverified items

**U1. `iroh-blobs`**

````text
iroh_tickets::Ticket::encode_string / decode_string and iroh::EndpointAddr method signatures: external crates whose source was not read. Only BlobTicket's own impl Ticket (KIND, encode_bytes, decode_bytes), Display derive and FromStr impl in src/ticket.rs are verified.
````

**U2. `iroh-blobs`**

````text
Exact generated shape of irpc's macro-generated api::proto::Command enum: the #[rpc_requests(message = Command, ...)] attribute and docs saying Command exists were read; the macro expansion was not.
````

**U3. `iroh-blobs`**

````text
External type aliases (irpc::Result, irpc::Client, n0_error::Result, n0_error::AnyError, bao_tree::ChunkRanges, iroh::EndpointAddr) are used verbatim as written in iroh-blobs source; their defining crates were not read.
````

**U4. `iroh-blobs`**

````text
docs.rs cross-check for Hash inherent methods: struct.Hash.html was not successfully retrieved (enum.BlobFormat.html was); Hash signatures are verified against src/hash.rs only.
````

**U5. `iroh-blobs`**

````text
Empirical runtime behavior (restart recovery, GC, ticket string round-trips, download progress) was not executed; persistence claims come from source and source documentation only.
````

### 3.7 `n0-dns-resolver` -- 6 unverified items

**U1. `n0-dns-resolver`**

````text
Macro-expanded public items generated by #[stack_error(derive, add_meta, std_sources)] on n0_dns_resolver::Error, n0_dns_resolver::TransportError, iroh_dns::dns::DnsError and LookupError are not enumerated; only the source-level enum declarations were read. Generated trait impls (StackError, std::error::Error, Display) were not expanded.
````

**U2. `n0-dns-resolver`**

````text
No compilation against Rust 1.91 was performed; MSRV/edition are taken from Cargo.toml (rust-version = "1.91", edition = "2024") and the crates.io index record (rust_version = "1.91").
````

**U3. `n0-dns-resolver`**

````text
docs.rs root index page https://docs.rs/n0-dns-resolver/0.1.1/n0_dns_resolver/ displayed item labels only, not declarations; API completeness here is established by grepping all `pub` declarations in the v0.1.1 tag sources, not from a rustdoc all-items page.
````

**U4. `n0-dns-resolver`**

````text
cfg-dependent items are reported from source attributes only; no wasm_browser, Windows, Apple, Android, TLS-feature or no-default-features build was compiled or tested.
````

**U5. `n0-dns-resolver`**

````text
No live DNS queries, relay connections, or pkarr publishes were executed; all iroh relay/node-discovery claims are read from iroh tag v1.5.0 source and the n0-dns-resolver v0.1.1 example documentation.
````

**U6. `n0-dns-resolver`**

````text
The docs.rs page for individual type/struct pages other than struct.DnsResolver.html was not fetched; signatures for Builder, Nameserver, records, errors and public_resolvers come from the tag sources only.
````

### 3.8 `iroh-docs` -- 8 unverified items

**U1. `iroh-docs`**

````text
Docs::author: no such public method in iroh-docs 0.101.0. Verified absent by exhaustive grep of the published crate and tag; no signature supplied. Use DocsApi::author_create/author_default/author_list/author_set_default/author_export/author_import/author_delete instead.
````

**U2. `iroh-docs`**

````text
Doc::insert: no such public method in 0.101.0. Verified absent; use Doc::set_bytes or Doc::set_hash (or low-level Replica::insert / Replica::hash_and_insert).
````

**U3. `iroh-docs`**

````text
Doc::keys: no such public method in 0.101.0. Verified absent; use Doc::get_many(impl Into<Query>) / Doc::get_one with Query::all / Query::key_prefix / Query::key_exact.
````

**U4. `iroh-docs`**

````text
Doc::delete: no such public method in 0.101.0. Verified absent; use Doc::del(author_id, prefix) -> Result<usize> (prefix tombstone) or Replica::delete_prefix.
````

**U5. `iroh-docs`**

````text
Doc::import_export: no such public method in 0.101.0. Verified absent; use Doc::import_file / Doc::export_file, DocsApi::import / import_namespace / import_and_subscribe, and DocsApi::author_import / author_export.
````

**U6. `iroh-docs`**

````text
Doc::content_status (or Docs::content_status): no such public method in 0.101.0. Verified absent; content status is exposed as ContentStatus, ContentStatusCallback, LiveEvent::InsertRemote.content_status and LiveEvent::ContentReady. Doc::status() returns OpenState { sync, subscribers, handles }.
````

**U7. `iroh-docs`**

````text
Docs::new is NOT a storage-path constructor: the only exact signature is `pub fn new(engine: Engine) -> Self` (src/protocol.rs:48); construct through Docs::memory()/Docs::persistent(path) -> Builder::spawn(endpoint, blobs, gossip) or DocsApi::spawn(Arc<Engine>). Docs::open/create/list are inherited through Deref<Target = DocsApi>, not inherent methods.
````

**U8. `iroh-docs`**

````text
WebFetch of https://docs.rs/iroh-docs/0.101.0/iroh_docs/ (bare directory URL) returned 'No response from model'; the equivalent https://docs.rs/iroh-docs/0.101.0/iroh_docs/index.html page was fetched successfully and no API claim relies on the failed fetch.
````

### 3.9 `iroh-roq` -- 5 unverified items

**U1. `iroh-roq`**

````text
https://crates.io/api/v1/crates/iroh-roq returned HTTP 403; no metadata from that endpoint was used. All version data came from https://index.crates.io/ir/oh/iroh-roq and the downloaded .crate instead.
````

**U2. `iroh-roq`**

````text
Compatibility of iroh-roq with modern iroh (latest non-yanked 1.5.0) was NOT source-verified: I did not read iroh 1.x sources to check whether `iroh::endpoint::Connection::{send_datagram, read_datagram, open_uni, accept_uni}`, `Endpoint::builder().alpns(..)`, `Connection::alpn()`, or `Endpoint::connect(node_addr, ALPN)` still exist with these signatures. The type-mismatch conclusion follows only from the semver-distinct `Connection` type in `Session::new`.
````

**U3. `iroh-roq`**

````text
A WebFetch prose summary of docs.rs SendFlow/SendStream rendered `packet: &[RtpPacket]`; the raw docs.rs HTML code-headers and the published crate source both say `packet: &RtpPacket`. The summary was discarded; source is authoritative.
````

**U4. `iroh-roq`**

````text
No build, test, fuzz, or interoperability run was performed; runtime behavior, RoQ draft conformance, QUIC datagram MTU limits, and interop with the Go RoQ implementation named in README are unverified.
````

**U5. `iroh-roq`**

````text
No signature is asserted for `rtp::packet::Packet`, `iroh_quinn_proto::VarInt`, or `iroh::endpoint::Connection` themselves -- those are re-exported/third-party types whose own sources were not read for this contract.
````

### 3.10 `ministack` -- 7 unverified items

**U1. `ministack`**

````text
No live Docker daemon was contacted and no container was started in this session: the real-engine boot behaviour is sourced from code literals + README, not witnessed at runtime.
````

**U2. `ministack`**

````text
crates.io/docs.rs verification path is inapplicable: https://index.crates.io/mi/ni/ministack returns HTTP 404; ministack is a Python/PyPI package, so there is no Rust crate version or docs.rs signature page.
````

**U3. `ministack`**

````text
Exact Docker Hub image digests/tags for ministackorg/ministack:latest and :full were not read from a registry manifest; only Dockerfile, docker-compose.yml, README and CI references were read.
````

**U4. `ministack`**

````text
README marketing claims (~270MB/~110MB image size, ~30MB idle RAM, <2s startup, '60+ services') were not measured here; the SERVICE_REGISTRY count of 91 was computed from app.py source.
````

**U5. `ministack`**

````text
ministack/services/mq.py (AmazonMQ) and msk.py (MSK BYO broker) were not read; only README:455 and README:954-957/438 statements are cited, so their implementation details are unverified.
````

**U6. `ministack`**

````text
ministack/services/opensearchserverless.py and core/pgproxy.py were fetched but only grep-level evidence is cited; no exhaustive API extraction was done for them.
````

**U7. `ministack`**

````text
The gm skill was loaded as required by CLAUDE.md, but its spool daemon is not present in this working directory (one instruction dispatch returned no response); the extraction was performed directly with gh api/curl per the task's explicit evidence rules.
````

### 3.11 `iroh-smol-kv` -- 5 unverified items

**U1. `iroh-smol-kv`**

````text
Published versions 0.1.0-0.4.0: only sparse-index rows and crates.io version metadata were read; their src/lib.rs was not extracted. Signatures are exactly 0.5.0 only.
````

**U2. `iroh-smol-kv`**

````text
No declared MSRV: no rust-version in Cargo.toml, Cargo.toml.orig or published Cargo.toml; rust_version=null in all index rows and crates.io records; no CI MSRV file in HEAD tree. edition=2024; an edition-2024 toolchain floor was not measured against a compiler.
````

**U3. `iroh-smol-kv`**

````text
Foreign type signatures not extracted: iroh 1.4 PublicKey/SecretKey/EndpointId/Signature/SignatureError; iroh-gossip 0.103.0 GossipTopic/Event/GossipSender/GossipReceiver; irpc 1.0 Error/channel; n0-future 0.3.2 Stream/Future; bytes::Bytes; n0-error::Meta/StackError. Only their names as used here are reported.
````

**U4. `iroh-smol-kv`**

````text
n0-error's #[stack_error(...)] macro source was not read; InsertError's meta: Meta fields and trait impls come from rendered rustdoc for 0.5.0, not the macro crate.
````

**U5. `iroh-smol-kv`**

````text
Companion APIs not extracted: iroh-smol-kv-uniffi/src/code/*.rs and uniffi-example/src/lib.rs were listed but not read; iroh-smol-kv-uniffi 0.4.0 is git-only (index/API 404); uniffi-example/go/go.mod was not read. No runtime test, formal liveness/consensus proof, race analysis or gossip-convergence fuzz was performed; the verdict is static source analysis.
````

### 3.12 `laboon-cli` -- 6 unverified items

**U1. `laboon-cli`**

````text
No crates.io crate exists for this dependency: curl https://index.crates.io/la/bo/laboon -> 404 and https://index.crates.io/la/bo/laboon-cli -> 404; https://docs.rs/laboon/latest/laboon/ -> 404; https://crates.io/api/v1/crates/laboon -> 403. No published Rust version or docs.rs signature can be reported.
````

**U2. `laboon-cli`**

````text
docs/permissions-and-limits.md contains no platform support matrix (only three per-platform caveats), so the requested matrix cannot be extracted from it; the OS x arch matrix in this contract is derived from .goreleaser.yaml, README.md, AGENTS.md and the release asset names instead.
````

**U3. `laboon-cli`**

````text
The 12 SHA-256 sums are transcribed verbatim from the released checksums.txt (whose own sha256 was computed locally); the archive binaries themselves were not downloaded and re-hashed.
````

**U4. `laboon-cli`**

````text
Go 1.27.1 as a build requirement is verified from go.mod/go.work/README.md/CI, not by running `go build` or `go test` in this environment; no toolchain availability check was performed.
````

**U5. `laboon-cli`**

````text
laboond's HTTP handlers were read from source but not executed; live response behavior (hijack upgrades, stdcopy framing, stream timing) is not independently witnessed.
````

**U6. `laboon-cli`**

````text
The CLI binaries in the release were not executed; the CLI surface is derived from cmd/laboon/main.go source at the exact tag, not from `--help` output.
````

### 3.13 `quic-rpc` -- 0 unverified items

- (none)
### 3.14 `iroh` -- 9 unverified items

**U1. `iroh`**

````text
NodeId: ABSENT in iroh 1.5.0 (source checked: full recursive git tree at v1.5.0, iroh/src/lib.rs:285-288 re-exports, iroh-base/src/key.rs:70, docs.rs/iroh/1.5.0/iroh/index.html). Use `iroh::EndpointId` = `pub type EndpointId = PublicKey`.
````

**U2. `iroh`**

````text
NodeAddr: ABSENT in iroh 1.5.0 (same sources; CHANGELOG.md:828 'Rename Node to Endpoint in all cases'). Use `iroh::EndpointAddr`.
````

**U3. `iroh`**

````text
Discovery / Discovery builder / `Builder::discovery(...)`: ABSENT in iroh 1.5.0 (iroh/src/address_lookup.rs:144,335; CHANGELOG.md:727 'Rename `Discovery` to `AddressLookup`'). Use `iroh::address_lookup::{AddressLookup, AddressLookupBuilder}` and `Builder::address_lookup(...)`; the `presets::N0` preset installs DnsAddressLookup + PkarrPublisher + PkarrResolver.
````

**U4. `iroh`**

````text
NodeTicket / iroh::ticket module / iroh::NodeTicket: ABSENT in iroh 1.5.0 (the v1.5.0 recursive tree has zero paths containing 'ticket'; docs.rs root has no ticket module; CHANGELOG.md:826 'Move tickets into their own crate'). Use the separate crate `iroh-tickets = "1.0.0"`, type `iroh_tickets::endpoint::EndpointTicket`. No crate named `iroh-ticket` exists on the crates.io index (HTTP 404 for https://index.crates.io/ir/oh/iroh-ticket).
````

**U5. `iroh`**

````text
ProtocolHandler::handle_connection: ABSENT as a trait method (docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html shows only accept/on_accepting/shutdown; iroh/src/protocol.rs:636 defines a PRIVATE free fn). Do not implement it.
````

**U6. `iroh`**

````text
Endpoint::builder() (zero-argument), Endpoint::relay_mode(...), Endpoint::alpns(...), Endpoint::discovery(...): ABSENT in 1.5.0. `Endpoint::builder` requires `preset: impl Preset`; relay_mode/alpns live on `endpoint::Builder`; `Endpoint::set_alpns` is the live-Endpoint setter.
````

**U7. `iroh`**

````text
https://crates.io/api/v1/crates/iroh/1.5.0 returned HTTP 403 from this host and was NOT used as evidence; exact published version, checksum, yanked status, rust_version and features came from https://index.crates.io/ir/oh/iroh plus the checksum-verified published crate tarball. No signature depends on the 403'd endpoint.
````

**U8. `iroh`**

````text
docs.rs/iroh-base/1.5.0/iroh_base/struct.EndpointAddr.html returned no model response; EndpointAddr signatures were instead read from iroh-base/src/endpoint_addr.rs at tag v1.5.0 and cross-checked at https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html.
````

**U9. `iroh`**

````text
EndpointAddr Display / FromStr: NOT implemented in iroh-base 1.5.0 (verified by the full impl list in iroh-base/src/endpoint_addr.rs and docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html). Do not assume a string form; use serde or iroh-tickets.
````

## 4. SMALLEST COHERENT PINNED DEPENDENCY SET THAT BUILDS TOGETHER

### 4.1 The set

````text
[dependencies]
iroh = "=1.5.0"
iroh-blobs = "=0.103.1"
iroh-docs = "=0.101.0"
iroh-gossip = "=0.101.0"
quic-rpc = { version = "=0.20.0", default-features = false, features = ["flume-transport"] }
tokio = { version = "1", features = ["full"] }
````

| crate | pin | why this exact version | evidence |
|---|---|---|---|
| `iroh` | `=1.5.0` | 1.x API (EndpointId/EndpointAddr/AddressLookup), MSRV 1.91; all four companions accept ^1 / ^1.4 | `https://index.crates.io/ir/oh/iroh` |
| `iroh-blobs` | `=0.103.1` | only unyanked 0.103.x satisfying iroh-docs 0.101.0's `^0.103`; uses irpc 0.17 | `https://index.crates.io/ir/oh/iroh-blobs` |
| `iroh-docs` | `=0.101.0` | latest published iroh-docs (34 index versions, last 0.101.0); CRDT document substrate | `https://index.crates.io/ir/oh/iroh-docs` |
| `iroh-gossip` | `=0.101.0` | only 0.101.x satisfying iroh-docs 0.101.0's `^0.101.0` with feature `net` | `https://index.crates.io/ir/oh/iroh-gossip` |
| `quic-rpc` | `=0.20.0` | latest published quic-rpc; transport-agnostic with default `flume-transport` | `https://index.crates.io/qu/ic/quic-rpc` |

**Feature constraints**

- `quic-rpc`: default-features = false, features = ["flume-transport"]; the iroh-transport feature MUST stay off (it pulls iroh ^0.35)
- `iroh`: default features (metrics, fast-apple-datapath, portmapper, tls-ring) are fine
- `iroh-gossip`: default net+metrics; iroh-docs 0.101.0 requires its net feature
- `iroh-docs`: default metrics, rpc, fs-store, redb-v2-migration are fine
- `iroh-blobs`: default hide-proto-docs, fs-store, rpc are fine

**Transitive pins resolved by this set** (from the witness `Cargo.lock`; these do not need to be declared directly)

| crate | version |
|---|---|
| `iroh-base` | `1.5.0` |
| `iroh-tickets` | `1.0.0` |
| `irpc` | `0.17.0` |
| `noq` | `1.5.0` |
| `n0-watcher` | `1.0.1` |
| `n0-dns-resolver` | `0.1.1` |
| `iroh-dns` | `1.5.0` |
| `portmapper` | `0.20.2` |
| `netwatch` | `0.20.2` |

**Toolchain:** Rust 1.91+ (iroh 1.5.0, iroh-blobs 0.105.0/0.103.1, iroh-docs 0.101.0, iroh-gossip 0.103.0/0.101.0 all declare rust_version 1.91; quic-rpc 0.20.0 declares 1.76). Witness build used rustc 1.98.1.

### 4.2 Live build witness

- Command: `cargo check --offline --manifest-path /Users/dylanwong/projects/aws0-research-2026-11-19/_sheet-inputs/verify-probe/Cargo.toml --target-dir /Users/dylanwong/projects/aws0-research-2026-11-19/_sheet-inputs/target`
- Exit code: **0**; last line: ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 56s``.
- Log: `/Users/dylanwong/projects/aws0-research-2026-11-19/_sheet-inputs/cargo-check-coherent.log`; lock: `/Users/dylanwong/projects/aws0-research-2026-11-19/_sheet-inputs/verify-probe/Cargo.lock`; manifest: `/Users/dylanwong/projects/aws0-research-2026-11-19/_sheet-inputs/verify-probe/Cargo.toml`.
- `cargo tree -d` on that manifest shows **no duplicate versions** among `iroh`, `iroh-blobs`, `iroh-docs`, `iroh-gossip`, `quic-rpc`, `irpc`, `iroh-base`, `iroh-tickets`, `noq`, `n0-watcher`. Remaining duplicates are unrelated transitive crates (`base64`, `syn`, `redb`, `windows-*`, ...) which Cargo permits and which do not cross these APIs.
- The naive five-crate probe at `/Users/dylanwong/projects/aws0-deps-2026-11-19/probe/` also compiles, but its lock (`/Users/dylanwong/projects/aws0-deps-2026-11-19/probe/Cargo.lock`) resolves **two** copies each of `iroh-blobs` (0.103.1 + 0.105.0), `iroh-gossip` (0.101.0 + 0.103.0) and `irpc` (0.17.0 + 1.0.0).

### 4.3 Compatibility matrix (crates.io sparse-index evidence)

| crate@version | rust_version | yanked | cksum | relevant deps |
|---|---|---|---|---|
| `iroh@1.5.0` | 1.91 | False | `40ac5d8330439176254581b6db4bc6f1d947d7cc741fda4e04b96d758addf1db` | iroh-base ^1.5.0; iroh-base ^1.5.0; iroh-dns ^1.5.0; iroh-relay ^1.4.2; iroh-relay ^1.4.2; n0-error ^1.0.0; n0-error ^1.0.0; n0-future ^0.3 |
| `iroh-blobs@0.105.0` | 1.91 | False | `b9ef502f80b48a0d87bc6b4212a9140ecc2915e2683cb3f2606b9241fec4349a` | iroh ^1.0.0; iroh ^1.0.0; iroh-base ^1.0.0; iroh-tickets ^1.0.0; irpc ^1.0.0; n0-error ^1.0.0; n0-future ^0.3.0; noq ^1.0.0 (optional) |
| `iroh-blobs@0.103.1` | 1.91 | False | `a3148ad01585eb6b7cfb641e4100564f4f56db644f4cbed8c10ab2478663161f` | iroh ^1.0.0; iroh ^1.0.0; iroh-base ^1.0.0; iroh-tickets ^1.0.0; irpc ^0.17.0; n0-error ^1.0.0; n0-future ^0.3.0; noq ^1.0.0 (optional) |
| `iroh-blobs@0.103.0` | 1.91 | True | `5be50b0e2d0a9ba65cee4e0dfb708b3704e02ad12bd4c14c6307e94245943126` | iroh ^1.0.0; iroh ^1.0.0; iroh-base ^1.0.0; iroh-tickets ^1.0.0; irpc ^0.17.0; n0-error ^1.0.0; n0-future ^0.3.0; noq ^1.0.0 (optional) |
| `iroh-docs@0.101.0` | 1.91 | False | `8fd1bd5e39d0321a3c4a2bcef9650476c076e2df41a0e84577eca23d6de6c8ab` | iroh ^1; iroh ^1; iroh-blobs ^0.103; iroh-gossip ^0.101.0; iroh-tickets ^1; irpc ^0.17; n0-error ^1; n0-future ^0.3.1 |
| `iroh-gossip@0.103.0` | 1.91 | False | `74c4170847b5603f489c4d7771f1e7b263fa63a2521dbbb747601931987aa777` | iroh ^1.4 (optional); iroh ^1; iroh-base ^1; n0-error ^1; n0-future ^0.3 |
| `iroh-gossip@0.101.0` | 1.91 | False | `4e1dc4b05f73e7a1b9e83b531eb63c3fd671b0af3aeb13b59c546dd7ca747515` | iroh ^1 (optional); iroh ^1; iroh-base ^1; irpc ^0.17.0 (optional); n0-error ^1; n0-future ^0.3; noq ^1 (optional) |
| `quic-rpc@0.20.0` | 1.76 | False | `18bad98bd048264ceb1361ff9d77a031535d8c1e3fe8f12c6966ec825bf68eb7` | iroh ^0.35 (optional); quinn ^0.13 (optional); quinn ^0.13 |

Semver rules that decide the outcome:

- ^1 and ^1.0.0 and ^1.4 all admit iroh 1.5.0 (one iroh 1.x universe).
- ^0.103 admits only >=0.103.0,<0.104.0; iroh-blobs 0.105.0 is excluded.
- ^0.101.0 admits only >=0.101.0,<0.102.0; iroh-gossip 0.103.0 is excluded.
- ^0.35 (quic-rpc iroh-transport) is a 0.x iroh universe, not iroh 1.5.0.
- ^0.17 vs ^1.0.0 for irpc are different major universes.

**Caveat.** This coherent five-crate set deliberately uses `iroh-blobs 0.103.1` and `iroh-gossip 0.101.0`, i.e. **older versions than the two surviving contracts in section 1** (0.105.0 and 0.103.0). The exact signatures in sections 1.6 and 1.3 are therefore authoritative for their own pinned versions but are NOT the API of the coherent build. Re-verify any blobs/gossip call against the downgraded versions before writing code. If the contracted 0.105.0/0.103.0 APIs are mandatory, the coherent alternative is a four-crate set (`iroh 1.5.0` + `iroh-blobs 0.105.0` + `iroh-gossip 0.103.0` + `quic-rpc 0.20.0`) that omits `iroh-docs` -- no published `iroh-docs` accepts those versions.

## 5. HARD BLOCKERS (with evidence)

`severity: hard` = a version/type incompatibility that prevents a single coherent iroh 1.5.0 stack. `severity: constraint` = a pinning rule that must be obeyed.

### B1 -- [HARD] iroh-docs 0.101.0 excludes iroh-blobs 0.105.0 (the contracted blobs version)

iroh-docs 0.101.0 declares iroh-blobs = "^0.103". For a 0.x crate, ^0.103 means >=0.103.0,<0.104.0, so 0.105.0 is outside the range. Cargo therefore resolves TWO copies (0.103.1 for docs, 0.105.0 for the direct dep). iroh_docs::engine::Engine::spawn takes iroh_blobs::api::Store and iroh_docs::protocol::Builder::spawn takes BlobsStore, so a 0.105.0 Store cannot be passed to a Docs built against 0.103.1. The 800-signatures sheet documents 0.105.0: those exact signatures are NOT usable in a single-version stack with iroh-docs 0.101.0.

Evidence:

- `https://index.crates.io/ir/oh/iroh-docs` -- 0.101.0 deps: iroh-blobs ^0.103, iroh ^1, irpc ^0.17, noq ^1 (optional)
- `/Users/dylanwong/projects/aws0-deps-2026-11-19/probe/Cargo.lock` -- live Cargo.lock contains both iroh-blobs 0.103.1 and 0.105.0
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-docs.json` -- Engine::spawn / Builder::spawn signatures with iroh_blobs::api::Store / BlobsStore
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/sendme.json` -- sendme 0.37.1 notes ^0.103.1 excludes 0.104.0/0.104.1/0.105.0

**Resolution:** Pin iroh-blobs = "=0.103.1" when iroh-docs 0.101.0 is in the graph. Accept that the sheet's 0.105.0 signatures then do not apply, or drop iroh-docs (no newer iroh-docs exists on crates.io).

### B2 -- [HARD] iroh-docs 0.101.0 excludes iroh-gossip 0.103.0 (the contracted gossip version)

iroh-docs 0.101.0 declares iroh-gossip = "^0.101.0" with features ["net"]; ^0.101.0 means >=0.101.0,<0.102.0. A direct iroh-gossip 0.103.0 dependency therefore resolves a second copy. iroh_docs::protocol::Builder::spawn(self, endpoint: Endpoint, blobs: BlobsStore, gossip: Gossip) and Engine::spawn(..., gossip: Gossip, ...) consume the 0.101.0 Gossip type, so a 0.103.0 Gossip will not type-check. Moreover 0.103.0 removed the rpc/rpc-tls-* features and irpc/noq dependencies that 0.101.0 has (surviving iroh-gossip contract), so the two APIs differ.

Evidence:

- `https://index.crates.io/ir/oh/iroh-docs` -- 0.101.0 deps: iroh-gossip ^0.101.0, features ["net"]
- `https://index.crates.io/ir/oh/iroh-gossip` -- 0.103.0 features2: no rpc, no rpc-tls-ring/aws-lc-rs; deps have no irpc/noq
- `https://index.crates.io/ir/oh/iroh-gossip` -- 0.101.0 features2 includes rpc/rpc-tls-*, deps include irpc ^0.17 and noq ^1
- `/Users/dylanwong/projects/aws0-deps-2026-11-19/probe/Cargo.lock` -- live Cargo.lock contains both iroh-gossip 0.101.0 and 0.103.0
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-docs.json` -- Builder::spawn / Engine::spawn take Gossip

**Resolution:** Pin iroh-gossip = "=0.101.0" for the five-crate set, and re-verify any gossip API used (this sheet documents 0.103.0). If 0.103.0 is mandatory, do not include iroh-docs 0.101.0.

### B3 -- [HARD] irpc 0.17 vs irpc 1.0 split at the iroh-docs / iroh-blobs RPC boundary

iroh-docs 0.101.0 and iroh-blobs 0.103.1 both require irpc ^0.17, while iroh-blobs 0.105.0 requires irpc ^1.0.0. Using the contracted blobs 0.105.0 with docs 0.101.0 resolves both irpc 0.17.0 and 1.0.0; irpc::Client<proto::Request> from one does not unify with the other. Store::from_client(client: impl Into<irpc::Client<proto::Request>>) in the 0.105.0 contract is the concrete boundary.

Evidence:

- `https://index.crates.io/ir/oh/iroh-blobs` -- 0.105.0 deps: irpc ^1.0.0
- `https://index.crates.io/ir/oh/iroh-blobs` -- 0.103.1 deps: irpc ^0.17.0
- `https://index.crates.io/ir/oh/iroh-docs` -- 0.101.0 deps: irpc ^0.17
- `/Users/dylanwong/projects/aws0-deps-2026-11-19/probe/Cargo.lock` -- live Cargo.lock contains both irpc 0.17.0 and irpc 1.0.0
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-blobs.json` -- Store::from_client(impl Into<irpc::Client<proto::Request>>)

**Resolution:** Use iroh-blobs 0.103.1 so the whole stack resolves a single irpc 0.17.0 (confirmed by the witness build).

### B4 -- [HARD] quic-rpc 0.20.0 iroh-transport feature pins iroh ^0.35, incompatible with iroh 1.5.0

quic-rpc 0.20.0 has an optional iroh dependency with req ^0.35, activated only by features2.iroh-transport = ["dep:iroh", ...]. Enabling it in an iroh 1.5.0 build resolves a second, 0.x iroh whose Endpoint/Connection types are unrelated to iroh 1.5.0's. quic-rpc 0.20.0 is otherwise transport-agnostic (default = flume-transport; quinn-transport uses the renamed iroh-quinn 0.13 dependency, not iroh).

Evidence:

- `https://index.crates.io/qu/ic/quic-rpc` -- 0.20.0 deps: iroh ^0.35 optional; features2.iroh-transport enables dep:iroh; default = flume-transport
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/quic-rpc.json` -- quinn dep is package="iroh-quinn" version 0.13 renamed to quinn; no Message trait
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-roq.json` -- independent corroboration: a 0.x iroh Connection cannot be handed to an iroh 1.x service without a port

**Resolution:** Declare quic-rpc = { version = "=0.20.0", default-features = false, features = ["flume-transport"] } (or quinn-transport/hyper-transport) and never enable iroh-transport. For iroh-native RPC use irpc 0.17/1.0 or iroh's own ProtocolHandler instead.

### B5 -- [HARD] Other surviving contracts that cannot be wired into an iroh 1.5.0 build

iroh-roq 0.1.0 (the only published release) requires iroh ^0.33; its unpublished HEAD requires 0.35. iroh-dag-sync pins iroh = "1.0.0-rc.1", iroh-blobs = "0.102", iroh-gossip = "0.100" and is a binary-only experiment with no linkable library target. sendme 0.37.1 is binary-only (no src/lib.rs), so it is not a library dependency at all. These are hard blockers for including those contracts in the same iroh 1.5.0 stack, even though their own signatures were source-verified.

Evidence:

- `/Users/dylanwong/projects/aws0-research-2026-11-19/_refute-iroh-roq/dl/index.iroh-roq.json` -- iroh-roq 0.1.0 index line: iroh ^0.33, rust_version 1.81
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-roq.json` -- published 0.1.0 pins iroh 0.33; HEAD main pins 0.35 and is unpublished
- `/Users/dylanwong/projects/aws0-research-2026-11-19/findings/.verify-iroh-dag-sync/Cargo.toml` -- iroh = "1.0.0-rc.1", iroh-blobs = "0.102", iroh-gossip = "0.100"
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/iroh-dag-sync.json` -- experiment, bin-only, no lib target, no CRDT merge semantics
- `/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/sendme.json` -- binary-only: src/main.rs, no src/lib.rs

**Resolution:** Keep these out of the pinned build; treat them as reference implementations, or vendor and port them to iroh 1.5.0.

### B6 -- [CONSTRAINT] iroh-blobs 0.103.0 is yanked; pin 0.103.1

The crates.io sparse index marks iroh-blobs 0.103.0 yanked=true. iroh-docs 0.101.0's ^0.103 range therefore must be satisfied by 0.103.1 (unyanked, rust_version 1.91).

Evidence:

- `https://index.crates.io/ir/oh/iroh-blobs` -- yanked = true
- `https://index.crates.io/ir/oh/iroh-blobs` -- yanked = false, cksum a3148ad01585eb6b7cfb641e4100564f4f56db644f4cbed8c10ab2478663161f

**Resolution:** iroh-blobs = "=0.103.1".

## 6. Source URL index

293 distinct URLs appear in the 14 contracts' `source_refs` and per-signature `source` fields. They are reproduced here for auditability; the per-signature source above is authoritative for each exact signature.

- <https://api.github.com/repos/n0-computer/net-tools/git/trees/HEAD?recursive=1>
- <https://api.github.com/repos/n0-computer/net-tools/commits/HEAD>
- <https://api.github.com/repos/n0-computer/net-tools/tags>
- <https://api.github.com/repos/n0-computer/net-tools/contents/Cargo.toml>
- <https://api.github.com/repos/n0-computer/net-tools/contents/README.md>
- <https://api.github.com/repos/n0-computer/net-tools/contents/Cargo.toml?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/README.md?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/Cargo.lock?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/Cargo.toml?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/README.md?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/portmapper/examples/monitor_and_probe.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/Cargo.toml?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/README.md?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/lib.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/interfaces.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/netmon.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/contents/netwatch/src/interfaces/netdev_impl.rs?ref=334793cfdaf3487db0c721b9312179a8ae3b0dcb>
- <https://api.github.com/repos/n0-computer/net-tools/commits?path=netdev/Cargo.toml&per_page=5>
- <https://api.github.com/repos/n0-computer/net-tools/commits?path=nested_enum_utils/Cargo.toml&per_page=5>
- <https://api.github.com/repos/n0-computer/net-tools/git/trees/89782fb542d217fd0d59975e13c828f4a1adf0ae?recursive=1>
- <https://api.github.com/search/code?q=repo%3An0-computer%2Fnet-tools+nested_enum_utils>
- <https://api.github.com/search/code?q=repo%3An0-computer%2Fnet-tools+netdev>
- <https://index.crates.io/po/rt/portmapper>
- <https://index.crates.io/ne/tw/netwatch>
- <https://index.crates.io/ne/td/netdev>
- <https://index.crates.io/ne/st/nested_enum_utils>
- <https://index.crates.io/ne/t-/net-tools>
- <https://static.crates.io/crates/portmapper/portmapper-0.20.2.crate>
- <https://static.crates.io/crates/netwatch/netwatch-0.20.2.crate>
- <https://static.crates.io/crates/netdev/netdev-0.47.0.crate>
- <https://static.crates.io/crates/nested_enum_utils/nested_enum_utils-0.2.3.crate>
- <https://docs.rs/portmapper/0.20.2/portmapper/>
- <https://docs.rs/portmapper/0.20.2/portmapper/struct.Client.html>
- <https://docs.rs/netwatch/0.20.2/netwatch/>
- <https://docs.rs/netwatch/0.20.2/netwatch/interfaces/struct.State.html>
- <https://docs.rs/netdev/0.47.0/netdev/>
- <https://docs.rs/netdev/0.47.0/netdev/interface/fn.get_interfaces.html>
- <https://docs.rs/nested_enum_utils/0.2.3/nested_enum_utils/>
- <https://docs.rs/nested_enum_utils/0.2.3/nested_enum_utils/attr.enum_conversions.html>
- <https://index.crates.io/to/ki/tokio-rustls-acme>
- <https://crates.io/api/v1/crates/tokio-rustls-acme/0.10.0>
- <https://static.crates.io/crates/tokio-rustls-acme/tokio-rustls-acme-0.10.0.crate>
- <https://github.com/n0-computer/tokio-rustls-acme>
- <https://api.github.com/repos/n0-computer/tokio-rustls-acme/tags?per_page=100>
- <https://api.github.com/repos/n0-computer/tokio-rustls-acme/commits/main>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/lib.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/config.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/acceptor.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/incoming.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/axum.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/state.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/resolver.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/cache.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/caches/dir.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/src/acme.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/Cargo.toml>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/README.md>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/high_level.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/low_level.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/low_level_axum.rs>
- <https://raw.githubusercontent.com/n0-computer/tokio-rustls-acme/4017266013e0bd00bae83e9edc410f3624f80815/examples/high_level_warp.rs>
- <https://docs.rs/tokio-rustls-acme/0.10.0/tokio_rustls_acme/index.html>
- <https://docs.rs/crate/tokio-rustls-acme/0.10.0/builds>
- <https://index.crates.io/ir/oh/iroh-gossip>
- <https://crates.io/api/v1/crates/iroh-gossip/0.103.0>
- <https://static.crates.io/crates/iroh-gossip/iroh-gossip-0.103.0.crate>
- <https://github.com/n0-computer/iroh-gossip>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/Cargo.toml?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/README.md?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/lib.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/api.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/net.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/error.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/proto.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/proto/state.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/src/proto/util.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/examples/chat.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/contents/examples/setup.rs?ref=v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/git/trees/v0.103.0?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh-gossip/git/ref/tags/v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh-gossip/git/tags/dd1c197f85385d6e0f534791d81ca17cf16a4c76>
- <https://api.github.com/repos/n0-computer/iroh-gossip/commits/e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106>
- <https://github.com/n0-computer/iroh-gossip/tree/v0.103.0>
- <https://api.github.com/repos/n0-computer/iroh/git/trees/HEAD?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh/git/matching-refs/tags/iroh-gossip>
- <https://api.github.com/repos/n0-computer/iroh/commits/e135cf36bc7eeba1a3a6e3054ba6d4f2aabc7106>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Gossip.html>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.Builder.html>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipTopic.html>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.GossipSender.html>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/struct.TopicId.html>
- <https://docs.rs/iroh-gossip/0.103.0/iroh_gossip/constant.ALPN.html>
- <https://index.crates.io/ir/oh/iroh>
- <https://index.crates.io/ir/oh/iroh-blobs>
- <https://index.crates.io/se/nd/sendme>
- <https://crates.io/api/v1/crates/sendme/0.37.1/download>
- <https://crates.io/api/v1/crates/iroh/1.5.0/download>
- <https://crates.io/api/v1/crates/iroh-blobs/0.103.1/download>
- <https://crates.io/api/v1/crates/iroh-tickets/1.0.0/download>
- <https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/protocol/constant.ALPN.html>
- <https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/ticket/struct.BlobTicket.html>
- <https://docs.rs/iroh-blobs/0.103.1/iroh_blobs/index.html>
- <https://docs.rs/iroh/1.5.0/iroh/protocol/struct.RouterBuilder.html>
- <https://crates.io/api/v1/crates/iroh-base/1.5.0/download>
- <https://api.github.com/repos/n0-computer/iroh-experiments>
- <https://api.github.com/repos/n0-computer/iroh-experiments/git/trees/HEAD?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/Cargo.toml>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/README.md>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/args.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/main.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/protocol.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/sync.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/tables.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/traversal.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/src/util.rs>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/iroh-dag-sync/.gitignore>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/README.md>
- <https://api.github.com/repos/n0-computer/iroh-experiments/contents/.github/workflows/ci.yml>
- <https://api.github.com/repos/n0-computer/iroh-experiments/commits/main>
- <https://api.github.com/repos/n0-computer/iroh-experiments/commits?path=iroh-dag-sync&per_page=3>
- <https://api.github.com/repos/n0-computer/iroh-experiments/tags?per_page=100>
- <https://github.com/n0-computer/iroh-experiments/blob/41ebc455678d7789459add9c2218d463bc74866a/iroh-dag-sync/>
- <https://index.crates.io/ir/oh/iroh-dag-sync>
- <https://crates.io/api/v1/crates/iroh-dag-sync>
- <https://docs.rs/iroh-dag-sync/>
- <https://docs.rs/iroh-dag-sync/latest/iroh_dag_sync/>
- <https://raw.githubusercontent.com/n0-computer/iroh-experiments/main/iroh-dag-sync/src/lib.rs>
- <https://crates.io/api/v1/crates/iroh-blobs/0.105.0>
- <https://static.crates.io/crates/iroh-blobs/iroh-blobs-0.105.0.crate>
- <https://api.github.com/repos/n0-computer/iroh-blobs/tags>
- <https://api.github.com/repos/n0-computer/iroh-blobs/git/trees/HEAD?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh-blobs/git/trees/v0.105.0?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh-blobs/contents/<path>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/api/struct.Store.html>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/api/blobs/struct.Blobs.html>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/store/fs/struct.FsStore.html>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/store/mem/struct.MemStore.html>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/ticket/struct.BlobTicket.html>
- <https://docs.rs/iroh-blobs/0.105.0/iroh_blobs/enum.BlobFormat.html>
- <https://index.crates.io/n0/-d/n0-dns-resolver>
- <https://static.crates.io/crates/n0-dns-resolver/n0-dns-resolver-0.1.1.crate>
- <https://docs.rs/n0-dns-resolver/0.1.1/n0_dns_resolver/>
- <https://docs.rs/n0-dns-resolver/0.1.1/n0_dns_resolver/struct.DnsResolver.html>
- <https://index.crates.io/ir/oh/iroh-docs>
- <https://static.crates.io/crates/iroh-docs/iroh-docs-0.101.0.crate>
- <https://api.github.com/repos/n0-computer/iroh-docs/git/trees/HEAD?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh-docs/git/trees/v0.101.0?recursive=1>
- <https://api.github.com/repos/n0-computer/iroh-docs/tags?per_page=100>
- <https://api.github.com/repos/n0-computer/iroh-docs/git/tags/39453262e639cb1004d493e4a82f9995bc4bb604>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/lib.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/api.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/actor.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/engine.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/api/protocol.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/api/actor.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/keys.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/sync.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/protocol.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/ticket.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/store.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/src/net.rs?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/Cargo.toml?ref=v0.101.0>
- <https://api.github.com/repos/n0-computer/iroh-docs/contents/README.md?ref=v0.101.0>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/index.html>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.DocsApi.html>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/api/struct.Doc.html>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/protocol/struct.Docs.html>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/engine/struct.Engine.html>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/struct.Entry.html>
- <https://docs.rs/iroh-docs/0.101.0/iroh_docs/sync/enum.ContentStatus.html>
- <https://index.crates.io/ir/oh/iroh-roq>
- <https://static.crates.io/crates/iroh-roq/iroh-roq-0.1.0.crate>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.Session.html>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendFlow.html>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.ReceiveFlow.html>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/struct.SendStream.html>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/constant.ALPN.html>
- <https://docs.rs/crate/iroh-roq/0.1.0/source/src/lib.rs>
- <https://github.com/n0-computer/iroh/blob/v0.35.0/iroh/src/protocol.rs>
- <https://github.com/n0-computer/iroh-roq/blob/main/README.md>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/lib.rs>
- <https://github.com/n0-computer/iroh-roq/blob/v0.1.0/src/lib.rs>
- <https://docs.rs/iroh-roq/0.1.0/iroh_roq/index.html>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/session.rs>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/send_flow.rs>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/receive_flow.rs>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/send_stream.rs>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/{send_flow>
- <https://github.com/n0-computer/iroh-roq/blob/main/src/{lib>
- <https://github.com/n0-computer/iroh-roq/blob/v0.1.0/Cargo.toml>
- <https://github.com/n0-computer/iroh-roq/blob/main/Cargo.toml>
- <https://pypi.org/pypi/ministack/json>
- <https://pypi.org/pypi/ministack/1.5.43/json>
- <https://index.crates.io/mi/ni/ministack>
- <https://index.crates.io/ir/oh/iroh-smol-kv>
- <https://index.crates.io/ir/oh/iroh-smol-kv-uniffi>
- <https://crates.io/api/v1/crates/iroh-smol-kv>
- <https://crates.io/api/v1/crates/iroh-smol-kv-uniffi>
- <https://static.crates.io/crates/iroh-smol-kv/iroh-smol-kv-0.5.0.crate>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/all.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Client.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.WriteScope.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Filter.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Config.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.ExpiryConfig.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.IterResult.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.SubscribeResponse.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/struct.Subscribe.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.SubscribeItem.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.SubscribeMode.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/enum.InsertError.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/struct.SignedValue.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/proto/enum.GossipMessage.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/util/index.html>
- <https://docs.rs/crate/iroh-smol-kv/0.5.0/features>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/fn.parse_filter.html>
- <https://docs.rs/iroh-smol-kv/0.5.0/iroh_smol_kv/index.html>
- <https://api.github.com/repos/samyfodil/laboon>
- <https://api.github.com/repos/samyfodil/laboon/git/trees/HEAD?recursive=1>
- <https://api.github.com/repos/samyfodil/laboon/git/trees/72b4fb91209d8d9576749da097c3fda4f9365f6f?recursive=1>
- <https://api.github.com/repos/samyfodil/laboon/commits/main>
- <https://api.github.com/repos/samyfodil/laboon/git/ref/tags/v0.1.0>
- <https://api.github.com/repos/samyfodil/laboon/releases>
- <https://github.com/samyfodil/laboon/releases/download/v0.1.0/checksums.txt>
- <https://proxy.golang.org/github.com/samyfodil/laboon/@v/list>
- <https://index.crates.io/la/bo/laboon>
- <https://index.crates.io/la/bo/laboon-cli>
- <https://docs.rs/laboon/latest/laboon/>
- <https://api.github.com/search/code?q=WithMountReadOnly+repo:samyfodil/laboon>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/laboon.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/options.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/options.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/cmd/laboon/main.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/fs.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/filesystems/hostfs/hostdirfs.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/laboon.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboon/main.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/daemon.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/main.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/handlers.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/cmd/laboond/handshake.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/go.mod>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/go.work>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/README.md>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/.github/workflows/release.yml>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/docs/permissions-and-limits.md>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/.goreleaser.yaml>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/AGENTS.md>
- <https://api.github.com/repos/samyfodil/laboon/releases/latest>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/LICENSE>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/example_test.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/internal/sandbox/egress.go>
- <https://github.com/samyfodil/laboon/blob/ad7a15ba1997d6fb3ec24b1970a75015f40285ce/internal/sandbox/config.go>
- <https://github.com/samyfodil/laboon/blob/72b4fb91209d8d9576749da097c3fda4f9365f6f/internal/sandbox/config.go>
- <https://github.com/n0-computer/quic-rpc/blob/v0.20.0/<path>
- <https://index.crates.io/qu/ic/quic-rpc>
- <https://static.crates.io/crates/quic-rpc/quic-rpc-0.20.0.crate>
- <https://docs.rs/quic-rpc/0.20.0/quic_rpc/client/struct.RpcClient.html>
- <https://docs.rs/quic-rpc/0.20.0/quic_rpc/server/struct.RpcServer.html>
- <https://docs.rs/quic-rpc/0.20.0/quic_rpc/transport/quinn/struct.QuinnListener.html>
- <https://static.crates.io/crates/iroh/iroh-1.5.0.crate>
- <https://index.crates.io/ir/oh/iroh-base>
- <https://index.crates.io/n0/-w/n0-watcher>
- <https://static.crates.io/crates/n0-watcher/n0-watcher-1.0.0.crate>
- <https://static.crates.io/crates/n0-watcher/n0-watcher-1.0.1.crate>
- <https://index.crates.io/ir/oh/iroh-tickets>
- <https://static.crates.io/crates/iroh-tickets/iroh-tickets-1.0.0.crate>
- <https://index.crates.io/ir/oh/iroh-ticket>
- <https://index.crates.io/3/n/noq>
- <https://docs.rs/iroh/1.5.0/iroh/index.html>
- <https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Builder.html>
- <https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Endpoint.html>
- <https://docs.rs/iroh/1.5.0/iroh/endpoint/struct.Connection.html>
- <https://docs.rs/iroh/1.5.0/iroh/endpoint/enum.RelayMode.html>
- <https://docs.rs/iroh/1.5.0/iroh/protocol/trait.ProtocolHandler.html>
- <https://docs.rs/iroh/1.5.0/iroh/protocol/struct.Router.html>
- <https://docs.rs/iroh/1.5.0/iroh/address_lookup/trait.AddressLookup.html>
- <https://docs.rs/iroh/1.5.0/iroh/address_lookup/trait.AddressLookupBuilder.html>
- <https://docs.rs/iroh/1.5.0/iroh/struct.EndpointAddr.html>
- <https://docs.rs/iroh/1.5.0/iroh/type.EndpointId.html>
- <https://docs.rs/iroh/1.5.0/iroh/endpoint/presets/index.html>
- <https://docs.rs/iroh-tickets/1.0.0/iroh_tickets/trait.Ticket.html>
- <https://docs.rs/n0-watcher/1.0.0/n0_watcher/trait.Watcher.html>
- <https://docs.rs/iroh-tickets/1.0.0/iroh_tickets/endpoint/struct.EndpointTicket.html>
- <https://docs.rs/noq/1.5.0/noq/struct.OpenBi.html>
- <https://docs.rs/noq/1.5.0/noq/struct.AcceptBi.html>
- <https://crates.io/api/v1/crates/iroh/1.5.0>

## 7. Machine-readable companion

`/Users/dylanwong/projects/aws0-research-2026-11-19/contracts/api-sheet.json` contains the full surviving contract objects, all 14 refutation verdicts, every unverified item verbatim, the pinned set, the compatibility/index records, all blockers with evidence, the witness metadata, and the source URL index. `api-sheet-summary.json` holds the compact summary subset.
