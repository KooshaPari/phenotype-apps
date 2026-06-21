# ADR-079: Secrets-Vault Migration Roadmap (L50)

| Field | Value |
|---|---|
| **Status** | ACCEPTED |
| **Date** | 2026-06-21 |
| **Pillar** | L50 (Secrets Management), closes L48 + L52 cross-references |
| **Cycle** | v19 (cycle 9, P1 reduction) |
| **Author** | forge subagent (L5-153) on macbook |
| **Plan** | `plans/2026-06-21-v19-71-pillar-cycle-9-p0.md` Track T1 |
| **Numbering note** | Originally planned as ADR-077 per v19 plan; renumbered to ADR-079 because ADR-077 is taken by `2026-06-20/ADR-077-slsa-l3-provenance.md` and ADR-078 by `2026-06-20/ADR-078-cosign-keyless-signing.md`. Tracks T2-T5 in the v19 plan must similarly renumber when authored. |
| **Related** | ADR-046 (federation mTLS+OIDC), ADR-012 (pheno-tracing), ADR-026 (Factory AI Agent Readiness), ADR-027 (LFS 3-tier), ADR-042 (security cadence), ADR-077 (SLSA L3 provenance, 2026-06-20), ADR-078 (cosign keyless signing, 2026-06-20), ADR-015 v2.1 (worklog schema) |

## Context

The 71-pillar cycle-8 probe (`findings/2026-06-21-v18-cycle-8-probe.md` §6 finding 1) scored **L50 (Secrets Management)** at **1.5/3** — the lowest score in the Security domain (L46-L55). Drivers:

- **6 distinct secret backends** in active use across the 11 substrate crates: env-vars, dotenv, file-permissions, AWS Secrets Manager, Vault via direct token (1 crate), GitHub Actions secrets (CI only).
- **0 documented rotation policy**; secrets live until manually rotated or until a personnel change forces ad-hoc rotation.
- **0 secret-access audit log**; `cargo audit` does not check for in-repo hardcoded secrets. `gitleaks` partial coverage in CI but no fleet-wide `.pre-commit-config.yaml`.
- **2 archived substrate repos** (`phenotype-ops`, `pheno-tracing`) still hold dormant tokens; archived ≠ purged.
- **43 dependabot vulnerabilities** in the queue; no SLSA L3 provenance attestation on any binary (closed by ADR-077/2026-06-20).
- **0 fleet-wide OIDC trust** between CI → cloud → federated services; ADR-046 sets the federation contract but no concrete implementation has shipped.

The fleet grew 11 substrate crates + 9 dependent SDKs + 6 federated services in 18 months. Secret sprawl is now the **#1 unaddressed risk** in the cycle-9 audit and a precondition for any SOC 2 Type II evidence collection (cycle-8 deliverable `findings/2026-06-21-v18-T1-L17-fedramp-soc2-readiness.md` flags L50 as a SOC 2 CC6.1 control gap).

## Decision

Adopt **HashiCorp Vault** as the canonical secrets backend for the Phenotype fleet. Migrate incrementally, substrate-by-substrate, with a **12-week (≈90-day) phased rollout** ending **2026-09-13**.

### Why Vault (not HCP Vault, not AWS Secrets Manager)

- **Self-hosted first, HCP if HA needed before 2026-Q4.** Self-hosted gives full audit-log ownership (required by SOC 2 CC7.2) and lets us reuse existing on-prem K8s capacity.
- **Vault's dynamic credentials** (AWS, database, PKI) close 3 separate backlog items (L50 secrets, L52 encryption-at-rest, L49 key-management) with one platform.
- **Vault's auth methods** (AppRole, Kubernetes, OIDC, Userpass) cover all 4 substrate classes (Rust process, Python service, Go service, CLI tool) without bespoke credential plumbing.
- **Mature polyglot SDKs** (`vault` Rust crate, `hvac` Python, `vault/api` Go) — no in-house client code required.
- **OTLP-native audit logs** ship straight to `pheno-otel` per ADR-012, no extra log-forwarding pipeline.

### Phase 0 — Pre-migration (week 1, 2026-06-22 → 2026-06-28)

| # | Task | Owner | Done = |
|---|---|---|---|
| 0.1 | Stand up Vault dev mode in `pheno-port-adapter` (already has `rust-vault` + `rustls` deps) | orch | `vault server -dev` reachable, `vault status` exits 0 |
| 0.2 | Author `docs/architecture/secrets-management-convention.md` (200 LOC, 4 sections: classification, storage, rotation, audit) | orch (T1 this turn) | committed, cross-referenced from ADR-079 |
| 0.3 | Add `gitleaks` pre-commit hook to monorepo-root `.pre-commit-config.yaml` | orch | `pre-commit run --all-files` clean |
| 0.4 | Audit dormant secrets in archived repos `phenotype-ops` + `pheno-tracing` (read-only token inventory) | orch | report in `findings/2026-06-28-dormant-secret-audit.md` |
| 0.5 | Establish rotation cadence (cross-refs ADR-078/cosign `rotation-cadence.md`): 90d service tokens, 30d human tokens, 24h dynamic DB creds, immediate on personnel change | orch | `docs/architecture/secrets-management-convention.md` §3 |
| 0.6 | Wire `scripts/vault_smoke.sh` into `just vault-smoke` target | orch (T1 this turn) | smoke green on local + CI |

### Phase 1 — Non-prod substrates (weeks 2-3, 2026-06-29 → 2026-07-12)

Migrate 4 substrate crates in test/dev only:

- `pheno-config` (lowest risk; config-only secrets)
- `pheno-context` (request-scoped context, optional secrets)
- `pheno-flags` (boolean flags; no real secrets)
- `pheno-errors` (no secrets; metadata only — establishes the migration runbook template)

Migration is mechanical: `std::env::var("FOO")` → `vault.read("secret/data/pheno/dev/pheno-config/foo")`. All 4 crates already use `pheno-port-adapter` indirectly through workspace deps.

**Exit criteria:** all 4 crates pass `just test` with Vault dev server; no env-var reads remain in `src/` (grep clean).

### Phase 2 — Prod substrates (weeks 4-6, 2026-07-13 → 2026-08-02)

Migrate 4 production-bound substrate crates:

- `pheno-tracing` — OTLP endpoint + API key (currently GitHub Actions secret)
- `pheno-port-adapter` — TCP listener + optional TLS cert (currently file-perm)
- `pheno-otel` — OTLP collector config (currently AWS Secrets Manager)
- `pheno-mcp-router` — MCP API tokens + LLM provider keys (currently direct tokens)

Vault **AppRole** auth with **24h token TTL**. Vault agent sidecar in K8s for `pheno-port-adapter` + `pheno-otel`. OTLP audit logs to `pheno-otel` collector per ADR-012.

**Exit criteria:** zero hardcoded secrets in `src/**` (gitleaks + ripgrep both clean); all 4 crates in production use Vault via the `vault::Client` wrapper; rollback path tested (see §Rollout §4 below).

### Phase 3 — Federated services (weeks 7-9, 2026-08-03 → 2026-08-23)

Migrate 3 federated services:

- `PhenoMCP` (Python service; uses `hvac` Python client with `pyo3-vault-rs` FFI bridge for hot-path reads)
- `phenoForge` (Go service; uses Vault agent sidecar + Kubernetes auth method per ADR-046)
- `HeliosLab` (Helios CLI tool; uses `vault` CLI subprocess, userpass + MFA enforced)

**Cross-cutting work:** ship `docs/architecture/secrets-management-convention.md` §2 (OIDC binding for CI) to `phenoCI-templates` so all 47 active repos pick up the binding automatically.

**Exit criteria:** all 3 services pass a 24h soak with Vault audit log showing 100% secret-read coverage; SLO p99 secret-read latency < 50ms (budget per ADR-040 80% lib coverage gate framework).

### Phase 4 — SDKs + workflow migration (weeks 10-12, 2026-08-24 → 2026-09-13)

- `phenotype-go-sdk` (vault/api Go client)
- `phenotype-python-sdk` (hvac Python client + auto-refresh wrapper)
- `phenotype-rust-sdk` (vault rust crate, already a dep)

SDK consumers migrate **opportunistically** — breaking change gated to a major version bump (v3.0.0 for Rust, v2.0.0 for Python/Go). Deprecation notice in CHANGELOG.md 6 weeks before cutover (2026-07-13).

### Cross-track compatibility

- **ADR-046 (Federation mTLS+OIDC):** Vault serves as the OIDC issuer for service-to-service identity; CI jobs obtain Vault tokens via OIDC instead of long-lived `VAULT_TOKEN` env vars. Compatible.
- **ADR-012 (pheno-tracing substrate):** Vault audit logs export via OTLP/HTTP to `pheno-otel` collector. Same pipeline as substrate traces.
- **ADR-026 (Factory AI Agent Readiness):** Vault adoption is the canonical "Documented → Standardized" lever for the **Security** pillar of the Factory AI 5-level model. Reference impl ships with this ADR.
- **ADR-077 (SLSA L3 provenance, 2026-06-20):** Vault stores the cosign signing key used by ADR-077's SLSA workflow. Eliminates the long-lived `COSIGN_KEY` GitHub Actions secret.
- **ADR-078 (cosign keyless signing, 2026-06-20):** cosign key references Vault PKI engine; rotation is automatic (24h cert TTL).
- **ADR-027 (LFS 3-tier policy):** No interaction. LFS objects are not secrets.
- **ADR-042 (security audit cadence):** Vault audit log review becomes a monthly `cargo audit` + `vault audit verify` line item.

## Consequences

### Positive

- **L50 closes 1.5 → 2.0 (Adequate).** 6 backends → 1.
- **L52 (Encryption-at-Rest)** gains an indirect bump from Vault's encrypted-at-rest transit storage + sealed-unseal lifecycle.
- **L54 (Federated Identity)** gets a concrete OIDC client surface (handoff to T3 in v19).
- **SOC 2 CC6.1** (logical access controls) gets a defensible audit-trail baseline.
- **ADR-046 implementation path** becomes concrete — Vault is the OIDC issuer, not just a theoretical federation contract.
- **Cost reduction** at fleet scale: AWS Secrets Manager per-secret pricing at $0.40/secret/month × ~200 secrets = $80/month → Vault self-hosted = $0 incremental (already-paid K8s capacity).
- **Personnel-change rotation** becomes one Vault command (`vault token revoke -prefix`) instead of N GitHub Actions secret edits + N AWS Secrets Manager updates.

### Negative / Tradeoffs

- **Self-hosted Vault = ops burden.** Sealed-unseal ceremony, HA Raft cluster sizing, backup/restore. Mitigated by phased rollout (Phase 0 ha dev mode, Phase 3 first prod HA cluster).
- **Vendor lock-in mitigation:** all Vault access goes through `pheno-port-adapter` Vault port; swapping backends is a single port impl change (per ADR-038 hexagonal L4 policy).
- **12-week timeline risk:** any of the 47 active repos could stall Phase 4 SDK migration. Mitigated by opportunistic SDK rollout — SDK consumers migrate when they next touch the auth path.
- **AppRole secret_id rotation must be automated** — manual rotation will rot. Phase 2 includes the rotation automation script (not separately tracked here; lives in `scripts/vault_rotate_approle.sh`).
- **HCP Vault decision deferred** to post-Phase 4 evaluation (2026-09-13+). If we hit >100 RPS before then, we revisit.

### Carry-over to v20+

1. **HCP Vault vs self-hosted HA** — decision at >100 RPS sustained.
2. **Vault disaster recovery** — RTO 4h, RPO 15m. Replicate to second region. TBD ADR.
3. **Vault as PKI issuer for TLS certs** (currently self-signed in `pheno-port-adapter`). Defer to v20+ to avoid double-scope.
4. **Cross-region Vault replication** for federated multi-region deployments. ADR-046 followup.

## Rollout (12-week / 90-day wave plan)

| Week | Phase | Workstream A (substrate) | Workstream B (services + SDKs) | Workstream C (governance + audit) |
|---|---|---|---|---|
| 1 (06-22) | Phase 0 | Vault dev cluster up; `vault_smoke.sh` green | gitleaks pre-commit hook landed | `secrets-management-convention.md` v1.0 |
| 2 (06-29) | Phase 1 | `pheno-config` migrated to Vault | dormant-secret audit report | gitleaks CI check active on PRs |
| 3 (07-06) | Phase 1 | `pheno-context`, `pheno-flags`, `pheno-errors` migrated | OIDC binding ref (for ADR-046 hand-off) | monthly security review (per ADR-042) |
| 4 (07-13) | Phase 2 | `pheno-tracing` migrated (OTLP key in Vault) | AppRole rotation script live | vault audit log → OTLP wired (ADR-012) |
| 5 (07-20) | Phase 2 | `pheno-port-adapter` migrated (TLS cert via Vault PKI) | first prod HA Raft cluster stand-up | soak test 24h on `pheno-tracing` |
| 6 (07-27) | Phase 2 | `pheno-otel`, `pheno-mcp-router` migrated | rollback drill (simulated Vault seal) | mid-cycle 71-pillar refresh — L50 should hit 2.0 |
| 7 (08-03) | Phase 3 | — | `PhenoMCP` (Python hvac) migrated | SOC 2 CC6.1 evidence draft |
| 8 (08-10) | Phase 3 | — | `phenoForge` (Go k8s auth) migrated | pen-test vendor shortlist (T5 input) |
| 9 (08-17) | Phase 3 | — | `HeliosLab` (CLI userpass+MFA) migrated | 24h soak across all 3 services |
| 10 (08-24) | Phase 4 | `phenotype-rust-sdk` v3.0.0 cutover window opens | deprecation notice in CHANGELOG.md | — |
| 11 (08-31) | Phase 4 | `phenotype-python-sdk` v2.0.0 cutover window opens | SDK consumer migration tracking dashboard | — |
| 12 (09-07) | Phase 4 | `phenotype-go-sdk` v2.0.0 cutover window closes | final soak test + rollback rehearsal | end-cycle 71-pillar refresh — final L50 score |
| 13 (09-13) | closure | `README.md` updated; legacy `.env` templates archived | ADR-079 closure comment + v20+ carry-over list | SOC 2 CC6.1 evidence finalized |

**Critical-path tracking:** weekly Monday 09:00 PDT status update in `findings/2026-06-28-vault-week-N.md` per ADR-041 cadence. Slip > 1 week triggers v19 plan re-cut.

## Risk register (5 items)

| # | Risk | Severity | Likelihood | Mitigation |
|---|---|---|---|---|
| R1 | Vault HA cluster sizing is misjudged at >100 RPS | Med | Med | Phase 3 includes load test (`scripts/vault_load_test.sh`); trigger HCP Vault evaluation if p99 > 50ms sustained |
| R2 | AppRole secret_id rotation drifts to manual | High | Low | `scripts/vault_rotate_approle.sh` + cron; CI fails PR if any secret_id > 90d old |
| R3 | 47 active repos refuse opportunistic SDK migration (Phase 4 drag) | Med | High | Phase 4 is non-blocking; SDK v2.0.0/v3.0.0 deprecated path is `vault::legacy::env_var` with `#[deprecated]` warning — no hard cutover |
| R4 | Vault audit log to OTLP drops events under load (back-pressure) | Med | Low | `pheno-otel` collector has 10k events/s backpressure buffer per ADR-012; audit drops page on-call via `pheno-events` |
| R5 | SOC 2 auditor wants per-secret access attribution beyond Vault's request-path metadata | Low | Med | Vault audit log enrichment with `X-Forwarded-For` + `pheno-context` trace ID (cross-ref ADR-012); see `secrets-management-convention.md` §4 |

## Success criteria

- [ ] **6 backends → 1** (Vault only). AWS Secrets Manager + GitHub Actions secrets + dotenv all retired from `src/**` (gitleaks + ripgrep clean).
- [ ] **100% rotation compliance** for service tokens (Vault audit shows 0 tokens > 90 days).
- [ ] **100% MFA** for human tokens (Vault userpass + TOTP enforced).
- [ ] **Audit log coverage** = 100% of secret reads, written to OTLP within 5s.
- [ ] **p99 secret-read latency** < 50ms (per ADR-040 framework coverage budget).
- [ ] **71-pillar L50** moves from 1.5 → **2.0** on the 2026-06-28 refresh.
- [ ] **SOC 2 CC6.1 evidence** finalized by 2026-09-13 closure.

## Open questions

- **Vault HA backend (Raft vs Consul):** Raft chosen for simplicity; revisit at >100 RPS or >3 nodes.
- **Self-hosted vs HCP Vault:** Self-hosted first; HCP if HA needed before 2026-Q4.
- **Disaster recovery:** RTO 4h, RPO 15m. Replicate to second region. TBD ADR (carry-over to v20+).
- **Key custody for ADR-077 cosign signing:** Vault transit engine stores the key; rotation cadence TBD with the SLSA track owner.

## Migration plan reference

See `plans/2026-06-21-v19-71-pillar-cycle-9-p0.md` §T1 for week-by-week schedule, ownership, and rollback triggers. See `docs/architecture/secrets-management-convention.md` for path conventions, OIDC binding, dynamic credential rotation TTLs, and the `.env`-migration runbook.

## Acceptance

This ADR is **ACCEPTED** for v19 cycle-9 execution. All 4 phases owned by the v19 orchestrator with weekly checkpoints per ADR-041. Plan owner: orch-v19 (forge). Subagent execution: forge (L5-153) on macbook.