#!/usr/bin/env bash
# OrqaStudio connector — SessionStart hook
#
# 1. Verify connector installation (symlinks, plugin state)
# 2. Start daemon if not running
# 3. Run health checks (graph integrity, git state)
# 4. Load session continuity context

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"
ORQA_DIR="$PROJECT_DIR/.orqa"
CLAUDE_DIR="$PROJECT_DIR/.claude"

# ─── Session Guard ───────────────────────────────────────────────────────────
GUARD="$PROJECT_DIR/tmp/.session-started"
if [ -f "$GUARD" ]; then
  exit 0
fi
mkdir -p "$PROJECT_DIR/tmp"
touch "$GUARD"

OUTPUT=""

# ─── Connector Installation Check ────────────────────────────────────────────
# Verify .claude/ directory has required symlinks and structure.
INSTALL_OK=true

if [ ! -L "$CLAUDE_DIR/agents" ] && [ ! -d "$CLAUDE_DIR/agents" ]; then
  OUTPUT="${OUTPUT}SETUP: .claude/agents missing — run orqa install to set up the connector\n"
  INSTALL_OK=false
fi

if [ ! -L "$CLAUDE_DIR/rules" ]; then
  OUTPUT="${OUTPUT}SETUP: .claude/rules symlink missing — run orqa install to set up the connector\n"
  INSTALL_OK=false
fi

if [ ! -f "$CLAUDE_DIR/CLAUDE.md" ] && [ ! -L "$CLAUDE_DIR/CLAUDE.md" ]; then
  OUTPUT="${OUTPUT}SETUP: .claude/CLAUDE.md missing — run orqa install to set up the connector\n"
  INSTALL_OK=false
fi

if [ "$INSTALL_OK" = false ]; then
  OUTPUT="${OUTPUT}\nRun: orqa install\n\n"
fi

# ─── Dev Environment Check ───────────────────────────────────────────────────
# Check if daemon is running. Don't auto-start — `orqa dev` manages lifecycle.
PORT_BASE="${ORQA_PORT_BASE:-10200}"
DAEMON_PORT=$((PORT_BASE + 58))
DAEMON_HEALTHY=false
if curl -sf --max-time 1 "http://127.0.0.1:${DAEMON_PORT}/health" > /dev/null 2>&1; then
  DAEMON_HEALTHY=true
fi

if [ "$DAEMON_HEALTHY" = false ]; then
  OUTPUT="${OUTPUT}DEV ENVIRONMENT NOT RUNNING: Daemon not responding on port ${DAEMON_PORT}.\n"
  OUTPUT="${OUTPUT}Start the dev environment in a separate terminal: orqa dev\n\n"
fi

# ─── Graph Integrity ─────────────────────────────────────────────────────────
if command -v orqa &> /dev/null; then
  ENFORCE_OUTPUT=$(cd "$PROJECT_DIR" && orqa enforce --fix 2>&1 || true)
  if echo "$ENFORCE_OUTPUT" | grep -q "error"; then
    OUTPUT="${OUTPUT}GRAPH INTEGRITY ISSUES:\n${ENFORCE_OUTPUT}\n\n"
  fi
fi

# ─── Demoted Rule Stability ──────────────────────────────────────────────────
# Track stability of inactive (demoted) rules and surface deletion candidates.
CONNECTOR_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
if [ -f "$CONNECTOR_DIR/hooks/scripts/stability-check.mjs" ]; then
  STABILITY_OUTPUT=$(node "$CONNECTOR_DIR/hooks/scripts/stability-check.mjs" "$PROJECT_DIR" 2>&1 || true)
  if [ -n "$STABILITY_OUTPUT" ]; then
    OUTPUT="${OUTPUT}${STABILITY_OUTPUT}\n\n"
  fi
fi

# ─── Git State ───────────────────────────────────────────────────────────────
STASHES=$(cd "$PROJECT_DIR" && git stash list 2>/dev/null || true)
if [ -n "$STASHES" ]; then
  OUTPUT="${OUTPUT}WARNING: Git stashes found:\n${STASHES}\n\n"
fi

CURRENT_BRANCH=$(cd "$PROJECT_DIR" && git branch --show-current 2>/dev/null || true)
if [ "$CURRENT_BRANCH" = "main" ]; then
  UNCOMMITTED=$(cd "$PROJECT_DIR" && git status --short 2>/dev/null | wc -l | tr -d ' ')
  if [ "$UNCOMMITTED" -gt 0 ]; then
    OUTPUT="${OUTPUT}NOTE: ${UNCOMMITTED} uncommitted files on main.\n\n"
  fi
fi

# ─── Session Continuity ─────────────────────────────────────────────────────
# Load persistent migration context (not overwritten by stop hook)
if [ -f "$PROJECT_DIR/tmp/migration-context.md" ]; then
  MIGRATION_CTX=$(cat "$PROJECT_DIR/tmp/migration-context.md")
  OUTPUT="${OUTPUT}═══ MIGRATION CONTEXT ═══\n${MIGRATION_CTX}\n═══ END MIGRATION CONTEXT ═══\n\n"
fi

# Load previous session state
if [ -f "$PROJECT_DIR/tmp/session-state.md" ]; then
  SESSION_STATE=$(cat "$PROJECT_DIR/tmp/session-state.md")
  OUTPUT="${OUTPUT}═══ PREVIOUS SESSION STATE ═══\n${SESSION_STATE}\n═══ END SESSION STATE ═══\n\n"
  OUTPUT="${OUTPUT}Read the session state above. Resume where the previous session left off.\n\n"
fi

if [ -f "$PROJECT_DIR/tmp/governance-context.md" ]; then
  GOV_CONTEXT=$(cat "$PROJECT_DIR/tmp/governance-context.md")
  OUTPUT="${OUTPUT}GOVERNANCE CONTEXT:\n${GOV_CONTEXT}\n\n"
fi

# ─── Dogfood ─────────────────────────────────────────────────────────────────
if [ -f "$ORQA_DIR/project.json" ]; then
  if grep -q '"dogfood"[[:space:]]*:[[:space:]]*true' "$ORQA_DIR/project.json" 2>/dev/null; then
    OUTPUT="${OUTPUT}DOGFOOD MODE ACTIVE: You are editing the app from the CLI.\n"
    OUTPUT="${OUTPUT}- Ensure dev environment is running: orqa dev (in a separate terminal)\n"
    OUTPUT="${OUTPUT}- See RULE-6083347d for dogfood rules\n\n"
  fi
fi

# ─── Checklist ───────────────────────────────────────────────────────────────
OUTPUT="${OUTPUT}SESSION START:\n"
OUTPUT="${OUTPUT}1. Read context above (migration context + session state)\n"
OUTPUT="${OUTPUT}2. Set scope: which epic/task is the focus?\n"
OUTPUT="${OUTPUT}3. Keep tmp/session-state.md up to date as you work\n"

if [ -n "$OUTPUT" ]; then
  echo -e "$OUTPUT"
fi

exit 0
