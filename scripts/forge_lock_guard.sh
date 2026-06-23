#!/bin/bash
# forge_lock_guard.sh — orchestrator-side detection guard for forge.db lock cascade
#
# Issue: KooshaPari/phenotype-apps#146
# ADR:   docs/adr/2026-06-22/ADR-094-no-process-termination.md
# Finding: findings/2026-06-22-forge-db-lock-cascade.md
#
# PURPOSE
#   Read-only inspector that decides whether it is safe to dispatch subagents
#   in parallel via the forge MCP integration. NEVER modifies, signals, or
#   kills any process. (See ADR-094 — process termination is forbidden.)
#
# EXIT CODES
#   0  SAFE     — proceed with parallel dispatch
#   2  WARN     — proceed sequentially; sleep 10 between dispatches
#   3  BLOCK    — do not dispatch; surface a `database is locked` regression
#   1  ERROR    — guard itself failed (missing tool, unreadable DB path); caller decides
#
# USAGE
#   ./scripts/forge_lock_guard.sh                      # default forge.db location
#   FORGE_DB=/path/to/forge.db ./scripts/forge_lock_guard.sh
#   FAKE_STALE=1 ./scripts/forge_lock_guard.sh         # simulate stale procs (CI / test)
#   MAX_ACTIVE=2 ./scripts/forge_lock_guard.sh        # override the parallel-safe threshold

set -euo pipefail

# ----------------------------------------------------------------------------
# Configuration (env-overridable)
# ----------------------------------------------------------------------------
FORGE_DB="${FORGE_DB:-/Users/kooshapari/.claude/forge.db}"
MAX_ACTIVE="${MAX_ACTIVE:-4}"          # parallel-safe upper bound on active procs
BLOCK_ACTIVE="${BLOCK_ACTIVE:-8}"      # hard block threshold
STALE_SECONDS="${STALE_SECONDS:-7200}" # 2 hours — anything older is "stale"
RECENT_LOCK_SECONDS="${RECENT_LOCK_SECONDS:-30}" # DB mtime within this window = recent contention

# ----------------------------------------------------------------------------
# Helpers
# ----------------------------------------------------------------------------
log() { printf '[forge_lock_guard] %s\n' "$*" >&2; }

# Convert ps etime output ([DD-]HH:MM:SS or MM:SS) to seconds.
# Returns 0 on parse error (treated as "unknown age", which is safer than 0).
etime_to_seconds() {
    local etime="$1"
    local d=0 h=0 m=0 s=0 total=0
    if [[ "$etime" =~ ^([0-9]+)-([0-9]+):([0-9]+):([0-9]+)$ ]]; then
        d=${BASH_REMATCH[1]}; h=${BASH_REMATCH[2]}; m=${BASH_REMATCH[3]}; s=${BASH_REMATCH[4]}
    elif [[ "$etime" =~ ^([0-9]+):([0-9]+):([0-9]+)$ ]]; then
        h=${BASH_REMATCH[1]}; m=${BASH_REMATCH[2]}; s=${BASH_REMATCH[3]}
    elif [[ "$etime" =~ ^([0-9]+):([0-9]+)$ ]]; then
        m=${BASH_REMATCH[1]}; s=${BASH_REMATCH[2]}
    else
        echo 0; return 0
    fi
    total=$(( d*86400 + h*3600 + m*60 + s ))
    echo "$total"
}

# ----------------------------------------------------------------------------
# Signal collection (READ-ONLY — no kill, no signal, no write)
# ----------------------------------------------------------------------------
ACTIVE=0
STALE=0
RECENT_LOCK=0
WAL_PRESENT="unknown"

# Test mode: simulate stale procs without touching the host
if [[ "${FAKE_STALE:-0}" == "1" ]]; then
    log "FAKE_STALE=1 — simulating 6 active + 3 stale procs (no host inspection)"
    ACTIVE=6
    STALE=3
else
    # 1. Count active `forge --conversation-id` procs (pgrep -c is portable)
    if command -v pgrep >/dev/null 2>&1; then
        ACTIVE=$(pgrep -f 'forge --conversation-id' 2>/dev/null | wc -l | tr -d ' ') || ACTIVE=0
    else
        log "WARN: pgrep not available; cannot count forge procs"
    fi

    # 2. Count stale procs (> STALE_SECONDS old)
    if command -v ps >/dev/null 2>&1; then
        while IFS= read -r pid; do
            [[ -z "$pid" ]] && continue
            etime=$(ps -o etime= -p "$pid" 2>/dev/null | tr -d ' ' || true)
            [[ -z "$etime" ]] && continue
            sec=$(etime_to_seconds "$etime")
            if [[ "$sec" -ge "$STALE_SECONDS" ]]; then
                STALE=$(( STALE + 1 ))
            fi
        done < <(pgrep -f 'forge --conversation-id' 2>/dev/null || true)
    fi

    # 3. WAL sidecar presence (rollback-mode confirmation)
    if [[ -e "$FORGE_DB" ]]; then
        if [[ -e "${FORGE_DB}-wal" ]]; then
            WAL_PRESENT="yes"
        else
            WAL_PRESENT="no"
        fi

        # 4. DB mtime recency (recent contention signal)
        if command -v stat >/dev/null 2>&1; then
            now=$(date +%s)
            # macOS stat: -f %m ; GNU stat: -c %Y
            mtime=$(stat -f %m "$FORGE_DB" 2>/dev/null || stat -c %Y "$FORGE_DB" 2>/dev/null || echo 0)
            age=$(( now - mtime ))
            if [[ "$age" -le "$RECENT_LOCK_SECONDS" ]]; then
                RECENT_LOCK=1
            fi
        fi
    else
        log "INFO: $FORGE_DB not present on this host (orchestrator may be Linux; not macOS dev box)"
    fi
fi  # /FAKE_STALE guard

# ----------------------------------------------------------------------------
# Decision
# ----------------------------------------------------------------------------
LEVEL="SAFE"
EXIT=0

# Hard block thresholds
if [[ "$ACTIVE" -ge "$BLOCK_ACTIVE" ]]; then
    LEVEL="BLOCK"
    EXIT=3
# Soft warn thresholds
elif [[ "$ACTIVE" -ge "$MAX_ACTIVE" ]]; then
    LEVEL="WARN"
    EXIT=2
elif [[ "$STALE" -ge 1 ]]; then
    LEVEL="WARN"
    EXIT=2
elif [[ "$RECENT_LOCK" -eq 1 && "$ACTIVE" -ge 2 ]]; then
    LEVEL="WARN"
    EXIT=2
fi

# WAL absent is informational only (do not escalate on its own — fresh dev boxes
# may not have a forge.db at all). It is logged so the operator can correlate
# with `database is locked` errors.
if [[ "$WAL_PRESENT" == "no" && "$LEVEL" == "SAFE" ]]; then
    log "INFO: forge.db is in rollback journal mode (no -wal sidecar). WAL would let concurrent readers coexist with one writer."
fi

# ----------------------------------------------------------------------------
# Report
# ----------------------------------------------------------------------------
cat <<EOF
[forge_lock_guard] decision=$LEVEL exit=$EXIT
  forge_db=$FORGE_DB
  active_forge_procs=$ACTIVE  (parallel_safe<=$MAX_ACTIVE, block>=$BLOCK_ACTIVE)
  stale_forge_procs=$STALE   (stale_threshold=${STALE_SECONDS}s)
  recent_db_write=$RECENT_LOCK (window=${RECENT_LOCK_SECONDS}s)
  wal_sidecar=$WAL_PRESENT
EOF

exit "$EXIT"