# Release Process

How to cut a release for **PhenoCompose**. PhenoCompose is a
multi-language artifact (Rust crate, OCI image, npm + PyPI bindings,
GitHub release). All of them move together on a single git tag.

## Cadence

- **Patch** — as needed, security-driven. Same-day SLA after triage.
- **Minor** — every 4 weeks, aligned to the Phenotype roll-out
  calendar. Feature driven.
- **Major** — ad-hoc, when a breaking API change lands. ADR required.

## Source of truth

- **Conventional Commits** drive every published artifact.
- `cliff.toml` at the repo root renders `CHANGELOG.md` automatically.
- `release-please` (configured in
  `.github/release-please-config.json`) opens a release PR that
  bumps version, regenerates CHANGELOG, and tags the merge commit.

## Tag format

```
v<MAJOR>.<MINOR>.<PATCH>
v2.3.0
v2.3.1
```

Pre-release tags follow SemVer 2.0 build metadata:
`v2.4.0-rc.1`, `v2.4.0-beta.2`.

## Release checklist

1. **Verify CI is green** on `main` for the last 24h.
2. **Open a release PR** via release-please; review the version
   bump and the rendered CHANGELOG section.
3. **Sign-off** from @KooshaPari (CODEOWNERS default) and one
   domain reviewer (Core, Integrations, SDK, or Security).
4. **Merge** the release PR. release-please tags the merge commit
   and publishes the GitHub Release automatically.
5. **Trigger the publish pipeline** — `.github/workflows/release.yml`
   runs on `release: published` and publishes:
   - `cargo publish` to crates.io for every workspace crate.
   - `pnpm publish` to npm for the TypeScript binding.
   - `uv publish` + `maturin` wheel build + PyPI Trusted Publishing
     for the Python binding.
   - `docker buildx build --push` to GHCR for the OCI image.
6. **Verify** every artifact published by checking the GitHub
   Actions summary and running
   `gh release verify <tag>`.

## Provenance

Every release artifact carries:

- **SLSA Build L2 provenance** —
  `.github/workflows/release-attestation.yml` writes an OIDC-signed
  in-toto provenance statement per artifact.
- **SBOM (CycloneDX)** — generated from the
  release-please merge commit and attached to the GitHub Release.

See `docs/slsa.md` for the full provenance contract.

## Rollback

A release can be rolled back in three ways, in increasing severity:

1. **Yanked crates/packages** — `cargo yank`, `npm unpublish`
   (within the npm 72-hour window), `pip` removal request.
2. **Container re-tag** — `docker pull <bad> && docker tag <bad>
   <channel>:latest`, then update the OCI manifest list so the
   `:latest` pointer drops the bad digest.
3. **Git re-tag** — push a new patch tag that points at the previous
   good commit (`git tag -f v2.3.1 <good-sha> && git push --tags -f`).
   Notify downstream consumers via GitHub Security Advisory.

## Hot-fix workflow

For **Critical** security fixes (per `SECURITY.md` § Severity Rating)
the release process is collapsed into ~30 minutes:

1. Land the fix on a private branch.
2. Open the release PR directly from the private branch to
   `release/X.Y` (skipping release-please's auto-bump).
3. After merge, manually tag with the next patch version.
4. release.yml publishes as normal.

## Post-release

1. Announce on `#phenocompose` (Phenotype Discord) and the
   GitHub Discussions → Announcements category.
2. Update `docs/sessions/<latest-session>/STATUS.md` with the
   released version and any rollout caveats.
3. File follow-up issues for any rollout regressions within
   7 days of release.

## References

- `cliff.toml` — changelog generation
- `SECURITY.md` — severity model + SLOs
- `docs/slsa.md` — SLSA Build L2 provenance
- `.github/workflows/release.yml` — publish pipeline
- `.github/workflows/release-attestation.yml` — provenance generator
