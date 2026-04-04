<!-- Process diagnostics view for OrqaDev. Shows a prominent Start/Stop button
     for the dev environment, plus a grid of ProcessCard components auto-discovered
     from the daemon health endpoint. Status is polled every 2 seconds via the
     devtools_process_status IPC command. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { Button, ConnectionIndicator, EmptyState, Stack, HStack, Grid, Heading, Text, Caption, Code, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { assertNever } from "@orqastudio/types";
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

	// Resolve button label from the current dev controller state with exhaustiveness check.
	function resolveButtonLabel(state: typeof devController.state): string {
		switch (state) {
			case "starting":
				return "Starting...";
			case "stopping":
				return "Stopping...";
			case "running":
				return "Stop Dev Environment";
			case "stopped":
				return "Start Dev Environment";
			default:
				return assertNever(state);
		}
	}

	const buttonLabel = $derived(resolveButtonLabel(devController.state));

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

<!-- Outer container: fills the tab content area with vertical scroll. -->
<ScrollArea class="h-full">
<Stack gap={4} class="p-4">
	<!-- Dev environment control: title on left, controls on right. -->
	<HStack justify="between">
		<Heading level={5}>Dev Environment</Heading>
		<HStack gap={3}>
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
		</HStack>
	</HStack>

	<!-- Process grid -->
	{#if !initialized}
		<div class="process-view__loading">
			<Text muted>Loading process status...</Text>
		</div>
	{:else if processes.length === 0 && !isRunning}
		<div class="process-view__empty-wrapper">
			<EmptyState
				title="Dev environment is stopped"
				description='Click "Start Dev Environment" to launch daemon, search, Vite, and Tauri.'
			/>
		</div>
	{:else}
		<!-- Responsive card grid: 2 → md:3 → lg:5 columns. -->
		<Grid cols={2} md={3} gap={3} class="process-view__grid">
			{#each processes as process (process.source)}
				<ProcessCard
					{process}
					selected={selectedSource === process.source}
					onselect={handleSelect}
				/>
			{/each}
		</Grid>

		{#if selectedSource !== null}
			<div class="process-view__filter-hint">
				<Text size="xs" muted>
					Showing logs for source <Code>{selectedSource}</Code>.
					Click the card again to clear the filter. Navigate to the
					<Text size="xs">Logs</Text> tab to see filtered entries.
				</Text>
			</div>
		{/if}
	{/if}
</Stack>
</ScrollArea>

<style>
	/* Loading placeholder centred in the remaining space. */
	.process-view__loading {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Empty state wrapper: centred, fills available height. */
	.process-view__empty-wrapper {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* lg:5 columns is not in Grid's built-in map so we override globally.
	   Grid passes the class down to its inner div. */
	@media (min-width: 1024px) {
		:global(.process-view__grid) {
			grid-template-columns: repeat(5, 1fr);
		}
	}

	/* Hint strip shown when a source filter is active. */
	.process-view__filter-hint {
		background-color: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--spacing-2) var(--spacing-3);
	}
</style>
