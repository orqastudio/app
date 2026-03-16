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
	DEFAULT_ARTIFACT_LINK_COLORS,
	PLATFORM_ARTIFACT_TYPES,
	PLATFORM_RELATIONSHIPS,
	PLATFORM_NAVIGATION,
} from "./project.js";
export type { PlatformArtifactType } from "./project.js";

export type {
	Session,
	SessionSummary,
	SessionStatus,
} from "./session.js";

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

export type { OrqaError } from "./errors.js";

export type {
	SetupStatus,
	SetupStepStatus,
	StepStatus,
	ClaudeCliInfo,
} from "./setup.js";

export type {
	EnforcementRule,
	EnforcementEntry,
	Condition,
	EnforcementViolation,
	StoredEnforcementViolation,
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
} from "./artifact-graph.js";
export { ARTIFACT_TYPES } from "./artifact-graph.js";

export { INVERSE_MAP, SINGLE_REF_FIELDS, ARRAY_REF_FIELDS } from "./constants.js";

// Plugin types
export type {
	PluginManifest,
	PluginProvides,
	ArtifactSchema,
	ArtifactSchemaFrontmatter,
	ViewRegistration,
	WidgetRegistration,
	RelationshipType,
	SettingsRegistration,
	DefaultNavItem,
	NavItemType,
	NavigationItem,
	PluginProjectConfig,
} from "./plugin.js";
