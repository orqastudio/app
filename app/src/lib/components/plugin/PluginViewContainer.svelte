<script lang="ts">
	import { onMount, onDestroy, tick } from "svelte";
	import { readTextFile } from "@tauri-apps/plugin-fs";
	import { logger } from "@orqastudio/sdk";
	import { getPluginPath } from "$lib/services/plugin-service.js";
	import {
		Caption,
		CardRoot,
		CardContent,
		ScrollArea,
		Center,
		Text,
		Box,
	} from "@orqastudio/svelte-components/pure";

	const log = logger("plugin-view");

	let { pluginName, viewKey }: { pluginName: string; viewKey: string } = $props();

	let container = $state<HTMLDivElement | undefined>(undefined);
	let error: string | null = $state(null);
	let loading = $state(true);
	let cleanup: (() => void) | null = null;

	onMount(async () => {
		try {
			// Get the plugin's install path from the backend via the plugin service.
			const pluginPath = await getPluginPath(pluginName);

			// Load the plugin's pre-bundled IIFE view module.
			// IIFE format resolves shared dependencies via output.globals
			// (e.g. window.__orqa.svelte), so no import rewriting is needed.
			const bundlePath = `${pluginPath}/dist/views/${viewKey}.js`;
			const content = await readTextFile(bundlePath);

			// Execute the IIFE. It returns the module exports object.

			const module = new Function(content + "\nreturn OrqaPluginView;")() as Record<
				string,
				unknown
			>;

			if (!module.default && !module.mount) {
				error = `Plugin "${pluginName}" view "${viewKey}" does not export a default component or mount function`;
				loading = false;
				return;
			}

			// Set loading false so the container Box renders, then mount into it
			// on the next tick once the DOM has updated.
			loading = false;
			await tick();

			if (!container) {
				error = "Plugin mount target not available after render";
				return;
			}

			if (typeof module.mount === "function") {
				cleanup = module.mount(container);
			} else if (module.default) {
				const { mount: svelteMount, unmount: svelteUnmount } = await import("svelte");
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const instance = svelteMount(module.default as any, { target: container });
				cleanup = () => svelteUnmount(instance);
			}
		} catch (err) {
			log.error("Failed to load plugin view", { pluginName, viewKey, err });
			error = `Failed to load plugin view: ${err instanceof Error ? err.message : String(err)}`;
			loading = false;
		}
	});

	onDestroy(() => {
		if (cleanup) cleanup();
	});
</script>

<ScrollArea full>
	{#if loading}
		<Center full>
			<Caption>Loading plugin view...</Caption>
		</Center>
	{:else if error}
		<Center full>
			<CardRoot>
				<CardContent>
					<Text tone="destructive">{error}</Text>
				</CardContent>
			</CardRoot>
		</Center>
	{:else}
		<!-- Box bind:ref gives the raw HTMLDivElement, which the plugin IIFE mount function requires. -->
		<Box bind:ref={container} height="full" width="full"></Box>
	{/if}
</ScrollArea>
