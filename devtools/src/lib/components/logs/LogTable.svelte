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
		Center,
		HStack,
		LogColLabel,
		LogSpacer,
		LogViewport,
		SectionFooter,
		SectionHeader,
		Stack,
		Box,
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

	// Reference to the scrollable viewport div — bound via LogViewport's ref prop.
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
	<!-- Column header bar: SectionHeader compact provides px-2 py-1 border-b layout.
	     Col labels in an HStack with gap-0 so they align with LogRow columns.
	     Toolbar (scroll-lock, export, clear) in a zero-gap HStack pushed to the end. -->
	<SectionHeader variant="compact" role="row">
		{#snippet start()}
			<HStack gap={0}>
				{#each COLUMNS as col (col.label)}
					<LogColLabel width={col.width ?? undefined} fill={col.width === null}>
						{col.label}
					</LogColLabel>
				{/each}
			</HStack>
		{/snippet}
		{#snippet end()}
			<HStack gap={1}>
				{#if !historicalMode.value && !scrollLock.enabled}
					<!-- Follow button: primary-tinted to indicate auto-scroll is available. -->
					<Button variant="ghost" size="xs" onclick={enableScrollLock}>Follow</Button>
				{/if}
				<LogExport />
				<Button variant="ghost" size="xs" onclick={clearEvents}>Clear</Button>
			</HStack>
		{/snippet}
	</SectionHeader>

	<!-- Historical session banner: shown above the viewport when viewing a past session.
	     SectionHeader compact with primary-subtle background provides the tinted strip. -->
	{#if historicalMode.value}
		<SectionHeader variant="compact" background="primary-subtle">
			{#snippet start()}
				<Box flex={1} minWidth={0}>
					<Caption truncate tone="primary" italic>
						Viewing historical session — {historicalSessionLabel}
					</Caption>
				</Box>
			{/snippet}
			{#snippet end()}
				<Button variant="ghost" size="xs" onclick={handleReturnToLive}>Return to live</Button>
			{/snippet}
		</SectionHeader>
	{/if}

	<!-- Load earlier button: prepends history events before the oldest visible
	     event. Hidden when all available history has been loaded or in historical mode. -->
	{#if !historyExhausted.value && !historicalMode.value}
		<SectionHeader variant="compact">
			<Button variant="ghost" size="xs" disabled={historyLoading.value} onclick={loadHistory}>
				{historyLoading.value ? "Loading…" : "Load earlier"}
			</Button>
		</SectionHeader>
	{/if}

	<!-- Scrollable viewport: LogViewport handles the bind:this and role/aria attrs.
	     The ref binding is required for virtualisation scroll position tracking. -->
	<LogViewport
		bind:ref={viewport}
		ariaLabel="Log events"
		ariaRowCount={filteredEvents.length}
		onscroll={handleScroll}
	>
		<!-- LogSpacer: its height equals the total height of all filtered rows so
		     the scrollbar reflects the full filtered dataset. Rows are absolutely
		     positioned inside it via their pre-computed offsets. -->
		<LogSpacer height={totalHeight}>
			{#each visibleEvents as ev (ev.id)}
				<LogRow
					event={ev}
					style="top: {rowOffsets[filteredEvents.indexOf(ev)]}px;"
					ondraweropen={handleRowClick}
				/>
			{/each}
		</LogSpacer>

		<!-- Empty state: centered caption shown when there are no matching events. -->
		{#if filteredEvents.length === 0}
			<Center full>
				<Caption>
					{#if events.length === 0}
						{connectionEmptyMessage(connectionStatus.value)}
					{:else}
						No events match the active filters
					{/if}
				</Caption>
			</Center>
		{/if}
	</LogViewport>

	<!-- Load more button: shown below the viewport when viewing a historical session
	     and more pages are available. SectionFooter compact provides border-t layout. -->
	{#if historicalMode.value && !historicalExhausted.value}
		<SectionFooter variant="compact">
			<Button variant="ghost" size="xs" disabled={historyLoading.value} onclick={handleLoadMore}>
				{historyLoading.value ? "Loading…" : "Load more"}
			</Button>
		</SectionFooter>
	{/if}

	<!-- Status strip: SectionFooter compact provides border-t, bg-surface, px-2 py-1. -->
	<SectionFooter variant="compact" background="surface">
		{#snippet start()}
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
		{/snippet}
		{#snippet end()}
			{#if !historicalMode.value && scrollLock.enabled}
				<!-- Primary tone pairs with the Follow button to confirm auto-scroll is active. -->
				<Caption tone="primary">Auto-scroll on</Caption>
			{/if}
		{/snippet}
	</SectionFooter>
</Stack>
