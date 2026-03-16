/**
 * Central store registry for the OrqaStudio SDK.
 *
 * The app calls `initializeStores()` once at startup to create all store
 * instances. Plugins and components call `getStores()` to access the same
 * instances — no matter how many copies of the SDK exist in the dependency
 * tree, the registry is resolved to a single module via the app's bundler.
 *
 * This guarantees a single set of stores across the app and all plugins.
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
import { ErrorStoreImpl, initBrowserHandlers } from "./stores/errors.svelte.js";
import { NavigationStore } from "./stores/navigation.svelte.js";
import { ToastStore, createToastConvenience } from "./stores/toast.svelte.js";

import type { ArtifactGraphConfig } from "./graph/artifact-graph.svelte.js";
import type { Toast } from "./stores/toast.svelte.js";

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
// Module-level singleton — one per bundled copy of the SDK
// ---------------------------------------------------------------------------

let _stores: OrqaStores | null = null;

/**
 * Create and register all SDK store instances.
 *
 * Call this exactly once during app startup (e.g. in the root +layout.svelte).
 * Subsequent calls return the same instances without re-creating.
 *
 * @returns The full set of store instances.
 */
export function initializeStores(): OrqaStores {
    if (_stores) return _stores;

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

    _stores = {
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

    return _stores;
}

/**
 * Access the registered store instances.
 *
 * Throws if `initializeStores()` has not been called yet. Safe to call from
 * component code, store methods, and plugin modules — by the time any of
 * those execute, the app will have initialized the stores.
 */
export function getStores(): OrqaStores {
    if (!_stores) {
        throw new Error(
            "[OrqaStudio SDK] Stores not initialized. " +
            "Call initializeStores() in your app's root layout before accessing stores."
        );
    }
    return _stores;
}
