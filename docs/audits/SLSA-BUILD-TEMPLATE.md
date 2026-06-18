# SLSA Build L2/L3 Template

> **Source audit:** `FLEET-AUDIT-REPORT.md` — S8 (SLSA Build L2) is the #2 P0 (priority 36, 8/11 audited repos at score 0).
> **Method:** Add a release workflow that emits SLSA Build provenance attestation.
> **How to use:** Copy the workflow below to `.github/workflows/release-attest.yml` in your repo, commit, push. Lifts S8 from 0 to 2 (wired).

## What is SLSA Build L2/L3?

SLSA Build L2 = build provenance signed by a trusted builder, generated from a hosted build platform.
SLSA Build L3 = L2 + signed by a non-forgeable key (e.g., GitHub's OIDC token) + isolation guarantees.

**For GitHub Actions users**, SLSA Build L3 is achievable with:
1. The official `actions/attest-build-provenance` action (or `slsa-framework/slsa-github-generator`)
2. `id-token: write` permission (for the OIDC token)
3. Build happens on GitHub-hosted runners (isolation by default)
4. All third-party actions SHA-pinned (no mutable refs)

The template below achieves L3 with minimal new code.

## Template: `.github/workflows/release-attest.yml`

```yaml
name: Release + SLSA Build Attestation

on:
  push:
    tags:
      - "v*"
  workflow_dispatch:
    inputs:
      version:
        description: "Release version (e.g., v1.6.8)"
        required: true
        type: string

# Required for SLSA Build L3:
# - id-token: write (OIDC token for signing provenance)
# - contents: read (read repo for the build)
# - attestations: write (write the provenance attestation)
permissions:
  contents: read
  id-token: write
  attestations: write

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  build:
    name: Build + Attest
    runs-on: ubuntu-24.04
    # No mutable action references. All third-party actions MUST be SHA-pinned.
    steps:
      - name: Checkout
        uses: actions/checkout@<PINNED-SHA>  # e.g., de0fac2e4500dabe0009e67214ff5f5447ce83dd

      - name: Build artifact
        # Replace this block with your actual build step.
        # For Rust: `cargo build --release`
        # For Python: `python -m build`
        # For Node: `npm run build`
        # For Go: `go build -o dist/ ./...`
        run: |
          echo "TODO: replace with your build"
          mkdir -p dist
          echo "build artifact" > dist/artifact

      - name: Attest build provenance
        # The official SLSA Build L3 action.
        # Pin to a specific SHA — find latest at:
        # https://github.com/actions/attest-build-provenance/releases
        uses: actions/attest-build-provenance@<PINNED-SHA>  # e.g., v1.3.0 → specific SHA
        with:
          subject-name: ${{ github.repository }}
          subject-digest: sha256:$(sha256sum dist/artifact | awk '{print $1}')  # update path
          push-to-registry: false  # set true if you also publish to GHCR

      - name: Upload artifact
        uses: actions/upload-artifact@<PINNED-SHA>
        with:
          name: ${{ github.event.inputs.version || github.ref_name }}-artifact
          path: dist/
```

## How to apply

1. Copy the template above to your repo as `.github/workflows/release-attest.yml`.
2. Replace the `Build artifact` step with your actual build.
3. Pin the SHAs of all `uses:` actions:
   - `actions/checkout` — find latest at https://github.com/actions/checkout/releases
   - `actions/attest-build-provenance` — find latest at https://github.com/actions/attest-build-provenance/releases
   - `actions/upload-artifact` — find latest at https://github.com/actions/upload-artifact/releases
4. Update the `subject-digest` to match your build output path.
5. Commit + push. Tag a `v*` release to trigger.

## S8 score lift

- **0 → 1 (ad-hoc):** workflow file added but not exercised yet.
- **0 → 2 (wired):** workflow runs on a tagged release; SLSA Build provenance attestation is produced and visible in the GitHub UI under the release.
- **0 → 3 (measured):** provenance verification is enforced in CI on PRs (the workflow runs `gh attestation verify` on each release artifact).

## Reference: OmniRoute

OmniRoute is the reference repo for SLSA Build L3 (S8=3). Its pattern: `release.yml` has `id-token: write` and a release flow that produces provenance. The template above is a generic extraction of that pattern, with the official `actions/attest-build-provenance` action that OmniRoute doesn't yet use.

To lift OmniRoute to 3: replace its custom provenance step with the `actions/attest-build-provenance` action in the template (no other changes needed — the perms and runner are already correct).

## How to validate

After applying:
1. `git tag v0.0.1-test && git push --tags` to trigger the workflow
2. GitHub UI → Releases → find the tag → see "Provenance" attached to the release
3. `gh attestation verify <artifact>` (local CLI) to confirm the signature

## Provenance

- **Template version:** 1.0
- **Author:** Phenotype Org holistic audit, 2026-06-16
- **Audit that produced it:** `FLEET-AUDIT-30-PILLAR.md` (S8 P0)
- **Reference repo:** `KooshaPari/OmniRoute` (S8=3)
- **License:** Same as the parent repo
