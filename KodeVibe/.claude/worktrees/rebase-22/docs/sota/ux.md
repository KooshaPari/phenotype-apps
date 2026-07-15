# UX — SOTA (KodeVibe)

## Status

**Active — developer-facing CLI and hook UX.**

## Requirements

| Requirement | Weight |
|-------------|--------|
| Actionable scan output with severity tiers | must |
| Hook install in ≤3 commands | should |
| CI mode with stable exit codes | must |

## Chosen strategy

Shell CLI provides familiar `scan`, `install-hooks`, and `config` commands; delegates heavy analysis to Go engine. Reports support JSON/HTML for CI artifacts.

## Evolution triggers

- IDE extension demand → evaluate vs MCP-only agent path

Update [../../../SOTA.md](../../../SOTA.md) UX row when major UX change ships.
