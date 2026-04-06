<!-- Individual log table row. Renders timestamp, level badge, source, category,
     and message columns. Clicking the row toggles an expanded metadata panel
     that shows the raw metadata JSON for that event. Color-coded by level.
     A copy button appears on hover to copy the full entry to the clipboard. -->
<script lang="ts">
	import {
		Button,
		Code,
		LogRowShell,
		LogRowActions,
		LogRowMetadata,
		LogColumn,
		LogLevelBadge,
	} from "@orqastudio/svelte-components/pure";
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	let {
		event,
		topPx = 0,
		ondraweropen,
	}: {
		event: LogEvent;
		// Virtualiser-computed top offset in pixels, forwarded to LogRowShell.
		topPx?: number;
		// When provided, row clicks open the drawer instead of the inline expand panel.
		ondraweropen?: (event: LogEvent) => void;
	} = $props();

	// Whether the metadata panel is expanded for this row.
	let expanded = $state(false);

	// Brief confirmation text shown after a successful copy.
	let copyFeedback = $state<"idle" | "copied">("idle");

	/**
	 * Copy the full log entry as JSON to the clipboard, showing brief confirmation feedback.
	 * @param e - The click event; propagation is stopped to prevent row expansion.
	 * @returns Resolves after the clipboard write attempt completes.
	 */
	async function copyToClipboard(e: MouseEvent): Promise<void> {
		e.stopPropagation();
		try {
			await navigator.clipboard.writeText(JSON.stringify(event, null, 2));
			copyFeedback = "copied";
			setTimeout(() => {
				copyFeedback = "idle";
			}, 1500);
		} catch {
			// Clipboard API unavailable or denied — silently ignore.
		}
	}

	/**
	 * Format a Unix millisecond timestamp as HH:MM:SS.mmm for the time column.
	 * @param ms - Unix timestamp in milliseconds.
	 * @returns Time string in HH:MM:SS.mmm format.
	 */
	function formatTimestamp(ms: number): string {
		const d = new Date(ms);
		const hh = d.getHours().toString().padStart(2, "0");
		const mm = d.getMinutes().toString().padStart(2, "0");
		const ss = d.getSeconds().toString().padStart(2, "0");
		const mmm = d.getMilliseconds().toString().padStart(3, "0");
		return `${hh}:${mm}:${ss}.${mmm}`;
	}

	// Map log level to the LogRowShell level prop (lowercase).
	const ROW_LEVEL_MAP: Record<string, "debug" | "info" | "warn" | "error" | "perf"> = {
		Debug: "debug",
		Info: "info",
		Warn: "warn",
		Error: "error",
		Perf: "perf",
	};

	const rowLevel = $derived(ROW_LEVEL_MAP[event.level]);
	const metadataJson = $derived(JSON.stringify(event.metadata, null, 2));
</script>

<!-- LogRowShell provides absolute positioning + group class for hover-reveal actions. -->
<LogRowShell level={rowLevel} {topPx}>
	<!-- Main row: full-width row button with all columns. -->
	<Button
		variant="row"
		size="log-row"
		onclick={() => {
			if (ondraweropen) {
				ondraweropen(event);
			} else {
				expanded = !expanded;
			}
		}}
		aria-expanded={expanded}
	>
		<LogColumn variant="timestamp">{formatTimestamp(event.timestamp)}</LogColumn>
		<LogColumn variant="badge">
			<LogLevelBadge level={event.level} />
		</LogColumn>
		<LogColumn variant="source">{event.source}</LogColumn>
		<LogColumn variant="category">{event.category}</LogColumn>
		<LogColumn variant="fill">{event.message}</LogColumn>
	</Button>

	<!-- Copy-to-clipboard button: revealed on row hover via group-hover in LogRowActions. -->
	<LogRowActions>
		<Button
			variant="ghost"
			size="xs"
			onclick={copyToClipboard}
			aria-label="Copy log entry to clipboard"
			title="Copy as JSON"
		>
			{copyFeedback === "copied" ? "Copied" : "Copy"}
		</Button>
	</LogRowActions>

	<!-- Expanded metadata panel: shown below the row when expanded. -->
	{#if expanded}
		<LogRowMetadata>
			<Code block={true} compact={true}>{metadataJson}</Code>
		</LogRowMetadata>
	{/if}
</LogRowShell>
