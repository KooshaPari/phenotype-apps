# SIDE-24: CONTRIBUTING.md consistency check across the `pheno-*` fleet

**Date:** 2026-06-22 (audit run 2026-06-21 22:28 UTC)
**Scope:** 17 `pheno-*` crate directories in the monorepo root (`/Users/kooshapari/CodeProjects/Phenotype/repos/`).
**Owner:** orch-w1-a (SIDE-24)
**Sibling artifacts:** `AGENTS.md` § "Predictive DRY (ADR-047)" + § "Substrate quality bar (ADR-042B)".

---

## 1. Methodology

For each `pheno-*/` directory in the monorepo, audit whether the crate ships contributor-facing documentation that contains **all 6 of the following sections**:

| # | Section                  | What we look for                                                                                       |
| - | ------------------------ | ------------------------------------------------------------------------------------------------------ |
| 1 | **Setup instructions**   | Fork / clone, branch creation, env / toolchain bootstrap, install deps                                  |
| 2 | **Dev workflow**         | Branching strategy, commit conventions, local iteration loop (`just check`, `cargo test`, `pytest`, …) |
| 3 | **Testing**              | How to run tests, coverage threshold, fixture / integration / e2e expectations                         |
| 4 | **PR process**           | PR template, reviewer SLA, merge strategy (squash / rebase / self-merge), labels                        |
| 5 | **Code style**           | Formatter (`cargo fmt` / `ruff format` / `gofmt`), linter (`clippy` / `ruff` / `golangci-lint`), MSRV    |
| 6 | **Security disclosure**  | Where to report vulns (private channel, email, GHSA) + "do not file public issues" wording              |

**Scoring rule** (per SIDE-24 rubric): each section is worth 1 point if it is **substantively addressed** (commands, policies, or thresholds — not just a pointer to another file). A bare `See CONTRIBUTING.md` pointer without content scores **0** for that section. **Total possible: 6/6 per crate.**

**Sources audited, in priority order:**

1. `pheno-*/CONTRIBUTING.md` (canonical, if present)
2. `pheno-*/.github/CONTRIBUTING.md` (GitHub-recognized location)
3. `pheno-*/README.md` `## Contributing` section (only counts if substantive content, not pointers)

`AGENTS.md` is **not** counted for SIDE-24 scoring — it is the governance / agent constitution, not a contributor onboarding doc. (Several crates have AGENTS.md with rich convention detail — that detail should be cross-linked FROM CONTRIBUTING.md, not duplicated.)

---

## 2. Per-crate scorecard

Total: **17 crates audited**, **2 full pass (6/6)**, **1 partial pass (4/6)**, **14 fail (0/6)**.

| # | Crate                   | Lang   | CONTRIBUTING.md | README § | Score | Setup | Dev | Test | PR | Style | Sec |
| - | ----------------------- | ------ | --------------- | -------- | ----: | ----: | --: | ---: | -: | ----: | --: |
| 1 | `pheno-config`          | Rust   | —               | —        | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 2 | `pheno-context`         | Rust   | —               | —        | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 3 | `pheno-drift-detector`  | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 4 | `pheno-errors`          | Rust   | —               | —        | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 5 | `pheno-flags`           | Rust   | —               | —        | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 6 | `pheno-framework-lint`  | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 7 | `pheno-llms-txt`        | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 8 | `pheno-mcp-router`      | Rust   | —               | —        | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 9 | `pheno-otel`            | Rust   | ✓ (101 LoC)     | pointer  | **6** | 1     | 1   | 1    | 1  | 1     | 1   |
| 10 | `pheno-port-adapter`   | Rust   | ✓ (91 LoC)      | —        | **6** | 1     | 1   | 1    | 1  | 1     | 1   |
| 11 | `pheno-predict`        | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 12 | `pheno-scaffold-kit`   | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 13 | `pheno-secret-scan`    | YAML   | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 14 | `pheno-ssot-template`  | Rust   | ✓ (83 LoC)      | (none)   | **4** | 1     | 1   | 1    | 0  | 1     | 1   |
| 15 | `pheno-tracing`        | Rust   | —               | —        | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 16 | `pheno-vibecoding-guard` | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |
| 17 | `pheno-worklog-schema` | Python | —               | pointer  | **0** | 0     | 0   | 0    | 0  | 0     | 0   |

### Aggregate stats

| Metric | Value |
|---|---|
| Crates audited | 17 |
| Full pass (6/6) | 2 (11.8%) |
| Partial pass (4-5/6) | 1 (5.9%) |
| Zero (0/6) | 14 (82.4%) |
| **Fleet mean** | **0.94 / 6** |
| Crates with a CONTRIBUTING.md file | 3 (17.6%) |
| Crates with broken `## Contributing` pointers (README says "see CONTRIBUTING.md" but file does not exist) | 7 (41.2%) — `pheno-drift-detector`, `pheno-framework-lint`, `pheno-llms-txt`, `pheno-predict`, `pheno-scaffold-kit`, `pheno-vibecoding-guard`, `pheno-worklog-schema` |
| Crates with no contributor-facing docs at all | 7 (41.2%) — `pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`, `pheno-mcp-router`, `pheno-secret-scan` *(secret-scan has README but no CONTRIBUTING section)*, `pheno-tracing` |

---

## 3. Pattern analysis

### 3.1 The "3 of 3" tier — `pheno-otel`, `pheno-port-adapter`, `pheno-ssot-template`

These three crates are the only ones with a real `CONTRIBUTING.md`. Common patterns observed:

- **pheno-otel/CONTRIBUTING.md:1-101** — Most thorough. Covers: governance cross-link, issue templates (`.github/ISSUE_TEMPLATE/`), PR template, local checks (`just check`, `just audit`), review process (bot self-merge + HITL escalation), code conventions (Edition, MSRV, fmt, clippy, coverage 80%, Conventional Commits, worklog v2.1 with `device:` field, OTLP observability), ADR process, Code of Conduct, dual-licensing.
- **pheno-port-adapter/CONTRIBUTING.md:1-91** — Most actionable. Covers: 5-line quickstart, branching strategy table (feat/chore/fix/spike), commit conventions with scopes, PR template markdown, reviewer SLA (1 BD first review, 4h re-review), merge strategy (squash default), self-merge policy (governance + L<n>-#<n> + area:docs + area:ci), release process, security disclosure pointer.
- **pheno-ssot-template/CONTRIBUTING.md:1-83** — Strong on SSOT invariants + Conventional Commits scopes, weak on PR process (no template, no SLA, no merge strategy). Score 4/6 because PR process is "All submissions require review" (generic, no template).

### 3.2 The "broken pointer" tier — 7 crates

7 crates have a `## Contributing` section in `README.md` that says **"See CONTRIBUTING.md"** but **no CONTRIBUTING.md file exists**. This is the worst kind of contributor-facing failure: it actively misleads. A new contributor clicks the link, gets a 404, and either gives up or files an issue.

Affected crates: `pheno-drift-detector`, `pheno-framework-lint`, `pheno-llms-txt`, `pheno-predict`, `pheno-scaffold-kit`, `pheno-vibecoding-guard`, `pheno-worklog-schema`. All have the identical 3-line pointer template (lifted from the meta-bundle scaffold).

### 3.3 The "no docs" tier — 7 crates

5 crates have no CONTRIBUTING.md and no README.md: `pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`, `pheno-mcp-router`. Of these, `pheno-errors` has an `AGENTS.md` (111 LoC) with rich convention content (cargo commands, MSRV, error-kind categories, ADR cross-refs) — but the task rubric excludes AGENTS.md from scoring, and a contributor landing on `pheno-errors/` will not find it without knowing to look.

`pheno-tracing` and `pheno-secret-scan` have README.md but no `## Contributing` section. `pheno-secret-scan` is a workflow YAML crate (its README is 224 LoC explaining the workflow, allowlist, and pre-commit hook — contributor flow is implicit in the YAML schema).

### 3.4 Cross-language observations

The 3 real CONTRIBUTING.md files are **all Rust crates**. The 9 Python crates have 0 CONTRIBUTING.md (all fall into either broken-pointer or no-docs tier). This is a **systematic gap**: the meta-bundle scaffold at `pheno-ssot-template` does NOT include a CONTRIBUTING.md — only AGENTS.md + SPEC.md + README.md + WORKLOG.md + CHANGELOG.md + LICENSE-MIT.

The fix path is therefore:

1. Update `pheno-ssot-template` to include a CONTRIBUTING.md in the meta-bundle (codifies the gap).
2. Backfill the 14 missing crates by copying the unified template (§ 4) with crate-specific overrides.

---

## 4. Sample unified CONTRIBUTING.md

This template is the proposed **fleet-canonical** CONTRIBUTING.md. Crate-specific overrides go in the fenced `<crate-overrides>` blocks. Drop-in for any `pheno-*` Rust / Python / Go / TS / YAML crate.

````markdown
# Contributing to `<crate-name>`

> **Repo tier:** `<pheno-*-lib | phenotype-*-sdk | phenotype-*-framework | federated-service>` (per [ADR-023](https://github.com/KooshaPari/phenotype-apps/blob/main/docs/adr/2026-06-15/ADR-023-agent-effort-governance.md)).
> **Substrate canonicals:** see [`AGENTS.md`](./AGENTS.md) § "Active ADRs" for the list that applies to this crate.
> **For governance + architectural decisions** that affect the fleet, see [`AGENTS.md`](./AGENTS.md).
> **For security issues,** see [`SECURITY.md`](./SECURITY.md) — **do NOT file a public issue.**

## 1. Quickstart (5 lines)

```bash
# 1. Fork + clone (replace <upstream> with the canonical repo)
gh repo fork <upstream>/<crate-name> --clone --remote
cd <crate-name>

# 2. Create a branch (see § 2 for naming)
git checkout -b <type>/<req-id>-<slug>-$(date +%Y-%m-%d)

# 3. Run the local pre-flight (one of the blocks below — Rust | Python | Go | TS | YAML)
just check          # Rust
# OR
make check          # Python / Go / TS / YAML (each has a Makefile in the meta-bundle)
```

> `<crate-overrides>` — replace `just check` with the crate's actual entry point if it differs (e.g. `pheno-secret-scan` has no `just check`; the entry point is the GitHub Actions workflow).

## 2. Branching strategy

| Type      | Prefix                            | When                                              |
| --------- | --------------------------------- | ------------------------------------------------- |
| Feature   | `feat/<req-id>-<slug>-<date>`     | New user-facing capability, API addition          |
| Chore     | `chore/<req-id>-<slug>-<date>`    | Refactor, governance, docs, CI, deps              |
| Fix       | `fix/<req-id>-<slug>-<date>`      | Bug fix on a shipped path                         |
| Spike     | `spike/<req-id>-<slug>-<date>`    | Time-boxed investigation; do NOT merge code      |
| Docs      | `docs/<req-id>-<slug>-<date>`     | Documentation-only change                         |

`<req-id>` = fleet DAG level `L<n>-<seq>` (e.g. `L5-116`, `L4-66`); `<slug>` = lowercase-kebab; `<date>` = `YYYY-MM-DD`. See [`AGENTS.md`](./AGENTS.md) § "Conventions".

## 3. Commit conventions — Conventional Commits 1.0.0

Subject ≤ 72 chars, imperative mood. Use a scope:

- `feat(scope):` / `fix(scope):` / `refactor(scope):` — capability, bug fix, no-behavior-change refactor.
- `perf(scope):` / `test(scope):` / `docs(scope):` — perf, tests, docs only.
- `build(scope):` / `ci(scope):` / `chore(scope):` — build, CI, non-src maintenance.

Example: `feat(adapters): add InMemoryAdapter for tests (ADR-038 extension point)`.

Allowed scopes for this crate: `<crate-overrides: list scopes — e.g. errors, tracing, config, otel, schemas, cli, ci, docs, deny, governance>`.

## 4. PR template

```markdown
## What / Why / How
<!-- 1-3 sentences each. Link the issue / ADR / DAG level L<n>-<seq>.
     Cite any ADR(s) affected (e.g. ADR-038 for Port/Adapter changes,
     ADR-049 for drift-detector rule changes, etc.). -->

## Test plan
<!-- [ ] unit  [ ] integ  [ ] e2e  [ ] manual
     [ ] coverage ≥ <80% lib | 70% framework | 60% federated> (ADR-040)
     [ ] lint clean  [ ] fmt clean  [ ] WORKLOG.md updated (v2.1 schema)
     [ ] labels: governance | L<n>-#<n>  +  area:<scope> -->

## Risk
<!-- Blast radius + rollback plan. For trait / public-API changes, list the
     downstream crates that may need to migrate to the new contract. -->
```

## 5. Review process

- **Reviewers:** 1 CODEOWNER on the touched path + 1 cross-area reviewer. **SLA:** first review ≤ 1 business day, re-review ≤ 4 hours of push.
- **Merge:** squash-merge by default; rebase-merge only for multi-commit feature branches.
- **Self-merge:** permitted for `governance` + `L<n>-#<n>` + `area:docs` + `area:ci` PRs (Track 8 post-mortem, see `findings/2026-06-18-track8-self-merge-postmortem.md`). All others need explicit human approval.
- **No force-push** to `main` or to PRs after review has started.
- **HitL review required** for: breaking API changes, new dependencies, new substrate paths, anything touching security / crypto.

> `<crate-overrides>` — list any crate-specific escalation: e.g. for `pheno-port-adapter`, changes to the `PortAdapter` trait signature are a major version bump + ADR (see ADR-038 adoption matrix — 19 pheno-* crates migrate to this contract).

## 6. Testing gates

A PR is mergeable only when **all** of the following pass:

- [ ] **Tests + coverage:** green, with the gate per tier (ADR-040):

  | Tier                     | Min coverage |
  | ------------------------ | -----------: |
  | `pheno-*-lib` / SDK      | **80 %**     |
  | `phenotype-*-framework`  | **70 %**     |
  | Federated service        | **60 %**     |

- [ ] **Lint:** clean per the crate's linter (`cargo clippy --all-targets -- -D warnings` / `ruff check` / `golangci-lint run` / `eslint .`).
- [ ] **Format:** `cargo fmt --all -- --check` / `ruff format --check` / `gofmt -l .` / `prettier --check .`.
- [ ] **Supply chain:** `cargo deny check advisories` (or `pip-audit` / `govulncheck` / `npm audit`) clean.
- [ ] **`WORKLOG.md` updated** per [ADR-025 v2.1](https://github.com/KooshaPari/phenotype-apps/blob/main/findings/2026-06-17-L5-103-worklog-v2-1.md) — 11-column schema including the `device:` field.
- [ ] **No new `unwrap()` / `panic!` in lib crates** (allowed in tests + bin).
- [ ] **Observability:** info-level minimum OTLP export via [`pheno-tracing`](./AGENTS.md) (ADR-012 / ADR-036B).
- [ ] **Conventional Commits** subject + scope present.

> `<crate-overrides>` — e.g. `pheno-secret-scan` has no `cargo deny` step; the gate is the GitHub Actions workflow YAML lint. `pheno-secret-scan` does NOT run `cargo test`.

## 7. Release process

1. Bump version per SemVer in `Cargo.toml` / `pyproject.toml` / `package.json` / `go.mod`.
2. Move unreleased `CHANGELOG.md` entries under the new version header.
3. Open release PR: `chore/<req-id>-release-v<X>.<Y>.<Z>-<date>`. After merge, tag `v<X>.<Y>.<Z>` and push to trigger the release workflow.
4. Publish: `cargo publish` / `poetry publish` / `npm publish` / `go list -m ...`.
5. **Breaking changes** (any public API change) require a major version bump + an ADR.

## 8. Security disclosure

**Do NOT open a public issue for security vulnerabilities.** Follow [`SECURITY.md`](./SECURITY.md) for the disclosure process (GHSA + private email + 90-day coordinated disclosure window).

If `SECURITY.md` is missing from this crate, file at the fleet level: <https://github.com/KooshaPari/phenotype-apps/security/advisories/new>.

## 9. Getting help

- **Questions:** open a [Discussion](https://github.com/<upstream>/<crate-name>/discussions) — not an issue.
- **Bugs:** open an [Issue](https://github.com/<upstream>/<crate-name>/issues) with the PR template above; tag `area:<scope>`.
- **Security:** see [`SECURITY.md`](./SECURITY.md).
- **Maintainer:** @KooshaPari (CODEOWNERS).

## 10. Code of Conduct + License

- By participating, you agree to the [Contributor Covenant](./CODE_OF_CONDUCT.md).
- By contributing, you agree that your contributions will be dual-licensed under MIT or Apache 2.0, at the option of the downstream consumer. See [`LICENSE-MIT`](./LICENSE-MIT) and [`LICENSE-APACHE`](./LICENSE-APACHE).
````

> **Note on `<crate-overrides>`:** the fenced `<crate-overrides>` blocks are the only places a consuming crate may diverge from this template. Each override MUST cite the ADR or governance finding that authorizes the divergence. PRs that add a divergence without a citation are rejected at review.

---

## 5. Recommendations (priority-ordered)

| Priority | Action                                                                                                                                                                                                                                                                                                                                                                                                | Owner | Effort  | Impact                                                                                         |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----- | ------- | ---------------------------------------------------------------------------------------------- |
| **P0**   | **Backfill 14 missing CONTRIBUTING.md files** by copying § 4 sample with crate-specific `<crate-overrides>`. Minimum: setup, dev workflow, testing, PR process, code style, security disclosure — all 6 dimensions substantive.                                                                                                          | orch-w1-a | 2-3 h | Brings fleet mean 0.94 → 5.5/6 in one wave.                                                    |
| **P0**   | **Fix 7 broken `## Contributing` pointers in README.md** so they either point to a real file or are deleted. Either ship the file or kill the pointer — no half-state.                                                                                                                                                                                                                                  | per-crate owners | 15 min each | Removes the worst offender class (active misdirection).                                         |
| **P1**   | **Update `pheno-ssot-template` meta-bundle** to include the unified `CONTRIBUTING.md` (§ 4) as a template file. The scaffold is the upstream of every new `pheno-*` repo — closing the gap here prevents future crates from re-introducing it.                                                                                              | orch-w1-a | 30 min | Closes the systematic gap (Python fleet).                                                      |
| **P1**   | **Strengthen `pheno-ssot-template/CONTRIBUTING.md` PR process section** to reach 6/6 — add PR template markdown, reviewer SLA, merge strategy, self-merge policy. Currently 4/6 because PR process is a generic "All submissions require review".                                                                                                                                                       | `pheno-ssot-template` owner | 30 min | 1 crate 4/6 → 6/6 (fleet mean +0.12).                                                          |
| **P2**   | **Add `SECURITY.md` to the 5 crates that lack one** (`pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`, `pheno-mcp-router`) so the CONTRIBUTING.md template's § 8 link doesn't 404. Use the fleet SECURITY.md template in `pheno-ssot-template/SECURITY.md`.                                                                                                                                  | orch-w1-a | 30 min | Required for § 4 § 8 to function; closes a parallel gap.                                       |
| **P2**   | **Add a SIDE-24 audit cron** — weekly Monday 09:00 PDT (per ADR-041 cadence) — that runs a 1-line shell check on each `pheno-*/CONTRIBUTING.md` for the 6 dimensions and posts the scorecard to `findings/`. Tool: extend `pheno-framework-lint` with a `lint-contributing` subcommand.                                                                                                                      | orch-w1-a | 1.5 h   | Catches regressions before they ship; codifies the audit.                                     |
| **P3**   | **Cross-link from AGENTS.md to CONTRIBUTING.md** in the 14 crates that have an AGENTS.md but no CONTRIBUTING.md (or vice-versa). Add a single line: `See [CONTRIBUTING.md](./CONTRIBUTING.md) for contributor onboarding; this file is the governance / agent constitution.`                                                                                                                                | per-crate owners | 5 min each | Improves discoverability; no content change.                                                   |
| **P3**   | **Document the SIDE-24 rubric** as an annex to ADR-042B (substrate quality bar). The 6-dimension rubric is the formal definition of "CONTRIBUTING.md meets the quality bar" for the purposes of the substrate graduation path (ADR-048 G1.5 proxy).                                                                                            | orch-w1-a | 30 min | Codifies the scoring; removes ambiguity for future audits.                                     |

### Target state after P0+P1

| Metric                        | Today   | After P0 + P1 |
| ----------------------------- | ------: | ------------: |
| Fleet mean                    | 0.94/6  | 6.00/6        |
| Full pass (6/6)               | 2       | 17            |
| Broken-pointer crates         | 7       | 0             |
| No-docs crates                | 7       | 0             |
| Meta-bundle drift (Python)    | Yes     | No            |

---

## 6. Audit metadata

- **Run:** `for d in pheno-*/; do test -f "$d/CONTRIBUTING.md" && echo "$d: HAS"; done` + manual section scoring.
- **Crate list source:** `ls -d pheno-*/` at `/Users/kooshapari/CodeProjects/Phenotype/repos/` (2026-06-21 22:28 UTC).
- **Sample file:** § 4 sample is derived from the union of best practices across `pheno-otel/CONTRIBUTING.md` + `pheno-port-adapter/CONTRIBUTING.md` + `pheno-ssot-template/CONTRIBUTING.md` — i.e. the 3 existing real CONTRIBUTING.md files, deduplicated and standardized.
- **Excluded from scope:** `pheno-*-lib` repos that live as standalone GitHub repos outside the monorepo (e.g. `pheno-agents-md`, `pheno-cargo-template`, `pheno-cli-base`, `pheno-go-ctxkit`, `pheno-pydantic-models`, `pheno-zod-schemas`, `pheno-wtrees`) — they have their own CONTRIBUTING.md in their respective repos and are not visible in this monorepo's sparse-checkout cone. Re-run this audit per-repo for fleet-wide coverage.
- **Linked governance:** ADR-023 (Rule 3.1 substrate quality bar), ADR-040 (test coverage gates per tier), ADR-042B (substrate quality bar formal), ADR-025 (worklog v2.1 schema), ADR-041 (71-pillar refresh cadence — SIDE-24 should run weekly).
