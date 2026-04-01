<!-- Individual log table row. Renders timestamp, level badge, source, category,
     and message columns. Clicking the row toggles an expanded metadata panel
     that shows the raw metadata JSON for that event. Color-coded by level. -->
<script lang="ts">
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	let {
		event,
		style = "",
	}: {
		event: LogEvent;
		// Inline style string injected by the virtualiser for position/transform.
		style?: string;
	} = $props();

	// Whether the metadata panel is expanded for this row.
	let expanded = $state(false);

	// Format a Unix millisecond timestamp as HH:MM:SS.mmm.
	function formatTimestamp(ms: number): string {
		const d = new Date(ms);
		const hh = d.getHours().toString().padStart(2, "0");
		const mm = d.getMinutes().toString().padStart(2, "0");
		const ss = d.getSeconds().toString().padStart(2, "0");
		const mmm = d.getMilliseconds().toString().padStart(3, "0");
		return `${hh}:${mm}:${ss}.${mmm}`;
	}

	// CSS classes for the level badge background and text, keyed by level string.
	const LEVEL_BADGE: Record<string, string> = {
		Debug: "bg-surface-raised text-content-muted",
		Info: "bg-blue-500/15 text-blue-400",
		Warn: "bg-yellow-500/15 text-yellow-400",
		Error: "bg-red-500/20 text-red-400",
		Perf: "bg-indigo-500/15 text-indigo-400",
	};

	// CSS classes for the overall row background tint, keyed by level string.
	// Debug rows are dimmed; others get a very subtle tint on the whole row.
	const ROW_TINT: Record<string, string> = {
		Debug: "opacity-50",
		Info: "",
		Warn: "bg-yellow-500/5",
		Error: "bg-red-500/8",
		Perf: "bg-indigo-500/5",
	};

	const badgeClass = $derived(LEVEL_BADGE[event.level] ?? "bg-surface-raised text-content-muted");
	const rowTintClass = $derived(ROW_TINT[event.level] ?? "");

	// Pretty-print the metadata JSON for the expanded panel. Falls back to
	// JSON.stringify when the value is not an object.
	const metadataJson = $derived(JSON.stringify(event.metadata, null, 2));
</script>

<!-- Absolute-positioned wrapper lets the virtualiser translate rows without
     reflowing the DOM. The inner content uses a normal block layout. -->
<div
	class="absolute left-0 right-0 {rowTintClass}"
	{style}
	role="row"
>
	<!-- Main row: fixed-height single line with all columns. -->
	<button
		class="flex w-full cursor-pointer items-center gap-0 px-2 py-0 text-left hover:bg-surface-raised/60 focus:outline-none focus-visible:ring-1 focus-visible:ring-blue-500"
		style="height: 24px; line-height: 24px;"
		onclick={() => (expanded = !expanded)}
		aria-expanded={expanded}
	>
		<!-- Timestamp: monospace, fixed width so columns align. -->
		<span class="w-[90px] shrink-0 font-mono text-[11px] text-content-muted tabular-nums">
			{formatTimestamp(event.timestamp)}
		</span>

		<!-- Level badge: small pill aligned left. -->
		<span
			class="mr-2 w-[42px] shrink-0 rounded px-1 py-0.5 text-center font-mono text-[10px] font-medium uppercase leading-none {badgeClass}"
		>
			{event.level}
		</span>

		<!-- Source: subtle muted label. -->
		<span class="w-[80px] shrink-0 truncate text-[11px] text-content-muted">
			{event.source}
		</span>

		<!-- Category: slightly stronger colour. -->
		<span class="w-[120px] shrink-0 truncate text-[11px] text-content-base/70">
			{event.category}
		</span>

		<!-- Message: fills remaining space, truncates with ellipsis. -->
		<span class="min-w-0 flex-1 truncate text-[11px] text-content-base">
			{event.message}
		</span>
	</button>

	<!-- Expanded metadata panel: shown below the row when expanded. Rendered
	     inline rather than absolutely so the virtualiser height calculation
	     accounts for it via a separate measurement pass. -->
	{#if expanded}
		<div class="border-b border-border bg-surface-raised/40 px-[90px] py-2">
			<pre class="max-h-60 overflow-auto font-mono text-[11px] text-content-base whitespace-pre-wrap">{metadataJson}</pre>
		</div>
	{/if}
</div>
