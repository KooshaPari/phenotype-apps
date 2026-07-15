# review.md — Kilo Code Stand

## Kilo Code Stand

- **standard_id:** `kilo-code-stand@1`
- **applies_to:** all PRs; agent-authored commits
- **owner:** KooshaPari
- **charter:** [charter.md](charter.md)
- **sota:** [SOTA.md](SOTA.md)

## Review tiers

| Tier | Action | Rules |
|------|--------|-------|
| **Block** | Fail PR | Secrets/credentials; scope outside charter; LLM validation code (kwality boundary); engine behavior change without test; deletes without migration proof |
| **Warn** | Comment | Doc drift from SOTA; incomplete OKF manifest; vibe rule changes without dimension note |
| **Info** | Optional | Style; micro-optimizations |

## Test policy

| Change type | Required evidence |
|-------------|-------------------|
| Bugfix | Regression test in `engine/` or repro steps |
| Feature | Engine test + SOTA/security dimension note if new vibe class |
| Refactor | No behavior change proof (engine tests green) |
| Docs-only | OKF manifest bump if structure changed |

## Agent roster

| Agent | Trigger | Must read |
|-------|---------|-----------|
| CI / CodeQL | PR | — |
| Compliance scanner | PR | charter.md, this file |
| KodeVibe | PR (if `.kodevibe.yaml`) | review tiers (self-check) |
| kwality | PR label `llm-review` | review LLM section |
| Review agent | PR | **this file**, intent summary |

## Org blocklist (always Block)

- Force-push to `main` / `master`
- Push to remotes outside `KooshaPari/*` without explicit user approval
- `git commit --amend` on pushed commits without user request
- LLM validation logic added here (use kwality)
- Deleting KodeVibeGo archive references without tombstone policy

## LLM review section

When `llm-review` label present:

1. Confirm PR description cites intent or FR ID
2. Check SOTA alignment for new dependencies
3. Flag assumptions not in [docs/intent/synthesis.md](docs/intent/synthesis.md)

## Output format (agents)

```markdown
## Kilo Review Summary
- verdict: pass | fail | needs-human
- charter_alignment: yes | no | unclear
- sota_alignment: yes | no | n/a
- findings:
  - { severity: block|warn|info, file, line, rule_id, message }
```

## Changelog

| Date | Change |
|------|--------|
| 2026-06-16 | Initial Kilo Code Stand from genesis template |
