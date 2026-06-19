# WORKLOG V21 GATE — Spec (L5-103 / ADR-025)

**Author:** forge orchestrator (L5-103, v8 plan, 2026-06-19)
**Status:** SPEC READY
**Canonical workflow:** `KooshaPari/pheno-ci-templates/.github/workflows/worklog-schema-v21-gate.yml@main`
**Replaces:** ad-hoc per-repo WORKLOG.md validators; v2.0 10-col format (ADR-015 superseded)

## 1. Problem

Every pheno-* fleet repo carries a `WORKLOG.md` — a human-readable,
markdown-table audit log of work performed against the repo. The fleet
currently has 12+ such files (one per pheno-* lib, framework, SDK, and
federated service), and they all need to converge on a single canonical
schema.

The schema was bumped to **v2.1** (11 columns, with the new `device:`
column added at position 9) per **ADR-025** (L5-103, 2026-06-17). The
prior v2.0 schema (10 columns, no `device:`) is **deprecated** with a
hard cutover at **2026-06-22**. v1 (6-col, pre-substrate) and v1+device
(legacy hybrid) are tolerated indefinitely per ADR-025 — no migration
required for either.

The pre-existing enforcement landscape is broken:

1. **No single source of truth.** Some repos use `pheno-worklog-schema`'s
   upstream validator; some have hand-rolled Python scripts; some have
   nothing at all. Drift across the fleet is unmeasured.
2. **Cutover enforcement cannot scale.** 12+ repos × bespoke validators
   × an absolute date gate is unmaintainable.
3. **`device:` column enforcement is impossible without a fleet-wide
   gate.** The `device:` field is the audit trail for the device-fit
   gate in **ADR-023** (L5-101, 2026-06-15). A WORKLOG.md without a
   `device:` column cannot answer "what ran where?".

## 2. Decision

The canonical, fleet-wide WORKLOG v2.1 enforcement point is a **reusable
GitHub Actions workflow** at
`KooshaPari/pheno-ci-templates/.github/workflows/worklog-schema-v21-gate.yml`.
Every pheno-* consumer repo opts in by adding a thin caller workflow
in its own `.github/workflows/` that does nothing but `uses:` the
canonical one with appropriate `with:` inputs. The validator itself
lives in exactly one place; consumer repos do not re-implement it.

Authority chain (top-down): **ADR-025** (the "what") →
`pheno-worklog-schema/SPEC-v2.1.md` (the spec) → this workflow (the
enforcement point) → this spec (the workflow's contract).

A PR that violates v2.1 cannot be merged on or after **2026-06-22** if
the consumer repo has opted in (which is the default for every pheno-*
repo, see § 7).

## 3. WORKLOG v2.1 gate workflow

The workflow exposes five `workflow_call` inputs. Defaults match the
workflow's own header so the consumer can omit any input it is happy
with the default on.

| Input           | Type    | Default                                       | Purpose |
|-----------------|---------|-----------------------------------------------|---------|
| `cutover_date`  | string  | `"2026-06-22"`                                | v2.0 deprecation horizon (YYYY-MM-DD). v2.0 = WARN pre-cutover, FAIL post-cutover. |
| `warn_only`     | boolean | `false`                                       | Downgrade FAIL → WARN (exit 0). Use during the 2026-06-19/20/21 opt-in window. |
| `fail_new`      | boolean | `true`                                        | Require newly-added WORKLOG.md files to be v2.1 (no new v2.0/v1/v1+device files). |
| `paths`         | string  | `"WORKLOG.md docs/WORKLOG.md **/WORKLOG.md"`  | Glob (newline- or space-separated) of WORKLOG.md paths to validate. |
| `base_ref`      | string  | `""` (resolves to `github.base_ref` on PR, `main` otherwise) | Base ref for detecting newly-added files via `git diff --diff-filter=A`. |

The workflow runs on `ubuntu-latest` with a 5-minute timeout, checks
out the repo with `fetch-depth: 0` (full history is required for the
`git diff` based `fail_new` check), and writes its per-file report to
`findings/worklog-v21-gate.txt` (uploaded as a 30-day-retained workflow
artifact). The validator is an **inline Python script** vendored into
the workflow to avoid touching the `pheno-worklog-schema` submodule
(whose upstream owner reverted the L5-103 PR); the policy in the
script is the canonical reference, not the substrate library.

## 4. Format policy

The workflow recognizes exactly four historical formats plus two
non-format states. Detection is header-based — the first markdown
table header in the file is authoritative, columns are lowercased,
the markdown separator row (`|---|---|...`) is skipped.

| Format     | Columns | Has `device:` | Status (default)        | Rationale |
|------------|---------|---------------|-------------------------|-----------|
| **v2.1**   | 11      | yes           | **PASS** (always)       | Canonical per ADR-025. |
| **v2.0**   | 10      | no            | WARN pre-cutover / FAIL post-cutover | Deprecated per ADR-025; cutover 2026-06-22. |
| **v1+device** | <11  | yes           | **PASS** (always)       | Legacy hybrid; tolerated indefinitely per ADR-025. |
| **v1**     | 6-9     | no            | **PASS** (always)       | Pre-substrate; tolerated indefinitely per ADR-025. |
| `empty`    | —       | —             | FAIL                    | No markdown table header found. |
| `malformed` | any    | any           | FAIL                    | Header detected but column count + `device` position do not match any known format. |

`empty` and `malformed` are intentionally not "PASS even in legacy":
a WORKLOG.md that doesn't parse is a bug, regardless of era. Repos
that don't yet ship a WORKLOG.md trivially pass the gate (the `paths`
glob simply matches nothing).

## 5. Cutover behavior

The cutover logic is a single date comparison (`today >= cutover_date`):

- **Pre-cutover (today < 2026-06-22):** v2.0 → **WARN** with a
  per-file message stating "N day(s) remaining". The CI job exits
  with code 0; the WARN line is emitted to stderr and recorded in
  the gate report artifact. WARN does not block merges.
- **Post-cutover (today ≥ 2026-06-22):** v2.0 → **FAIL**. The CI job
  exits with code 1 and the PR cannot be merged (assuming the
  consumer repo's branch protection requires this workflow to pass,
  which is the recommended config).
- **v1 and v1+device:** **PASS** at all times. No cutover for legacy
  formats. They are out of scope for the v2.1 bump and have no
  deprecation horizon. Migrating them is **opt-in** and optional —
  these files are usually in archived/PAUSED repos per ADR-023 and
  reformatting them is wasted work.

## 6. fail_new policy

`fail_new=true` (the default) layers an additional check on top of the
format policy: **any WORKLOG.md file newly added in the current PR
must be v2.1**. Detection uses `git diff --name-only --diff-filter=A
<base_ref>...HEAD` against the consumer repo's base ref (`main` by
default, or `github.base_ref` on PRs).

A newly-added v2.0 file fails under `fail_new=true`:

- Pre-cutover: FAIL (overrides the WARN that an existing v2.0 file
  would get). Reason: "newly-added file must be v2.1 (got v2.0; ...)".
- Post-cutover: FAIL for a stricter reason — a new file inheriting a
  deprecated format is worse than an existing one.

A newly-added v1 or v1+device file also fails under `fail_new=true`,
even though the existing-format policy tolerates both indefinitely.
The rationale: a brand-new WORKLOG.md should be authored in the
canonical format, not a legacy one. There is no migration cost to
using v2.1 the first time. (See ADR-025 §"New file policy".)

## 7. Consumer integration

A consumer repo adds a thin caller workflow — the entire integration
is a 12-line YAML file. Copy this verbatim into
`KooshaPari/<consumer-repo>/.github/workflows/worklog-schema-v21-gate.yml`:

```yaml
name: WORKLOG v2.1 gate

on:
  pull_request:
  push:
    branches: [main]

jobs:
  worklog-gate:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/worklog-schema-v21-gate.yml@main
    with:
      cutover_date: "2026-06-22"
      warn_only: false
      fail_new: true
      paths: "WORKLOG.md docs/WORKLOG.md **/WORKLOG.md"
```

For repos that maintain WORKLOG.md in a non-standard location (e.g.
`logs/WORKLOG.md`), override the `paths` input with additional globs.
The `**/WORKLOG.md` glob in the default also covers multi-worklog
repos (one per workspace member) — no input override required.

Branch protection: the consumer repo SHOULD mark this workflow as a
required status check on `main` and on PRs. Without that, the gate is
informational only and does not block bad merges.

## 8. Opt-in window

The fleet has a **3-day soft rollout window** before the hard cutover:

| Date range        | `warn_only` setting                | Effect |
|-------------------|------------------------------------|--------|
| 2026-06-19 → 2026-06-21 | `warn_only: true` (recommended) | v2.0 files log WARN, exit 0. Safe to merge. |
| 2026-06-22+       | `warn_only: false` (default)       | v2.0 files FAIL. Hard enforcement begins. |

During the opt-in window, consumers SHOULD set `warn_only: true` to
avoid surprising a PR author with a hard FAIL on a v2.0 file that
was committed before the bump. After the cutover, consumers MUST
remove `warn_only: true` (or set it to `false`) — leaving
`warn_only: true` post-cutover effectively disables the gate.

The validator does **not** auto-flip `warn_only` on the cutover date.
Each consumer repo is responsible for the one-line edit on or before
2026-06-22.

## 9. Migration path for tracked v2.0 files

For every existing v2.0 WORKLOG.md in the fleet, the canonical
migration script is at `pheno-worklog-schema/migrate_v2_to_v2_1.py`.
The script reads the v2.0 markdown table, inserts a new `device`
column at position 9 (0-indexed), infers the `device` value for
each existing row from the row's `Notes` column (e.g. "on MacBook"
→ `macbook`, "subagent run" → `subagent`, "CI" → `ci`, "heavy-runner"
→ `heavy-runner`), falls back to `device: unknown` for rows that
don't yield an inference, and writes the v2.1 file in place (or to
a new path with `--out`).

Run locally:

```bash
python3 pheno-worklog-schema/migrate_v2_to_v2_1.py \
  --in WORKLOG.md \
  --out WORKLOG.md \
  --report findings/worklog-v2-1-migration.txt
```

The migration is idempotent: running it on a v2.1 file is a no-op,
which makes it safe to include in a pre-commit hook as well (see
§ 10).

## 10. Local pre-commit / lefthook alternative

The CI gate is the **canonical** enforcement point — it is what
auditors and the AGENTS.md `device:` field forensic chain ultimately
trust. But for fast dev feedback (no 30-second CI round-trip), a
local validator is available: `pheno-worklog-schema/validate_worklog.py`
uses the same detection logic as the inline workflow script and
runs in < 1 second with no GitHub Actions dependency.

Wire it into a `lefthook.yml` pre-commit job:

```yaml
# lefthook.yml
pre-commit:
  commands:
    worklog-v21-check:
      glob: "**/WORKLOG.md"
      run: |
        python3 pheno-worklog-schema/validate_worklog.py \
          --path {staged_files} \
          --cutover-date 2026-06-22
```

The local validator is a **development convenience**, not an
authoritative gate. The CI workflow is the only source of truth.
A WORKLOG.md that passes locally but triggers the CI gate is a real
failure (e.g. base_ref drift, newly-added file detection difference).

## 11. References

- ADR-015 — V2 10-column WORKLOG.md schema (canonical, 2026-06-15 V5 SOTA sweep; the schema this worklog v2.1 supersedes)
- ADR-023 — Agent-effort governance / device-fit gate (L5-101, 2026-06-15; the policy that motivates the `device:` column)
- ADR-025 — WORKLOG v2.1 schema bump (11th `device:` column; L5-103, 2026-06-17; supersedes ADR-015; deprecation 2026-06-22)
- ADR-030 — `pheno-worklog-schema` v2.1 substrate (L5-104.5, 2026-06-17; the library the inline validator mirrors)
- `pheno-worklog-schema/SPEC-v2.1.md` — the v2.1 spec source-of-truth (column order, semantics, examples)
- `pheno-worklog-schema/migrate_v2_to_v2_1.py` — the migration script for v2.0 → v2.1
- `pheno-worklog-schema/validate_worklog.py` — the local dev validator
- `pheno-ci-templates/.github/workflows/worklog-schema-v21-gate.yml` — the canonical reusable workflow
- `findings/2026-06-17-L5-103-worklog-v2-1.md` — the ADR-025 decision log

## 12. Cross-references

- T19.1 audit doc: `findings/2026-06-18-ci-templates-audit.md` (T19.2 HOOKS_SKIP / T19.3 SKIP / T19.4 OIDC / T19.5 SBOM originate here too)
- T19.2 `HOOKS_SKIP=1` spec — `pheno-ci-templates/docs/specs/HOOKS_SKIP.md` (companion env-var spec)
- T19.3 `SKIP` env-var spec — `pheno-ci-templates/docs/specs/SKIP.md` (companion env-var spec for pre-commit configs)
- T19.4 OIDC spec — `pheno-ci-templates/docs/specs/OIDC.md` (related release-time gate; not invoked by WORKLOG.md)
- T19.5 SBOM spec — `pheno-ci-templates/docs/specs/SBOM.md` (related release-time gate; not invoked by WORKLOG.md)
- v8 plan Track T19: `plans/2026-06-18-v8-dag-stable.md` § 3.11 (this spec is a T19.6 child of that track)
