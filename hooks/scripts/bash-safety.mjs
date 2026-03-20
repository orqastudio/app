#!/usr/bin/env node
// PostToolUse hook: checks Bash tool calls for dangerous command patterns.
//
// Reads hook input from stdin (JSON with tool_name, tool_input).
// Blocked patterns: exit 2, write JSON to stderr with permissionDecision: "deny".
// Warn patterns: exit 0, write JSON to stdout with systemMessage.
// Safe patterns: exit 0, no output.

/**
 * @typedef {{ severity: "block" | "warn", id: string, pattern: RegExp, reason: string }} SafetyRule
 */

/** @type {SafetyRule[]} */
const SAFETY_RULES = [
  // --- BLOCK: git commit/push with --no-verify (bypasses pre-commit hooks) ---
  {
    severity: "block",
    id: "bash-no-verify",
    pattern: /git\s+(commit|push)\b[^|&;]*--no-verify/i,
    reason:
      "--no-verify bypasses pre-commit hooks. Fix the underlying issue instead of skipping enforcement.",
  },

  // --- BLOCK: force push to main or master ---
  {
    severity: "block",
    id: "bash-force-push-main",
    pattern: /git\s+push\b[^|&;]*(-f\b|--force\b)[^|&;]*(main|master)\b/i,
    reason:
      "Force pushing to main/master is forbidden. Use a feature branch and PR instead.",
  },
  {
    severity: "block",
    id: "bash-force-push-main-reversed",
    // also catches: git push origin main --force
    pattern: /git\s+push\b[^|&;]*(main|master)\b[^|&;]*(-f\b|--force\b)/i,
    reason:
      "Force pushing to main/master is forbidden. Use a feature branch and PR instead.",
  },

  // --- BLOCK: git reset --hard (destroys uncommitted work) ---
  {
    severity: "block",
    id: "bash-reset-hard",
    pattern: /git\s+reset\b[^|&;]*--hard/i,
    reason:
      "git reset --hard destroys uncommitted changes permanently. Use git stash or git reset --soft instead.",
  },

  // --- BLOCK: git clean -f (deletes untracked files) ---
  {
    severity: "block",
    id: "bash-clean-force",
    pattern: /git\s+clean\b[^|&;]*-[a-zA-Z]*f/i,
    reason:
      "git clean -f permanently deletes untracked files. Verify there is nothing important untracked first.",
  },

  // --- BLOCK: rm -rf on dangerous paths (root, home, cwd) ---
  {
    severity: "block",
    id: "bash-rm-rf-root",
    pattern: /\brm\b[^|&;]*-[a-zA-Z]*r[a-zA-Z]*f[a-zA-Z]*[^|&;]*\s+(\/|~|\.)\s*(?:$|[|&;])/i,
    reason:
      "rm -rf on / ~ or . is catastrophic. Specify an explicit target path.",
  },
  {
    severity: "block",
    id: "bash-rm-rf-root-flags-last",
    // catches: rm / -rf, rm ~ -rf, rm . -rf
    pattern: /\brm\b[^|&;]*\s+(\/|~|\.)\s+[^|&;]*-[a-zA-Z]*r[a-zA-Z]*f/i,
    reason:
      "rm -rf on / ~ or . is catastrophic. Specify an explicit target path.",
  },

  // --- BLOCK: sudo commands (unexpected privilege escalation) ---
  {
    severity: "block",
    id: "bash-sudo",
    pattern: /(?:^|[|&;`\s])\bsudo\b/i,
    reason:
      "sudo commands require explicit user approval. Run the command directly and request elevated access separately if needed.",
  },

  // --- BLOCK: eval with untrusted / variable input ---
  {
    severity: "block",
    id: "bash-eval-variable",
    pattern: /\beval\s+["']?\$[\w{(]/i,
    reason:
      "eval with variable input is a code-injection risk. Avoid eval or ensure input is fully controlled.",
  },

  // --- BLOCK: fork bomb pattern ---
  {
    severity: "block",
    id: "bash-fork-bomb",
    pattern: /:\s*\(\s*\)\s*\{[^}]*:\s*[|&]\s*:\s*&[^}]*\}/,
    reason: "Fork bomb detected. This will exhaust system resources.",
  },

  // --- WARN: git push --force-with-lease (safer but still overwrites remote) ---
  {
    severity: "warn",
    id: "bash-force-with-lease",
    pattern: /git\s+push\b[^|&;]*--force-with-lease/i,
    reason:
      "--force-with-lease is safer than --force but still rewrites remote history. Confirm this is intentional.",
  },

  // --- WARN: rm -rf on specific non-root paths ---
  {
    severity: "warn",
    id: "bash-rm-rf-path",
    // matches rm -rf with a non-trivial path (not just / ~ or .)
    pattern: /\brm\b[^|&;]*-[a-zA-Z]*r[a-zA-Z]*f[a-zA-Z]*[^|&;]*\s+(?!\/\s|~\s|\.\s|\/\s*$|~\s*$|\.\s*$)[^\s|&;]/i,
    reason:
      "rm -rf with a specific path — verify the target directory before executing.",
  },

  // --- WARN: git branch -D (force delete branch) ---
  {
    severity: "warn",
    id: "bash-branch-force-delete",
    pattern: /git\s+branch\b[^|&;]*-[a-zA-Z]*D/,
    reason:
      "git branch -D force-deletes a branch even if it has unmerged commits. Confirm the branch is fully merged.",
  },

  // --- WARN: git checkout -- . or git restore . (discard all working tree changes) ---
  {
    severity: "warn",
    id: "bash-discard-all-changes",
    pattern: /git\s+(?:checkout\s+--\s*\.|restore\s+\.)/i,
    reason:
      "This discards ALL uncommitted working tree changes. Confirm there is nothing unsaved.",
  },

  // --- WARN: SQL DROP TABLE ---
  {
    severity: "warn",
    id: "bash-sql-drop-table",
    pattern: /\bDROP\s+TABLE\b/i,
    reason:
      "DROP TABLE is destructive and irreversible without a backup. Confirm this is intentional.",
  },

  // --- WARN: SQL DELETE FROM without WHERE ---
  {
    severity: "warn",
    id: "bash-sql-delete-no-where",
    // DELETE FROM <table> not followed by WHERE within the same statement segment
    pattern: /\bDELETE\s+FROM\s+\w[\w.]*(?:\s+(?!WHERE\b)[^;]*)?(?:;|$)/i,
    reason:
      "DELETE FROM without a WHERE clause removes all rows. Add a WHERE clause or confirm full-table deletion is intended.",
  },
];

/**
 * Normalise a command string for pattern matching:
 * - collapse newlines and excess whitespace to single spaces
 * - keep the full string so piped/chained segments are still visible
 *
 * @param {string} command
 * @returns {string}
 */
function normaliseCommand(command) {
  return command.replace(/[\r\n]+/g, " ").replace(/\s{2,}/g, " ").trim();
}

/**
 * @param {string} command
 * @returns {{ blocked: SafetyRule[], warned: SafetyRule[] }}
 */
function checkCommand(command) {
  const normalised = normaliseCommand(command);
  const blocked = [];
  const warned = [];
  const seenIds = new Set();

  for (const rule of SAFETY_RULES) {
    if (seenIds.has(rule.id)) continue;
    if (!rule.pattern.test(normalised)) continue;

    seenIds.add(rule.id);
    if (rule.severity === "block") {
      blocked.push(rule);
    } else {
      warned.push(rule);
    }
  }

  return { blocked, warned };
}

async function main() {
  let input = "";
  for await (const chunk of process.stdin) {
    input += chunk;
  }

  /** @type {{ tool_name?: string, tool_input?: { command?: string }, cwd?: string }} */
  let hookInput;
  try {
    hookInput = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  const toolName = hookInput.tool_name || "";
  const command = (hookInput.tool_input || {}).command || "";

  // Only applies to Bash tool calls
  if (toolName !== "Bash") {
    process.exit(0);
  }

  // Empty commands are safe
  if (!command.trim()) {
    process.exit(0);
  }

  const { blocked, warned } = checkCommand(command);

  if (blocked.length === 0 && warned.length === 0) {
    process.exit(0);
  }

  if (blocked.length > 0) {
    // Build a combined message for all violations
    const lines = ["BASH SAFETY — command blocked:"];

    for (const rule of blocked) {
      lines.push(`  [${rule.id}] ${rule.reason}`);
    }

    // Include warn-level hits in the blocked output for context
    if (warned.length > 0) {
      lines.push("Additional warnings:");
      for (const rule of warned) {
        lines.push(`  [${rule.id}] ${rule.reason}`);
      }
    }

    process.stderr.write(
      JSON.stringify({
        hookSpecificOutput: {
          permissionDecision: "deny",
        },
        systemMessage: lines.join("\n"),
      })
    );
    process.exit(2);
  }

  // Warn-only path: allow the command but surface the warnings
  const lines = ["BASH SAFETY — command allowed with warnings:"];
  for (const rule of warned) {
    lines.push(`  [${rule.id}] ${rule.reason}`);
  }

  process.stdout.write(
    JSON.stringify({
      systemMessage: lines.join("\n"),
    })
  );
  process.exit(0);
}

main().catch(() => process.exit(0));
