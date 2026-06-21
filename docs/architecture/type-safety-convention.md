# Type-Safety Convention (Fleet-Wide)

> **Status**: ACTIVE (v17 Wave C, T8 deliverable) — first adopted 2026-06-21
> **Owner**: worklog-schema circle
> **Scope**: All 11 surviving `pheno-*` Rust crates + new pheno-* additions
> **Enforcement**: `deny(missing_docs)` + `deny(unsafe_code)` per crate, gated by CI

---

## 1. Purpose

Document the **type-safety invariants** that every pheno-* crate must hold, the
**lint set** that enforces them, and the **remediation process** when a new
build fails. This is the v17 cycle 7 P0 deliverable that surfaces documentation
gaps and unsafe code at compile time, before they reach code review.

## 2. The Four Invariants

### Invariant 1 — **No unsafe code**

```rust
#![deny(unsafe_code)]
```

Rationale: every pheno-* lib is a single-concern substrate. They compose via
traits (`Port`, `Adapter`, `Service`), not via memory tricks. Unsafe is a
code smell that signals the layer is too low-level. If you need unsafe, the
abstraction is wrong — propose a higher-level primitive.

**Exception process**: file a `// SAFETY: <comment>` block + ADR. The ADR
review board (per ADR-023) must approve before the deny can be relaxed for
that crate. No exceptions are baked in by default.

### Invariant 2 — **No undocumented public items**

```rust
#![deny(missing_docs)]
```

Rationale: a public item without a `///` doc comment is a footgun. The
fleet's consumer crates (dispatch-mcp, PhenoCompose, PlayCua, BytePort,
AgilePlus, thegent, Tokn, etc.) consume pheno-* APIs without source access
to the lib. A missing doc is a runtime surprise.

**Coverage standard**: every `pub` item — fn, struct, enum, trait, type
alias, const, static, mod — must have a `///` doc comment. The first line
is a single-sentence summary. The body explains the contract: preconditions,
postconditions, error modes, thread-safety.

### Invariant 3 — **Errors are `Send + Sync + 'static`**

```rust
pub trait AppError: std::error::Error + Send + Sync + 'static {
    fn kind(&self) -> ErrorKind;
    fn http_status(&self) -> u16;
}
```

Rationale: pheno-* errors flow through the L4 hexagonal port surface and
must cross thread boundaries (axum handlers → tracing spans → worklog
emitters). A non-Send error deadlocks the runtime.

**Implementation pattern**: every `AppError` variant has `#[derive(thiserror::Error)]`
and a `From` impl for the underlying source. No `Arc<Mutex<...>>` in error
chains (causes display/debug asymmetry).

### Invariant 4 — **Ports are object-safe + `Send + Sync`**

```rust
pub trait Port: Send + Sync {
    type Request: Send + 'static;
    type Response: Send + 'static;
    type Error: AppError;
    async fn call(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
}
```

Rationale: every pheno-* lib exposes its capability as a `Port` trait (per
ADR-014 hexagonal policy). The trait must be object-safe so consumers can
swap implementations via `Arc<dyn Port>` in dependency injection. `Send + Sync`
is required for the same reason as Invariant 3.

**Anti-pattern**: trait methods that return `Self` by value, generic methods
without `where Self: Sized`, methods with non-`Send` parameters.

## 3. Lint Set (applied at the crate root)

Every pheno-* lib.rs starts with the same 4-line lint block:

```rust
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
```

`rust_2018_idioms` catches the most common lint group violations
(non-`async` async fn, elided lifetimes in pub items, etc.).
`missing_debug_implementations` is a *warning* (not deny) because some
newtype wrappers intentionally skip Debug to avoid leaking PII.

## 4. The Three Crates (v17 P0 scope)

The first three crates to receive the deny block:

| Crate | Lines | Public items | T8 status |
|-------|-----:|---------------|-----------|
| **pheno-port-adapter** | 195 | `enum AdapterError`, `struct Connection`, `trait PortAdapter`, 3 re-exports | pending |
| **pheno-errors** | 392 | `mod rfc7807`, `enum AppError` + 8 variants, 4 `From` impls | pending |
| **pheno-tracing** | 100 | 4 `pub mod`, ~15 re-exports | pending |

**Implementation plan** (v17 Wave C, T8):
1. Add the 4-line lint block to all 3 crates
2. Run `cargo check` on each — collect the new errors (this is the doc-gap list)
3. Add the missing `///` comments (one commit per crate, ~15 min each)
4. Verify with `cargo test` (must remain green)
5. Tag each crate as `v0.2.0` (the deny is a breaking-API change semver-wise)

## 5. CI Enforcement

Per ADR-049 (drift detector), every pheno-* lib's lint block is verified at
PR time:

```yaml
# .github/workflows/lint-pheno.yml
- name: deny(missing_docs) check
  run: |
    for crate in pheno-*/; do
      if [ -f "$crate/src/lib.rs" ]; then
        grep -q "#!\[deny(missing_docs)\]" "$crate/src/lib.rs" || {
          echo "::error::$crate/src/lib.rs is missing deny(missing_docs)"
          exit 1
        }
      fi
    done
```

This is the same shape as the `pheno-vibecoding-guard` hook (per V13) but
enforced at the monorepo root, not per repo.

## 6. Remediation Process

When a new crate is added without the deny block:

1. **PR is auto-flagged** by `lint-pheno.yml`
2. **Author adds the 4-line block** (one-line commit)
3. **cargo check surfaces doc gaps** (this is the *point* — you can't merge
   a new public item without a doc)
4. **Author writes the docs** in a follow-up commit
5. **CI green, ready to merge**

For an *existing* crate (the 11 surviving pheno-* libs), the migration is
staged:

- **v17 Wave C (this week)**: 3 critical crates (pheno-port-adapter,
  pheno-errors, pheno-tracing)
- **v18 Wave A**: 4 SDK-promotion candidates (pheno-cli-base,
  pheno-config, pheno-context, pheno-flags)
- **v18 Wave B**: 4 remaining (pheno-agents-md, pheno-cli-base-revived,
  pheno-fastapi-base, pheno-otel)

This staging matches the SDK graduation path from ADR-048. A crate moves to
SDK tier only after it passes the deny-block test.

## 7. Migration Helper Script

For the 11 surviving crates, the migration is mechanical. The script
`scripts/pheno-adopt-deny.sh` (per ADR-049 drift detector) automates it:

```bash
#!/usr/bin/env bash
# Add the 4-line deny block to any pheno-* lib.rs that doesn't have it
for crate in pheno-*/; do
  if [ -f "$crate/src/lib.rs" ] && ! head -5 "$crate/src/lib.rs" | grep -q "deny(missing_docs)"; then
    tmp=$(mktemp)
    {
      echo "#![deny(unsafe_code)]"
      echo "#![deny(missing_docs)]"
      echo "#![deny(rust_2018_idioms)]"
      echo "#![warn(missing_debug_implementations)]"
      echo ""
      cat "$crate/src/lib.rs"
    } > "$tmp"
    mv "$tmp" "$crate/src/lib.rs"
    echo "  ✓ $crate/src/lib.rs: deny block added"
  fi
done
```

This script is idempotent (checks for the deny block first) and safe to
run repeatedly.

## 8. Open Questions (deferred to v18+)

1. **Should the deny apply to `pub(crate)` items too?** — Current draft
   only catches `pub`. `pub(crate)` is fine without docs because it's an
   implementation detail. No change planned.
2. **Should doc-tests be required?** — `deny(missing_docs)` doesn't
   enforce doc-tests. A future `deny(invalid_doc_comments)` or
   `missing_doc_code_examples` could enforce them, but that's a v18+ ADR.
3. **Workspace-level lints vs crate-level?** — The lint block is per-crate
   (not workspace). A workspace-level `lints.rs` would force all crates
   to use the same set, but it can't be enforced for sub-crates that
   opt out via `[lints] workspace = "..."` (the inverse direction). Per-crate
   is the right choice for the federated fleet.

## 9. References

- **ADR-014**: Hexagonal port + adapter (L4)
- **ADR-022**: Configra substrate (L1)
- **ADR-023**: Agent effort governance (worklog + crutches)
- **ADR-040**: Observability substrate (pheno-otel)
- **ADR-048**: Substrate graduation path (4-tier gate)
- **ADR-049**: App-substrate drift detector
- **FLEET_100TASK_DAG_V4 §70.3**: AX/L16 acceptance (the 7 AI-DD crutches)
- **PERMISSIONS.md**: subagent process + repo safety rules

---

**This document is itself a v17 Wave C T8 deliverable.** Sign-off requires:
- 3 surviving crates migrated to the deny block (this turn)
- All migration commits green in CI
- v0.2.0 release tags cut on the 3 crates
