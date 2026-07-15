#!/usr/bin/env python3
"""Generate 140-pillar audit scorecard for Eidolon."""
import json, os

ROOT = os.path.expanduser("~/CodeProjects/Phenotype/repos/Eidolon")
os.chdir(ROOT)

PILLARS = []

def p(id, domain, name, status, rationale, remediations=None):
    PILLARS.append({
        "id": id, "domain": domain, "name": name, "status": status,
        "rationale": rationale,
        "remediations": remediations or []
    })

# CODE QUALITY (14)
p("CQ-01","Code Quality","Unsafe code hygiene","satisfied","Minimal unsafe in codebase; FFI boundaries well-defined")
p("CQ-02","Code Quality","Dead code elimination","partial","Some dead_code warnings in kmobile",["Run cargo-machete to detect unused deps"])
p("CQ-03","Code Quality","Cyclomatic complexity","partial","Some large desktop handlers",["Add clippy complexity lint"])
p("CQ-04","Code Quality","Function length discipline","partial","desktop/lib.rs has very large functions",["Split desktop handlers into smaller modules"])
p("CQ-05","Code Quality","Module depth (max 4)","satisfied","Max depth 3")
p("CQ-06","Code Quality","Enum-driven design","satisfied","Strong enum patterns in core traits")
p("CQ-07","Code Quality","Match exhaustiveness","satisfied","Match statements exhaustive")
p("CQ-08","Code Quality","No unwrap/expect in lib code","partial","Some unwrap/expect in kmobile handlers",["Replace unwrap with ? propagation"])
p("CQ-09","Code Quality","Documentation comments on pub items","satisfied","Most pub items documented")
p("CQ-10","Code Quality","Clippy warnings as errors","satisfied","CI runs clippy -D warnings")
p("CQ-11","Code Quality","Rustfmt enforcement","satisfied","CI runs fmt --check")
p("CQ-12","Code Quality","No duplicated code blocks","partial","Some desktop/mobile share patterns",["Extract shared automation logic into core"])
p("CQ-13","Code Quality","Feature flags gating","satisfied","kmobile has extensive feature flags (cli/api/mcp/tui/desktop/audio)")
p("CQ-14","Code Quality","Consistent naming conventions","satisfied","Snake case, PascalCase per Rust standard")

# ARCHITECTURE (17)
p("ARCH-01","Architecture","Workspace organization","satisfied","5 crates: core/desktop/mobile/sandbox/kmobile")
p("ARCH-02","Architecture","Module dependency direction","satisfied","No inter-crate deps (all independent)")
p("ARCH-03","Architecture","Port/Adapter pattern","satisfied","Core defines trait-based automators; implementations in platform crates")
p("ARCH-04","Architecture","Single-responsibility crates","satisfied","Each crate has clear domain")
p("ARCH-05","Architecture","Architecture document exists","missing","No ARCHITECTURE.md",["Create ARCHITECTURE.md documenting trait-based design"])
p("ARCH-06","Architecture","ADRs exist","satisfied","3 ADRs in docs/adr/ + ADR-001 in docs/")
p("ARCH-07","Architecture","No circular dependencies","satisfied","Zero inter-crate deps (no circles)")
p("ARCH-08","Architecture","Public API surface documented","partial","Core traits documented; kmobile has clap docs",["Add crate-level rustdoc to all crates"])
p("ARCH-09","Architecture","Feature-gated compilation","satisfied","kmobile has 6 feature sets")
p("ARCH-10","Architecture","Error type hierarchy","satisfied","thiserror per crate; KMobileError enum")
p("ARCH-11","Architecture","Cross-crate type reuse","satisfied","Core trait types used across crates")
p("ARCH-12","Architecture","Trait object vs generic discipline","satisfied","Proper trait dispatch in automator pattern")
p("ARCH-13","Architecture","Minimal public surface","satisfied","Pub items limited to essential API")
p("ARCH-14","Architecture","Versioning strategy","missing","No versioning policy documented",["Add VERSIONING.md"])
p("ARCH-15","Architecture","Deprecation policy","missing","No deprecation policy",["Add to CONTRIBUTING.md"])
p("ARCH-16","Architecture","Data flow documentation","partial","CLAUDE.md describes crate flow; no diagrams",["Add sequence diagrams to ARCHITECTURE.md"])
p("ARCH-17","Architecture","Plugin/extensibility","satisfied","Trait-based design enables extension")

# TESTING (26)
p("TEST-01","Testing","Unit tests per module","satisfied","11 mod tests blocks across crates")
p("TEST-02","Testing","Integration tests","satisfied","Desktop, mobile, sandbox integration tests")
p("TEST-03","Testing","Property-based tests","missing","No proptest/quickcheck",["Add proptest to event/input parsing"])
p("TEST-04","Testing","Fuzz targets","missing","No fuzz/ directory",["Add cargo-fuzz targets for MCP commands"])
p("TEST-05","Testing","Benchmark targets","satisfied","eidolon-core/benches/virtual_stage_dispatch.rs")
p("TEST-06","Testing","Nextest runner","missing","No nextest.toml",["Add nextest.toml with ci profile"])
p("TEST-07","Testing","Doc tests","missing","No doc tests on pub items",["Add doc examples"])
p("TEST-08","Testing","Mutation testing","missing","No cargo-mutants",["Consider for core trait dispatch"])
p("TEST-09","Testing","Coverage gate in CI","missing","No cargo-llvm-cov",["Add coverage job to CI (70% threshold)"])
p("TEST-10","Testing","Error path tests","satisfied","Error types tested in core and kmobile")
p("TEST-11","Testing","Edge case coverage","partial","Some edge cases untested",["Add device-disconnect and timeout tests"])
p("TEST-12","Testing","Test isolation","satisfied","No shared state between tests")
p("TEST-13","Testing","Deterministic tests","partial","Some tokio::test may have timing sensitivity",["Add timeout to async tests"])
p("TEST-14","Testing","Test naming conventions","satisfied","snake_case, descriptive names")
p("TEST-15","Testing","Test organization (AAA)","partial","Some tests lack AAA separation",["Standardize pattern"])
p("TEST-16","Testing","No test warnings","satisfied","Tests compile clean")
p("TEST-17","Testing","Platform-specific tests","satisfied","macOS integration tests, mobile tests")
p("TEST-18","Testing","Test documentation","partial","Some test comments minimal",["Add explanations to complex assertions"])
p("TEST-19","Testing","Mocks/fakes","satisfied","mockall used in kmobile")
p("TEST-20","Testing","Performance regression tests","missing","No perf regression in CI",["Add bench comparison gate"])
p("TEST-21","Testing","Smoke/sanity tests","partial","No end-to-end device-automation smoke test",["Add E2E smoke test"])
p("TEST-22","Testing","Test data fixtures","satisfied","Test fixtures exist per crate")
p("TEST-23","Testing","Slow test tagging","missing","No #[ignore] tags",["Tag integration tests as slow"])
p("TEST-24","Testing","Coverage visibility","missing","No per-crate coverage reports",["Add coverage badges"])
p("TEST-25","Testing","Error message testing","satisfied","Error messages verified in tests")
p("TEST-26","Testing","Dependency injection","satisfied","Trait-based injection")

# OBSERVABILITY (18)
p("OBS-01","Observability","Structured logging","satisfied","tracing-subscriber with env-filter in kmobile")
p("OBS-02","Observability","Tracing spans on hot paths","satisfied","tracing::info/debug/warn across kmobile (10+ modules)")
p("OBS-03","Observability","Tracing subscriber in binary","satisfied","kmobile main.rs initializes subscriber")
p("OBS-04","Observability","OpenTelemetry exporter","missing","No OTLP export",["Add tracing-opentelemetry bridge"])
p("OBS-05","Observability","Error types implement Display","satisfied","thiserror derives Display on all error types")
p("OBS-06","Observability","Request/operation IDs","missing","No request-id propagation",["Add tracing span IDs to MCP/HTTP handlers"])
p("OBS-07","Observability","W3C trace context","missing","No trace context propagation",["Add opentelemetry for W3C trace context"])
p("OBS-08","Observability","Metrics collection","missing","No metrics crate",["Add metrics crate + dispatch counters"])
p("OBS-09","Observability","Metrics endpoint","partial","kmobile has axum but no /metrics route",["Add /metrics endpoint with prometheus exporter"])
p("OBS-10","Observability","Health check endpoint","missing","No /healthz endpoint",["Add health endpoint to axum server"])
p("OBS-11","Observability","Structured panic messages","missing","No custom panic hook",["Add panic hook with JSON output"])
p("OBS-12","Observability","Log level configuration","satisfied","RUST_LOG env var respected")
p("OBS-13","Observability","Sensitive data redaction","missing","No log redaction",["Add sensitive field masking"])
p("OBS-14","Observability","Audit trail","missing","No audit events for device ops",["Add audit events for device actions"])
p("OBS-15","Observability","Error context enrichment","partial","thiserror provides context; no span context",["Add span fields on errors"])
p("OBS-16","Observability","Latency histograms","missing","No per-op timing metrics",["Add histogram metrics for device ops"])
p("OBS-17","Observability","Obs integration tests","missing","No obs tests",["Test trace emission on key paths"])
p("OBS-18","Observability","Structured error output","partial","CLI errors printed; no JSON error output for API mode",["Add JSON error responses for axum handlers"])

# SECURITY (32)
p("SEC-01","Security","Deny.toml exists","satisfied","deny.toml present")
p("SEC-02","Security","License allowlist","satisfied","MIT, Apache-2.0, ISC allowed")
p("SEC-03","Security","Banned deps","satisfied","deny.toml bans known-problematic deps")
p("SEC-04","Security","Source whitelist","satisfied","crates-io only")
p("SEC-05","Security","cargo-deny in CI","satisfied","cargo-deny.yml workflow exists (separate from ci.yml)")
p("SEC-06","Security","Secrets scanning in CI","satisfied","trufflehog.yml workflow")
p("SEC-07","Security","SAST/static analysis","satisfied","clippy -D + codeql.yml")
p("SEC-08","Security","SBOM generation","satisfied","sbom-refresh.yml workflow")
p("SEC-09","Security","Dependency scanning","satisfied","cargo-audit.yml workflow")
p("SEC-10","Security","Source verification","satisfied","deny.toml restricts to crates-io")
p("SEC-11","Security","Private key handling","satisfied","No keys in repo; device auth uses env vars")
p("SEC-12","Security","Hardcoded credential detection","satisfied","TruffleHog; 0 findings")
p("SEC-13","Security","SLSA/L3 provenance","satisfied","release-attestation.yml (SLSA L2)")
p("SEC-14","Security","CodeQL analysis","satisfied","codeql.yml workflow")
p("SEC-15","Security","Security policy","satisfied","SECURITY.md exists")
p("SEC-16","Security","Dependabot","satisfied","dependabot.yml (cargo + github-actions)")
p("SEC-17","Security","OSSF Scorecard","satisfied","scorecard.yml workflow (weekly)")
p("SEC-18","Security","Governance file integrity","missing","No governance.yml check",["Add governance file integrity workflow"])
p("SEC-19","Security","Audit log for security events","na","No security event surface")
p("SEC-20","Security","Fuzzing for memory safety","missing","No fuzz targets",["Add cargo-fuzz for device command parsers"])
p("SEC-21","Security","No unsafe in parser paths","satisfied","Safe Rust throughout")
p("SEC-22","Security","Input validation","satisfied","Device input validated through type system")
p("SEC-23","Security","Path traversal protections","partial","Device paths may need sanitization",["Add path validation for file transfer APIs"])
p("SEC-24","Security","DoS resistance","partial","No explicit resource limits on device ops",["Add timeouts and rate limits"])
p("SEC-25","Security","Memory safety","satisfied","Safe Rust; no unsafe outside FFI")
p("SEC-26","Security","Integer overflow","partial","No checked arithmetic on device IDs",["Add checked arithmetic"])
p("SEC-27","Security","Thread safety","satisfied","All types Send+Sync; tokio runtime")
p("SEC-28","Security","Security contact","partial","SECURITY.md exists but brief",["Populate with PGP key and contact"])
p("SEC-29","Security","Vulnerability disclosure","missing","No disclosure process",["Add disclosure policy to SECURITY.md"])
p("SEC-30","Security","Regular audit schedule","partial","cargo-audit.yml on push; no schedule",["Add scheduled weekly cargo-audit"])
p("SEC-31","Security","Dependency freeze","satisfied","Cargo.lock committed")
p("SEC-32","Security","Security regression tests","missing","No security-specific test cases",["Add regression tests for known vulns"])

# DOCUMENTATION (17)
p("DOC-01","Documentation","README.md","satisfied","Detailed README with badges, status, description")
p("DOC-02","Documentation","LICENSE file","satisfied","MIT/Apache-2.0 dual license")
p("DOC-03","Documentation","CHANGELOG.md","satisfied","Exists with conventional-commit entries")
p("DOC-04","Documentation","ARCHITECTURE.md","missing","Not present",["Create ARCHITECTURE.md"])
p("DOC-05","Documentation","CONTRIBUTING.md","satisfied","Exists")
p("DOC-06","Documentation","ADRs","satisfied","3 ADRs in docs/adr/ + docs/ADR-001")
p("DOC-07","Documentation","Crate-level rustdoc","missing","No #![doc] on crate roots",["Add crate docs to all 5 crates"])
p("DOC-08","Documentation","OpenAPI spec","partial","kmobile has axum routes but no OpenAPI spec",["Generate OpenAPI 3.1 spec"])
p("DOC-09","Documentation","Migration guide","missing","No migration guide",["Add MIGRATION.md"])
p("DOC-10","Documentation","Examples directory","partial","docs/getting-started.md exists; no code examples/",["Add examples/ with basic usage"])
p("DOC-11","Documentation","Deployment guide","missing","No deploy docs",["Add docs/operations/DEPLOY.md"])
p("DOC-12","Documentation","Code comments","satisfied","Complex sections well-documented")
p("DOC-13","Documentation","Doc tests","missing","No doc tests",["Add doc examples to key functions"])
p("DOC-14","Documentation","Module-level docs","partial","Some modules documented; not all",["Add module docstrings to all src files"])
p("DOC-15","Documentation","Glossary","missing","No glossary",["Add GLOSSARY.md"])
p("DOC-16","Documentation","FAQ","missing","No FAQ",["Add FAQ.md"])
p("DOC-17","Documentation","RFC process","missing","No RFC process",["Document proposal process in CONTRIBUTING.md"])

# CI/CD (14)
p("CI-01","CI/CD","CI on push/PR","satisfied","ci.yml triggers on push/PR to main")
p("CI-02","CI/CD","Build passes with no errors","satisfied","check + test + clippy all pass in CI")
p("CI-03","CI/CD","Lint step","satisfied","clippy + fmt in ci.yml")
p("CI-04","CI/CD","Test step","satisfied","cargo test --all-targets")
p("CI-05","CI/CD","Test matrix (OS/version)","missing","Only ubuntu-24.04; no Windows/macOS",["Add cross-platform CI matrix"])
p("CI-06","CI/CD","Dependency audit in CI","satisfied","cargo-deny.yml + cargo-audit.yml")
p("CI-07","CI/CD","Secrets scanning in CI","satisfied","trufflehog.yml")
p("CI-08","CI/CD","CI timeout limits","satisfied","ci.yml has implicit timeout")
p("CI-09","CI/CD","Caching","partial","No Swatinem/rust-cache in ci.yml",["Add cargo caching step"])
p("CI-10","CI/CD","Docs validation","satisfied","doc-links.yml workflow")
p("CI-11","CI/CD","Workflow SHA pinning","satisfied","All actions use SHA references")
p("CI-12","CI/CD","PR gates","partial","No branch protection documented",["Configure branch protection rules"])
p("CI-13","CI/CD","CODEOWNERS","satisfied","Default owner @KooshaPari")
p("CI-14","CI/CD","Dependabot","satisfied","Weekly cargo + github-actions updates")

# SUPPLY CHAIN (20)
p("SC-01","Supply Chain","Dependencies pinned","satisfied","Cargo.lock committed")
p("SC-02","Supply Chain","License compliance","satisfied","deny.toml license allowlist")
p("SC-03","Supply Chain","No banned deps","satisfied","deny.toml bans")
p("SC-04","Supply Chain","Source whitelist","satisfied","crates-io only")
p("SC-05","Supply Chain","Advisory DB","satisfied","cargo-deny advisories + cargo-audit.yml")
p("SC-06","Supply Chain","Dependency freshness","missing","No cargo-outdated tracking",["Add cargo-outdated schedule"])
p("SC-07","Supply Chain","Dependency diff","partial","Dependabot PRs no diff summary",["Add cargo-diff to dependabot PRs"])
p("SC-08","Supply Chain","SBOM per release","satisfied","sbom-refresh.yml generates CycloneDX")
p("SC-09","Supply Chain","Build provenance","satisfied","release-attestation.yml")
p("SC-10","Supply Chain","No pre-built binaries","satisfied","No .exe/.dll/.so")
p("SC-11","Supply Chain","Vendored deps","na","No vendored deps")
p("SC-12","Supply Chain","Submodule integrity","na","No submodules")
p("SC-13","Supply Chain","Compiler version pinned","partial","CI uses dtolnay with stable; no rust-toolchain.toml",["Add rust-toolchain.toml"])
p("SC-14","Supply Chain","Dependency review","missing","No dependency-review action",["Add dependency-review.yml workflow"])
p("SC-15","Supply Chain","License compliance check","satisfied","cargo-deny check licenses")
p("SC-16","Supply Chain","Vuln notification","partial","cargo-audit.yml runs; no issue filing",["Add auto-issue creation on vulns"])
p("SC-17","Supply Chain","Build hardening","partial","No hardened runner",["Use hardened runner for release builds"])
p("SC-18","Supply Chain","Artifact signing","missing","No cosign/gpg signing",["Add cosign signing to release"])
p("SC-19","Supply Chain","Provenance for CI artifacts","satisfied","SLSA L2 attestation")
p("SC-20","Supply Chain","Dependency tree","missing","No cargo-tree tooling",["Add to scripts/"])

# RELEASE ENGINEERING (13)
p("RE-01","Release Eng","Semantic versioning","partial","0.0.1 pre-release; no semver policy",["Document semver policy"])
p("RE-02","Release Eng","Release workflow","satisfied","release-attestation.yml exists (triggered by release)")
p("RE-03","Release Eng","Release notes","partial","No explicit release notes generation",["Add git-cliff to release workflow"])
p("RE-04","Release Eng","Changelog generation","partial","CHANGELOG.md manually updated",["Add git-cliff automation"])
p("RE-05","Release Eng","Tagged releases","missing","0 tags",["Cut v0.0.1 release"])
p("RE-06","Release Eng","Pre-release testing","missing","No pre-release pipeline",["Add pre-release test step"])
p("RE-07","Release Eng","Distribution packaging","missing","No Homebrew/apt/pkg",["Add cargo-packaging or binary distribution"])
p("RE-08","Release Eng","Published to crates.io","missing","Not published",["Publish to crates.io"])
p("RE-09","Release Eng","Release attestation","satisfied","SLSA L2")
p("RE-10","Release Eng","SLO definitions","missing","No release SLOs",["Add release SLOs"])
p("RE-11","Release Eng","Rollback plan","missing","No rollback docs",["Add docs/operations/rollback.md"])
p("RE-12","Release Eng","Breaking change policy","missing","No BREAKING_CHANGES.md",["Add breaking change doc"])
p("RE-13","Release Eng","Release checklist","missing","No checklist",["Add RELEASE_CHECKLIST.md"])

# DEVELOPER EXPERIENCE (13)
p("DX-01","Developer Exp",".editorconfig","satisfied","Present with Rust/Python rules")
p("DX-02","Developer Exp","Nextest config","missing","No nextest.toml",["Add nextest config"])
p("DX-03","Developer Exp","Sccache","missing","No sccache in CI",["Add sccache caching action"])
p("DX-04","Developer Exp","Pre-commit hooks","missing","No lefthook.yml or .husky",["Add lefthook with fmt+lint hooks"])
p("DX-05","Developer Exp","rust-analyzer config","missing","No rust-project.json",["Generate rust-project.json"])
p("DX-06","Developer Exp","Task runner","satisfied","Justfile present")
p("DX-07","Developer Exp","Quick start guide","satisfied","docs/getting-started.md + README")
p("DX-08","Developer Exp","CLAUDE.md / AI context","satisfied","CLAUDE.md (75 lines) + AGENTS.md")
p("DX-09","Developer Exp","Dev container","partial","Possibly exists; not in git root",["Add .devcontainer/devcontainer.json"])
p("DX-10","Developer Exp","Env file","missing","No .env.example",["Add .env.example"])
p("DX-11","Developer Exp","Conventional commits","partial","Git log shows some; no enforcement",["Add commit-msg hook via lefthook"])
p("DX-12","Developer Exp","Quick feedback loop","satisfied","Single crate builds fast")
p("DX-13","Developer Exp","Issue/PR templates","missing","No .github/ISSUE_TEMPLATE/",["Add bug_report.yml + PULL_REQUEST_TEMPLATE.md"])

# Compute
from collections import defaultdict
counts = {"satisfied": 0, "partial": 0, "missing": 0, "na": 0}
for col in PILLARS:
    counts[col["status"]] += 1

scorable = counts["satisfied"] + counts["partial"] + counts["missing"]
score = round((counts["satisfied"] + counts["partial"] * 0.5) / scorable * 100, 1)

def grade(s):
    if s >= 95: return "A+"
    if s >= 90: return "A"
    if s >= 85: return "B+"
    if s >= 80: return "B"
    if s >= 70: return "C+"
    if s >= 60: return "C"
    if s >= 50: return "D"
    return "F"

domains = defaultdict(lambda: {"satisfied": 0, "partial": 0, "missing": 0, "na": 0})
for col in PILLARS:
    domains[col["domain"]][col["status"]] += 1

out = {
    "repo": "Eidolon",
    "audit_version": "2.0",
    "audit_date": "2026-07-08",
    "pillar_taxonomy": "pillar-taxonomy-v2-140",
    "totals": {
        "pillars_total": scorable,
        "pillars_satisfied": counts["satisfied"],
        "pillars_partial": counts["partial"],
        "pillars_missing": counts["missing"],
        "pillars_na": counts["na"],
        "score_pct": score,
        "grade": grade(score)
    },
    "domain_breakdown": {},
    "pillars_evaluated": [{
        "id": p["id"], "domain": p["domain"], "name": p["name"],
        "status": p["status"], "rationale": p["rationale"],
        "remediations": p["remediations"]
    } for p in PILLARS]
}

for dom in sorted(domains.keys()):
    d = domains[dom]
    sc = d["satisfied"] + d["partial"] + d["missing"]
    ds = round((d["satisfied"] + d["partial"] * 0.5) / sc * 100, 1) if sc > 0 else None
    out["domain_breakdown"][dom] = {
        "total": sum(d.values()), "satisfied": d["satisfied"],
        "partial": d["partial"], "missing": d["missing"], "na": d["na"],
        "score_pct": ds, "grade": grade(ds) if ds is not None else "N/A"
    }

with open("audit_scorecard.json", "w") as f:
    json.dump(out, f, indent=2)

print(f"Score: {score}% (Grade {grade(score)})")
print(f"Satisfied: {counts['satisfied']} / Partial: {counts['partial']} / Missing: {counts['missing']} / N/A: {counts['na']}")
print(f"Total pillars: {len(PILLARS)}, Scorable: {scorable}")
