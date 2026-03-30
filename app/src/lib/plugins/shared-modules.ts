/**
 * Expose shared modules on window.__orqa for plugin bundles.
 *
 * Plugin views are pre-bundled with @orqastudio/sdk and
 * @orqastudio/svelte-components marked as externals. At runtime,
 * their imports resolve from this global, giving plugins access to
 * the same store instances and component library as the app.
 *
 * Call exposeSharedModules() once at app startup, before any
 * plugin bundles are loaded.
 */

import * as sdk from "@orqastudio/sdk";
import * as svelteComponents from "@orqastudio/svelte-components/pure";
import * as svelteComponentsConnected from "@orqastudio/svelte-components/connected";
import * as graphVisualiser from "@orqastudio/graph-visualiser";
import * as svelte from "svelte";

declare global {
	interface Window {
		__orqa: {
			sdk: typeof sdk;
			components: typeof svelteComponents;
			componentsConnected: typeof svelteComponentsConnected;
			graphVisualiser: typeof graphVisualiser;
			svelte: typeof svelte;
		};
	}
}

export function exposeSharedModules(): void {
	window.__orqa = {
		sdk,
		components: svelteComponents,
		componentsConnected: svelteComponentsConnected,
		graphVisualiser,
		svelte,
	};
}
