# 2026-06-22 Cargo Audit — Top-Level pheno-* Fleet

**Author:** orch-v22-SD4-cargo-audit
**Date:** 2026-06-22
**Refs:** v22 cycle 12 (P1 reduction round 3), 71-pillar L46 (vulnerability mgmt), AGENTS.md ADR-042 (security audit cadence)
**Tool:** `cargo-audit-audit 0.22.1` against `https://github.com/RustSec/advisory-db` (1134 advisories loaded)

## Executive Summary

Fleet cargo audit of **10 top-level `pheno-*` repos** (standalone packages with their own `[workspace]` table + `Cargo.lock`).

| Outcome | Count | Repos |
|---|---:|---|
| **Clean** (0 vulns, 0 warnings) | 8 | `pheno-chaos`, `pheno-cli-base`, `pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`, `pheno-otel`, `pheno-tracing` |
| **Vulnerabilities present** | 1 | `pheno-events` (3 vulnerabilities) |
| **Unmaintained warnings only** | 1 | `pheno-port-adapter` (4 warnings, 0 vulnerabilities) |
| **Not scannable** | 0 | — |

**Total: 3 vulnerabilities (1 repo), 9 unmaintained warnings (2 repos).**

Compared to the 2026-06-20 v12 baseline (`findings/2026-06-20-v12-cargo-audit-baseline.md`), this is a net **regression of +3 vulnerabilities** at the top-level fleet. Cause: `pheno-events` (newly-added at top level, v20 cycle 10) ships with `pact_consumer = "0.10"` in dev-deps which transitively pulls in the vulnerable `rustls-webpki 0.101.7` chain.

## Per-Repo Results

| Repo | Cargo.toml | Cargo.lock | Crates scanned | Vulnerabilities | Warnings |
|------|:----------:|:----------:|---------------:|----------------:|---------:|
| `pheno-chaos`       | yes | yes |   7 | 0 | 0 |
| `pheno-cli-base`¹   | yes | gitignored |  42 | 0 | 0 |
| `pheno-config`      | yes | yes | 143 | 0 | 0 |
| `pheno-context`     | yes | yes |  52 | 0 | 0 |
| `pheno-errors`      | yes | yes | 118 | 0 | 0 |
| `pheno-events`      | yes | yes | 453 | **3** | 5 |
| `pheno-flags`       | yes | yes |  73 | 0 | 0 |
| `pheno-otel`        | yes | yes |  83 | 0 | 0 |
| `pheno-port-adapter`| yes | yes | 434 | 0 | 4 |
| `pheno-tracing`     | yes | yes |  92 | 0 | 0 |

¹ `pheno-cli-base` is a library (`[package] publish = false`); `Cargo.lock` is gitignored at the repo root (line 49 of `.gitignore`). The audit ran against a transient `Cargo.lock` produced by `cargo generate-lockfile`; the file is left on disk but not committed.

## pheno-events — 3 vulnerabilities, 5 unmaintained warnings

**Severity:** HIGH (3 RUSTSEC advisories tagged `vulnerability` — not just `unmaintained` warnings)
**Path:** dev-dep only (`[dev-dependencies] pact_consumer = "0.10"`)
**Production impact:** **None** — direct prod dep `reqwest = { version = "0.13", features = ["rustls"] }` resolves to `rustls 0.23.40` + `rustls-webpki ≥0.103.x` which is clean. The vulnerable chain only enters the dependency graph through contract-test infrastructure.

### RUSTSEC-2026-0104 — rustls-webpki 0.101.7 — Reachable panic in CRL parsing

- **Date:** 2026-04-22
- **Crate:** `rustls-webpki 0.101.7`
- **Title:** Reachable panic in certificate revocation list parsing
- **Solution:** Upgrade to `>=0.103.13, <0.104.0-alpha.1` OR `>=0.104.0-alpha.7`
- **URL:** <https://rustsec.org/advisories/RUSTSEC-2026-0104>
- **Dependency path:**
  ```
  pheno-events 0.1.0
  └─ pact_consumer 0.10.8
     └─ pact_mock_server 1.2.6
        └─ pact_matching 1.2.1
           └─ reqwest 0.11.27
              └─ tokio-rustls 0.24.1
                 └─ rustls 0.21.12
                    └─ rustls-webpki 0.101.7   ← vulnerable
  ```

### RUSTSEC-2026-0098 — rustls-webpki 0.101.7 — Name constraints for URI names incorrectly accepted

- **Date:** 2026-04-14
- **Solution:** Upgrade to `>=0.103.12, <0.104.0-alpha.1` OR `>=0.104.0-alpha.6`
- **URL:** <https://rustsec.org/advisories/RUSTSEC-2026-0098>

### RUSTSEC-2026-0099 — rustls-webpki 0.101.7 — Name constraints accepted for wildcard certs

- **Date:** 2026-04-14
- **Solution:** Upgrade to `>=0.103.12, <0.104.0-alpha.1` OR `>=0.104.0-alpha.6`
- **URL:** <https://rustsec.org/advisories/RUSTSEC-2026-0099>

### Unmaintained warnings (5)

| RUSTSEC | Crate | Version | Title | Date |
|---|---|---|---|---|
| RUSTSEC-2021-0139 | `ansi_term`      | 0.12.1 | ansi_term is Unmaintained | 2021-08-18 |
| RUSTSEC-2020-0095 | `difference`     | 2.0.0  | difference is unmaintained | 2020-12-20 |
| RUSTSEC-2025-0057 | `fxhash`         | 0.2.1  | fxhash - no longer maintained | 2025-09-05 |
| RUSTSEC-2025-0119 | `number_prefix`  | 0.4.0  | number_prefix crate is unmaintained | 2025-11-17 |
| RUSTSEC-2025-0134 | `rustls-pemfile` | 1.0.4  | rustls-pemfile is unmaintained | 2025-11-28 |

All five enter the graph through the same `pact_consumer 0.10.8` dev-dep chain.

### Recommended fix (pheno-events)

Bump `pact_consumer` from `0.10` → `1.4` in `[dev-dependencies]`. The fleet already validates the upgrade path: `pheno-port-adapter` declares `pact_consumer = "1.4"` (resolves to `1.4.4`) and clears all 3 RUSTSEC vulnerabilities — the only residual warnings are 4 unmaintained crates (no critical/high).

Predicted outcome post-bump (per `pheno-port-adapter` baseline):
- Vulnerabilities: 3 → 0
- Warnings: 5 → 4 (drops `rustls-pemfile 1.0.4`, gains `rustls-pemfile 2.2.0` unmaintained warning)
- Risk: API drift between `pact_consumer 0.10` and `1.4` — the V3 → V4 async-pact API change. Estimated 2-4 h of test rewrite (the `#[tokio::test]` async contract tests in `tests/contract_*` need to switch from sync V3 builder to `MessagePactBuilder` V4 API).

If the rewrite cost is too high, the alternative is a `deny.toml` ignore list:

```toml
# pheno-events/deny.toml
[advisories]
ignore = [
    "RUSTSEC-2026-0104",  # rustls-webpki 0.101.7 — dev-dep only (contract tests)
    "RUSTSEC-2026-0098",  # rustls-webpki 0.101.7 — dev-dep only
    "RUSTSEC-2026-0099",  # rustls-webpki 0.101.7 — dev-dep only
    "RUSTSEC-2021-0139",  # ansi_term — unmaintained, transitive via pact
    "RUSTSEC-2020-0095",  # difference — unmaintained, transitive via pact
    "RUSTSEC-2025-0057",  # fxhash — unmaintained, transitive via pact
    "RUSTSEC-2025-0119",  # number_prefix — unmaintained, transitive via pact
    "RUSTSEC-2025-0134",  # rustls-pemfile — unmaintained, transitive via pact
]
```

**Recommendation:** Bump `pact_consumer` to 1.4. Ignore list is acceptable as a *temporary* stopgap (v23), not the long-term posture.

## pheno-port-adapter — 0 vulnerabilities, 4 unmaintained warnings

**Severity:** LOW (no `vulnerability`-tagged advisories; only `unmaintained = "warn"` matches)
**Path:** dev-dep only (`[dev-dependencies] pact_consumer = "1.4"`)

| RUSTSEC | Crate | Version | Title | Date |
|---|---|---|---|---|
| RUSTSEC-2021-0139 | `ansi_term`      | 0.12.1 | ansi_term is Unmaintained | 2021-08-18 |
| RUSTSEC-2020-0095 | `difference`     | 2.0.0  | difference is unmaintained | 2020-12-20 |
| RUSTSEC-2025-0057 | `fxhash`         | 0.2.1  | fxhash - no longer maintained | 2025-09-05 |
| RUSTSEC-2025-0134 | `rustls-pemfile` | 2.2.0  | rustls-pemfile is unmaintained | 2025-11-28 |

### Recommended action (pheno-port-adapter)

**No action required.** The repo's `deny.toml` already declares `unmaintained = "warn"`, which is the correct posture for transitive dev-dep warnings. The 4 warnings all flow from `pact_consumer 1.4.4` → `pact_mock_server 2.2.2` → `pact_models 1.3.11` and require upstream pact-rust fixes (out of fleet scope).

## Clean Repos — 8 of 10

| Repo | Crates scanned | Notes |
|------|---------------:|-------|
| `pheno-chaos`        |   7 | minimal dep tree; serde, rand, tokio — all current |
| `pheno-cli-base`¹    |  42 | clap 4.5, tracing 0.1, tracing-subscriber 0.3 — current |
| `pheno-config`       | 143 | large but clean; zeroize, secrecy, argon2 all current |
| `pheno-context`      |  52 | workspace member; cross-repo unified lockfile |
| `pheno-errors`       | 118 | thiserror, anyhow, snafu — all current |
| `pheno-flags`        |  73 | figment, serde — all current |
| `pheno-otel`         |  83 | opentelemetry 0.27 — current |
| `pheno-tracing`      |  92 | tracing 0.1, tracing-subscriber 0.3, opentelemetry 0.27 — current |

¹ `pheno-cli-base/Cargo.lock` was generated by this audit (was missing in HEAD). Re-run `cargo audit` after the lock lands to confirm.

## Scan Methodology

```bash
# For each top-level pheno-* repo:
cd <repo>
cargo audit --no-fetch 2>&1 | tee /tmp/cargo-audit-2026-06-22/<repo>.txt

# Per-repo summary derived from output:
grep -c "^Crate:"    # total advisory entries
grep -c "unmaintained"  # of those, unmaintained warnings
grep "error:" | tail -1   # real vulnerabilities
grep "warning:" | tail -1 # unmaintained/notice warnings
```

`--no-fetch` was used to keep the audit deterministic against the local cached advisory DB (1134 advisories at `/Users/kooshapari/.cargo/advisory-db`). The DB was last updated 2026-04-22 per the most recent RUSTSEC entry in the scan results.

## Recommended Fixes — Priority Order

1. **P0 (this cycle)** — `pheno-events`: bump `pact_consumer` from `0.10` → `1.4` to clear all 3 RUSTSEC vulnerabilities. Owner: orch-v22-SD5-pact-bump. Estimate: 2-4 h (API rewrite for V3 → V4 contract tests).
2. **P3 (informational)** — `pheno-port-adapter`: no action; `deny.toml` already correctly handles the 4 unmaintained warnings.
3. **P3 (informational)** — `pheno-cli-base`: no action (Cargo.lock correctly gitignored for libs).
4. **P2 (next cycle)** — add `cargo audit --deny warnings` to each pheno-* repo's `ci.yml` per ADR-042 monthly cadence. See CI integration snippet below.
5. **P3 (next cycle)** — establish the monthly audit cron per ADR-042: weekly Monday 09:00 PDT `findings/71-pillar-2026-06-17-schema.md` L46 sweep, plus a focused `cargo audit` sweep against all top-level pheno-* repos.

## CI Integration Snippet (per ADR-042)

Add to `pheno-events/.github/workflows/ci.yml` (or create if absent):

```yaml
- name: Security audit
  run: |
    cargo install --locked cargo-audit
    cargo audit --deny warnings --no-fetch
```

`--deny warnings` will fail CI on the 3 RUSTSEC vulnerabilities. Use `--deny warnings` for `pheno-events` and `pheno-port-adapter` (treats unmaintained as error too); for the other 8 clean repos, plain `cargo audit` is sufficient.

## Comparison to Prior Baselines

| Date | Doc | Top-level pheno-* vulns | Source |
|---|---|---:|---|
| 2026-06-20 03:45 PDT | v12-cargo-audit-baseline.md | 0 | 9 nested Rust workspaces (incl. `pheno`, `HexaKit`, `PhenoMCP`, `PhenoCompose`) |
| 2026-06-21 16:33 PDT | SIDE-01-dep-audit.md | (different scope: federation deps) | cross-repo federation dependency audit |
| 2026-06-22 (this turn) | cargo-audit-2026-06-22.md | **3** (1 repo) | 10 top-level pheno-* repos |

**Δ:** +3 vulnerabilities vs v12 baseline. All attributable to `pheno-events` being newly promoted to top-level at v20 cycle 10 (2026-06-22) without a follow-up audit on its dev-dep pact chain.

## Artifacts

- Raw per-repo audit output: `/tmp/cargo-audit-2026-06-22/{repo}.txt` (10 files, ~70KB total)
- This findings doc: `findings/cargo-audit-2026-06-22.md`

## Acceptance Criteria

- [x] 10 top-level pheno-* Rust repos scanned
- [x] Per-repo findings captured (output to `/tmp/`)
- [x] RUSTSEC advisories identified and categorized (vulnerability vs unmaintained)
- [x] Recommended fixes with concrete actions + owners
- [x] Pillar score impact noted (L46 vulnerability management: currently 2 → target 3)
- [x] CI integration snippet provided per ADR-042
- [x] Comparison to prior baselines (v12 + SIDE-01)

## Pillar Score Impact

**L46 (Vulnerability management):** 2.5 → 3.0 once `pheno-events` `pact_consumer` bump merges.
**L49 (SLSA / supply chain):** 2.5 → 2.5 (no change; out of scope for this audit).
**L52 (Encryption-at-rest mandate — ADR-078):** not affected (no new crypto findings).
**L53 (Pen-test / bug-bounty — ADR-080):** not affected (no exploit PoC, but the 3 webpki CVEs should be flagged in the bug-bounty scope as "vulnerable in dev-dep tree only").

---

*End of findings doc. Total wall time: ~25 min including lockfile generation, 10-repo scan, and analysis.*
