<script lang="ts">
	import "../app.css";
	import "svelte-highlight/styles/github-dark-dimmed.css";
	import { setContext } from "svelte";
	import { TooltipProvider } from "@orqastudio/svelte-components/pure";
	import { ToastContainer } from "@orqastudio/svelte-components/connected";
	import { initializeStores, injectNavigation, logger } from "@orqastudio/sdk";
	import { pushState, replaceState } from "$app/navigation";
	import { initializeGraphViz } from "$lib/graph-viz.svelte";
	import { registerInstalledPlugins } from "$lib/plugins/loader";
	import { exposeSharedModules } from "$lib/plugins/shared-modules";

	const log = logger("lifecycle");
	log.info("app boot", { timestamp: Date.now() });

	// Expose shared modules on window.__orqa for plugin bundles to reference
	exposeSharedModules();

	// Inject SvelteKit navigation into SDK router (SDK can't import $app/navigation directly)
	injectNavigation(pushState, replaceState);

	// Create all SDK store instances — must happen before any component accesses getStores().
	const stores = initializeStores();
	initializeGraphViz();

	// Discover and register all installed plugins from project.json / plugins/ directory.
	// Store the promise so AppLayout can await graph init until schemas are available.
	const pluginsReady = registerInstalledPlugins(stores.pluginRegistry);
	setContext("pluginsReady", pluginsReady);

	const { navigationStore } = stores;

	// Initialize hash-based routing — restores view state from URL and enables back/forward
	navigationStore.initRouter();

	let { children } = $props();

	function handleKeydown(event: KeyboardEvent) {
		if (event.ctrlKey || event.metaKey) {
			switch (event.key) {
				case "b":
					event.preventDefault();
					navigationStore.toggleNavPanel();
					break;
			}
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<TooltipProvider>
	{@render children()}
</TooltipProvider>

<ToastContainer />
