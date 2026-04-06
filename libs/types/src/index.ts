export type {
	Project,
	ProjectSummary,
	DetectedStack,
	ScanResult,
	ProjectSettings,
	ChildProjectConfig,
	GovernanceCounts,
	ProjectScanResult,
	ArtifactTypeConfig,
	ArtifactGroupConfig,
	ArtifactEntry,
	ProjectRelationshipConfig,
	DeliveryConfig,
	DeliveryTypeConfig,
	DeliveryParentConfig,
	RelationshipDisplayConfig,
	ArtifactLinkDisplayMode,
	ArtifactLinksConfig,
	StatusAutoRule,
	StatusDefinition,
} from "./project.js";
export {
	isArtifactGroup,
	PLATFORM_CONFIG,
	PLATFORM_ARTIFACT_TYPES,
	PLATFORM_RELATIONSHIPS,
	PLATFORM_SEMANTICS,
	PLATFORM_NAVIGATION,
} from "./project.js";
export type { PlatformArtifactType } from "./project.js";

export type { Session, SessionSummary, SessionStatus } from "./session.js";

export type {
	Message,
	MessageRole,
	ContentType,
	StreamStatus,
	MessageId,
	SearchResult,
} from "./message.js";

export type {
	Artifact,
	ArtifactSummary,
	ArtifactType,
	ComplianceStatus,
	ArtifactRelationship,
} from "./artifact.js";

export type {
	NavReadme,
	NavTree,
	NavGroup,
	NavType,
	NavDocNode,
	DocNode,
	FilterableField,
	SortableField,
	SortConfig,
	LayoutSection,
	NavigationLayout,
	NavigationDefaults,
	NavigationConfig,
	ArtifactViewState,
} from "./nav-tree.js";

export type {
	ResolvedTheme,
	ThemeToken,
	SidecarStatus,
	SidecarState,
	StartupTask,
	StartupSnapshot,
} from "./settings.js";

export type { StreamEvent } from "./streaming.js";

// Workflow types
export { STATE_CATEGORIES } from "./workflow.js";
export type {
	StateCategory,
	GuardType,
	FieldCheckOperator,
	RelationshipCheckCondition,
	QueryExpectedResult,
	FieldCheckParams,
	RelationshipCheckParams,
	QueryGuardParams,
	RoleCheckParams,
	CodeHookGuardParams,
	GuardParams,
	Guard,
	ActionType,
	NotifyChannel,
	NotifySeverity,
	SetFieldParams,
	AppendLogParams,
	CreateArtifactParams,
	NotifyParams,
	CodeHookActionParams,
	ActionParams,
	Action,
	WorkflowState,
	Transition,
	GatePattern,
	GateTimeoutAction,
	GateTimeout,
	GatePresentSection,
	GateVerdict,
	GatePhaseGather,
	GatePhasePresent,
	GatePhaseCollect,
	GatePhaseExecute,
	GatePhaseLearn,
	GatePhases,
	Gate,
	ContributionPoint,
	WorkflowVariant,
	SelectionRule,
	WorkflowDefinition,
} from "./workflow.js";

export type { OrqaError } from "./errors.js";

export type { CanonicalHookEvent, HookContext, HookResult, HookViolation } from "./hooks.js";

export type { SetupStatus, SetupStepStatus, StepStatus, ClaudeCliInfo } from "./setup.js";

export type {
	EnforcementRule,
	EnforcementEntry,
	Condition,
	EnforcementViolation,
	StoredEnforcementViolation,
	EnforcementEvent,
	EnforcementResponse,
	EnforcementResult,
	EnforcementResolution,
} from "./enforcement.js";

export type { Lesson, NewLesson, LessonStatus, LessonCategory } from "./lessons.js";

export type {
	ArtifactNode,
	ArtifactRef,
	GraphStats,
	ArtifactGraphType,
	CanonicalStatus,
	ArtifactStatus,
	IntegrityCategory,
	IntegritySeverity,
	IntegrityCheck,
	ProposedTransition,
	AppliedFix,
	HealthSnapshot,
	GraphHealthData,
	AncestryNode,
	AncestryChain,
	TracedArtifact,
	TraceabilityResult,
} from "./artifact-graph.js";

export { buildInverseMap, hasSemantic, keysForSemantic } from "./constants.js";

// Schema-generated types (namespace-qualified to avoid collisions with existing hand-written types)
export * as Generated from "./generated/index.js";

// Plugin types
export type {
	PluginManifest,
	MergeDecision,
	KeyCollision,
	PluginProvides,
	ArtifactSchema,
	ArtifactSchemaFrontmatter,
	EnforcementDeclaration,
	EnforcementActions,
	ActionDeclaration,
	WatchDeclaration,
	ViewRegistration,
	WidgetRegistration,
	RelationshipType,
	PlatformArtifactType as PlatformArtifactTypeInterface,
	RelationshipSemantic,
	PlatformConfig,
	SettingsRegistration,
	DefaultNavItem,
	NavItemType,
	NavigationItem,
	RelationshipConstraints,
	RelationshipStatusRule,
	PluginProjectConfig,
	AliasMapping,
	ConflictResolutionSuggestion,
	SystemRequirement,
	SidecarRegistration,
	CliToolRegistration,
	HookRegistration,
	CliToolRunResult,
	CliToolRunStatus,
	HookGenerationResult,
	ProviderConfig,
	RegistryEntry,
	RegistryCatalog,
	PluginLockEntry,
	PluginInstallProgress,
	PluginUpdate,
	DiscoveredPlugin,
	PluginContentMapping,
	PluginDependencies,
	PluginLifecycle,
	BundledAgentRef,
	BundledKnowledgeRef,
	PluginSymlinkDeclaration,
	PluginAggregatedFile,
	KnowledgeInjectionTier,
	PromptPriority,
	KnowledgeDeclaration,
	PromptSectionType,
	PromptSection,
	ArtifactViewerDeclaration,
	RoleDefinition,
	SettingsPageDeclaration,
	WorkflowRegistration,
	SchemaCategory,
	PipelineStageConfig,
} from "./plugin.js";

/**
 * Exhaustiveness check utility. Use as the default case in switch statements
 * over discriminated unions to get a compile-time error when a new variant
 * is added but not handled.
 * @param value - The value that should never be reached; TypeScript enforces this at compile time.
 * @param message - Optional custom error message to throw instead of the default.
 * @example
 * ```ts
 * type Status = "active" | "completed" | "error";
 * function handle(s: Status) {
 *   switch (s) {
 *     case "active": return "...";
 *     case "completed": return "...";
 *     case "error": return "...";
 *     default: return assertNever(s);
 *   }
 * }
 * ```
 */
export function assertNever(value: never, message?: string): never {
	throw new Error(message ?? `Unexpected value: ${value as unknown}`);
}

/**
 * Recursive readonly type. Makes all properties and nested objects/arrays
 * deeply immutable at the type level. Use for data crossing the IPC boundary.
 * @example
 * ```ts
 * type FrozenProject = DeepReadonly<Project>;
 * // FrozenProject.name is readonly string
 * // FrozenProject.sessions[0].title is readonly string
 * ```
 */
export type DeepReadonly<T> = T extends (infer U)[]
	? ReadonlyArray<DeepReadonly<U>>
	: T extends Map<infer K, infer V>
		? ReadonlyMap<DeepReadonly<K>, DeepReadonly<V>>
		: T extends Set<infer U>
			? ReadonlySet<DeepReadonly<U>>
			: T extends object
				? { readonly [K in keyof T]: DeepReadonly<T[K]> }
				: T;
