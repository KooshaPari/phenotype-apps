# KodeVibeGo Predecessor Lineage

**Status:** Fully superseded — [KodeVibeGo](https://github.com/KooshaPari/KodeVibeGo) archived 2026-05-31  
**Successors:** [KodeVibe](../) (Shell CLI) + [HexaKit](https://github.com/KooshaPari/HexaKit) (governance)

---

## Migration Checklist (100% Moved)

| Pattern | KodeVibeGo Source | Successor | Status |
|---------|-------------------|-----------|--------|
| `.kodevibe.yaml` schema | `internal/models/models.go`, `.kodevibe.yaml` | `docs/kodevibe-config-schema.md`, `.kodevibe.yaml` | ✅ |
| Vibe checker registry | `pkg/vibes/registry.go` | HexaKit `phenotype-compliance-scanner` | ✅ |
| Advanced scoring engine | `pkg/scoring/advanced_scoring.go` | HexaKit governance docs | ✅ |
| MCP context payload | `pkg/mcp/mcp.go` | HexaKit `docs/governance/kodevibego-mcp-scoring.md` | ✅ |
| Agent quick endpoints | `pkg/server/server.go` (`/quick`, `/status/compact`) | Documented in config schema | ✅ |
| HTML report templates | `pkg/report/html_*.go` | Reference only (phenotype-org-audits pattern) | ✅ |
| VS Code extension stub | `vscode-extension/` | Not migrated (out of scope) | ⬜ dropped |
| VitePress docs site | `docs/` | HexaKit docs infrastructure | ✅ |

---

## What KodeVibe (Shell) Owns

- CLI commands: `scan`, `fix`, `watch`, `hooks`, `config`, `report`, `profile`
- `.kodevibe.yaml` parsing and vibe execution
- Text/JSON report output

## What HexaKit Owns

- `Checker` registry pattern → `phenotype-compliance-scanner` crate
- Weighted scoring with trend/momentum analysis
- MCP `QualityTargets` + `AIOptimization` context for AI fix loops
- CI quality gates (`quality-gate.yml`, `gate-check.yml`)

---

## References

- [Configuration Schema](./kodevibe-config-schema.md)
- [HexaKit Governance: KodeVibeGo MCP & Scoring](https://github.com/KooshaPari/HexaKit/blob/main/docs/governance/kodevibego-mcp-scoring.md)
- [KodeVibeGo GitHub (archived)](https://github.com/KooshaPari/KodeVibeGo)
