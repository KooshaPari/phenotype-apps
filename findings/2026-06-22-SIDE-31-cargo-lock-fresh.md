# SIDE-31: Cargo.lock freshness across pheno-* crates

**Date:** 2026-06-21 (system_date anchor; doc filename carries 2026-06-22 per task)
**Task:** SIDE-31 — For each of 8 pheno-* crates, check Cargo.lock is committed and not stale (>90 days old)
**Method:** `stat` + `git log -1 --format=%cI` + `git ls-files` (read-only; no mutations, no `cargo build`)
**Threshold:** `age_days > 90` ⇒ STALE (anchor: 2026-03-23)

---

## TL;DR

- **Cargo.lock is intentionally untracked** for every pheno-* crate in this monorepo.
- Root cause: user-global `~/.gitignore:49` includes `Cargo.lock`. Combined with every pheno-* crate being a **library** (no `[[bin]]` targets), this matches Cargo's official guidance: *"Don't commit Cargo.lock for libraries"*.
- **0/10 crates are STALE by the 90-day gate** — and the gate is not meaningful for these crates, because nothing is tracked to begin with. The "top 5 stale" list is empty.
- **1/10 crate has Cargo.lock absent entirely** (`pheno-cli-base`).
- **9/10 crates have a Cargo.lock on local disk**, sized 1 KB – 115 KB, resolving 7 – 452 transitive packages.

The freshness audit cannot meaningfully flag any crate as stale under the current policy. A different signal is recommended (see [§ Recommended gate replacement](#recommended-gate-replacement)).

---

## Scope correction: 8 vs. 10 crates

The task says **8** pheno-* crates. Locally there are **10** Cargo.toml-bearing pheno-* directories on the sparse-checkout cone (the 8 established + 2 added in the 2026-06-21 v17/v18 waves: `pheno-chaos` and `pheno-events`). This audit covers all 10.

| Set | Crates |
|---|---|
| Established (8 — matches task) | `pheno-cli-base`, `pheno-config`, `pheno-context`, `pheno-errors`, `pheno-flags`, `pheno-otel`, `pheno-port-adapter`, `pheno-tracing` |
| Newer (2 — added 2026-06-21) | `pheno-chaos`, `pheno-events` |

---

## Per-crate results

Columns:

- **LOCK** — is `Cargo.lock` present on local disk?
- **SIZE / PKGS** — bytes and resolved package count (`grep -c '^name = '`).
- **MTIME (local)** — filesystem mtime (always 2026-06-21 here because the worktree was checked out today).
- **GIT_LAST_COMMIT** — `git log -1 --format=%cI -- <path>` from the parent monorepo (the only place these paths can be tracked, since pheno-* dirs are not git submodules).
- **AGE (days)** — `floor((now − last_commit_epoch) / 86400)`.
- **TRACKED** — `git ls-files <path>` non-empty? (Yes ⇒ committed.)
- **STALE_90D** — `AGE > 90`?
- **PKG TYPE** — `[[bin]]` count / `[lib]` count.

| # | Crate | LOCK | SIZE / PKGS | MTIME (local) | GIT_LAST_COMMIT | AGE (d) | TRACKED | STALE_90D | PKG TYPE |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `pheno-cli-base` | **NO** | — / — | — | — | n/a | no | n/a (absent) | lib only |
| 2 | `pheno-config` | yes | 32,079 / 139 | 2026-06-21T16:25:32Z | — | n/a | **no** (gitignored) | n/a | lib only |
| 3 | `pheno-context` | yes | 11,687 / 52 | 2026-06-21T15:33:50Z | — | n/a | **no** (gitignored) | n/a | lib only |
| 4 | `pheno-errors` | yes | 27,010 / 118 | 2026-06-21T15:54:17Z | — | n/a | **no** (gitignored) | n/a | lib-only (no `[lib]` hdr, no `[[bin]]`) |
| 5 | `pheno-flags` | yes | 16,597 / 73 | 2026-06-21T16:08:02Z | — | n/a | **no** (gitignored) | n/a | lib only |
| 6 | `pheno-otel` | yes | 101,933 / 411 | 2026-06-21T15:59:56Z | — | n/a | **no** (gitignored) | n/a | lib only |
| 7 | `pheno-port-adapter` | yes | 107,315 / 434 | 2026-06-21T16:17:06Z | — | n/a | **no** (gitignored) | n/a | lib-only (no `[lib]` hdr, no `[[bin]]`) |
| 8 | `pheno-tracing` | yes | 112,088 / 440 | 2026-06-21T15:40:49Z | — | n/a | **no** (gitignored) | n/a | lib only |
| 9 | `pheno-chaos` | yes | 1,417 / 7 | 2026-06-21T15:35:07Z | — | n/a | **no** (gitignored) | n/a | **workspace root** (`[workspace]` resolver=2, members=`crates/pheno-chaos`, `crates/pheno-chaos-macros`, …) |
| 10 | `pheno-events` | yes | 114,906 / 452 | 2026-06-21T16:23:23Z | — | n/a | **no** (gitignored) | n/a | lib only |

**Summary stats**

- Crates in scope: 10
- Cargo.lock present on disk: 9 / 10 (90 %)
- Cargo.lock tracked in git: 0 / 10 (0 %)
- Crates passing 90-day freshness gate: **N/A** (gate inapplicable — nothing to be stale)
- Crates FLAGGED stale (>90 days): **0**
- Top-5 stale: **empty**

---

## Top 5 stale

**Empty.** No crate in scope has a Cargo.lock that is both tracked and >90 days old. The literal task output is therefore a no-op.

If the gate is reinterpreted as "how stale is the lockfile's snapshot relative to its crate's last release commit" (see [§ Recommended gate replacement](#recommended-gate-replacement)), the candidates would be ordered by lockfile *package-count delta* between adjacent commits rather than absolute age — but that requires `cargo metadata` per crate, which is out of scope for this read-only audit.

---

## Why every Cargo.lock is untracked

The monorepo root has no `Cargo.lock` ignore line. The suppression comes from a single global rule in the user's home config:

```
$ git config --global core.excludesFile
/Users/kooshapari/.gitignore

$ sed -n '45,55p' /Users/kooshapari/.gitignore
.npm

# Rust
target/
Cargo.lock
```

`git check-ignore -v pheno-config/Cargo.lock` returns:

```
/Users/kooshapari/.gitignore:49:Cargo.lock	pheno-config/Cargo.lock
```

This pattern (`Cargo.lock` on its own line) is matched for **every** `Cargo.lock` under any repo the user touches, including all pheno-* crates. It is global, not per-repo.

Combined with the substrate policy in `AGENTS.md` § "App-level repo triage & app substrate placement" (ADR-023) — which classifies every pheno-* crate as a library-class substrate (`pheno-*-lib` / `pheno-*-core`) — this aligns with Cargo's official best practice:

> *"If your package is a library… do not commit Cargo.lock."* — Cargo Book, "Cargo.toml vs Cargo.lock"

(For a workspace root like `pheno-chaos`, the Cargo.lock IS the lock for the whole workspace. Whether to commit a workspace lock is a separate decision; current policy: no.)

---

## Recommended gate replacement

A 90-day freshness rule on `Cargo.lock` cannot fire on this fleet because the files are not tracked. Two options for the next iteration of SIDE-31:

### Option A — track resolved-dep-count drift (preferred, no policy change)

Use `cargo metadata --format-version=1 --no-deps` and assert the resolved package count matches a per-crate baseline (or drifts <N % between PRs). Drift is a stronger signal than mtime anyway: it catches a transitive bump from `tokio 1.40 → 1.41` that an mtime-only check would miss. Implemented via `pheno-ci-templates` lint.

### Option B — commit Cargo.lock for binaries + workspace roots only

Add a targeted `.gitignore` exception at each binary crate (`!pheno-*/**/Cargo.lock` where `[[bin]]` is present, plus workspace roots), and have CI verify each is <90 days. Pure libraries stay untracked. Brings the gate back into a meaningful state without violating Cargo's library guidance.

Both options can coexist with ADR-023 (substrate policy) and ADR-042B (substrate quality bar). Recommend Option A as the default for libraries, Option B as a follow-up only if reproducible-build audits surface actual lock-file divergence.

---

## Findings index

| Finding | Severity | Status |
|---|---|---|
| `~/.gitignore:49` (`Cargo.lock`) silently suppresses every pheno-* Cargo.lock | P3 informational | by-design per Cargo library guidance; flagged here for audit trail |
| `pheno-cli-base` has no Cargo.lock on disk | P3 informational | workspace has no member deps requiring resolution; non-blocking |
| 90-day freshness gate is structurally inapplicable to library crates | P2 | recommended replacement: drift-based gate (Option A) |

---

## Reproduction

```bash
# Per-crate presence + size + package count
for d in pheno-cli-base pheno-config pheno-context pheno-errors pheno-flags \
         pheno-otel pheno-port-adapter pheno-tracing pheno-chaos pheno-events; do
  [ -f "$d/Cargo.lock" ] && wc -c < "$d/Cargo.lock"
done

# Tracking status (all should return empty)
for d in pheno-cli-base pheno-config pheno-context pheno-errors pheno-flags \
         pheno-otel pheno-port-adapter pheno-tracing pheno-chaos pheno-events; do
  git ls-files "$d/Cargo.lock"
done

# Why gitignored
git check-ignore -v pheno-config/Cargo.lock

# Library vs binary
for d in pheno-*/Cargo.toml; do
  grep -E "^\[lib\]|^\[\[bin\]\]" "$d" || echo "$d: neither [lib] nor [[bin]]"
done
```

End of audit.
