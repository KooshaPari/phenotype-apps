# ADR-017: Settly archive — supersede by phenoVessel + ADR-013

**Status:** Accepted 2026-06-15
**Deciders:** PhenoRust 1.0 architecture circle
**Supersedes:** ad-hoc `settly` re-exports in 3+ pheno-* repos

## Context

`settly` was a 1× "settlement + idempotency key" helper crate promoted to
phenotype-org during V3. The intended consumer was the `agileplus-factory`
queue (claim → worktree → trail → PR) and a few `pheno-*` Rust services
that needed at-least-once / exactly-once semantics.

Audit (2026-06-15) showed:

- The "settlement" half was never implemented (the 2 stub methods were
  `unimplemented!()` in the 1.0.0 release).
- The "idempotency key" half was 80 LOC of HMAC + content-hash; 4 of 5
  consumers re-implemented the same logic inline because the crate's
  API didn't match their `async fn` shape.
- Maintenance was $0 (no commits in 14 months), but the namespace was
  blocking two planned forks.

## Decision

Archive `settly`. The two halves are replaced as follows:

1. **Settlement / claim**: consumed by `phenoVessel` (L4 substrate, the
   `Settlement` port from the Hexagonal L4 ports pattern in ADR-014).
   `phenoVessel` ships a `SettlementAdapter` that wraps the existing
   `agileplus-factory` queue's claim-lifecycle code (it already does
   the job; the abstraction is what was missing).
2. **Idempotency key**: 80 LOC of `phenoVessel::idempotency` (the
   HMAC + content-hash helper, lifted out and tested standalone). All
   4 inline copies can migrate with a 1-line `use` change.

## Consequences

**Positive**

- Removes a dead 1.0.0 crate from the public API surface.
- Two planned forks (`pheno-mcp`'s claim-watcher + `pheno-sandbox`'s
  container-claim) can proceed now.
- L4 substrate (`phenoVessel`) gets the `Settlement` port it was
  missing for the v6 worklog-v2 schema (ADR-015).

**Negative**

- 3 downstream consumers need a 1-line `use` change:
  `use settle::IdempotencyKey;` → `use phenovessel::idempotency::Key;`.
- `settly`'s published docs URL becomes a 410; a redirect is published.

**Mitigation**

- The redirect is published before the archive PR merges.
- The migration is mechanical and tracked per consumer in
  `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md`.

## Alternatives considered

- **Resurrect `settly` and implement the stubs.** Rejected: 14 months
  without a consumer proves the API is wrong; the right fix is
  re-examine the problem, not the implementation.
- **Fork `settly` and add the missing methods.** Rejected: violates
  the fork-only-not-rewrite policy (ADR-016); the consumers want
  `async`, not blocking.
- **Leave it as-is.** Rejected: it occupies a public namespace and
  blocks forks; zero value in retention.

## References

- `settly/Cargo.toml` (1.0.0, archived 2026-06-15).
- `phenoVessel/src/settlement.rs` — new `Settlement` port + adapter.
- `phenoVessel/src/idempotency.rs` — HMAC + content-hash helper, 80 LOC.
- `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md` — consumer migration table.
- `V6 §Track 3` — V6 plan, Settly-archive work item.
