# ADR-019: `phenoVessel` deprecation — complete (consumed by `pheno-tracing` + `phenoVessel` substrate)

**Status:** Accepted 2026-06-15
**Deciders:** PhenoRust 1.0 architecture circle
**Supersedes:** ad-hoc `phenoVessel` re-exports as a public API crate

## Context

`phenoVessel` was published in V4 as a "sandbox + workspace + ipc"
meta-crate. The V5 audit found that the public surface (12 ports,
9 adapters) was not actually used outside of `pheno-mcp`'s
`SandboxServer` and `agileplus-cli`'s `sandbox run`.

The V6 work (Track 3 — PRCP pattern) shows the correct substrate
is a smaller L4 port collection plus the existing `pheno-tracing`
for observability:

1. The 9 `phenoVessel` adapters collapse to 2 (`LocalExecAdapter`
   + `DockerAdapter`), because the Firecracker / gVisor /
   landlock code is now an `agileplus-factory` runtime concern,
   not a port.
2. The 12 `phenoVessel` ports collapse to 4 (`Exec`,
   `Filesystem`, `Network`, `Clock`) — the Hexagonal L4 pattern
   in ADR-014.
3. The "vessel" name was overloaded (Sandbox, Container, VM,
   Workspace) and removed by 2026-06-14.

## Decision

`phenoVessel` is **deprecated as a public crate** and rolled
into the `phenoVessel` substrate crate under the new hex-L4
naming. The migration:

1. The 4 ports and 2 adapters are published as
   `pheno-vessel-core` (a 600-LOC crate).
2. The 9 adapter re-exports and 8 unused ports are removed.
3. The 2 consumers (`pheno-mcp`, `agileplus-cli`) update their
   `Cargo.toml` to depend on `pheno-vessel-core` directly.
4. The old `phenoVessel` crate is marked `deprecated`,
   `publish = false`, and re-exports the 4 ports from
   `pheno-vessel-core` for 1 minor version as a transition.

## Consequences

**Positive**

- The 4-port/2-adapter shape is what every consumer actually
  needs; 2,400 LOC of speculative APIs are removed.
- The hexagonal L4 pattern (ADR-014) is the actual abstraction
  the org should have shipped in V4.
- `pheno-mcp` and `agileplus-cli` lose a 9-port surface they
  never used; the 4 ports they do use become first-class.

**Negative**

- The 1-minor-version deprecation cycle has to be published
  with a `cargo publish --dry-run` and a CHANGELOG entry per
  consumer.
- 3 archived `phenoVessel::Foo` re-exports get 410s.

**Mitigation**

- The deprecation cycle is 1 minor version (4 weeks), not a
  hard break; consumers have time to migrate.
- A redirect to the new `pheno-vessel-core` docs is published
  before the deprecation ships.

## Alternatives considered

- **Keep `phenoVessel` as the public surface; just add
  `pheno-vessel-core` as an internal split.** Rejected: the
  speculative APIs are net-negative; the right move is to
  delete, not hide.
- **Inline the 4 ports into `pheno-mcp` and `agileplus-cli`
  directly.** Rejected: that regresses to pre-V4 (every
  consumer has its own port trait, and we re-learn the
  lesson from V3).

## References

- `phenoVessel/Cargo.toml` (deprecated 2026-06-15).
- `pheno-vessel-core/src/ports.rs` — 4 ports.
- `pheno-vessel-core/src/adapters/local.rs` — `LocalExecAdapter`.
- `pheno-vessel-core/src/adapters/docker.rs` — `DockerAdapter`.
- `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md` — consumer migration.
- `V6 §Track 3` — PRCP work item: `phenoVessel` → `pheno-vessel-core`.
