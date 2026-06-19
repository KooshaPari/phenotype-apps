# PERMISSIONS.md — Subagent Process & Repo Governance

**Date:** 2026-06-19
**Status:** ACTIVE (enforced from this commit forward)
**Refs:** AGENTS.md, ADR-023-agent-effort-governance, ADR-049-app-substrate-drift-detector, FLEET_DAG_v3.db, v8 plan

---

## 1. Subagent Process Tree — STRICT PROHIBITION

**The native `Task` tool is the only allowed way to spawn subagents in this fleet.** Subagent types are `forge` (code, implementation) and `muse` (planning, research). They are managed by the orchestrator and share the parent process's filesystem, network, and (critically) process namespace.

### Forbidden process operations

**ANY agent in this fleet (parent or subagent) is FORBIDDEN from running any of:**

```bash
# ❌ FORBIDDEN — these patterns can match sibling subagents
pkill -f "forge"             # matches subagent Type=forge cmdlines
pkill -f "muse"              # matches subagent Type=muse cmdlines
pkill -f "codex"             # matches codex exec cmdlines
pkill -f "claude"            # matches ALL claude processes (including me!)
pkill -f "agent"             # matches any subagent / agent-runtime

killall forge codex claude   # name-based kill is also forbidden
```

### Required reaping pattern

Subagent reaping MUST be one of:

```bash
# ✅ ALLOWED — PID-targeted (single known process)
kill -TERM <pid>
kill -KILL <pid>

# ✅ ALLOWED — session-UUID-targeted (reaps one subagent only)
ps -ef | grep "<uuid>" | awk '{print $2}' | xargs -r kill -TERM

# ✅ ALLOWED — parent-tree reaping (only the calling agent's own children)
#   Use the orchestrator's own process tree:
pgrep -P <parent_pid> | xargs -r kill -TERM
```

### Why this matters (concrete example)

In the 2026-06-19 02:45 PDT session, a subagent was dispatched via the `Task` tool with `subagent_type=forge`. Another subagent, confused about its role, ran `pkill -9 -f forge`, which:

1. Killed the sibling forge subagent mid-task
2. Killed the 12th codex exec background process
3. Killed the parent's `bash -c` wrapper for the subagent dispatch
4. Made the parent's `posix_spawn` quota look exhausted (it wasn't — the children were reaped, not the parent)

This is the "26+N leaked processes" / "fork failed: resource temporarily unavailable" error pattern. The fix is **never use substring `-f` matching on subagent type names**.

### Audit hook (reap loop)

For interactive-parent sessions that need to clean up their OWN children:

```bash
# Get this session's parent PID
SESSION_PID=$$
# Reap only children of THIS session
pgrep -P $SESSION_PID | xargs -r kill -TERM 2>/dev/null
# Reap any direct grandchildren
pgrep -P $(pgrep -P $SESSION_PID 2>/dev/null) 2>/dev/null | xargs -r kill -TERM 2>/dev/null
```

---

## 2. Repo & Branch Operations

| Operation | Allowed? | Notes |
|-----------|:--------:|-------|
| `git push --force` to `main` | ❌ NO | Use `--force-with-lease` and only on a branch, not main |
| `git push` to `main` without review | ❌ NO | Use a PR (even self-PR with auto-merge) |
| `gh repo delete` on a `KooshaPari/*` repo | ❌ NO | Archive only, then 90-day retention |
| `gh repo archive` on a `KooshaPari/*` repo | ✅ YES | Dmouse92 auth still has this; KooshaPari needs `admin:org` |
| `git reset --hard HEAD~N` | ⚠️ ONLY with stash backup first | `git stash push -u -m "before-reset" && git reset --hard HEAD~N` |
| `rm -rf` on a path | ⚠️ ONLY with `git status` check first | Verify path is `.gitignore`'d or untracked |
| `git add -A` | ❌ NO | Use explicit `git add <path1> <path2> ...` to avoid sweeping staged deletions into a commit |
| `git add .` | ❌ NO | Same reason — sweeps untracked + modified + deletion-cache into one commit |
| `git commit` with `--no-verify` | ❌ NO | The pre-commit hook is the safety net |

---

## 3. SQLite DAG Operations (FLEET_DAG_v3.db)

| Operation | Allowed? | Notes |
|-----------|:--------:|-------|
| Read via `dagctl next` / `dagctl status` | ✅ YES | Always read first |
| Write via `dagctl claim` / `dagctl done` | ✅ YES | Use the CLI, not raw SQL |
| Bulk `UPDATE` / `DELETE` without WHERE | ❌ NO | Use the `*_by_id` functions |
| `DROP TABLE` | ❌ NO | Migrations are append-only |
| `VACUUM` while another process is reading | ⚠️ ONLY with locking check | `lsof FLEET_DAG_v3.db` first |

---

## 4. Subagent Output Validation

Before a subagent's work is accepted by the parent:

1. **Verify the commit exists** in the local repo (`git log -1 --oneline`)
2. **Verify the file is on disk** (`ls -la <path>`) — subagents can claim they wrote to a path that doesn't exist
3. **Verify the tests pass** (don't trust `pytest passed` from a subagent — re-run)
4. **Check the diff size** — a 3,108-file deletion in a "doc commit" is a bug, not a feature

If any check fails, the parent's `verify-before-completion` directive is not satisfied, and the work must be redone by either the same subagent (with a tighter prompt) or a fresh one.

---

## 5. Escalation & Override

In rare cases (disk fill, fork-bomb recovery, the parent itself stuck), the following **ARE allowed** but must be followed by a `findings/<date>-<reason>-recovery.md` document within 24 hours:

- `pkill -9 -P <parent_pid>` — kill ALL children of a specific parent
- `kill -9 1` (init) — NEVER, this is init, you will not recover
- Reboot the machine — only after all subagents and background codex execs are confirmed dead

If you're tempted to do `pkill -f claude` because "there are too many" — that means you've spawned too many subagents. Fix that by reaping via UUID targeting, not by killing the entire fleet.

---

**Enforcement:** This file is checked at every parent commit. Subagents that violate these rules are terminated and their work is discarded (the parent records the violation in a `findings/<date>-subagent-violation.md` doc).
