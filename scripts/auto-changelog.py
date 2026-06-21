#!/usr/bin/env python3
"""
auto-changelog.py — L67 CHANGELOG automation (V19-T4, 2026-06-21).

Parses Conventional Commits between the last release tag and HEAD, and
generates (or refreshes) an "Unreleased" section in CHANGELOG.md.

Spec: https://www.conventionalcommits.org/en/v1.0.0/
Allowed types: feat, fix, chore, docs, refactor, test, build, ci, perf, revert
  (mirrors .commitlintrc.json type-enum.)

Companion: cliff.toml (used by tag-push `changelog.yml` for the full
regeneration on release). This script is the *PR-driven*, push-to-main
counterpart — it proposes only the "Unreleased" section so reviewers can
eyeball the diff before merge.

Fallbacks
---------
1. Non-conventional commits (no `<type>(<scope>): <subject>` prefix) are
   grouped under a dedicated "Other / Uncategorized" bucket rather than
   dropped, so no work is silently lost.
2. If no tags exist, the script parses all commits reachable from HEAD
   and warns via stderr; CI marks the run yellow but still emits the PR.
3. If the existing CHANGELOG.md already has a populated "Unreleased"
   section, the script refreshes it in place (idempotent) rather than
   duplicating.

Exit codes
----------
0  success (CHANGELOG.md updated or no-op when empty diff)
1  invocation error
2  git command failure (e.g. corrupt history)
3  CHANGELOG.md write failure

Stdlib-only (no third-party deps), per ADR-024 substrate quality bar.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass
from datetime import date
from pathlib import Path
from typing import Iterable

# ---------------------------------------------------------------------------
# Conventional Commits — types allowed by .commitlintrc.json
# ---------------------------------------------------------------------------

# Order matters: drives the rendered section order.
SECTION_ORDER: list[str] = [
    "Features",
    "Bug Fixes",
    "Performance",
    "Refactoring",
    "Documentation",
    "Testing",
    "Build",
    "CI",
    "Reverts",
    "Other / Uncategorized",
]

# Display map: human-friendly title + emoji (matches cliff.toml convention).
SECTION_DISPLAY: dict[str, str] = {
    "Features": "Features",
    "Bug Fixes": "Bug Fixes",
    "Performance": "Performance",
    "Refactoring": "Refactoring",
    "Documentation": "Documentation",
    "Testing": "Testing",
    "Build": "Build",
    "CI": "Continuous Integration",
    "Reverts": "Reverts",
    "Other / Uncategorized": "Other / Uncategorized (non-conventional commits)",
}

# Conventional type -> section bucket. Mirrors .commitlintrc.json type-enum.
TYPE_TO_SECTION: dict[str, str] = {
    "feat": "Features",
    "fix": "Bug Fixes",
    "perf": "Performance",
    "refactor": "Refactoring",
    "docs": "Documentation",
    "test": "Testing",
    "build": "Build",
    "ci": "CI",
    "revert": "Reverts",
    "chore": "CI",  # chores are noisy in changelogs; bucket with CI per cliff.toml pattern
}

# Conventional Commits regex (1.0.0). Strict enough to be safe, loose enough
# to accept the L5-123 governance batch which uses double scopes
# ("feat(security,perf): …") and inline PR refs ("(#47)").
CC_RE = re.compile(
    r"^(?P<type>[a-zA-Z]+)"          # type (feat, fix, chore, …)
    r"(?:\((?P<scope>[^)]*)\))?"      # optional (scope, possibly comma-separated)
    r"(?P<breaking>!)?: "             # optional breaking-change bang, then ': '
    r"(?P<subject>.+?)\s*"           # subject (non-greedy)
    r"(?:\(#(?P<pr>\d+)\))?\s*$"      # optional trailing PR ref (#NNN)
)

# Commit filters — mirrors cliff.toml `commit_filters`. Lowercase match.
SKIP_SUBSTRINGS: tuple[str, ...] = (
    "skip changelog",
    "skip-changelog",
    "[skip changelog]",
)

HEADER_RE = re.compile(r"^## \[?Unreleased\]?", re.IGNORECASE | re.MULTILINE)


# ---------------------------------------------------------------------------
# Data model
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class Commit:
    sha: str
    short_sha: str
    subject: str
    body: str
    section: str
    breaking: bool
    pr_number: str | None
    is_conventional: bool

    def render(self, repo_url: str | None) -> str:
        """Render as a single bullet line."""
        prefix = "**BREAKING** " if self.breaking else ""
        sha_link = self._format_sha_link(repo_url)
        line = f"- {prefix}{self.subject}"
        if sha_link:
            line += f" ([`{self.short_sha}`]({sha_link}))"
        if self.body:
            # Indent body so it nests under the bullet in rendered Markdown.
            excerpt = self._excerpt(self.body)
            if excerpt:
                line += f"\n  > {excerpt}"
        return line

    def _format_sha_link(self, repo_url: str | None) -> str | None:
        if not repo_url:
            return None
        return f"{repo_url.rstrip('/')}/commit/{self.sha}"

    @staticmethod
    def _excerpt(body: str, max_lines: int = 4) -> str:
        """First N non-empty body lines, single-spaced."""
        lines = [ln.strip() for ln in body.splitlines() if ln.strip()]
        if not lines:
            return ""
        excerpt = " ".join(lines[:max_lines])
        # Markdown links inside body can break list nesting — leave as-is.
        return excerpt[:280] + ("…" if len(excerpt) > 280 else "")


# ---------------------------------------------------------------------------
# Git helpers
# ---------------------------------------------------------------------------


def run_git(args: list[str], cwd: Path) -> str:
    """Run git, return stdout, raise on non-zero exit."""
    proc = subprocess.run(
        ["git", *args],
        cwd=str(cwd),
        capture_output=True,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(
            f"git {' '.join(args)} failed (rc={proc.returncode}): "
            f"{proc.stderr.strip()}"
        )
    return proc.stdout


def find_last_release_tag(cwd: Path) -> str | None:
    """Return the most recent tag reachable from HEAD, or None."""
    # Prefer annotated tags; fall back to lightweight.  Most recent first.
    try:
        out = run_git(
            ["tag", "--sort=-version:refname", "--merged", "HEAD"], cwd=cwd
        )
    except RuntimeError:
        return None
    tags = [t for t in out.splitlines() if t.strip()]
    return tags[0] if tags else None


def iter_commits(cwd: Path, since: str | None) -> Iterable[dict[str, str]]:
    """Yield raw commit dicts between `since` and HEAD.

    Uses a unique separator (%x00) so multi-line bodies survive parsing.
    """
    rev_range = f"{since}..HEAD" if since else "HEAD"
    fmt = "%H%x00%h%x00%s%x00%b%x00%EOT"
    try:
        out = run_git(
            ["log", "--no-merges", f"--format={fmt}", rev_range], cwd=cwd
        )
    except RuntimeError as exc:
        print(f"warn: {exc}", file=sys.stderr)
        return iter(())

    commits: list[dict[str, str]] = []
    for block in out.split("EOT"):
        block = block.strip("\n")
        if not block:
            continue
        parts = block.split("\x00")
        if len(parts) < 4:
            continue
        sha, short_sha, subject, body = parts[0], parts[1], parts[2], parts[3]
        commits.append(
            {"sha": sha, "short_sha": short_sha, "subject": subject, "body": body}
        )
    return commits


# ---------------------------------------------------------------------------
# Parsing & grouping
# ---------------------------------------------------------------------------


def should_skip(subject: str) -> bool:
    s = subject.lower()
    return any(skip in s for skip in SKIP_SUBSTRINGS)


def parse_commit(raw: dict[str, str]) -> Commit | None:
    """Parse a raw commit dict into a Commit, or None if it should be skipped."""
    subject = raw["subject"]
    if should_skip(subject):
        return None

    m = CC_RE.match(subject)
    if not m:
        # Fallback bucket: non-conventional commits still surface in the PR.
        return Commit(
            sha=raw["sha"],
            short_sha=raw["short_sha"],
            subject=subject,
            body=raw["body"],
            section="Other / Uncategorized",
            breaking=False,
            pr_number=None,
            is_conventional=False,
        )

    ctype = m.group("type").lower()
    breaking = bool(m.group("breaking")) or "BREAKING CHANGE" in raw["body"].upper()
    section = TYPE_TO_SECTION.get(ctype, "Other / Uncategorized")

    return Commit(
        sha=raw["sha"],
        short_sha=raw["short_sha"],
        subject=m.group("subject").strip(),
        body=raw["body"],
        section=section,
        breaking=breaking,
        pr_number=m.group("pr"),
        is_conventional=True,
    )


def group_commits(commits: list[Commit]) -> dict[str, list[Commit]]:
    grouped: dict[str, list[Commit]] = defaultdict(list)
    for c in commits:
        grouped[c.section].append(c)
    return grouped


# ---------------------------------------------------------------------------
# CHANGELOG.md manipulation
# ---------------------------------------------------------------------------


def build_unreleased_section(
    grouped: dict[str, list[Commit]],
    since_tag: str | None,
    head_sha: str,
    today: date,
    repo_url: str | None,
) -> str:
    """Render the markdown block that goes under `## [Unreleased]`."""
    total = sum(len(v) for v in grouped.values())
    since_part = f"…{since_tag}" if since_tag else "history start"
    lines: list[str] = []
    lines.append(f"## [Unreleased] - {today.isoformat()}")
    lines.append("")
    lines.append(
        f"_Auto-generated by `scripts/auto-changelog.py` "
        f"(V19-T4, L67) — {total} commit(s) since `{since_part}` → `{head_sha[:7]}`._"
    )
    lines.append("")

    if total == 0:
        lines.append("_No conventional commits since the last release tag._")
        lines.append("")
        return "\n".join(lines)

    for section in SECTION_ORDER:
        commits = grouped.get(section, [])
        if not commits:
            continue
        lines.append(f"### {SECTION_DISPLAY[section]}")
        lines.append("")
        for c in commits:
            lines.append(c.render(repo_url))
        lines.append("")
    return "\n".join(lines)


def upsert_unreleased(changelog_text: str, new_section: str) -> str:
    """Replace the existing Unreleased section (if any) with `new_section`.

    If `## [Unreleased]` does not exist, insert it directly under the header
    preamble ("# Changelog" + "All notable changes…").
    """
    if HEADER_RE.search(changelog_text):
        # Replace the existing Unreleased header and everything up to the
        # next `## [` heading OR end of file.
        pattern = re.compile(
            r"^## \[?Unreleased\]?.*?(?=^## \[|\Z)",
            re.IGNORECASE | re.MULTILINE | re.DOTALL,
        )
        return pattern.sub(new_section.rstrip() + "\n\n", changelog_text, count=1)

    # No Unreleased section yet — splice under the header preamble.
    header_match = re.match(
        r"(# Changelog\s*\n+All notable changes[^\n]*\n*)",
        changelog_text,
        re.IGNORECASE,
    )
    if header_match:
        prefix = header_match.group(1)
        return prefix + "\n" + new_section.rstrip() + "\n\n" + changelog_text[len(prefix):]
    # No recognizable header — prepend.
    return new_section.rstrip() + "\n\n" + changelog_text


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    p = argparse.ArgumentParser(
        prog="auto-changelog.py",
        description="Generate / refresh the [Unreleased] section of CHANGELOG.md "
        "from Conventional Commits since the last release tag.",
    )
    p.add_argument(
        "--changelog",
        default="CHANGELOG.md",
        help="Path to CHANGELOG.md (default: CHANGELOG.md).",
    )
    p.add_argument(
        "--since",
        default=None,
        help="Override the lower-bound ref (default: latest tag reachable from HEAD).",
    )
    p.add_argument(
        "--repo-url",
        default=os.environ.get("PHENO_REPO_URL"),
        help="Repository URL for commit links "
        "(default: $PHENO_REPO_URL or empty for plain SHAs).",
    )
    p.add_argument(
        "--dry-run",
        action="store_true",
        help="Print the new [Unreleased] block to stdout instead of writing the file.",
    )
    p.add_argument(
        "--quiet",
        action="store_true",
        help="Suppress informational output on stderr.",
    )
    return p.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    cwd = Path.cwd()
    log = (lambda *a, **kw: None) if args.quiet else (lambda *a, **kw: print(*a, file=sys.stderr, **kw))

    changelog_path = Path(args.changelog)
    if not changelog_path.exists():
        # Create an empty changelog file with the canonical header preamble.
        log(f"info: {changelog_path} does not exist; creating with header preamble.")
        changelog_path.write_text(
            "# Changelog\n\nAll notable changes to this project will be documented in this file.\n",
            encoding="utf-8",
        )

    since = args.since
    if since is None:
        since = find_last_release_tag(cwd)
    log(f"info: lower-bound ref = {since or '(history start)'}")

    raw_commits = list(iter_commits(cwd, since))
    log(f"info: parsed {len(raw_commits)} commit(s) from git log")

    parsed: list[Commit] = []
    for raw in raw_commits:
        c = parse_commit(raw)
        if c is not None:
            parsed.append(c)
    log(f"info: {len(parsed)} commit(s) after skip-filter")

    grouped = group_commits(parsed)
    head_sha = run_git(["rev-parse", "HEAD"], cwd=cwd).strip()
    today = date.today()

    new_section = build_unreleased_section(
        grouped=grouped,
        since_tag=since,
        head_sha=head_sha,
        today=today,
        repo_url=args.repo_url,
    )

    if args.dry_run:
        sys.stdout.write(new_section)
        return 0

    try:
        original = changelog_path.read_text(encoding="utf-8")
    except OSError as exc:
        print(f"error: failed to read {changelog_path}: {exc}", file=sys.stderr)
        return 3

    updated = upsert_unreleased(original, new_section)
    if updated == original:
        log("info: no diff — CHANGELOG.md already up-to-date.")
        return 0

    try:
        changelog_path.write_text(updated, encoding="utf-8")
    except OSError as exc:
        print(f"error: failed to write {changelog_path}: {exc}", file=sys.stderr)
        return 3

    log(f"info: wrote {changelog_path} ({len(updated) - len(original):+d} bytes).")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        sys.exit(130)
