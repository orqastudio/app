---
id: KNOW-c7fb7c83
type: knowledge
name: Tech Debt Management
status: active
relationships:
  - target: DOC-343aeb1f
    type: synchronised-with
  - target: DOC-a1b2c3d4
    type: synchronised-with

---

# Tech Debt Management

Tech debt is the gap between what the code says and what the schemas define. In OrqaStudio, the schemas (core.json, plugin manifests) are the source of truth. Any code that hardcodes what the schemas should provide is debt.

## Zero-Debt Principle

The goal is not "low debt" — it's **zero debt**. Every architectural change must be followed through to completion before new work begins. Partial migrations are worse than no migration because they create two systems that must be understood simultaneously.

## What Creates Debt

1. **Schema changes without code follow-through** — changing core.json relationships but not updating components that reference the old keys
2. **Duplicate implementations** — two files doing the same thing (e.g. stale config copies, duplicated helper functions)
3. **Hardcoded values** — relationship keys, artifact types, status values, or paths in code that should come from the schema
4. **In-app manifests out of sync with plugin manifests** — the in-app registration diverging from the canonical orqa-plugin.json
5. **Dead dependencies** — packages listed but never imported
6. **Stale paths** — references to old directory structures after renames

## How to Prevent Debt

### Before making a schema change:
1. **Grep the entire codebase** for the values you're about to change
2. **List every file** that references the old values
3. **Change everything in the same commit** — schema + all references
4. **Run `cargo test` AND `orqa enforce`** before committing

### After making a schema change:
1. Run the tech debt audit: `grep -rn "old-value" app/ libs/ connectors/ plugins/`
2. Check in-app manifests match plugin manifests
3. Verify no hardcoded values bypass the schema
4. Run the full verification suite: `make verify`

### When adding new code:
1. **Never hardcode relationship keys** — read them from the registry or schema
2. **Never hardcode artifact types** — read them from the type registry
3. **Never hardcode paths** — derive from config or conventions
4. **Never duplicate helpers** — extract to a shared module
5. **Never add a dependency you don't import** — check before adding

## How to Detect Debt

### Automated checks (run regularly):
```bash
# Find hardcoded relationship keys in frontend
grep -rn '"delivers"\|"drives"\|"informs"' app/ui/src/ --include="*.ts" --include="*.svelte"

# Find old type references
grep -rn 'ToolRegistration\|ToolRunResult\|allTools' app/ui/src/ libs/sdk/src/

# Find duplicated functions
grep -rn 'fn active_project_path' app/backend/src-tauri/src/

# Find unused dependencies
# (compare package.json deps against actual imports)

# Verify schemas match
diff <(jq '.provides.schemas[].key' plugins/software/orqa-plugin.json) \
     <(grep -o '"key": "[^"]*"' app/ui/src/lib/plugins/software-project/manifest.ts)
```

### Manual review triggers:
- After renaming repos, folders, or types
- After changing the relationship vocabulary
- After adding/removing a plugin
- After absorbing a library into another
- Before any release

## How to Fix Debt

1. **Create a task** for each category of debt found
2. **Fix in isolation** — one category per commit, not mixed with feature work
3. **Verify with tests** — `cargo test`, `make verify`, `orqa enforce`
4. **Update skills and docs** if the fix changes how something works

## Debt Categories

| Category | Detection | Example |
|---|---|---|
| Hardcoded values | grep for string literals | `"delivers"` in component code |
| Stale copies | find duplicate files | Embedded JSON that should be include_str! |
| Dead code | unused imports, unreachable functions | Imported type that was renamed |
| Out-of-sync manifests | diff in-app vs plugin | Different field names for same schema |
| Duplicated helpers | grep for function names | `active_project_path` in 6 files |
| Stale paths | grep for old directory names | `governance/rules` after rename to `process/rules` |
| Dead dependencies | compare deps vs imports | Package in package.json never imported |
