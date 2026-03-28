#!/usr/bin/env bash
# OrqaStudio connector — SessionStart hook
#
# Delegates all startup checks to POST /session-start on the daemon.
# The daemon handles installation, graph integrity, git state, session
# continuity, and dogfood detection — returning a structured response.

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"
DAEMON_PORT="${ORQA_PORT_BASE:-9120}"

# ─── Session Guard ───────────────────────────────────────────────────────────
GUARD="$PROJECT_DIR/.state/.session-started"
if [ -f "$GUARD" ]; then exit 0; fi
mkdir -p "$PROJECT_DIR/.state"

# ─── Delegate to Daemon ──────────────────────────────────────────────────────
response=$(curl -s -X POST "http://127.0.0.1:${DAEMON_PORT}/session-start" \
  -H "Content-Type: application/json" \
  -d "{\"project_path\": \"${PROJECT_DIR}\"}" \
  --connect-timeout 3 --max-time 10 2>/dev/null || true)

if [ -z "$response" ]; then
  MSG="OrqaStudio daemon is not running. Rule enforcement requires the daemon.\\n\\n"
  MSG="${MSG}Start it with: orqa daemon start\\n"
  MSG="${MSG}Daemon expected on port ${DAEMON_PORT} (ORQA_PORT_BASE, default 9120)."
  printf '{"hookSpecificOutput":{"permissionDecision":"deny"},"systemMessage":"%s"}' "$MSG" >&2
  exit 2
fi

# ─── Format Response ─────────────────────────────────────────────────────────
python3 - "$response" <<'EOF'
import json, sys
d = json.loads(sys.argv[1])
failed = [c["message"] for c in d.get("checks", []) if not c["passed"]]
if failed: print("SETUP ISSUES:\n" + "\n".join(failed) + "\n")
if d.get("warnings"): print("WARNINGS:\n" + "\n".join(d["warnings"]) + "\n")
if d.get("migration_context"):
    print(f"═══ MIGRATION CONTEXT ═══\n{d['migration_context']}\n═══ END MIGRATION CONTEXT ═══\n")
if d.get("session_state"):
    print(f"═══ PREVIOUS SESSION STATE ═══\n{d['session_state']}\n═══ END SESSION STATE ═══\n")
    print("Read the session state above. Resume where the previous session left off.\n")
if d.get("governance_context"): print(f"GOVERNANCE CONTEXT:\n{d['governance_context']}\n")
if d.get("checklist"):
    print("SESSION START:\n" + "\n".join(f"{i+1}. {v}" for i, v in enumerate(d["checklist"])))
EOF

touch "$GUARD"
exit 0
