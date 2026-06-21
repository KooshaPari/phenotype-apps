# ADR-080: Encryption-at-Rest Mandate (L52 Fleet-Wide Policy)

| Field | Value |
|---|---|
| **Status** | PROPOSED (v19 T2, 2026-06-21) |
| **Date** | 2026-06-21 |
| **Pillar** | L52 (Encryption at Rest) |
| **Cycle** | 9 (v19) |
| **Author** | forge (L5-154) |
| **Sponsors** | L5-150 (fleet governance), L5-151 (v19 plan owner), L5-153 (security cadence) |
| **Supersedes** | None — fleet-wide net-new mandate (companion to ADR-079 Vault migration) |
| **Depends on** | **ADR-079 (Vault migration roadmap)** — Phase 1 must complete before Phase 2 of this mandate starts |
| **Related** | ADR-046 (Federation mTLS + OIDC), ADR-079 (Vault migration), ADR-081 (Pen-test roadmap), ADR-042 (Security cadence), ADR-035 (Configra migration gates), ADR-027 (LFS policy), ADR-012 (pheno-tracing canonical), ADR-013 (pheno-mcp-router canonical) |
| **Plan ref** | `plans/2026-06-21-v19-71-pillar-cycle-9-p0.md` § T2 |
| **Authoritative sources** | NIST SP 800-57 Part 1 Rev. 5, NIST SP 800-38D, CIS Benchmark v8 (AWS / Linux / Kubernetes / SQLite) |
| **Score target** | L52 1.5 → 2.0 (v18 cycle 8 probe finding 2) |

---

## 1. Context

The 71-pillar cycle-8 probe (`findings/2026-06-21-v18-cycle-8-probe.md` § 6) scored **L52 (Encryption-at-Rest) at 1.5 / 3** — the second-lowest score in the security domain after L50 (which ADR-079 closes). Driving findings:

1. **Zero fleet-wide policy.** No ADR, runbook, or CI gate exists that mandates encryption-at-rest for any of the 4 fleet-critical substrates (`pheno-otel`, `pheno-events`, `pheno-predict`, `phenotype-registry`). Per-substrate decisions are ad-hoc — wherever a developer remembered to flip a flag.
2. **4 distinct on-disk data classes** sit unprotected in default configurations:
   - `pheno-otel` OTLP span archive (NDJSON files, default `~/.local/share/pheno-otel/`)
   - `pheno-events` append-only journal (`segments/*.log`, `wal/*.wal`)
   - `pheno-predict` state caches (LMDB + Pickle files under `~/.cache/pheno-predict/`)
   - `phenotype-registry` SQLite DB (`registry.sqlite`, default `~/.local/share/phenotype-registry/`)
3. **No key-management story.** The substrate crates ship with hardcoded fallback key material (`b"pheno-default-key-do-not-use-in-prod"` in `pheno-events` v0.4.x; `os.urandom(32)` re-derived per process in `pheno-predict` v0.2.x). Vault is not yet wired (ADR-079 Phase 1 in flight).
4. **Compliance gap.** FedRAMP Moderate (SC-28, SC-28(1)) and SOC2 CC6.1 require encryption-at-rest for any data classified as "sensitive" (per ADR-018 data-classification framework). The 4 substrates above all hold telemetry, journal, and registry data that crosses into the sensitive tier when the operator's workflow ingests third-party API keys, OAuth tokens, or customer identifiers.
5. **Container/VM baseline** is silent on swap/tmpfs encryption. Devcontainers (`devcontainer.json`) provision 4 GiB tmpfs at `/tmp` by default; Docker host swap is typically unencrypted.

This ADR is the **policy + phased execution plan** that turns L52 from 1.5/3 into 2.0/3 by end of v19 cycle 9, with a documented glide path to 3.0/3 in v20+ once Vault Phase 1 (ADR-079) delivers dynamic key material.

### 1.1 Scope boundaries

**In scope** (this ADR mandates):
- Encryption-at-rest for 4 fleet-critical substrates (data at rest on disk).
- Default key management (transitional: passphrase-derived keys via Argon2id) until ADR-079 Phase 1 ships Vault transit.
- Container + devcontainer + host baseline (swap, tmpfs, persistent volumes).
- Per-substrate threat model + recommended algorithm (AES-256-GCM, ChaCha20-Poly1305, or AES-256-XTS per data class).

**Out of scope** (covered elsewhere or deferred):
- In-memory secret zeroization → addressed by `.cargo/audit-rules.toml` + `pheno-config/src/secrets.rs` `ZeroizeOnDrop` (track T2 deliverable 2/3 per plan, separate PR).
- Transport encryption (TLS / mTLS) → covered by ADR-046 (federation mTLS).
- Key rotation cadence → defined in ADR-079 § 4 (transit engine rotation every 90d, wrap-TTL 300s).
- Application-layer field-level encryption (e.g., per-customer PII) → deferred to v20+ data-classification work.

### 1.2 Threat model summary

| Threat | Vector | Affected substrate | Mitigation in this ADR |
|---|---|---|---|
| Disk theft / lost laptop | Physical access, stolen backup | All 4 | LUKS / FileVault / EBS encryption + per-file app-layer crypto |
| Compromised CI runner | Disk snapshot exfiltration | `pheno-otel` archive, `phenotype-registry` DB | Ephemeral keys + Vault transit (Phase 2 of this ADR) |
| Insider with filesystem read | Read-only mount, backup restore | `pheno-events` journal, `pheno-predict` cache | AES-256-GCM with key never written to disk |
| Cloud snapshot leak | AWS EBS snapshot shared publicly | All | EBS default encryption enforced at AWS account level |
| Container escape | Pod → host filesystem | All | tmpfs `/tmp` mounted with `noswap,noexec`, encrypted tmpfs |

---

## 2. Decision

The Phenotype fleet mandates **encryption-at-rest** for all persistent substrate data, enforced at three layers:

### Layer 1 — Host / volume layer

| Surface | Mandate | Standard |
|---|---|---|
| Linux host rootfs | LUKS2 with Argon2id PBKDF, AES-256-XTS | NIST SP 800-38D § 5.2, CIS Linux v8 § 1.4.1 |
| macOS development hosts | FileVault 2 (XTS-AES-128) | CIS macOS v8 § 1.2 |
| AWS EBS volumes | `Encrypted: true`, default KMS key `alias/aws/ebs` unless explicitly rotated | CIS AWS v8 § 2.1.1 |
| AWS S3 buckets | `BucketEncryption` with `aws:kms` SSE (SSE-KMS), default-deny unencrypted PutObject | CIS AWS v8 § 2.1.5 |
| Kubernetes PersistentVolumes | StorageClass with `parameters.csi.storage.k8s.io/node-publish-secret-name` referencing Vault-injected secret | CIS Kubernetes v8 § 5.5.1 |
| Devcontainer `/tmp` tmpfs | `mount -t tmpfs -o size=4g,noexec,nosuid,nodev tmpfs /tmp` + in-memory encryption for secrets (no on-disk swap) | NIST SP 800-53 SC-28(1) |

### Layer 2 — Application / file layer

Each of the 4 fleet-critical substrates MUST apply **authenticated encryption** (AEAD) to its on-disk data using one of:

- **AES-256-GCM** (default — hardware acceleration on x86 AES-NI, ARMv8 Cryptography Extensions)
- **ChaCha20-Poly1305** (fallback for software-only or ARMv7 targets)
- **AES-256-XTS** (full-disk / volume layer only; never for file-level — XTS does not authenticate)

**Key derivation:** Until Vault transit (ADR-079 Phase 1) ships, use **Argon2id** (memory = 64 MiB, iterations = 3, parallelism = 4) over a high-entropy passphrase supplied via `PHENO_REST_PASSPHRASE` env var (CI: GitHub Actions `secrets.PHENO_REST_PASSPHRASE`; rejected if unset in production profile). Once Vault Phase 1 is GA, **all substrate key derivation MUST route through Vault Transit (`transit/encrypt/pheno-<substrate>`)** — no exceptions.

**Per-substrate mandates (Section 4 detail):**

| Substrate | Data file(s) | Cipher | Key store (transitional) | Key store (post-ADR-079 Phase 1) |
|---|---|---|---|---|
| `pheno-otel` | `~/.local/share/pheno-otel/ndjson/*.ndjson` | AES-256-GCM | Argon2id(PHENO_REST_PASSPHRASE) | Vault transit `transit/encrypt/pheno-otel` |
| `pheno-events` | `segments/*.log`, `wal/*.wal` | AES-256-GCM (per-segment nonce) | Argon2id(PHENO_REST_PASSPHRASE) | Vault transit `transit/encrypt/pheno-events` |
| `pheno-predict` | LMDB env + Pickle files | ChaCha20-Poly1305 | Argon2id(PHENO_REST_PASSPHRASE) | Vault transit `transit/encrypt/pheno-predict` |
| `phenotype-registry` | `registry.sqlite` (page-level) | AES-256-GCM via SQLCipher | Argon2id(PHENO_REST_PASSPHRASE) | Vault transit `transit/encrypt/phenotype-registry` |

### Layer 3 — Detection / CI gate

- **`cargo-deny`** banlist (`.cargo/audit-rules.toml`) — bans `unsafe_code` in 4 substrate secret-handling modules and bans plain `String::from_utf8(secret_bytes)` patterns (track T2 deliverable per plan § T2.2).
- **CI workflow `ci/encryption-at-rest.yml`** — runs on every PR touching the 4 substrate paths; verifies:
  - Default key material is **absent** in release builds (`rg "pheno-default-key-do-not-use-in-prod" target/release/` returns 0).
  - Substrate binary refuses to start without `PHENO_REST_PASSPHRASE` or `VAULT_ADDR` set (smoke test).
  - Per-substrate fuzz target (`proptest`/`arbitrary`) generates 10k random payloads; each round-trips through the cipher and produces a valid ciphertext + auth tag.
- **Audit cadence** — `findings/<date>-l52-encryption-at-rest-audit.md` written monthly per ADR-042 (security audit cadence). Score updated in 71-pillar refresh per ADR-041 (weekly Monday 09:00 PDT).

---

## 3. Authoritative Sources (Standards Compliance)

### 3.1 NIST SP 800-57 Part 1 Rev. 5 — Recommendation for Key Management

Cited: SP 800-57 Part 1 Rev. 5 (2020), Section 5.3.1 (cryptographic algorithm strength), Section 5.6 (key lifetimes).

- **§ 5.3.1** — AES-256 with 256-bit keys provides 256 bits of effective security. Acceptable through 2030+ for TOP SECRET.
- **§ 5.6** — Cryptographic key lifetimes: data-encryption keys SHOULD be replaced when the volume of encrypted data exceeds 2^32 blocks (~64 GiB for 128-bit blocks). For our 4 substrates, the LMDB cache (`pheno-predict`) is the only one approaching this; key rotation on cache-size threshold is mandated (Section 4.3).
- **§ 6.2.1.3** — Key derivation functions: PBKDF2 (deprecated) → Argon2id (preferred since 2019). Argon2id is mandated in this ADR.

### 3.2 NIST SP 800-38D — Recommendation for Block Cipher Modes (GCM)

Cited: SP 800-38D (2007), Section 5.2.1.1 (GCM construction), Section 8 (IV construction).

- **§ 5.2.1.1** — GCM is the only AES mode mandated for new applications; CBC, CTR, XTS for storage only.
- **§ 8.2** — IV construction: 96-bit random IV per encryption operation. **CRITICAL**: GCM nonce reuse with the same key catastrophically breaks confidentiality AND authenticity. Per-substrate implementations MUST generate IVs via `OsRng` (Rust `rand::rngs::OsRng`, Python `secrets.token_bytes(12)`), and MUST include a monotonic counter suffix as a defense-in-depth (Section 4 detail).

### 3.3 CIS Benchmark v8 — Controls Mapped

| Substrate / Surface | CIS Control | Section | Implementation in this ADR |
|---|---|---|---|
| AWS EBS | CIS AWS v8 § 2.1.1 | "Ensure EBS volumes are encrypted" | Layer 1 § 2 (mandate EBS default encryption at AWS account level) |
| AWS S3 | CIS AWS v8 § 2.1.5 | "Ensure S3 buckets enforce encryption" | Layer 1 § 2 (mandate SSE-KMS) |
| Linux host disk | CIS Linux v8 § 1.4.1 | "Ensure filesystem integrity is regularly checked" + LUKS reference | Layer 1 § 2 |
| Kubernetes PV | CIS Kubernetes v8 § 5.5.1 | "Ensure default service account is not actively used" + PV encryption references | Layer 1 § 2 |
| macOS host | CIS macOS v8 § 1.2 | "Ensure FileVault is enabled" | Layer 1 § 2 |
| Application secrets | CIS v8 IG3 § 3.11 | "Encrypt sensitive data at rest" | Layer 2 (entire section) |
| Backup media | CIS v8 IG3 § 11.2 | "Ensure automated backups are encrypted" | Implied via EBS / S3 mandates; backup runbook in ADR-079 § 5 |

---

## 4. Per-Substrate Specifications

### 4.1 `pheno-otel` — OTLP span archive

**Data:** NDJSON files under `~/.local/share/pheno-otel/ndjson/{date}.ndjson`, one span per line, default chunk size 16 MiB. May contain PII (user IDs in `attributes`), API keys in `http.headers` (redacted by SDK but raw on disk), OAuth tokens in `attributes.oauth.token`.

**Mandate:**
- AEAD: AES-256-GCM, 96-bit random nonce per 16 MiB chunk, 128-bit auth tag.
- File format: `.ndjson.enc` (binary envelope: `magic(4) = "POTE" || version(1) = 0x01 || nonce(12) || ciphertext+tag`).
- Reader: streaming decrypt via `pheno-otel::archive::decrypt_chunked()` — never load full file into memory.
- Key handling: derived from `PHENO_REST_PASSPHRASE` via Argon2id; key never logged, zeroized on drop.

**Threat coverage:** Disk theft ✓, CI snapshot exfiltration ✓ (transitional: passphrase is per-environment; post-Vault: transit key never leaves Vault).

### 4.2 `pheno-events` — append-only event journal

**Data:** `segments/NNNNNN.log` (sealed, immutable) and `wal/active.wal` (write-ahead log). Segments hold JSON-encoded events with optional PII. WAL holds pending writes not yet flushed.

**Mandate:**
- AEAD: AES-256-GCM, **per-record nonce** (not per-segment — re-keying on segment rotation is too coarse).
- File format extension: `.log.enc` (envelope same as § 4.1, `magic = "PEVT"`, `version = 0x01`).
- Active WAL: encrypted in-memory page-by-page; flushed pages are AEAD-encrypted before fsync.
- Sealed segment rotation: when current segment reaches 64 MiB, the in-flight cipher key is rotated per NIST SP 800-57 § 5.6 (2^32 block limit). New segment uses derived key `KDF(master_key, segment_id)`.

**Backward compatibility:** v0.4.x and earlier used plaintext `b"pheno-default-key-do-not-use-in-prod"` for testing. v0.5.0 (Q3 2026) removes the default key entirely; the substrate binary will fail to start without a real key source. A one-way migration tool `pheno-events-migrate encrypt <path>` ships in v0.5.0 to convert legacy plaintext journals.

### 4.3 `pheno-predict` — prediction state caches

**Data:** LMDB env (mmap'd) at `~/.cache/pheno-predict/lmdb/`, Python Pickle files at `~/.cache/pheno-predict/predictions/`. Holds ML model state, feature vectors, and prediction outcomes — may include user IDs and aggregated behavior vectors.

**Mandate:**
- AEAD: ChaCha20-Poly1305 (chosen for software-only fallback on Apple Silicon before AES-NI in userspace; both are 256-bit key, 96-bit nonce, 128-bit tag — equivalent security).
- LMDB: page-level encryption via LMDB's `MDB_WRITEMAP` + custom encryption shim. Each 4 KiB page encrypted with derived per-page nonce `KDF(master_key, page_id)`.
- Pickle files: per-file envelope `magic = "PPRE"`, `version = 0x01`, same as § 4.1 structure.
- Key rotation: triggered on `cache_size_bytes > 64 GiB` OR `cache_age_days > 90`, whichever first. Background rotation: open new LMDB env with new key, copy-decrypt-rewrite, atomic rename.

**Subtle point:** LMDB mmap'd pages are visible to the OS in plaintext after decryption. The cache MUST be re-encrypted on `mlock()` failure. Pheno-predict MUST call `mlockall(MCL_CURRENT | MCL_FUTURE)` on Linux and `mlock()` on macOS to prevent plaintext pages from being swapped to unencrypted swap (paired with Layer 1 swap encryption).

### 4.4 `phenotype-registry` — SQLite database

**Data:** Single SQLite file `registry.sqlite` containing the package registry (47+ rows of crates/packages across the fleet), plus audit log, plus PRCP (Polyglot Reuse via Canonical Ports) cross-references. May include fork source URLs and committer emails.

**Mandate:**
- AEAD: SQLCipher 4.x with AES-256-GCM, 16 KiB page size (SQLCipher default).
- Key handling: SQLCipher's PBKDF2-HMAC-SHA512 with 256k iterations, salt from `OsRng`.
- Database creation: `phenotype-registry init --passphrase-env PHENO_REST_PASSPHRASE`; the passphrase is required at `open()` time, not embedded.
- Backup: `sqlite3 registry.sqlite ".backup '/backups/registry-{date}.sql.enc'"` — the SQLCipher-encrypted backup is then AEAD-re-encrypted at the file layer with a separate Vault transit key (defense in depth).

**Migration path:** Legacy unencrypted DBs (pre-this-ADR) must be migrated via `phenotype-registry migrate-encrypt` — opens plaintext, writes through SQLCipher to new file, atomic rename, leaves plaintext `.bak` for 7d then auto-purges.

---

## 5. 3-Phase Rollout

The rollout is **explicitly keyed to ADR-079 (Vault migration) Phase 1 completion**. We do not begin substrate re-keying onto Vault transit until Vault is GA (per ADR-079 § 3, Phase 1 exit gate).

### Phase 1 — Discovery, classification, and transitional keys (weeks 1-2, 2026-06-22 → 2026-07-05)

**Goal:** All 4 substrates enforce AEAD at rest using Argon2id-derived keys. Vault transit is not yet wired.

**Scope:**
- Implement `pheno_rest::Cipher` trait (one file, ~150 LOC, pure-Rust) — supports AES-256-GCM and ChaCha20-Poly1305, Argon2id KDF, OsRng nonces, zeroize-on-drop.
- Port to all 4 substrates: `pheno-otel`, `pheno-events`, `pheno-predict`, `phenotype-registry` (separate PRs, one per substrate).
- CI gate: `.github/workflows/ci/encryption-at-rest.yml` — fuzz + smoke + key-absence assertions.
- `.cargo/audit-rules.toml` — add `bans = ["unsafe_code"]` for the 4 substrate paths.
- Devcontainer `devcontainer.json` update — mount `/tmp` as encrypted tmpfs; document `PHENO_REST_PASSPHRASE` requirement.
- Reference docker-compose update — add `tmpfs: [{ target: /tmp, mode: 1777, size: 4g }]` + `cap_add: [IPC_LOCK]` for mlock.

**Exit gate:**
- All 4 substrates ship a release that refuses to start without `PHENO_REST_PASSPHRASE` or `VAULT_ADDR` set (assertion test green in CI).
- 1,000 proptest cases per substrate round-trip AEAD without panic (5k cases total).
- 0 instances of hardcoded fallback keys (`pheno-default-key-do-not-use-in-prod`, `b"\x00" * 32`) in release binaries (CI scan green).
- Documentation: `docs/architecture/encryption-at-rest.md` (per-substrate threat model + algorithm choice, ~250 LOC) lands on `main`.

**Rollback trigger:** Any substrate's AEAD round-trip fails in CI, OR any production telemetry reports > 1% decrypt failures within 24h of rollout.

**Lock-in:** After Phase 1 exit, plaintext fallbacks are **deleted** (not merely deprecated) from the substrate source — no going back.

### Phase 2 — Vault transit integration (weeks 3-6, 2026-07-06 → 2026-07-31)

**Pre-condition:** **ADR-079 Phase 1 must have completed and the Vault dev cluster must be GA** (per ADR-079 § 3 Phase 1 exit gate: 14 consecutive days green `vault status`).

**Goal:** Substrate key derivation routes through Vault Transit instead of local Argon2id. No plaintext key material on disk.

**Scope:**
- Enable Vault Transit engine: `vault secrets enable transit` per ADR-079 § 4.
- Create 4 transit keys: `pheno-otel`, `pheno-events`, `pheno-predict`, `phenotype-registry` — all AES-256-GCM, automatic derivation enabled.
- Update `pheno_rest::Cipher` to accept a Vault client; if `VAULT_ADDR` + `VAULT_TOKEN` are set, route all KDF + AEAD key-handling through `transit/encrypt/<substrate>` and `transit/decrypt/<substrate>`. If unset, fall back to Argon2id (dev only — production startup script enforces `VAULT_ADDR` set).
- Per-substrate PRs switch the default to Vault transit.
- Migration runbook: `docs/operations/vault-transit-migration.md` — 12-step procedure to rotate keys across 4 substrates without downtime, including rollback (re-pin to previous transit key version).
- Wire `pheno-tracing` (ADR-012) OTLP export for all key-handling operations: emit `vault.transit.encrypt` / `vault.transit.decrypt` spans, set `vault.key_version` attribute, sample at 1% (high-volume).

**Exit gate:**
- All 4 substrates default to Vault transit (Argon2id fallback disabled by default in release builds).
- 1 human + 1 CI runner can complete `vault write transit/encrypt/pheno-otel plaintext=...` end-to-end in < 800 ms p95 (matches ADR-079 § 3 Phase 2 exit gate).
- 0 instances of `PHENO_REST_PASSPHRASE` referenced in production deployment manifests (`kustomize`, `helm`, `terraform`).
- 30 days of green CI on the encryption-at-rest workflow with zero decrypt-failure alerts.

**Rollback trigger:** Vault transit p95 latency > 2s sustained, OR any Vault outage > 30 min in any single substrate. Rollback: pin substrate to previous transit key version, then to Argon2id if needed (the Phase 1 fallback stays compiled but disabled by default).

### Phase 3 — Detection, audit, and certification (weeks 7-8, 2026-08-01 → 2026-08-14)

**Pre-condition:** **Phase 2 must be stable for 14 consecutive days.**

**Goal:** L52 score advances from 2.0 (Adequate) toward 2.5 (Solid); fleet is ready for external attestation.

**Scope:**
- Monthly audit cadence (per ADR-042) — `findings/<date>-l52-encryption-at-rest-audit.md` written, scored 0-3 per pillar per substrate.
- Detection: gitleaks pre-receive hook extended to flag hardcoded AES keys (`grep -E "[A-Fa-f0-9]{64}"`) and suspicious 32-byte constant arrays.
- Compliance evidence: `docs/compliance/fedramp-sc28.md` — FedRAMP SC-28 + SC-28(1) control implementation summary, with per-substrate evidence links.
- Compliance evidence: `docs/compliance/soc2-cc6.1.md` — SOC2 CC6.1 logical access evidence, 8 artifacts cross-referenced.
- Tabletop exercise: simulate stolen-disk scenario on 1 staging substrate, time the recovery / decrypt-with-no-key procedure.
- L52 re-scored in the 71-pillar refresh (cycle 9 closure, 2026-06-28 → 2026-08-04 per ADR-041 cadence).

**Exit gate:**
- 71-pillar refresh (2026-08-04 cycle) shows L52 = 2.0+ across all 4 substrates.
- Compliance docs reviewed and merged (FedRAMP SC-28, SOC2 CC6.1).
- Tabletop recovery time-to-decide documented: ≤ 4 hours (target), 4-24 hours (acceptable), > 24 hours (FAIL).

**Future (v20+):** External attestation — SOC2 Type II audit (Q4 2026), FedRAMP Moderate authorization (Q1 2027), CIS Benchmark v8 attestation (annual, Q1 2027). These are out of scope for v19 but pre-documented here so Phase 3 evidence is reusable.

---

## 6. Consequences

### 6.1 Positive

- **L52 climbs 1.5 → 2.0** in the 71-pillar refresh (cycle 9, 2026-08-04). Direct closure of cycle-8 probe finding 2.
- **FedRAMP SC-28 / SC-28(1) compliance** is achievable without re-architecture. Evidence docs land in `docs/compliance/`.
- **SOC2 CC6.1** evidence is reusable for both encryption-at-rest and the wider secrets-management story (cross-references ADR-079).
- **No plaintext key material on disk** by end of Phase 2. Disk-theft and CI-snapshot-leak threat vectors are closed.
- **CI gate prevents regressions** — the encryption-at-rest workflow blocks PRs that reintroduce plaintext fallbacks.
- **Defense in depth** — AES-256-GCM authenticated encryption (Layer 2) + EBS / LUKS / SQLCipher (Layer 1) means a single layer failure does not expose plaintext data.

### 6.2 Negative / Costs

- **Performance overhead** — AES-256-GCM with AES-NI is < 5% throughput cost on modern x86/ARM. ChaCha20-Poly1305 is faster on software-only targets. Estimated hit: 2-8% on `pheno-otel` archive write throughput, < 1% on `pheno-events` segment rotation, negligible on `phenotype-registry` SQLite (SQLCipher is < 3% in published benchmarks). Net fleet-wide: < 3% throughput regression; acceptable per ADR-040 lib coverage gate and ADR-019 performance optimization patterns.
- **Operational complexity** — operators must set `PHENO_REST_PASSPHRASE` (transitional) or `VAULT_ADDR` (post-Phase 2) on every deployment. Migration scripts (`pheno-events-migrate`, `phenotype-registry migrate-encrypt`) are required for legacy data.
- **Vault dependency** — Phase 2 makes Vault a hard dependency for the 4 substrates. Vault outage = 4 substrates degraded (reads still work with cached keys, writes fail closed). Mitigation: ADR-079 Phase 2 exit gate ensures Vault HA + 30-day uptime before substrate cutover.
- **Key rotation logistics** — per NIST SP 800-57 § 5.6, key rotation is required when encrypted volume exceeds 2^32 blocks. Only `pheno-predict` LMDB is at risk; rotation logic adds ~300 LOC to the substrate.

### 6.3 Neutral / Risks

- **Algorithm agility** — this ADR mandates AES-256-GCM and ChaCha20-Poly1305. A future migration to post-quantum (e.g., ML-KEM hybrid) is **out of scope** for v19; tracked under ADR-NNN-post-quantum-readiness (placeholder, to be authored in v20+).
- **Argon2id parameters** — `memory = 64 MiB, iterations = 3, parallelism = 4` is OWASP-recommended for 2024. May need to increase `iterations` if hardware improves; tracked via annual security review (ADR-042 cadence).

---

## 7. Compliance Mapping

| Framework | Control | Substrate | Evidence |
|---|---|---|---|
| FedRAMP Moderate | SC-28 (Protection of Information at Rest) | All 4 | `docs/compliance/fedramp-sc28.md` (Phase 3 deliverable) |
| FedRAMP Moderate | SC-28(1) (Cryptographic Protection) | All 4 | Same — AES-256-GCM meets FIPS 140-3 (when FIPS module used) |
| SOC2 (TSC 2017) | CC6.1 (Logical Access — Encryption) | All 4 | `docs/compliance/soc2-cc6.1.md` (Phase 3 deliverable) |
| SOC2 (TSC 2017) | CC6.7 (Restriction of data during transmission) | n/a | Covered by ADR-046 (mTLS) |
| NIST 800-53 r5 | SC-12, SC-13 | All 4 | NIST SP 800-57 + 800-38D citations in § 3 |
| CIS Benchmark v8 | AWS § 2.1.1, § 2.1.5 | Host EBS + S3 | Layer 1 § 2 |
| CIS Benchmark v8 | Linux § 1.4.1 | Host disk | Layer 1 § 2 |
| CIS Benchmark v8 | Kubernetes § 5.5.1 | K8s PV | Layer 1 § 2 |
| OpenSSF Best Practices badge | "Cryptographic Practices" | All 4 | Layer 2 + 3 |

---

## 8. Rollback Strategy

The mandate is designed for **forward-only** migration: Phase 1 deletes plaintext fallback paths, Phase 2 deletes Argon2id as default, Phase 3 locks in the audit cadence. Rollback is therefore not a "return to plaintext" but a "revert to prior transit key version".

| Phase | Rollback action | Time-to-recover | Data loss risk |
|---|---|---|---|
| Phase 1 mid-rollout | `git revert` substrate AEAD PR; clear `.ndjson.enc` archive; restore plaintext readers | < 30 min | None (no plaintext writes post-Phase-1-merge) |
| Phase 2 mid-rollout | Pin substrate to previous Vault transit key version (`vault write transit/keys/<key>/config`); if Vault fully down, set `PHENO_REST_PASSPHRASE` to enable Argon2id fallback (compiled but disabled by default — re-enable via `PHENO_REST_ALLOW_ARGON2ID=1`) | < 1 hour | None (encrypted data is portable across key versions) |
| Phase 3 mid-rollout | Reverse a compliance doc merge | < 15 min | None |

**Critical rule:** Once Phase 1 ships, **plaintext data MUST NOT be written back to disk by the substrate**. The Cipher trait gates all writes; bypassing the gate is a CI-blocked violation per `.cargo/audit-rules.toml` `bans.unsafe-code` rule on the substrate paths.

---

## 9. Open Questions

1. **FIPS 140-3 validation** — does the fleet require FIPS-mode cryptography (AWS GovCloud, FedRAMP High)? If yes, we need to migrate from `aes-gcm` crate to FIPS-validated `aws-lc-rs` or `openssl` provider. Tracking: defer to v20+ unless ADR-046 explicitly says FedRAMP High.
2. **Post-quantum readiness** — when NIST PQC standards are finalized (FIPS 203/204/205), what is the migration plan? Defer to v20+.
3. **Cross-substrate key reuse** — should the 4 substrates share a single Vault transit key, or one per substrate? This ADR mandates **one per substrate** for blast-radius isolation; revisit if Vault transit quotas become a bottleneck.

---

## 10. References

### 10.1 Authoritative standards

- **NIST SP 800-57 Part 1 Rev. 5** — Recommendation for Key Management: Part 1 — General. NIST, 2020. <https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final>
- **NIST SP 800-38D** — Recommendation for Block Cipher Modes of Operation: Galois/Counter Mode (GCM) and GMAC. NIST, 2007. <https://csrc.nist.gov/publications/detail/sp/800-38d/final>
- **NIST SP 800-53 Rev. 5** — Security and Privacy Controls for Information Systems and Organizations. SC-28 + SC-28(1). <https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final>
- **CIS Benchmark v8.0** — Center for Internet Security, multiple platform benchmarks. <https://www.cisecurity.org/cis-benchmarks/>
- **OWASP Password Storage Cheat Sheet (2024)** — Argon2id parameter recommendations.

### 10.2 Phenotype-internal references

- **ADR-046** (Federation mTLS + OIDC) — companion ADR for transport-layer encryption.
- **ADR-079** (Vault migration roadmap, v19 T1) — **hard dependency** for Phase 2 of this mandate.
- **ADR-042** (Security audit cadence) — monthly cadence for L52 audit docs.
- **ADR-041** (71-pillar refresh cadence) — weekly Monday 09:00 PDT refresh; L52 score update.
- **ADR-035** (Configra migration gates) — pattern reference for cargo deny + migration PR sequencing.
- **ADR-027** (LFS policy) — Tier 2/3 binaries that may cross-reference encrypted regions.
- **ADR-012 / ADR-036B** (pheno-tracing canonical) — OTLP instrumentation of vault transit ops (Phase 2).
- **ADR-013** (pheno-mcp-router canonical) — model for how substrate mandates are documented.
- **ADR-018** (PRCP pattern) — data-classification framework referenced in § 1.
- **ADR-040** (test coverage gates per tier) — `proptest` + fuzz gates for the 4 substrates.
- **ADR-023** (Agent-effort governance, Rule 3.1) — substrate quality bar applicable here.
- **ADR-015 v2.1** (worklog schema) — `device:` field required in worklog rows.
- **`plans/2026-06-21-v19-71-pillar-cycle-9-p0.md`** § T2 — source plan for this ADR.
- **`findings/2026-06-21-v18-cycle-8-probe.md`** § 6 finding 2 — gap analysis that motivated this ADR.
- **`findings/2026-06-21-v17-cycle-7-probe.md`** § 5 — prior cycle's L52 score baseline.

### 10.3 Substrate references

- `pheno-otel/src/archive.rs` — NDJSON writer, gets `encrypt_chunked()`.
- `pheno-events/src/journal/` — segment + WAL, gets `encrypt_record()`.
- `pheno-predict/src/cache/` — LMDB env + Pickle, gets `encrypt_page()` + `encrypt_file()`.
- `phenotype-registry/src/db/` — SQLite open path, switches to SQLCipher.

### 10.4 Tooling references

- `aes-gcm` crate (Rust) — AES-256-GCM implementation.
- `chacha20poly1305` crate (Rust) — ChaCha20-Poly1305 implementation.
- `argon2` crate (Rust) — Argon2id KDF.
- `zeroize` crate (Rust) — `ZeroizeOnDrop` derive (already in use per ADR-035).
- `sqlcipher` (C library, Rust bindings `rusqlite` with `sqlcipher` feature).
- `vault` CLI — Transit engine (per ADR-079).
- `cargo-deny` — `bans = ["unsafe_code"]` in `.cargo/audit-rules.toml`.

---

**Authored 2026-06-21 by forge (L5-154) as part of v19 T2 cycle 9 worklog batch. Companion to ADR-079 (Vault migration). Supersedes no prior ADR. Status: PROPOSED → moves to ACCEPTED on PR merge.**
