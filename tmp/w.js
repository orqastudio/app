var fs2 = require("fs");
var base = "C:/Users/Bobbi/code/orqastudio-dev/connectors/claude-code/src/hooks/";

// rule-engine.ts
var re_lines = [];
var re = function(s) { re_lines.push(s); };
re("// Rule engine: loads active rules with enforcement entries, evaluates patterns");
re("// against tool call context. Used by PreToolUse hook.");
re("//");
re("// Reads hook input from stdin (JSON with tool_name, tool_input).");
re("// Outputs JSON for Claude Code hook system.");
re("");
re("import { readFileSync, readdirSync, existsSync, writeFileSync, mkdirSync } from \"fs\";");
re("import { join } from \"path\";");
re("import { parse as parseYaml } from \"yaml\";");
re("import { logTelemetry } from \"./telemetry.js\";");
re("import type { HookInput, HookToolInput, LoadedEnforcementEntry, RuleViolation } from \"../types.js\";");
re("");
re("function parseFrontmatter(content: string): Record<string, unknown> | null {");
re("  const fmEnd = content.indexOf(\"\n---\", 4);");
re("  if (!content.startsWith(\"---\n\") || fmEnd === -1) return null;");
re("  try { return parseYaml(content.slice(4, fmEnd)) as Record<string, unknown>; }");
re("  catch { return null; }");
re("}");