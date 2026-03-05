---
scope: system
---

# No Aliases or Hacky Fixes (NON-NEGOTIABLE)

When a value or type mismatch exists between layers, fix the root cause. Never paper over it with aliases, shims, or duplicate mappings.

## What Counts as an Alias or Hack

- Duplicate entries in TypeScript union types to handle both spellings of the same concept
- Widened type unions: adding `| "deploy"` to a type union so the frontend tolerates a backend value it shouldn't be producing
- `TYPE_ALIASES` or `ID_NORMALIZER` maps in the frontend that rewrite backend values before use
- Fallback lookups that hide missing entries: `labels[key] ?? key`
- Duplicate entries in capability maps, enum-equivalent unions, or label maps that refer to the same concept under two different keys
- Transform/adapter layers inserted between Rust and TypeScript that should already agree on a schema
- Rust `match` arms that handle both the correct and incorrect variant names
- `#[serde(alias = "...")]` used to tolerate incorrect field names instead of fixing the source

## The Root Cause Rule

Data flows through layers: Rust domain -> Rust command -> IPC serialization -> TypeScript type -> Svelte store -> Svelte component. A mismatch at any layer must be fixed at the layer that **introduced** the mismatch.

| Scenario | Wrong fix | Correct fix |
|----------|-----------|-------------|
| Rust returns `"deploy"`, frontend expects `"space_deploy"` | Add `"deploy"` to TypeScript union | Fix the Rust serialization to produce `"space_deploy"` |
| IPC serialization drops a field the frontend needs | Add a null-check fallback in the component | Add the field to the Rust response struct |
| TypeScript union is missing a value Rust can produce | Widen the union with an alias | Fix Rust to only produce canonical values, add the single canonical value to the union |
| Two label map keys map to the same display string | Keep both keys | Remove the non-canonical key everywhere and use one canonical key |

## FORBIDDEN Patterns

```typescript
// FORBIDDEN: alias entry alongside the canonical key
const STATUS_LABELS: Record<string, string> = {
  scan_complete: "Scan Complete",
  complete: "Scan Complete",       // alias — FORBIDDEN
};

// FORBIDDEN: widened union to tolerate a backend bug
type ScanStatus = "scan_complete" | "running" | "complete";  // "complete" is an alias — FORBIDDEN

// FORBIDDEN: normalizer map hiding the real problem
const STATUS_ALIASES: Record<string, ScanStatus> = {
  complete: "scan_complete",
};

// FORBIDDEN: fallback that hides a missing entry
const label = STATUS_LABELS[status] ?? status;  // missing entry = a bug, not a default
```

```rust
// FORBIDDEN: serde alias to tolerate incorrect field names
#[derive(Deserialize)]
struct Config {
    #[serde(alias = "scanDir")]  // hiding that some source writes "scanDir" instead of "scan_dir"
    scan_dir: String,
}
```

## Required Pattern

1. **Identify which layer introduced the wrong value** — use `code_research` or `search_regex` to trace the value from its origin to the mismatch point
2. **Fix that layer** — normalize at the source (Rust struct, serde configuration, type definition)
3. **Remove all downstream workarounds** — delete alias entries, revert widened unions, remove normalizer maps
4. **Verify consistency** — every layer uses the same single canonical identifier; `search_regex` for the old alias to confirm it is gone

## One Canonical Identifier Per Concept

Every status value, type discriminator, and configuration key MUST appear exactly once across:

- Rust enum variants and struct field names
- Serde serialization output
- TypeScript type unions and interfaces
- Label maps and capability maps in the frontend

If the same concept has two IDs in any of these locations, that is a bug — not a feature.

## Related Rules

- `end-to-end-completeness.md` — type consistency must hold across all layers in the same commit
- `error-ownership.md` — a type mismatch is an error you own; fix it, don't work around it
- `no-stubs.md` — alias entries are a form of fake data hiding real gaps
- `chunkhound-usage.md` — use `search_regex` to find all usages of an identifier before renaming or removing it
