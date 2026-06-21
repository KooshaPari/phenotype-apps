# CLI Accessibility Requirements (L41)

> **Status (2026-06-21):** Authoritative. Written in response to v17 Wave C — T10 — L41 a11y closure track.
> The three governance CLI tools (`pheno-predict`, `pheno-framework-lint`, `pheno-drift-detector`) are
> currently **absent from the local checkout** (verified 2026-06-21 13:04 PDT via `ls -la`; see
> `findings/2026-06-21-v17-Wave-C-subagent-C.md`). This document is therefore the canonical spec;
> an executable `scripts/check-a11y.sh` should be generated from it the next time any of the three
> tools lands back on disk.

This document codifies the accessibility (a11y) requirements for every CLI tool that ships under the
`pheno-*` namespace on the local monorepo. It exists to keep the **screen-reader**, **machine-readable**,
and **motor-impaired** user experience of `pheno-predict`, `pheno-framework-lint`, and
`pheno-drift-detector` consistent with the rest of the Phenotype fleet and with WCAG 2.2 AA.

## 1. Scope

| Tool                   | Substrate (ADR)        | Primary surface              |
| :--------------------- | :--------------------- | :--------------------------- |
| `pheno-predict`        | ADR-047 (DRY predict)  | 4-criterion rule evaluator   |
| `pheno-framework-lint` | ADR-048 (graduation)   | Substrate graduation gate runner |
| `pheno-drift-detector` | ADR-049 (drift)        | App-substrate 3-pass drift scanner |

All three are governance tools. They are the front line of the **HITL-less dev from base intent**
goal in `AGENTS.md` § "Quality bar for new substrate" — every one of them is invoked by humans
(reviewers, ADR authors) and by automated CI. **Both audiences must be able to use them.**

## 2. Universal requirements (every CLI in the fleet)

A CLI tool in the Phenotype fleet **MUST** satisfy all of the following. Numbered for direct citation
in the per-tool tables in § 3.

| #   | Requirement                                                                                                                                                                                                                | Rationale                                                            |
| :-- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| R1  | Accept `--help` (and `-h`) on every subcommand. Output is plain text, ≤ 80 columns, exits 0.                                                                                                                                 | Screen reader discovery (NVDA, JAWS, VoiceOver, Orca).               |
| R2  | Every flag **MUST** carry a non-empty help string. A bare `--flag` with no `about=`/`help=` is a **lint failure**.                                                                                                          | `clap` / `argparse` defaults to empty help — silent failure mode.    |
| R3  | Every tool **MUST** expose a `--json` mode that emits its primary output as a single JSON document on stdout. JSON schema is documented in the tool's `SPEC.md`.                                                              | Machine consumption; subagent invocation; CI gating.                |
| R4  | When `--json` is set, **no decorative characters** (ANSI color, spinners, progress bars, Unicode box-drawing) are written to stdout. Status messages go to stderr only.                                                       | Pipe-parseable output.                                               |
| R5  | All error paths **MUST** exit non-zero with a structured exit code: `1` user error, `2` config error, `3` I/O error, `4` upstream/network error, `5` internal error. The codes are fixed fleet-wide.                          | Automation depends on disambiguating failures.                       |
| R6  | Colors, when enabled, **MUST** be color-blind safe. Use the Okabe–Ito palette or a `--no-color` fallback. Never rely on red/green alone — pair color with a glyph (`[OK]`, `[FAIL]`) or a word.                              | Deuteranopia / protanopia / tritanopia.                              |
| R7  | All interactive prompts **MUST** have a non-interactive equivalent (`--yes`, `--assume-defaults`, `--input-file`).                                                                                                                                                                          | CI use; motor-impaired users; subagent use.                          |
| R8  | TTY width detection: respect `$COLUMNS` (if set), fall back to `tput cols`, fall back to 80. Wrap help text accordingly. Never assume 120-column terminals.                                                                    | tmux, ssh from phones, accessible terminals.                         |
| R9  | Every tool **MUST** print its version as `pheno-<tool> <semver> (commit <sha>, built <date>)` on `--version` and include the same triple in `--json` output under a `version` key.                                                                                                          | Reproducibility; support workflows.                                  |
| R10 | Help text **MUST** lead with a one-line summary, then a usage line, then a flag table, then 1–3 examples. Flag tables **MUST** have header columns (Flag, Description, Default).                                                                                                          | Linear reading; not all screen readers handle wide tables gracefully. |

## 3. Per-tool flag matrix

### 3.1 `pheno-predict`

Predictive DRY discipline — checks whether a candidate abstraction would survive the 4-criterion rule
of ADR-047. See `docs/adr/2026-06-18/ADR-047-predictive-dry.md`.

| Flag                          | Type     | Required | Default     | Screen-reader label            | Notes                                                  |
| :---------------------------- | :------- | :------: | :---------- | :----------------------------- | :----------------------------------------------------- |
| `--candidate <PATH>`          | path     |    Y     | —           | Candidate abstraction path     | Path to the candidate code or schema.                  |
| `--existing <PATH>`           | path     |    N     | auto        | Existing abstractions root     | Defaults to fleet substrate root.                      |
| `--json`                      | bool     |    N     | `false`     | JSON output mode               | See R3, R4.                                            |
| `--yes`                       | bool     |    N     | `false`     | Assume yes for prompts         | See R7.                                                |
| `--strict`                    | bool     |    N     | `false`     | Fail on advisory hits          | Promotes ADVISORY → FAIL.                              |
| `--no-color`                  | bool     |    N     | auto        | Disable ANSI color             | See R6.                                                |
| `--help` / `-h`               | bool     |    N     | —           | Show usage                     | See R1.                                                |
| `--version` / `-V`            | bool     |    N     | —           | Show version                   | See R9.                                                |

### 3.2 `pheno-framework-lint`

Substrate graduation gate runner (ADR-048). Enforces the 4-tier gate table across a candidate substrate.

| Flag                          | Type     | Required | Default     | Screen-reader label            | Notes                                                  |
| :---------------------------- | :------- | :------: | :---------- | :----------------------------- | :----------------------------------------------------- |
| `--substrate <PATH>`          | path     |    Y     | —           | Substrate root                 | Candidate repo to grade.                               |
| `--tier <0..4>`               | int      |    N     | `1`         | Target graduation tier         | 0=raw, 1=lib, 2=sdk, 3=framework, 4=federated service. |
| `--json`                      | bool     |    N     | `false`     | JSON output mode               | See R3, R4.                                            |
| `--yes`                       | bool     |    N     | `false`     | Assume yes for prompts         | See R7.                                                |
| `--baseline <FILE>`           | path     |    N     | —           | Baseline gate table JSON       | For diff grading against a prior run.                  |
| `--no-color`                  | bool     |    N     | auto        | Disable ANSI color             | See R6.                                                |
| `--help` / `-h`               | bool     |    N     | —           | Show usage                     | See R1.                                                |
| `--version` / `-V`            | bool     |    N     | —           | Show version                   | See R9.                                                |

### 3.3 `pheno-drift-detector`

App-substrate 3-pass drift scanner (ADR-049). Reports when an app-level repo has drifted from its
canonical substrate (or vice versa).

| Flag                          | Type     | Required | Default     | Screen-reader label            | Notes                                                  |
| :---------------------------- | :------- | :------: | :---------- | :----------------------------- | :----------------------------------------------------- |
| `--app <PATH>`                | path     |    Y     | —           | App-level repo root            | The consumer side.                                     |
| `--substrate <PATH>`          | path     |    Y     | —           | Substrate repo root            | The producer side.                                     |
| `--json`                      | bool     |    N     | `false`     | JSON output mode               | See R3, R4.                                            |
| `--pass <1\|2\|3>`            | enum     |    N     | all         | Limit to a single pass         | Pass 1 = symbol, pass 2 = signature, pass 3 = semantic. |
| `--severity <info\|warn\|err>`| enum     |    N     | `warn`      | Minimum severity to report     | Filters out lower severities.                          |
| `--no-color`                  | bool     |    N     | auto        | Disable ANSI color             | See R6.                                                |
| `--help` / `-h`               | bool     |    N     | —           | Show usage                     | See R1.                                                |
| `--version` / `-V`            | bool     |    N     | —           | Show version                   | See R9.                                                |

## 4. Verification (when the tools land)

When `pheno-predict`, `pheno-framework-lint`, or `pheno-drift-detector` returns to the local
checkout, a `scripts/check-a11y.sh` (~80–100 lines, bash + jq) **MUST** be generated that walks each
tool, runs `--help`, and asserts:

1. Every required flag in § 3 is present (R2).
2. Every `--help` output starts with a one-line summary (R10).
3. Every tool exits 0 on `--version` and prints the version triple (R9).
4. Every tool, when run with `--json` against a trivial input, emits a parseable JSON document on stdout (R3, R4).
5. No flag table cell in `--help` exceeds 80 columns (R8).

Until then, § 3 + R1–R10 are the binding contract that any new implementation must satisfy. PRs that
add or modify one of the three tools **MUST** cite this document by file path in the PR description.

## 5. References

- `docs/adr/2026-06-18/ADR-047-predictive-dry.md`
- `docs/adr/2026-06-18/ADR-048-substrate-graduation-path.md`
- `docs/adr/2026-06-18/ADR-049-app-substrate-drift-detector.md`
- `findings/71-pillar-2026-06-20-weekly-2.md` (cycle 2 audit; L41 a11y pillar)
- `findings/2026-06-21-v17-Wave-C-subagent-C.md` (this turn's decision log)
- `docs/architecture/PRINCIPLES.md`
- WCAG 2.2 AA — <https://www.w3.org/TR/WCAG22/>
