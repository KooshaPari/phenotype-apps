# ACTION-PLAN: Tokn (strongest in fleet)

> **Generated 2026-06-16.** Current mean: **0.51 / 3.0**. Owner TBD.
> Repo path: `/Users/kooshapari/CodeProjects/Phenotype/repos/Tokn`

## Why this repo is the strongest

Tokn is one of the top 5 repos in the fleet by mean pillar score. Below is what it does well and the remaining gaps.

## Strongest pillars (score 3)

(none yet)

Total score-3 pillars: **0 / 109**

## Remaining gaps (0-1, top 5)

| # | Task | Pillars | Effort | Risk | Role |
|---|------|---------|--------|------|------|
| 1 | Add OpenAPI | AP1 | 3-5 min | Low | general-dev |
| 2 | Investigate and fix AS1 | AS1 | 3-5 min | Low | general-dev |
| 3 | Investigate and fix AS2 | AS2 | 3-5 min | Low | general-dev |
| 4 | Investigate and fix AT1 | AT1 | 3-5 min | Low | general-dev |
| 5 | Investigate and fix AT2 | AT2 | 3-5 min | Low | general-dev |

## Pattern: extract to phenoShared

- Tokn is a candidate to **donate** its governance/security/testing patterns back to `phenoShared` so the other 137 repos can adopt.
- Top pattern to extract: most-used score-3 pillar implementation.

## Re-score trigger

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/Tokn
git add -A && git commit -m 'feat(Tokn): adopt remaining quick wins'
```
