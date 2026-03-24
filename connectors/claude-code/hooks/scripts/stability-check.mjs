#!/usr/bin/env node
// stability-check.mjs — Demoted-rule stability tracker
//
// Runs at session start. For each rule with status: inactive and a demoted_date,
// checks tmp/precommit-violations.jsonl for violations in that rule's domain
// since the last session. Increments or resets stability_count accordingly.
// Surfaces rules that have reached their stability_threshold for deletion.
//
// Usage: node stability-check.mjs [project-root]

import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";
import matter from "gray-matter";

// ---------------------------------------------------------------------------
// Find project root
// ---------------------------------------------------------------------------

function findProjectRoot(startDir) {
  let dir = startDir;
  while (dir !== "/" && dir !== "." && !dir.endsWith(":\\")) {
    if (existsSync(join(dir, ".orqa"))) return dir;
    const parent = join(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  return startDir;
}

// ---------------------------------------------------------------------------
// Find all rule files across .orqa/ and plugins/
// ---------------------------------------------------------------------------

function findRuleFiles(projectRoot) {
  const files = [];
  const dirs = [
    join(projectRoot, ".orqa", "process", "rules"),
    ...findPluginRuleDirs(projectRoot),
  ];

  for (const dir of dirs) {
    if (!existsSync(dir)) continue;
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (!entry.isFile() || !entry.name.endsWith(".md")) continue;
      if (!entry.name.startsWith("RULE-")) continue;
      files.push(join(dir, entry.name));
    }
  }

  return files;
}

function findPluginRuleDirs(projectRoot) {
  const dirs = [];
  for (const container of ["plugins", "connectors"]) {
    const containerDir = join(projectRoot, container);
    if (!existsSync(containerDir)) continue;
    for (const entry of readdirSync(containerDir, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.name.startsWith(".")) continue;
      const rulesDir = join(containerDir, entry.name, "rules");
      if (existsSync(rulesDir)) dirs.push(rulesDir);
    }
  }
  return dirs;
}

// ---------------------------------------------------------------------------
// Load demoted rules (status: inactive with demoted_date)
// ---------------------------------------------------------------------------

function loadDemotedRules(projectRoot) {
  const rules = [];
  for (const filePath of findRuleFiles(projectRoot)) {
    let parsed;
    try {
      parsed = matter(readFileSync(filePath, "utf-8"));
    } catch {
      continue;
    }

    const data = parsed.data;
    // Only process inactive rules with a demoted_date
    const status = String(data.status || "").toLowerCase();
    if (status !== "inactive" || !data.demoted_date) continue;

    rules.push({
      filePath,
      id: data.id,
      title: data.title || data.id,
      demotedDate: data.demoted_date,
      demotedReason: data.demoted_reason || "",
      replacedBy: data.replaced_by || "",
      stabilityThreshold: data.stability_threshold ?? 10,
      stabilityCount: data.stability_count ?? 0,
      // Use the rule's enforcement domain tags, or fallback to a general domain
      domains: extractDomains(data),
      rawContent: readFileSync(filePath, "utf-8"),
      frontmatter: data,
    });
  }
  return rules;
}

function extractDomains(data) {
  // Try to extract domain from enforcement entries
  const domains = new Set();
  if (Array.isArray(data.enforcement)) {
    for (const entry of data.enforcement) {
      if (entry.domain) domains.add(entry.domain);
      if (entry.mechanism) domains.add(entry.mechanism);
    }
  }
  // Fallback: use the replaced_by field as a domain hint
  if (data.replaced_by) {
    domains.add(data.replaced_by.toLowerCase().replace(/\s+/g, "-"));
  }
  // Always include "governance" as a baseline domain
  domains.add("governance");
  return [...domains];
}

// ---------------------------------------------------------------------------
// Load violations from the JSONL log
// ---------------------------------------------------------------------------

function loadViolations(projectRoot) {
  const logPath = join(projectRoot, "tmp", "precommit-violations.jsonl");
  if (!existsSync(logPath)) return [];

  const lines = readFileSync(logPath, "utf-8").split("\n").filter(Boolean);
  const violations = [];

  for (const line of lines) {
    try {
      violations.push(JSON.parse(line));
    } catch {
      // Skip malformed lines
    }
  }

  return violations;
}

// ---------------------------------------------------------------------------
// Check stability for each demoted rule
// ---------------------------------------------------------------------------

function checkStability(rules, violations) {
  const results = [];

  for (const rule of rules) {
    const demotedDate = new Date(rule.demotedDate);

    // Find violations that match this rule's domains and occurred after demotion
    const relevantViolations = violations.filter((v) => {
      const vDate = new Date(v.timestamp);
      if (vDate < demotedDate) return false;
      // Match on domain overlap
      return rule.domains.some(
        (d) =>
          v.domain === d ||
          v.violation_type === d ||
          (v.detail && v.detail.toLowerCase().includes(d))
      );
    });

    const hadRecentViolations = relevantViolations.length > 0;
    const previousCount = rule.stabilityCount;
    let newCount;

    if (hadRecentViolations) {
      newCount = 0; // Reset on any violation
    } else {
      newCount = previousCount + 1; // Increment for clean session
    }

    const reachedThreshold = newCount >= rule.stabilityThreshold;

    results.push({
      rule,
      previousCount,
      newCount,
      hadRecentViolations,
      reachedThreshold,
      relevantViolationCount: relevantViolations.length,
    });
  }

  return results;
}

// ---------------------------------------------------------------------------
// Update rule frontmatter with new stability_count
// ---------------------------------------------------------------------------

function updateRuleStabilityCount(rule, newCount) {
  const content = rule.rawContent;
  const parsed = matter(content);
  parsed.data.stability_count = newCount;

  // Reconstruct the file with updated frontmatter
  const newContent = matter.stringify(parsed.content, parsed.data);
  writeFileSync(rule.filePath, newContent, "utf-8");
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

function main() {
  const projectRoot = findProjectRoot(process.argv[2] || process.cwd());
  const rules = loadDemotedRules(projectRoot);

  if (rules.length === 0) {
    // No demoted rules — nothing to track
    process.exit(0);
  }

  const violations = loadViolations(projectRoot);
  const results = checkStability(rules, violations);

  const output = [];

  for (const result of results) {
    // Update the rule file with the new count
    updateRuleStabilityCount(result.rule, result.newCount);

    if (result.reachedThreshold) {
      output.push(
        `STABLE: ${result.rule.id} (${result.rule.title}) has been stable for ` +
          `${result.newCount} sessions (threshold: ${result.rule.stabilityThreshold}). ` +
          `Safe to delete. Replaced by: ${result.rule.replacedBy || "N/A"}. ` +
          `Confirm deletion?`
      );
    } else if (result.hadRecentViolations) {
      output.push(
        `RESET: ${result.rule.id} stability count reset to 0 ` +
          `(${result.relevantViolationCount} violation(s) found). ` +
          `Was at ${result.previousCount}/${result.rule.stabilityThreshold}.`
      );
    } else if (result.newCount > 0) {
      output.push(
        `TRACKING: ${result.rule.id} — ${result.newCount}/${result.rule.stabilityThreshold} ` +
          `clean sessions since demotion.`
      );
    }
  }

  if (output.length > 0) {
    console.log("=== Demoted Rule Stability ===");
    for (const line of output) {
      console.log(line);
    }
    console.log("==============================");
  }
}

main();
