// Rule engine: loads active rules with enforcement entries, evaluates patterns
// against tool call context. Used by PreToolUse hook.
//
// Reads hook input from stdin (JSON with tool_name, tool_input).
// Outputs JSON for Claude Code hook system.

import { readFileSync, readdirSync, existsSync, writeFileSync, mkdirSync } from "fs";
import { join } from "path";
import { parse as parseYaml } from "yaml";
import { logTelemetry } from "./telemetry.js";
import type { HookInput, HookToolInput, LoadedEnforcementEntry, RuleViolation } from "../types.js";

// Parse YAML frontmatter from a markdown file using the yaml library.
function parseFrontmatter(content: string): Record<string, unknown> | null {
  const fmEnd = content.indexOf("
---", 4);
  if (!content.startsWith("---
") || fmEnd === -1) return null;
  try {
    return parseYaml(content.slice(4, fmEnd)) as Record<string, unknown>;
  } catch {
    return null;
  }
}