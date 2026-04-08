<!-- IssueRow renders a single row in the Issues list. Displays severity badge,
     truncated title with component+timestamp caption, event count, and a sparkline.
     Used inside a vertically scrolling issue list — one instance per grouped issue. -->
<script lang="ts">
	import { cn } from "../../utils/cn.js";
	import { Stack } from "../layout/index.js";
	import { Text, Caption } from "../typography/index.js";
	import { SmallBadge } from "../small-badge/index.js";
	import { CountBadge } from "../count-badge/index.js";
	import { Sparkline } from "../sparkline/index.js";
	import type { BadgeVariant } from "../badge/index.js";

	let {
		title,
		component,
		level,
		last_seen,
		count,
		sparkline_buckets,
		selected = false,
		onclick,
	}: {
		/** Unique fingerprint for the issue group (used as key by parent list). */
		fingerprint: string;
		/** Issue title shown as the primary text. */
		title: string;
		/** Component/module name where the issue originates. */
		component: string;
		/** Severity level label: Error, Warn, Info, Debug, or Perf. */
		level: string;
		/** Unix timestamp (ms) of the first occurrence. */
		first_seen: number;
		/** Unix timestamp (ms) of the most recent occurrence. */
		last_seen: number;
		/** Total event count for this issue group. */
		count: number;
		/** Bucketed event counts for sparkline rendering (left-to-right). */
		sparkline_buckets: number[];
		/** Whether this row is currently selected. Applies a highlighted background. */
		selected?: boolean;
		/** Click handler called when the row is activated. */
		onclick?: () => void;
	} = $props();

	/**
	 * Maps issue severity level to its SmallBadge variant.
	 * @param l - The severity level string (Error, Warn, Info, Debug, Perf).
	 * @returns The corresponding BadgeVariant for SmallBadge.
	 */
	function levelToVariant(l: string): BadgeVariant {
		switch (l) {
			case "Error":
				return "destructive";
			case "Warn":
				return "warning";
			case "Info":
				return "default";
			case "Debug":
				return "secondary";
			case "Perf":
				return "secondary";
			default:
				return "secondary";
		}
	}

	/**
	 * Converts a Unix millisecond timestamp to a human-readable relative time string.
	 * Returns strings like "just now", "2m ago", "1h ago", "3d ago".
	 * @param timestamp - Unix timestamp in milliseconds.
	 * @returns A short relative time string.
	 */
	function relativeTime(timestamp: number): string {
		const diff = Date.now() - timestamp;
		const seconds = Math.floor(diff / 1000);
		if (seconds < 60) return "just now";
		const minutes = Math.floor(seconds / 60);
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}

	/**
	 * Handles keyboard events for the row, activating onclick on Enter or Space
	 * to match native button semantics.
	 * @param e - The keyboard event.
	 */
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === "Enter" || e.key === " ") {
			e.preventDefault();
			onclick?.();
		}
	}

	/**
	 * Maps issue severity level to a sparkline stroke color using CSS custom properties.
	 * Mirrors the SmallBadge variant semantics for visual consistency.
	 * @param l - The severity level string (Error, Warn, Info, Debug, Perf).
	 * @returns A CSS color string for the sparkline stroke.
	 */
	function levelToSparklineColor(l: string): string {
		switch (l) {
			case "Error":
				return "hsl(var(--destructive))";
			case "Warn":
				return "hsl(var(--warning))";
			case "Info":
				return "hsl(var(--primary))";
			default:
				return "hsl(var(--muted-foreground))";
		}
	}

	const variant = $derived(levelToVariant(level));
	const time = $derived(relativeTime(last_seen));
	const sparklineColor = $derived(levelToSparklineColor(level));
</script>

<div
	class={cn(
		"hover:bg-accent/50 flex w-full cursor-pointer items-center gap-3 rounded px-2 py-2 text-left",
		selected && "bg-accent",
	)}
	role="button"
	tabindex={0}
	{onclick}
	onkeydown={handleKeydown}
	aria-pressed={selected}
>
	<SmallBadge {variant}>{level}</SmallBadge>

	<Stack gap={0} flex={1} minHeight={0}>
		<Text variant="body-strong" truncate>{title}</Text>
		<Caption truncate>{component} · {time}</Caption>
	</Stack>

	<CountBadge {count} />

	<Sparkline values={sparkline_buckets} width={80} height={24} strokeColor={sparklineColor} />
</div>
