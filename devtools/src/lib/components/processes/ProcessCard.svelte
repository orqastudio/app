<!-- Individual process status card. Displays process name, color-coded status dot,
     PID, uptime, and memory usage. Clicking the card fires an onselect event so
     the parent view can filter the log table to this process's source. -->
<script lang="ts">
	export type ProcessStatus = "running" | "stopped" | "crashed" | "unknown";

	export interface ProcessInfo {
		name: string;
		source: string;
		status: ProcessStatus;
		pid: number | null;
		uptime_seconds: number | null;
		memory_bytes: number | null;
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

	// Map status to a Tailwind colour class for the status dot.
	const dotClass = $derived(
		process.status === "running"
			? "bg-green-500"
			: process.status === "crashed"
				? "bg-red-500"
				: process.status === "stopped"
					? "bg-gray-400"
					: "bg-yellow-400",
	);

	// Human-readable status label.
	const statusLabel = $derived(
		process.status === "running"
			? "Running"
			: process.status === "crashed"
				? "Crashed"
				: process.status === "stopped"
					? "Stopped"
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
</script>

<!-- Card: rounded panel with a hover ring and a selected-state highlight.
     Cursor pointer indicates the card is clickable to filter logs. -->
<button
	class="bg-surface-raised border-border hover:border-border-focus flex w-full cursor-pointer flex-col gap-3 rounded-lg border p-4 text-left transition-colors
	       {selected ? 'border-accent-base ring-accent-base ring-1' : ''}"
	onclick={handleClick}
	aria-pressed={selected}
>
	<!-- Header row: process name + status dot -->
	<div class="flex items-center justify-between gap-2">
		<span class="text-content-base text-sm font-medium">{process.name}</span>
		<span class="flex items-center gap-1.5">
			<span class="size-2.5 rounded-full {dotClass}"></span>
			<span
				class="text-xs font-medium
				       {process.status === 'running'
					? 'text-green-500'
					: process.status === 'crashed'
						? 'text-red-500'
						: process.status === 'stopped'
							? 'text-content-muted'
							: 'text-yellow-400'}"
			>{statusLabel}</span>
		</span>
	</div>

	<!-- Detail rows: PID, uptime, memory. Each row is only rendered when the
	     field is available so cards don't show empty dashes for every field. -->
	<div class="flex flex-col gap-1">
		{#if process.pid !== null}
			<div class="flex items-center justify-between">
				<span class="text-content-muted text-xs">PID</span>
				<span class="text-content-base font-mono text-xs">{process.pid}</span>
			</div>
		{/if}

		{#if process.uptime_seconds !== null}
			<div class="flex items-center justify-between">
				<span class="text-content-muted text-xs">Uptime</span>
				<span class="text-content-base text-xs">{formatUptime(process.uptime_seconds)}</span>
			</div>
		{/if}

		{#if process.memory_bytes !== null}
			<div class="flex items-center justify-between">
				<span class="text-content-muted text-xs">Memory</span>
				<span class="text-content-base text-xs">{formatMemory(process.memory_bytes)}</span>
			</div>
		{/if}

		<!-- When no detail fields are available, show a muted placeholder row so
		     all cards have consistent height in the grid. -->
		{#if process.pid === null && process.uptime_seconds === null && process.memory_bytes === null}
			<span class="text-content-muted text-xs">No details available</span>
		{/if}
	</div>

	<!-- Footer: source identifier shown in small muted text for reference. -->
	<div class="border-border border-t pt-2">
		<span class="text-content-muted font-mono text-xs">source: {process.source}</span>
	</div>
</button>
