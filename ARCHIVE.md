# Archive Registry

Effective 2026-07-05, the following subprojects in this monorepo are
**STRICT PAUSE — DO NOT UNPAUSE**. The archive banner has been prepended
to each repo's README. Issues and PRs will be closed without action.

## Archived subprojects

| Repo | Description | Banner effective | Unpausing-risk |
| --- | --- | --- | --- |
| AtomsBot | original (1st) | 2026-07-05 | LOW |
| AtomsBot-2nd | iteration 2 | 2026-07-05 | LOW |
| AtomsBot-3rd | iteration 3 | 2026-07-05 | LOW |
| AtomsBot-4th | iteration 4 | 2026-07-05 | LOW |
| AtomsBot-5th | iteration 5 (final) | 2026-07-05 | LOW |
| GDK | Graph/DAG Kernel | 2026-07-05 | LOW |
| KaskMan | Kafka-cascade orchestration manager | 2026-07-05 | LOW |

## Strict pause rules (every archived repo)

1. **No new commits.** The repo is read-only at the meta level.
2. **No new PRs.** Open PRs may be merged only if they are a security
   fix; otherwise close as `not planned`.
3. **No agent work.** No agent should pick this subproject up from a
   phenodag queue entry, a stale worktree, or a re-activation trigger.
   If dispatched here, refuse and report back to the root manager.
4. **No dependabot / kwatch / auto-resume.** No automatic system may
   bring this subproject back. If such a system is doing so, that
   system is misconfigured and should be patched at the source.
5. **Banner is permanent.** The `# ARCHIVED — DO NOT UNPAUSE` banner
   at the top of each README is the source of truth. Do not remove it.
6. **If you are a human:** do not open a new PR. Open a discussion in
   the polyrepo strategy session, not a code change. Revivals must be
   approved by the portfolio owner with a written rationale in the
   session docs.
7. **If you are an agent:** report the dispatch back to root with the
   polyrepo portfolio session reference.

## Rationale

These subprojects were superseded by a more general architecture (the
phenodag/Tracera/AgilePlus stack and the central AuthKit auth boundary).
The Phenotype portfolio strategy session on 2026-07-05 decided to stop
maintaining them in parallel and freeze them with a strict-pause banner
to prevent auto-resume by dependabot, kwatch, or worktree re-activation.

The archive is a security/stability/strategic decision, not abandonment.
Code and history are preserved; only maintenance is paused.

## Sibling absorptions (NOT archived, but being absorbed)

- `phenodag` (and `phenodag-tool`) -> being absorbed into `Tracera` and
  `AgilePlus` (see phenodag FR map and spec-level deltas in the polyrepo
  portfolio strategy session). The thin-redirector will be shipped for
  one release before the source is fully archived.
- `Authvault` (in this monorepo) -> being absorbed into `AuthKit`
  (D1 KEEP AuthKit, DELETE Authvault). PR-C handles the deprecation
  banner in Authvault's README and the GitHub Archived flag.

## Refs

- Session: `docs/sessions/2026-07-05-polyrepo-portfolio-strategy/`
- Plan: `04-plans/02-phenotype-apps-spine.md`
- Master synthesis: `00_MASTER_SYNTHESIS.md`
