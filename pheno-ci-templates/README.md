# pheno-ci-templates

Canonical reusable GitHub Actions CI + release workflows for the **pheno-\***
fleet. One source of truth for the four languages the fleet ships in
(Rust, Python, Go, Node); each consumer pulls the workflows in via
`uses:` and only ever sees the language jobs that match the manifests
present in the consumer repo.

The two workflows are:

- **`ci.yml`** — runs on `pull_request` / `push` (via the caller's
  trigger) and at PR time checks fmt + clippy + test + optional coverage
  for every detected language. The `workflow_call` trigger is paired with
  `workflow_dispatch` so the template can be smoke-tested from the
  Actions tab without any caller.
- **`release.yml`** — runs on tag push (`v*` or `release/*`) and creates
  a GitHub release, then publishes artifacts to crates.io / npm / PyPI /
  $GOBIN. Each publish step is gated by a `run_<lang>` boolean so a
  single tag can release only the languages the calling crate owns.

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

## Pinning

Always pin to a tag or SHA in production, not `@main`:

```yaml
uses: KooshaPari/pheno-ci-templates/.github/workflows/ci.yml@v1.0.0
```

The first tagged release will be cut from the `v1.0.0` tag on this
template repo.

## Testing the template itself

Both workflows are smoke-tested by GitHub Actions on merge to `main`
(the `workflow_dispatch` triggers exist exactly for this). They are not
exercised by a local test runner; YAML validity is verified at PR time
by the standard GitHub Actions YAML linter, and structural correctness
is checked via `python3 -c "import yaml; yaml.safe_load(open('...'))"`
locally.
