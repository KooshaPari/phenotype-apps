# forge DB Lock Cascade (issue #146)

**Status:** ROOT-CAUSE-DOCUMENTED
**Date:** 2026-06-22
**Issue:** `KooshaPari/phenotype-apps#146`
**Author:** orchestrator (cloud-agent session)
**Refs:**
- `docs/adr/2026-06-22/ADR-094-no-process-termination.md` (process-termination governance)
- `scripts/forge_lock_guard.sh` (orchestrator-side detection guard — implementation of the workaround)

---

## Summary

`/Users/kooshapari/.claude/forge.db` is a shared SQLite file used by `forge` CLI, `codex` CLI, and the `claude` MCP forge integration. Dispatching 2+ subagents in parallel via the `Task` tool or `dispatch-worker` CLI produces `Error: database is locked` for at least one of them. Sequential dispatch is significantly more reliable.

The fix lives in the **MCP/forge server, not in the orchestrator** (per issue body). This finding documents the root cause and provides an **orchestrator-side detection guard** so the workaround can be enforced without modifying the 3rd-party `forge` binary.

---

## Repro

1. Dispatch 2-5 subagents in parallel via the `Task` tool (or `dispatch-worker` CLI).
2. Observe at least one returns `Error: database is locked` after 5-30 s.
3. The lock persists 5-30 s, then clears.
4. Sequential dispatch succeeds reliably.

### Observed (2026-06-22 17:52 PDT)

- 19 `forge --conversation-id <uuid>` procs (mix of fresh 20-36 m and stale 2-13 h)
- 1 `forge --conversation-id 958643c9-...` from 24+ h ago (prior session, never terminated)
- 1 `claude --dangerously-skip-permissions` (current session)
- `forge.db` is 4096 bytes, **no `-wal` / `-shm` sidecar files**

The absence of `-wal` / `-shm` confirms the database is in the default rollback journal mode (not WAL). Combined with `busy_timeout = 0` (the SQLite default), every concurrent reader/writer that loses the lock race fails immediately with `SQLITE_BUSY`.

---

## Root cause

Three independent SQLite defaults conspire:

1. **Locking model: POSIX advisory locks.** SQLite uses OS file locking (flock on macOS/Linux). When two processes open the same DB and both attempt to write, the second one gets `SQLITE_BUSY`.
2. **`busy_timeout = 0` (instant fail).** The default busy handler returns immediately. There is no wait window for the holder to commit and release the lock.
3. **Rollback journal mode (not WAL).** Without WAL, every writer takes an exclusive lock on the database file. Readers cannot coexist with a writer at all. The `-wal` / `-shm` sidecar files are absent → rollback mode is confirmed.

Parallel subagent dispatch opens N `forge` procs, each opens `forge.db`, each tries to register its `conversation_id` row. Exactly one wins; the rest fail instantly.

---

## Proposed fix (in MCP/forge server, NOT in orchestrator)

The proper fix lives in the `forge` MCP server source. Order of impact:

1. `PRAGMA busy_timeout = 30000` on every connection — converts instant failure into a 30 s wait.
2. `PRAGMA journal_mode = WAL` on first open — allows concurrent readers + a single writer.
3. Connection pool with max 4 concurrent connections per process — bounds the contention.
4. Retry logic with exponential backoff for `SQLITE_BUSY` — belt-and-suspenders for the gap between (1) and (2).

These four changes together eliminate the cascade at the source.

**Reference implementation in this monorepo:** `dagctl.go:222` shows the canonical pattern:

```go
dsn := path + "?_pragma=journal_mode(WAL)&_pragma=busy_timeout(5000)&_pragma=synchronous(NORMAL)"
```

The `forge` MCP server should adopt the same DSN-style PRAGMA application.

---

## Workaround (orchestrator-side, until fix lands)

Until the `forge` MCP server ships the four PRAGMAs above, the orchestrator must self-throttle. The mitigation has three parts:

1. **Pre-dispatch guard.** Run `scripts/forge_lock_guard.sh` before any parallel `Task` dispatch. If the script returns non-zero, the orchestrator aborts the parallel plan and falls back to sequential dispatch.
2. **Concurrency cap.** Limit parallel `Task` calls to 1 at a time when the script warns of elevated risk (>=4 stale `forge --conversation-id` procs).
3. **Backoff between dispatches.** `sleep 10` between dispatches when the lock-guard detects recent `database is locked` failures in the journal.

### Implementation

The guard (`scripts/forge_lock_guard.sh`) inspects:

| Signal | Source | Threshold |
|---|---|---|
| Active `forge --conversation-id` proc count | `pgrep` | `>=4` → WARN, `>=8` → BLOCK |
| `forge.db` `-wal` sidecar present | `ls -la` | absent → WARN (rollback mode confirmed) |
| `forge.db` last-modified recency | `stat -f %m` | modified in last 30 s → WARN (recent contention) |
| Stale procs (>2 h old) | `ps -o etime=` | any → WARN (orphaned lock holders) |

Exit codes:

- `0` — safe to dispatch in parallel
- `2` — WARN; dispatch sequentially, sleep 10 s between calls
- `3` — BLOCK; do not dispatch; investigate stale procs (but **do not kill them** — see ADR-094)

---

## Related

- ADR-094 (this turn): `docs/adr/2026-06-22/ADR-094-no-process-termination.md`
- `scripts/forge_lock_guard.sh` (this turn)
- `dagctl.go:222` (canonical WAL + busy_timeout DSN pattern)
- GitHub issue: `KooshaPari/phenotype-apps#146`
- User directive (2026-06-22): *"strictly never allowed to kill ... forge procs. subagents via task tool and if 'locked' → debug → make a gh issue or relevant non src code requiring fix"*

---

## Verification

To verify the guard works locally:

```bash
# happy path
./scripts/forge_lock_guard.sh; echo "exit=$?"

# simulate stale procs (read-only; does NOT spawn or kill anything)
FAKE_STALE=1 ./scripts/forge_lock_guard.sh; echo "exit=$?"
```

The script is read-only. It inspects `pgrep` and `ls` output only. It never signals, kills, or modifies any process or file.