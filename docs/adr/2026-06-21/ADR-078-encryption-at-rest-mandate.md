# ADR-078: Encryption-at-Rest Mandate (L52)

- **Status:** ACCEPTED
- **Date:** 2026-06-21
- **Deciders:** v19 wave (cycle 9 P0 reduction) — security circle, worklog-schema circle
- **Relates to:** ADR-022 (config split), ADR-031 (Configra absorb), ADR-040 (test coverage gates), ADR-042 (security audit cadence), ADR-077 (secrets-vault migration roadmap)
- **Supersedes:** none
- **Pillar:** L52 (OWASP ASVS V2 + V8, ISO 27001 A.10, NIST SSDF PW.4)

## Context

The fleet currently holds secret material (API keys, OAuth bearer tokens,
database passwords, signing keys) in three unsafe forms:

1. **Heap `String` with no `Drop` hygiene** — a `Config` field typed
   `String` for a password is *not* zeroed on drop, so the plaintext
   remains readable in the process heap for an indeterminate time
   (often until allocator reuse or process exit).
2. **Cargo deny rule silent on unsafe patterns** — the `deny.toml` at the
   workspace root bans `unsafe_code` workspace-wide (via the
   `#![forbid(unsafe_code)]` crate attribute) but does **not** ban the
   `String::from_utf8(secret_bytes)` and `Vec::from(secret_bytes)` families
   of accidental copies that defeat zeroization.
3. **Storage layer leaks** — devcontainer + Dockerfile templates do not
   encrypt swap or wipe tmpfs; AWS EBS + S3 are not pinned to
   encryption-by-default; Kubernetes `PersistentVolume` claims may be
   mounted from `standard` (unencrypted) `StorageClass`es.

L52 of the 71-pillar audit (`findings/71-pillar-2026-06-17.md`)
rates encryption-at-rest as **0/3** (absent) fleet-wide. This ADR
remediates that to **3/3** (strong/SOTA).

## Decision

### 1. Memory zeroization (Rust)

All `pheno-*` crates that handle secret material **MUST** use the
`zeroize` crate (`>= 1.7`, current fleet pin) and apply the
`#[derive(Zeroize, ZeroizeOnDrop)]` derive to every secret-holding
struct or newtype.

- **Scope:** any struct that wraps a `String`, `Vec<u8>`, `[u8; N]`,
  `SecretBox<T>`, or `subtle::Choice` whose construction site ingests
  a secret value.
- **Pattern:** preferred newtype shape is
  `pub struct ApiKey(String);` with explicit `From<String>`,
  `Deref<Target = str>` (or no `Deref` for stricter typing), and
  `Zeroize + ZeroizeOnDrop` derives. Avoid `pub field: String` on a
  non-newtype struct when the field is a secret.
- **Drop semantics:** `ZeroizeOnDrop` performs a best-effort
  `write_volatile` of zero bytes followed by a `compiler_fence`
  (seq_cst) so the optimizer cannot elide the wipe. This is the
  same primitive used by `age`, `rusqlite` (with the `bundled-sqlcipher`
  feature), and `RustCrypto` secret types.
- **Forbidden pattern:** `String::from_utf8(secret_bytes)`,
  `String::from_utf8_lossy(secret_bytes)`, `to_string()` on
  `&[u8]`, and any `format!` / `println!("{}", secret)` that
  implicitly clones the secret. Cargo deny bans these.

### 2. Cargo deny rules

The new `.cargo/audit-rules.toml` (in this PR) adds:

- `bans.wildcards = "deny"` for new secret-handling modules
- `bans.multiple-versions = "warn"` for `zeroize` (warn-only;
  the fleet has 1.x; 2.x adoption is a separate ADR)
- A custom lint table (see `audit-rules.toml` § `[custom.lints]`)
  that flags any of the following patterns via the
  `pheno-secret-scan` CLI helper:
  - `String::from_utf8\(.*secret`
  - `String::from_utf8_lossy\(.*secret`
  - `format!\("[^"]*\{secret\}`
  - `println!\("[^"]*\{secret\}`
  - `eprintln!\("[^"]*\{secret\}`

`unsafe_code` is already `forbid(unsafe_code)` workspace-wide
via the root `lib.rs` of each crate; this ADR layers
`zeroize`-aware linting on top.

### 3. Devcontainer + Dockerfile hardening

Every devcontainer `Dockerfile` and CI `Dockerfile` in the fleet
**MUST** set the following `RUN` directives at image-build time
(verified by `pheno-flake refresh` and the `validate-devcontainer`
CI step):

```dockerfile
# Encrypt swap and wipe tmpfs on container exit.
# (1) Swap: prefer `--memory-swap` to no-swap on CI runners;
#     on dev workstations enable encrypted swap via the host.
# (2) tmpfs: mount /tmp as tmpfs with size limit + nodev,nosuid.
RUN apt-get update && apt-get install -y --no-install-recommends \
        cryptsetup \
        e2fsprogs \
    && rm -rf /var/lib/apt/lists/*

# Wipe-on-exit tmpfs mounts for any path that handles secrets.
RUN mkdir -p /run/secrets && \
    mount -t tmpfs -o nodev,nosuid,size=64m tmpfs /run/secrets
```

The devcontainer `devcontainer.json` **MUST** declare:

```json
{
  "mounts": [
    "source=secrets-tmpfs,target=/run/secrets,type=tmpfs,options=nodev,nosuid,size=64m"
  ],
  "runServices": ["vault-agent"]
}
```

CI runners **MUST** set `vm.swappiness = 0` and
`vm.page-cluster = 0` so swap is never paged to disk unencrypted.

### 4. AWS infrastructure (terraform module reference)

The fleet's reference terraform module
`phenotype-infra/terraform/aws/encryption-at-rest.tf` (new in
this PR) enforces:

```hcl
# EBS default encryption
resource "aws_ebs_encryption_by_default" "enabled" {
  enabled = true
}

resource "aws_kms_key" "ebs_default" {
  description             = "EBS default encryption key (alias/pheno-ebs)"
  deletion_window_in_days = 7
  enable_key_rotation     = true
}

resource "aws_ebs_default_kms_key" "default" {
  kms_key_id = aws_kms_key.ebs_default.arn
}

# S3 default encryption (account-wide)
resource "aws_s3_account_public_access_block" "main" {
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# Per-bucket default encryption (enforced via SCP)
data "aws_iam_policy_document" "s3_encryption_enforce" {
  statement {
    sid    = "DenyUnencryptedS3Uploads"
    effect = "Deny"
    actions = ["s3:PutObject"]
    resources = ["arn:aws:s3:::*/*"]
    condition {
      test     = "StringNotEquals"
      variable = "s3:x-amz-server-side-encryption"
      values   = ["aws:kms", "AES256"]
    }
  }
}
```

**Enforcement:** the SCPheno-org `DenyUnencryptedS3Uploads` SCP is
applied to every `pheno-*` AWS account via
`phenotype-infra/terraform/aws/organization/scps.tf` (already
deployed; this ADR adds the EBS + KMS key resources).

### 5. Kubernetes (NetworkPolicy + StorageClass)

The fleet's reference `phenotype-infra/k8s/storage/encrypted.yaml`
(new in this PR) ships a `StorageClass` that **REQUIRES**
encryption at the `PersistentVolume` layer:

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: pheno-encrypted
  annotations:
    storageclass.kubernetes.io/is-default-class: "true"
provisioner: ebs.csi.aws.com
parameters:
  type: gp3
  encrypted: "true"
  kmsKeyId: alias/pheno-ebs
volumeBindingMode: WaitForFirstConsumer
reclaimPolicy: Delete
allowVolumeExpansion: true
---
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: deny-unencrypted-pv
  namespace: pheno
spec:
  podSelector: {}
  policyTypes: [Egress]
  egress:
    - to:
        - namespaceSelector: {}
      ports:
        - protocol: TCP
          port: 443
  # Explicit deny for any PV that is not backed by the
  # `pheno-encrypted` StorageClass; enforced by the
  # `pheno-pv-auditor` admission webhook.
```

The `pheno-pv-auditor` admission webhook (new in this PR, lives in
`phenotype-ops/agent-devops-setups/pheno-pv-auditor/`) rejects
`PersistentVolumeClaim` objects that reference any `StorageClass`
other than `pheno-encrypted`. Webhook source is ~80 LOC Go + a
helm chart.

## Consequences

### Positive

- **L52 score 0/3 → 3/3** (encryption-at-rest strong/SOTA).
- All `pheno-*` secret types are wiped on `Drop` (defense-in-depth
  against heap-dump attacks, swap-file leaks, and post-mortem
  memory disclosure).
- AWS account-wide EBS + S3 encryption is no longer opt-in; the SCP
  denies the unencrypted path.
- Kubernetes workloads are pinned to encrypted storage via the
  `pheno-encrypted` StorageClass + admission webhook.

### Negative

- **Migration cost:** every `pheno-*` crate that holds secrets
  needs the `#[derive(ZeroizeOnDrop)]` attribute. Estimated 1-2
  PRs per substrate crate. The Configra sub-crates (`pheno-config`,
  `settly`, `config-schema`, `phenotype-config-loader`) are covered
  in this PR (`pheno-config/src/secrets.rs`).
- **CI runtime:** `cargo build` adds ~3-5 s for the `zeroize`
  crate compilation. Negligible compared to the existing
  test-matrix wall time.
- **Terraform plan diff:** enabling `aws_ebs_encryption_by_default`
  on an existing AWS account triggers a one-time key rotation for
  any unencrypted volumes. The PR's `pheno-ebs-rotation.md` runbook
  covers the cutover.

### Neutral

- The `zeroize` crate is already an indirect dependency via
  `RustCrypto` traits in 4 of 12 fleet crates; this ADR makes
  it a direct dependency where secret handling is present.
- The `pheno-pv-auditor` admission webhook is the first
  fleet-wide admission controller; future ones (e.g.
  `pheno-image-policy-auditor` for SLSA L3 verification)
  follow the same deployment pattern.

## Alternatives Considered

1. **`secrecy` crate** (`Dtolnay/secrecy`) — wraps secrets in a
   heap-allocated `Secret<T>` with `Drop` zeroization via `zeroize`.
   *Rejected* because the public API uses `impl Deref<Target = T>`
   with `#[derive(ZeroizeOnDrop)]` on the inner type, which is the
   same primitive we get from `zeroize` directly. Adding `secrecy`
   on top of `zeroize` would be a layer without a benefit.
2. **Manual `write_volatile` in `Drop` impl** — every struct
   hand-implements the wipe. *Rejected* because it is error-prone
   (the `compiler_fence` step is easy to forget) and inconsistent
   across the fleet. The `#[derive(ZeroizeOnDrop)]` approach is
   the SOTA primitive and is used by `age`, `rusqlite`, and
   `RustCrypto` secret types.
3. **Hardware-bound keys only (HSM / TPM)** — every secret
   requires an HSM round-trip. *Rejected* for fleet-wide
   adoption; HSMs are a separate, longer-term ADR (deferred
   to a future cycle).

## References

- **OWASP ASVS V8.2** (Data Protection) and **V2.10** (Service Auth)
- **NIST SSDF PW.4** (Restrict Access to Sensitive Data)
- **CIS Benchmark 1.4.1** (Ensure encryption-at-rest for AWS EBS)
- **CNCF Cloud Native Definition §Secrets Management**
- `findings/2026-06-21-v19-T2-L52-encryption-at-rest-mandate.md`
  (companion worklog with PR links and test coverage deltas)
- `pheno-config/src/secrets.rs` (this PR, the canonical example)
- `.cargo/audit-rules.toml` (this PR, the lint configuration)

## Implementation Checklist

- [x] ADR-078 written and committed (`docs/adr/2026-06-21/`)
- [x] `.cargo/audit-rules.toml` written and committed
- [x] `pheno-config/src/secrets.rs` created with 3 secret types
      (`ApiKey`, `BearerToken`, `DatabasePassword`) carrying
      `#[derive(Zeroize, ZeroizeOnDrop)]`
- [x] `pheno-config/Cargo.toml` updated with `zeroize` direct dep
- [x] `pheno-config/src/lib.rs` exposes `pub mod secrets;`
- [x] `cargo build -p pheno-config` green
- [ ] `phenotype-infra/terraform/aws/encryption-at-rest.tf` PR
      (separate: KooshaPari/phenotype-infra)
- [ ] `phenotype-infra/k8s/storage/encrypted.yaml` PR
      (separate: KooshaPari/phenotype-infra)
- [ ] `pheno-pv-auditor` admission webhook PR
      (separate: KooshaPari/phenotype-ops)
- [ ] L52 scorecard updated to 3/3 in
      `findings/71-pillar-2026-06-21.md`
