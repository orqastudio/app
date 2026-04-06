<!-- Individual log table row. Renders timestamp, level badge, source, category,
     and message columns. Clicking the row toggles an expanded metadata panel
     that shows the raw metadata JSON for that event. Color-coded by level.
     A copy button appears on hover to copy the full entry to the clipboard. -->
<script lang="ts">
	import { Button, Badge, Code } from "@orqastudio/svelte-components/pure";
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	let {
		event,
		style = "",
		ondraweropen,
	}: {
		event: LogEvent;
		// Inline style string injected by the virtualiser for position/transform.
		style?: string;
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
		// Prevent the click from toggling the expand state.
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

	// Badge variant for the level indicator, keyed by level string.
	// Uses library Badge variants where they fit; falls back to secondary.
	const LEVEL_BADGE_VARIANT: Record<
		string,
		"secondary" | "destructive" | "outline" | "default" | "warning"
	> = {
		Debug: "outline",
		Info: "default",
		Warn: "warning",
		Error: "destructive",
		Perf: "secondary",
	};

	// CSS classes for the overall row background tint, keyed by level string.
	// Debug rows are dimmed; others get a very subtle tint on the whole row.
	const ROW_TINT: Record<string, string> = {
		Debug: "log-row--debug",
		Info: "",
		Warn: "log-row--warn",
		Error: "log-row--error",
		Perf: "log-row--perf",
	};

	const badgeVariant = $derived(LEVEL_BADGE_VARIANT[event.level] ?? "outline");
	const rowTintClass = $derived(ROW_TINT[event.level] ?? "");

	// Pretty-print the metadata JSON for the expanded panel. Falls back to
	// JSON.stringify when the value is not an object.
	const metadataJson = $derived(JSON.stringify(event.metadata, null, 2));
</script>

<!-- Absolute-positioned wrapper lets the virtualiser translate rows without
     reflowing the DOM. The inner content uses a normal block layout. -->
<div class="log-row {rowTintClass}" {style} role="row">
	<!-- Main row: fixed-height single line with all columns. The copy button
	     is positioned at the right edge and revealed on group hover.
	     Wrapper span with display:contents is invisible to layout; scoped class
	     provides the hook for :global() CSS overrides on the Button inside. -->
	<span class="log-row__main-wrap" style="display: contents;">
		<Button
			variant="ghost"
			style="height: 24px; line-height: 24px;"
			onclick={() => {
				if (ondraweropen) {
					ondraweropen(event);
				} else {
					expanded = !expanded;
				}
			}}
			aria-expanded={expanded}
		>
			<!-- Timestamp: monospace, fixed width so columns align. -->
			<span class="log-row__timestamp">
				{formatTimestamp(event.timestamp)}
			</span>

			<!-- Level badge: small pill aligned left, using shared Badge component.
		     Wrapper span with display:contents provides :global() hook without
		     affecting layout. -->
			<span class="log-row__badge-cell">
				<span class="log-row__badge-wrap" style="display: contents;">
					<Badge variant={badgeVariant}>
						{event.level}
					</Badge>
				</span>
			</span>

			<!-- Source: subtle muted label. -->
			<span class="log-row__source">
				{event.source}
			</span>

			<!-- Category: slightly stronger colour. -->
			<span class="log-row__category">
				{event.category}
			</span>

			<!-- Message: fills remaining space, truncates with ellipsis. -->
			<span class="log-row__message">
				{event.message}
			</span>
		</Button>
	</span>

	<!-- Copy-to-clipboard button: absolutely positioned at the right of the row,
	     visible only when the row is hovered. Wrapper span provides scoped hook. -->
	<span class="log-row__copy-wrapper">
		<span class="log-row__copy-inner" style="display: contents;">
			<Button
				variant="ghost"
				size="icon-sm"
				onclick={copyToClipboard}
				aria-label="Copy log entry to clipboard"
				title="Copy as JSON"
			>
				{copyFeedback === "copied" ? "Copied" : "Copy"}
			</Button>
		</span>
	</span>

	<!-- Expanded metadata panel: shown below the row when expanded. Rendered
	     inline rather than absolutely so the virtualiser height calculation
	     accounts for it via a separate measurement pass. -->
	{#if expanded}
		<div class="log-row__metadata">
			<!-- Code block replaces raw pre element for metadata JSON display.
		     Scoped wrapper provides the :global() hook without passing class to Code. -->
			<span class="log-row__code-wrap">
				<Code block={true}>{metadataJson}</Code>
			</span>
		</div>
	{/if}
</div>

<style>
	/* Outer row wrapper: absolute-positioned by the virtualiser. */
	.log-row {
		position: absolute;
		left: 0;
		right: 0;
	}

	/* Level-based background tints. */
	.log-row--debug {
		opacity: 0.5;
	}

	.log-row--warn {
		background-color: color-mix(in srgb, var(--color-warning) 5%, transparent);
	}

	.log-row--error {
		background-color: color-mix(in srgb, var(--color-destructive) 8%, transparent);
	}

	.log-row--perf {
		background-color: color-mix(in srgb, var(--color-secondary) 20%, transparent);
	}

	/* Main row button: full-width flex, overrides Button defaults for compact row display.
	   Targets the button element inside the .log-row__main-wrap display:contents span. */
	:global(.log-row__main-wrap button) {
		display: flex !important;
		width: 100% !important;
		cursor: pointer !important;
		align-items: center !important;
		gap: 0 !important;
		padding: 0 var(--spacing-2) !important;
		text-align: left !important;
		justify-content: flex-start !important;
		border-radius: 0 !important;
	}

	:global(.log-row__main-wrap button:focus-visible) {
		box-shadow: inset 0 0 0 1px var(--color-ring) !important;
	}

	/* Timestamp column: monospace, fixed width, tabular figures. */
	.log-row__timestamp {
		width: 90px;
		flex-shrink: 0;
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--color-content-muted);
		font-variant-numeric: tabular-nums;
	}

	/* Badge cell: fixed width to match column header. */
	.log-row__badge-cell {
		width: 52px;
		flex-shrink: 0;
		margin-right: var(--spacing-2);
		display: flex;
		align-items: center;
	}

	/* Override Badge sizing to fit the compact 24px row height.
	   Targets the badge element inside the .log-row__badge-wrap display:contents span. */
	:global(.log-row__badge-wrap [data-slot="badge"]) {
		font-family: var(--font-mono);
		font-size: 10px;
		line-height: 1;
		padding: 1px var(--spacing-1);
		text-transform: uppercase;
		width: 42px;
		justify-content: center;
	}

	/* Source column: muted, fixed width. */
	.log-row__source {
		width: 80px;
		flex-shrink: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 11px;
		color: var(--color-content-muted);
	}

	/* Category column: slightly stronger, fixed width. */
	.log-row__category {
		width: 120px;
		flex-shrink: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 11px;
		color: var(--color-muted-foreground);
	}

	/* Message column: fills remaining width. */
	.log-row__message {
		min-width: 0;
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: 11px;
		color: var(--color-foreground);
	}

	/* Copy button wrapper: absolute-positioned to the right; hidden by default,
	   visible when the row is hovered via the group-hover pattern. */
	.log-row__copy-wrapper {
		position: absolute;
		right: var(--spacing-1);
		top: 0;
		display: none;
		height: 24px;
		align-items: center;
	}

	.log-row:hover .log-row__copy-wrapper {
		display: flex;
	}

	/* Override Button size to match the 24px row height.
	   Targets the button element inside the .log-row__copy-inner display:contents span. */
	:global(.log-row__copy-inner button) {
		height: 20px !important;
		width: auto !important;
		padding: 0 var(--spacing-1-5) !important;
		font-size: 10px !important;
	}

	/* Expanded metadata panel: indented to align under the message column. */
	.log-row__metadata {
		border-bottom: 1px solid var(--color-border);
		background-color: color-mix(in srgb, var(--color-muted) 40%, transparent);
		padding: var(--spacing-2) 0 var(--spacing-2) 90px;
	}

	/* Code block for metadata JSON: compact pre-wrap display inside the expanded panel.
	   Targets the pre element inside the .log-row__code-wrap span. */
	:global(.log-row__code-wrap pre) {
		max-height: 15rem;
		overflow: auto;
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--color-foreground);
		white-space: pre-wrap;
	}
</style>
