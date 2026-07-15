# KodeVibeGo runtime migration

The Go static-analysis engine from [KodeVibeGo](https://github.com/KooshaPari/KodeVibeGo) lives under `engine/` in this repository (git subtree, squashed).

## Runtime surface (100% in `engine/`)

| Area | Path |
|------|------|
| CLI | `engine/cmd/cli` |
| Daemon / REST + WebSocket | `engine/cmd/server`, `engine/pkg/server` |
| Scanner + vibes registry | `engine/pkg/scanner`, `engine/pkg/vibes` |
| MCP hooks | `engine/pkg/mcp` |
| Scoring | `engine/pkg/scoring` |
| Reports (text/JSON/HTML/JUnit/CSV) | `engine/pkg/report` |
| Fix engine, watch mode | `engine/pkg/fix`, `engine/pkg/watch` |
| Config | `engine/pkg/config`, `engine/.kodevibe.yaml` |
| VS Code extension stub | `engine/vscode-extension` |

Governance rule schema and YAML templates remain in [HexaKit](https://github.com/KooshaPari/HexaKit) (`phenotype-compliance-scanner`, PR #152).

## Build

```bash
make engine-build   # engine/build/kodevibe + kodevibe-server
make engine-test
```

The `kodevibe` shell entrypoint delegates to `engine/build/kodevibe` when present (`scan`, `watch`, `server`, `fix`).

## Lineage

KodeVibeGo is archived after this migration; do not delete the archived repository.
