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
		HStack,
		Caption,
		Code,
		Box,
		type ConnectionState,
	} from "@orqastudio/svelte-components/pure";
	import { assertNever } from "@orqastudio/types";

	export type ProcessStatus = "running" | "stopped" | "crashed" | "not_found" | "unknown";

	export interface ProcessInfo {
		readonly name: string;
		readonly source: string;
		readonly status: ProcessStatus;
		readonly pid: number | null;
		readonly uptime_seconds: number | null;
		readonly memory_bytes: number | null;
		readonly binary_path: string | null;
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
	/**
	 *
	 * @param status
	 */
	function resolveConnectionState(status: ProcessStatus): ConnectionState {
		switch (status) {
			case "running":
				return "connected";
			case "crashed":
				return "disconnected";
			case "unknown":
				return "reconnecting";
			case "stopped":
			case "not_found":
				return "waiting";
			default:
				return assertNever(status);
		}
	}

	// Human-readable status label passed to ConnectionIndicator as an override.
	/**
	 *
	 * @param status
	 */
	function resolveStatusLabel(status: ProcessStatus): string {
		switch (status) {
			case "running":
				return "Running";
			case "crashed":
				return "Crashed";
			case "stopped":
				return "Stopped";
			case "not_found":
				return "Not found";
			case "unknown":
				return "Unknown";
			default:
				return assertNever(status);
		}
	}

	const connectionState = $derived<ConnectionState>(resolveConnectionState(process.status));
	const statusLabel = $derived(resolveStatusLabel(process.status));

	// Format uptime_seconds into a human-readable string (e.g. "2h 14m" or "45s").
	/**
	 *
	 * @param seconds
	 */
	function formatUptime(seconds: number): string {
		if (seconds < 60) return `${seconds}s`;
		const mins = Math.floor(seconds / 60);
		if (mins < 60) return `${mins}m`;
		const hours = Math.floor(mins / 60);
		const remainingMins = mins % 60;
		return remainingMins > 0 ? `${hours}h ${remainingMins}m` : `${hours}h`;
	}

	// Format memory_bytes into a human-readable string (MB or KB).
	/**
	 *
	 * @param bytes
	 */
	function formatMemory(bytes: number): string {
		if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(1)} MB`;
		if (bytes >= 1_024) return `${(bytes / 1_024).toFixed(1)} KB`;
		return `${bytes} B`;
	}

	/**
	 *
	 */
	function handleClick() {
		onselect?.(process.source);
	}

	// Allow keyboard activation so the card is operable without a pointer.
	/**
	 *
	 * @param event
	 */
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" || event.key === " ") {
			event.preventDefault();
			onselect?.(process.source);
		}
	}

	// Extract the filename portion of a binary path for the compact display.
	// Falls back to the full path when there is no directory separator.
	/**
	 *
	 * @param path
	 */
	function binaryFilename(path: string): string {
		const last = path.replace(/\\/g, "/").split("/").pop();
		return last ?? path;
	}
</script>

<!-- Wrapper span: provides :global() hook for card styling overrides.
     CardRoot does not accept class; data-selected is passed through restProps. -->
<span class="process-card__wrap" style="display: contents;">
	<CardRoot
		data-selected={selected}
		onclick={handleClick}
		onkeydown={handleKeydown}
		onmouseenter={() => (hovered = true)}
		onmouseleave={() => (hovered = false)}
		aria-pressed={selected}
		tabindex={0}
	>
		<!-- Header: process name on the left, connection state indicator on the right.
		     Box provides the container for the status indicator without a raw span. -->
		<CardHeader>
			<CardTitle>{process.name}</CardTitle>
			<Box>
				<ConnectionIndicator state={connectionState} label={statusLabel} />
			</Box>
		</CardHeader>

		<!-- Content: detail rows for PID, uptime, and memory when available. -->
		<CardContent>
			{#if process.pid !== null}
				<HStack justify="between">
					<Caption>PID</Caption>
					<!-- Code already renders text-xs font-mono; no extra class needed. -->
					<Code>{process.pid}</Code>
				</HStack>
			{/if}

			{#if process.uptime_seconds !== null}
				<HStack justify="between">
					<Caption>Uptime</Caption>
					<!-- Caption renders text-xs; replaces unsupported <Text size="xs">. -->
					<Caption>{formatUptime(process.uptime_seconds)}</Caption>
				</HStack>
			{/if}

			{#if process.memory_bytes !== null}
				<HStack justify="between">
					<Caption>Memory</Caption>
					<!-- Caption renders text-xs; replaces unsupported <Text size="xs">. -->
					<Caption>{formatMemory(process.memory_bytes)}</Caption>
				</HStack>
			{/if}

			<!-- Placeholder row so all cards have consistent height when no details
			     are available from the daemon yet. -->
			{#if process.pid === null && process.uptime_seconds === null && process.memory_bytes === null}
				<Caption>No details available</Caption>
			{/if}
		</CardContent>

		<Separator />

		<!-- Footer: source identifier for log filtering reference. -->
		<CardFooter>
			<Code>{process.source}</Code>
		</CardFooter>

		<!-- Binary path row: visible on hover when the daemon has reported the path.
		     Shows the filename in the row and the full absolute path as a tooltip. -->
		{#if hovered && process.binary_path !== null}
			<Separator />
			<!-- CardFooter title forwards to the underlying div via restProps. -->
			<CardFooter title={process.binary_path}>
				<HStack justify="between" full gap={2}>
					<Caption>Binary</Caption>
					<!-- Box contains the truncated filename. -->
					<Box>
						<!-- Wrapper span targets the code element for truncation styles. -->
						<span class="process-card__binary-wrap" style="display: contents;">
							<Code>{binaryFilename(process.binary_path)}</Code>
						</span>
					</Box>
				</HStack>
			</CardFooter>
		{/if}
	</CardRoot>
</span>

<style>
	/* Clickable card: pointer cursor, full width, and selected-state highlight ring.
	   Targets CardRoot inside the wrapper span via data-slot. */
	:global(.process-card__wrap [data-slot="card"]) {
		cursor: pointer;
		width: 100%;
		text-align: left;
		transition: border-color 150ms;
	}

	:global(.process-card__wrap [data-slot="card"][data-selected="true"]) {
		border-color: var(--color-accent-base);
		box-shadow: 0 0 0 1px var(--color-accent-base);
	}

	/* Header layout: name left, status indicator right. */
	:global(.process-card__wrap [data-slot="card-header"]) {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	/* Binary filename code: truncates with ellipsis within the Box overflow container.
	   Targets code element inside the binary wrapper span. */
	:global(.process-card__binary-wrap code) {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 70%;
		display: block;
	}
</style>
