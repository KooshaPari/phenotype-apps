# SIDE-26 — Top-level Makefile vs Justfile parity

**Date:** 2026-06-21 PDT
**Scope:** Root `/Users/kooshapari/CodeProjects/Phenotype/repos/` task-runner surface
**Files compared:**
- `Justfile` (172 lines, fleet-wide, canonical per `AGENTS.md` line 20)
- `Makefile.cli` (84 lines, Bifrost CLI build only)

---

## 1. Files present at root

```
Justfile        5,498 bytes   2026-06-21 13:41  fleet-wide task runner (ADR-022 / ADR-031)
Makefile.cli    2,320 bytes   2026-06-21 13:41  Bifrost CLI build only (Go)
```

There is **no** top-level `Makefile` or `makefile` — only the suffix-tagged `Makefile.cli`. Justfile is the only entry in `.pre-commit-config.yaml` and the only one referenced by `grade.sh` / `.github/workflows/*`.

## 2. Side-by-side target table

| Concern | Justfile recipe | Makefile.cli target | Parity |
|---|---|---|---|
| Help / list targets | `default` (`just --list`) | `cli-help` (`@echo` block) | both |
| Self-verify / parse-check | `justfile-verify` (L29.1) | — | just only |
| Dependency install | `install` (polyglot auto-detect) | — | just only |
| Build | `build` (polyglot auto-detect) | `cli-build`, `cli-release` | both, disjoint |
| Test | `test` (polyglot auto-detect) | `cli-test` | both, disjoint |
| Lint / vet | `lint` (polyglot auto-detect) | — | just only |
| Format | `fmt` (polyglot auto-detect) | — | just only |
| CI composite | `ci` (`install build test lint`) | — | just only |
| Secret scan | `audit` (trufflehog) | — | just only |
| Dep policy | `deny` (`govulncheck` / `cargo deny`) | — | just only |
| SSOT validation | `validate-ssot` | — | just only |
| Bench | `bench` (criterion + pytest-benchmark) | — | just only |
| CI cache stats | `cache-stats log_path=…` | — | just only |
| Changelog | `changelog` (git-cliff, L67) | — | just only |
| Grade composite | `grade` (delegates to `grade.sh`) | — | just only |
| Clean | `clean` | `cli-clean` | both, disjoint |
| Coverage | `coverage` (Go, 85 % gate) | — | just only |
| CLI run/version/init/server/deploy/config/plugin/dataset | — | `cli-run` `cli-version` `cli-init` `cli-server` `cli-deploy` `cli-config` `cli-plugin` `cli-dataset` | make only |

**Recipe counts:** Justfile 17 recipes — Makefile.cli 13 targets. **Overlap:** 3 (build / test / clean). **Total unique surface:** 17 + 13 − 3 = 27 targets across the two files.

## 3. Conventions compared

| Aspect | Justfile | Makefile.cli |
|---|---|---|
| Runner | `just` (casey/just) | POSIX `make` (any flavour) |
| Shebang per recipe | `#!/usr/bin/env bash` + `set -euo pipefail` | implicit shell (`@` for silent) |
| Globals | `set shell := ["bash", "-cu"]` (single line) | Variables with `:=` (5 vars + 1 list) |
| Language scope | polyglot auto-detect (`package.json` / `Cargo.toml` / `pyproject.toml` / `go.mod`) | Go-only (hardcodes `./cmd/bifrost`) |
| Variable syntax | `{{name}}` interpolation | `$(NAME)` and `$$VAR` (shell) |
| Parametric recipes | yes (`cache-stats log_path="…"` default) | no; uses `$(ARGS)` passed through |
| Help UX | `just --list` auto-generated | hand-written `cli-help` target with `@echo` |
| Tier markers | yes (`# Tier-0 hygiene:` prefix + L<n> refs) | none |
| Pillar cross-ref | yes (L29.1, L31, L57, L65, L67, etc.) | none |
| Pre-commit hook | yes (`justfile-verify` in `.pre-commit-config.yaml` L16-19) | no |
| CI workflow consumer | yes (`.github/workflows/audit.yml`, `deny.yml`) | no (Bifrost builds via its own Go toolchain) |
| Output glyphs | none | `✓` Unicode tick after each `@echo` |
| Failure messaging | `if [ -x ./scripts/… ]; then … else echo missing; exit 1` | no error handling; relies on `set -e` (none set) |
| Polyglot install | yes (5 toolchains) | no (`go install` only) |
| Conditional logic | yes (per-recipe `if [ -f … ]`) | no (`@for platform in $(PLATFORMS)` only) |
| Cross-platform release | implicit via `cli-release` | explicit 5-platform matrix (`darwin/amd64` `darwin/arm64` `linux/amd64` `linux/arm64` `windows/amd64`) |

## 4. Doc style compared

**Justfile** (excerpt `Justfile:9-13`):
```
# Tier-0 hygiene: Justfile parse + variable evaluation check (L29.1)
# Invoked by .pre-commit-config.yaml `justfile-verify` hook. Passes if
# (a) `just --list` parses the recipe block cleanly and (b) all `set` /
# `export` variables evaluate without error.
justfile-verify:
```

- Header comment line per recipe, 1-3 lines
- Tier prefix (`Tier-0 hygiene:`) for fleet-wide recipes
- Pillar ref (`L29.1`, `L65`, `L67`, etc.) embedded in comment
- Indentation: 4 spaces under recipe name
- Blank line between recipe groups

**Makefile.cli** (excerpt `Makefile.cli:13-22`):
```
cli-help:
	@echo "Bifrost CLI Build Commands"
	@echo "=========================="
	@echo "  make cli-build       - Build CLI for current platform"
	…
```

- No header comment line above targets
- Help target is the only documentation surface
- Tab-indented recipe bodies (Makefile requirement)
- Echo-based UX with `✓` glyph (3 per target on average)
- `.PHONY` declared once at top (only 6 of 13 targets listed)

## 5. Overlap & gap analysis

### True overlap (3 targets)

| Semantic | Justfile | Makefile.cli | Notes |
|---|---|---|---|
| build | `build` (polyglot: `cargo build --workspace` / `npm run build` / `go build ./...`) | `cli-build` (only `go build -o $(BUILD_DIR)/bifrost ./cmd/bifrost`) | Makefile.cli is a **subset** of Justfile's `build` |
| test | `test` (polyglot: `cargo test --workspace` / `npm test` / `go test ./...` / `pytest tests/`) | `cli-test` (`go test -v ./cmd/bifrost/cli/...`) | Same subset relationship |
| clean | `clean` (`rm -rf node_modules dist target build .next coverage __pycache__`) | `cli-clean` (`rm -rf $(BUILD_DIR) $(DIST_DIR)`) | Makefile.cli is again narrower |

### Just-only surface (14 recipes)

These have **no Makefile.cli equivalent** and are the bulk of the fleet hygiene story:
- `justfile-verify` — pre-commit gate (L29.1)
- `install` — polyglot dep install
- `lint`, `fmt` — code quality (per-language)
- `ci` — composite
- `audit` (trufflehog), `deny` (cargo-deny / govulncheck), `validate-ssot` — security/SSOT
- `bench` — perf regression (L57)
- `cache-stats log_path=…` — CI cache visibility (L31)
- `changelog` — L67 auto-gen via git-cliff
- `grade` — composite final gate
- `coverage` — 85 % gate

### Makefile-only surface (10 targets)

- `cli-help` — Justfile's `default` (`just --list`) covers this; **redundant**
- `cli-install` — Justfile's `install` covers Go; **redundant**
- `cli-release` — 5-platform cross-compile; **no equivalent** in Justfile today
- `cli-run`, `cli-version`, `cli-init`, `cli-server`, `cli-deploy`, `cli-config`, `cli-plugin`, `cli-dataset` — Bifrost CLI subcommand dispatch; **no equivalent**

## 6. Why this split exists

The bifurcation is historical, not architectural:

1. `Justfile` was the fleet task runner from the v6 / v7 era (ADR-022, ADR-031). It was designed for polyglot fleet hygiene.
2. `Makefile.cli` was added when Bifrost CLI landed in the monorepo (the `argis-extensions` → `phenotype-apps` lineage). Go projects conventionally ship a `Makefile`; the Bifrost maintainers kept the idiomatic Go pattern.
3. There was never an explicit decision to consolidate the two. Both exist today in the same directory with **no** cross-reference (verified: `grep -n "Makefile.cli\|cli-build\|cli-test" Justfile` returns 0 hits; `grep -n "just " Makefile.cli` returns 0 hits).

The risk surface from the dual-runner is small but real:
- A contributor who runs `make cli-build` never touches `justfile-verify`, so the L29.1 pre-commit gate is the only thing catching Justfile regressions — there's no gate for `Makefile.cli` parse correctness.
- CI workflows only invoke `just` recipes; the `cli-release` cross-platform matrix in `Makefile.cli` is **never run in CI** — it has no provenance beyond manual developer invocation.
- `grade.sh` (the canonical fleet grade) calls `just` exclusively. A `Makefile.cli` failure does not block grade.

## 7. Recommendation

**Adopt Justfile as the sole task runner; absorb Makefile.cli as a `cli-*` recipe group; delete Makefile.cli.** Specifically:

1. **Keep Justfile as canonical** (already codified in `AGENTS.md:20` — *"Orchestration: `just` (Justfile)"*). No new ADR needed.
2. **Move the 13 Makefile.cli targets into Justfile** as a `cli-*` group, preserving Go-idiom cross-compile matrix under `cli-release`. Just handles multi-line bash cleanly via `#!/usr/bin/env bash` recipe bodies.
3. **Delete `Makefile.cli`** once the absorbed recipes are committed and tested (`just cli-build && just cli-test && just cli-release` from a clean checkout).
4. **Update `.github/workflows/`** to invoke `just cli-release` for the Bifrost release artifact job — this brings the cross-platform matrix under CI provenance for the first time.
5. **Add `justfile-verify`-style check for the new `cli-*` recipes** (the existing L29.1 hook covers this automatically once they live in Justfile).

### Why Just, not Make

| Criterion | Just | Make | Verdict |
|---|---|---|---|
| Cross-platform shell | bash / sh / cmd configurable per-recipe | one shell per Makefile | just wins |
| Auto-detected polyglot install/test/build | trivial (the file already does this) | requires more branching | just wins |
| `just --list` self-doc | yes | requires hand-rolled `help` target | just wins |
| `justfile-verify` L29.1 hook | already in pre-commit | no equivalent | just wins |
| Fleet convention | codified in `AGENTS.md` and ADR-022 | none | just wins |
| Cross-compile matrix | one recipe, multi-line bash | idiomatic `$(PLATFORMS)` loop | tie (both fine) |
| Every Linux distro has it | no (single Rust binary to install) | yes (GNU make) | make wins |

The single Make advantage (ubiquity) does not outweigh Just's fleet-convention status. The fleet already requires `just` (L29.1, pre-commit, grade.sh).

## 8. Sample unified Justfile target list (proposed)

Add the following block to the root `Justfile` (replaces `Makefile.cli`):

```just
# ----------------------------------------------------------------------------
# Bifrost CLI build group (was Makefile.cli — absorbed SIDE-26, 2026-06-21)
# ----------------------------------------------------------------------------

# Variables
cli_name      := env_var_or_default("CLI_NAME", "bifrost")
cli_version   := env_var_or_default("CLI_VERSION", "1.0.0")
cli_main      := env_var_or_default("CLI_MAIN", "./cmd/bifrost")
build_dir     := env_var_or_default("BUILD_DIR", "./bin")
dist_dir      := env_var_or_default("DIST_DIR", "./dist")
cli_platforms := "darwin/amd64 darwin/arm64 linux/amd64 linux/arm64 windows/amd64"

# Default CLI help (was `cli-help` in Makefile.cli)
[doc("Show Bifrost CLI build commands")]
cli-help:
    @just --list cli

# Build Bifrost CLI for the current platform (was `cli-build`)
cli-build:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{build_dir}}"
    go build -o "{{build_dir}}/{{cli_name}}" \
        -ldflags="-X main.Version={{cli_version}}" \
        "{{cli_main}}"
    @echo "✓ Built: {{build_dir}}/{{cli_name}}"

# Install Bifrost CLI globally (was `cli-install`)
cli-install: cli-build
    #!/usr/bin/env bash
    set -euo pipefail
    go install -ldflags="-X main.Version={{cli_version}}" "{{cli_main}}"
    @echo "✓ Installed: {{cli_name}}"

# Run Bifrost CLI tests (was `cli-test`)
cli-test:
    #!/usr/bin/env bash
    set -euo pipefail
    go test -v ./cmd/bifrost/cli/...

# Clean Bifrost CLI build artifacts (was `cli-clean`)
cli-clean:
    #!/usr/bin/env bash
    set -euo pipefail
    rm -rf "{{build_dir}}" "{{dist_dir}}"
    @echo "✓ Cleaned"

# Cross-platform release build (was `cli-release` — first time under CI)
[doc("Build Bifrost CLI for all 5 platforms")]
cli-release:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{dist_dir}}"
    for platform in {{cli_platforms}}; do
        goos=$(echo "$platform" | cut -d/ -f1)
        goarch=$(echo "$platform" | cut -d/ -f2)
        output="{{dist_dir}}/{{cli_name}}-{{cli_version}}-$goos-$goarch"
        [ "$goos" = "windows" ] && output="${output}.exe"
        echo "  Building $goos/$goarch..."
        GOOS=$goos GOARCH=$goarch go build -o "$output" \
            -ldflags="-X main.Version={{cli_version}}" "{{cli_main}}"
    done
    @echo "✓ Built all platforms in {{dist_dir}}"

# Bifrost CLI subcommand dispatch (was `cli-run` / `cli-version` / etc.)
[doc("Run a Bifrost CLI subcommand (default: --help)")]
cli subcommand="--help":
    #!/usr/bin/env bash
    set -euo pipefail
    "{{build_dir}}/{{cli_name}}" {{subcommand}}
```

After absorbing this block, the unified Justfile has **17 + 8 = 25 recipes** (the 5 pure dispatch recipes `cli-run` / `cli-version` / `cli-init` / `cli-server` / `cli-deploy` / `cli-config` / `cli-plugin` / `cli-dataset` collapse into one parametric `cli subcommand=…` recipe). Then delete `Makefile.cli`.

## 9. Migration steps (proposed)

1. **T1:** Land the `cli-*` block in a `chore/side-26-absorb-makefile-cli-2026-06-21` branch.
2. **T2:** Verify `just cli-build && just cli-test && just cli-release` from a clean checkout on macOS (heavy-runner — `device: heavy-runner`, the cross-platform build matrix takes >5 min).
3. **T3:** Update `.github/workflows/release.yml` (if present) to call `just cli-release` instead of `make cli-release`.
4. **T4:** Delete `Makefile.cli` and commit. CI must pass `just justfile-verify` post-deletion (L29.1 hook).
5. **T5:** Update this finding with closure status, then reference it from the v19 T-side audit closure doc.

## 10. References

- `Justfile:1` — fleet-wide task runner header
- `Makefile.cli:1` — `.PHONY` declaration
- `.pre-commit-config.yaml:16-19` — `justfile-verify` L29.1 hook
- `AGENTS.md:20` — "Orchestration: just (Justfile)" canonical statement
- ADR-022 — config consolidation (cited in Justfile header)
- ADR-031 — Configra absorb (cited in Justfile header)
- ADR-040 — test coverage gates per tier (codifies 80%/70%/60% gates that `just coverage` enforces)
