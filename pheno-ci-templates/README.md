# pheno-ci-templates

Canonical reusable GitHub Actions CI + release workflows for the **pheno-\***
fleet. One source of truth for the four languages the fleet ships in
(Rust, Python, Go, Node); each consumer pulls the workflows in via
`uses:` and only ever sees the language jobs that match the manifests
present in the consumer repo.

The seven workflows are:

- **`ci.yml`** — runs on `pull_request` / `push` (via the caller's
  trigger) and at PR time checks fmt + clippy + test + optional coverage
  for every detected language. The `workflow_call` trigger is paired with
  `workflow_dispatch` so the template can be smoke-tested from the
  Actions tab without any caller.
- **`release.yml`** — runs on tag push (`v*` or `release/*`) and creates
  a GitHub release, then publishes artifacts to crates.io / npm / PyPI /
  $GOBIN. Each publish step is gated by a `run_<lang>` boolean so a
  single tag can release only the languages the calling crate owns.
- **`deny.yml`** — runs `cargo-deny` (advisories, bans, licenses, sources)
  against the consumer's `Cargo.lock`. Absorbed from `McpKit` on
  2026-06-18 as part of the McpKit CI absorption (ADR-003, G-7).
- **`codeql.yml`** — runs GitHub CodeQL static analysis on the consumer
  repo with caller-selectable languages. Absorbed from `McpKit` on
  2026-06-18 (G-7). Replaces the McpKit passthrough to
  `phenoShared/.../codeql.yml` with a self-contained workflow.
- **`scorecard.yml`** — runs the OpenSSF Scorecard supply-chain
  analysis and publishes SARIF results to the Security tab. Absorbed
  from `McpKit` on 2026-06-18 (G-7).
- **`secrets-scan.yml`** — runs TruffleHog (filesystem or github mode)
  to detect leaked secrets. Absorbed from the two McpKit secrets-scan
  files (`secrets-scan.yml` + `trufflehog.yml`) on 2026-06-18 (G-7);
  deduplicated into a single workflow with a `scan_mode` input.
- **`release-attestation.yml`** — produces SLSA Build L2 provenance
  attestation for release artifacts. Absorbed from `McpKit` on
  2026-06-18 (G-7).

## Inputs

`ci.yml` accepts the following inputs (all optional with sensible
defaults; pass them via the caller's `with:` block):

| Input             | Type    | Default | Purpose                                                                 |
|-------------------|---------|--------:|-------------------------------------------------------------------------|
| `rust_msrv`       | string  | `1.75`  | Rust toolchain version (matches the fleet's `rust-toolchain.toml`)      |
| `python_version`  | string  | `3.11`  | Python version installed via `actions/setup-python`                     |
| `go_version`      | string  | `1.22`  | Go version installed via `actions/setup-go`                             |
| `node_version`    | string  | `20`    | Node.js version installed via `actions/setup-node`                      |
| `run_coverage`    | boolean | `false` | Run coverage reporting (cargo-llvm-cov / pytest-cov / go cover / npm)  |
| `run_clippy`      | boolean | `true`  | Run `cargo clippy --all-targets -- -D warnings` on the rust job         |

Pass-through secrets (so the caller does not have to redeclare them):
`CARGO_REGISTRY_TOKEN`, `NPM_TOKEN`, `PYPI_TOKEN`, `GITHUB_TOKEN`.

`release.yml` adds four feature flags for the publishing steps:

| Input              | Type    | Default | Purpose                                                |
|--------------------|---------|--------:|--------------------------------------------------------|
| `run_cargo_build`  | boolean | `true`  | Run `cargo build --release` + `cargo publish`          |
| `run_npm_publish`  | boolean | `true`  | Run `npm publish`                                      |
| `run_pip_build`    | boolean | `true`  | Run `python -m build` + `twine upload`                 |
| `run_go_install`   | boolean | `true`  | Run `go install ./...`                                 |

`deny.yml` accepts:

| Input               | Type   | Default           | Purpose                                              |
|---------------------|--------|------------------:|------------------------------------------------------|
| `working_directory` | string | `.`               | Path to the `Cargo.toml` root                        |
| `command`           | string | `check`           | cargo-deny subcommand (`check`, `check bans`, etc.)  |

`codeql.yml` accepts:

| Input        | Type   | Default                                          | Purpose                                  |
|--------------|--------|-------------------------------------------------:|------------------------------------------|
| `languages`  | string | `["actions","go","javascript","python"]`         | JSON array of languages to analyze       |
| `build_mode` | string | `autobuild`                                      | `none` / `autobuild` / `manual`          |

`scorecard.yml` accepts:

| Input             | Type    | Default | Purpose                                                |
|-------------------|---------|--------:|--------------------------------------------------------|
| `results_format`  | string  | `sarif` | `sarif` / `json` / `default`                           |
| `publish_results` | boolean | `true`  | Publish results to the repository Security tab         |

`secrets-scan.yml` accepts:

| Input           | Type    | Default   | Purpose                                                                |
|-----------------|---------|----------:|------------------------------------------------------------------------|
| `scan_mode`     | string  | `github`  | `github` (CLI) or `filesystem` (Docker image, offline-friendly)        |
| `only_verified` | boolean | `true`    | Only report verified (live) secrets                                    |
| `base_branch`   | string  | `main`    | Base branch for diff-scanning (filesystem mode only)                   |

`release-attestation.yml` accepts:

| Input               | Type   | Default                                            | Purpose                                            |
|---------------------|--------|---------------------------------------------------:|----------------------------------------------------|
| `working_directory` | string | `.`                                                | Path to the crate / package root                   |
| `toolchain`         | string | `stable`                                           | Rust toolchain channel                             |
| `build_command`     | string | `cargo build --release --locked --workspace --all-targets` | Shell command to build release artifacts  |
| `artifact_name`     | string | `release-artifacts`                                | Name for the uploaded artifact                     |
| `retention_days`    | number | `90`                                               | Artifact retention in days (1..90)                 |

## Usage

### As a reusable workflow in a consumer's CI

```yaml
# .github/workflows/ci.yml — in the consumer repo
name: CI
on:
  pull_request:
  push:
    branches: [main]

jobs:
  ci:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/ci.yml@main
    with:
      rust_msrv: "1.75"
      python_version: "3.11"
      go_version: "1.22"
      node_version: "20"
      run_coverage: ${{ github.event_name == 'pull_request' }}
      run_clippy: true
    secrets: inherit
```

### As a reusable workflow for releases

```yaml
# .github/workflows/release.yml — in the consumer repo
name: Release
on:
  push:
    tags: ["v*"]

jobs:
  release:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/release.yml@main
    with:
      run_cargo_build: true
      run_npm_publish: false   # this crate doesn't ship a Node pkg
      run_pip_build: false     # this crate doesn't ship a Python pkg
      run_go_install: false    # this crate doesn't ship a Go pkg
    secrets: inherit
    permissions:
      contents: write
      id-token: write
```

### As a reusable workflow for security + supply-chain

The five absorbed McpKit workflows (`deny.yml`, `codeql.yml`,
`scorecard.yml`, `secrets-scan.yml`, `release-attestation.yml`) all
follow the same `uses:` + `with:` + `secrets: inherit` pattern. Wire
them into a consumer's `.github/workflows/security.yml` like so:

```yaml
# .github/workflows/security.yml — in the consumer repo
name: Security
on:
  pull_request:
  push:
    branches: [main]

jobs:
  deny:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/deny.yml@main
    with:
      command: "check"
  codeql:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/codeql.yml@main
    with:
      languages: '["actions","go","javascript","python"]'
      build_mode: "autobuild"
    permissions:
      contents: read
      actions: read
      security-events: write
  scorecard:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/scorecard.yml@main
  secrets-scan:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/secrets-scan.yml@main
    with:
      scan_mode: "github"
      only_verified: true
    secrets: inherit
```

For release-time SLSA provenance, wire `release-attestation.yml` into
the consumer's release pipeline:

```yaml
# .github/workflows/release-attest.yml — in the consumer repo
name: Release Attestation
on:
  release:
    types: [published]

jobs:
  attest:
    uses: KooshaPari/pheno-ci-templates/.github/workflows/release-attestation.yml@main
    with:
      working_directory: "."
      build_command: "cargo build --release --locked --workspace --all-targets"
      artifact_name: "release-artifacts"
    permissions:
      contents: read
      id-token: write
      attestations: write
```

### Language auto-detect

Each language job is gated by a `hashFiles()` check against the
consumer repo's manifests:

- **Rust** — runs if `**/Cargo.toml` exists.
- **Python** — runs if `**/pyproject.toml` or `**/setup.py` exists.
- **Go** — runs if `**/go.mod` exists.
- **Node** — runs if `**/package.json` exists.

A monorepo with no Rust crate, no `package.json`, and no `go.mod` will
only run the Python job (or skip all four if it has no manifests). A
crates-only repo like `pheno-otel` will only run the Rust job.

The `summary` job aggregates the results of all four language jobs and
fails if any of them failed; it is itself skipped when none of the
manifests are detected.

## Caching

- **Rust** — caches `~/.cargo/registry`, `~/.cargo/git`, and `target/`
  keyed on `Cargo.lock` + `Cargo.toml` hash.
- **Python** — `actions/setup-python` `cache: pip` keyed on
  `requirements*.txt` / `pyproject.toml`.
- **Go** — `actions/setup-go` `cache: true` (uses `go.sum`).
- **Node** — `actions/setup-node` `cache: npm` keyed on
  `package-lock.json`.

## Permissions

- `ci.yml` requests `contents: read` only.
- `release.yml` requests `contents: write` (to create the GitHub
  release) and `id-token: write` (for OIDC-based publishing — the
  `cargo publish` and `twine upload` steps use trusted publishing where
  available and fall back to long-lived tokens otherwise).
- `deny.yml` requests `contents: read` only.
- `codeql.yml` requests `contents: read`, `security-events: write`, and
  `actions: read` at the workflow level; the job re-declares them for
  the `analyze` step.
- `scorecard.yml` requests `read-all` at the workflow level; the job
  re-declares `security-events: write`, `id-token: write`,
  `contents: read`, `actions: read` for the SARIF publish step.
- `secrets-scan.yml` requests `contents: read` only; the `github` scan
  mode requires the consumer to pass `secrets: inherit` (or an explicit
  `GITHUB_TOKEN`).
- `release-attestation.yml` requests `contents: read`, `id-token: write`,
  and `attestations: write`; the job re-declares them. The OIDC token is
  required for the SLSA generator to mint the build provenance.

## Pinning

Always pin to a tag or SHA in production, not `@main`:

```yaml
uses: KooshaPari/pheno-ci-templates/.github/workflows/ci.yml@v1.0.0
```

The first tagged release will be cut from the `v1.0.0` tag on this
template repo.

## McpKit absorption (2026-06-18)

The five security + supply-chain workflows (`deny.yml`, `codeql.yml`,
`scorecard.yml`, `secrets-scan.yml`, `release-attestation.yml`) were
absorbed from the now-archived `KooshaPari/McpKit` repo on 2026-06-18 as
part of the McpKit CI absorption (ADR-003, gap-hunt recommendation G-7
/ O-5). The original McpKit `ci.yml` was the only file that overlapped
with the existing canonical templates and was already promoted to
`ci.yml` here; the six workflows in this PR were the gap.

The two source McpKit secrets-scan files (`secrets-scan.yml` +
`trufflehog.yml`) were deduplicated into a single `secrets-scan.yml`
with a `scan_mode` input (`github` = modern CLI, `filesystem` = legacy
Docker image path). The McpKit `codeql.yml` was a passthrough to
`KooshaPari/phenoShared/.github/workflows/reusable/codeql.yml`; this
absorption inlines the CodeQL setup so consumers do not need to depend
on phenoShared.

Source: `McpKit/.github/workflows/{cargo-deny,codeql,scorecard,secrets-scan,trufflehog,release-attestation}.yml`
(archived; read-only).
Decision log: ADR-003 (McpKit MERGE → PhenoMCP) and ADR-024 (71-pillar
audit, G-7 / O-5 recommendation).
Absorption PR: see PR linked from `KooshaPari/phenotype-apps` branch
`chore/absorb-mcpkit-ci-workflows-2026-06-18`.

## Testing the template itself

Both workflows are smoke-tested by GitHub Actions on merge to `main`
(the `workflow_dispatch` triggers exist exactly for this). They are not
exercised by a local test runner; YAML validity is verified at PR time
by the standard GitHub Actions YAML linter, and structural correctness
is checked via `python3 -c "import yaml; yaml.safe_load(open('...'))"`
locally.
