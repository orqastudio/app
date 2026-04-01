<!-- Individual process status card. Displays process name, color-coded status dot,
     PID, uptime, and memory usage. Clicking the card fires an onselect event so
     the parent view can filter the log table to this process's source. Hovering
     expands the card to show the binary path when available. -->
<script lang="ts">
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

	// Map status to a Tailwind colour class for the status dot.
	const dotClass = $derived(
		process.status === "running"
			? "bg-green-500"
			: process.status === "crashed"
				? "bg-red-500"
				: process.status === "stopped"
					? "bg-gray-400"
					: process.status === "not_found"
						? "bg-gray-600"
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
					: process.status === "not_found"
						? "Not found"
						: "Unknown",
	);

	// Colour class for the status text label.
	const statusTextClass = $derived(
		process.status === "running"
			? "text-green-500"
			: process.status === "crashed"
				? "text-red-500"
				: process.status === "stopped"
					? "text-content-muted"
					: process.status === "not_found"
						? "text-content-muted"
						: "text-yellow-400",
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

	// Extract the filename portion of a binary path for the compact display.
	// Falls back to the full path when there is no directory separator.
	function binaryFilename(path: string): string {
		const last = path.replace(/\\/g, "/").split("/").pop();
		return last ?? path;
	}
</script>

<!-- Card: rounded panel with a hover ring and a selected-state highlight.
     Cursor pointer indicates the card is clickable to filter logs.
     onmouseenter/leave toggle the hovered state to show/hide binary path. -->
<button
	class="bg-surface-raised border-border hover:border-border-focus flex w-full cursor-pointer flex-col gap-3 rounded-lg border p-4 text-left transition-colors
	       {selected ? 'border-accent-base ring-accent-base ring-1' : ''}"
	onclick={handleClick}
	onmouseenter={() => (hovered = true)}
	onmouseleave={() => (hovered = false)}
	aria-pressed={selected}
>
	<!-- Header row: process name + status dot -->
	<div class="flex items-center justify-between gap-2">
		<span class="text-content-base text-sm font-medium">{process.name}</span>
		<span class="flex items-center gap-1.5">
			<span class="size-2.5 rounded-full {dotClass}"></span>
			<span class="text-xs font-medium {statusTextClass}">{statusLabel}</span>
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

	<!-- Binary path: shown on hover when the daemon has reported it.
	     Displays the filename in the normal row and the full absolute path
	     as a tooltip title. Hidden when the binary path is unavailable. -->
	{#if hovered && process.binary_path !== null}
		<div
			class="border-border border-t pt-2"
			title={process.binary_path}
		>
			<div class="flex items-center justify-between gap-2">
				<span class="text-content-muted text-xs">Binary</span>
				<span class="text-content-muted font-mono text-xs truncate max-w-[70%]" title={process.binary_path}>
					{binaryFilename(process.binary_path)}
				</span>
			</div>
		</div>
	{/if}
</button>
