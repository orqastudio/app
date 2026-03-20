#!/usr/bin/env node
// Prompt-based skill injector: examines user prompt, classifies intent,
// and injects relevant domain skills as systemMessage.
//
// Used by UserPromptSubmit hook. Reads hook input from stdin.
// Outputs JSON with systemMessage containing skill content.
//
// Intent classification currently uses keyword heuristics. Designed for
// easy upgrade to AI classification (Haiku model call) when API access
// is available in the hook context.

import { readFileSync, readdirSync, existsSync, mkdirSync, writeFileSync } from "fs";
import { join } from "path";

// Intent-to-knowledge mapping table
// Each entry: { keywords: string[], skills: string[], description: string }
// Knowledge names must match directory names under .orqa/process/knowledge/ or app/.orqa/process/knowledge/
const INTENT_MAP = [
  // ── Backend / IPC ──────────────────────────────────────────────────────────
  {
    keywords: ["tauri command", "ipc", "invoke", "#[tauri::command]", "add a command", "new command"],
    skills: ["orqa-ipc-patterns", "orqa-error-composition"],
    description: "IPC boundary work",
  },
  {
    keywords: ["domain", "domain service", "domain model", "business logic"],
    skills: ["orqa-domain-services", "orqa-error-composition"],
    description: "Domain logic",
  },
  {
    keywords: ["repository", "database", "sqlite", "migration", "query"],
    skills: ["orqa-repository-pattern"],
    description: "Data access layer",
  },
  {
    keywords: ["stream", "sidecar", "ndjson", "provider", "streaming"],
    skills: ["orqa-streaming"],
    description: "Streaming pipeline",
  },
  {
    keywords: ["rust", "async", "tokio", "future", "trait", "impl", "cargo"],
    skills: ["rust-async-patterns", "orqa-backend-best-practices"],
    description: "Rust / async backend work",
  },
  {
    keywords: ["typescript", "type alias", "generic", "discriminated union", "mapped type", "conditional type"],
    skills: ["typescript-advanced-types"],
    description: "TypeScript advanced types",
  },
  // ── Frontend / UI ─────────────────────────────────────────────────────────
  {
    keywords: ["store", "reactive", "$state", "$derived", "$effect", "rune"],
    skills: ["orqa-store-patterns", "orqa-store-orchestration"],
    description: "Store architecture",
  },
  {
    keywords: ["component", "svelte component", "ui component", "create a component"],
    skills: ["svelte5-best-practices", "orqa-frontend-best-practices"],
    description: "Component work",
  },
  {
    keywords: ["tailwind", "design system", "css", "theme", "token", "color", "spacing", "typography"],
    skills: ["tailwind-design-system"],
    description: "Tailwind / design system",
  },
  {
    keywords: ["extract component", "shared component", "reusable component", "component library"],
    skills: ["component-extraction", "svelte5-best-practices"],
    description: "Component extraction",
  },
  {
    keywords: ["ux", "accessibility", "usability", "a11y", "user experience", "ux review", "compliance"],
    skills: ["ux-compliance-review"],
    description: "UX and accessibility compliance",
  },
  // ── Testing ────────────────────────────────────────────────────────────────
  {
    keywords: ["test", "testing", "vitest", "cargo test", "coverage", "unit test", "integration test"],
    skills: ["orqa-testing"],
    description: "Testing work",
  },
  {
    keywords: ["e2e", "playwright", "end-to-end", "test engineering", "test strategy", "test plan"],
    skills: ["test-engineering"],
    description: "Test engineering and E2E",
  },
  {
    keywords: ["qa", "quality assurance", "acceptance", "verification", "qa verification"],
    skills: ["qa-verification"],
    description: "QA verification process",
  },
  // ── Debugging / Diagnostics ───────────────────────────────────────────────
  {
    keywords: ["debug", "fix", "broken", "error", "failing", "crash", "bug", "investigate", "diagnose"],
    skills: ["diagnostic-methodology", "systems-thinking"],
    description: "Diagnostic work",
  },
  // ── Planning / Architecture ────────────────────────────────────────────────
  {
    keywords: ["plan", "approach", "design", "architect", "tradeoff", "trade-off"],
    skills: ["planning", "systems-thinking"],
    description: "Planning phase",
  },
  {
    keywords: ["architecture", "evaluate architecture", "assess design", "design review", "architectural decision", "adr"],
    skills: ["architectural-evaluation", "systems-thinking"],
    description: "Architectural evaluation",
  },
  {
    keywords: ["system", "holistic", "impact analysis", "second-order", "ripple effect", "dependencies between"],
    skills: ["systems-thinking"],
    description: "Systems thinking",
  },
  {
    keywords: ["research", "investigate", "gather information", "explore options", "compare"],
    skills: ["research-methodology"],
    description: "Research methodology",
  },
  // ── Refactoring / Tech Debt ────────────────────────────────────────────────
  {
    keywords: ["refactor", "restructur", "reorganiz", "extract", "consolidat", "move files", "migrate files"],
    skills: ["restructuring-methodology", "systems-thinking"],
    description: "Refactoring work",
  },
  {
    keywords: ["tech debt", "cleanup", "refactor debt", "pay down debt", "dead code", "legacy code"],
    skills: ["tech-debt-management"],
    description: "Tech debt management",
  },
  {
    keywords: ["compose", "composable", "modular", "reuse", "combine", "compose modules"],
    skills: ["composability"],
    description: "Composability and modularity",
  },
  // ── Code Quality ───────────────────────────────────────────────────────────
  {
    keywords: ["code review", "quality review", "lint", "static analysis", "clippy", "eslint", "code quality"],
    skills: ["code-quality-review"],
    description: "Code quality review",
  },
  {
    keywords: ["security", "audit", "vulnerability", "owasp", "injection", "xss", "auth", "permissions"],
    skills: ["security-audit"],
    description: "Security audit",
  },
  // ── Search ─────────────────────────────────────────────────────────────────
  {
    keywords: ["search", "find", "where is", "locate", "grep", "semantic search", "chunkhound"],
    skills: ["search"],
    description: "Code and artifact search",
  },
  // ── Governance / Artifacts ─────────────────────────────────────────────────
  {
    keywords: ["governance", "rule", "knowledge", "artifact", "enforcement"],
    skills: ["orqa-governance", "orqa-documentation"],
    description: "Governance work",
  },
  {
    keywords: ["artifact status", "status transition", "promote", "lifecycle", "state machine", "in progress", "complete"],
    skills: ["artifact-status-management"],
    description: "Artifact status management",
  },
  {
    keywords: ["relationship", "link artifact", "connect artifact", "bidirectional", "artifact graph", "references"],
    skills: ["artifact-relationships"],
    description: "Artifact relationships",
  },
  {
    keywords: ["create artifact", "new artifact", "write artifact", "artifact template", "frontmatter"],
    skills: ["artifact-creation", "artifact-ids"],
    description: "Artifact creation",
  },
  {
    keywords: ["artifact id", "generate id", "artifact identifier", "id format"],
    skills: ["artifact-ids"],
    description: "Artifact ID generation",
  },
  {
    keywords: ["governance maintenance", "maintain governance", "audit governance", "graph health", "integrity check"],
    skills: ["governance-maintenance"],
    description: "Governance maintenance",
  },
  {
    keywords: ["governance context", "governance background", "orqa overview", "orqa architecture"],
    skills: ["governance-context", "orqa-architecture"],
    description: "Governance context and architecture",
  },
  {
    keywords: ["schema", "validate", "frontmatter", "core.json", "schema validation", "yaml schema"],
    skills: ["schema-validation"],
    description: "Schema validation",
  },
  {
    keywords: ["naming", "convention", "rename", "identifier", "naming convention", "name format"],
    skills: ["naming-conventions"],
    description: "Naming conventions",
  },
  {
    keywords: ["delegate", "delegation", "agent role", "orchestrat", "subagent", "assign task"],
    skills: ["delegation-patterns"],
    description: "Delegation patterns",
  },
  {
    keywords: ["rule enforcement", "enforce rule", "gate check", "pipeline gate", "pre-commit hook"],
    skills: ["rule-enforcement"],
    description: "Rule enforcement",
  },
  // ── Plugin Development ──────────────────────────────────────────────────────
  {
    keywords: ["plugin", "develop plugin", "create plugin", "build plugin", "first-party plugin", "core plugin"],
    skills: ["plugin-development-first-party", "orqa-plugin-development"],
    description: "First-party plugin development",
  },
  {
    keywords: ["third-party plugin", "community plugin", "external plugin", "publish plugin"],
    skills: ["plugin-development-third-party"],
    description: "Third-party plugin development",
  },
  {
    keywords: ["install plugin", "plugin setup", "configure plugin", "enable plugin", "plugin config"],
    skills: ["plugin-setup"],
    description: "Plugin setup and installation",
  },
  // ── Project Setup / Inference ───────────────────────────────────────────────
  {
    keywords: ["infer project", "detect project", "project type", "stack detection", "detect stack"],
    skills: ["project-inference", "project-type-software"],
    description: "Project inference and detection",
  },
  {
    keywords: ["project setup", "setup project", "initialize project", "new project", "onboard project"],
    skills: ["project-setup"],
    description: "Project setup",
  },
  {
    keywords: ["project migration", "migrate project", "upgrade project", "move project"],
    skills: ["project-migration"],
    description: "Project migration",
  },
  // ── Epic / Skills Maintenance ───────────────────────────────────────────────
  {
    keywords: ["epic", "requirement", "infer requirement", "epic scope", "derive requirement"],
    skills: ["epic-requirement-inference"],
    description: "Epic requirement inference",
  },
  {
    keywords: ["skills maintenance", "update skill", "maintain skill", "skill quality", "knowledge quality"],
    skills: ["skills-maintenance"],
    description: "Skills and knowledge maintenance",
  },
  // ── Logging ────────────────────────────────────────────────────────────────
  {
    keywords: ["log", "logging", "logger", "console.log", "tracing"],
    skills: ["centralized-logging"],
    description: "Logging work",
  },
  // ── Licensing ──────────────────────────────────────────────────────────────
  {
    keywords: ["license", "licensing", "dependency license", "license compatibility", "open source license"],
    skills: ["dependency-license-compatibility", "licensing-decisions"],
    description: "License and dependency licensing",
  },
];

// Classify intent from user prompt using keyword matching
// Returns array of unique skill names to inject
function classifyIntent(prompt) {
  const lower = prompt.toLowerCase();
  const matchedSkills = new Set();

  for (const entry of INTENT_MAP) {
    const matches = entry.keywords.some((kw) => lower.includes(kw.toLowerCase()));
    if (matches) {
      for (const skill of entry.skills) {
        matchedSkills.add(skill);
      }
    }
  }

  return [...matchedSkills];
}

// Read the session-level injected skills state
function readInjectedSkills(projectDir) {
  const stateFile = join(projectDir, "tmp", ".injected-skills.json");
  if (!existsSync(stateFile)) return [];
  try {
    return JSON.parse(readFileSync(stateFile, "utf-8"));
  } catch {
    return [];
  }
}

// Write the session-level injected skills state
function writeInjectedSkills(projectDir, skills) {
  const tmpDir = join(projectDir, "tmp");
  if (!existsSync(tmpDir)) {
    mkdirSync(tmpDir, { recursive: true });
  }
  writeFileSync(join(tmpDir, ".injected-skills.json"), JSON.stringify(skills));
}

// Strip YAML frontmatter from skill content
function stripFrontmatter(content) {
  const match = content.match(/^---\n[\s\S]*?\n---\n([\s\S]*)$/);
  if (match) return match[1].trim();
  return content.trim();
}

// Read knowledge files, deduplicating against already-injected
function collectKnowledgeContent(projectDir, skillNames) {
  const alreadyInjected = readInjectedSkills(projectDir);
  const alreadySet = new Set(alreadyInjected);

  // Filter to only new skills
  const newSkills = skillNames.filter((name) => !alreadySet.has(name));
  if (newSkills.length === 0) return null;

  const parts = [];
  const injectedNow = [];

  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || "";

  for (const name of newSkills) {
    // Search plugin knowledge/ first, then project-level, then app-level.
    // KNOW.md is the canonical filename post-rename; SKILL.md is the legacy fallback.
    const candidates = [
      pluginRoot ? join(pluginRoot, "knowledge", name, "KNOW.md") : "",
      pluginRoot ? join(pluginRoot, "knowledge", name, "SKILL.md") : "",
      pluginRoot ? join(pluginRoot, "knowledge", `${name}.md`) : "",
      join(projectDir, ".orqa", "process", "knowledge", name, "KNOW.md"),
      join(projectDir, ".orqa", "process", "knowledge", `${name}.md`),
      join(projectDir, "app", ".orqa", "process", "knowledge", name, "KNOW.md"),
      join(projectDir, "app", ".orqa", "process", "knowledge", `${name}.md`),
    ].filter(Boolean);
    const knowledgePath = candidates.find((p) => existsSync(p));
    if (!knowledgePath) continue;
    try {
      const raw = readFileSync(knowledgePath, "utf-8");
      const content = stripFrontmatter(raw);
      if (content) {
        parts.push(content);
        injectedNow.push(name);
      }
    } catch {
      // Skip unreadable files silently
    }
  }

  if (parts.length === 0) return null;

  // Persist updated state
  writeInjectedSkills(projectDir, [...alreadyInjected, ...injectedNow]);

  return parts.join("\n\n---\n\n");
}

// Read project.json and extract settings for template resolution
function readProjectSettings(projectDir) {
  const projectJsonPath = join(projectDir, ".orqa", "project.json");
  if (!existsSync(projectJsonPath)) return {};
  try {
    return JSON.parse(readFileSync(projectJsonPath, "utf-8"));
  } catch {
    return {};
  }
}

// Resolve {{variables}} in template from project settings
function resolveTemplate(template, projectDir) {
  const settings = readProjectSettings(projectDir);
  const pluginsConfig = settings.plugins || {};
  const plugins = Object.entries(pluginsConfig)
    .filter(([, cfg]) => cfg.installed && cfg.enabled)
    .map(([name]) => name);

  const vars = {
    "project.name": settings.name || "unknown",
    "project.dogfood": settings.dogfood ? "active — you are editing the app from the CLI" : "inactive",
    "project.plugins": plugins.length > 0 ? plugins.join(", ") : "none",
  };

  return template.replace(/\{\{(\w+(?:\.\w+)*)\}\}/g, (match, key) => {
    return vars[key] ?? match;
  });
}

// Read the context reminder from plugin root and resolve template variables
function readContextReminder(projectDir) {
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || "";
  if (!pluginRoot) return "";
  const reminderPath = join(pluginRoot, "context-reminder.md");
  if (!existsSync(reminderPath)) return "";
  try {
    const template = readFileSync(reminderPath, "utf-8").trim();
    return resolveTemplate(template, projectDir);
  } catch {
    return "";
  }
}

// Main
async function main() {
  let input = "";
  for await (const chunk of process.stdin) {
    input += chunk;
  }

  let hookInput;
  try {
    hookInput = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  const userMessage = hookInput.user_message || hookInput.prompt || "";
  const projectDir = hookInput.cwd || process.env.CLAUDE_PROJECT_DIR || ".";

  if (!userMessage) {
    process.exit(0);
  }

  const parts = [];

  // Always inject context reminder with resolved project variables
  const reminder = readContextReminder(projectDir);
  if (reminder) {
    parts.push(reminder);
  }

  // Skill injection is DISABLED for the orchestrator's UserPromptSubmit hook.
  // The orchestrator delegates — it doesn't implement. Implementation agents
  // receive domain skills via their Agent tool prompt, not via this hook.
  // Only the context reminder (above) injects into the orchestrator.

  if (parts.length === 0) {
    process.exit(0);
  }

  // Return combined content as systemMessage
  const output = JSON.stringify({
    systemMessage: parts.join("\n\n---\n\n"),
  });
  process.stdout.write(output);
  process.exit(0);
}

main().catch(() => process.exit(0));
