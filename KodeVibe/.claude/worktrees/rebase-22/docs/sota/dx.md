# DX — SOTA (KodeVibe)

## Workflow (chosen)

1. `make engine-build` — compile Go engine
2. `./kodevibe scan` or vibecheck CLI (delegates to engine binary)
3. `kodevibe install-hooks` for git integration
4. CI: `kodevibe scan --ci --strict`

```bash
make engine-build
./kodevibe scan --vibes security,code
```

## Evolution triggers

- Engine build moves to pure `go build` — update workflow
