# Review: Target Enforcement Configs

## Verdict: PASS

All 10 acceptance criteria pass. Two minor observations noted below but none rise to FAIL level.

## Acceptance Criteria

- [x] AC1: ESLint config imports from plugin bases, doesn't duplicate -- **PASS**
- [x] AC2: Clippy workspace lints match research recommendations -- **PASS**
- [x] AC3: clippy.toml settings are appropriate -- **PASS** (with minor observation)
- [x] AC4: tsconfig presets match typescript plugin source exactly -- **PASS**
- [x] AC5: markdownlint handles frontmatter correctly -- **PASS**
- [x] AC6: prettier matches observed codebase style -- **PASS**
- [x] AC7: pre-commit is thin (delegates to orqa CLI + direct tool invocations) -- **PASS**
- [x] AC8: pre-commit covers all 7+ required checks from architecture -- **PASS**
- [x] AC9: post-commit auto-pushes correctly with skip logic -- **PASS**
- [x] AC10: All configs have valid syntax -- **PASS**

## Detailed Evidence

### AC1: ESLint config imports from plugin bases

**PASS.** The target `targets/enforcement/eslint/eslint.config.js` imports from `@orqastudio/plugin-svelte/eslint` (line 10: `import { svelte } from "@orqastudio/plugin-svelte/eslint"`) and spreads the result at line 44: `...svelte(sveltePlugin)`. This is the correct composition pattern.

The Svelte plugin (`plugins/svelte/src/eslint/index.ts`) in turn imports the TypeScript base (`import { base } from "@orqastudio/plugin-typescript/eslint"`), which provides `tseslint.configs.recommended`, `no-explicit-any`, `ban-ts-comment`, `no-unused-vars` with underscore pattern, `no-console`, and test/worker overrides.

The target config adds only project-specific rules:
- Layer 2: Environment globals (browser + node for Tauri)
- Layer 3: Tauri-specific `svelte/no-navigation-without-resolve: off`
- Layer 4: Project architecture rules (RULE-006 invoke restriction, RULE-033 title attribute restriction)
- Layer 5: Ignores

No duplication of plugin-provided rules exists. This matches the research recommendation exactly.

**Note:** ARCHITECTURE.md A.4 lists three separate ESLint files (`base.config.js`, `svelte.config.js`, `app.config.js`) but the target has a single `eslint.config.js`. This is architecturally correct -- the plugin bases are consumed via npm imports, not as separate target files. The A.4 listing is aspirational/illustrative and the single-file approach is the right implementation.

### AC2: Clippy workspace lints match research recommendations

**PASS.** The target `workspace-lints.toml` contains:

- `[rust]` section with `rust_2018_idioms`, `unsafe_code = "deny"`, `unused_qualifications` -- matches research section 5
- `[clippy]` pedantic group at priority -1 -- matches current Cargo.toml and research
- All 12 pedantic allows from research: `needless_pass_by_value`, `module_name_repetitions`, `must_use_candidate`, `missing_errors_doc`, `missing_panics_doc`, `doc_markdown`, 5 cast lints, `struct_excessive_bools`, `similar_names`, `unreadable_literal`, `single_match_else`, `items_after_statements`, `unnecessary_wraps` -- matches research section 2 "Missing allows that will cause friction"
- Restriction cherry-picks: `unwrap_used`, `dbg_macro`, `todo`, `print_stdout`, `print_stderr`, `str_to_string`, `string_to_string`, `clone_on_ref_ptr`, `rest_pat_in_fully_bound_structs` -- matches research section 3
- Nursery cherry-picks: `or_fun_call`, `redundant_pub_crate`, `use_self` -- matches research section 4
- `expect_used` intentionally NOT included (research: 745 occurrences, is the convention)

Every lint in the target file has a comment explaining why. The lint levels (deny vs warn vs allow) match the research rationale.

### AC3: clippy.toml settings are appropriate

**PASS.** The target `clippy.toml` contains:
- `too-many-lines-threshold = 50` -- matches RULE-006 coding standards (hard ceiling 50 lines), consistent with research
- `allow-unwrap-in-tests = true` -- matches research recommendation
- `allow-expect-in-tests = true` -- matches research recommendation

**Minor observation:** `cognitive-complexity-threshold = 20` is set, but the `cognitive_complexity` lint is NOT enabled in `workspace-lints.toml`. The research explicitly recommended NOT enabling this lint ("known to be inaccurate, the Clippy team themselves don't recommend it"). The threshold setting is therefore dead configuration -- it has no effect. This is harmless but could be confusing. Not a FAIL because it causes no behavioral issues, but the implementer could remove it for clarity.

### AC4: tsconfig presets match typescript plugin source exactly

**PASS.** Byte-for-byte comparison:

| File | Target (`targets/enforcement/tsconfig/`) | Plugin source (`plugins/typescript/src/tsconfig/`) | Match? |
|------|------|------|------|
| `base.json` | strict, noUncheckedIndexedAccess, esModuleInterop, skipLibCheck, forceConsistentCasingInFileNames, isolatedModules, resolveJsonModule | Identical | YES |
| `app.json` | extends base, ES2022, ESNext module, bundler resolution, DOM libs, verbatimModuleSyntax, noEmit | Identical | YES |
| `library.json` | extends base, ES2022, NodeNext module+resolution, declaration, declarationMap, sourceMap, node types | Identical | YES |

All three files are exact copies of the plugin source. The `$schema` references and `extends` paths are identical.

### AC5: markdownlint handles frontmatter correctly

**PASS.** Two critical frontmatter settings are correct:

1. `MD041: false` (line 24) -- disables "first line should be a top-level heading" because all OrqaStudio artifacts start with `---` YAML frontmatter, not `# Heading`. This matches research finding: "Every governance artifact starts with YAML frontmatter."

2. `MD025: { "front_matter_title": "" }` (line 12) -- tells markdownlint not to treat the frontmatter `title:` field as a competing H1 heading. Empty string means "no frontmatter title field name" so the `# Title` heading in the body is the only H1. This matches research recommendation exactly.

Additional relevant settings: `MD013: false` (no line length -- research: tables, Mermaid, YAML descriptions exceed 80 chars), `MD034: false` (bare URLs allowed in research artifacts), `MD036: false` (bold-as-label pattern used in artifacts), `MD033: { "allowed_elements": ["br"] }` (Mermaid `<br/>` tags).

### AC6: prettier matches observed codebase style

**PASS.** Verified against actual codebase files:

| Setting | .prettierrc value | Observed in codebase | Match? |
|---------|----------|----------|------|
| Indentation | `"useTabs": true` | `app/src/routes/+layout.svelte` uses tabs | YES |
| Quotes | Default (double) | `import { onMount } from "svelte"` | YES |
| Semicolons | Default (`true`) | All statements end with `;` | YES |
| Print width | `"printWidth": 100` | Lines observed in 80-120 range | YES |
| End of line | `"endOfLine": "lf"` | Windows dev env needs explicit LF | YES |

Plugins correctly ordered: `prettier-plugin-svelte` before `prettier-plugin-tailwindcss` (tailwind must be last per research).

The `.prettierignore` correctly excludes: build artifacts, node_modules, generated files, `src-tauri/` (Rust, formatted by rustfmt), `.orqa/` and `.state/` (governance markdown, formatted by markdownlint).

Settings left at defaults (`semi`, `singleQuote`, `trailingComma`) are correctly omitted -- research confirmed defaults match the codebase convention.

### AC7: pre-commit is thin

**PASS.** The pre-commit script contains zero business logic. It:

1. Detects staged file types via `git diff --cached` (shell, not business logic)
2. Delegates artifact work to `$ORQA fix autolink --staged` and `$ORQA validate --staged`
3. Delegates Rust checks to `$ORQA check rustfmt` and `$ORQA check clippy`
4. Delegates frontend lint to `npx eslint` (direct tool invocation)
5. Delegates markdown lint to `npx markdownlint-cli2` (direct tool invocation)
6. Delegates stub scanning to `$ORQA check stubs --staged`
7. Delegates suppression audit to `$ORQA check lint-suppressions --staged`
8. Delegates tests to `$ORQA check test-rust --staged` and `$ORQA check test-frontend --staged`

The only "logic" is the `log_violation()` function for stability tracking and the `find_root()` function for project root detection -- both are hook infrastructure, not business logic.

The CLI fallback (`node $PROJECT_ROOT/libs/cli/dist/cli.js`) handles the case where `orqa` is not on PATH.

### AC8: pre-commit covers all 7+ required checks from architecture

**PASS.** ARCHITECTURE.md A.4 specifies 7 pre-commit target checks:

| A.4 Check | Script Coverage | Section |
|-----------|----------------|---------|
| 1. Artifact frontmatter validation | `$ORQA validate --staged` | Section 2 |
| 2. Relationship validation | `$ORQA validate --staged` | Section 2 |
| 3. Schema compliance | `$ORQA validate --staged` | Section 2 |
| 4. Lint checks (eslint/clippy) | `$ORQA check clippy`, `npx eslint` | Sections 3, 4 |
| 5. Tests affected by staged changes | `$ORQA check test-rust --staged`, `$ORQA check test-frontend --staged` | Section 8 |
| 6. Knowledge size constraints | `$ORQA validate --staged` (per comment at line 20-21) | Section 2 |
| 7. Status value validity | `$ORQA validate --staged` (per comment at line 20-21) | Section 2 |

All 7 are covered. The script also adds 4 checks beyond A.4's minimum:
- Auto-linking (mutation step, section 1)
- Markdownlint (section 5)
- Stub scanner (section 6)
- Lint suppression audit (section 7)

These extras are consistent with the research recommendations and don't conflict with A.4.

**Note:** Prettier format checking is NOT in the pre-commit. The research (githooks-research.md, open question #3) flagged this as a deliberate omission. The ARCHITECTURE.md A.4 lists prettier under enforcement *configs* but does not list it in the pre-commit target checks. This is an acceptable gap -- prettier can be added later when the format baseline is established.

### AC9: post-commit auto-pushes correctly with skip logic

**PASS.** The post-commit script correctly:

1. Uses `set -uo pipefail` (NOT `set -e` -- intentional, push failure must not crash the hook)
2. Skips during rebase (`rebase-merge`, `rebase-apply` directories)
3. Skips during merge (`MERGE_HEAD` file)
4. Skips during cherry-pick (`CHERRY_PICK_HEAD` file)
5. Skips during revert (`REVERT_HEAD` file)
6. Skips if no remote configured (`git remote get-url origin`)
7. Skips on detached HEAD (empty `git branch --show-current` -- handles bisect)
8. Sets upstream on first push: `git push -u origin "$BRANCH"`
9. Falls back to simple `git push` when upstream already set
10. Warns on failure instead of blocking (correct -- post-commit cannot prevent the commit)

The skip logic covers all standard automated git operations. The bisect case (detached HEAD) is handled at line 53.

### AC10: All configs have valid syntax

**PASS.** Validated all config files:

| File | Format | Validation Method | Result |
|------|--------|-------------------|--------|
| `eslint/eslint.config.js` | JavaScript ESM | Readable, 109 lines, valid structure | VALID |
| `clippy/clippy.toml` | TOML | Line-by-line key=value parse | VALID |
| `clippy/workspace-lints.toml` | TOML | Line-by-line parse, sections `[rust]` and `[clippy]` | VALID |
| `tsconfig/base.json` | JSON | `JSON.parse()` | VALID |
| `tsconfig/app.json` | JSON | `JSON.parse()` | VALID |
| `tsconfig/library.json` | JSON | `JSON.parse()` | VALID |
| `markdownlint/.markdownlint.json` | JSON | `JSON.parse()` | VALID |
| `prettier/.prettierrc` | JSON | `JSON.parse()` | VALID |
| `prettier/.prettierignore` | gitignore format | Line-by-line, valid patterns | VALID |
| `githooks/pre-commit` | Bash | `bash -n` syntax check | VALID |
| `githooks/post-commit` | Bash | `bash -n` syntax check | VALID |

## Issues Found

No blocking issues. Two minor observations:

1. **Dead config in clippy.toml** (`targets/enforcement/clippy/clippy.toml:21`): `cognitive-complexity-threshold = 20` configures a threshold for the `cognitive_complexity` lint, but that lint is not enabled in `workspace-lints.toml`. The research explicitly recommended NOT enabling it. The setting is harmless but could be removed for clarity.

2. **File naming vs ARCHITECTURE.md A.4**: A.4 lists `eslint/base.config.js`, `eslint/svelte.config.js`, `eslint/app.config.js` as three separate files, but the target has a single `eslint/eslint.config.js`. This is architecturally correct (plugins are consumed via npm imports, not as separate target files), but the A.4 listing should be updated to match reality when ARCHITECTURE.md is next revised.

## Lessons

- The single-file ESLint config with plugin imports is the correct pattern for flat config composition. Splitting into base/svelte/app files at the target level would create unnecessary indirection since the npm package already provides the layering.
- Dead config values (like threshold settings for disabled lints) should be avoided in target state artifacts to prevent confusion during implementation.
