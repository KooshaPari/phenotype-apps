#!/usr/bin/env python3
"""
30-Pillar / 109-Sub-Pillar audit scoring script.

Usage:
  python3 score.py                              # score all repos (JSON to stdout)
  python3 score.py --repo <name>                # score one repo
  python3 score.py --diff <prev.json>           # compare to previous
  python3 score.py --diff <prev.json> --current <cur.json>  # diff two snapshots

READ-ONLY. Does not modify any files outside docs/audits/scripts/.
"""
import argparse, json, os, sys
from pathlib import Path
from collections import Counter

REPOS_ROOT = Path("/Users/kooshapari/CodeProjects/Phenotype/repos")
SKIP_LIST = {"FocalPoint", "AtomsBot", "QuadSGM", "Parpoura"}

def safe_read(p, n=3000):
    f = Path(p)
    if f.exists():
        try: return f.read_text(errors='ignore')[:n]
        except: return ""
    return ""

def score_repo(name):
    p = REPOS_ROOT / name
    if not p.exists():
        return {"mean": None, "scores": {}, "skip_reason": "no_local_dir", "file_flags": {}}
    if name in SKIP_LIST:
        return {"mean": None, "scores": {}, "skip_reason": "user_skip_list", "file_flags": {}}
    flags = {}
    scores = {}

    claude = safe_read(p / "CLAUDE.md", 3000)
    readme = safe_read(p / "README.md", 2000)
    wf_files = list((p / ".github" / "workflows").glob("*.yml")) if (p / ".github" / "workflows").exists() else []
    wf_text = ""
    for wf in wf_files[:5]:
        try: wf_text += wf.read_text(errors='ignore')[:1500]
        except: pass

    flags["claude"] = (p / "CLAUDE.md").exists()
    flags["readme"] = (p / "README.md").exists()
    flags["workflows"] = len(wf_files)
    flags["license"] = (p / "LICENSE").exists()
    flags["coc"] = (p / "CODE_OF_CONDUCT.md").exists()
    flags["sec"] = (p / "SECURITY.md").exists()
    flags["contr"] = (p / "CONTRIBUTING.md").exists()
    flags["codeowners"] = (p / "CODEOWNERS").exists()
    flags["changelog"] = (p / "CHANGELOG.md").exists()
    flags["precommit"] = (p / ".pre-commit-config.yaml").exists()
    flags["gitleaks"] = (p / ".gitleaks.toml").exists() or (p / ".gitleaks").exists()
    flags["zizmor"] = (p / ".zizmor.yml").exists()
    flags["quality_baseline"] = (p / "quality-baseline.json").exists()
    flags["openapi"] = (p / "docs/reference/openapi.yaml").exists() or (p / "openapi.yaml").exists() or (p / "openapi.yml").exists()
    flags["repo_map"] = (p / "docs/architecture/REPOSITORY_MAP.md").exists() or (p / "REPOSITORY_MAP.md").exists()
    flags["journeys"] = (p / "docs/operations/journey-traceability.md").exists()
    flags["i18n"] = (p / "src/i18n/messages").exists() or (p / "locale").exists() or (p / "locales").exists() or (p / "src/locales").exists()
    flags["size_limit"] = (p / ".size-limit.json").exists()
    cargo_text = safe_read(p / "Cargo.toml", 1000)
    flags["cargo_workspace"] = "[workspace]" in cargo_text
    flags["adrs"] = (p / "docs/adr").is_dir() and bool(list((p / "docs/adr").glob("*.md")))
    flags["tests"] = (p / "tests").exists() or (p / "test").exists() or (p / "__tests__").exists()
    flags["e2e"] = (p / "tests/e2e").exists() or (p / "e2e").exists() or (p / "playwright.config.ts").exists() or (p / "playwright.config.js").exists()
    flags["integration"] = (p / "tests/integration").exists() or (p / "integration").exists()
    flags["live"] = (p / "tests/live").exists()
    flags["slsa"] = "attest-build-provenance" in wf_text
    flags["migrations"] = (p / "migrations").exists() or (p / "src/lib/db/migrations").exists() or (p / "db/migrations").exists()
    flags["lockfile"] = (p / "Cargo.lock").exists() or (p / "package-lock.json").exists() or (p / "poetry.lock").exists() or (p / "go.sum").exists()
    flags["benches"] = (p / "benches").exists() or (p / "benchmarks").exists() or "bench" in claude.lower()

    non_sha = 0
    for wf in wf_files:
        try:
            for line in wf.read_text(errors='ignore').split("\n"):
                if "uses:" in line and "@" in line and ("actions/" in line or "github/codeql-action" in line):
                    after = line.split("@")[1].split("'")[0].split('"')[0]
                    if not (len(after) >= 40 and all(c in "0123456789abcdef" for c in after.lower())):
                        non_sha += 1
        except: pass
    flags["sha_pin"] = (len(wf_files) > 0 and non_sha == 0)
    flags["macos_runner"] = "macos" in wf_text
    flags["windows_runner"] = "windows" in wf_text
    flags["codeql"] = "codeql" in wf_text.lower()
    flags["sca"] = "deny" in wf_text.lower() or "audit" in wf_text.lower() or (p / "deny.toml").exists() or (p / ".cargo/audit.toml").exists()
    flags["authz"] = "authz" in claude or "scope" in claude or (p / "src/server/authz").exists() or (p / "authz").exists()
    flags["input_valid"] = "zod" in (claude+readme).lower() or "validate" in claude.lower()
    flags["output_sanit"] = "sanitize" in claude.lower() or "buildErrorBody" in claude
    flags["threat"] = (p / "docs/security/THREAT_MODEL.md").exists() or "threat" in claude.lower()
    flags["release_yml"] = any("release" in wf.name.lower() for wf in wf_files)
    flags["runbooks"] = (p / "docs/runbooks").exists() or (p / "runbooks").exists() or (p / "RUNBOOK.md").exists()
    flags["env_example"] = (p / ".env.example").exists() or (p / ".env.sample").exists() or "zod" in claude.lower()
    flags["secret_zero"] = "zeroize" in claude or "AES-256" in claude or "encrypt" in claude.lower()
    flags["pii"] = "PII" in claude or "scrub" in claude.lower()
    flags["circuit"] = "circuit" in claude.lower() or "breaker" in claude.lower()
    flags["retry"] = "retry" in claude.lower() or "backoff" in claude.lower()
    flags["bulkhead"] = "bulkhead" in claude.lower() or "isolation" in claude.lower()
    flags["pino"] = "pino" in (claude+readme).lower()
    flags["cycles_check"] = "check:cycles" in claude or "check:cycles" in readme
    flags["coverage_cfg"] = (p / "codecov.yml").exists() or (p / ".codecov.yml").exists() or "test:coverage" in claude or "cargo test" in claude or "pytest" in claude or "go test" in claude
    flags["worktree"] = "worktree" in claude.lower()
    flags["branch_prefix"] = "feat/" in claude or "fix/" in claude
    flags["conv_commits"] = "conventional" in claude.lower() or "feat(" in claude
    flags["no_verify_rule"] = "no-verify" in claude or "Husky" in claude
    flags["co_author_rule"] = "AI" in claude or "Co-Authored" in claude
    flags["msrv"] = "MSRV" in claude or "msrv" in claude or "rust-toolchain" in claude
    flags["node_matrix"] = "Node" in claude or "node " in claude.lower() or "node:" in wf_text
    flags["abort"] = "abort" in claude.lower() or "abort" in readme.lower()
    flags["wal"] = "WAL" in claude or "journal" in claude.lower()

    # A
    scores["A1"] = 2 if flags["cargo_workspace"] else (1 if (p / "Cargo.toml").exists() or (p / "package.json").exists() or (p / "pyproject.toml").exists() or (p / "go.mod").exists() else 0)
    scores["A2"] = 2 if flags["adrs"] else 0
    scores["A3"] = 1 if claude else 0
    scores["A4"] = 2 if flags["cycles_check"] else (1 if flags["cargo_workspace"] else 0)
    scores["A5"] = 1 if (claude or readme) else 0
    # X
    scores["X1"] = 3 if flags["quality_baseline"] else (1 if flags["claude"] else 0)
    scores["X2"] = 1 if '"strict": true' in claude else 0
    scores["X3"] = 1 if flags["quality_baseline"] else 0
    scores["X4"] = 0
    scores["X5"] = 1 if flags["quality_baseline"] else 0
    scores["X6"] = 2 if flags["precommit"] else 0
    # D
    scores["D1"] = 1 if flags["journeys"] else 0
    scores["D2"] = 2 if flags["journeys"] else 0
    scores["D3"] = 1 if flags["claude"] else 0
    scores["D4"] = 2 if flags["changelog"] else 0
    scores["D5"] = 3 if flags["openapi"] else (1 if flags["readme"] else 0)
    scores["D6"] = 2 if flags["repo_map"] else 0
    # U
    scores["U1"] = 0
    scores["U2"] = 1 if flags["readme"] else 0
    scores["U3"] = 0
    scores["U4"] = 0
    # UX
    scores["UX1"] = 0; scores["UX2"] = 0; scores["UX3"] = 0
    # AT
    scores["AT1"] = 0; scores["AT2"] = 0; scores["AT3"] = 0
    scores["AT4"] = 3 if flags["i18n"] else 0
    scores["AT5"] = 1 if flags["i18n"] else 0
    # T
    scores["T1"] = 2 if flags["coverage_cfg"] else (1 if flags["tests"] else 0)
    scores["T2"] = 2 if flags["integration"] else 0
    scores["T3"] = 2 if flags["e2e"] else 0
    scores["T4"] = 1 if flags["tests"] else 0
    scores["T5"] = 1 if flags["tests"] else 0
    scores["T6"] = 1 if flags["tests"] else 0
    # P
    scores["P1"] = 2 if flags["benches"] else 0
    scores["P2"] = 0
    scores["P3"] = 2 if flags["size_limit"] else 0
    scores["P4"] = 0
    scores["P5"] = 0
    # S
    scores["S1"] = 2 if flags["codeql"] else (1 if flags["zizmor"] else 0)
    scores["S2"] = 2 if flags["sca"] else 0
    scores["S3"] = 2 if flags["gitleaks"] else 0
    scores["S4"] = 2 if flags["authz"] else 0
    scores["S5"] = 2 if flags["input_valid"] else 0
    scores["S6"] = 2 if flags["output_sanit"] else 0
    scores["S7"] = 1 if flags["threat"] else 0
    scores["S8"] = 2 if flags["slsa"] else 0
    scores["S9"] = 3 if flags["sha_pin"] else 0
    # Q
    n_wf = len(wf_files)
    scores["Q1"] = 2 if (n_wf >= 4 or flags["quality_baseline"]) else (1 if n_wf > 0 else 0)
    scores["Q2"] = 3 if flags["quality_baseline"] else 0
    scores["Q3"] = 1 if flags["quality_baseline"] else 0
    scores["Q4"] = 1 if flags["coverage_cfg"] else 0
    # E
    scores["E1"] = 1 if flags["worktree"] else 0
    scores["E2"] = 1 if flags["branch_prefix"] else 0
    scores["E3"] = 1 if flags["conv_commits"] else 0
    scores["E4"] = 1 if flags["co_author_rule"] else 0
    scores["E5"] = 2 if flags["no_verify_rule"] else 0
    # G
    scores["G1"] = 2 if flags["codeowners"] else 0
    scores["G2"] = 1 if flags["sec"] else 0
    scores["G3"] = 1 if flags["contr"] else 0
    scores["G4"] = 1 if flags["license"] else 0
    scores["G5"] = 1 if flags["coc"] else 0
    scores["G6"] = 1 if flags["changelog"] else 0
    # O
    scores["O1"] = 2 if flags["release_yml"] else 0
    scores["O2"] = 1 if flags["runbooks"] else 0
    scores["O3"] = 0; scores["O4"] = 0; scores["O5"] = 0
    # SC
    scores["SC1"] = 2 if flags["lockfile"] else 0
    scores["SC2"] = 0
    scores["SC3"] = 1 if flags["slsa"] else 0
    scores["SC4"] = 0
    # OB
    scores["OB1"] = 1 if flags["pino"] else 0
    scores["OB2"] = 0; scores["OB3"] = 0; scores["OB4"] = 0
    # C
    scores["C1"] = 0 if (flags["macos_runner"] or flags["windows_runner"]) else 2
    scores["C2"] = 0; scores["C3"] = 0
    # DA
    scores["DA1"] = 2 if flags["migrations"] else 0
    scores["DA2"] = 0; scores["DA3"] = 0
    # RT
    scores["RT1"] = 1 if (flags["node_matrix"] or flags["msrv"]) else 0
    scores["RT2"] = 0
    # RE
    scores["RE1"] = 2 if flags["lockfile"] else 0
    scores["RE2"] = 1 if flags["lockfile"] else 0
    # AP
    scores["AP1"] = 3 if flags["openapi"] else 0
    scores["AP2"] = 1 if flags["tests"] else 0
    # DM
    scores["DM1"] = 1 if (claude or readme) else 0
    scores["DM2"] = 1 if flags["input_valid"] else 0
    # EH
    scores["EH1"] = 1 if flags["input_valid"] else 0
    scores["EH2"] = 2 if flags["output_sanit"] else 0
    # CN
    scores["CN1"] = 0
    scores["CN2"] = 1 if flags["abort"] else 0
    scores["CN3"] = 0
    # PS
    scores["PS1"] = 1 if flags["migrations"] else 0
    scores["PS2"] = 1 if flags["wal"] else 0
    # CF
    scores["CF1"] = 2 if flags["env_example"] else 0
    scores["CF2"] = 1 if flags["secret_zero"] else 0
    # PR
    scores["PR1"] = 1 if flags["pii"] else 0
    scores["PR2"] = 0
    # RL
    scores["RL1"] = 2 if flags["circuit"] else 0
    scores["RL2"] = 2 if flags["retry"] else 0
    scores["RL3"] = 1 if flags["bulkhead"] else 0
    # AS
    scores["AS1"] = 0; scores["AS2"] = 0
    # AU
    scores["AU1"] = 1 if flags["journeys"] else 0
    scores["AU2"] = 2 if flags["adrs"] else 0

    vals = list(scores.values())
    mean = sum(vals) / len(vals) if vals else 0
    return {
        "mean": round(mean, 2),
        "scores": {k: {"score": int(v), "evidence": "auto-presence-check"} for k, v in scores.items()},
        "file_flags": flags,
        "skip_reason": None,
    }


def list_active_repos():
    """Read /tmp/repos-active.txt if present, else enumerate."""
    if os.path.exists("/tmp/repos-active.txt"):
        repos = []
        with open("/tmp/repos-active.txt") as f:
            for line in f:
                parts = line.rstrip("\n").split("\t")
                if len(parts) < 5: continue
                name = parts[0]
                if name in SKIP_LIST: continue
                if name == "OmniRoute": continue  # hand-scored
                repos.append(name)
        return repos
    return [p.name for p in REPOS_ROOT.iterdir() if p.is_dir() and p.name not in SKIP_LIST and p.name != "OmniRoute"]


def diff_scores(prev, cur):
    """Return list of regressions (pillar was >0 in prev, now 0 in cur)."""
    regressions = []
    for repo_name in cur.get("repos", {}):
        if repo_name in SKIP_LIST: continue
        prev_repo = prev.get("repos", {}).get(repo_name)
        cur_repo = cur.get("repos", {}).get(repo_name)
        if not prev_repo or not cur_repo: continue
        for p in cur_repo.get("scores", {}):
            prev_s = prev_repo.get("scores", {}).get(p, {}).get("score", 0)
            cur_s = cur_repo.get("scores", {}).get(p, {}).get("score", 0)
            if prev_s > 0 and cur_s == 0:
                regressions.append({"repo": repo_name, "pillar": p, "prev": prev_s, "cur": cur_s})
    return regressions


def main():
    parser = argparse.ArgumentParser(description="30-Pillar audit scorer")
    parser.add_argument("--repo", help="Score a single repo")
    parser.add_argument("--diff", help="Path to previous scores JSON")
    parser.add_argument("--current", help="Path to current scores JSON (for diff mode)")
    args = parser.parse_args()

    if args.diff and args.current:
        prev = json.load(open(args.diff))
        cur = json.load(open(args.current))
        regs = diff_scores(prev, cur)
        print(json.dumps({"regressions": regs, "n_regressions": len(regs)}, indent=1))
        sys.exit(1 if regs else 0)

    if args.repo:
        result = {args.repo: score_repo(args.repo)}
    else:
        result = {}
        for name in list_active_repos():
            try: result[name] = score_repo(name)
            except Exception as e: result[name] = {"mean": None, "scores": {}, "skip_reason": f"err: {str(e)[:80]}"}

    n_scored = sum(1 for r in result.values() if r.get("mean") is not None)
    print(json.dumps({"scored_at": __import__("datetime").datetime.now().isoformat(), "n_scored": n_scored, "repos": result}, indent=1))


if __name__ == "__main__":
    main()
