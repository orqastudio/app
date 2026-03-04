---
name: block-any-type
enabled: true
event: file
action: block
conditions:
  - field: file_path
    operator: regex_match
    pattern: ui/.*\.(ts|svelte)$
  - field: new_text
    operator: regex_match
    pattern: :\s*any\b|as\s+any\b|<any>
---

**BLOCKED: `any` type annotations are forbidden in TypeScript and Svelte files.**

Use a specific type, generic, or `unknown` with type narrowing instead. `any` defeats the purpose of strict typing.

```typescript
// WRONG
function parse(data: any): string { ... }

// RIGHT
function parse(data: unknown): string { ... }
function parse<T extends Record<string, unknown>>(data: T): string { ... }
```

See: `.claude/rules/coding-standards.md` — "Strict TypeScript: `strict: true`. No `any` types. No `@ts-ignore`."
