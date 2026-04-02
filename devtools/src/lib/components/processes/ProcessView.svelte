<!-- Process diagnostics view for OrqaDev. Shows a prominent Start/Stop button
     for the dev environment, plus a grid of ProcessCard components auto-discovered
     from the daemon health endpoint. Status is polled every 2 seconds via the
     devtools_process_status IPC command. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { Button, ConnectionIndicator, EmptyState } from "@orqastudio/svelte-components/pure";
	import ProcessCard from "./ProcessCard.svelte";
	import type { ProcessInfo } from "./ProcessCard.svelte";
	import {
		devController,
		startDev,
		stopDev,
	} from "../../stores/dev-controller.svelte.js";

	const POLL_INTERVAL_MS = 2000;

	let processes = $state<ProcessInfo[]>([]);
	let initialized = $state(false);
	let selectedSource = $state<string | null>(null);
	let pollError = $state(false);
	let pollTimer: ReturnType<typeof setInterval> | null = null;

	async function fetchStatus(): Promise<void> {
		try {
			const result = await invoke<ProcessInfo[]>("devtools_process_status");
			processes = result;
			pollError = false;
		} catch {
			pollError = true;
		} finally {
			initialized = true;
		}
	}

	function handleSelect(source: string): void {
		selectedSource = selectedSource === source ? null : source;
	}

	const isRunning = $derived(
		devController.state === "running" || devController.state === "starting",
	);
	const isBusy = $derived(
		devController.state === "starting" || devController.state === "stopping",
	);

	function handleToggle(): void {
		if (isRunning) {
			stopDev();
		} else {
			startDev();
		}
	}

	const buttonLabel = $derived(
		devController.state === "starting"
			? "Starting..."
			: devController.state === "stopping"
				? "Stopping..."
				: devController.state === "running"
					? "Stop Dev Environment"
					: "Start Dev Environment",
	);

	// Map poll error / running state to a ConnectionIndicator state value.
	const connectionState = $derived(
		pollError
			? "disconnected" as const
			: initialized && isRunning
				? "connected" as const
				: "waiting" as const,
	);

	// Human-readable label for the connection indicator shown beside the button.
	const connectionLabel = $derived(
		pollError ? "Daemon unreachable" : isRunning ? "Running" : undefined,
	);

	onMount(() => {
		fetchStatus();
		pollTimer = setInterval(fetchStatus, POLL_INTERVAL_MS);
	});

	onDestroy(() => {
		if (pollTimer !== null) {
			clearInterval(pollTimer);
		}
	});
</script>

<div class="process-view">
	<!-- Dev environment control -->
	<div class="process-view__header">
		<h2 class="process-view__title">Dev Environment</h2>
		<div class="process-view__controls">
			{#if pollError || (initialized && isRunning)}
				<ConnectionIndicator state={connectionState} label={connectionLabel} />
			{/if}
			<Button
				variant={isRunning ? "destructive" : "default"}
				size="sm"
				disabled={isBusy}
				onclick={handleToggle}
			>
				{buttonLabel}
			</Button>
		</div>
	</div>

	<!-- Process grid -->
	{#if !initialized}
		<div class="process-view__loading">Loading process status...</div>
	{:else if processes.length === 0 && !isRunning}
		<div class="process-view__empty-wrapper">
			<EmptyState
				title="Dev environment is stopped"
				description='Click "Start Dev Environment" to launch daemon, search, Vite, and Tauri.'
			/>
		</div>
	{:else}
		<div class="process-view__grid">
			{#each processes as process (process.source)}
				<ProcessCard
					{process}
					selected={selectedSource === process.source}
					onselect={handleSelect}
				/>
			{/each}
		</div>

		{#if selectedSource !== null}
			<div class="process-view__filter-hint">
				Showing logs for source <span class="process-view__filter-hint-source">{selectedSource}</span>.
				Click the card again to clear the filter. Navigate to the
				<span class="process-view__filter-hint-nav">Logs</span> tab to see filtered entries.
			</div>
		{/if}
	{/if}
</div>

<style>
	/* Outer container: fills the tab content area with vertical scroll. */
	.process-view {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: auto;
		padding: var(--spacing-4);
		gap: var(--spacing-4);
	}

	/* Top bar: title on the left, controls on the right. */
	.process-view__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.process-view__title {
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-content-base);
	}

	/* Controls group: indicator + button aligned right. */
	.process-view__controls {
		display: flex;
		align-items: center;
		gap: var(--spacing-3);
		font-size: var(--text-xs);
	}

	/* Loading placeholder centred in the remaining space. */
	.process-view__loading {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: var(--text-sm);
		color: var(--color-content-muted);
	}

	/* Empty state wrapper: centred, fills available height. */
	.process-view__empty-wrapper {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Responsive card grid. */
	.process-view__grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--spacing-3);
	}

	@media (min-width: 768px) {
		.process-view__grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.process-view__grid {
			grid-template-columns: repeat(5, 1fr);
		}
	}

	/* Hint strip shown when a source filter is active. */
	.process-view__filter-hint {
		background-color: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--spacing-2) var(--spacing-3);
		font-size: var(--text-xs);
		color: var(--color-content-muted);
	}

	.process-view__filter-hint-source {
		font-family: var(--font-mono);
		color: var(--color-content-base);
	}

	.process-view__filter-hint-nav {
		color: var(--color-content-base);
		font-weight: 500;
	}
</style>
