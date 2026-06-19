#!/bin/bash
#=====================================================================
# shard-lock.sh — FLEET_DAG.db shard-lock protocol
#
# Implements the 8-step shard-lock protocol for distributed subagent
# task execution. Each step is a function so subagents can source this
# script and call the functions directly.
#
# Protocol steps:
#   1. REAP orphans
#   2. CHECK dependencies (edges table)
#   3. ACQUIRE claim (INSERT INTO claims — abort on fail)
#   4. CREATE worktree (git worktree add)
#   5. HEARTBEAT every 5 min
#   6. WORK (on chore/* branch in worktree)
#   7. RELEASE claim (DELETE FROM claims)
#   8. MARK task done/failed (UPDATE tasks SET status)
#
# Usage:
#   source scripts/shard-lock.sh
#   shard_acquire "repo:KWatch" "task-05-01"
#   shard_heartbeat "repo:KWatch"
#   ... do work ...
#   shard_release "repo:KWatch" "task-05-01" "done"
#
# Database path (configurable via SHARD_DB env var)
#=====================================================================
set -eo pipefail

SHARD_DB="${SHARD_DB:-FLEET_DAG.db}"
SHARD_ORPHAN_MINUTES="${SHARD_ORPHAN_MINUTES:-15}"
SHARD_HEARTBEAT_SECONDS="${SHARD_HEARTBEAT_SECONDS:-300}"

#--- 1. ORPHAN REAPER ---
# Removes any claims older than SHARD_ORPHAN_MINUTES that the
# claiming agent has abandoned (no heartbeat, no release).
# Safe to call before every ACQUIRE — PK constraint prevents
# deleting live claims (live agent's heartbeat resets the timer).
shard_reap_orphans() {
    sqlite3 "$SHARD_DB" "
        DELETE FROM claims
        WHERE since < datetime('now', '-${SHARD_ORPHAN_MINUTES} minutes');
    "
    echo "[shard] reaped claims older than ${SHARD_ORPHAN_MINUTES}m"
}

#--- 2. DEPENDENCY CHECK ---
# Verifies that all predecessor tasks (from edges) are 'done'
# before allowing a claim on the given task_id.
# Returns 0 if all dependencies are met, 1 otherwise.
shard_check_deps() {
    local task_id="$1"
    local blocked
    blocked=$(sqlite3 "$SHARD_DB" "
        SELECT COUNT(*) FROM edges e
        JOIN tasks t ON e.from_task = t.id
        WHERE e.to_task = '${task_id}' AND t.status != 'done';
    ")
    if [ "$blocked" -gt 0 ]; then
        echo "[shard] BLOCKED: task ${task_id} has ${blocked} unfinished dependencies"
        return 1
    fi
    echo "[shard] deps OK: ${task_id} is unblocked"
    return 0
}

#--- 3. ACQUIRE CLAIM ---
# Atomically acquires a claim on a resource (repo or branch).
# Fails (exit 1) if another agent holds the claim.
# The resource format is: "repo:RepoName" or "branch:RepoName/BranchName".
shard_acquire() {
    local resource="$1"
    local task_id="$2"
    local agent="${3:-${SHARD_AGENT_ID:-agent-$$}}"

    echo "[shard] acquiring claim: resource=${resource} task=${task_id} agent=${agent}"

    # Step 1: reap orphans
    shard_reap_orphans

    # Step 2: check dependencies
    if ! shard_check_deps "$task_id"; then
        echo "[shard] ABORT: dependencies not met for ${task_id}"
        return 1
    fi

    # Determine resource_type from prefix
    local resource_type="repo"
    case "$resource" in
        repo:*)   resource_type="repo"   ; resource="${resource#repo:}" ;;
        branch:*) resource_type="branch" ; resource="${resource#branch:}" ;;
    esac

    # Step 3: acquire (PK constraint on resource gives atomicity)
    sqlite3 "$SHARD_DB" "
        INSERT INTO claims (resource, resource_type, agent, since, task)
        VALUES ('${resource}', '${resource_type}', '${agent}',
                datetime('now'), '${task_id}');
    " 2>/dev/null || {
        echo "[shard] FAIL: resource ${resource} already claimed by another agent"
        sqlite3 "$SHARD_DB" "SELECT agent, since FROM claims WHERE resource='${resource}';"
        return 1
    }

    echo "[shard] ACQUIRED: ${resource} → ${agent} for task ${task_id}"
    return 0
}

#--- 4. CREATE WORKTREE ---
# Creates an isolated git worktree for the claimed resource.
# Worktree path: .worktrees/{resource_type}/{resource}/
# Branch: chore/{resource}-{task_id}-{date}
shard_create_worktree() {
    local resource="$1"
    local task_id="$2"
    local base_branch="${3:-main}"
    local date_slug
    date_slug=$(date +%Y-%m-%d)
    local branch_name="chore/${resource}-${task_id}-${date_slug}"
    local wt_path=".worktrees/${resource_type:-repo}/${resource}/"

    echo "[shard] creating worktree: ${wt_path} branch=${branch_name}"

    mkdir -p "$(dirname "$wt_path")"

    # If worktree already exists, skip
    if [ -d "$wt_path" ]; then
        echo "[shard] worktree already exists at ${wt_path}, reusing"
    else
        git worktree add -b "$branch_name" "$wt_path" "$base_branch" 2>/dev/null || {
            echo "[shard] worktree add failed (branch may exist); trying checkout"
            git worktree add "$wt_path" "$branch_name" 2>/dev/null || {
                echo "[shard] WORKTREE FAIL: could not create worktree"
                return 1
            }
        }
    fi

    echo "[shard] worktree ready at ${wt_path} on branch ${branch_name}"
    echo "$wt_path"
    return 0
}

#--- 5. HEARTBEAT ---
# Updates the 'since' timestamp on an active claim, proving liveness.
# Call this in a loop every SHARD_HEARTBEAT_SECONDS while working.
shard_heartbeat() {
    local resource="$1"
    local agent="${2:-${SHARD_AGENT_ID:-agent-$$}}"
    # Strip prefix if present
    case "$resource" in
        repo:*)   resource="${resource#repo:}" ;;
        branch:*) resource="${resource#branch:}" ;;
    esac

    # Single-statement check + update using RETURNING (sqlite 3.35+)
    # This avoids the cross-connection `changes()` bug where separate
    # sqlite3 invocations have independent change counters.
    local result
    result=$(sqlite3 "$SHARD_DB" "
        UPDATE claims
        SET since = datetime('now')
        WHERE resource = '${resource}' AND agent = '${agent}'
        RETURNING since;
    " 2>/dev/null | head -1)
    if [ -z "$result" ]; then
        echo "[shard] HEARTBEAT FAIL: no active claim for ${resource} by ${agent}"
        return 1
    fi
    echo "[shard] heartbeat: ${resource} @ ${result}"
    return 0
}

#--- 6. WORK (helper) ---
# Changes directory to the worktree and runs the provided command.
# Exports SHARD_RESOURCE, SHARD_TASK_ID, SHARD_AGENT_ID for use
# within the work command.
shard_work() {
    local resource="$1"
    local task_id="$2"
    local work_cmd="$3"

    local wt_path
    wt_path=".worktrees/repo/${resource}/"

    if [ ! -d "$wt_path" ]; then
        echo "[shard] WORK FAIL: worktree ${wt_path} does not exist"
        return 1
    fi

    export SHARD_RESOURCE="$resource"
    export SHARD_TASK_ID="$task_id"
    export SHARD_AGENT_ID="${SHARD_AGENT_ID:-agent-$$}"

    echo "[shard] working: ${task_id} in ${wt_path}"
    cd "$wt_path" || return 1
    eval "$work_cmd"
    local rc=$?
    cd - >/dev/null || true
    echo "[shard] work complete: ${task_id} exit=${rc}"
    return $rc
}

#--- 7+8. RELEASE + MARK ---
# Releases the claim and marks the task's final status in the DAG.
# Status: 'done' or 'failed'
shard_release() {
    local resource="$1"
    local task_id="$2"
    local status="${3:-done}"
    local agent="${4:-${SHARD_AGENT_ID:-agent-$$}}"
    # Strip prefix if present
    case "$resource" in
        repo:*)   resource="${resource#repo:}" ;;
        branch:*) resource="${resource#branch:}" ;;
    esac

    echo "[shard] releasing claim: ${resource} task=${task_id} status=${status}"

    # Step 7: release claim
    sqlite3 "$SHARD_DB" "
        DELETE FROM claims
        WHERE resource = '${resource}' AND agent = '${agent}';
    "

    # Step 8: mark task done/failed
    sqlite3 "$SHARD_DB" "
        UPDATE tasks SET status = '${status}'
        WHERE id = '${task_id}';
    "

    echo "[shard] RELEASED: ${resource} task ${task_id} → ${status}"
    return 0
}

#--- FULL CYCLE (one-shot convenience) ---
# Runs the complete protocol for a single task: acquire → worktree →
# heartbeat loop → work → release.
# Usage:
#   shard_run "KWatch" "task-05-01" "forge -p 'meta-bundle KWatch'"
shard_run() {
    local resource="$1"
    local task_id="$2"
    local work_cmd="$3"
    local agent="${SHARD_AGENT_ID:-agent-$$}"

    export SHARD_AGENT_ID="$agent"

    echo "=============================================="
    echo "[shard] RUN START: ${task_id} on ${resource}"
    echo "=============================================="

    # Acquire
    if ! shard_acquire "repo:${resource}" "$task_id"; then
        echo "[shard] SKIP: could not acquire ${resource}"
        return 1
    fi

    # Create worktree (background heartbeat loop starts)
    local wt_path
    wt_path=$(shard_create_worktree "$resource" "$task_id") || {
        shard_release "$resource" "$task_id" "failed"
        return 1
    }

    # Launch heartbeat loop in background
    (
        while true; do
            sleep "$SHARD_HEARTBEAT_SECONDS"
            shard_heartbeat "$resource" "$agent" || exit 1
        done
    ) &
    local hb_pid=$!
    trap "kill ${hb_pid} 2>/dev/null; shard_release \"${resource}\" \"${task_id}\" \"failed\"" EXIT

    # Do the work
    shard_work "$resource" "$task_id" "$work_cmd"
    local rc=$?

    # Release
    kill "$hb_pid" 2>/dev/null || true
    trap - EXIT
    if [ $rc -eq 0 ]; then
        shard_release "$resource" "$task_id" "done"
    else
        shard_release "$resource" "$task_id" "failed"
    fi

    echo "=============================================="
    echo "[shard] RUN END: ${task_id} → exit=${rc}"
    echo "=============================================="
    return $rc
}

#--- DISPLAY ---
shard_status() {
    echo "=== Active Claims ==="
    sqlite3 "$SHARD_DB" "
        SELECT resource, resource_type, agent, since, task
        FROM claims
        ORDER BY since;
    " -header -column 2>/dev/null || echo "(none)"
    echo ""
    echo "=== DAG Progress ==="
    sqlite3 "$SHARD_DB" "
        SELECT status, COUNT(*) as cnt
        FROM tasks
        GROUP BY status;
    " -header -column
}

#--- MAIN ---
if [ "${BASH_SOURCE[0]}" = "$0" ]; then
    case "${1:-status}" in
        acquire)     shift; shard_acquire "$@" ;;
        heartbeat)   shift; shard_heartbeat "$@" ;;
        release)     shift; shard_release "$@" ;;
        reap)        shift; shard_reap_orphans "$@" ;;
        run)         shift; shard_run "$@" ;;
        status|show) shard_status ;;
        *)
            echo "Usage: $0 {acquire|heartbeat|release|reap|run|status}"
            echo ""
            echo "  acquire <resource> <task_id> [agent]    — claim a resource"
            echo "  heartbeat <resource> [agent]            — send heartbeat"
            echo "  release <resource> <task_id> [status]   — release + mark"
            echo "  reap                                    — reap orphan claims"
            echo "  run <resource> <task_id> <cmd>          — full cycle"
            echo "  status                                  — show DAG state"
            exit 1
            ;;
    esac
fi
