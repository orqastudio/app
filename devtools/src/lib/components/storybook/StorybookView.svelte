<!-- Storybook integration view. Renders the local Storybook dev server in an iframe.
     Polls the Storybook URL every 5 seconds to detect whether the server is running.
     Shows an empty state when the server is not reachable. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { ConnectionIndicator, EmptyState, Box } from "@orqastudio/svelte-components/pure";
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
	/**
	 * Issue a HEAD request to the Storybook URL to check whether the dev server is reachable.
	 * In no-cors mode a successful request returns an opaque response; a network error throws.
	 * @returns Resolves after the reachability check completes.
	 */
	async function checkStorybook(): Promise<void> {
		try {
			await fetch(STORYBOOK_URL, {
				method: "HEAD",
				// No-cors returns an opaque response (status 0) on success; a network
				// error throws, letting us detect an unreachable host via the catch.
				mode: "no-cors",
				cache: "no-store",
				signal: AbortSignal.timeout(3000),
			});
			// If fetch did not throw, the server responded — mark it as running.
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
<Box position="relative" height="full" width="full">
	<!-- Connection status indicator: positioned top-right over the iframe or the
	     empty state so it is always visible regardless of which branch renders. -->
	<Box position="absolute" right={3} top={3} zIndex={10}>
		<ConnectionIndicator state={connectionState} label={connectionLabel} />
	</Box>

	{#if isRunning}
		<!-- Storybook iframe. sandbox allows scripts and same-origin access so that
		     Storybook's own navigation can function normally inside the frame.
		     iframe is not a prohibited element; inline style used here because the
		     fill-container dimensions are fixed and not driven by Tailwind tokens. -->
		<iframe
			src={STORYBOOK_URL}
			title="Storybook"
			style="height: 100%; width: 100%; flex: 1; border: none;"
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
</Box>
