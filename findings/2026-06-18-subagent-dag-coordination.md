# Subagent DAG Coordination — 2026-06-18

Coordination file for parallel subagent dispatches. Append status blocks
as you claim/complete work so other agents can avoid stomping each other.

---

## Slot: phenotype-manifest CLI build (clean-state subagent)

**Claimed at:** 2026-06-18 (PDT)
**Agent ID:** forge (clean-state, dispatched by main orchestrator)
**Repo path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-ops/tools/phenotype-manifest/`
**Scope:** Fix all source files in `src/` to compile against current
deps, then `cargo build --release` and test the binary.

### Files claimed (write-locked)
- `src/crypto.rs`
- `src/generate.rs`
- `src/schema.rs`
- `src/pillar.rs` (verify only)
- `src/main.rs` (auxiliary fixes for compile errors)
- `src/verify.rs` (auxiliary: rename `verify_manifest_cmd` vs `verify_manifest` mismatch)

### What I'm fixing
1. `crypto.rs` — ed25519-dalek 2.0 API: `SigningKey::from_bytes` returns `SigningKey` (not Result), `SigningKey::generate` removed, use `OsRng.fill_bytes` + `from_bytes`. Replace `try_into()?` double-`?` with `copy_from_slice`. Use `pem::parse(&content)` not `pem::parse(content)`.
2. `generate.rs` — inline tilde expand (no `shellexpand` dep), `head.tree_id()` returns `Oid` not `Result`, add `use std::fs`.
3. `schema.rs` — `jsonschema::Validator` not `JSONSchema`, `jsonschema::options().with_draft(...).build(...)` not `.compile(...)`, `validate()` returns `Result<(), Box<ValidationError>>` (not `iter_errors`).
4. `pillar.rs` — verify `regex` import works (dep is in Cargo.toml).
5. `main.rs` — fix `lefthook_dstook_dst` typo → `lefthook_dst`, replace `SigningKey::generate` with `from_bytes(OsRng.fill_bytes)`, inline tilde expand.

### Status (append when done)
- [ ] crypto.rs fixed
- [ ] generate.rs fixed
- [ ] schema.rs fixed
- [ ] main.rs fixed
- [ ] cargo check passes
- [ ] cargo build --release passes
- [ ] binary tested (--help, init --help)
- [ ] completion status appended
