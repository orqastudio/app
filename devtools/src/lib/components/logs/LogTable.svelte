<!-- Virtualised log event table. Uses CSS-based virtualisation: only the rows
     visible in the viewport (plus a small overscan) are rendered. Each row is
     absolutely positioned inside a spacer div whose height equals
     totalRows * ROW_HEIGHT, so the scrollbar reflects the full dataset without
     rendering every row. Expanded rows use a fixed EXPANDED_HEIGHT estimate.

     Auto-scroll is active by default and is temporarily suspended when the user
     scrolls away from the bottom. The scroll-lock toggle button restores it. -->
<script lang="ts">
	import { onMount, onDestroy, tick } from "svelte";
	import {
		events,
		filteredEvents,
		connectionStatus,
		totalReceived,
		scrollLock,
		startLogStream,
		clearEvents,
		loadHistory,
		historyLoading,
		historyExhausted,
	} from "../../stores/log-store.svelte.js";
	import LogRow from "./LogRow.svelte";
	import LogExport from "./LogExport.svelte";

	// Height of a collapsed row in pixels. Must match the inline style in LogRow.
	const ROW_HEIGHT = 24;

	// Estimated height of an expanded row (collapsed + metadata panel). Used to
	// keep scroll position stable when a row is expanded; not pixel-perfect.
	const EXPANDED_HEIGHT = 160;

	// Number of extra rows to render above and below the visible window. Reduces
	// the chance of a blank flash during fast scrolling.
	const OVERSCAN = 10;

	// Column header labels and their pixel widths, matching LogRow column widths.
	const COLUMNS: Array<{ label: string; width: number | null }> = [
		{ label: "Time", width: 90 },
		{ label: "Level", width: 54 }, // badge (42) + mr-2 (8) + padding
		{ label: "Source", width: 80 },
		{ label: "Category", width: 120 },
		{ label: "Message", width: null }, // flex-1, fills rest
	];

	// Reference to the scrollable viewport div.
	let viewport: HTMLDivElement | null = $state(null);

	// Current scrollTop of the viewport.
	let scrollTop = $state(0);

	// Height of the viewport in pixels (updates on resize).
	let viewportHeight = $state(0);

	// Track which row indices are expanded so height accounting is correct.
	// Key: event id (stable across buffer resets within a session).
	const expandedRows = $state(new Set<number>());

	// ResizeObserver to track viewport height changes.
	let resizeObserver: ResizeObserver | null = null;

	// Cleanup function returned by startLogStream.
	let stopLogStream: (() => void) | null = null;

	// Compute the total scrollable height based on the filtered event list.
	// Expanded rows are taller; collapsed rows are ROW_HEIGHT each.
	const totalHeight = $derived(
		filteredEvents.reduce((acc, ev) => {
			return acc + (expandedRows.has(ev.id) ? EXPANDED_HEIGHT : ROW_HEIGHT);
		}, 0),
	);

	// Compute the start/end indices of the visible slice by iterating through
	// cumulative heights. This handles the variable-height case correctly.
	const visibleRange = $derived.by(() => {
		let cumulative = 0;
		let start = 0;
		let end = filteredEvents.length;
		let foundStart = false;

		for (let i = 0; i < filteredEvents.length; i++) {
			const rowH = expandedRows.has(filteredEvents[i].id) ? EXPANDED_HEIGHT : ROW_HEIGHT;
			if (!foundStart && cumulative + rowH > scrollTop - OVERSCAN * ROW_HEIGHT) {
				start = Math.max(0, i - OVERSCAN);
				foundStart = true;
			}
			if (cumulative > scrollTop + viewportHeight + OVERSCAN * ROW_HEIGHT) {
				end = Math.min(filteredEvents.length, i + OVERSCAN);
				break;
			}
			cumulative += rowH;
		}

		return { start, end };
	});

	// Pre-compute the absolute top offset for each visible row so LogRow gets
	// the correct transform without iterating events inside the template.
	const rowOffsets = $derived.by(() => {
		const offsets: number[] = [];
		let cumulative = 0;
		for (let i = 0; i < filteredEvents.length; i++) {
			offsets.push(cumulative);
			cumulative += expandedRows.has(filteredEvents[i].id) ? EXPANDED_HEIGHT : ROW_HEIGHT;
		}
		return offsets;
	});

	// Slice of filtered events to render (visible + overscan).
	const visibleEvents = $derived(filteredEvents.slice(visibleRange.start, visibleRange.end));

	// Handle scroll events from the viewport. Updates scrollTop and, if the user
	// scrolls up away from the bottom, disables scroll-lock.
	function handleScroll(): void {
		if (!viewport) return;
		scrollTop = viewport.scrollTop;
		const atBottom =
			viewport.scrollTop + viewport.clientHeight >= viewport.scrollHeight - ROW_HEIGHT;
		if (!atBottom && scrollLock.enabled) {
			scrollLock.enabled = false;
		}
	}

	// Scroll the viewport to the bottom. Called when new events arrive and
	// scroll-lock is enabled, and when the user clicks the scroll-lock button
	// to re-enable it.
	async function scrollToBottom(): Promise<void> {
		await tick();
		if (viewport) {
			viewport.scrollTop = viewport.scrollHeight;
			scrollTop = viewport.scrollTop;
		}
	}

	// Re-enable scroll-lock and immediately jump to the bottom.
	function enableScrollLock(): void {
		scrollLock.enabled = true;
		scrollToBottom();
	}

	// When filtered events change and scroll-lock is on, scroll to bottom.
	// Reads filteredEvents.length as the reactive trigger so it fires on each
	// new event that passes the active filters.
	$effect(() => {
		if (filteredEvents.length > 0 && scrollLock.enabled) {
			scrollToBottom();
		}
	});

	onMount(async () => {
		// Wire up the ResizeObserver to track viewport height changes.
		if (viewport) {
			viewportHeight = viewport.clientHeight;
			resizeObserver = new ResizeObserver((entries) => {
				for (const entry of entries) {
					viewportHeight = entry.contentRect.height;
				}
			});
			resizeObserver.observe(viewport);
		}

		// Start consuming Tauri log events.
		stopLogStream = await startLogStream();
	});

	onDestroy(() => {
		if (resizeObserver) resizeObserver.disconnect();
		if (stopLogStream) stopLogStream();
	});
</script>

<!-- Outer container fills the height of the Logs tab content area. -->
<div class="flex h-full flex-col overflow-hidden">
	<!-- Column header bar: fixed, never scrolls. -->
	<div
		class="border-b border-border bg-surface-base flex shrink-0 items-center px-2 text-[10px] font-medium uppercase tracking-wider text-content-muted"
		style="height: 24px;"
		role="row"
	>
		{#each COLUMNS as col (col.label)}
			{#if col.width !== null}
				<span class="shrink-0" style="width: {col.width}px;">{col.label}</span>
			{:else}
				<span class="min-w-0 flex-1">{col.label}</span>
			{/if}
		{/each}

		<!-- Toolbar: scroll-lock toggle, export, and clear buttons pinned to the right. -->
		<div class="ml-auto flex shrink-0 items-center gap-1">
			{#if !scrollLock.enabled}
				<button
					class="rounded bg-blue-500/20 px-1.5 py-0.5 text-[10px] text-blue-400 transition-colors hover:bg-blue-500/30"
					onclick={enableScrollLock}
				>
					Follow
				</button>
			{/if}
			<LogExport />
			<button
				class="rounded px-1.5 py-0.5 text-[10px] text-content-muted transition-colors hover:bg-surface-raised hover:text-content-base"
				onclick={clearEvents}
			>
				Clear
			</button>
		</div>
	</div>

	<!-- Load earlier button: prepends history events before the oldest visible
	     event. Shown above the virtualised scroll area so it is reachable without
	     scrolling to the top (which would disable scroll-lock). Hidden when all
	     available history has been loaded. -->
	{#if !historyExhausted.value}
		<div class="border-b border-border bg-surface-base flex shrink-0 items-center justify-center px-2" style="height: 24px;">
			<button
				class="rounded px-2 py-0.5 text-[10px] text-content-muted transition-colors hover:bg-surface-raised hover:text-content-base disabled:cursor-not-allowed disabled:opacity-40"
				disabled={historyLoading.value}
				onclick={loadHistory}
			>
				{historyLoading.value ? "Loading…" : "Load earlier"}
			</button>
		</div>
	{/if}

	<!-- Scrollable viewport: the only element that scrolls. Uses role="table" so
	     screen readers understand the virtualised row structure. -->
	<div
		bind:this={viewport}
		class="relative flex-1 overflow-x-hidden overflow-y-auto"
		role="table"
		aria-label="Log events"
		aria-rowcount={filteredEvents.length}
		onscroll={handleScroll}
	>
		<!-- Spacer div: its height equals the total height of all filtered rows so
		     the scrollbar reflects the full filtered dataset. Rows are absolutely
		     positioned inside it via their pre-computed offsets. -->
		<div class="relative w-full" style="height: {totalHeight}px;">
			{#each visibleEvents as ev (ev.id)}
				<LogRow
					event={ev}
					style="top: {rowOffsets[filteredEvents.indexOf(ev)]}px;"
				/>
			{/each}
		</div>

		<!-- Empty state: shown when there are no matching events. -->
		{#if filteredEvents.length === 0}
			<div class="flex h-full items-center justify-center text-sm text-content-muted">
				{#if events.length === 0}
					{#if connectionStatus.value === "connecting"}
						Waiting for events…
					{:else if connectionStatus.value === "disconnected"}
						Daemon disconnected — no events
					{:else}
						No events yet
					{/if}
				{:else}
					No events match the active filters
				{/if}
			</div>
		{/if}
	</div>

	<!-- Status strip: event count and auto-scroll indicator. -->
	<div
		class="border-t border-border bg-surface-base flex shrink-0 items-center gap-3 px-2 text-[10px] text-content-muted"
		style="height: 20px;"
	>
		{#if filteredEvents.length !== events.length}
			<span>{filteredEvents.length} of {totalReceived.value} events</span>
		{:else}
			<span>{totalReceived.value} events received</span>
		{/if}
		{#if scrollLock.enabled}
			<span class="text-blue-400">Auto-scroll on</span>
		{/if}
	</div>
</div>
