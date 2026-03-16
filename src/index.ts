// Graph SDK
export { artifactGraphSDK, ARTIFACT_TYPE_COLORS } from "./graph/artifact-graph.svelte.js";
export type { ArtifactGraphConfig, NodePosition, GraphHealth, BackboneArtifact, KnowledgeGaps } from "./graph/artifact-graph.svelte.js";

// IPC utilities
export { invoke, extractErrorMessage, createStreamChannel } from "./ipc/invoke.js";

// Frontmatter parser
export { parseFrontmatter } from "./utils/frontmatter.js";
export type { FrontmatterResult } from "./utils/frontmatter.js";

// Stores
export { sessionStore } from "./stores/session.svelte.js";
export { projectStore } from "./stores/project.svelte.js";
export { artifactStore } from "./stores/artifact.svelte.js";
export { conversationStore } from "./stores/conversation.svelte.js";
export type { ToolCallState, ContextEntry, PendingApproval } from "./stores/conversation.svelte.js";
export { enforcementStore } from "./stores/enforcement.svelte.js";
export { lessonStore } from "./stores/lessons.svelte.js";
export { setupStore } from "./stores/setup.svelte.js";
export { settingsStore } from "./stores/settings.svelte.js";
export type { ThemeMode, DefaultModel } from "./stores/settings.svelte.js";
export { errorStore, initBrowserHandlers } from "./stores/errors.svelte.js";
export type { AppError } from "./stores/errors.svelte.js";
