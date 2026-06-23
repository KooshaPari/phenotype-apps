# ADR-094: No-Process-Termination Governance for forge Procs

**Status:** ACCEPTED
**Date:** 2026-06-22
**Author:** orchestrator (cloud-agent session)
**L5-104 follow-up**
**Refs:**
- Issue: `KooshaPari/phenotype-apps#146`
- Finding: `findings/2026-06-22-forge-db-lock-cascade.md`
- Guard script: `scripts/forge_lock_guard.sh`

---

## Context

Issue #146 reports that `/Users/kooshapari/.claude/forge.db` becomes contended when 2+ subagents are dispatched in parallel. One natural response is to kill stale `forge --conversation-id <uuid>` processes to release SQLite locks. The user has explicitly forbidden this:

> "strictly never allowed to kill ... forge procs. subagents via task tool and if 'locked' → debug → make a gh issue or relevant non src code requiring fix" (2026-06-22)

This ADR codifies that prohibition as fleet-wide governance so it cannot be undone by an eager cleanup script in a future session.

---

## Decision

The orchestrator (and any tooling it produces) **must not** terminate, signal, or otherwise interfere with `forge` processes. Specifically:

1. **No `kill` / `pkill` / `killall`** against any process whose argv matches `forge` or `forge --conversation-id`.
2. **No `SIGTERM` / `SIGKILL` / `SIGINT`** sent to any PID discovered via `pgrep -f forge`.
3. **No `pkill -f "forge"` patterns** in any shell script checked into this monorepo.
4. **No `-9` escalations** as a workaround for SQLite lock contention.
5. **No process-tree traversal** that would propagate signals to a `forge` ancestor (e.g., `kill -- -$PGID`).

The only sanctioned response to a `database is locked` error is:

1. Detect it (`scripts/forge_lock_guard.sh`).
2. Self-throttle (sequential dispatch + `sleep 10` between calls).
3. Document it (this ADR + the finding).

---

## Rationale

1. **User directive.** This is a hard constraint from the org owner. Override of user directives is not permitted under any tier of governance.
2. **Forking semantics.** A `forge` proc is a child of the orchestrator's parent (or a sibling of it). Killing it may orphan the orchestrator's conversation state, lose context, or trip the parent MCP's own watchdog.
3. **Lock is a symptom, not a cause.** The contention lives in the SQLite default-mode configuration. Killing the holder releases the lock briefly but does not fix the next contention, and the killed proc may have held other state (journal, WAL, temp tables) that the next opener does not expect.
4. **Fix is upstream.** Per the issue body, the proper fix is `PRAGMA busy_timeout` + WAL + connection pool in the MCP/forge server. The orchestrator cannot deliver that fix; it can only wait for it.

---

## Consequences

### Positive

- No risk of accidentally killing a `claude` or `codex` parent that shares PID/PID-namespace state with `forge`.
- No risk of breaking the orchestrator's own session continuity.
- Forces the fix to land in the right place (the MCP/forge server), where it has permanent effect.

### Negative

- Sequential dispatch is slower than parallel dispatch. Until the upstream fix lands, throughput per session is bounded by single-subagent latency.
- Operators must self-throttle. The `forge_lock_guard.sh` script is the canonical enforcement point.

### Mitigations

- The guard script automates the self-throttle decision so operators do not need to remember the rule.
- This ADR is referenced from `findings/2026-06-22-forge-db-lock-cascade.md` so any future session that hits `database is locked` finds the policy immediately.

---

## Enforcement

Pre-commit / pre-push hooks in this monorepo **should** (future work) grep for forbidden patterns:

```
\b(kill|pkill|killall)\b.*\bforge\b
pkill\s+-f\s+.*forge
```

If a script under `scripts/` is found to violate this ADR, the commit must be rejected. (Hook implementation deferred — tracked separately. The current PR ships the policy + the guard, not the hook.)

---

## Alternatives considered

1. **Allow `SIGTERM` against `forge` procs older than 24 h.** Rejected: the user directive is absolute. Even stale procs are off-limits.
2. **Move `forge.db` to a tmpfs / per-conversation file.** Rejected: the file path is owned by the MCP server, not by the orchestrator. The orchestrator cannot relocate it.
3. **Rewrite the SQLite layer in a shared library the orchestrator controls.** Out of scope for this issue. The MCP server fix is the right place.
4. **Use `PRAGMA wal_checkpoint(TRUNCATE)` via a sidecar helper.** Rejected: writing to `forge.db` from the orchestrator could mask the real fix and create a new contention surface.

---

## Related

- Issue: `KooshaPari/phenotype-apps#146`
- Finding: `findings/2026-06-22-forge-db-lock-cascade.md`
- Guard: `scripts/forge_lock_guard.sh`
- Canonical SQLite fix pattern: `dagctl.go:222`