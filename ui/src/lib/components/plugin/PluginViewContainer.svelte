<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { convertFileSrc } from "@tauri-apps/api/core";

	let { pluginName, viewKey }: { pluginName: string; viewKey: string } = $props();

	let container: HTMLDivElement;
	let error: string | null = $state(null);
	let loading = $state(true);
	let cleanup: (() => void) | null = null;

	onMount(async () => {
		try {
			// Get the plugin's install path from the backend
			const pluginPath = await invoke<string>("plugin_get_path", { name: pluginName });

			// Load the plugin's pre-bundled view module
			// Plugins build to dist/views/{viewKey}.js
			const bundlePath = `${pluginPath}/dist/views/${viewKey}.js`;
			const bundleUrl = convertFileSrc(bundlePath);

			const module = await import(/* @vite-ignore */ bundleUrl);

			if (!module.default && !module.mount) {
				error = `Plugin "${pluginName}" view "${viewKey}" does not export a default component or mount function`;
				loading = false;
				return;
			}

			// Mount the plugin view into our container
			if (typeof module.mount === "function") {
				// Plugin provides a mount function (preferred)
				cleanup = module.mount(container);
			} else if (module.default) {
				// Plugin exports a Svelte 5 component — use mount()
				const { mount: svelteMount, unmount: svelteUnmount } = await import("svelte");
				const instance = svelteMount(module.default, { target: container });
				cleanup = () => svelteUnmount(instance);
			}

			loading = false;
		} catch (err) {
			error = `Failed to load plugin view: ${err instanceof Error ? err.message : String(err)}`;
			loading = false;
		}
	});

	onDestroy(() => {
		if (cleanup) cleanup();
	});
</script>

<div class="plugin-view-container h-full w-full overflow-auto">
	{#if loading}
		<div class="flex h-full items-center justify-center text-muted-foreground">
			Loading plugin view...
		</div>
	{:else if error}
		<div class="flex h-full items-center justify-center">
			<div class="max-w-md rounded-lg border border-destructive/50 bg-destructive/10 p-4">
				<p class="text-sm text-destructive">{error}</p>
			</div>
		</div>
	{:else}
		<div bind:this={container} class="h-full w-full"></div>
	{/if}
</div>
