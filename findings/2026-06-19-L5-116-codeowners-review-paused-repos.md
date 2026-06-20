# CODEOWNERS review for PAUSED app-level repos (ADR-023 FU7)

**ADR anchor:** `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md` Rule 2
**Author:** L5-116, 2026-06-19
**Status:** COMPLETE — audit data verified, proposals ready

| Repo | Has CODEOWNERS? | Size | SHA | Proposed action |
|---|---|---|---|---|
| `focalpoint` | Yes | 507 B | `e23a18a0` | Append ADR-023 soft-block comment (already routes to `@KooshaPari`) |
| `QuadSGM` | Yes | 203 B | `5aeebf8e` | Append ADR-023 soft-block comment (already routes to `@KooshaPari`) |
| `AtomsBot` | Yes | 45 B | `8e9f2014` | Replace with archival-mining rules (docs, tests, schemas allowed) |
| `AtomsBot-2nd` | **No** | — | — | Create CODEOWNERS with archival-mining rules |
| `AtomsBot-wtrees` | **No** | — | — | Create CODEOWNERS with archival-mining rules |

Verification: `gh api repos/KooshaPari/<repo>/contents/.github/CODEOWNERS` per repo, 2026-06-19.
