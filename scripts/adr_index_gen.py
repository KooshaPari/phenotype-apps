#!/usr/bin/env python3
r"""scripts/adr_index_gen.py — Regenerate the canonical docs/adr/INDEX.md.

Authority: v20 cycle-10 71-pillar plan § Track T1 (L1 Architecture
ADR consolidation, score 2.5 → 3.0). See AGENTS.md § "App substrate
placement" and `findings/71-pillar-2026-06-17-schema.md` § AX (L1-L12)
for context.

Usage:
    python3 scripts/adr_index_gen.py                  # writes docs/adr/INDEX.md
    python3 scripts/adr_index_gen.py --out <path>     # custom output path
    python3 scripts/adr_index_gen.py --check         # exit 1 if INDEX.md is stale
    python3 scripts/adr_index_gen.py --quiet         # no banner / progress
"""
from __future__ import annotations

import argparse
import datetime as _dt
import re
import sys
from pathlib import Path
from typing import Iterable, NamedTuple

REPO_ROOT = Path(__file__).resolve().parent.parent
ADR_ROOT = REPO_ROOT / "docs" / "adr"
DEFAULT_OUT = ADR_ROOT / "INDEX.md"

# Matches an ADR-### or ADR-###B reference in body text. Case-insensitive.
ADR_REF_RE = re.compile(r"\bADR-(\d+)(B?)\b", re.IGNORECASE)
ADR_FILENAME_RE = re.compile(r"^ADR-(\d+)(B?)-.+\.md$", re.IGNORECASE)
H1_RE = re.compile(r"^#\s+(.+?)\s*$", re.MULTILINE)
INLINE_STATUS_RE = re.compile(
    r"Status\s*:\s*\**\s*(?:\**\s*)?(?P<status>[A-Za-z][A-Za-z0-9_ \-/()\[\].|]*?)\**"
    r"\s*(?:\([^)]*\))?\s*(?:[—–-]\s*\d{4}-\d{2}-\d{2})?"
    r"(?:\s*\([^)]*\))?\s*$"
)
TABLE_STATUS_RE = re.compile(
    r"\|\s*\**\s*Status\s*\**\s*\|\s*\**\s*(?P<status>[A-Za-z][A-Za-z0-9_ \-/()\[\].|]*?)"
    r"\s*(?:\([^)]*\))?\s*(?:[—–-]\s*\d{4}-\d{2}-\d{2})?"
    r"(?:\s*\([^)]*\))?\s*\|"
)
DATE_INLINE_RE = re.compile(
    r"^\*?\*?Date\*?\*?\s*[:=]\s*(\d{4}-\d{2}-\d{2})",
    re.MULTILINE,
)


class AdrRecord(NamedTuple):
    adr_number: str
    adr_id: str
    title: str
    date: str
    status: str
    crossref_count: int
    relpath: str


def _normalise_status(raw):
    if not raw:
        return "UNKNOWN"
    s = raw.strip()
    upper = s.upper()
    for bucket in ("ACCEPTED", "PROPOSED", "SUPERSEDED", "REJECTED", "DEPRECATED", "DRAFT"):
        if upper == bucket:
            return bucket
    if upper.startswith("ACCEPT"):
        return "ACCEPTED (CLOSED)" if "CLOSED" in upper else "ACCEPTED"
    if upper.startswith("PROPOS"):
        return "PROPOSED"
    if upper.startswith("SUPERSED"):
        return "SUPERSEDED"
    if upper.startswith("REJECT"):
        return "REJECTED"
    if upper.startswith("DEPRECAT"):
        return "DEPRECATED"
    if upper.startswith("DRAFT"):
        return "DRAFT"
    if upper.startswith("CLOSED"):
        return "ACCEPTED (CLOSED)"
    return upper


def discover_adr_files(adr_root):
    if not adr_root.exists():
        return []
    found = []
    for path in sorted(adr_root.rglob("ADR-*.md")):
        if path.name.upper() == "INDEX.MD":
            continue
        if not ADR_FILENAME_RE.match(path.name):
            continue
        found.append(path)
    return found


def parse_adr(path, adr_root):
    rel = path.relative_to(REPO_ROOT)
    m = ADR_FILENAME_RE.match(path.name)
    assert m is not None
    num, suffix = m.group(1), m.group(2)
    adr_id = f"ADR-{num}{suffix}"
    adr_number = f"{num}{suffix}"
    text = path.read_text(encoding="utf-8", errors="replace")
    title_match = H1_RE.search(text)
    if title_match:
        raw_title = title_match.group(1).strip()
        prefix_re = re.compile(rf"^ADR-\d+B?\s*[:—–-]\s*", re.IGNORECASE)
        cleaned = prefix_re.sub("", raw_title, count=1)
        title = cleaned.strip() or raw_title
    else:
        title = path.stem.split("-", 1)[-1].replace("-", " ").title()
    parent = path.parent
    if parent == adr_root:
        date_match = DATE_INLINE_RE.search(text)
        date = date_match.group(1) if date_match else "pre-wave"
    else:
        if re.match(r"^\d{4}-\d{2}-\d{2}$", parent.name):
            date = parent.name
        else:
            date_match = DATE_INLINE_RE.search(text)
            date = date_match.group(1) if date_match else "unknown"
    status = "Unknown"
    for line in text.splitlines():
        tm = TABLE_STATUS_RE.search(line)
        if tm:
            status = tm.group("status").strip()
            break
        im = INLINE_STATUS_RE.search(line)
        if im:
            status = im.group("status").strip()
            break
    status = _normalise_status(status)
    self_id = adr_id.upper()
    self_base = f"ADR-{num}".upper()
    crossref_count = 0
    for ref in ADR_REF_RE.finditer(text):
        ref_num = ref.group(1)
        ref_id = f"ADR-{ref_num}{ref.group(2)}".upper()
        if ref_id == self_id:
            continue
        if ref_id == self_base:
            continue
        crossref_count += 1
    return AdrRecord(adr_number=adr_number, adr_id=adr_id, title=title,
                     date=date, status=status, crossref_count=crossref_count,
                     relpath=str(rel))


def render_index(records, generated_at):
    recs = sorted(records, key=lambda r: (r.date, r.adr_number))
    total = len(recs)
    by_status = {}
    for r in recs:
        by_status[r.status.upper()] = by_status.get(r.status.upper(), 0) + 1
    status_summary = ", ".join(
        f"{count} {status}"
        for status, count in sorted(by_status.items(), key=lambda kv: (-kv[1], kv[0]))
    )
    lines = []
    lines.append("# ADR Index — Canonical Cross-Reference")
    lines.append("")
    lines.append(
        "This is the **single canonical entry point** for every Architecture "
        "Decision Record in the `docs/adr/` tree. It is regenerated by "
        "`scripts/adr_index_gen.py`; do not hand-edit. The CI workflow "
        "`.github/workflows/adr-lint.yml` runs the generator on every PR "
        "that touches `docs/adr/**` or `scripts/adr_*.py` and fails if the "
        "committed `docs/adr/INDEX.md` is out of date."
    )
    lines.append("")
    lines.append(f"**Total ADRs indexed:** {total}")
    lines.append(f"**Status breakdown:** {status_summary}")
    lines.append("")
    lines.append(
        "Authority: v20 cycle-10 71-pillar plan § Track T1 (L1 Architecture "
        "ADR consolidation; score 2.5 → 3.0). Refresh cadence: weekly "
        "Monday 09:00 PDT (per ADR-041). The `adr-lint` workflow provides "
        "a per-PR freshness gate."
    )
    lines.append("")
    lines.append("---")
    lines.append("")
    lines.append("## Index table (chronological, oldest first)")
    lines.append("")
    lines.append("| ADR# | Title | Date | Status | Cross-refs count |")
    lines.append("|------|-------|------|--------|------------------|")
    for r in recs:
        title = r.title
        if len(title) > 110:
            title = title[:107] + "..."
        title = title.replace("|", "\\|")
        link = f"[{r.adr_id}]({r.relpath})"
        lines.append(
            f"| {link} | {title} | {r.date} | {r.status} | "
            f"{r.crossref_count} |"
        )
    lines.append("")
    lines.append("---")
    lines.append("")
    lines.append("## How to use this index")
    lines.append("")
    lines.append("- **Read an ADR.** Click the ADR link in the table to open the file.")
    lines.append(
        "- **Find cross-refs.** The `Cross-refs count` column counts outgoing "
        "ADR-### mentions in this ADR's body (siblings it cites). A high "
        "number (>10) means the ADR is load-bearing in the fleet — read it "
        "before changing it. Run "
        "`python3 scripts/adr_backlink_check.py --verbose` for the incoming "
        "direction (which ADRs cite this one)."
    )
    lines.append(
        "- **Check freshness.** The `adr-lint` CI workflow regenerates this "
        "file and fails if it differs from the committed version. Add a new "
        "ADR + regen = a clean PR."
    )
    lines.append(
        "- **Verify backlinks.** `python3 scripts/adr_backlink_check.py` "
        "walks every ADR file, extracts `ADR-###` mentions, and exits 1 if "
        "any reference points to a non-existent file. Run it locally "
        "before opening a PR."
    )
    lines.append("")
    lines.append("## Conventions")
    lines.append("")
    lines.append(
        "- ADR files live at `docs/adr/<YYYY-MM-DD>/ADR-NNN-slug.md` "
        "(preferred) or `docs/adr/ADR-NNN-slug.md` (legacy root-level; "
        "ADR-006..008 today)."
    )
    lines.append(
        "- Filename pattern: `ADR-<3-digit-number>(B?)-<kebab-slug>.md`. "
        "The optional `B` suffix is reserved for ADRs that disambiguate a "
        "series-number collision (e.g. ADR-050 router rebuild vs ADR-050 "
        "T12-closure)."
    )
    lines.append(
        "- Each ADR SHOULD have a `## Cross-references` section listing "
        "2-5 of the most-relevant sibling ADRs. The `adr-lint` workflow "
        "does not enforce this today; `adr_backlink_check.py --verbose` "
        "will report it as INFO."
    )
    lines.append("")
    lines.append("---")
    lines.append("")
    lines.append("<!-- generated: " + generated_at.isoformat(timespec="seconds") + " -->")
    lines.append("<!-- source: scripts/adr_index_gen.py | do not edit by hand -->")
    lines.append("")
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Regenerate the canonical docs/adr/INDEX.md from ADR files.",
    )
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--root", type=Path, default=ADR_ROOT)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--quiet", action="store_true")
    args = parser.parse_args()
    if not args.root.exists():
        print(f"ERROR: {args.root} does not exist", file=sys.stderr)
        return 2
    files = discover_adr_files(args.root)
    if not args.quiet:
        print(f"[adr_index_gen] discovered {len(files)} ADR file(s) under {args.root}")
    records = [parse_adr(p, args.root) for p in files]
    generated_at = _dt.datetime.now(_dt.timezone.utc)
    body = render_index(records, generated_at)
    if args.check:
        existing = args.out.read_text(encoding="utf-8") if args.out.exists() else ""
        existing_normalized = re.sub(r"<!-- generated: .*? -->", "<!-- generated: ... -->", existing)
        new_normalized = re.sub(r"<!-- generated: .*? -->", "<!-- generated: ... -->", body)
        if existing_normalized == new_normalized:
            if not args.quiet:
                print(f"[adr_index_gen] OK: {args.out} is up to date ({len(records)} ADRs)")
            return 0
        print(f"STALE: {args.out} would change.", file=sys.stderr)
        return 1
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(body, encoding="utf-8")
    if not args.quiet:
        print(f"[adr_index_gen] wrote {args.out} ({len(records)} ADRs)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
