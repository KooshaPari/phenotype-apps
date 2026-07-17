# Tracely absorption provenance

Absorbed from `KooshaPari/Tracely` on 2026-07-17 into `phenotype-apps/apps/tracely/`.

## Source state captured

- Repo: `git@github.com:KooshaPari/Tracely.git`
- HEAD: `c1c6f1f588dff13442ba5f691e91ed40b6df0638`
- Branch (local): `wip/2026-07-16-0027-auto` (auto-daemon, not main)
- Disk size at capture: 486 MB total / 146 files source-only (852 KB of source, rest was `target/` build artifacts)
- Pre-archive tarball: `/Users/kooshapari/CodeProjects/Phenotype/repos/_archive/Tracely-2026-07-17/Tracely-source-pre-archive.tar.gz`

## What was copied into this directory

- Root governance files: `Cargo.toml`, `Cargo.lock`, `README.md`, `STATUS.md`, `AGENTS.md`, `LICENSE`, `.gitignore`, `.gitattributes`, `.editorconfig`, `rust-toolchain.toml`, `deny.toml`
- `crates/tracely-core/` — the actual Tracely observability library (`name = "tracely"`, version `0.2.0`)

## What was deliberately NOT copied

- `crates/tracely-sentinel/` — this was `phenotype-sentinel` (rate-limit / circuit breaker / bulkhead), already absorbed into `phenotype-infrakit`. It is foreign content that was mis-shelved under the Tracely workspace at the source repo. The workspace `members = [...]` entry was removed and `Cargo.toml` carries a comment to that effect.
- `crates/helix-tracing/`, `crates/pheno-logging-zig/`, `crates/zerokit/` — these were 0-byte stubs for other already-absorbed repos. Not content.
- `target/` (485 MB of build artifacts) and `.git/`.

## Build verification

`cargo check -p tracely` was executed against this workspace after the manifest edit and succeeded. Build artifact output is host-local and is not committed (see `.gitignore`).

## Registry updates

- `registry/disposition-index.json` row `id: 59` (path: `KooshaPari/Tracely`) — disposition flipped to `ABSORB`, `target` updated, `fsm: absorbed`, `note` appended.
- `catalog/registry.yaml` row `id: tracely` — `status: absorbed`, corrected `language: rust`, `notes` rewritten.

## Archive action

Outer repo `KooshaPari/Tracely` was archived via `gh repo archive KooshaPari/Tracely -y` on 2026-07-17 after the source-copy was verified.
