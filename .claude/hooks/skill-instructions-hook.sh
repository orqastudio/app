#!/bin/bash
# UserPromptSubmit hook for skill-aware responses

cat <<'EOF'
REQUIRED: SKILL LOADING PROTOCOL

Before writing any code, complete these steps in order:

1. SCAN each project skill below and decide: LOAD or SKIP (with brief reason)
   - chunkhound (ALWAYS LOAD — mandatory for code search)
   - planning
   - skills-maintenance
   - architecture
   - svelte5-best-practices
   - typescript-advanced-types
   - tailwind-design-system
   - rust-async-patterns
   - tauri-v2

2. For every skill marked LOAD → immediately invoke Skill(name)
   If none need loading → write "Proceeding without skills"

3. DISCOVER: If the task involves a domain not covered by project skills above,
   search and AUTO-INSTALL from the open skills ecosystem. Do NOT ask permission.
   Run: `npx skills find <query>` then `npx skills add <best-match> -y`
   Then invoke Skill(<installed-name>) to load it.

4. DOCUMENTATION-FIRST (MANDATORY): Before writing ANY code, verify that documentation
   exists for the feature area and read it. Documentation defines the target state; code
   is the implementation. See .claude/rules/documentation-first.md for the full workflow:
   Document → Approve → Implement → Verify. Use code_research to find relevant docs.

5. Only after steps 2-4 complete may you begin coding.

IMPORTANT: Skipping step 2 invalidates step 1. Always call Skill() for relevant items.
Step 4 (code_research) is now mandatory for ALL tasks, not just multi-file changes.
Step 5 (documentation review) is mandatory for ALL code changes.

Sample output:
- svelte5-best-practices: LOAD - building components
- typescript-advanced-types: SKIP - not needed for this task
- tailwind-design-system: LOAD - styling work
- rust-async-patterns: LOAD - backend work
- tauri-v2: LOAD - IPC commands
- DISCOVER: task involves drag-and-drop → `npx skills find drag-drop`

Then call:
> Skill(svelte5-best-practices)
> Skill(tailwind-design-system)
> Skill(rust-async-patterns)
> Skill(tauri-v2)

Now implementation can begin.
EOF
