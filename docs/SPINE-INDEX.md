# Phenotype Apps -- Spine INDEX

> Spine role locked 2026-07-05. Master index of the apps catalog spine.

## What this spine is

`phenotype-apps` is the **apps catalog spine** for the Phenotype polyrepo.
It is a meta-portfolio of sub-repos -- a directory of pointers to the
canonical app repos. It does NOT host production code.

## Sub-project status

| Sub-project | Status | Notes |
|---|---|---|
| AtomsBot | ARCHIVED (strict pause) | 2026-07-05 |
| AtomsBot-2nd | ARCHIVED (strict pause) | 2026-07-05; had Discord<->GitHub bridge content |
| AtomsBot-3rd | ARCHIVED (strict pause) | 2026-07-05; empty placeholder |
| AtomsBot-4th | ARCHIVED (strict pause) | 2026-07-05; empty placeholder |
| AtomsBot-5th | ARCHIVED (strict pause) | 2026-07-05; empty placeholder |
| GDK | ARCHIVED (strict pause) | 2026-07-05 |
| KaskMan | ARCHIVED (strict pause) | 2026-07-05 |

See `docs/ARCHIVE.md` for the full list with archive reasons.

## Active sub-projects (representative)

- AuthKit (canonical Rust auth boundary, KEEP per D1)
- Authvault (archived 2026-07-05 per D1)
- BytePort (deployment platform; out of root scope; owned by BytePort team)
- OmniRoute (AI gateway; out of root scope; owned by OmniRoute team)
- AgilePlus, Tracera, substrate, pheno, phenotype-infra (other spines)
- 300+ other apps, libraries, experiments

## Recent activity

- v28 cycle-18 T1 (alertmanager latency channel) (#149)
- v27 cycle-17 T2 (ADR-103 latency-budget-to-CI) (#148)
- v25 cycle-15 T2 (L39 adopt clap-ext for bins) (#145)
- 100+ pillar org audit template (L0..L122) (#73)
- See commit log for the full activity stream.

## Open questions for the sponsor

- apps-extract branch is in flight (R-C). Coordinate before merging the
  spine charter.
- See `docs/sessions/2026-07-05-polyrepo-portfolio-strategy/00_MASTER_SYNTHESIS.md`
  for the live set.
