# ADR-049: App-substrate drift detector (3-pass algorithm)

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7)
**L5-114** (v8 wave C)
**Refs:**
- ADR-023 (agent-effort governance)
- ADR-024 (71-pillar)
- ADR-040 (test coverage gates per tier)
- ADR-046 (federation mTLS + OIDC)
- ADR-048 (substrate graduation path)

---

## Context

App-level repos (`Civis`, `Dino`, `WSM`, `focalpoint`, `QuadSGM`, `AtomsBot*`, `HwLedger`) drift from substrate in 3 ways:

1. **Direct dependency drift** — app imports a substrate lib, but the substrate lib has moved APIs
2. **Transitive substrate change** — substrate crate X depends on substrate crate Y; Y's API change ripples to X's app consumers
3. **Bucket drift** — app's bucket (ACTIVE/CONDITIONAL/PAUSED per ADR-023) is stale (e.g. work resumed on a PAUSED app)

The drift detector is a 3-pass algorithm that catches each.

## Decision

**A weekly drift-detection pass runs alongside the 71-pillar refresh. The detector is `pheno-drift-detector` and uses a 3-pass algorithm.**

### The 3 passes

| Pass | Name | What it detects | Output |
|---|---|---|---|
| 1 | **Direct API drift** | App imports a substrate lib whose public API has changed since the app's last successful build | Per-app list of broken imports + remediation (bump dep + fix call sites) |
| 2 | **Transitive change impact** | Substrate crate Y's API change since the last weekly run; report all app consumers of Y (directly or transitively) | Per-app impact report: "Y changed; you depend on Y via Z; bump Z to <new>" |
| 3 | **Bucket drift** | App's actual git activity (open PRs, recent commits, branch ahead of upstream) vs declared bucket (ACTIVE/CONDITIONAL/PAUSED) | Per-app drift report: "AtomsBot is PAUSED but has 3 new commits on main; bucket needs to be PAUSED-as-target (legally mined) per ADR-023" |

### Algorithm

```python
def detect_drift():
    # Pass 1: direct API drift
    for app in APPS:
        last_good_build = app.last_good_build
        for dep in app.substrate_deps:
            if dep.api_hash != last_good_build.dep_api_hashes[dep.name]:
                yield (app, dep, "DIRECT_API_DRIFT")

    # Pass 2: transitive change impact
    for substrate_change in last_week_substrate_changes:
        for app in APPS:
            if app.depends_on(substrate_change.crate):
                yield (app, substrate_change, "TRANSITIVE_IMPACT")

    # Pass 3: bucket drift
    for app in APPS:
        bucket_declared = app.bucket  # from ADR-023 table
        activity = measure_activity(app)
        if bucket_declared == "PAUSED" and activity.has_new_commits:
            if app.is_capstone_legally_mined:
                yield (app, None, "BUCKET_OK_LEGALLY_MINED")
            else:
                yield (app, None, "BUCKET_DRIFT_UNDECLARED")
        elif bucket_declared == "ACTIVE" and activity.idle > 90_days:
            yield (app, None, "BUCKET_CANDIDATE_TO_RECLASSIFY")
```

### Enforcement

CI workflow (`.github/workflows/drift-detector.yml`):
```yaml
on:
  schedule:
    - cron: '0 17 * * 1'  # Mon 09:00 PST, weekly
  workflow_dispatch:

jobs:
  detect:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run drift detector
        run: pheno-drift-detector --out findings/drift-detection-2026-MM-DD.md
      - name: Open issue on drift
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const findings = fs.readFileSync('findings/drift-detection-2026-MM-DD.md', 'utf8');
            // parse + open per-app issues
```

### Drift findings

Each drift finding has:
- App name
- Drift type (direct / transitive / bucket)
- Severity (HIGH if app is ACTIVE + API broken; MEDIUM if PAUSED + new commits; LOW if transitive)
- Remediation (bump dep, reclassify bucket, etc.)
- Owner (from CODEOWNERS)

## Consequence

- 9 app-level repos scanned weekly for 3 drift types
- Direct API drift caught within 1 week (vs discovered at next build)
- Transitive changes reported proactively (vs discovered at next release)
- Bucket drift detected and reported as ADR-023 governance violation
- The detector is the source of truth for "is the app-substrate boundary healthy?"

## Cross-references

- ADR-023 (app-level triage + substrate placement)
- ADR-024 (71-pillar)
- ADR-040 (test coverage gates per tier)
- ADR-046 (federation mTLS + OIDC — drift detector covers federation too)
- ADR-048 (substrate graduation path — bucket drift detection)
- ADR-047 (predictive DRY — the detector reports DRY violations)
- ADR-050 / ADR-077 (vault migration — example of API drift target)
