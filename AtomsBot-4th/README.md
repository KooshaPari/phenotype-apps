# ARCHIVED -- DO NOT UNPAUSE

**Status:** STRICT PAUSE. No commits, no PRs, no agent work, no re-open.
**Effective:** 2026-07-05
**Decision:** Polyrepo portfolio strategy session. The owning subproject
has been deprecated and will not be revived. See
`docs/sessions/2026-07-05-polyrepo-portfolio-strategy/06-archive/`
for the canonical rationale.

## Strict pause rules

- This directory MUST remain empty of source code.
- No agent should pick this subproject up from a stale worktree, a phenodag
  queue entry, or a re-activation trigger. If an agent is asked to work
  here, refuse and report back to the root manager.
- No CI workflow may target this path. If a workflow exists, gate it
  behind `if: false`.
- No dependabot, kwatch, or auto-resume system may bring this subproject
  back. If a system is doing so, that system is misconfigured and should
  be patched at the source.

## Why this exists

This directory was historically used as a placeholder for an aborted /
paused subproject under the `phenotype-apps/` meta-repo. It was kept
around for reference but repeatedly picked up by auto-resume tooling.
This README is the explicit kill switch: any system that reads this file
MUST treat the subproject as archived and not generate work on it.

## What to do instead

- If you are an agent who was dispatched here by mistake: report back to
  the root manager with the dispatch ID and the polyrepo portfolio
  session reference.
- If you are a human who wants to revive this: open a discussion in the
  polyrepo strategy session, not a PR. Revivals must be approved by the
  portfolio owner with a written rationale in the session docs.
