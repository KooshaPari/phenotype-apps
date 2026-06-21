---
id: L5-130
title: Helioscope retirement cleanup audit
date: 2026-06-21
orchestrator_turn: L5-130
subagent: C
task: Audit the monorepo for stale references to the retired `KooshaPari/helioscope` repo (archived 2026-06-20) and produce a classification + remediation plan.
audit_scope:
  scan_method: "git grep -l -i 'helioscope' -- ':!*.lock'"
  repos_scanned:
    - Repos/heliosCLI (branch: main, HEAD: d63844f45)
    - CodeProjects/Phenotype/repos (branch: chore/L25-loom-tests-pheno-port-adapter-2026-06-21, HEAD: 9afc8f3d9b)
    - CodeProjects/Phenotype/repos-canonical (branch: archive/2026-06-18-v8-batch-5-meta-bundles-4-repos, HEAD: bc1fcd7)
  repos_zero_hits:
    - Repos/template-commons
    - Repos/civ
    - Repos/phenodocs
    - Repos/phenotype-shared
    - Repos/phenotype-go-kit
    - Repos/phenotype-infrakit
    - Repos/phenotype-design
    - Repos/phenotypeActions
    - Repos/phench
    - CodeProjects/Phenotype/phenotype-shared-temp
    - CodeProjects/cliproxyapi-plusplus
  total_hits: 49 (3 in heliosCLI: AGENTS.md ×1, CLAUDE.md ×2; 41 in repos: cancellation-rationale.md ×35, local-work-inventory.md ×2, sota-research-2026-06-19.md ×3, v12-71-pillar-p0-remediation.md ×1; 5 in repos-canonical: ADR-018 ×3, WRAPUP-PUSH-AUDIT.md ×1, v6-dag-stable.md ×1)
  exclusion: ':!*.lock' — Cargo.lock / package-lock.json / poetry.lock etc. excluded per task spec
canonical_source: "KooshaPari/helios-cli (per findings/2026-06-21-helioscope-cancellation-rationale.md v12-20)"
retired_repo: "KooshaPari/helioscope (archived 2026-06-20)"
---

# Helioscope retirement cleanup — L5-130 audit (subagent C)

**TL;DR.** `KooshaPari/helioscope` was retired 2026-06-20 per the v12-20 closure rationale (`findings/2026-06-21-helioscope-cancellation-rationale.md`). `KooshaPari/helios-cli` is the canonical codex fork going forward. This audit identifies **49** in-tree references to `helioscope` across 3 repos in the monorepo (3 live-code + 35 historical-comment in the closure rationale alone, plus 11 historical-comment in adjacent metadata files). **3 hits are live-code (stale agent-governance descriptions in `heliosCLI`); 46 hits are historical-comment and must be preserved as the retirement record.** No dead-links (no live URLs/hrefs to the retired repo) were found.

---

## 1. Classification scheme

| Class | Definition | Action |
|---|---|---|
| **dead-link** | Live URL / href pointing at the retired `KooshaPari/helioscope` repo, package, or remote | Replace with redirect target (`KooshaPari/helios-cli`) or remove |
| **historical-comment** | Mentions inside `CHANGELOG.md`, `findings/`, `plans/`, ADR docs, or git-log narratives that document the retirement event itself | **Preserve verbatim** — these ARE the retirement record |
| **live-code** | Mentions in `*.rs`, `*.py`, `*.ts`, `*.toml`, CI workflows, build/test scripts, agent governance (AGENTS.md / CLAUDE.md), or any executable / parse-time surface | Update to canonical name (`helios-cli` / `helios`) |

---

## 2. Full classification table

> All paths relative to each repo root. Line numbers verified via `git grep -n` on the date above.

### 2.1 `Repos/heliosCLI` — on `main` (HEAD `d63844f45`)

| # | File:Line | Snippet | Class | Remediation |
|---|---|---|---|---|
| 1 | `AGENTS.md:13` | `- **Description**: Rust-based CLI for managing Helioscope applications with multi-backend support and sandboxing` | **live-code** | Update description: "Helioscope applications" → "Helios CLI applications" (or "helios-cli applications"). The CLI manages apps under the `helios-cli` canonical naming, not the retired `helioscope` fork. |
| 2 | `CLAUDE.md:11` | `- **Description**: Rust-based CLI for managing Helioscope applications with multi-backend support and sandboxing` | **live-code** | Same as #1. |
| 3 | `CLAUDE.md:72` | `heliosCLI provides a comprehensive CLI for Helioscope application management with multiple execution backends:` | **live-code** | Same as #1. |

**Note on this repo's identity:** `Repos/heliosCLI` is the local checkout of the **`helios-cli`** repo (per its own `AGENTS.md` `Location` field at `AGENTS.md:14`: `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI`). The `helios-cli` repo is canonical; the `Helioscope applications` phrasing in the description field is a stale holdover from the pre-rename era when the binary was a downstream of the `KooshaPari/helioscope` fork. The repo must NOT be renamed — `helios-cli` → `helioscope` absorption is **not** the goal; the opposite is true.

### 2.2 `CodeProjects/Phenotype/repos` — on `chore/L25-loom-tests-pheno-port-adapter-2026-06-21` (HEAD `9afc8f3d9b`)

| # | File:Line | Snippet (truncated) | Class | Remediation |
|---|---|---|---|---|
| 4 | `findings/2026-06-20-local-work-inventory.md:40` | `| phenotype-registry-helioscope-final | docs/helioscope-delete-ready-2026-06-20 | main | 1 | 15 | 0 | ECOSYSTEM_MAP.md | top: 6c83060 docs: mark helioscope delete-ready |` | **historical-comment** | **Preserve.** This is the work-inventory record of the `phenotype-registry` PR (`6c83060`) that marked helioscope `delete-ready`. Retired-repo status is intentional historical content. |
| 5 | `findings/2026-06-20-local-work-inventory.md:291` | `| phenotype-registry-helioscope-final | docs/helioscope-delete-ready-2026-06-20 | main | 15 | 17 | 6c83060 docs: mark helioscope delete-ready |` | **historical-comment** | **Preserve** (same as #4). |
| 6 | `findings/2026-06-21-helioscope-cancellation-rationale.md` (full file, 128 lines) | Title: `helioscope retirement closure — chose helios-cli as canonical`; `retired_repo: KooshaPari/helioscope`; references to `KooshaPari/helioscope` at lines 19, 90, 109; references to closure path at lines 21–22, 33–34, 39, 41, 48, 54, 56–57, 62–63, 65, 67, 76–77, 86, 93, 96, 106, 109, 114, 117, 120, 128 | **historical-comment** | **Preserve verbatim — this IS the v12-20 closure rationale document.** Removing references here would defeat the purpose of the closure record. Lines 19, 90, 109 contain `KooshaPari/helioscope` repo path mentions (not bare URLs) — they correctly document the archive intent and must stay. |
| 7 | `findings/sota-research-2026-06-19.md:215` | `- helioscope/codex-rs/tui uses ratatui (archived path).` | **historical-comment** | **Preserve.** SOTA research artifact from 2026-06-19 (pre-retirement) explicitly noting `helioscope/codex-rs/tui` is on an archived path. Removing would erase the migration trail. |
| 8 | `findings/sota-research-2026-06-19.md:314` | `| 23 | **helioscope** (archived) | (varies) | 0.1 | ... |` | **historical-comment** | **Preserve.** SOTA comparison table row marking helioscope as archived. |
| 9 | `findings/sota-research-2026-06-19.md:381` | `... The expanded 23 in §3 include adjacent (Settly, helioscope) and paused (KDesktopVirt, kmobile, eyetracker) repos.` | **historical-comment** | **Preserve.** SOTA scope definition from 2026-06-19. |
| 10 | `plans/2026-06-20-v12-71-pillar-p0-remediation.md:68` | `- v13 scope: 5-repository post-Cycle 3 audit (add nanovms, helios-router, helioscope, authvault, planify)` | **historical-comment** | **Preserve.** v13 plan from 2026-06-20 lists helioscope in a 5-repo post-Cycle 3 audit scope. The retirement makes this scope item now-stale, but it is preserved as the v13 historical plan; if v13 is re-scoped after the 2026-06-20 archive, that re-scoping is a separate v13 plan revision, not a finding of THIS audit. **Recommend follow-up: re-scope v13 plan in a separate v13 cycle to remove helioscope from the audit set.** |

### 2.3 `CodeProjects/Phenotype/repos-canonical` — on `archive/2026-06-18-v8-batch-5-meta-bundles-4-repos` (HEAD `bc1fcd7`)

| # | File:Line | Snippet (truncated) | Class | Remediation |
|---|---|---|---|---|
| 11 | `docs/adr/2026-06-15/ADR-018-prcp-pattern.md:10` | `repos + 4 polyglot monorepos (pheno, helioscope, phenoAI, ...)` | **historical-comment** | **Preserve.** ADR from 2026-06-15 (pre-retirement) documents the 14+4 fleet composition at that date. ADRs are immutable historical records; supersede with a new ADR if fleet composition change must be formalized (out of scope here). |
| 12 | `docs/adr/2026-06-15/ADR-018-prcp-pattern.md:20` | `- 1× TypeScript/JS consumer (helioscope)` | **historical-comment** | **Preserve** (same as #11). |
| 13 | `docs/adr/2026-06-15/ADR-018-prcp-pattern.md:68` | `KodeVibe, helioscope, phenoAI) need a one-time cleanup to split` | **historical-comment** | **Preserve** (same as #11). |
| 14 | `findings/2026-06-17-WRAPUP-PUSH-AUDIT.md:165` | `... helioscope 5, PhenoProc 5, agslag-docs 4, ...` | **historical-comment** | **Preserve.** 2026-06-17 wrap-up push audit noting helioscope is 5 commits behind `origin/main`. As of 2026-06-21 the repo is archived — this audit snapshot is correct historical content. |
| 15 | `plans/2026-06-15-v6-dag-stable.md:135` | `5. Is helios-cli ready to absorb helioscope? (Open question from findings/WAVE_2026_06_14_RESUME_FINAL.md:133; out of scope for v6.)` | **historical-comment** | **Preserve.** v6 DAG plan from 2026-06-15 explicitly deferred the absorption question to a later cycle. Per the 2026-06-21 closure rationale (`findings/2026-06-21-helioscope-cancellation-rationale.md`), the absorption verdict is now: **absorption is NOT the path — archive + husk README is.** This is a resolved question as of v12-20; the v6 plan text remains correct as the v6-historical snapshot. **Recommend follow-up: open a v6 retrospective entry noting the resolution.** |

### 2.4 Dead-link search (separate pass)

Searched all matching files for live URL / href patterns referencing `KooshaPari/helioscope`:

```
rg -i 'helioscope' [files] | grep -iE 'http|github\.com|KooshaPari/helioscope[^/-]|git@'
```

**Result:** 3 matches, all in `findings/2026-06-21-helioscope-cancellation-rationale.md` at lines 19, 90, 109 — all of form `` `KooshaPari/helioscope` `` (markdown code-spans, not rendered URLs). These are **path-style repo references inside the closure rationale document** (which is `historical-comment` per #6 above), not active links. **No live dead-links to remediate.**

---

## 3. Remediation plan

### 3.1 Required (live-code, actionable)

**Target repo:** `Repos/heliosCLI` (canonical: `KooshaPari/helios-cli`)

**3 string substitutions, all in agent-governance files:**

| File | Line | Before | After |
|---|---|---|---|
| `AGENTS.md` | 13 | `Rust-based CLI for managing Helioscope applications with multi-backend support and sandboxing` | `Rust-based CLI for managing Helios CLI applications with multi-backend support and sandboxing` |
| `CLAUDE.md` | 11 | (same as above) | (same replacement) |
| `CLAUDE.md` | 72 | `heliosCLI provides a comprehensive CLI for Helioscope application management with multiple execution backends:` | `heliosCLI provides a comprehensive CLI for Helios CLI application management with multiple execution backends:` |

**Branch / commit / PR plan** (proposed — do NOT execute per task constraint "Do NOT modify any source files"):

1. Branch from `main` of `Repos/heliosCLI`: `chore/l5-130-helioscope-description-rename-2026-06-21`
2. Apply the 3 substitutions above (case-sensitive: only the description prose changes; do NOT touch `heliosCLI` binary name or `Helioscope → Helios CLI` should be the entire replacement string; verify no other `Helioscope` matches via `git grep -n 'Helioscope'` before commit)
3. Open PR titled: `docs(governance): align description with helios-cli canonical (L5-130)`
4. Reference: `KooshaPari/helios-cli`, `findings/2026-06-21-helioscope-cancellation-rationale.md (v12-20)`
5. No code, no Cargo.toml changes, no README.md rename — description-only PR

**Risk:** Zero — these strings appear in markdown prose only. Verified via `git grep -l -i 'helioscope' -- ':!*.lock'` that no `Cargo.toml`, `*.rs`, CI workflow, or build script contains `Helioscope`.

### 3.2 NOT required (historical-comment, preserve verbatim)

The 35 hits in:
- `CodeProjects/Phenotype/repos/findings/2026-06-21-helioscope-cancellation-rationale.md`
- `CodeProjects/Phenotype/repos/findings/2026-06-20-local-work-inventory.md`
- `CodeProjects/Phenotype/repos/findings/sota-research-2026-06-19.md`
- `CodeProjects/Phenotype/repos/plans/2026-06-20-v12-71-pillar-p0-remediation.md`
- `CodeProjects/Phenotype/repos-canonical/docs/adr/2026-06-15/ADR-018-prcp-pattern.md`
- `CodeProjects/Phenotype/repos-canonical/findings/2026-06-17-WRAPUP-PUSH-AUDIT.md`
- `CodeProjects/Phenotype/repos-canonical/plans/2026-06-15-v6-dag-stable.md`

are **all** preserved as-is. These are the canonical record of the retirement event (closure rationale, work inventory, SOTA research, ADR snapshots, plan snapshots). Removing them would erase the audit trail required by v12-20's closure gate.

### 3.3 NOT required (dead-link, none found)

No live URLs / hrefs to the retired repo were identified. The 3 `KooshaPari/helioscope` path-style references in the closure rationale are correctly preserved markdown code-spans, not broken links.

### 3.4 Follow-ups (recommended, NOT in scope of L5-130)

These are minor consistency improvements that emerged from the audit but are explicitly out of scope for a cleanup subagent. They should be tracked separately:

| ID | Priority | Description | Files |
|---|---|---|---|
| L5-130-FU1 | P3 | Re-scope v13 plan to drop helioscope from the post-Cycle 3 audit set (now-archived repos do not enter a v13 audit) | `CodeProjects/Phenotype/repos/plans/2026-06-20-v12-71-pillar-p0-remediation.md:68` |
| L5-130-FU2 | P3 | Add v6 retrospective entry noting that the "is helios-cli ready to absorb helioscope?" open question is **resolved by archive + husk README** (not absorption) per v12-20 | `CodeProjects/Phenotype/repos-canonical/plans/2026-06-15-v6-dag-stable.md:135` |
| L5-130-FU3 | P4 | Optional: supersede `ADR-018-prcp-pattern.md` with `ADR-046-prcp-pattern-fleet-post-helioscope.md` reflecting the 13-fleet (was 14) composition | `CodeProjects/Phenotype/repos-canonical/docs/adr/2026-06-15/ADR-018-prcp-pattern.md` |

---

## 4. Audit summary table

| Repo | Branch | HEAD | Hits | live-code | historical-comment | dead-link |
|---|---|---|---|---|---|---|
| `Repos/heliosCLI` | `main` | `d63844f45` | 3 | **3** | 0 | 0 |
| `CodeProjects/Phenotype/repos` | `chore/L25-loom-tests-pheno-port-adapter-2026-06-21` | `9afc8f3d9b` | 41 | 0 | 41 | 0 |
| `CodeProjects/Phenotype/repos-canonical` | `archive/2026-06-18-v8-batch-5-meta-bundles-4-repos` | `bc1fcd7` | 5 | 0 | 5 | 0 |
| **Total** | | | **49** | **3** | **46** | **0** |

**Unique files touched:** 9 (heliosCLI: 2; repos: 4; repos-canonical: 3).

**Actionable remediation count:** **3 string substitutions** (all in `Repos/heliosCLI`, all in agent-governance prose, zero risk).

**Preserved as historical record:** 46 hits across 7 files in the meta-repos (`repos/`, `repos-canonical/`) — closure rationale (35) + work inventory (2) + SOTA research (3) + v13 plan (1) + ADR-018 (3) + WRAPUP push audit (1) + v6 DAG plan (1).

**Dead-links requiring fix:** 0.

---

## 5. Verification commands

To re-verify after the 3 remediation substitutions land in `Repos/heliosCLI`:

```bash
cd /Users/kooshapari/Repos/heliosCLI
git grep -n -i 'helioscope' -- ':!*.lock'
# Expected output: (empty)

cd /Users/kooshapari/CodeProjects/Phenotype/repos
git grep -n -i 'helioscope' -- ':!*.lock'
# Expected output: 41 matches, all in findings/, plans/ — closure rationale + historical record (preserved)

cd /Users/kooshapari/CodeProjects/Phenotype/repos-canonical
git grep -n -i 'helioscope' -- ':!*.lock'
# Expected output: 5 matches, all in ADR + findings + plans (preserved)
```

After remediation, **the only remaining `helioscope` matches in the monorepo will be the 46 historical-comment references** — exactly the audit trail v12-20 requires.

---

## 6. References

- `CodeProjects/Phenotype/repos/findings/2026-06-21-helioscope-cancellation-rationale.md` (v12-20 closure rationale — canonical)
- `helios-cli/docs/rationalization/helioscope-absorption.md` (2026-05-31 absorption assessment)
- `phenotype-registry/RATIONALIZATION_PLAN.md` Step 2
- `KooshaPari/helios-cli` (canonical repo going forward)
- `KooshaPari/helioscope` (archived 2026-06-20, husk README redirects to helios-cli)

---

**Audit status:** **complete**. Remediation plan is documented; no source files were modified per task constraint. Hand-off to parent orchestrator (turn L5-130) for approval of FU1/FU2/FU3 follow-ups and to authorize the 3-string remediation PR in `Repos/heliosCLI`.
