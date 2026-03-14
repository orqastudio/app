---

id: PILLAR-001
title: Clarity Through Structure
description: Making thinking, standards, and decisions visible and structured.
status: active
created: 2026-03-09
updated: 2026-03-09
gate:
  - Does this make governance artifacts visible and manageable?
  - Does it produce structured knowledge (plans, decisions, rules)?
  - Does it enforce a workflow that ensures understanding precedes action?
  - Does it surface what would otherwise be hidden in files, terminal output, or people's heads?
  - Does the system mechanically enforce its own structural rules?
relationships:
  - type: grounded-by
    target: RULE-001
    rationale: Agent delegation enforces structured coordination by requiring explicit role assignment and scoped task handoffs
  - type: grounded-by
    target: RULE-002
    rationale: Architecture decisions are captured as structured artifacts with defined fields, making design rationale visible
  - type: grounded-by
    target: RULE-003
    rationale: Config-driven scanning ensures artifact paths are explicit and verifiable, not guessed or hardcoded
  - type: grounded-by
    target: RULE-004
    rationale: Artifact lifecycle defines visible status transitions and gates that make work progress structured and traceable
  - type: grounded-by
    target: RULE-005
    rationale: Code search usage makes codebase knowledge discoverable through semantic queries rather than hidden in files
  - type: grounded-by
    target: RULE-006
    rationale: Coding standards make quality expectations explicit and machine-verifiable rather than implicit tribal knowledge
  - type: grounded-by
    target: RULE-007
    rationale: Centralised make targets make all development commands visible and discoverable in one place
  - type: grounded-by
    target: RULE-008
    rationale: Documentation-first ensures the target state is visible in docs before implementation begins
  - type: grounded-by
    target: RULE-010
    rationale: End-to-end completeness makes the full feature stack visible by requiring all layers in the same commit
  - type: grounded-by
    target: RULE-012
    rationale: Error ownership makes quality gaps visible by requiring all failures to be acknowledged and fixed
  - type: grounded-by
    target: RULE-013
    rationale: Git workflow with worktrees and commit discipline creates a visible, traceable change history
  - type: grounded-by
    target: RULE-014
    rationale: Historical preservation keeps research and task records visible while ensuring docs reflect current state
  - type: grounded-by
    target: RULE-016
    rationale: Separating IDs from priority makes artifact ordering explicit through scoring rather than implicit from creation order
  - type: grounded-by
    target: RULE-018
    rationale: Banning aliases and hacks keeps data flow transparent by requiring one canonical identifier per concept
  - type: grounded-by
    target: RULE-019
    rationale: No deferred deliverables makes scope commitments visible by requiring everything listed to ship
  - type: grounded-by
    target: RULE-020
    rationale: No stubs ensures implementations are real and visible rather than hiding gaps behind fake data
  - type: grounded-by
    target: RULE-021
    rationale: Pillar alignment sections in docs make the connection between work and guiding principles visible
  - type: grounded-by
    target: RULE-022
    rationale: Plan compliance requires architectural verification and UX design to be visible before implementation starts
  - type: grounded-by
    target: RULE-023
    rationale: Required reading makes prerequisite knowledge explicit by listing the docs agents must load
  - type: grounded-by
    target: RULE-024
    rationale: Reusable components create a visible shared library that prevents hidden duplication across the UI
  - type: grounded-by
    target: RULE-025
    rationale: Root directory cleanliness keeps project structure visible by preventing clutter in the top-level directory
  - type: grounded-by
    target: RULE-026
    rationale: Skill enforcement makes agent capabilities visible through declared skill lists and tiered loading
  - type: grounded-by
    target: RULE-027
    rationale: Structure before work requires visible artifacts (epics, tasks) to exist before any implementation begins
  - type: grounded-by
    target: RULE-028
    rationale: Systems thinking makes architectural relationships and boundaries visible before changes are made
  - type: grounded-by
    target: RULE-031
    rationale: Vision alignment makes the connection between features and guiding principles a visible, required check
  - type: grounded-by
    target: RULE-032
    rationale: Schema validation makes frontmatter constraints visible and machine-enforceable via JSON Schema
  - type: grounded-by
    target: RULE-033
    rationale: Consistent tooltip components make UI interaction hints visible through a structured design system
  - type: grounded-by
    target: RULE-034
    rationale: Standardised artifact link format makes cross-references visible and parseable across all documents
  - type: grounded-by
    target: RULE-035
    rationale: Skill portability constraints make the boundary between core and project knowledge visible and enforced
  - type: grounded-by
    target: RULE-036
    rationale: Context window management makes orchestrator resource usage visible and bounded to prevent degradation
  - type: grounded-by
    target: RULE-037
    rationale: Tool access restrictions make role boundaries visible by explicitly defining what each role can and cannot do
  - type: grounded-by
    target: RULE-038
    rationale: User-invocable semantics make skill accessibility visible by declaring whether users can trigger them directly
  - type: grounded-by
    target: RULE-039
    rationale: Session management makes work state visible across sessions through explicit state files and resume protocols
  - type: grounded-by
    target: RULE-040
    rationale: Provider-agnostic capabilities make tool mapping visible through explicit capability-to-tool resolution tables
  - type: grounded-by
    target: RULE-041
    rationale: Data persistence boundaries make storage decisions visible by defining which channel owns which data
  - type: grounded-by
    target: RULE-042
    rationale: Automated skill injection makes domain knowledge loading visible through explicit path-to-skill mappings
  - type: grounded-by
    target: RULE-043
    rationale: Tooling ecosystem management makes linter-to-standard mappings visible and traceable
  - type: grounded-by
    target: RULE-044
    rationale: Core graph firmware protection makes the boundary between immutable core and editable project content visible
  - type: grounded-by
    target: AD-001
    rationale: Thick backend architecture makes the service layer structure visible with clear domain boundaries
  - type: grounded-by
    target: AD-002
    rationale: IPC boundary design makes the frontend-backend contract visible through a single invoke() interface
  - type: grounded-by
    target: AD-003
    rationale: Result-type error propagation makes error paths visible and explicit rather than hidden via panics
  - type: grounded-by
    target: AD-004
    rationale: Svelte 5 runes make reactive state management visible through explicit $state/$derived/$effect declarations
  - type: grounded-by
    target: AD-006
    rationale: Component purity makes data flow visible by separating fetch responsibility (pages) from display (components)
  - type: grounded-by
    target: AD-007
    rationale: Agent SDK sidecar integration makes the AI communication layer visible as a distinct architectural component
  - type: grounded-by
    target: AD-008
    rationale: Subscription authentication makes access control boundaries visible and structured
  - type: grounded-by
    target: AD-009
    rationale: Streaming pipeline makes the data flow from LLM to UI visible through explicit channel-based architecture
  - type: grounded-by
    target: AD-010
    rationale: Tool implementation as MCP makes AI tool capabilities visible through a standardised protocol
  - type: grounded-by
    target: AD-011
    rationale: Security model makes trust boundaries and access controls visible as explicit architectural decisions
  - type: grounded-by
    target: AD-012
    rationale: Tauri plugin selections make infrastructure choices visible and documented as explicit decisions
  - type: grounded-by
    target: AD-013
    rationale: Frontend library selections make UI technology choices visible and documented as explicit decisions
  - type: grounded-by
    target: AD-017
    rationale: Composability principle makes the design philosophy visible by requiring small, self-contained, combinable units
  - type: grounded-by
    target: AD-019
    rationale: Three-zone layout makes the UI structure visible with clear panel boundaries and purposes
  - type: grounded-by
    target: AD-020
    rationale: Filesystem-driven doc browsing makes documentation structure visible by mirroring the directory tree
  - type: grounded-by
    target: AD-021
    rationale: .orqa/ as single source of truth makes governance data visible in one canonical location
  - type: grounded-by
    target: AD-022
    rationale: Config-driven artifact scanning makes the artifact discovery mechanism visible and configurable
  - type: grounded-by
    target: AD-023
    rationale: Plans merged into research schema simplifies the artifact taxonomy making the planning structure clearer
  - type: grounded-by
    target: AD-024
    rationale: Native search engine makes code knowledge visible through embedded semantic search without external dependencies
  - type: grounded-by
    target: AD-025
    rationale: Provider-agnostic AI integration makes the abstraction layer between app and LLM providers visible
  - type: grounded-by
    target: AD-026
    rationale: Domain service extraction makes backend responsibilities visible by separating domain logic from command handlers
  - type: grounded-by
    target: AD-027
    rationale: Domain-agnostic vision makes the product scope visible as a clarity engine applicable beyond software
  - type: grounded-by
    target: AD-028
    rationale: Three-tier skill loading makes the knowledge injection hierarchy visible with explicit agent/orchestrator/wrapper tiers
  - type: grounded-by
    target: AD-029
    rationale: Universal roles make agent delegation structure visible with seven portable roles instead of domain-specific agents
  - type: grounded-by
    target: AD-030
    rationale: Skill-driven project initialisation makes onboarding visible through structured inference and setup skills
  - type: grounded-by
    target: AD-031
    rationale: Pillars as first-class artifacts make guiding principles visible, queryable, and machine-injectable
  - type: grounded-by
    target: AD-032
    rationale: SQLite scoped to conversations makes persistence boundaries visible with clear channel separation
  - type: grounded-by
    target: AD-033
    rationale: Core UI boundary makes the app's scope visible by limiting core to navigate, search, and edit
  - type: grounded-by
    target: AD-034
    rationale: Schema-driven filtering makes artifact browsing options visible by deriving them from type schemas
  - type: grounded-by
    target: AD-035
    rationale: Config-driven navigation defaults make per-type browsing preferences visible and version-controlled
  - type: grounded-by
    target: AD-036
    rationale: Cross-linking as default behaviour makes artifact relationships visible by rendering links automatically
  - type: grounded-by
    target: AD-037
    rationale: AI-driven search makes cross-artifact knowledge visible through semantic intent-based queries
  - type: grounded-by
    target: AD-038
    rationale: Graph-based knowledge injection makes context loading visible by injecting graph relationships, not raw content
  - type: grounded-by
    target: AD-039
    rationale: Core graph firmware principle makes the immutable/editable boundary visible as an explicit architectural decision
  - type: grounded-by
    target: AD-040
    rationale: Task-first audit trail makes work history visible through structured task artifacts with optional epic grouping
  - type: grounded-by
    target: AD-041
    rationale: CLI rule loading makes the rule injection mechanism visible and explicit until selective loading is built
  - type: grounded-by
    target: AD-042
    rationale: Knowledge maturity pipeline makes the governance learning loop visible as a structured progression
  - type: grounded-by
    target: IMPL-001
    rationale: Lesson on Vite optimisation makes dependency rebuild triggers visible as documented knowledge
  - type: grounded-by
    target: IMPL-002
    rationale: Lesson on killing dev servers makes process cleanup requirements visible and repeatable
  - type: grounded-by
    target: IMPL-003
    rationale: Lesson on dev lifecycle makes orchestrator responsibility for environment management visible
  - type: grounded-by
    target: IMPL-004
    rationale: Lesson on $derived.by() makes Svelte 5 reactivity syntax requirements visible to prevent silent bugs
  - type: grounded-by
    target: IMPL-005
    rationale: Lesson on config-disk alignment makes path integrity requirements visible as a documented pattern
  - type: grounded-by
    target: IMPL-006
    rationale: Lesson on symlinks makes the .claude/.orqa relationship visible to prevent governance divergence
  - type: grounded-by
    target: IMPL-007
    rationale: Lesson on agentic restructuring makes the constraint against self-modifying delegation visible
  - type: grounded-by
    target: IMPL-008
    rationale: Lesson on domain extraction makes the refactoring trigger (monolithic commands) visible as a pattern
  - type: grounded-by
    target: IMPL-009
    rationale: Lesson on domain-neutral naming makes naming conventions visible to prevent future renames
  - type: grounded-by
    target: IMPL-010
    rationale: Lesson on app-native docs makes the requirement for in-app documentation rendering visible
  - type: grounded-by
    target: IMPL-013
    rationale: Lesson on process skills makes the separation between orchestration and implementation skills visible
  - type: grounded-by
    target: IMPL-014
    rationale: Lesson on epic titles makes the naming convention (outcomes, not process words) visible
  - type: grounded-by
    target: IMPL-015
    rationale: Lesson on commit boundaries makes the file accumulation threshold visible as an explicit rule
  - type: grounded-by
    target: IMPL-016
    rationale: Lesson on deferred deliverables makes scope commitment violations visible as a documented anti-pattern
  - type: grounded-by
    target: SKILL-001
    rationale: Architectural evaluation skill makes compliance verification methodology visible and teachable
  - type: grounded-by
    target: SKILL-002
    rationale: Architecture skill makes ADR patterns and structural analysis methodology visible
  - type: grounded-by
    target: SKILL-043
    rationale: Backend best practices skill makes Rust development patterns visible as structured guidance
  - type: grounded-by
    target: SKILL-003
    rationale: ChunkHound skill makes semantic code search capabilities visible and documented
  - type: grounded-by
    target: SKILL-041
    rationale: Component extraction skill makes the methodology for identifying reusable UI components visible
  - type: grounded-by
    target: SKILL-008
    rationale: Composability skill makes the design philosophy of small, combinable units visible as structured methodology
  - type: grounded-by
    target: SKILL-047
    rationale: Epic requirement inference skill makes the process of deriving requirements from epics visible
  - type: grounded-by
    target: SKILL-042
    rationale: Frontend best practices skill makes Svelte/TypeScript development patterns visible as structured guidance
  - type: grounded-by
    target: SKILL-007
    rationale: Governance maintenance skill makes the process of maintaining rules, skills, and agents visible
  - type: grounded-by
    target: SKILL-050
    rationale: Migration and link verification skill makes the process of validating artifact integrity visible
  - type: grounded-by
    target: SKILL-038
    rationale: Artifact audit methodology makes the process of systematic artifact review visible and repeatable
  - type: grounded-by
    target: SKILL-005
    rationale: Orqa code search skill makes context-aware search resolution visible as a wrapper pattern
  - type: grounded-by
    target: SKILL-037
    rationale: Documentation authoring skill makes writing conventions and cross-referencing patterns visible
  - type: grounded-by
    target: SKILL-009
    rationale: Domain services skill makes backend service anatomy and extraction patterns visible
  - type: grounded-by
    target: SKILL-010
    rationale: Error composition skill makes the error type hierarchy and propagation patterns visible
  - type: grounded-by
    target: SKILL-011
    rationale: Governance patterns skill makes .orqa/ artifact conventions visible as structured knowledge
  - type: grounded-by
    target: SKILL-012
    rationale: IPC patterns skill makes the Tauri invoke contract and type-matching requirements visible
  - type: grounded-by
    target: SKILL-013
    rationale: Native search skill makes the embedded ONNX+DuckDB search architecture visible
  - type: grounded-by
    target: SKILL-020
    rationale: Plugin development skill makes the plugin SDK and extension patterns visible
  - type: grounded-by
    target: SKILL-014
    rationale: Repository pattern skill makes the data access abstraction and trait boundaries visible
  - type: grounded-by
    target: SKILL-039
    rationale: Schema compliance skill makes frontmatter validation patterns visible and teachable
  - type: grounded-by
    target: SKILL-044
    rationale: Search architecture skill makes the full search pipeline design visible as structured knowledge
  - type: grounded-by
    target: SKILL-015
    rationale: Store orchestration skill makes cross-store coordination patterns visible
  - type: grounded-by
    target: SKILL-016
    rationale: Store patterns skill makes runes-based reactive state management patterns visible
  - type: grounded-by
    target: SKILL-017
    rationale: Streaming pipeline skill makes the NDJSON streaming architecture from sidecar to UI visible
  - type: grounded-by
    target: SKILL-019
    rationale: Planning skill makes the planning methodology and plan structure visible as structured guidance
  - type: grounded-by
    target: SKILL-046
    rationale: Skill makes its domain knowledge visible as structured, teachable methodology
  - type: grounded-by
    target: SKILL-021
    rationale: Project inference skill makes the process of detecting project type and stack visible
  - type: grounded-by
    target: SKILL-022
    rationale: Project migration skill makes the process of upgrading project structure visible and repeatable
  - type: grounded-by
    target: SKILL-023
    rationale: Project setup skill makes the initialisation workflow visible as structured steps
  - type: grounded-by
    target: SKILL-024
    rationale: Software project type skill makes software-specific conventions visible as structured patterns
  - type: grounded-by
    target: SKILL-026
    rationale: Restructuring methodology makes safe incremental refactoring patterns visible
  - type: grounded-by
    target: SKILL-049
    rationale: Rule enforcement architecture skill makes the full enforcement pipeline visible as structured knowledge
  - type: grounded-by
    target: SKILL-027
    rationale: Rust async patterns skill makes concurrent programming conventions visible
  - type: grounded-by
    target: SKILL-028
    rationale: Security audit skill makes the vulnerability assessment methodology visible and repeatable
  - type: grounded-by
    target: SKILL-029
    rationale: Skills maintenance skill makes the skill lifecycle and update processes visible
  - type: grounded-by
    target: SKILL-030
    rationale: Svelte 5 best practices skill makes runes patterns and component conventions visible
  - type: grounded-by
    target: SKILL-040
    rationale: Systems thinking skill makes the methodology for identifying systems, boundaries, and relationships visible
  - type: grounded-by
    target: SKILL-031
    rationale: Tailwind design system skill makes UI styling conventions and component variants visible
  - type: grounded-by
    target: SKILL-032
    rationale: Tauri v2 development skill makes desktop app framework patterns visible
  - type: grounded-by
    target: SKILL-034
    rationale: TypeScript advanced types skill makes type-level programming patterns visible
  - type: grounded-by
    target: SKILL-036
    rationale: UX compliance review skill makes the methodology for verifying UI against specs visible
  - target: AD-043
    type: grounded-by
    rationale: Auto-generated inverse of grounded-by relationship from AD-043
  - target: RULE-045
    type: grounded-by
    rationale: Auto-generated inverse of grounded-by relationship from RULE-045
  - target: PILLAR-003
    type: informs
    rationale: Auto-generated inverse of informs relationship from PILLAR-003
  - target: AD-046
    type: grounded
    rationale: Auto-generated inverse of grounded relationship from AD-046
  - target: IMPL-046
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-046
  - target: IMPL-050
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-050
  - target: IMPL-026
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-026
  - target: IMPL-057
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-057
  - target: IMPL-064
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from IMPL-064
  - target: DOC-036
    type: documented-by
    rationale: Referenced in documentation page Artifact Framework
  - target: AGENT-003
    type: grounded
    rationale: Orchestrator is grounded by this pillar — Clarity Through Structure is injected as foundational context
  - target: DOC-064
    type: informs
    rationale: Clarity Through Structure pillar is distilled into the product-purpose grounding document — inverse of informed-by on DOC-064
  - target: IDEA-096
    type: informs
    rationale: "Auto-generated inverse of informs relationship from IDEA-096"
---
## What This Pillar Means

Clarity Through Structure is the principle that thinking, standards, and decisions must be visible and structured — not hidden in people's heads, buried in terminal output, or scattered across incompatible files.

This pillar governs features that:

- **Make governance tangible** — Rules, agents, skills, and hooks are browsable, editable documents, not invisible config files
- **Produce structured knowledge** — Plans, decisions, and research are first-class artifacts with frontmatter, connections, and lifecycle states
- **Enforce understanding before action** — Documentation-first workflow, plan approval gates, definition of ready
- **Surface hidden information** — AI transparency (system prompts, context injection, thinking), scanner dashboards, compliance indicators

## Examples of Work That Serves This Pillar

- Artifact browser that renders `.orqa/` content as navigable documents
- Rule editor that lets users view and modify enforcement rules in-app
- System prompt transparency showing what context the AI receives
- Scanner dashboard displaying pass/fail trends and violation details
- Architecture decision records that capture why the system is built this way

## Anti-Patterns

- Features that add capability without making governance more visible
- Tools that work silently without surfacing what they do
- Hiding complexity behind automation without providing an inspection layer
- Adding features that don't produce or organize structured knowledge

## Conflict Resolution

Pillars are equal in importance. When this pillar appears to conflict with Pillar 2 (Learning Through Reflection), the conflict should be flagged to the user for resolution rather than one pillar automatically winning. Agents do not prioritise one pillar over another unilaterally.
