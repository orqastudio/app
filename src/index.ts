// Registry — the primary API for accessing stores
export { initializeStores, getStores } from "./registry.svelte.js";
export type { OrqaStores } from "./registry.svelte.js";

// Graph SDK class + types
export { ArtifactGraphSDK } from "./graph/artifact-graph.svelte.js";
export type { ArtifactGraphConfig } from "./graph/artifact-graph.svelte.js";

// IPC utilities
export { invoke, extractErrorMessage, createStreamChannel } from "./ipc/invoke.js";

// Frontmatter parser
export { parseFrontmatter } from "./utils/frontmatter.js";
export type { FrontmatterResult } from "./utils/frontmatter.js";

// Store classes (for advanced use / testing — prefer getStores() for access)
export {
	SessionStore,
	ProjectStore,
	ArtifactStore,
	ConversationStore,
	EnforcementStore,
	LessonStore,
	SetupStore,
	SettingsStore,
	ErrorStoreImpl,
	NavigationStore,
	ToastStore,
	createToastConvenience,
} from "./stores/index.js";

// Store-related types
export type { ToolCallState, ContextEntry, PendingApproval } from "./stores/index.js";
export type { ThemeMode, DefaultModel } from "./stores/index.js";
export type { AppError } from "./stores/index.js";
export type { ActivityView, ActivityGroup, ExplorerView, SubCategoryConfig } from "./stores/index.js";
export type { Toast, ToastType } from "./stores/index.js";
