<!-- Storybook integration view. Renders the local Storybook dev server in an iframe.
     Polls the Storybook URL every 5 seconds to detect whether the server is running.
     Shows an empty state when the server is not reachable. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { ConnectionIndicator, EmptyState } from "@orqastudio/svelte-components/pure";
	import type { ConnectionState } from "@orqastudio/svelte-components/pure";

	import { DEFAULT_PORT_BASE, PORT_OFFSETS } from "@orqastudio/constants";
	const STORYBOOK_URL = `http://localhost:${DEFAULT_PORT_BASE + PORT_OFFSETS.storybook}`;
	const POLL_INTERVAL_MS = 5000;

	// Whether the Storybook server is currently reachable.
	let isRunning = $state(false);

	// Whether the initial check has completed (avoids a flash of the "not running"
	// message on first load before the first poll finishes).
	let checked = $state(false);

	let pollTimer: ReturnType<typeof setInterval> | null = null;

	// Derives a ConnectionState value from polling results for use with ConnectionIndicator.
	const connectionState = $derived<ConnectionState>(
		!checked ? "waiting" : isRunning ? "connected" : "disconnected",
	);

	// Derives the human-readable label shown beside the connection indicator dot.
	const connectionLabel = $derived(
		!checked ? "Checking…" : isRunning ? "Storybook connected" : "Storybook not running",
	);

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

<!-- Fill the full height of the tab content area. Position the connection
     indicator absolutely so it floats over both the iframe and the empty state. -->
<div class="relative h-full w-full">
	<!-- Connection status indicator: positioned top-right over the iframe or the
	     empty state so it is always visible regardless of which branch renders. -->
	<div class="absolute right-3 top-3 z-10">
		<ConnectionIndicator state={connectionState} label={connectionLabel} />
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
		<!-- Empty state: shown when the poll confirms Storybook is not reachable. -->
		<EmptyState
			icon="book-open"
			title="Storybook is not running"
			description="Start the dev server to use this view: cd libs/svelte-components && npm run storybook — the view will load automatically once the server is up."
		/>
	{/if}
</div>
