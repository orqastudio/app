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
		Button,
		Caption,
		HStack,
		SectionHeader,
		SectionFooter,
		Stack,
	} from "@orqastudio/svelte-components/pure";
	import { assertNever } from "@orqastudio/types";
	import {
		events,
		filteredEvents as getFilteredEvents,
		connectionStatus,
		totalReceived,
		scrollLock,
		startLogStream,
		clearEvents,
		loadHistory,
		historyLoading,
		historyExhausted,
		historicalMode,
		historicalTotal,
		historicalExhausted,
		loadMoreHistoricalEvents,
		exitHistoricalMode,
	} from "../../stores/log-store.svelte.js";
	import {
		activeSessionId,
		sessionDisplayLabel,
		sessions,
		switchToCurrentSession,
	} from "../../stores/session-store.svelte.js";
	import { openDrawer } from "../../stores/drawer-store.svelte.js";
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	const filteredEvents = $derived(getFilteredEvents());
	import LogRow from "./LogRow.svelte";
	import LogExport from "./LogExport.svelte";

	/**
	 * Handle a row click from the virtualised log table. Opens the EventDrawer
	 * for the clicked event, passing the current filtered event list as the
	 * navigation context so next/prev steps through visible events.
	 * @param event - The log event whose row was clicked.
	 */
	function handleRowClick(event: LogEvent): void {
		openDrawer(event, filteredEvents);
	}

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
	// Legitimate exception: raw div with bind:this required for virtualisation scroll binding.
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

	// Display label for the historical session currently being viewed.
	const historicalSessionLabel = $derived(
		sessions.find((s) => s.id === activeSessionId.value)
			? sessionDisplayLabel(sessions.find((s) => s.id === activeSessionId.value)!)
			: "historical session",
	);

	/**
	 * Resolve the empty-state message for the current connection status.
	 * @param status - The current connection status value.
	 * @returns A human-readable description of why no events are visible.
	 */
	function connectionEmptyMessage(status: typeof connectionStatus.value): string {
		switch (status) {
			case "connecting":
				return "Waiting for events\u2026";
			case "disconnected":
				return "Daemon disconnected \u2014 no events";
			case "connected":
				return "No events yet";
			default:
				return assertNever(status);
		}
	}

	/** Load the next page of events for the current historical session. */
	async function handleLoadMore(): Promise<void> {
		if (activeSessionId.value) {
			await loadMoreHistoricalEvents(activeSessionId.value);
		}
	}

	/** Return to the live feed, re-activating the ring buffer stream. */
	async function handleReturnToLive(): Promise<void> {
		await switchToCurrentSession();
		await exitHistoricalMode();
	}

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

	/** Handle viewport scroll: update scrollTop and disable scroll-lock if user scrolled up. */
	function handleScroll(): void {
		if (!viewport) return;
		scrollTop = viewport.scrollTop;
		const atBottom =
			viewport.scrollTop + viewport.clientHeight >= viewport.scrollHeight - ROW_HEIGHT;
		if (!atBottom && scrollLock.enabled) {
			scrollLock.enabled = false;
		}
	}

	/** Scroll the viewport to the bottom after the current tick. */
	async function scrollToBottom(): Promise<void> {
		await tick();
		if (viewport) {
			viewport.scrollTop = viewport.scrollHeight;
			scrollTop = viewport.scrollTop;
		}
	}

	/** Re-enable scroll-lock and immediately jump to the bottom of the log. */
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

<!-- Outer container: Stack fills the height of the Logs tab content area. -->
<Stack gap={0} height="full">
	<!-- Column header bar: SectionHeader compact provides px-2 py-1 border-b layout. -->
	<SectionHeader variant="compact" role="row">
		{#each COLUMNS as col (col.label)}
			{#if col.width !== null}
				<div class="log-table__col-label log-table__col-label--fixed" style="width: {col.width}px;">
					{col.label}
				</div>
			{:else}
				<div class="log-table__col-label log-table__col-label--flex">{col.label}</div>
			{/if}
		{/each}

		<!-- Toolbar: scroll-lock toggle, export, and clear buttons pinned to the right.
		     Follow button is hidden when viewing a historical session because auto-scroll
		     is meaningless for static data. -->
		<div class="log-table__toolbar">
			{#if !historicalMode.value && !scrollLock.enabled}
				<!-- Wrapper span with display:contents is invisible to flex layout;
				     scoped class provides the hook for :global() CSS overrides. -->
				<span class="log-table__follow-wrap" style="display: contents;">
					<Button variant="ghost" size="icon-sm" onclick={enableScrollLock}>Follow</Button>
				</span>
			{/if}
			<LogExport />
			<span class="log-table__clear-wrap" style="display: contents;">
				<Button variant="ghost" size="icon-sm" onclick={clearEvents}>Clear</Button>
			</span>
		</div>
	</SectionHeader>

	<!-- Historical session banner: shown above the viewport when viewing a past
	     session. Scoped div provides the height/bg; HStack wraps the inner content. -->
	{#if historicalMode.value}
		<div class="log-table__historical-banner">
			<HStack justify="between" gap={2} full>
				<div class="log-table__historical-label">
					<Caption truncate>
						Viewing historical session — {historicalSessionLabel}
					</Caption>
				</div>
				<span class="log-table__return-wrap" style="display: contents;">
					<Button variant="ghost" size="icon-sm" onclick={handleReturnToLive}>
						Return to live
					</Button>
				</span>
			</HStack>
		</div>
	{/if}

	<!-- Load earlier button: prepends history events before the oldest visible
	     event. Shown above the virtualised scroll area so it is reachable without
	     scrolling to the top (which would disable scroll-lock). Hidden when all
	     available history has been loaded. Hidden during historical mode (load-more
	     is shown at the bottom instead). -->
	{#if !historyExhausted.value && !historicalMode.value}
		<!-- SectionHeader compact provides border-b and centered layout. -->
		<SectionHeader variant="compact">
			<span class="log-table__load-wrap" style="display: contents;">
				<Button
					variant="ghost"
					size="icon-sm"
					disabled={historyLoading.value}
					onclick={loadHistory}
				>
					{historyLoading.value ? "Loading…" : "Load earlier"}
				</Button>
			</span>
		</SectionHeader>
	{/if}

	<!-- Scrollable viewport: the only element that scrolls.
	     LEGITIMATE EXCEPTION: raw div with bind:this required for virtualisation.
	     role="table" communicates the virtualised row structure to screen readers. -->
	<div
		bind:this={viewport}
		class="log-table__viewport"
		role="table"
		aria-label="Log events"
		aria-rowcount={filteredEvents.length}
		onscroll={handleScroll}
	>
		<!-- Spacer div: its height equals the total height of all filtered rows so
		     the scrollbar reflects the full filtered dataset. Rows are absolutely
		     positioned inside it via their pre-computed offsets. -->
		<div class="log-table__spacer" style="height: {totalHeight}px;">
			{#each visibleEvents as ev (ev.id)}
				<LogRow
					event={ev}
					style="top: {rowOffsets[filteredEvents.indexOf(ev)]}px;"
					ondraweropen={handleRowClick}
				/>
			{/each}
		</div>

		<!-- Empty state: centered caption shown when there are no matching events. -->
		{#if filteredEvents.length === 0}
			<div class="log-table__empty">
				<Caption>
					{#if events.length === 0}
						{connectionEmptyMessage(connectionStatus.value)}
					{:else}
						No events match the active filters
					{/if}
				</Caption>
			</div>
		{/if}
	</div>

	<!-- Load more button: shown below the viewport when viewing a historical session
	     and more pages are available. Appends the next page of events to the buffer. -->
	{#if historicalMode.value && !historicalExhausted.value}
		<!-- SectionFooter compact provides border-t and centered layout. -->
		<SectionFooter variant="compact">
			<span class="log-table__load-wrap" style="display: contents;">
				<Button
					variant="ghost"
					size="icon-sm"
					disabled={historyLoading.value}
					onclick={handleLoadMore}
				>
					{historyLoading.value ? "Loading…" : "Load more"}
				</Button>
			</span>
		</SectionFooter>
	{/if}

	<!-- Status strip: scoped div provides height/border/bg/padding; HStack provides inner layout. -->
	<div class="log-table__status">
		<HStack gap={3} full>
			{#if historicalMode.value}
				<Caption variant="caption-tabular"
					>{events.length} of {historicalTotal.value} events loaded</Caption
				>
			{:else if filteredEvents.length !== events.length}
				<Caption variant="caption-tabular"
					>{filteredEvents.length} of {totalReceived.value} events</Caption
				>
			{:else}
				<Caption variant="caption-tabular">{totalReceived.value} events received</Caption>
			{/if}
			{#if !historicalMode.value && scrollLock.enabled}
				<div class="log-table__autoscroll-label"><Caption>Auto-scroll on</Caption></div>
			{/if}
		</HStack>
	</div>
</Stack>

<style>
	/* Column header labels. */
	.log-table__col-label {
		font-size: 10px;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-content-muted);
		background-color: var(--color-surface-base);
	}

	/* Fixed-width column label. */
	.log-table__col-label--fixed {
		flex-shrink: 0;
	}

	/* Flex-fill column label (Message). */
	.log-table__col-label--flex {
		min-width: 0;
		flex: 1;
	}

	/* Toolbar: right-aligned group of compact action buttons. */
	.log-table__toolbar {
		margin-left: auto;
		display: flex;
		flex-shrink: 0;
		align-items: center;
		gap: var(--spacing-1);
	}

	/* Compact 20px height override for toolbar and load buttons. Wrappers use
	   display:contents so they are invisible to flex layout; the :global() selectors
	   target the Button's rendered button element inside each wrapper span. */
	:global(.log-table__follow-wrap button),
	:global(.log-table__clear-wrap button),
	:global(.log-table__load-wrap button),
	:global(.log-table__return-wrap button) {
		height: 20px !important;
		width: auto !important;
		padding: 0 var(--spacing-1-5) !important;
		font-size: 10px !important;
	}

	/* Follow button: uses primary tint to indicate active auto-scroll state. */
	:global(.log-table__follow-wrap button) {
		background-color: color-mix(in srgb, var(--color-primary) 20%, transparent) !important;
		color: var(--color-primary) !important;
	}

	:global(.log-table__follow-wrap button:hover) {
		background-color: color-mix(in srgb, var(--color-primary) 30%, transparent) !important;
	}

	/* Historical session banner: shown between header and load-earlier bar. */
	.log-table__historical-banner {
		background-color: color-mix(in srgb, var(--color-primary) 8%, var(--color-background));
		border-bottom: 1px solid var(--color-border);
		height: 24px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		padding: 0 var(--spacing-2);
	}

	/* Historical label wrapper: flex-1 so it truncates before the return button. */
	.log-table__historical-label {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		color: var(--color-primary);
		font-style: italic;
	}

	/* Return to live button: primary text to pair with the banner.
	   Height/padding already covered by the shared rule above. */
	:global(.log-table__return-wrap button) {
		color: var(--color-primary) !important;
		flex-shrink: 0;
	}

	/* Scrollable viewport area. */
	.log-table__viewport {
		position: relative;
		flex: 1;
		overflow-x: hidden;
		overflow-y: auto;
	}

	/* Spacer that carries the full virtual height. */
	.log-table__spacer {
		position: relative;
		width: 100%;
	}

	/* Empty-state message centred in the viewport. */
	.log-table__empty {
		display: flex;
		height: 100%;
		align-items: center;
		justify-content: center;
	}

	/* Status strip: compact bottom bar. */
	.log-table__status {
		border-top: 1px solid var(--color-border);
		background-color: var(--color-surface-base);
		height: 20px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		padding: 0 var(--spacing-2);
	}

	/* Auto-scroll active indicator: primary color to pair with the follow button. */
	.log-table__autoscroll-label {
		color: var(--color-primary);
	}
</style>
