#!/bin/bash
# UserPromptSubmit hook for skill-aware responses

cat <<'EOF'
REQUIRED: SKILL LOADING PROTOCOL

Before writing any code, complete these steps in order:

1. SCAN each project skill below and decide: LOAD or SKIP (with brief reason)
   - chunkhound (ALWAYS LOAD — mandatory for code search)
   - planning
   - skills-maintenance
   - svelte
   - typescript
   - tailwind

2. For every skill marked LOAD → immediately invoke Skill(name)
   If none need loading → write "Proceeding without skills"

3. DISCOVER: If the task involves a domain not covered by project skills above,
   search and AUTO-INSTALL from the open skills ecosystem. Do NOT ask permission.
   Run: `npx skills find <query>` then `npx skills add <best-match> -y`
   Then invoke Skill(<installed-name>) to load it.

4. DOCUMENTATION-FIRST (MANDATORY): Before writing ANY code, verify that documentation
   exists for the feature area and read it. Documentation defines the target state; code
   is the implementation. See .claude/rules/documentation-first.md for the full workflow:
   Document → Approve → Implement → Verify.

5. Only after steps 2-4 complete may you begin coding.

IMPORTANT: Skipping step 2 invalidates step 1. Always call Skill() for relevant items.
Step 4 (documentation review) is mandatory for ALL code changes.

Sample output:
- svelte: LOAD - building components
- typescript: SKIP - not needed for this task
- tailwind: LOAD - styling work
- DISCOVER: task involves Rust/Tauri → `npx skills find tauri`

Then call:
> Skill(svelte)
> Skill(tailwind)

Now implementation can begin.
EOF
