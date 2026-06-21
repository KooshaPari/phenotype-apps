# SIDE-33: Cargo.toml hygiene audit — pheno-* family

**Date:** 2026-06-22
**Scope:** Every Cargo.toml whose `package.name` starts with `pheno-` (root + sub-crates + fuzz harness).
**Mode:** Read-only — no edits applied.
**Checks:** 7 required `[package]` fields — `license`, `repository`, `documentation`, `readme`, `keywords`, `categories`, `description`.

---

## Executive summary

| Metric | Value |
| --- | ---: |
| pheno-* Cargo.toml packages audited | **12** |
| Fully compliant (all 7 fields present) | **2** (pheno-otel, pheno-tracing) |
| Fully missing (all 7 fields absent) | **2** (pheno-port-adapter, pheno-port-adapter-fuzz) |
| Packages with `publish = true` (publish-affecting gap) | **4** (pheno-errors, pheno-otel, pheno-port-adapter, pheno-tracing) |
| `publish = true` packages with critical gaps | **2** (pheno-errors: 4 missing; pheno-port-adapter: 7 missing) |

**Fleet compliance: 17 % (2/12).** 10 of 12 pheno-* packages are missing at least 3 of the 7 required fields.

The two `publish = true` non-compliant crates are **publish-blocking**: `cargo publish` will reject them on the first metadata lint it cannot infer (e.g. missing `description` → `cargo publish` error, missing `license` / `license-file` → error).

---

## Scope-detection methodology

`grep -lE '^name\s*=\s*"pheno-' pheno-*/Cargo.toml pheno-*/*/Cargo.toml pheno-*/*/*/Cargo.toml`

Returns 12 package manifests (one root `pheno-chaos/Cargo.toml` is a `[workspace]` declaration, not a `[package]`, so it is out of scope as a crate but in scope as the workspace that sets the `license.workspace = true` default consumed by the two `pheno-chaos-*` sub-crates).

---

## Per-crate findings

Legend: `✓` present · `✗` missing · `(ws)` inherited from `[workspace.package]`.

| # | Crate | File | license | repository | documentation | readme | keywords | categories | description | Missing |
|---:|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | `pheno-port-adapter` | `pheno-port-adapter/Cargo.toml:1-5` | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | **7** |
| 2 | `pheno-port-adapter-fuzz` | `pheno-port-adapter/fuzz/Cargo.toml:1-5` | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ | **7** |
| 3 | `pheno-chaos` | `pheno-chaos/crates/pheno-chaos/Cargo.toml:2-8` | ✓ (ws) | ✗ | ✗ | ✗ | ✗ | ✗ | ✓ `Cargo.toml:8` | **5** |
| 4 | `pheno-chaos-macros` | `pheno-chaos/crates/pheno-chaos-macros/Cargo.toml:2-8` | ✓ (ws) | ✗ | ✗ | ✗ | ✗ | ✗ | ✓ `Cargo.toml:8` | **5** |
| 5 | `pheno-config` | `pheno-config/Cargo.toml:2-7` | ✓ `Cargo.toml:6` | ✗ | ✗ | ✗ | ✗ | ✗ | ✓ `Cargo.toml:5` | **5** |
| 6 | `pheno-errors` | `pheno-errors/Cargo.toml:2-7` | ✓ `Cargo.toml:5` | ✓ `Cargo.toml:6` | ✗ | ✗ | ✗ | ✗ | ✓ `Cargo.toml:7` | **4** |
| 7 | `pheno-context` | `pheno-context/Cargo.toml:2-10` | ✓ `Cargo.toml:5` | ✗ | ✗ | ✗ | ✓ `Cargo.toml:8` | ✓ `Cargo.toml:9` | ✓ `Cargo.toml:7` | **3** |
| 8 | `pheno-events` | `pheno-events/Cargo.toml:2-9` | ✓ `Cargo.toml:5` | ✗ | ✗ | ✗ | ✓ `Cargo.toml:7` | ✓ `Cargo.toml:8` | ✓ `Cargo.toml:6` | **3** |
| 9 | `pheno-flags` | `pheno-flags/Cargo.toml:3-12` | ✓ `Cargo.toml:7` | ✗ | ✗ | ✗ | ✓ `Cargo.toml:10` | ✓ `Cargo.toml:11` | ✓ `Cargo.toml:9` | **3** |
| 10 | `pheno-cli-base` | `pheno-cli-base/Cargo.toml:2-11` | ✓ `Cargo.toml:6` | ✗ | ✗ | ✓ `Cargo.toml:10` | ✓ `Cargo.toml:8` | ✓ `Cargo.toml:9` | ✓ `Cargo.toml:7` | **2** |
| 11 | `pheno-otel` | `pheno-otel/Cargo.toml:2-13` | ✓ `Cargo.toml:6` | ✓ `Cargo.toml:8` | ✓ `Cargo.toml:9` | ✓ `Cargo.toml:12` | ✓ `Cargo.toml:10` | ✓ `Cargo.toml:11` | ✓ `Cargo.toml:7` | **0** ✓ |
| 12 | `pheno-tracing` | `pheno-tracing/Cargo.toml:2-13` | ✓ `Cargo.toml:6` | ✓ `Cargo.toml:8` | ✓ `Cargo.toml:9` | ✓ `Cargo.toml:12` | ✓ `Cargo.toml:10` | ✓ `Cargo.toml:11` | ✓ `Cargo.toml:7` | **0** ✓ |

---

## Aggregate field-level view

| Field | Present | Missing | Missing % |
| --- | ---: | ---: | ---: |
| `description` | 10 | 2 | 17 % |
| `license` | 10 | 2 | 17 % |
| `keywords` | 6 | 6 | 50 % |
| `categories` | 6 | 6 | 50 % |
| `readme` | 3 | 9 | 75 % |
| `repository` | 3 | 9 | 75 % |
| `documentation` | 2 | 10 | 83 % |

**Top-3 most-missing fields:** `documentation` (10/12 missing), `repository` (9/12), `readme` (9/12).

---

## Publish-status view (impact-class ranking)

`publish = true` crates are subject to `cargo publish` linting. The 4 crates flagged `publish = true` (or unset, defaulting to true) are:

| Crate | `publish` flag | File | Missing | Publish-blocking? |
| --- | --- | --- | ---: | --- |
| `pheno-port-adapter` | unset (default `true`) | `pheno-port-adapter/Cargo.toml:1-9` | **7** | **YES** — `cargo publish` rejects on missing `description` and missing `license`/`license-file` |
| `pheno-errors` | unset (default `true`) | `pheno-errors/Cargo.toml:2-7` | **4** | **YES** — `cargo publish` rejects on missing `keywords`/`categories` (warnings → errors when `--strict`) |
| `pheno-otel` | `true` `Cargo.toml:13` | `pheno-otel/Cargo.toml:13` | **0** | no — clean |
| `pheno-tracing` | `true` `Cargo.toml:13` | `pheno-tracing/Cargo.toml:13` | **0** | no — clean |

The 8 crates with `publish = false` (`pheno-chaos`, `pheno-chaos-macros`, `pheno-cli-base`, `pheno-config`, `pheno-context`, `pheno-events`, `pheno-flags`, `pheno-port-adapter-fuzz`) have lower publish-urgency, but metadata still affects `cargo doc`, `cargo search` indexability, and ADR-023 Rule 3.1 substrate-quality-bar compliance.

---

## Crate-by-crate detail

### 1. `pheno-port-adapter` — **7 missing** (worst, publish-blocker)

`pheno-port-adapter/Cargo.toml:1-9`:
- `[package]` table starts at line 1; only `name`, `version`, `edition`, `rust-version` are present (lines 2-5).
- No `license`, `description`, `repository`, `documentation`, `readme`, `keywords`, `categories`.
- `publish` flag is unset (defaults to `true`), so `cargo publish` would reject this manifest.
- Note: line 7 contains an empty `[workspace]` table; this is a workspace marker, not metadata.

### 2. `pheno-port-adapter-fuzz` — **7 missing** (fuzz harness, lower urgency)

`pheno-port-adapter/fuzz/Cargo.toml:1-5`:
- `[package]` table only sets `name`, `version`, `publish = false`, `edition`.
- No `license`, `description`, `repository`, `documentation`, `readme`, `keywords`, `categories`.
- `publish = false` (`Cargo.toml:4`) — no publish-block.
- Lowest urgency: fuzz harnesses are throw-away dev tooling.

### 3. `pheno-chaos` — **5 missing**

`pheno-chaos/crates/pheno-chaos/Cargo.toml:1-8`:
- `license.workspace = true` (line 6) → inherited from `pheno-chaos/Cargo.toml:12` ("MIT OR Apache-2.0"). ✓
- `description` present (line 8). ✓
- Missing: `repository`, `documentation`, `readme`, `keywords`, `categories`.

### 4. `pheno-chaos-macros` — **5 missing**

`pheno-chaos/crates/pheno-chaos-macros/Cargo.toml:1-8`:
- `license.workspace = true` (line 6) → same workspace default. ✓
- `description` present (line 8). ✓
- Missing: `repository`, `documentation`, `readme`, `keywords`, `categories`.

### 5. `pheno-config` — **5 missing**

`pheno-config/Cargo.toml:2-7`:
- `license = "MIT OR Apache-2.0"` (line 6). ✓
- `description` (line 5). ✓
- Missing: `repository`, `documentation`, `readme`, `keywords`, `categories`.
- `publish = false` (line 7) — not a publish-blocker.

### 6. `pheno-errors` — **4 missing** (publish-blocker)

`pheno-errors/Cargo.toml:2-7`:
- `license = "MIT"` (line 5). ✓
- `repository = "https://github.com/KooshaPari/pheno-errors"` (line 6). ✓
- `description` (line 7). ✓
- Missing: `documentation`, `readme`, `keywords`, `categories`.
- `publish` flag unset (defaults to `true`) — cargo publish may reject missing keywords/categories under strict mode.

### 7. `pheno-context` — **3 missing**

`pheno-context/Cargo.toml:2-10`:
- `license` (line 5), `keywords` (line 8), `categories` (line 9), `description` (line 7). ✓
- Missing: `repository`, `documentation`, `readme`.
- `publish = false` (line 10) — no publish-block.

### 8. `pheno-events` — **3 missing**

`pheno-events/Cargo.toml:2-9`:
- `license` (line 5), `keywords` (line 7), `categories` (line 8), `description` (line 6). ✓
- Missing: `repository`, `documentation`, `readme`.
- `publish = false` (line 9) — no publish-block.

### 9. `pheno-flags` — **3 missing**

`pheno-flags/Cargo.toml:3-12`:
- `license` (line 7), `keywords` (line 10), `categories` (line 11), `description` (line 9). ✓
- Missing: `repository`, `documentation`, `readme`.
- `publish = false` (line 12) — no publish-block.

### 10. `pheno-cli-base` — **2 missing**

`pheno-cli-base/Cargo.toml:2-11`:
- `license` (line 6), `description` (line 7), `keywords` (line 8), `categories` (line 9), `readme = "README.md"` (line 10). ✓
- Missing: `repository`, `documentation`.
- `publish = false` (line 11) — no publish-block.

### 11. `pheno-otel` — **0 missing** ✓ fully compliant

`pheno-otel/Cargo.toml:2-13`: all 7 fields present.
- `description` line 7, `license` line 6, `repository` line 8, `documentation` line 9, `keywords` line 10, `categories` line 11, `readme` line 12.

### 12. `pheno-tracing` — **0 missing** ✓ fully compliant

`pheno-tracing/Cargo.toml:2-13`: all 7 fields present.
- `description` line 7, `license` line 6, `repository` line 8, `documentation` line 9, `keywords` line 10, `categories` line 11, `readme` line 12.

---

## Cross-cutting observations

1. **The two compliant crates (`pheno-otel`, `pheno-tracing`) are the two published-to-crates.io observability substrates.** They are also the two crates with `[[package.metadata.docs.rs]]` blocks (e.g. `pheno-tracing/Cargo.toml:56-58`), which is the strongest signal that someone ran a hygiene pass against them. The 10 others never had that pass.

2. **`pheno-port-adapter` is the largest inverse outlier.** Its sibling observability crates (pheno-otel, pheno-tracing) are clean; the hex-port substrate adjacent to them is the dirtiest. This is likely an oversight from when the crate was carved out (it has an empty `[workspace]` at `Cargo.toml:7` declaring it standalone — that block was added to suppress parent-workspace lookup, not to add metadata).

3. **Workspace-inheritance does not cover the missing fields.** `pheno-chaos/Cargo.toml:8-13` only sets `version`, `edition`, `rust-version`, `license`, `publish` in `[workspace.package]`. None of `description`, `repository`, `documentation`, `readme`, `keywords`, `categories` are inheritable defaults; every member crate must declare them itself. The two `pheno-chaos-*` sub-crates therefore have no excuse to be missing these fields — they need to be added at the member level.

4. **No crate has `homepage`, `exclude`, or `include`** — those are out of scope for this SIDE-33 audit, but worth noting for future hygiene passes.

5. **`publish = true` vs `publish = false` asymmetry.** Half the publish-flagged crates default to `true` (implicit), so a `cargo publish --dry-run` pass on the fleet would surface 2 publish-blocking crates: `pheno-port-adapter` and `pheno-errors`. Fixing those is a hard prerequisite for any future crates.io release of those two crates.

---

## Recommended remediation order (read-only audit; not applied)

If a follow-up SIDE-34 (or similar) were to land fixes, the priority order would be:

1. **`pheno-port-adapter`** — fill all 7 fields; this is a publish-blocker and a fleet substrate (consumed by pheno-errors via path-dep).
2. **`pheno-errors`** — fill `documentation`, `readme`, `keywords`, `categories` (4 fields); publish-blocker.
3. **`pheno-config`** — fill 5 fields; substrate for the v19 T2 L52 encryption work.
4. **`pheno-chaos`, `pheno-chaos-macros`** — fill 5 fields each; both inherit `license` from `[workspace.package]` already, so only 5 need adding per crate.
5. **`pheno-context`, `pheno-events`, `pheno-flags`** — add `repository`, `documentation`, `readme` to each (3 fields each).
6. **`pheno-cli-base`** — add `repository`, `documentation` (2 fields).
7. **`pheno-port-adapter-fuzz`** — fuzz harness; lowest priority; add 7 fields only if a hygiene lint is wired into CI.

---

## Reproduction commands

```bash
# Discover pheno-* packages
grep -lE '^name\s*=\s*"pheno-' \
  pheno-*/Cargo.toml \
  pheno-*/*/Cargo.toml \
  pheno-*/*/*/Cargo.toml 2>/dev/null | sort -u

# Field presence check (one file at a time)
for f in <list above>; do
  echo "=== $f ==="
  for field in license repository documentation readme keywords categories description; do
    grep -E "^${field}(\s*=|\.)" "$f" >/dev/null \
      && echo "  $field: PRESENT" \
      || echo "  $field: MISSING"
  done
done
```

---

## Source evidence

- All 12 manifest files were read in this session (2026-06-22) from `repos/` working tree.
- AGENTS.md context: project is the `KooshaPari/Phenotype` monorepo; `pheno-*` family is the canonical Rust substrate tier per ADR-022 / ADR-023 / ADR-024.
- This audit is read-only and produced no side effects beyond this single markdown file.