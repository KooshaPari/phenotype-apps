# FocalPoint Language and Technology Hierarchy

_Authority: Phenotype scripting policy. Deviations require explicit approval and must be
documented with an inline justification comment referencing this file._

## Hierarchy

| Rank | Language / Runtime | Use case |
|---:|----|---|
| 1 | **Rust** | All core logic: domain, storage, events, rules, rewards, penalties, audit, crypto, FFI bindings, CLI tooling |
| 2 | **Swift / SwiftUI** | iOS native app shell, FamilyControls driver, Xcode project |
| 3 | **Go** | Backend micro-services (`services/`) only where async HTTP or gRPC is the dominant concern |
| 4 | **Python** | Scripting, ML/data pipelines, evaluation harnesses. Requires `uv` for isolation. Not for application hot paths. |
| 5 | **TypeScript (Bun)** | Web builder (`apps/builder`), VitePress docs site. Not for backend services. |
| 6 | **Bash** | Five-line glue only. Must include an inline justification comment. No new shell scripts for CI, hooks, sync, or codegen. |

## Exception policy

To introduce a language outside the hierarchy or swap to a lower-ranked alternative:

1. File a doc commit adding an entry to `docs/adr/` explaining the tradeoff.
2. Reference this file in the ADR and in the inline comment at the deviation site.
3. Get approval from the operator (PR review) before merging.

## Polyglot boundary rules

- Rust ↔ Swift: UniFFI only. No raw `unsafe extern "C"` without a companion safety comment.
- Rust ↔ Go: FFI via shared-library `.so`/`.dylib` or gRPC. No inline `cgo` importing Rust.
- Rust ↔ Python: PyO3 or subprocess. No `ctypes` unless PyO3 is not available.
- Any ↔ Shell: Pass runtime values via environment variables, never string interpolation into script bodies.

## Dependency governance

All Rust crates are governed by `deny.toml` (cargo-deny). Exceptions require an `[advisories]` or
`[bans.skip]` entry with a justification comment and a tracking issue URL.

For third-party libraries in any language, prefer wrapping a well-maintained OSS package over
reimplementing (Phenotype wrap-over-handroll mandate). Document the wrapped library in comments:
`// wraps: <lib-name> <version>`.
