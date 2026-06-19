# Shard-Lock Protocol — Validated Reference (2026-06-19)

**Status:** VALIDATED (end-to-end pass 2026-06-19 01:48 PDT)
**Author:** Koosha Pari <koosha@phenotype.com> (orchestrator + subagent validation pair)
**Implements:** FLEET_DAG.db-based shard-lock protocol for parallel subagent task execution
**Replaces:** ad-hoc serial orchestrator work, naive file-based `.shard-locks/`, `git worktree` collision races

---

## 1. Why this protocol

A Phenotype monorepo of ~50 sub-repos with a v8 wave of 18 tracks × 160 tasks is too large for a single orchestrator to execute serially. Parallel subagents (forge CLI, task tool, copilot-agent) are required to hit the ~22h wall-clock budget. Without coordination, parallel agents race on shared resources:

- **Two agents claim the same `KWatch` repo** → conflicting commits, lost work
- **An agent crashes mid-task** → its claim is orphaned; subsequent agents fail to acquire
- **An agent claims a task before its dependency is done** → wasted work, rollback pain

The shard-lock protocol solves all three with one durable substrate: the existing `claims` table in `FLEET_DAG.db`.

---

## 2. The 8-step protocol

```
┌──────────────────────────────────────────────────────────────┐
│                 SHARD-LOCK PROTOCOL v1                       │
│                                                              │
│  Database: FLEET_DAG.db                                      │
│  Reaper:   DELETE FROM claims WHERE since < '-15m'          │
│  Heartbeat: UPDATE claims SET since=now()                    │
│  Worktree:  .worktrees/{resource_type}/{resource}/          │
│  Branch:    chore/{resource}-{task_id}-{date}               │
│                                                              │
│  1. REAP orphans                                             │
│  2. CHECK dependencies (edges table)                         │
│  3. ACQUIRE claim (INSERT INTO claims — abort on fail)       │
│  4. CREATE worktree (git worktree add)                       │
│  5. HEARTBEAT every 5 min                                    │
│  6. WORK (on chore/* branch in worktree)                     │
│  7. RELEASE claim (DELETE FROM claims)                       │
│  8. MARK task done/failed (UPDATE tasks SET status)          │
└──────────────────────────────────────────────────────────────┘
```

---

## 3. The `claims` schema (FLEET_DAG.db)

```sql
CREATE TABLE claims (
    resource       TEXT PRIMARY KEY,    -- 'KWatch', 'KWatch:feat/foo', 'task-05-01'
    resource_type  TEXT NOT NULL,       -- 'repo' | 'branch' | 'task'
    agent          TEXT NOT NULL,       -- 'orch-w9-t161' | 'forge-1' | 'agent-final-01'
    since          TEXT NOT NULL,       -- 'YYYY-MM-DD HH:MM:SS' (datetime('now'))
    task           TEXT                 -- 'task-161' (FK into tasks.id)
);
```

The `resource` PK gives atomic ACQUIRE: `INSERT` fails with `UNIQUE constraint failed` if another agent already holds the claim. No `SELECT … FOR UPDATE`, no advisory locks, no race conditions.

---

## 4. The script: `scripts/shard-lock.sh` (333 lines)

### Usage patterns

**Source from subagent shell:**
```bash
source scripts/shard-lock.sh
shard_acquire "repo:KWatch" "task-05-01"
shard_heartbeat "repo:KWatch"          # every 5 min in a loop
shard_work "KWatch" "task-05-01" "git status"
shard_release "KWatch" "task-05-01" "done"
```

**One-shot CLI:**
```bash
./scripts/shard-lock.sh acquire  "repo:KWatch" "task-05-01" "agent-1"
./scripts/shard-lock.sh heartbeat "repo:KWatch"                "agent-1"
./scripts/shard-lock.sh release  "repo:KWatch" "task-05-01" "done" "agent-1"
./scripts/shard-lock.sh run      "KWatch" "task-05-01" "forge -p 'meta-bundle KWatch'"
./scripts/shard-lock.sh status                                # show all claims + DAG progress
./scripts/shard-lock.sh reap                                  # clean orphan claims
```

### Functions (all exported, all idempotent)

| Function | Args | Returns |
|---|---|---|
| `shard_reap_orphans` | — | always 0 (reaps then returns) |
| `shard_check_deps` | `task_id` | 0 if unblocked, 1 if blocked |
| `shard_acquire` | `resource task_id [agent]` | 0 if acquired, 1 if conflict |
| `shard_create_worktree` | `resource task_id [base]` | wt path on stdout |
| `shard_heartbeat` | `resource [agent]` | 0 if updated, 1 if no claim |
| `shard_work` | `resource task_id work_cmd` | exit code of `eval $work_cmd` |
| `shard_release` | `resource task_id [status] [agent]` | 0 (deletes claim, marks task) |
| `shard_run` | `resource task_id work_cmd` | full cycle, bg heartbeat loop |
| `shard_status` | — | claims table + DAG progress (text) |

### Configuration (env vars)

| Var | Default | Meaning |
|---|---|---|
| `SHARD_DB` | `FLEET_DAG.db` | sqlite3 database path |
| `SHARD_ORPHAN_MINUTES` | `15` | claim age threshold for reaper |
| `SHARD_HEARTBEAT_SECONDS` | `300` | heartbeat interval |
| `SHARD_AGENT_ID` | `agent-$$` | agent name (override for parallel subagents) |

---

## 5. The heartbeat bug (postmortem)

**Symptom:** After `shard_acquire` succeeded, `shard_heartbeat` reported `HEARTBEAT FAIL: no active claim` even though the row was visible via direct `sqlite3 FLEET_DAG.db "SELECT * FROM claims"`.

**Root cause:** The original implementation ran the UPDATE and the `SELECT changes()` in two separate `sqlite3` invocations:

```bash
sqlite3 "$SHARD_DB" "UPDATE claims SET since=... WHERE resource=... AND agent=...;"
local updated=$(sqlite3 "$SHARD_DB" "SELECT changes();")  # WRONG: separate connection
```

Each `sqlite3` invocation opens a new database connection. The `changes()` function in the second call referred to the *second* connection's change counter — which was 0 because the second connection only ran a SELECT. WAL isolation hides the UPDATE.

**Fix:** Combine the check + update into a single statement using `RETURNING` (sqlite 3.35+):

```bash
local result=$(sqlite3 "$SHARD_DB" "
    UPDATE claims SET since = datetime('now')
    WHERE resource = '${resource}' AND agent = '${agent}'
    RETURNING since;
" 2>/dev/null | head -1)
if [ -z "$result" ]; then
    echo "[shard] HEARTBEAT FAIL: no active claim for ${resource} by ${agent}"
    return 1
fi
```

`RETURNING` emits the updated row only if the WHERE matched; empty result = no claim.

**Lesson:** Any function in `shard-lock.sh` that needs to know "did my SQL change anything" must use a single statement with `RETURNING` or a CTE that returns the count. Never use a separate `changes()` invocation.

**Validation:** End-to-end pass 2026-06-19 01:48 PDT confirmed all 7 protocol paths:
1. ACQUIRE → claim created
2. HEARTBEAT (with prefix) → updates `since`
3. HEARTBEAT (without prefix) → updates `since` (prefix-stripping is consistent)
4. HEARTBEAT (wrong agent) → correctly rejected (empty result)
5. RELEASE → claim deleted, task marked `done`
6. HEARTBEAT after release → correctly rejected
7. PREVENT DUPLICATE ACQUIRE → `UNIQUE constraint failed` blocks the second INSERT

---

## 6. Integration pattern for parallel subagents

```bash
#!/bin/bash
# In a forge subagent shell
set -eo pipefail
source /Users/kooshapari/CodeProjects/Phenotype/repos/scripts/shard-lock.sh

export SHARD_AGENT_ID="orch-w9-$$"

# 1. Acquire
shard_acquire "repo:KWatch" "task-09-01" || { echo "could not claim KWatch"; exit 1; }

# 2. Create worktree
wt_path=$(shard_create_worktree "KWatch" "task-09-01")
cd "$wt_path"

# 3. Background heartbeat loop
(
  while true; do
    sleep 300
    shard_heartbeat "KWatch" "$SHARD_AGENT_ID" || exit 1
  done
) &
HB_PID=$!
trap "kill $HB_PID 2>/dev/null; shard_release 'KWatch' 'task-09-01' 'failed'" EXIT

# 4. Do the actual work
git pull --rebase origin main
# ... make changes ...
git add -A
git commit -m "feat: meta-bundle KWatch (T9, ADR-035)"
git push origin HEAD:feat/kwatch-meta-bundle

# 5. Release cleanly
kill $HB_PID 2>/dev/null
trap - EXIT
shard_release "KWatch" "task-09-01" "done"
```

The orchestrator (top-level shell) never touches the worktree. Each subagent is fully isolated by worktree + branch + claim. The `claims` table is the single source of truth for "who is doing what."

---

## 7. Operations

### Reap orphans (cron, every 5 min)

```bash
*/5 * * * * cd /Users/kooshapari/CodeProjects/Phenotype/repos && ./scripts/shard-lock.sh reap
```

### Status (manual, any time)

```bash
./scripts/shard-lock.sh status
```

### Force-release a stuck claim (manual)

```bash
sqlite3 FLEET_DAG.db "DELETE FROM claims WHERE resource='KWatch';"
sqlite3 FLEET_DAG.db "UPDATE tasks SET status='ready' WHERE id='task-09-01';"
```

### Watch live claims (tail)

```bash
watch -n 5 'sqlite3 FLEET_DAG.db "SELECT resource, agent, since FROM claims ORDER BY since DESC;"'
```

---

## 8. Comparison vs alternatives

| Approach | ACQ atomicity | Liveness | Orphan handling | Worktree isolation | Reuses DB |
|---|---|---|---|---|---|
| **shard-lock.sh (this)** | PK constraint | heartbeat | reaper | yes | yes |
| `.shard-locks/` files | mkdir atomic | none | stale files | manual | no |
| `git worktree` only | none | none | none | yes | no |
| `flock(1)` per-file | flock | none | stale locks | manual | no |
| `sqlite3` advisory | BEGIN IMMEDIATE | none | manual | manual | yes |
| Serial orchestrator | n/a | n/a | n/a | none | n/a |

---

## 9. Adoption

- **v8 wave (closed 2026-06-19)**: 4 of 18 tracks used the protocol
- **v9 wave (active 2026-06-19)**: 65 active claims visible via `shard_status`
- **Subagent integration**: forge CLI subagents can `source` the script in their shell; the `task` tool subagents can invoke it via `bash -c 'source ... && ...'`

### Reference: live claim snapshot (2026-06-19 01:55 PDT)

```
resource                              type    agent              since                 task
phenotype-lexer-rs                    repo    forge-1            2026-06-19T08:35:15Z  manual
phenotype-sdk                         repo    forge-1            2026-06-19T08:35:15Z  manual
phenotype-bot-framework               repo    forge-1            2026-06-19T08:35:15Z  manual
forgecode                             repo    forge-dev-session  2026-06-19T08:46:37Z  manual
forgecode:feat/session-viewer-perf-v2 branch  forge-dev-session  2026-06-19T08:46:37Z  manual
KooshaPari/phenotype-org-audits       repo    orch-w9-t161       2026-06-19T08:48:34Z  task-161
pheno-predict                         repo    orch-w9-t165       2026-06-19T08:48:34Z  task-165
```

---

## 10. References

- `scripts/shard-lock.sh` — the script (333 lines, 10980 bytes)
- `FLEET_DAG.db` — the database (1.0 MB, schema in `sqlite3 FLEET_DAG.db ".schema"`)
- `findings/2026-06-19-shard-lock-protocol-validated.md` — this file
- `AGENTS.md` — repo governance (the canonical context)
- `plans/2026-06-19-v9-plan-1.0.md` — strategic backlog using this protocol
- `findings/2026-06-19-L5-110-substrate-audit.md` — the L5-110 audit that produced 9 drift findings
