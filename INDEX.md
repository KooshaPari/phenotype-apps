# Phenotype-Apps Index

phenotype-apps is the **apps spine** of the KooshaPari phenotype ecosystem: the
top-level coordination point for the product-class apps under one polyrepo. It
is NOT a service, NOT a library, NOT a runtime. It is the index of every
canonical app, every archived app, and every spine the apps depend on or feed.

## Spine role (this repo)

- Index of every sub-repo with: name, role, last commit, archive status.
- Canonical home for cross-repo app specs, ADRs, and the apps-side policy.
- Single source of truth for app-level portfolio decisions.
- The place every agent goes to orient, not to execute.

## Active apps (canonical, in this monorepo)

| App | Role | Status |
| --- | --- | --- |
| AgilePlus | agile/workflow spine | active |
| AuthKit | Rust auth boundary (canonical successor to Authvault) | active |
| BytePort | surface platform (Surface 100% target) | active (separate owner) |
| Conft | config loader | active |
| Eidolon | observability | active |
| phenodag-tool | DAG/workflow scaffolding (DEPRECATED, see ARCHIVE) | frozen |
| Tracera | tracing/trace spine | active |
| Agentora | agent orchestration | active |
| Civis | civil/UX | active |
| AppGen | app generation | active |
| Apisync | API sync | active |
| agslag-docs | docs spine | active |
| apps | apps meta-pkg | active |
| AuthKit | canonical Rust auth (D1 keep) | active |
| BytePort | surface platform | active (separate owner) |

(See `git log --oneline -1 <app>/` for the latest activity in each.)

## Archived apps (do not resume; see ARCHIVE.md)

AtomsBot, AtomsBot-2nd, AtomsBot-3rd, AtomsBot-4th, AtomsBot-5th, GDK, KaskMan

## Sibling spines (cross-repo, not in this monorepo)

- `phenotype-org-audits` (separate repo) — org-audit spine
- `OmniRoute` (separate repo) — model router (out of root scope)
- `BytePort` (in this monorepo) — surface platform (out of root scope)

## Conventions

- Sub-repos here are pointers, not source of cross-repo contracts.
- Spine-level changes (this INDEX.md, ARCHIVE.md, README spine mission line)
  require a PR against `apps-extract` or `main` with sponsor sign-off.
- App-level changes live in the app's own subdir and do NOT need spine review.
- Archived apps: see ARCHIVE.md for the strict-pause rules. They are not
  removed from the monorepo; they are frozen with a permanent banner.

## When to update this file

- An app is added/archived/renamed.
- The role of an app changes.
- A sibling spine is added/removed.

