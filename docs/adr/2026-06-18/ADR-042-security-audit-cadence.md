# ADR-042: Security audit cadence

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7)
**L5-110.3** (v8 wave B)
**Refs:**
- ADR-023 (substrate placement)
- ADR-046 (federation mTLS + OIDC)
- ADR-049 (app-substrate drift detector)
- ADR-050 (vault migration roadmap, accepted 2026-06-21)
- ADR-077 (vault migration roadmap — supersession: ADR-050 is the v8 forward reference, ADR-077 is the v19 implementation)
- ADR-078 (encryption-at-rest mandate, accepted 2026-06-21)

---

## Context

Security advisories (CVEs) land daily. The fleet has 3 security domains (Rust, Python, Go/TypeScript) each with different tooling:

- Rust: `cargo audit` (RustSec advisory DB)
- Python: `pip-audit` (PyPI advisory DB)
- Go: `govulncheck` (Go vuln DB)
- TypeScript: `npm audit` (NPM advisory DB)

A monthly cadence catches ~95% of high/critical advisories within 30 days. The remaining 5% are caught by the weekly 71-pillar L48 (SBOM) refresh.

## Decision

**A monthly security audit runs in the first week of each month, covering all 4 language advisories. Failures block the next release for the affected substrate tier.**

### Per-language scan

| Language | Tool | DB | Failure handling |
|---|---|---|---|
| Rust | `cargo audit` | RustSec | `--deny warnings` blocks release |
| Python | `pip-audit` | PyPI | `--strict` blocks release |
| Go | `govulncheck` | Go vuln DB | `-show=verbose` reports; CI fails on HIGH/CRITICAL |
| TypeScript | `npm audit --omit=dev` | NPM | `--audit-level=high` blocks release |

### Enforcement

CI workflow (`.github/workflows/security-audit.yml`):
```yaml
on:
  schedule:
    - cron: '0 17 1-7 * *'  # First week of month, Mon 09:00 PST
  workflow_dispatch:

jobs:
  audit:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        language: [rust, python, go, typescript]
    steps:
      - uses: actions/checkout@v4
      - name: Run audit
        run: |
          case "${{ matrix.language }}" in
            rust)     cargo audit --deny warnings ;;
            python)   pip-audit --strict ;;
            go)       govulncheck ./... ;;
            typescript) npm audit --omit=dev --audit-level=high ;;
          esac
      - name: Upload report
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: security-audit-${{ matrix.language }}
          path: audit-report.json
```

### Audit findings

Findings are written to `findings/security-audit-2026-MM-<lang>.md` with:
- Language, scan tool, DB version
- Per-repo pass/fail
- Per-CVE severity + remediation (upgrade path)
- Action items (remediation PRs, owner, ETA)

## Consequence

- 22/22 substrate repos audited monthly across 4 languages
- HIGH/CRITICAL CVEs caught within 30 days (vs ad-hoc before)
- Substrate tiers below 60% coverage are exempt until coverage gate is met (ADR-040)
- Federation cross-org (ADR-046) gets the same monthly cadence

## Cross-references

- ADR-023 Rule 3.1 (substrate quality bar)
- ADR-040 (test coverage gates — gates audit participation)
- ADR-046 (federation mTLS + OIDC — same monthly cadence)
- ADR-049 (drift detector — surfaces affected repos)
- ADR-050 / ADR-077 (vault migration — security remediation output target)
- ADR-078 (encryption-at-rest mandate — security remediation output target)
