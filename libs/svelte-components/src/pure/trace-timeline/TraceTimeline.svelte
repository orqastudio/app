<!-- TraceTimeline — horizontal timeline with swim-lanes per source component.

Groups log events by their `source` field into labelled swim-lanes. Within each
lane, event nodes are positioned relative to the time range of all visible events
so the horizontal axis is a consistent shared time scale across lanes.

Clicking an event node calls onEventClick(eventId) so the host can navigate to
the full event detail. When the events array is empty, shows an EmptyState.
A time axis at the top shows relative timestamps for the left and right edges. -->

<script lang="ts">
	import { SvelteMap } from "svelte/reactivity";
	import { Stack } from "../layout/index.js";
	import { SmallBadge } from "../small-badge/index.js";
	import { Caption } from "../typography/index.js";
	import { EmptyState } from "../empty-state/index.js";
	import { Separator } from "../separator/index.js";
	import { Panel } from "../panel/index.js";

	/** A single event entry as accepted by TraceTimeline. */
	export interface TraceEvent {
		id: number;
		timestamp: number;
		source: string;
		level: string;
		message: string;
		correlation_id?: string;
	}

	export interface TraceTimelineProps {
		/** Events to render — may span multiple sources (swim-lanes). */
		events: TraceEvent[];
		/** Called when the user clicks an event node; receives the event id. */
		onEventClick?: (eventId: number) => void;
	}

	let { events, onEventClick }: TraceTimelineProps = $props();

	/**
	 * Derive swim-lane groups. Each unique source becomes one lane.
	 * Lanes are ordered by the timestamp of their first event so the
	 * top lane is whichever component emitted first.
	 */
	const lanes = $derived(() => {
		if (events.length === 0) return [];

		// Collect events per source using SvelteMap for reactive compatibility.
		const map = new SvelteMap<string, TraceEvent[]>();
		for (const ev of events) {
			const bucket = map.get(ev.source);
			if (bucket) {
				bucket.push(ev);
			} else {
				map.set(ev.source, [ev]);
			}
		}

		// Sort lanes by earliest event timestamp.
		return [...map.entries()]
			.map(([source, laneEvents]) => ({
				source,
				events: [...laneEvents].sort((a, b) => a.timestamp - b.timestamp),
			}))
			.sort((a, b) => a.events[0].timestamp - b.events[0].timestamp);
	});

	/** Earliest timestamp across all events — the left edge of the time axis. */
	const minTs = $derived(events.length > 0 ? Math.min(...events.map((e) => e.timestamp)) : 0);

	/** Latest timestamp across all events — the right edge of the time axis. */
	const maxTs = $derived(events.length > 0 ? Math.max(...events.map((e) => e.timestamp)) : 0);

	/** Total span in milliseconds. Avoids division-by-zero for single-event traces. */
	const span = $derived(maxTs > minTs ? maxTs - minTs : 1);

	/**
	 * Compute the percentage left-offset for a given timestamp.
	 * Clamps to [0, 100] to keep nodes inside the lane.
	 * @param ts - Unix millisecond timestamp.
	 * @returns Percentage string for use as CSS left value.
	 */
	function leftPercent(ts: number): string {
		const pct = Math.max(0, Math.min(100, ((ts - minTs) / span) * 100));
		return `${pct.toFixed(2)}%`;
	}

	/**
	 * Map a log level string to a CSS variable colour for the event dot.
	 * Defaults to the muted foreground for unknown levels.
	 * @param level - Log level string (e.g. "Error", "Warn", "Info").
	 * @returns CSS color value.
	 */
	function levelColor(level: string): string {
		switch (level) {
			case "Error":
				return "var(--color-destructive)";
			case "Warn":
				return "var(--color-warning, #f59e0b)";
			case "Debug":
				return "var(--color-muted-foreground)";
			case "Perf":
				return "var(--color-primary)";
			default:
				return "var(--color-foreground)";
		}
	}

	/**
	 * Format a relative time offset in milliseconds to a short display string.
	 * Values under one second show as "Xms"; larger values show as "X.Xs".
	 * @param ms - Milliseconds relative to trace start.
	 * @returns Human-readable relative time string.
	 */
	function formatRelative(ms: number): string {
		if (ms < 1000) return `${Math.round(ms)}ms`;
		return `${(ms / 1000).toFixed(1)}s`;
	}
</script>

{#if events.length === 0}
	<EmptyState
		icon="git-branch"
		title="No trace events"
		description="Select a correlation ID to view the trace."
	/>
{:else}
	<Panel padding="normal" background="none">
		<Stack gap={2}>
			<!-- Time axis: shows start offset (always 0) and end offset. -->
			<div class="trace-timeline__axis">
				<Caption variant="caption-mono" tone="muted">+0ms</Caption>
				<Caption variant="caption-mono" tone="muted">{formatRelative(span)}</Caption>
			</div>

			<Separator />

			<!-- Swim-lanes: one row per source component. -->
			{#each lanes() as lane (lane.source)}
				<div class="trace-timeline__lane">
					<!-- Lane label: fixed-width badge on the left. -->
					<div class="trace-timeline__lane-label">
						<SmallBadge variant="secondary">{lane.source}</SmallBadge>
					</div>

					<!-- Event rail: the positioned area where event dots live. -->
					<div class="trace-timeline__rail" aria-label="Events from {lane.source}">
						{#each lane.events as ev (ev.id)}
							<button
								class="trace-timeline__node"
								style="left: {leftPercent(ev.timestamp)}; background-color: {levelColor(ev.level)};"
								aria-label="{ev.level} event at +{formatRelative(
									ev.timestamp - minTs,
								)}: {ev.message}"
								type="button"
								onclick={() => onEventClick?.(ev.id)}
							></button>
						{/each}
					</div>
				</div>

				<Separator />
			{/each}
		</Stack>
	</Panel>
{/if}

<style>
	/* Time axis: labels at left and right edges of the timeline. */
	.trace-timeline__axis {
		display: flex;
		justify-content: space-between;
		padding: 0 calc(var(--trace-label-width, 120px) + 8px) 0 var(--trace-label-width, 120px);
	}

	/* Single swim-lane row: label + positioned rail side by side. */
	.trace-timeline__lane {
		display: flex;
		align-items: center;
		gap: 8px;
		min-height: 32px;
	}

	/* Fixed-width label column so all rails align vertically. */
	.trace-timeline__lane-label {
		flex: none;
		width: var(--trace-label-width, 120px);
		display: flex;
		justify-content: flex-end;
	}

	/* Relative-positioned rail fills remaining width; event dots are positioned
	   absolutely within it. */
	.trace-timeline__rail {
		flex: 1;
		position: relative;
		height: 24px;
		background: var(--color-muted, transparent);
		border-radius: 2px;
		overflow: visible;
	}

	/* Circular event marker. Positioned by inline left% computed from timestamp.
	   Vertically centred in the 24px-tall rail (half of 10px dot = 5px, so top 7px). */
	.trace-timeline__node {
		position: absolute;
		top: 7px;
		width: 10px;
		height: 10px;
		border-radius: 50%;
		border: none;
		padding: 0;
		cursor: pointer;
		transform: translateX(-50%);
		transition:
			transform 100ms ease,
			box-shadow 100ms ease;
	}

	.trace-timeline__node:hover {
		transform: translateX(-50%) scale(1.4);
		box-shadow: 0 0 0 2px var(--color-background, #fff);
	}

	.trace-timeline__node:focus-visible {
		outline: 2px solid var(--color-ring, #3b82f6);
		outline-offset: 2px;
	}
</style>
