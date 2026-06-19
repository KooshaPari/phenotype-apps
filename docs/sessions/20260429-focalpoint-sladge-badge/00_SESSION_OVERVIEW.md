# Session Overview

## Goal

Add the sladge badge to FocalPoint while preserving unrelated local changes in
the canonical checkout.

## Outcome

- Added the `AI Slop Inside` badge to `README.md`.
- Used isolated worktree `FocalPoint-wtrees/sladge-badge` because canonical
  `FocalPoint` already had unrelated README, iOS, Android, docs-site, FFI,
  audio, connector, generated framework, and status changes.
- Kept the change docs-only.

## Success Criteria

- README includes the sladge badge.
- Session docs explain the isolated-worktree decision.
- The worktree is clean after commit.
