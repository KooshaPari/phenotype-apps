#!/usr/bin/env bash
# scripts/release.sh — V21-T3 (L26 release coordination)
#
# End-to-end release runner for the Phenotype monorepo. Bumps the
# [workspace.package] version in the root Cargo.toml, regenerates
# CHANGELOG.md from conventional commits since the previous tag,
# creates an annotated git tag, pushes it to origin, and creates a
# GitHub Release with auto-generated notes via `gh`.
#
# Usage:
#   scripts/release.sh <version> [--ci] [--dry-run]
#
# Arguments:
#   <version>   SemVer (e.g. 0.0.13, 1.2.0-rc.1). Becomes tag v<version>.
#
# Flags:
#   --ci        CI / post-tag mode. Assume steps a-d are already done
#               (tag exists at remote). Only runs step e (create the
#               GitHub Release). Idempotent: safe to re-run.
#   --dry-run   Print what would happen. Do not mutate files, tags,
#               pushes, or releases.
#
# Required tools: git, gh, git-cliff, python3.
# Required: GITHUB_TOKEN or `gh auth login` for step e.

set -euo pipefail

# --- args ----------------------------------------------------------------
if [[ $# -lt 1 ]]; then
  cat >&2 <<EOF
usage: $0 <version> [--ci] [--dry-run]

  <version>   semver, becomes tag v<version>
  --ci        post-tag mode: only create the GitHub release
  --dry-run   print plan; do not mutate

example: $0 0.0.13
example: $0 0.0.13 --dry-run
EOF
  exit 64
fi

VERSION="$1"
shift

CI_MODE=false
DRY_RUN=false
for arg in "$@"; do
  case "$arg" in
    --ci)      CI_MODE=true ;;
    --dry-run) DRY_RUN=true ;;
    *) echo "unknown flag: $arg" >&2; exit 64 ;;
  esac
done

TAG="v${VERSION}"

# --- helpers -------------------------------------------------------------
log() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33mwarn:\033[0m %s\n' "$*" >&2; }
err() { printf '\033[1;31merror:\033[0m %s\n' "$*" >&2; }
die() { err "$*"; exit 1; }

run() {
  if [[ "$DRY_RUN" == "true" ]]; then
    printf '  [dry-run] %s\n' "$*"
  else
    "$@"
  fi
}

# --- preconditions -------------------------------------------------------
log "release.sh: $VERSION  tag=$TAG  ci=$CI_MODE  dry_run=$DRY_RUN"

command -v git      >/dev/null || die "git not found"
command -v python3  >/dev/null || die "python3 not found"
command -v gh       >/dev/null || die "gh not found"
command -v git-cliff >/dev/null || die "git-cliff not found (cargo install git-cliff)"

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[A-Za-z0-9.-]+)?$ ]]; then
  die "version '$VERSION' is not semver (expected MAJOR.MINOR.PATCH[-PRERELEASE])"
fi

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

# Refuse to clobber an existing tag
if git rev-parse "$TAG" >/dev/null 2>&1; then
  die "tag $TAG already exists locally; pick a new version or delete the tag first"
fi

# Working tree must be clean for a/b (CI mode can be dirty)
if [[ "$CI_MODE" == "false" ]] && ! git diff --quiet --ignore-submodules HEAD 2>/dev/null; then
  warn "working tree is dirty; commit or stash before releasing"
  git status --short >&2
  die "refusing to release with dirty tree"
fi

# Refuse to release from a non-main / non-release branch unless explicit
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
case "$CURRENT_BRANCH" in
  main|master|release/*) ;;
  *)
    warn "releasing from non-standard branch '$CURRENT_BRANCH'"
    if [[ "$DRY_RUN" == "false" ]] && [[ "$CI_MODE" == "false" ]]; then
      read -r -p "continue? [y/N] " ans
      [[ "$ans" == "y" || "$ans" == "Y" ]] || die "aborted"
    fi
    ;;
esac

# --- (a) bump [workspace.package] version in Cargo.toml -----------------
LAST_TAG="$(git describe --tags --abbrev=0 2>/dev/null || true)"

if [[ "$CI_MODE" == "true" ]]; then
  log "(a) Cargo.toml version bump: skipped (--ci)"
else
  log "(a) Bumping [workspace.package] version to $VERSION in Cargo.toml"
  if ! grep -q '^\[workspace\.package\]' Cargo.toml; then
    die "Cargo.toml has no [workspace.package] section"
  fi
  run env VERSION="$VERSION" python3 - <<'PY'
import os, re, pathlib, sys
version = os.environ["VERSION"]
p = pathlib.Path("Cargo.toml")
s = p.read_text()
# Match: [workspace.package]\n...version = "..." (stop at the next [section])
pattern = re.compile(
    r'(?ms)(^\[workspace\.package\][^\n]*\n(?:[^\[]*\n)*?version\s*=\s*")[^"]+(")',
)
new, n = pattern.subn(lambda m: m.group(1) + version + m.group(2), s, count=1)
if n == 0:
    sys.exit("could not find [workspace.package] version line in Cargo.toml")
p.write_text(new)
print(f"  wrote Cargo.toml: version = \"{version}\"")
PY
fi

# --- (b) update CHANGELOG.md with git log since last tag -----------------
log "(b) Regenerating CHANGELOG.md (since ${LAST_TAG:-<no prior tag>})"
if [[ ! -f cliff.toml ]]; then
  die "cliff.toml not found at repo root; required for step (b)"
fi
run git-cliff --tag "$TAG" --output CHANGELOG.md

# --- commit version + changelog (only when not --ci) ---------------------
if [[ "$CI_MODE" == "false" ]]; then
  log "Committing version bump + CHANGELOG"
  if ! git diff --quiet -- Cargo.toml CHANGELOG.md 2>/dev/null; then
    run git add Cargo.toml CHANGELOG.md
    run git commit -m "chore(release): $VERSION"
  else
    log "  no changes to commit"
  fi
fi

# --- (c) create annotated tag -------------------------------------------
log "(c) Creating annotated tag $TAG"
run git tag -a "$TAG" -m "Release $VERSION"

# --- (d) push to origin --------------------------------------------------
log "(d) Pushing $CURRENT_BRANCH + $TAG to origin"
run git push origin "$CURRENT_BRANCH" "$TAG"

# --- (e) create GitHub release with auto-generated notes -----------------
log "(e) Creating GitHub release for $TAG"

# If we have a CHANGELOG section for this tag, attach it as the release
# body; otherwise rely on --generate-notes.
NOTES_ARGS=()
if [[ -f CHANGELOG.md ]] && grep -q "## \[${VERSION}\]" CHANGELOG.md; then
  # Extract just the section for this version into a tmp file
  NOTES_FILE="$(mktemp -t release-notes.XXXXXX.md)"
  trap 'rm -f "$NOTES_FILE"' EXIT
  awk -v ver="## [${VERSION}]" '
    $0 ~ ver {found=1}
    found {print}
    found && /^## \[/ && $0 !~ ver {exit}
  ' CHANGELOG.md > "$NOTES_FILE"
  NOTES_ARGS=(--notes-file "$NOTES_FILE")
else
  NOTES_ARGS=(--generate-notes)
fi

run gh release create "$TAG" \
  --title "$TAG" \
  "${NOTES_ARGS[@]}"

log "Release $TAG complete."
log "View at: $(gh release view "$TAG" --json url -q .url 2>/dev/null || echo "<gh release view pending>")"
