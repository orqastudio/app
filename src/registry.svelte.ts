/**
 * Central store registry for the OrqaStudio SDK.
 *
 * Store instances live on `globalThis.__orqa_stores` — NOT a module-level
 * variable. This means the registry works across bundle boundaries:
 *
 * - The app calls `initializeStores()` → creates stores on globalThis
 * - A runtime-loaded plugin imports `@orqastudio/sdk` (its own bundled copy)
 *   and calls `getStores()` → reads from the same globalThis
 *
 * Both the app and every plugin resolve to the same store instances,
 * regardless of how many copies of the SDK exist in memory.
 */

import { ArtifactGraphSDK } from "./graph/artifact-graph.svelte.js";
import { SessionStore } from "./stores/session.svelte.js";
import { ProjectStore } from "./stores/project.svelte.js";
import { ArtifactStore } from "./stores/artifact.svelte.js";
import { ConversationStore } from "./stores/conversation.svelte.js";
import { EnforcementStore } from "./stores/enforcement.svelte.js";
import { LessonStore } from "./stores/lessons.svelte.js";
import { SetupStore } from "./stores/setup.svelte.js";
import { SettingsStore } from "./stores/settings.svelte.js";
import { ErrorStoreImpl } from "./stores/errors.svelte.js";
import { NavigationStore } from "./stores/navigation.svelte.js";
import { ToastStore, createToastConvenience } from "./stores/toast.svelte.js";

/** The full set of SDK store instances. */
export interface OrqaStores {
    artifactGraphSDK: ArtifactGraphSDK;
    sessionStore: SessionStore;
    projectStore: ProjectStore;
    artifactStore: ArtifactStore;
    conversationStore: ConversationStore;
    enforcementStore: EnforcementStore;
    lessonStore: LessonStore;
    setupStore: SetupStore;
    settingsStore: SettingsStore;
    errorStore: ErrorStoreImpl;
    navigationStore: NavigationStore;
    toastStore: ToastStore;
    /** Convenience functions: toast.success(), toast.error(), etc. */
    toast: ReturnType<typeof createToastConvenience>;
}

// ---------------------------------------------------------------------------
// Global bridge key — shared across all bundles in the same window
// ---------------------------------------------------------------------------

const REGISTRY_KEY = "__orqa_stores";

declare global {
    // eslint-disable-next-line no-var
    var __orqa_stores: OrqaStores | undefined;
}

/**
 * Create and register all SDK store instances.
 *
 * Call this exactly once during app startup (e.g. in the root +layout.svelte).
 * Subsequent calls — including from plugin bundles — return the existing instances.
 *
 * @returns The full set of store instances.
 */
export function initializeStores(): OrqaStores {
    if (globalThis[REGISTRY_KEY]) return globalThis[REGISTRY_KEY];

    const artifactGraphSDK = new ArtifactGraphSDK();
    const sessionStore = new SessionStore();
    const projectStore = new ProjectStore();
    const artifactStore = new ArtifactStore();
    const conversationStore = new ConversationStore();
    const enforcementStore = new EnforcementStore();
    const lessonStore = new LessonStore();
    const setupStore = new SetupStore();
    const settingsStore = new SettingsStore();
    const errorStore = new ErrorStoreImpl();
    const navigationStore = new NavigationStore();
    const toastStore = new ToastStore();
    const toast = createToastConvenience(toastStore);

    const stores: OrqaStores = {
        artifactGraphSDK,
        sessionStore,
        projectStore,
        artifactStore,
        conversationStore,
        enforcementStore,
        lessonStore,
        setupStore,
        settingsStore,
        errorStore,
        navigationStore,
        toastStore,
        toast,
    };

    globalThis[REGISTRY_KEY] = stores;
    return stores;
}

/**
 * Access the registered store instances.
 *
 * Works across bundle boundaries — a runtime-loaded plugin calling
 * `getStores()` from its own copy of the SDK will get the same
 * instances the app created.
 *
 * Throws if `initializeStores()` has not been called yet.
 */
export function getStores(): OrqaStores {
    const stores = globalThis[REGISTRY_KEY];
    if (!stores) {
        throw new Error(
            "[OrqaStudio SDK] Stores not initialized. " +
            "The host app must call initializeStores() before plugins can access stores."
        );
    }
    return stores;
}
