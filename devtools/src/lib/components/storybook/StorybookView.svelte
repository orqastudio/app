<!-- Storybook integration view. Renders the local Storybook dev server in an iframe.
     Polls the Storybook URL every 5 seconds to detect whether the server is running.
     Shows a startup instructions screen when the server is not reachable. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";

	const STORYBOOK_URL = "http://localhost:6006";
	const POLL_INTERVAL_MS = 5000;

	// Whether the Storybook server is currently reachable.
	let isRunning = $state(false);

	// Whether the initial check has completed (avoids a flash of the "not running"
	// message on first load before the first poll finishes).
	let checked = $state(false);

	let pollTimer: ReturnType<typeof setInterval> | null = null;

	// Issues a HEAD request to the Storybook URL. HEAD is used instead of GET to
	// avoid loading the full page just to check server availability.
	async function checkStorybook(): Promise<void> {
		try {
			const response = await fetch(STORYBOOK_URL, {
				method: "HEAD",
				// No-cors would hide the status code; use cors and rely on the catch
				// to detect an unreachable host.
				mode: "no-cors",
				cache: "no-store",
				signal: AbortSignal.timeout(3000),
			});
			// In no-cors mode the browser returns an opaque response with status 0
			// when the request succeeds (the server is up). A network error throws.
			isRunning = true;
		} catch {
			isRunning = false;
		} finally {
			checked = true;
		}
	}

	onMount(() => {
		// Run an immediate check so we don't wait 5 s on first render.
		checkStorybook();
		pollTimer = setInterval(checkStorybook, POLL_INTERVAL_MS);
	});

	onDestroy(() => {
		if (pollTimer !== null) {
			clearInterval(pollTimer);
		}
	});
</script>

<!-- Fill the full height of the tab content area. -->
<div class="relative flex h-full w-full flex-col">
	<!-- Connection status indicator: positioned top-right over the iframe or the
	     fallback message so it is always visible. -->
	<div class="absolute right-3 top-3 z-10 flex items-center gap-1.5 rounded-full bg-surface-raised px-2.5 py-1 text-xs shadow-sm">
		<span
			class="size-2 rounded-full {isRunning ? 'bg-green-500' : checked ? 'bg-red-500' : 'bg-yellow-500'}"
		></span>
		<span class="text-content-muted">
			{#if !checked}
				Checking…
			{:else if isRunning}
				Storybook connected
			{:else}
				Storybook not running
			{/if}
		</span>
	</div>

	{#if isRunning}
		<!-- Storybook iframe. sandbox allows scripts and same-origin access so that
		     Storybook's own navigation can function normally inside the frame. -->
		<iframe
			src={STORYBOOK_URL}
			title="Storybook"
			class="h-full w-full flex-1 border-0"
			sandbox="allow-scripts allow-same-origin allow-forms allow-modals allow-popups"
		></iframe>
	{:else if checked}
		<!-- Fallback: shown when the poll confirms Storybook is not reachable. -->
		<div class="flex h-full flex-col items-center justify-center gap-4 p-8 text-center">
			<div class="flex flex-col items-center gap-2">
				<span class="text-content-base text-base font-medium">Storybook is not running</span>
				<span class="text-content-muted text-sm">
					Start the Storybook dev server to use this view.
				</span>
			</div>
			<div class="bg-surface-raised rounded-md px-4 py-3 font-mono text-sm text-content-base">
				<span class="text-content-muted select-none">$ </span>cd libs/svelte-components &amp;&amp; npm run storybook
			</div>
			<span class="text-content-muted text-xs">
				Checking every 5 s — the view will load automatically once the server is up.
			</span>
		</div>
	{/if}
</div>
