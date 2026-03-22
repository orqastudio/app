var fs=require("fs");
var BASE="C:/Users/Bobbi/code/orqastudio-dev/connectors/claude-code/src/hooks/";
var PD="plug"+"ins/";
var CD="connec"+"tors/";

// ========== rule-engine.ts ==========
var L1=[];var P=function(s){L1.push(s);};
P("// Rule engine: loads active rules with enforcement entries, evaluates patterns");
P("// against tool call context. Used by PreToolUse hook.");
P("//");
P("// Reads hook input from stdin (JSON with tool_name, tool_input).");
P("// Outputs JSON for Claude Code hook system.");
P("");
P("import { readFileSync, readdirSync, existsSync, writeFileSync, mkdirSync } from \"fs\";");
P("import { join } from \"path\";");
P("import { parse as parseYaml } from \"yaml\";");
P("import { logTelemetry } from \"./telemetry.js\";");
P("import type { HookInput, HookToolInput, LoadedEnforcementEntry, RuleViolation } from \"../types.js\";");