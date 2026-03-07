---
name: orqa-governance
description: |
  OrqaStudio governance patterns: artifact types, scanning pipeline, lesson promotion,
  rule enforcement, frontmatter schemas, and .orqa/ directory structure.
  Use when: Working with governance artifacts (docs, research, plans, lessons, rules),
  modifying scanning or enforcement, or maintaining the .orqa/ directory.
version: 1.0.0
tags: [orqa, governance, artifacts, scanning, lessons, rules, enforcement]
user-invocable: true
---

# Orqa Governance Patterns

OrqaStudio's governance layer manages documentation, research, plans, lessons, rules, agents, skills, and hooks as browsable, scannable artifacts. Understanding this system is critical for anyone working on the governance features.

## .orqa/ Directory Structure

```
.orqa/
  project.json          # Project config (name, dogfood flag, default model, etc.)
  icon.svg              # Project icon
  lessons/              # Implementation lessons (IMPL-NNN.md with YAML frontmatter)
  plans/                # Implementation plans (markdown with YAML frontmatter)
  research/             # Research documents
    README.md           # Research section overview
    mvp/                # MVP-phase research (grouped by project phase)
      branding.md
      claude-integration.md
      design-tokens.md
      frontend.md
      onboarding.md
      persistence.md
      tauri-v2.md
      wireframing.md
```

## Artifact Frontmatter Schemas

All governance artifacts use YAML frontmatter parsed by a generic function.

### Generic Parser

```rust
// src-tauri/src/domain/artifact.rs
pub fn parse_frontmatter<T: DeserializeOwned + Default>(content: &str) -> (T, String) {
    // Extracts YAML between --- delimiters, parses into T
    // Returns (frontmatter, body_without_frontmatter)
    // Falls back to T::default() if parsing fails
}
```

### DocFrontmatter

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocFrontmatter {
    pub title: Option<String>,
    pub category: Option<String>,
    pub order: Option<i64>,
    pub tags: Vec<String>,
}
```

### ResearchFrontmatter

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResearchFrontmatter {
    pub title: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,       // draft, complete, superseded
    pub date: Option<String>,
    pub tags: Vec<String>,
}
```

### PlanFrontmatter

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlanFrontmatter {
    pub title: Option<String>,
    pub status: Option<String>,       // draft, approved, in-progress, complete
    pub created: Option<String>,
    pub updated: Option<String>,
    pub phases: Option<i64>,
    pub completed_phases: Option<i64>,
    pub tags: Vec<String>,
}
```

## Artifact Commands Pattern

Each artifact type (docs, research, plans) has a `*_tree_scan` and `*_read` command pair:

```rust
// src-tauri/src/commands/artifact_commands.rs

#[tauri::command]
pub fn doc_tree_scan(project_path: String) -> Result<Vec<DocTreeEntry>, OrqaError> {
    let docs_dir = Path::new(&project_path).join("docs");
    // Recursively scan directory, parse frontmatter, return tree
}

#[tauri::command]
pub fn doc_read(project_path: String, relative_path: String) -> Result<DocContent, OrqaError> {
    let full_path = Path::new(&project_path).join("docs").join(&relative_path);
    // Read file, parse frontmatter, return structured content
}
```

The same pattern repeats for `research_tree_scan`/`research_read` and `plan_tree_scan`/`plan_read`.

## Governance Scanning

The enforcement engine loads rules from `.claude/rules/` and checks compliance:

```rust
// src-tauri/src/domain/enforcement_engine.rs
pub struct EnforcementEngine {
    rules: Vec<Rule>,
}

impl EnforcementEngine {
    pub fn load(rules_dir: &Path) -> Result<Self, OrqaError> {
        // Scan .claude/rules/*.md, extract rule metadata from frontmatter
    }

    pub fn check_compliance(&self, context: &ComplianceContext) -> Vec<Violation> {
        // Run rules against the context, return violations
    }
}
```

Rules are loaded when a project is opened (`project_open` command) and stored in `AppState`.

## Lesson Pipeline

```
Lesson documented (.orqa/lessons/IMPL-NNN.md)
    → Recurrence tracked (frontmatter count field incremented)
    → Promoted at threshold (recurrence >= 2)
    → Becomes rule or coding standard addition
    → Enforcement verified
```

Lesson files have YAML frontmatter:

```yaml
---
id: IMPL-001
title: "Channel<T> requires Clone on StreamEvent"
category: implementation
recurrence: 0
promoted_to: null
tags: [tauri, streaming, channel]
---
```

## Rule File Format

Rules in `.claude/rules/` use simple frontmatter:

```yaml
---
scope: system
---

# Rule Title (NON-NEGOTIABLE)

Rule content...

## Related Rules
- other-rule.md
```

The `scope: system` means the rule applies to all agents in all contexts.

## Two-Pillar Alignment

Every governance artifact serves at least one pillar:

| Pillar | What It Covers |
|--------|---------------|
| **Self-Learning Loop** | Lessons, metrics, retrospectives, pattern promotion, knowledge accumulation |
| **Process Governance** | Rules, agents, scanners, enforcement, quality gates, architecture decisions |

Features that serve neither pillar are out of scope.

## Project Settings

```rust
// src-tauri/src/domain/project_settings.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub name: String,
    #[serde(default)]
    pub dogfood: bool,
    pub default_model: Option<String>,
    // ... other settings
}
```

When `dogfood: true`, enhanced caution rules apply (see `.claude/rules/dogfood-mode.md`).

## Key Files

| File | Purpose |
|------|---------|
| `.orqa/project.json` | Project configuration |
| `.orqa/lessons/` | Implementation lessons |
| `.orqa/plans/` | Implementation plans |
| `.orqa/research/` | Research documents |
| `.claude/rules/` | Governance rules |
| `.claude/agents/` | Agent definitions |
| `.claude/skills/` | Skill definitions |
| `src-tauri/src/domain/artifact.rs` | Frontmatter parsing, artifact types |
| `src-tauri/src/commands/artifact_commands.rs` | Tree scan and read commands |
| `src-tauri/src/domain/enforcement_engine.rs` | Rule loading and compliance checking |
| `src-tauri/src/domain/project_settings.rs` | Project settings struct |
| `docs/product/vision.md` | Two-Pillar framework definition |
