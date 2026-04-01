<!-- Process diagnostics view for OrqaDev. Renders a grid of ProcessCard components,
     one for each managed process (Daemon, MCP, LSP, Search, Sidecar). Status is
     polled from the daemon health endpoint every 2 seconds via the
     devtools_process_status IPC command. Clicking a card fires a log-source filter
     event so users can navigate directly to that process's log entries. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import ProcessCard from "./ProcessCard.svelte";
	import type { ProcessInfo } from "./ProcessCard.svelte";

	const POLL_INTERVAL_MS = 2000;

	// Current list of process info objects. Starts empty until the first poll
	// completes so we can distinguish "loading" from "no processes".
	let processes = $state<ProcessInfo[]>([]);

	// Whether the first poll has completed. Used to show a loading state briefly
	// instead of an empty grid on first render.
	let initialized = $state(false);

	// Source key of the currently selected card. Null when no card is selected.
	let selectedSource = $state<string | null>(null);

	// Whether the last poll attempt failed (daemon unreachable).
	let pollError = $state(false);

	let pollTimer: ReturnType<typeof setInterval> | null = null;

	// Fetch process status via IPC and update the reactive state.
	async function fetchStatus(): Promise<void> {
		try {
			const result = await invoke<ProcessInfo[]>("devtools_process_status");
			processes = result;
			pollError = false;
		} catch (err) {
			// Leave the existing card list intact so the UI shows stale data rather
			// than flashing empty — just mark that the poll failed.
			pollError = true;
		} finally {
			initialized = true;
		}
	}

	// Handle a card selection: toggle off when clicking the already-selected card.
	function handleSelect(source: string): void {
		selectedSource = selectedSource === source ? null : source;
	}

	onMount(() => {
		// Immediate fetch so the grid is populated without waiting for the first
		// poll interval.
		fetchStatus();
		pollTimer = setInterval(fetchStatus, POLL_INTERVAL_MS);
	});

	onDestroy(() => {
		if (pollTimer !== null) {
			clearInterval(pollTimer);
		}
	});
</script>

<!-- Outer container fills the tab content area. Scrollable vertically so the
     grid does not get clipped if more process cards are added later. -->
<div class="flex h-full flex-col overflow-auto p-4 gap-4">
	<!-- Header row: title + poll error indicator -->
	<div class="flex items-center justify-between">
		<h2 class="text-content-base text-sm font-semibold">Processes</h2>
		{#if pollError}
			<span class="flex items-center gap-1.5 text-xs text-yellow-400">
				<span class="size-2 rounded-full bg-yellow-400"></span>
				Daemon unreachable — status may be stale
			</span>
		{:else if initialized}
			<span class="flex items-center gap-1.5 text-xs text-content-muted">
				<span class="size-2 rounded-full bg-green-500"></span>
				Polling every 2s
			</span>
		{/if}
	</div>

	{#if !initialized}
		<!-- Loading state: shown only on first render before the initial poll returns. -->
		<div class="text-content-muted flex flex-1 items-center justify-center text-sm">
			Loading process status…
		</div>
	{:else}
		<!-- Process grid: 2 columns on small screens, 3 on medium, up to 5 on large.
		     Uses auto-fill so the layout adapts without explicit breakpoints. -->
		<div class="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-5">
			{#each processes as process (process.source)}
				<ProcessCard
					{process}
					selected={selectedSource === process.source}
					onselect={handleSelect}
				/>
			{/each}
		</div>

		<!-- Log filter hint: shown when a card is selected, telling the user
		     where to find the filtered log entries. -->
		{#if selectedSource !== null}
			<div class="bg-surface-raised border-border rounded-md border px-3 py-2 text-xs text-content-muted">
				Showing logs for source <span class="font-mono text-content-base">{selectedSource}</span>.
				Click the card again to clear the filter. Navigate to the
				<span class="text-content-base font-medium">Logs</span> tab to see filtered entries.
			</div>
		{/if}
	{/if}
</div>
