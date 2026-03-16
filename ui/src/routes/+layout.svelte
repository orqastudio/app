<script lang="ts">
	import "../app.css";
	import "svelte-highlight/styles/github-dark-dimmed.css";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import ToastContainer from "$lib/components/shared/ToastContainer.svelte";
	import { initializeStores, getStores } from "@orqastudio/sdk";
	import { initializeGraphViz } from "$lib/graph-viz.svelte";

	// Create all SDK store instances — must happen before any component accesses getStores().
	initializeStores();
	initializeGraphViz();

	const { navigationStore } = getStores();

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

<Tooltip.Provider>
	{@render children()}
</Tooltip.Provider>

<ToastContainer />
