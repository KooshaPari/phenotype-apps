# Architecture Decision Record (ADR) — Template

This directory follows the **MADR** (Markdown ADR) convention. Each ADR is a single
file that documents a significant architectural decision, including its context,
the decision itself, and its consequences.

## Template

```markdown
# [short title of solved problem and solution]

- **Status:** [proposed | accepted | deprecated | superseded]
- **Date:** [YYYY-MM-DD]
- **Supersedes:** [optional: ADR-NNN]
- **Superseded by:** [optional: ADR-NNN]

## Context

What is the issue motivating this decision? What forces are at play (technical,
business, schedule)? What options were considered?

## Decision

What was decided? If there were multiple options, why this one over the others?

## Consequences

Describe the resulting context after applying the decision. Use the following
three categories:

### Positive

- Benefit one
- Benefit two

### Negative

- Drawback one
- Drawback two

### Neutral

- Observation one
- Observation two
```

## Metadata Fields

| Field         | Description                                                        |
|---------------|--------------------------------------------------------------------|
| Status        | Lifecycle status of the decision                                   |
| Date          | ISO-8601 date the decision was made                                |
| Supersedes    | ADR(s) this decision replaces                                      |
| Superseded by | ADR(s) that replace this decision                                  |

## Status Definitions

| Status       | Meaning                                                              |
|--------------|----------------------------------------------------------------------|
| `proposed`   | Under discussion; not yet accepted                                   |
| `accepted`   | Approved and adopted                                                 |
| `deprecated` | No longer recommended; kept for historical record                    |
| `superseded` | Replaced by a newer ADR; kept for historical record                  |

## New ADR Checklist

1. Choose the next sequential number (`docs/adr/NNNN-title.md`)
2. Copy the template above
3. Fill in all sections with concrete, repo-specific reasoning
4. Add the entry to `docs/adr/INDEX.md`
5. If superseding an existing ADR, update the superseded ADR's `Superseded by`
