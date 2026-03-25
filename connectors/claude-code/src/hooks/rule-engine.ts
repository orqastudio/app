// PreToolUse hook — Write | Edit | Bash
//
// Local safety enforcement: file ownership (plugin manifest) and bash command
// safety checks. No daemon dependency — all evaluation runs in-process.
//
// This replaces the previous daemon-calling adapter. Workflow guard evaluation
// for artifact state transitions is handled by other hooks (artifact-enforcement).

import { readFileSync } from "node:fs";
import { resolve, relative } from "node:path";
import { readInput, outputBlock, outputWarn, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface Violation {
  id: string;
  action: "block" | "warn";
  message: string;
}

interface ManifestFileEntry {
  sourceHash?: string;
  installedHash?: string;
}

interface ManifestPluginEntry {
  files?: Record<string, ManifestFileEntry>;
}

interface ContentManifest {
  plugins?: Record<string, ManifestPluginEntry>;
}

// ---------------------------------------------------------------------------
// Bash safety rules — hardcoded, no daemon needed
// ---------------------------------------------------------------------------

const BASH_SAFETY_RULES: Array<{
  id: string;
  pattern: RegExp;
  action: "block" | "warn";
  message: string;
}> = [
  {
    id: "bash-no-verify",
    pattern: /--no-verify/,
    action: "block",
    message: "Bypassing git hooks with --no-verify is forbidden.",
  },
  {
    id: "bash-force-push",
    pattern: /push\s.*--force(?!\S)/,
    action: "block",
    message: "Force push is forbidden. Use --force-with-lease if necessary.",
  },
  {
    id: "bash-env-cat",
    pattern: /\b(cat|less|more|head|tail|bat)\b.*\.env\b/,
    action: "warn",
    message: "Reading .env files may expose secrets. Proceed with caution.",
  },
  {
    id: "bash-no-gpg-sign",
    pattern: /--no-gpg-sign/,
    action: "block",
    message: "Bypassing GPG signing with --no-gpg-sign is forbidden.",
  },
];

// ---------------------------------------------------------------------------
// File safety rules — glob patterns for sensitive files
// ---------------------------------------------------------------------------

const FILE_BLOCK_PATTERNS: Array<{
  id: string;
  patterns: string[];
  action: "block" | "warn";
  message: string;
}> = [
  {
    id: "file-secrets",
    patterns: [".env", ".env.*", "*.pem", "**/*.key", "**/*.p12", "**/credentials.json"],
    action: "block",
    message: "Do not write to secret/credential files.",
  },
];

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const toolName = hookInput.tool_name ?? "";
  const toolInput = hookInput.tool_input ?? {};
  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";

  // Only evaluate Write, Edit, Bash — allow everything else
  if (!["Write", "Edit", "Bash"].includes(toolName)) {
    outputAllow();
  }

  const violations: Violation[] = [];

  // --- Bash safety checks ---
  if (toolName === "Bash") {
    const command = toolInput.command ?? "";
    for (const rule of BASH_SAFETY_RULES) {
      if (rule.pattern.test(command)) {
        violations.push({ id: rule.id, action: rule.action, message: rule.message });
      }
    }
  }

  // --- File-related checks (Write / Edit) ---
  if (["Write", "Edit"].includes(toolName) && toolInput.file_path) {
    const filePath = toolInput.file_path;

    // 1. Plugin ownership check
    const ownershipViolation = checkPluginOwnership(filePath, projectDir);
    if (ownershipViolation) {
      violations.push(ownershipViolation);
    }

    // 2. Sensitive file pattern check
    const relPath = toRelativePath(filePath, projectDir);
    for (const rule of FILE_BLOCK_PATTERNS) {
      for (const pattern of rule.patterns) {
        if (globMatches(pattern, relPath)) {
          violations.push({ id: rule.id, action: rule.action, message: rule.message });
          break; // one match per rule is enough
        }
      }
    }
  }

  // --- Build result ---
  const hasBlock = violations.some((v) => v.action === "block");
  const action = violations.length === 0 ? "allow" : hasBlock ? "block" : "warn";
  const messages = violations.map((v) => v.message);

  logTelemetry("rule-engine", "PreToolUse", startTime, action, {
    tool: toolName,
    violations_found: violations.length,
    action,
  }, projectDir);

  if (action === "block") {
    outputBlock(messages);
  } else if (action === "warn" && messages.length > 0) {
    outputWarn(messages);
  }

  process.exit(0);
}

// ---------------------------------------------------------------------------
// Plugin ownership check
// ---------------------------------------------------------------------------

/**
 * Check if a file is owned by a plugin (listed in .orqa/manifest.json).
 * Returns a block violation if the file is plugin-owned.
 */
function checkPluginOwnership(filePath: string, projectDir: string): Violation | null {
  const relPath = toRelativePath(filePath, projectDir);
  const manifestPath = resolve(projectDir, ".orqa", "manifest.json");

  let manifest: ContentManifest;
  try {
    const raw = readFileSync(manifestPath, "utf-8");
    manifest = JSON.parse(raw) as ContentManifest;
  } catch {
    // No manifest or unparseable — no ownership to enforce
    return null;
  }

  const plugins = manifest.plugins ?? {};
  for (const [pluginName, entry] of Object.entries(plugins)) {
    const files = entry.files ?? {};
    if (relPath in files) {
      return {
        id: "plugin-ownership",
        action: "block",
        message: `This file is owned by plugin ${pluginName}. Edit the plugin source and run 'orqa plugin refresh' instead.`,
      };
    }
  }

  return null;
}

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/**
 * Normalise a file path to a forward-slash relative path from the project root.
 */
function toRelativePath(filePath: string, projectDir: string): string {
  const abs = resolve(filePath);
  const projAbs = resolve(projectDir);
  return relative(projAbs, abs).replace(/\\/g, "/");
}

// ---------------------------------------------------------------------------
// Minimal glob matcher (ported from Rust validation crate)
// ---------------------------------------------------------------------------

/**
 * Minimal glob matcher supporting:
 *   ?  — one character (not /)
 *   *  — zero or more characters (not /)
 *   ** — zero or more characters including /
 *
 * When ** is followed by /, the slash is consumed as part of the double-star.
 */
function globMatches(pattern: string, path: string): boolean {
  return globMatchImpl(pattern, path);
}

function globMatchImpl(p: string, s: string): boolean {
  if (p.length === 0) return s.length === 0;

  // Double-star
  if (p.startsWith("**")) {
    let restP = p.slice(2);
    if (restP.startsWith("/")) restP = restP.slice(1);
    if (restP.length === 0) return true;

    for (let i = 0; i <= s.length; i++) {
      if (globMatchImpl(restP, s.slice(i))) return true;
    }
    return false;
  }

  // Single-star
  if (p[0] === "*") {
    const restP = p.slice(1);
    for (let i = 0; i <= s.length; i++) {
      if (s[i] === "/") break;
      if (globMatchImpl(restP, s.slice(i))) return true;
    }
    return false;
  }

  // ? wildcard
  if (p[0] === "?") {
    if (s.length === 0 || s[0] === "/") return false;
    return globMatchImpl(p.slice(1), s.slice(1));
  }

  // Literal character
  if (s.length === 0 || p[0] !== s[0]) return false;
  return globMatchImpl(p.slice(1), s.slice(1));
}

main().catch(() => process.exit(0));
