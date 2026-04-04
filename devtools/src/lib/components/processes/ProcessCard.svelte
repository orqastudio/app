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
		Text,
		Code,
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
			<HStack justify="between">
				<Caption>PID</Caption>
				<Code class="text-xs">{process.pid}</Code>
			</HStack>
		{/if}

		{#if process.uptime_seconds !== null}
			<HStack justify="between">
				<Caption>Uptime</Caption>
				<Text size="xs">{formatUptime(process.uptime_seconds)}</Text>
			</HStack>
		{/if}

		{#if process.memory_bytes !== null}
			<HStack justify="between">
				<Caption>Memory</Caption>
				<Text size="xs">{formatMemory(process.memory_bytes)}</Text>
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
		<Code class="text-xs">{process.source}</Code>
	</CardFooter>

	<!-- Binary path row: visible on hover when the daemon has reported the path.
	     Shows the filename in the row and the full absolute path as a tooltip. -->
	{#if hovered && process.binary_path !== null}
		<Separator />
		<CardFooter title={process.binary_path}>
			<HStack justify="between" class="w-full gap-2">
				<Caption>Binary</Caption>
				<span title={process.binary_path} class="overflow-hidden text-ellipsis whitespace-nowrap max-w-[70%]">
					<Code class="text-xs">{binaryFilename(process.binary_path)}</Code>
				</span>
			</HStack>
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
</style>
