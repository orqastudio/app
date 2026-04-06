<!-- Process diagnostics view for OrqaDev. Shows a prominent Start/Stop button
     for the dev environment, plus a grid of ProcessCard components auto-discovered
     from the daemon health endpoint. Status is polled every 2 seconds via the
     devtools_process_status IPC command. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { Button, ConnectionIndicator, EmptyState, Panel, Stack, HStack, Grid, Heading, Text, Code, ScrollArea, Center } from "@orqastudio/svelte-components/pure";
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
<ScrollArea full>
<Panel padding="normal">
<Stack gap={4}>
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
		<!-- Center replaces the raw centering div. -->
		<Center full>
			<Text variant="body-muted">Loading process status...</Text>
		</Center>
	{:else if processes.length === 0 && !isRunning}
		<!-- Center replaces the raw centering div for the empty state. -->
		<Center full>
			<EmptyState
				title="Dev environment is stopped"
				description='Click "Start Dev Environment" to launch daemon, search, Vite, and Tauri.'
			/>
		</Center>
	{:else}
		<!-- Responsive card grid: 2 → md:3 → lg:5 columns.
		     Wrapper div provides the scoped lg:5-column override since Grid has no lg prop. -->
		<div class="process-view__grid-wrap">
			<Grid cols={2} md={3} gap={3}>
				{#each processes as process (process.source)}
					<ProcessCard
						{process}
						selected={selectedSource === process.source}
						onselect={handleSelect}
					/>
				{/each}
			</Grid>
		</div>

		{#if selectedSource !== null}
			<!-- Scoped CSS class provides the hint strip visual treatment. -->
			<div class="process-view__filter-hint">
				<Text variant="body-muted" block>
					Showing logs for source <Code>{selectedSource}</Code>.
					Click the card again to clear the filter. Navigate to the
					<Text variant="body">Logs</Text> tab to see filtered entries.
				</Text>
			</div>
		{/if}
	{/if}
</Stack>
</Panel>
</ScrollArea>

<style>
	/* Wrapper div for the process grid. lg breakpoint overrides to 5 columns
	   since Grid's built-in md max prop does not cover lg. */
	@media (min-width: 1024px) {
		:global(.process-view__grid-wrap > div) {
			grid-template-columns: repeat(5, 1fr) !important;
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
