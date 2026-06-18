---
title: "Threat Model"
version: 0.1.0
lastUpdated: 2026-06-16
---

# Threat Model

> **Source of truth:** BytePort (Phenotype-org infrastructure tooling, Svelte-based UI)
> **Scope:** Svelte web app, build pipeline, CI/CD, distribution

## Assets

1. **Svelte source (`src/`)** — Svelte components, TypeScript, route handlers. If mutable, an attacker can ship a backdoor in any of the UI flows.
2. **Build output (`dist/`, `build/`)** — Compiled static assets served to the user. If mutable in the build pipeline, can ship a backdoored bundle.
3. **CI pipeline (GitHub Actions)** — Builds, tests, and deploys. If mutable, can inject backdoors.
4. **Public S3 / CDN hosting target** — Where the built bundle is served from. If mutable, can substitute the bundle.
5. **User session tokens** — Bearer tokens for any backend API. If leaked, an adversary can drive the API on behalf of the user.

## Threats (STRIDE)

| Category | Threat | Likelihood | Impact | Mitigation |
|---|---|---|---|---|
| **Spoofing** | An adversary publishes a `BytePort` bundle under a similar download URL. | Medium | High | Releases are signed (cosign, keyless). The deployment pipeline uses SRI hashes for `<script>` and `<link>` tags. The README documents the canonical install command. |
| **Tampering** | A Svelte component is modified in a release to drop a backdoor (e.g., a hidden form that exfiltrates input). | Low | High | All commits are signed (gitsign, keyless). CI runs `codeql` JavaScript scan on every PR. The release script verifies the build output hash. |
| **Repudiation** | A contributor pushes a component change and later denies it. | Low | Medium | All commits are signed. Releases are tagged. The git history is the audit trail. |
| **Information Disclosure** | A Svelte component inadvertently includes a sensitive field (e.g., a hardcoded API key for a 3rd-party service). | Medium | Medium | The CI runs `gitleaks` on every PR. The Svelte preprocessor has a `redact-output` filter that masks known secret patterns in `console.log` statements. |
| **Denial of Service** | A maliciously-large bundle (1GB) is served to the user. | Medium | Low | The build enforces `max-bundle-size=10MB`. Bundles over the limit fail the build. |
| **Elevation of Privilege** | A malicious npm package in the dependency tree executes arbitrary code at build time (via `postinstall` script). | Low | Critical | `package-lock.json` is committed; CI uses `npm ci --ignore-scripts` by default. `npm audit` runs on every PR. npm provenance is verified on every install. |

## Residual Risk and Revision Cadence

The most material residual risk is **build pipeline compromise** — a malicious change to the Svelte build process affects every user of the bundle. The strongest available mitigation is the gitleaks pre-commit hook + commit signing, but these do not catch a deliberately obfuscated payload. The next highest residual is **CDN substitution** — if a CDN mirror is compromised, every user fetches the backdoored bundle. This threat model should be revised quarterly (February, May, August, November) or whenever a new Svelte component is added, a new dependency is integrated, or the hosting target changes. The revision trigger is any PR that adds a new component, a new dependency, or a new public-facing endpoint.
