# 0005 — Upstream sync strategy: automated weekly sync via GitHub Actions

- **Status:** accepted
- **Date:** 2026-07-08
- **Supersedes:** none

## Context

Planify is a fork of [makeplane/plane](https://github.com/makeplane/plane) (see
ADR-0001). The `upstream/` directory contains a verbatim seed of
`makeplane/plane@preview` v1.3.1, and Planify's customizations live outside that
directory (`site/`, `infra/`).

To stay current with upstream bug fixes, security patches, and feature
improvements, we need a repeatable process for pulling new upstream Plane
releases into Planify. Before this ADR, there was no defined sync process — the
initial seed was a one-time copy.

The following sync strategies were considered:

| Option                    | Automation | Conflict Handling | Audit Trail | Maintenance |
|---------------------------|------------|-------------------|-------------|-------------|
| GitHub fork sync button   | Manual     | None (overwrite)  | None        | Zero        |
| Manual cherry-pick        | None       | Full control      | Manual log  | High        |
| Git subtree              | Full       | Partial           | Full        | Medium      |
| **Automated PR (chosen)** | **Scheduled** | **PR review**  | **Full**    | **Low**     |
| Automated merge (direct)  | Full       | Auto-resolve    | Full        | Low         |
| Vendor copy (rsync/tar)   | Scripted   | None (overwrite)  | Snapshot    | Medium      |

Key considerations that drove the decision:

1. **Upstream isolation**: ADR-0003 mandates strict separation between
   `upstream/` and Planify's own code. Any sync strategy must preserve this
   boundary — never mix Planify and upstream changes in the same subtree.

2. **Merge conflicts are expected**: Planify may patch upstream files for
   compatibility (e.g., Docker Compose overrides, environment defaults).
   An automated merge that silently resolves conflicts would risk introducing
   subtle regressions.

3. **Audit trail**: Each sync should produce a clear record of what changed
   upstream and how it was integrated, so contributors can trace regressions
   back to specific upstream commits.

4. **Scheduled cadence**: Weekly syncs balance staying current against the
   overhead of reviewing and merging upstream changes. Monthly would risk
   conflict accumulation; daily would be too noisy for the current team size.

The **GitHub fork sync button** was rejected because it provides no audit
trail, no conflict review, and no control over which upstream changes are
pulled — it simply fast-forwards the fork.

**Git subtree** was rejected because it mixes upstream files into the same tree
as Planify files, violating ADR-0003's isolation policy. It also requires every
contributor to understand subtree merge semantics.

**Manual cherry-pick** was rejected as too error-prone and time-consuming for
the maintainer to execute reliably on a recurring schedule.

**Automated merge (direct to main)** was rejected because silently merging
upstream changes without review could break the site build or introduce
regressions that reach production before they are caught.

## Decision

Adopt a **weekly automated sync workflow via GitHub Actions** that opens a
pull request with upstream changes for manual review.

### Mechanism

1. **Trigger**: A scheduled GitHub Actions workflow runs every Sunday at 02:00
   UTC (`.github/workflows/upstream-sync.yml`). Manual trigger is also
   available via `workflow_dispatch`.

2. **Fetch**: The workflow fetches `makeplane/plane@preview` and compares it
   against the last sync point (the merge-base between `upstream/preview` and
   Planify's `main`).

3. **PR creation**: If new upstream commits exist, the workflow creates a Pull
   Request against Planify's `main` branch using
   `peter-evans/create-pull-request@v6`. The PR body includes a list of new
   commits and a merge checklist for the reviewer.

4. **Conflict handling**: No automatic conflict resolution. If the PR has
   conflicts, they are flagged in the PR body and must be resolved manually
   before merge.

5. **No-op case**: If there are no new upstream commits, the workflow exits
   silently without opening a PR.

### Rationale

- **PR-based review** gives the maintainer a chance to verify upstream changes
  against Planify's customizations before they land on `main`.
- **Scheduled automation** removes the burden of remembering to sync manually
  while keeping the process transparent and auditable.
- **Graceful conflict handling** avoids the risk of silently overwriting
  Planify-specific patches.
- **Commit log in PR body** gives reviewers a quick overview of what changed
  upstream without leaving GitHub.

## Consequences

### Positive

- **Regular, predictable sync cadence**: Upstream changes are pulled weekly
  without manual effort.
- **Full audit trail**: Each sync PR records exactly which upstream commits
  were pulled, when, and by whom they were merged.
- **Review before merge**: No upstream change reaches `main` without
  maintainer approval.
- **Graceful no-op**: Weeks with no upstream releases produce zero noise.
- **Manual override**: `workflow_dispatch` allows ad-hoc syncs when a critical
  upstream fix lands mid-week.

### Negative

- **Merge conflicts may stall sync**: If upstream modifies files that Planify
  has patched, the sync PR will sit until a maintainer resolves conflicts
  manually. During that time, further upstream releases may accumulate and
  increase the conflict surface.
- **PR noise**: If upstream releases frequently, weekly sync PRs may create
  review backlog. Mitigated by the no-op exit when nothing changed.
- **Upstream CI non-determinism**: The upstream-sync workflow only syncs code;
  it does not run the upstream test suite. A sync PR might pass Planify's CI
  but introduce upstream test failures that won't be caught until the next
  manual full build.

### Neutral

- **GitHub Actions dependency**: The sync process relies on a third-party
  action (`peter-evans/create-pull-request`). If this action becomes
  unavailable, the workflow will need to be rewritten using `gh` CLI or `git`
  commands directly.
- **Sync branch accumulation**: Each sync creates a branch
  (`sync/upstream-YYYYMMDD`). The workflow deletes the branch after merge, but
  unmerged PRs leave orphan branches. A periodic cleanup can be added later
  if this becomes noisy.

## See Also

- [ADR-0001](0001-use-plane-so-fork.md) — Why Planify is a fork of makeplane/plane
- [ADR-0003](0003-monorepo-structure.md) — Three-directory layout and upstream isolation
- [Upstream sync workflow](../../.github/workflows/upstream-sync.yml) — Implementation
