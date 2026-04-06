<!-- Process diagnostics view for OrqaDev. Shows a prominent Start/Stop button
     for the dev environment, plus a grid of ProcessCard components auto-discovered
     from the daemon health endpoint. Status is polled every 2 seconds via the
     devtools_process_status IPC command. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import {
		Button,
		Callout,
		ConnectionIndicator,
		EmptyState,
		Panel,
		Stack,
		HStack,
		Grid,
		Heading,
		Text,
		Code,
		ScrollArea,
		Center,
	} from "@orqastudio/svelte-components/pure";
	import { assertNever } from "@orqastudio/types";
	import ProcessCard from "./ProcessCard.svelte";
	import type { ProcessInfo } from "./ProcessCard.svelte";
	import { devController, startDev, stopDev } from "../../stores/dev-controller.svelte.js";

	const POLL_INTERVAL_MS = 2000;

	let processes = $state<ProcessInfo[]>([]);
	let initialized = $state(false);
	let selectedSource = $state<string | null>(null);
	let pollError = $state(false);
	let pollTimer: ReturnType<typeof setInterval> | null = null;

	/**
	 * Fetch the current process status list from the Rust backend via IPC and update the reactive state.
	 * @returns Resolves after the status list is fetched and stored.
	 */
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

	/**
	 * Toggle the selected process source filter so the log table highlights events from that process.
	 * @param source - The process source string to select or deselect.
	 */
	function handleSelect(source: string): void {
		selectedSource = selectedSource === source ? null : source;
	}

	const isRunning = $derived(
		devController.state === "running" || devController.state === "starting",
	);
	const isBusy = $derived(devController.state === "starting" || devController.state === "stopping");

	/**
	 * Toggle the dev environment: starts it if stopped, stops it if running.
	 */
	function handleToggle(): void {
		if (isRunning) {
			stopDev();
		} else {
			startDev();
		}
	}

	/**
	 * Resolve the start/stop button label from the dev controller state with exhaustiveness check.
	 * @param state - The current dev controller state.
	 * @returns A label string describing the current action or state.
	 */
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
			? ("disconnected" as const)
			: initialized && isRunning
				? ("connected" as const)
				: ("waiting" as const),
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
						description="Click 'Start Dev Environment' to launch daemon, search, Vite, and Tauri."
					/>
				</Center>
			{:else}
				<!-- Responsive card grid: 2 → md:3 → lg:5 columns. Grid handles all breakpoints natively. -->
				<Grid cols={2} md={3} lg={5} gap={3}>
					{#each processes as process (process.source)}
						<ProcessCard
							{process}
							selected={selectedSource === process.source}
							onselect={handleSelect}
						/>
					{/each}
				</Grid>

				{#if selectedSource !== null}
					<!-- Filter hint: Callout muted provides the bordered raised-surface strip. -->
					<Callout tone="muted" align="start">
						<Text variant="body-muted" block>
							Showing logs for source <Code>{selectedSource}</Code>. Click the card again to clear
							the filter. Navigate to the
							<Text variant="body">Logs</Text> tab to see filtered entries.
						</Text>
					</Callout>
				{/if}
			{/if}
		</Stack>
	</Panel>
</ScrollArea>
