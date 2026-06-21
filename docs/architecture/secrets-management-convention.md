# Secrets Management Convention (L50)

> **Status:** ACTIVE (ADR-077 + ADR-079, 2026-06-21)
> **Owner:** sec-eng-1
> **Applies to:** every substrate repo + every federated service + every CI workflow + every CLI tool
> **Companion ADR:** `docs/adr/2026-06-21/ADR-077-vault-migration-roadmap.md`

This is the canonical convention for **how secrets are stored, accessed, rotated, and audited** in the fleet. The backing system is **HashiCorp Vault 1.16.x** (per ADR-077). Every secret read in the fleet MUST go through Vault. Reading from `.env`, `os.environ`, `process.env`, `std::env::var`, or in-repo YAML is **forbidden** after the Phase 4 cutover (2026-09-13).

## 1. Vault path conventions

All secrets live under the canonical prefix `secret/data/pheno/`. The path is composed of five segments that map to environment, application, role, version, and key:

```
secret/data/pheno/{env}/{app}/{role}/{key}
```

| Segment | Allowed values | Example |
|---------|---------------|---------|
| `env` | `dev`, `staging`, `preprod`, `prod` | `prod` |
| `app` | substrate / service / CLI name (kebab-case) | `pheno-tracing`, `phenotype-go-sdk`, `helios-cli` |
| `role` | `app`, `ci`, `human`, `sdk` | `ci` |
| `key` | free-form, snake_case | `otlp_endpoint`, `aws_access_key_id`, `db_password` |

### 1.1 KV engine v2

Vault KV v2 stores each secret as a versioned document. Reading the latest version is the default:

```bash
vault kv get secret/data/pheno/prod/pheno-tracing/ci/otlp_endpoint
```

Pinning a version (for safe rollbacks):

```bash
vault kv get -version=3 secret/data/pheno/prod/pheno-tracing/ci/otlp_endpoint
```

Writing creates a new version. Old versions are kept until explicit `vault kv delete -versions=N`:

```bash
vault kv put secret/data/pheno/prod/pheno-tracing/ci/otlp_endpoint=value="https://otlp.example.internal:4317"
```

### 1.2 Path ownership

Every path has a single owning team. Cross-team access is denied by default and requires an ADR:

| Path prefix | Owner | Read policy |
|-------------|-------|-------------|
| `secret/data/pheno/prod/pheno-tracing/*` | platform-eng | `policy/ci-main` |
| `secret/data/pheno/prod/pheno-mcp-router/*` | platform-eng | `policy/ci-main` |
| `secret/data/pheno/prod/aws/*` | infra-eng | `policy/ci-aws-prod` |
| `secret/data/pheno/prod/db/*` | sec-eng | `policy/ci-db-prod` |
| `secret/data/pheno/*/sdk/*` | SDK consumers | `policy/sdk-consumer` |
| `secret/data/pheno/*/human/*` | humans (MFA) | `policy/human-admin` |

## 2. OIDC binding for CI

CI runners authenticate to Vault via **OIDC**, eliminating the long-lived `secrets.*` problem. Three providers are supported in this ADR; the canonical reference is ADR-079.

### 2.1 GitHub Actions

GitHub issues a short-lived JWT in `ACTIONS_ID_TOKEN_REQUEST_TOKEN`. Vault validates it against GitHub's OIDC discovery URL.

```yaml
# .github/workflows/ci.yml (Phase 2 onward)
- name: Vault auth (OIDC)
  uses: hashicorp/vault-action@v2
  with:
    method: oidc
    role: ci-main
    oidc-role-id: ${{ env.VAULT_ROLE_ID }}
    url: https://vault.example.internal:8200
    exportToken: true

- name: Read OTLP endpoint
  run: |
    echo "OTLP_ENDPOINT=$(vault kv get -format=json secret/data/pheno/prod/pheno-tracing/ci/otlp_endpoint | jq -r '.data.data.value')" >> $GITHUB_ENV
```

Bound claims (Appendix B of ADR-077):

- `repository = kooshapari/*`
- `ref = refs/heads/main` or `refs/heads/chore/*`
- `aud = vault.example.internal`

### 2.2 Buildkite

Buildkite issues OIDC tokens via the agent's `BUILDKITE_OIDC_TOKEN` env var. Vault's `auth/oidc/buildkite` validates the JWT against Buildkite's discovery URL.

```yaml
# .buildkite/pipeline.yml
steps:
  - label: ":lock: Vault auth"
    command: |
      export VAULT_TOKEN=$(buildkite-agent oidc request --audience vault.example.internal --lifetime 5m)
      vault login -method=oidc -token-only role=ci-main
```

Bound claims:

- `organization_slug = kooshapari`
- `pipeline_slug = *`

### 2.3 Jenkins

Jenkins issues OIDC tokens via the `oidc-jenkins-plugin`. Vault validates against the Jenkins OIDC issuer.

```groovy
// Jenkinsfile
stage('Vault auth') {
  steps {
    withCredentials([string(credentialsId: 'jenkins-oidc-issuer', variable: 'OIDC_ISSUER')]) {
      sh '''
        TOKEN=$(curl -s "${OIDC_ISSUER}/token" | jq -r .id_token)
        vault login -method=oidc -token-only role=ci-main jwt="$TOKEN"
      '''
    }
  }
}
```

Bound claims:

- `jenkins_url = https://ci.example.internal/`

### 2.4 What gets removed

After Phase 2 cutover, the following are deleted from all `.github/workflows/*.yml`:

- `secrets.AWS_ACCESS_KEY_ID`, `secrets.AWS_SECRET_ACCESS_KEY` (47 entries)
- `secrets.OTLP_API_KEY` (12 entries)
- `secrets.GITHUB_TOKEN` long-lived custom tokens (8 entries)

Replacement is OIDC exchange at workflow start (≤ 800 ms p95).

## 3. Dynamic credential rotation (TTLs)

All credentials are short-lived. The TTL table is the single source of truth:

| Credential type | TTL | Rotation trigger | Vault mechanism |
|-----------------|-----|------------------|-----------------|
| **AWS access keys** | 15 min | automatic | Vault AWS engine + IAM role assumption |
| **Database passwords** | 24 h | automatic | Vault database engine + dynamic role |
| **TLS certs** | 30 d | automatic | Vault PKI engine |
| **Service tokens (AppRole)** | 24 h | forced renewal at 50% TTL | `auth/approle/role/<role>/secret-id` |
| **CI tokens (OIDC)** | 1 h | workflow ends | `auth/oidc` |
| **Human tokens (userpass + MFA)** | 8 h | forced re-auth | `auth/userpass` + `identity/mfa` |
| **SDK consumer tokens** | n/a | pass-through | no long-lived creds |
| **Wrap tokens (cross-provider)** | 300 s | one-shot | `sys/wrapping/wrap` |

### 3.1 AWS dynamic credentials (15 min TTL)

```bash
vault read aws/creds/deploy-prod
# => Key: AKIA...  Secret: ...  lease: 900s
```

The returned credentials are IAM-session-bound (not long-lived IAM user keys) and expire automatically.

### 3.2 Database dynamic credentials (24 h TTL)

```bash
vault read database/creds/app-readonly
# => username: v-token-app-readonly-...  password: ...  lease: 86400s
```

The username is unique per request; the lease is bound to the requesting token.

### 3.3 Forced rotation triggers

Rotation happens **immediately** (not on TTL) on any of:

- Personnel change (departing employee / role change) → `vault token revoke -prefix <prefix>`.
- Suspected compromise → `vault operator rotate-root`.
- Fleet event (`fleet-vault-rotate` git hook, post-v19) → bulk rotation across all apps.

## 4. Migration runbook (from .env to Vault)

This runbook is referenced from ADR-077 § 8. Six steps, one PR per repo:

### Step 1 — Inventory

```bash
rg "(os\.environ|process\.env|std::env::var)" \
  --type-add 'env:*.env*' -t env -t py -t go -t rust -c
```

Produces a CSV: `repo,file,line,secret_name`. Owner: FTE-2 (platform-eng).

### Step 2 — Classify

Map each secret to a Vault path per § 1. Example:

| `.env` key | Vault path |
|------------|-----------|
| `OTLP_ENDPOINT` | `secret/data/pheno/prod/pheno-tracing/ci/otlp_endpoint` |
| `AWS_ACCESS_KEY_ID` | `aws/creds/deploy-prod` (dynamic, no path) |
| `DB_PASSWORD` | `database/creds/app-readonly` (dynamic, no path) |

### Step 3 — Migrate

One PR per file. Replace:

```python
# Before
import os
endpoint = os.environ["OTLP_ENDPOINT"]
```

With:

```python
# After
from vault_client import read
endpoint = read("secret/data/pheno/prod/pheno-tracing/ci/otlp_endpoint")["otlp_endpoint"]
```

Vault client bootstrap is the first call in `main()`:

```python
# bootstrap.py
from vault_client import bootstrap
bootstrap(role="ci-main", oidc_token=os.environ["ACTIONS_ID_TOKEN_REQUEST_TOKEN"])
```

### Step 4 — Verify

`scripts/check-vault-binding.sh` (Phase 3, owned by FTE-2) runs in CI: every secret read must have a corresponding Vault policy. Exit 1 if any direct `os.environ`/`process.env`/`std::env::var` read remains.

### Step 5 — Rotate

First read after migration triggers a one-time rotation. Old `.env` value is invalidated via `vault kv put ... value=<new>`.

### Step 6 — Decommission

- Rename `.env` → `.env.vault.example` with `<PLACEHOLDER>` values.
- Remove `os.environ["FOO"]` calls.
- PR description cites this ADR.

### Step 7 — Audit

After decommission, the gitleaks pre-commit hook + `scripts/check-no-env-leak.sh` (Phase 3) block any future `.env` regression at PR time.

## 5. Enforcement

| Check | Tool | When | Failure action |
|-------|------|------|----------------|
| No in-repo secrets | gitleaks pre-commit | every commit | commit blocked |
| No `.env` reads | `scripts/check-no-env-leak.sh` | every PR | CI fails |
| Vault path valid | `scripts/check-vault-binding.sh` | every PR | CI fails |
| OIDC bound audience | Vault policy audit | weekly (ADR-041 cadence) | alert sec-eng |
| MFA enforced | Vault userpass policy | weekly | alert sec-eng |
| Token age < TTL | Vault audit log query | weekly | auto-rotate |

## 6. Cross-references

- ADR-077 (this companion): `docs/adr/2026-06-21/ADR-077-vault-migration-roadmap.md`
- ADR-079 (OIDC reference): `docs/adr/2026-06-21/ADR-079-oidc-federation-reference.md`
- ADR-046 (Federation mTLS + OIDC): `docs/adr/2026-06-18/ADR-046-federation-mtls-oidc.md`
- ADR-042 (Security cadence): `docs/adr/2026-06-18/ADR-042-security-audit-cadence.md`
- ADR-041 (71-pillar refresh cadence): `docs/adr/2026-06-18/ADR-041-71-pillar-refresh-cadence.md`
- Smoke test: `scripts/vault_smoke.sh`
- L4 hexagonal ports: `docs/architecture/hexagonal-ports.md`