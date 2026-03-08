---
id: orqa-composability
title: "Orqa Composability"
name: orqa-composability
description: |
  OrqaStudio's composability philosophy: building systems from small, pure, swappable units.
  Shapes how agents think about code structure at both the function level and the feature level.
  Use when: Writing any new code, reviewing architecture, planning features, or refactoring
  existing modules. This is the meta-skill — it informs all other patterns.
version: 1.0.0
tags: [orqa, composability, architecture, pure-functions, pipeline, modularity]
user-invocable: true
---


OrqaStudio is built from small, composable units at every level: functions, modules, features, and integrations. This skill teaches the composability philosophy that shapes all code in the project, grounded in AD-017 (Composability Principle).

Composability is not just a coding pattern — it is a platform principle. Every software project initialized with OrqaStudio should be composable by default, because composable systems are dramatically easier to pivot, extend, and maintain as requirements change.

## The Core Idea

Every piece of the system should be:

1. **Small enough to understand in isolation**
2. **Pure enough to test without the world**
3. **Typed enough to compose safely**
4. **Swappable enough to replace without cascading changes**

This applies at every scale: individual functions, modules, features, integrations, and even the app itself. OrqaStudio eats its own cooking — the app is built with the same composability principles it enforces on projects it manages.

## Principle 1: Pure Over Stateful

Functions take inputs and return outputs. No hidden side effects, no global mutation, no reading from distant state.

### Rust: Pure Domain Functions

The enforcement engine is built entirely from pure functions. Each function takes explicit inputs and returns a value — no reading from `self`, no database, no filesystem hidden behind the call.

```rust
// src-tauri/src/domain/enforcement_engine.rs

/// Pure function: takes a string, returns a string. No side effects.
fn prose_excerpt(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.len() <= 200 {
        trimmed.to_string()
    } else {
        format!("{}…", &trimmed[..200])
    }
}

/// Pure function: takes an entry + index, returns a compiled entry or None.
/// The Option return type replaces exceptions — the caller decides what to do.
fn compile_entry(
    entry: &EnforcementEntry,
    rule_index: usize,
    rule_name: &str,
) -> Option<CompiledEntry> {
    let compiled_conditions = entry
        .conditions
        .iter()
        .filter_map(|c| match Regex::new(&c.pattern) {
            Ok(re) => Some((c.field.clone(), re)),
            Err(e) => {
                tracing::warn!("invalid regex '{}' in rule '{rule_name}': {e}", c.pattern);
                None
            }
        })
        .collect::<Vec<_>>();

    if compiled_conditions.len() != entry.conditions.len() {
        return None;
    }
    // ... build and return CompiledEntry
}
```

### Rust: Generic Composition

The `parse_frontmatter` function demonstrates generic composition — one function that works with any deserializable type:

```rust
// src-tauri/src/domain/artifact.rs

/// Parse YAML frontmatter into any deserializable type.
/// Pure function — no filesystem access.
pub fn parse_frontmatter<T: serde::de::DeserializeOwned + Default>(content: &str) -> (T, String) {
    let (fm_text, body) = extract_frontmatter(content);
    let frontmatter = fm_text
        .and_then(|text| serde_yaml::from_str::<T>(&text).ok())
        .unwrap_or_default();
    (frontmatter, body)
}

/// Typed entry points compose the generic function with specific types:
pub fn parse_doc_frontmatter(content: &str) -> (DocFrontmatter, String) {
    parse_frontmatter::<DocFrontmatter>(content)
}
pub fn parse_plan_frontmatter(content: &str) -> (PlanFrontmatter, String) {
    parse_frontmatter::<PlanFrontmatter>(content)
}
```

One generic core. Multiple typed entry points. Each frontmatter type is a separate struct, but they all compose through the same parsing pipeline.

### Svelte: Prop-Driven Components

```svelte
<!-- ui/lib/components/shared/EmptyState.svelte -->
<script lang="ts">
    import type { Component } from "svelte";

    let {
        icon: Icon,
        title,
        description,
        action,
    }: {
        icon?: Component;
        title: string;
        description?: string;
        action?: { label: string; onclick: () => void };
    } = $props();
</script>

<div class="flex flex-col items-center justify-center py-12 text-center">
    {#if Icon}<Icon class="mb-4 h-12 w-12 text-muted-foreground" />{/if}
    <h3 class="text-lg font-semibold">{title}</h3>
    {#if description}<p class="mt-1 text-sm text-muted-foreground">{description}</p>{/if}
    {#if action}<button class="..." onclick={action.onclick}>{action.label}</button>{/if}
</div>
```

This component is infinitely reusable — it works for empty session lists, empty artifact browsers, empty search results — because it has no knowledge of what it is being used for.

## Principle 2: Small Composable Units

Prefer 5 small functions over 1 large one. Each function has a single job.

### Rust: Enforcement Engine Decomposition

| Function | Lines | Single Responsibility |
|----------|------:|----------------------|
| `compile_entry` | 48 | Compile one rule entry into regex objects |
| `prose_excerpt` | 7 | Truncate prose for display |
| `collect_glob_paths` | 19 | Expand a glob pattern to file paths |
| `scan_file` | 32 | Check one file against one compiled entry |
| `evaluate_file` | 30 | Check all file entries against one event |
| `evaluate_bash` | 25 | Check all bash entries against one command |
| `scan` | 31 | Orchestrate full project scan |

The `scan` method composes these small units:

```rust
// src-tauri/src/domain/enforcement_engine.rs
pub fn scan(&self, project_path: &Path) -> Result<Vec<ScanFinding>, OrqaError> {
    let mut findings = Vec::new();
    for ce in &self.compiled {
        if ce.event != EventType::Scan { continue; }
        let scope = match &ce.scope {
            Some(s) => s,
            None => { tracing::warn!("..."); continue; }
        };
        let glob_pattern = project_path.join(scope);
        let paths = collect_glob_paths(&glob_pattern.to_string_lossy())?;
        for file_path in paths {
            let file_findings = scan_file(&file_path, ce, &self.rules[ce.rule_index])?;
            findings.extend(file_findings);
        }
    }
    Ok(findings)
}
```

Reading `scan` tells you *what* happens without drowning in *how*.

### Rust: Command Decomposition

```rust
// src-tauri/src/commands/governance_commands.rs
pub fn governance_analyze(...) -> Result<GovernanceAnalysis, OrqaError> {
    let session_id = create_governance_session(project_id, &state)?;
    let prompt = build_analysis_prompt(&scan_result);
    super::sidecar_commands::ensure_sidecar_running(&state)?;
    let raw_response = send_and_collect(&state, session_id, &prompt)?;
    let output = parse_claude_output(&raw_response)?;
    let now = current_timestamp();
    persist_analysis_and_recommendations(project_id, &analysis, &output, &now, &state)
}
```

Each helper is independently testable — `parse_claude_output`, `build_analysis_prompt`, etc. all have dedicated unit tests.

## Principle 3: Trait Boundaries and Interfaces

Define what something *does*, not what it *is*.

### Rust: Error Composition via From Traits

```rust
// src-tauri/src/error.rs
impl From<std::io::Error> for OrqaError {
    fn from(err: std::io::Error) -> Self { Self::FileSystem(err.to_string()) }
}
impl From<rusqlite::Error> for OrqaError {
    fn from(err: rusqlite::Error) -> Self { Self::Database(err.to_string()) }
}
impl From<serde_json::Error> for OrqaError {
    fn from(err: serde_json::Error) -> Self { Self::Serialization(err.to_string()) }
}
```

This enables `?` propagation everywhere. A function that reads a file, parses JSON, and queries SQLite can use `?` on all three — the `From` implementations compose the error types automatically.

### TypeScript: Discriminated Unions

```typescript
// ui/lib/stores/conversation.svelte.ts
export type ContextEntry =
    | { type: "system_prompt_sent"; customPrompt: string | null; governancePrompt: string; totalChars: number; }
    | { type: "context_injected"; messageCount: number; totalChars: number; messages: string; };
```

The `type` discriminant lets consumers `switch` on the variant with full type narrowing — the TypeScript equivalent of Rust's tagged enums.

## Principle 4: Pluggable by Default

Every integration point is swappable. The system defines *what* it needs, and providers implement *how*.

### Sidecar: Provider-Agnostic Interface

```typescript
// sidecar/src/provider.ts
type ResponseSender = (response: SidecarResponse) => void;

export async function streamMessage(
    sessionId: number,
    content: string,
    model: string | null,
    systemPrompt: string | null,
    sendResponse: ResponseSender,
    sdkSessionId: string | null = null,
    enableThinking: boolean = false,
): Promise<void> { /* ... */ }
```

The `ResponseSender` type is the contract. The Rust backend reads NDJSON lines from stdout — it does not care what generated them. Per AD-017, adding a new AI provider means implementing a new sidecar that speaks the same NDJSON protocol. Zero changes to Rust or Svelte.

### Tool Server as MCP

```typescript
// sidecar/src/provider.ts
function createOrqaToolServer(sendResponse: ResponseSender) {
    return createSdkMcpServer({
        name: 'orqa-studio-tools',
        tools: [
            tool('read_file', 'Read a file', { path: z.string() },
                async (args) => executeToolViaRust('read_file', args, sendResponse)),
            tool('write_file', 'Write a file', { path: z.string(), content: z.string() },
                async (args) => executeToolViaRust('write_file', args, sendResponse)),
            // ... same pattern for all tools
        ],
    });
}
```

Adding a new tool is one entry in the array. The tool execution goes through the NDJSON protocol to Rust, so the sidecar never touches the filesystem directly.

## Principle 5: Pipeline Composition

Complex operations are chains of simple transforms, not monolithic functions.

### Rust: Functional Pipelines

```rust
// src-tauri/src/commands/governance_commands.rs
fn format_file_list(scan: &GovernanceScanResult) -> String {
    scan.areas.iter()
        .filter(|a| a.covered)
        .flat_map(|a| a.files.iter().map(|f|
            format!("### {} ({})\n```\n{}\n```\n", f.path, a.name, f.content_preview)))
        .collect::<Vec<_>>()
        .join("\n")
}
```

Data flows through `iter() -> filter() -> flat_map() -> collect()`. No intermediate mutable variables.

### Svelte: Event Pipeline in Store

```typescript
// ui/lib/stores/conversation.svelte.ts
private handleStreamEvent(event: StreamEvent) {
    switch (event.type) {
        case "stream_start":
            this.isStreaming = true;
            this.streamingContent = "";
            break;
        case "text_delta":
            this.streamingContent += event.data.content;
            break;
        case "tool_use_start": {
            const newMap = new SvelteMap(this.activeToolCalls);
            newMap.set(event.data.tool_call_id, { /* ... */ });
            this.activeToolCalls = newMap;
            break;
        }
    }
}
```

Each `case` is a self-contained state transition. Adding a new event type means adding one case block.

## Principle 6: Feature as Module

A feature is a self-contained unit: domain logic + command handler + IPC type + store + component. Features can be added or removed without touching other features.

### The Four-Layer Feature Structure

```
src-tauri/src/domain/<feature>.rs              -- Pure domain logic and types
src-tauri/src/commands/<feature>_commands.rs    -- Thin #[tauri::command] wrappers
ui/lib/types/<feature>.ts                      -- TypeScript interfaces matching Rust types
ui/lib/stores/<feature>.svelte.ts              -- Runes-based store calling invoke()
ui/lib/components/<feature>/                   -- Svelte components receiving props
```

The domain module knows nothing about Tauri. The command module knows nothing about the UI. The store knows nothing about the components. Dependency direction is strictly one-way: component -> store -> IPC -> command -> domain.

## Principle 7: Functional Paradigm

Prefer `map`/`filter`/`collect` in Rust, derived state in Svelte, and data transformations over mutations.

```rust
// src-tauri/src/commands/governance_commands.rs
fn build_recommendations(project_id: i64, analysis_id: i64, output: &ClaudeAnalysisOutput, now: &str) -> Vec<Recommendation> {
    output.recommendations.iter()
        .map(|raw| {
            let priority = RecommendationPriority::parse(&raw.priority)
                .unwrap_or(RecommendationPriority::Recommended);
            Recommendation { id: 0, project_id, analysis_id, category: raw.category.clone(), priority, title: raw.title.clone(), /* ... */ }
        })
        .collect()
}
```

No mutations. No loop variables. Input data in, transformed data out.

## Anti-Patterns

### God Components

```svelte
<!-- WRONG: component that fetches data, manages state, AND renders UI -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  let sessions = $state([]);
  let messages = $state([]);
  let artifacts = $state([]);
  // 200 lines of data fetching, error handling, state management...
</script>
```

**Fix:** Split into three features. Each has its own store, components, commands. The page composes them.

### Thick Command Handlers

```rust
// WRONG: business logic inside a #[tauri::command]
#[tauri::command]
pub fn analyze_project(...) -> Result<Analysis, OrqaError> {
    // 150 lines of scanning, parsing, scoring...
}
```

**Fix:** Move the logic to `domain/analysis.rs`. The command becomes a two-line delegation.

### Store-to-Store Circular Dependencies

```typescript
// FORBIDDEN: store A imports store B, store B imports store A
```

**Fix:** Coordinate via a parent component's `$effect` blocks.

### Tightly Coupled Features

```rust
// WRONG: artifact module directly calls enforcement module
pub fn create_artifact(...) -> Result<Artifact, OrqaError> {
    enforcement_engine.evaluate_file(&artifact.path, &artifact.content)?; // tight coupling
    Ok(artifact)
}
```

**Fix:** The command handler is the composition point. Each domain module stays independent.

## Composability Checklist

Before writing or reviewing code, verify:

- [ ] **Pure functions:** Can each function be tested without a database, filesystem, or network?
- [ ] **Small units:** Is every function under 50 lines? Under 30 for domain logic?
- [ ] **Single responsibility:** Does each function do exactly one thing?
- [ ] **Typed boundaries:** Are interfaces defined as traits (Rust) or TypeScript types, not concrete implementations?
- [ ] **Swappable integrations:** Could the AI provider, database, or tool executor be replaced without changing the domain?
- [ ] **Pipeline flow:** Are complex operations expressed as chains of transforms, not nested conditionals?
- [ ] **Feature isolation:** Could this feature be removed without breaking other features?
- [ ] **Unidirectional dependencies:** Do stores and modules flow in one direction (component -> store -> command -> domain)?

## Platform Principle

Composability is not just for OrqaStudio's own codebase — it is a principle the platform promotes for every project it manages. When OrqaStudio initializes a new project, the governance framework, agent definitions, and coding standards should guide developers toward composable architecture by default. In a world of ever-changing demands, composable software is dramatically easier to pivot and extend. This skill encodes that belief into the development process.

## See Also

- `docs/architecture/decisions.md` — AD-017 defines the composability principle
- `src-tauri/src/domain/enforcement_engine.rs` — canonical example of pure function composition
- `src-tauri/src/error.rs` — canonical example of error composition via From traits
- `sidecar/src/provider.ts` — canonical example of a pluggable integration boundary

## Related Skills

- **orqa-domain-services** — how domain services compose without framework deps
- **orqa-repository-pattern** — how persistence composes via consistent repo interfaces
- **orqa-store-orchestration** — how frontend stores coordinate without coupling
- **orqa-error-composition** — how errors compose across layers via From traits
- **orqa-ipc-patterns** — the four-layer feature structure and IPC contracts
