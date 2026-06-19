# target-pruner

Automated tool for reclaiming disk space by pruning old cargo build artifacts and related caches.

## Overview

Phenotype multi-agent workspace frequently accumulates stale `target/` directories (6-8 GB each) from parallel cargo builds. This tool safely removes old builds based on **modification time (mtime)** plus git commit recency, preserving repos with recent edits or commits.

## Usage

```bash
# View targets that would be pruned (dry-run)
target-pruner --dry-run

# Actually prune old targets
target-pruner --prune

# Prune and show full report with bytes reclaimed
target-pruner --prune --report
```

## Scope

Targets for pruning (in priority order):

1. **Worktree targets** (`repos/.worktrees/*/target`) — safe to delete after branch is pushed
2. **Completed-push targets** — targets whose branch exists on `origin/`
3. **Archived worktrees** (`.worktrees/**` with age >7 days + no uncommitted changes)

## Staleness Source

By default the pruner uses **mtime** (file modification time) plus the
`.git/HEAD` commit recency check. mtime reflects real edits/builds and is not
perturbed by `du` or cargo metadata reads.

`--use-atime` is provided for backward compatibility only. **Do not rely on
atime during active multi-agent sessions:** on APFS, `du`, cargo, `walkdir`,
and any file-stat walk reset atime to "today", so the pruner will refuse to
free targets that are actually idle. If atime is the source and a target looks
recent for that reason, fall back to `rm -rf <repo>/target` after confirming no
`cargo` process is touching it (`ps aux | grep cargo`).

## Expansion Roadmap

Future versions will also prune:

- **node_modules** directories (especially in worktrees) — often 1-3 GB per project
- **Homebrew cache** (`~/Library/Caches/Homebrew`) — coordinated with `disk-emergency.rs`
- **npm cache** (`~/.npm/_cacache`) — if `disk-emergency` has completed

## Configuration

See `target-budget.toml` for age thresholds and exclusion patterns.

## See Also

- `/repos/docs/governance/disk_budget_policy.md`
- `/repos/scripts/disk-emergency.rs` — emergency playbook for 100% disk situations
