# Secrets-Management Convention (Vault-canonical)

> **Status:** ACTIVE | **Owner:** v19 T1 (L5-153) | **Companion to:** `docs/adr/2026-06-21/ADR-079-vault-migration-roadmap.md`
> **Last reviewed:** 2026-06-21 (v19 cycle 9 launch)

This document is the operational runbook for HashiCorp Vault usage across the Phenotype fleet. It is the authoritative reference for:

1. Secret classification (what counts as a secret).
2. Storage path conventions (where secrets live in Vault).
3. Rotation cadence (when and how secrets are rotated).
4. Audit + access attribution (how secret access is logged).
5. Migration runbook from `.env` to Vault (for existing apps).

It is the implementation reference for **ADR-079** and inherits the cross-cutting constraints from **ADR-046** (federation mTLS+OIDC), **ADR-012** (pheno-tracing substrate), and **ADR-077** (SLSA L3 provenance).

---

## 1. Secret classification

The fleet recognises 4 secret classes. Every secret MUST be tagged with one and only one class in Vault metadata.

| Class | Examples | Storage backend | Rotation cadence |
|---|---|---|---|
| **`service-token`** | AppRole `secret_id`, GitHub Actions OIDC binding, cross-service API tokens | Vault KV v2 (`secret/data/pheno/{env}/{app}/{role}/...`) | 90d |
| **`human-token`** | `vault userpass`, HeliosLab CLI auth, admin console logins | Vault userpass + TOTP | 30d |
| **`dynamic-credential`** | AWS STS dynamic creds, PostgreSQL dynamic creds | Vault AWS / database secret engines | 15min (AWS) / 24h (DB) |
| **`key-material`** | cosign signing key (ADR-077), TLS private keys, JWT signing keys | Vault Transit engine | 24h (cosign) / 30d (TLS) / 90d (JWT) |

A secret that does not fit any class MUST NOT be stored in Vault — it is a config value, not a secret, and belongs in `pheno-config`.

### What is NOT a secret

The following are **configuration values** and MUST stay in `pheno-config` (not Vault):

- Public API endpoints (`https://api.openai.com/v1`)
- Feature flags (`pheno-flags`)
- Non-secret database DSNs that already contain a username but no password
- Service mesh routing config
- Logging levels, retry counts, timeouts

Rationale: storing non-secrets in Vault dilutes the audit signal and increases blast radius.

---

## 2. Vault path conventions

All paths follow the pattern:

```
{engine}/data/{namespace}/{environment}/{app-or-service}/{role}/{secret-name}
```

### Concrete examples

```
# Service token (KV v2 engine)
secret/data/pheno/dev/pheno-config/db-password
secret/data/pheno/prod/pheno-port-adapter/tls-cert

# Dynamic credential (database engine)
database/creds/pheno-app-prod
database/creds/pheno-mcp-router

# AWS dynamic credential
aws/creds/pheno-deploy-prod

# Key material (transit engine)
transit/keys/cosign-signing-2026
transit/keys/jwt-signing-prod

# PKI cert (pki engine)
pki-int/issue/pheno-services-dot-io
```

### Namespace rules

| Environment | Namespace | Mount path | Notes |
|---|---|---|---|
| `dev` | `pheno/dev/` | mounted on every dev cluster | may share Vault instances across devs |
| `staging` | `pheno/staging/` | per-cluster mount | one Vault instance per region |
| `prod` | `pheno/prod/` | per-cluster mount, audit-log enforced | one Vault instance per region; replicated cross-region |

The `pheno/{env}` prefix is **mandatory**. A secret without the prefix will be rejected by the pre-commit lint (`scripts/vault_lint.sh`).

---

## 3. Rotation cadence

| Class | Cadence | Method | Owner |
|---|---|---|---|
| `service-token` | 90d | `vault write -f auth/approle/role/<role>/secret-id` then re-deploy | `scripts/vault_rotate_approle.sh` (cron) |
| `human-token` | 30d | re-auth + MFA at next login | user-driven |
| `dynamic-credential` (AWS) | 15min TTL | `vault read aws/creds/<role>` returns fresh STS | automatic on each Vault call |
| `dynamic-credential` (DB) | 24h TTL | `vault read database/creds/<role>` returns fresh creds | automatic on each Vault call |
| `key-material` (cosign) | 24h | ADR-077 SLSA workflow rotates the key + re-signs | CI-driven |
| `key-material` (TLS) | 30d | `pki-int/issue/...` with `ttl=720h`, auto-renewal via Vault agent | automatic |
| `key-material` (JWT) | 90d | manual + announced via `pheno-events` | human-driven with 2-person rule |

### Immediate-rotation triggers

Rotation MUST happen within 1 hour of any of:

- Personnel change (departure, role change, contract end)
- Suspected compromise (any SOC2/secrets finding)
- Fleet event `fleet-vault-rotate` (git hook or on-call trigger)
- Vendor-side key compromise announcement (e.g. AWS key rotation notice)

Use `scripts/vault_rotate_approle.sh --role <role> --reason "<trigger>"` for AppRole; `scripts/vault_revoke_human.sh --user <user> --reason "<trigger>"` for human tokens.

---

## 4. OIDC binding for CI

GitHub Actions, Buildkite, and Jenkins obtain Vault tokens **via OIDC**, not via long-lived `VAULT_TOKEN` env vars. This is the implementation of **ADR-046**'s federation contract for CI.

### GitHub Actions example

```yaml
# .github/workflows/deploy.yml
permissions:
  id-token: write   # required for OIDC
  contents: read
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: hashicorp/vault-action@v2.5.0
        with:
          method: jwt
          role: pheno-deploy-prod
          secrets: |
            secret/data/pheno/prod/pheno-port-adapter/tls-cert tls_cert
            secret/data/pheno/prod/pheno-otel/otlp-api-key otlp_api_key
        env:
          VAULT_ADDR: ${{ secrets.VAULT_ADDR }}
          VAULT_NAMESPACE: pheno/prod
```

The `pheno-deploy-prod` role is bound to the GitHub OIDC subject `repo:KooshaPari/phenotype-apps:environment:prod`. Subject-claim rules are enforced in the Vault role config (see `scripts/vault_apply_oidc_role.sh`).

### Buildkite / Jenkins

The same pattern applies via the `vault-jwt` auth method. Reference implementations:

- Buildkite: `examples/oidc_consumer/buildkite/` (shipped with T3 in v19)
- Jenkins: `examples/oidc_consumer/jenkins/` (T3 followup)

---

## 5. Audit + access attribution

Every secret read MUST be logged. The log pipeline is:

```
Vault audit log → file socket → pheno-otel OTLP/HTTP collector → pheno-tracing → Loki (90d) + S3 (7y)
```

ADR-012's `pheno-tracing` substrate is the canonical collector. Vault is configured with:

```hcl
audit {
  type "file" {
    path = "/vault/audit/audit.log"
  }
}

# Telemetry stanza for OTLP export
telemetry {
  otlp_endpoint = "http://pheno-otel:4317"
}
```

### Required log fields

Every audit entry MUST contain:

- `request.path` (Vault path)
- `request.client_token` (truncated token ID, not the full token)
- `request.remote_addr` (client IP)
- `pheno-trace-id` (injected by `pheno-port-adapter` if the call originates from a traced process)
- `actor` (Vault entity ID, resolved from the auth method)

### Alerts

- Failed-auth > 5 in 1m → page on-call via `pheno-events`
- Secret-read > 100 in 1m from a single token → page on-call
- Off-hours access (00:00-06:00 PT) → notification (no page)
- Geolocation anomalies (token auth from new country) → notification

---

## 6. Migration runbook: `.env` → Vault

For apps still using `.env` files, the migration is mechanical.

### Step 1 — Inventory

```bash
# Find all .env references in src/
rg -l '\.env' src/

# Find all env::var references
rg -l 'env::var' src/
```

### Step 2 — Categorise

For each `env::var("FOO")` call, determine its class:

- Real secret → migrate to Vault
- Config value → leave in `pheno-config`

### Step 3 — Vault write

```bash
# Example: pheno-config db-password
vault kv put secret/pheno/dev/pheno-config/db-password \
  value="$(openssl rand -hex 32)"
```

### Step 4 — Code change

```rust
// Before
let password = std::env::var("PHENO_CONFIG_DB_PASSWORD")?;

// After
let password = vault::read("secret/data/pheno/dev/pheno-config/db-password")?;
```

Use the `vault` Rust crate (already a transitive dep via `pheno-port-adapter`). For Python use `hvac`; for Go use `github.com/hashicorp/vault/api`.

### Step 5 — Cleanup

After deployment + 24h soak:

1. Delete the env-var from GitHub Actions secrets (or AWS Secrets Manager).
2. Delete the `.env.example` entries.
3. Add a `#[deprecated]` warning to any legacy `env::var` wrapper.
4. Update the app's README.md to reference this convention.

### Rollback

If Vault is unavailable in production:

1. Page on-call via `pheno-events`.
2. `vault status` to confirm Vault is sealed/unreachable.
3. Run `scripts/vault_emergency_token.sh <role>` to mint an emergency AppRole token with 1h TTL (requires `pheno-ops` human approval via MFA).
4. Inject the emergency token into the app's env vars via the fleet's emergency runbook.

The emergency token path is the ONLY legitimate exception to the "no long-lived Vault tokens in env vars" rule. It expires automatically.

---

## 7. References

- `docs/adr/2026-06-21/ADR-079-vault-migration-roadmap.md` — companion ADR
- `docs/adr/2026-06-18/ADR-046-federation-mtls-oidc.md` — federation contract
- `docs/adr/2026-06-15/ADR-012-pheno-tracing-canonical.md` — OTLP audit pipeline
- `docs/adr/2026-06-20/ADR-077-slsa-l3-provenance.md` — cosign key custody
- `docs/adr/2026-06-18/ADR-038-hexagonal-port-adapter-l4-policy.md` — Vault port abstraction
- `findings/2026-06-21-v18-cycle-8-probe.md` §6 — L50 baseline finding
- `findings/2026-06-21-v18-T1-L17-fedramp-soc2-readiness.md` — SOC 2 CC6.1 evidence linkage
- [HashiCorp Vault documentation](https://developer.hashicorp.com/vault/docs)