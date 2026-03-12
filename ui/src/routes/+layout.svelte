<script lang="ts">
	import "../app.css";
	import "svelte-highlight/styles/github-dark-dimmed.css";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { navigationStore } from "$lib/stores/navigation.svelte";

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
