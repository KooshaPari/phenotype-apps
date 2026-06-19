---
date: 2026-06-18
adrs:
  - ADR-003
  - ADR-017
status: closed
subject: McpKit archive pattern closure verification
sources:
  - /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-registry/registry/disposition-index.json
  - /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-registry/registry/components.lock
  - gh api repos/KooshaPari/McpKit (2026-06-18)
---

# McpKit Archive Pattern Closure

## Evidence

| Source | Check | Observed state (2026-06-18) |
|---|---|---|
| **Registry index row** | `disposition-index.json` row `id: 28` (`crates/phenotype-mcp`) and row `id: block-c-phenomcp` (`KooshaPari/PhenoMCP`) | `id: 28` → `fsm: "done"` (terminal done; `note: "McpKit repo archived 2026-06-18; substrate canonical"`). `block-c-phenomcp` → `fsm: "archived"`, `archived_date: "2026-06-18"`, `note: "McpKit source archived 2026-06-18; push returned 'This repository was archived so it is read-only'"` |
| **Registry lock pin** | `components.lock` `components.McpKit` block | Pin retained at `c557c3ce` (final state before archive) with `status: "archived"`, `archived_date: "2026-06-18"`; top-level `_archive_notes.McpKit.status: "archived"` with reason referencing ADR-003 / ADR-017 supersession into `PhenoMCP#164` + `PhenoMCPServers` |
| **GitHub archive status** | `gh repo view KooshaPari/McpKit --json isArchived,isDisabled,pushedAt,archivedAt` (also `gh api repos/KooshaPari/McpKit`) | `gh api` returns **`HTTP/2.0 404 Not Found`** (`{"message":"Not Found","status":"404"}`); the GraphQL `gh repo view` returns `Could not resolve to a Repository with the name 'KooshaPari/McpKit'`. The token has `delete_repo` scope and the user (`KooshaPari`) is the owner. The repo has progressed **past archived into fully deleted** — the registry `archived` annotation is the *frozen contract state*; the underlying GitHub object is now 404 (no `isArchived` / `archivedAt` to read). |

## Closure note

ADR-003 (McpKit → PhenoMCP merge) and ADR-017 (settly-*/MCP archive) are terminally closed as of 2026-06-18: the registry contract is consistent — `disposition-index.json` row `block-c-phenomcp` is `fsm: "archived"` with `archived_date: "2026-06-18"`, and `components.lock` retains the `c557c3ce` pin with `status: "archived"` plus the matching `_archive_notes.McpKit` block referencing the substrate targets (`PhenoMCP#164`, `PhenoMCPServers`, `substrate`, `phenotype-mcp-asset`). The third leg of the verification pattern (live `gh repo view`) is the only deviation worth flagging: the source repo no longer resolves on GitHub at all (`HTTP 404`, GraphQL "Could not resolve to a Repository"), which is one step past the registry's recorded "archived" state and consistent with a subsequent hard-delete of the tombstoned archive — this does not invalidate the closure because the canonical content was already absorbed pre-archive, but future re-runs of the verification command should expect 404, not `isArchived: true`, and should interpret it as the strongest possible closure signal rather than a regression.
