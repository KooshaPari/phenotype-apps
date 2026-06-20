# WORKLOG — pheno-worklog-schema

> Schema: v2.1 (ADR-025 / ADR-030) — 11 columns including `device:`.
> This file is dogfooded — the lib that parses/validates v2.1 must itself use v2.1.

| Date | Task ID | Layer | Action | Files | Notes | device | scope | risk | deps | links |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2026-06-20 | L5-119-11D | governance | add-tier0-metabundle | README.md, AGENTS.md, SPEC.md, llms.txt, CHANGELOG.md, WORKLOG.md, LICENSE-MIT, LICENSE-APACHE | Tier-0 meta-bundle (ADR-023 Rule 3.1, ADR-039, ADR-040). 80% lib coverage gate. | macbook | pheno-worklog-schema | low | none | AGENTS.md#tier-0 |
| 2026-06-20 | L5-119-11D | source | scaffold-src | src/pheno_worklog_schema/__init__.py, parser.py, emitter.py, __main__.py | v2.1 schema parser + emitter + CLI; v2.0→v2.1 migration helper; structlog wiring (ADR-036B). | macbook | pheno-worklog-schema | low | none | SPEC.md#api-surface |
| 2026-06-20 | L5-119-11D | test | add-tests | tests/test_parser.py, tests/test_emitter.py, tests/test_migrate_v20_to_v21.py | 10 unit + 4 migration cases. | macbook | pheno-worklog-schema | low | src/ | tests/ |
| 2026-06-20 | L5-119-11D | ci | add-workflow | .github/workflows/ci.yml | pytest matrix (3.10/3.11/3.12) + ruff + coverage at 80%. | macbook | pheno-worklog-schema | low | tests/ | .github/workflows/ci.yml |
| 2026-06-20 | L5-119-11D | docs | add-quickstart | examples/quickstart.py | 5-line quickstart (per ADR-023 quickstart rule). | macbook | pheno-worklog-schema | low | none | examples/quickstart.py |
| 2026-06-20 | ADR-025 | governance | fleet-v2.1-deprecation-sweep | WORKLOG.md | v2.1 deprecation sweep orchestrator pass: audited fleet WORKLOG.md files. Migrated `pheno-agents-md/WORKLOG.md` (was free-form prose) → v2.1 (11 cols, +`device:`); verified `pheno-llms-txt/WORKLOG.md` already at v2.1 (no migration rows needed); `pheno-prompt-test/WORKLOG.md` not present (blocker: repo absent from sparse-checkout cone, see subagent report). | macbook | pheno-worklog-schema | low | ADR-025, ADR-030 | docs/governance.md#v20-to-v21-migration |
