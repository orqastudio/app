<!-- Individual process status card. Displays process name, color-coded status
     indicator, PID, uptime, memory usage, source footer, and binary path on
     hover. Clicking the card fires an onselect event so the parent view can
     filter the log table to this process's source. All layout and visual
     treatment comes from the @orqastudio/svelte-components/pure library. -->
<script lang="ts">
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardContent,
		CardFooter,
		Separator,
		ConnectionIndicator,
		type ConnectionState,
	} from "@orqastudio/svelte-components/pure";

	export type ProcessStatus = "running" | "stopped" | "crashed" | "not_found" | "unknown";

	export interface ProcessInfo {
		name: string;
		source: string;
		status: ProcessStatus;
		pid: number | null;
		uptime_seconds: number | null;
		memory_bytes: number | null;
		binary_path: string | null;
	}

	let {
		process,
		selected = false,
		onselect,
	}: {
		process: ProcessInfo;
		selected?: boolean;
		onselect?: (source: string) => void;
	} = $props();

	// Whether the card is currently hovered, showing the expanded binary path row.
	let hovered = $state(false);

	// Map ProcessStatus to the closest ConnectionState equivalent.
	// running → connected (green), crashed → disconnected (red),
	// stopped → waiting (muted/gray in context), not_found → waiting,
	// unknown → reconnecting (yellow).
	const connectionState = $derived<ConnectionState>(
		process.status === "running"
			? "connected"
			: process.status === "crashed"
				? "disconnected"
				: process.status === "unknown"
					? "reconnecting"
					: "waiting",
	);

	// Human-readable status label passed to ConnectionIndicator as an override.
	const statusLabel = $derived(
		process.status === "running"
			? "Running"
			: process.status === "crashed"
				? "Crashed"
				: process.status === "stopped"
					? "Stopped"
					: process.status === "not_found"
						? "Not found"
						: "Unknown",
	);

	// Format uptime_seconds into a human-readable string (e.g. "2h 14m" or "45s").
	function formatUptime(seconds: number): string {
		if (seconds < 60) return `${seconds}s`;
		const mins = Math.floor(seconds / 60);
		if (mins < 60) return `${mins}m`;
		const hours = Math.floor(mins / 60);
		const remainingMins = mins % 60;
		return remainingMins > 0 ? `${hours}h ${remainingMins}m` : `${hours}h`;
	}

	// Format memory_bytes into a human-readable string (MB or KB).
	function formatMemory(bytes: number): string {
		if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(1)} MB`;
		if (bytes >= 1_024) return `${(bytes / 1_024).toFixed(1)} KB`;
		return `${bytes} B`;
	}

	function handleClick() {
		onselect?.(process.source);
	}

	// Allow keyboard activation so the card is operable without a pointer.
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" || event.key === " ") {
			event.preventDefault();
			onselect?.(process.source);
		}
	}

	// Extract the filename portion of a binary path for the compact display.
	// Falls back to the full path when there is no directory separator.
	function binaryFilename(path: string): string {
		const last = path.replace(/\\/g, "/").split("/").pop();
		return last ?? path;
	}
</script>

<!-- CardRoot renders the card surface. role="button" with tabindex and
     keyboard handler replicates the original <button> semantics while using
     the library's card styling. aria-pressed tracks the selected state. -->
<CardRoot
	role="button"
	tabindex={0}
	aria-pressed={selected}
	data-selected={selected}
	onclick={handleClick}
	onkeydown={handleKeydown}
	onmouseenter={() => (hovered = true)}
	onmouseleave={() => (hovered = false)}
	class="process-card"
>
	<!-- Header: process name on the left, connection state indicator on the right. -->
	<CardHeader>
		<CardTitle>{process.name}</CardTitle>
		<span class="process-card__status">
			<ConnectionIndicator state={connectionState} label={statusLabel} />
		</span>
	</CardHeader>

	<!-- Content: detail rows for PID, uptime, and memory when available. -->
	<CardContent>
		{#if process.pid !== null}
			<div class="process-card__row">
				<span class="process-card__label">PID</span>
				<span class="process-card__value process-card__value--mono">{process.pid}</span>
			</div>
		{/if}

		{#if process.uptime_seconds !== null}
			<div class="process-card__row">
				<span class="process-card__label">Uptime</span>
				<span class="process-card__value">{formatUptime(process.uptime_seconds)}</span>
			</div>
		{/if}

		{#if process.memory_bytes !== null}
			<div class="process-card__row">
				<span class="process-card__label">Memory</span>
				<span class="process-card__value">{formatMemory(process.memory_bytes)}</span>
			</div>
		{/if}

		<!-- Placeholder row so all cards have consistent height when no details
		     are available from the daemon yet. -->
		{#if process.pid === null && process.uptime_seconds === null && process.memory_bytes === null}
			<span class="process-card__empty">No details available</span>
		{/if}
	</CardContent>

	<Separator />

	<!-- Footer: source identifier for log filtering reference. -->
	<CardFooter>
		<span class="process-card__source">source: {process.source}</span>
	</CardFooter>

	<!-- Binary path row: visible on hover when the daemon has reported the path.
	     Shows the filename in the row and the full absolute path as a tooltip. -->
	{#if hovered && process.binary_path !== null}
		<Separator />
		<CardFooter title={process.binary_path}>
			<div class="process-card__row process-card__row--full">
				<span class="process-card__label">Binary</span>
				<span class="process-card__value process-card__value--mono process-card__value--truncate" title={process.binary_path}>
					{binaryFilename(process.binary_path)}
				</span>
			</div>
		</CardFooter>
	{/if}
</CardRoot>

<style>
	/* Clickable card: pointer cursor and selected-state highlight ring. */
	:global(.process-card) {
		cursor: pointer;
		width: 100%;
		text-align: left;
		transition: border-color 150ms;
	}

	:global(.process-card[data-selected="true"]) {
		border-color: var(--color-accent-base);
		box-shadow: 0 0 0 1px var(--color-accent-base);
	}

	/* Header layout: name left, status indicator right. */
	:global(.process-card [data-slot="card-header"]) {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	/* Status indicator sits beside the title. */
	.process-card__status {
		font-size: var(--text-xs);
		font-weight: 500;
	}

	/* Detail rows: label on the left, value on the right. */
	.process-card__row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	/* Full-width row variant used in the binary path footer. */
	.process-card__row--full {
		width: 100%;
		gap: 0.5rem;
	}

	.process-card__label {
		font-size: var(--text-xs);
		color: var(--color-content-muted);
	}

	.process-card__value {
		font-size: var(--text-xs);
		color: var(--color-content-base);
	}

	.process-card__value--mono {
		font-family: var(--font-mono);
	}

	.process-card__value--truncate {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 70%;
	}

	.process-card__empty {
		font-size: var(--text-xs);
		color: var(--color-content-muted);
	}

	.process-card__source {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-content-muted);
	}
</style>
