<script lang="ts">
	import "../app.css";
	import "svelte-highlight/styles/github-dark-dimmed.css";
	import { TooltipProvider } from "@orqastudio/svelte-components/pure";
	import { ToastContainer } from "@orqastudio/svelte-components/connected";
	import { initializeStores, getStores } from "@orqastudio/sdk";
	import { initializeGraphViz } from "$lib/graph-viz.svelte";
	import { registerSoftwareProjectPlugin } from "$lib/plugins/software-project";

	// Create all SDK store instances — must happen before any component accesses getStores().
	const stores = initializeStores();
	initializeGraphViz();

	// Register installed plugins (async — loads manifests from disk via IPC)
	registerSoftwareProjectPlugin(stores.pluginRegistry);

	const { navigationStore } = stores;

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
