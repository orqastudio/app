<!-- Process diagnostics view for OrqaDev. Shows a grid of ProcessCard components
     auto-discovered from the daemon health endpoint. Status is polled every 2
     seconds via the devtools_process_status IPC command. CLI-managed processes
     (search, app) are tracked from process manager events in the log stream. -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import {
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
	import ProcessCard from "./ProcessCard.svelte";
	import type { ProcessInfo } from "./ProcessCard.svelte";
	import { getTrackedProcesses } from "../../stores/process-tracker.svelte.js";

	const POLL_INTERVAL_MS = 2000;

	let daemonProcesses = $state<ProcessInfo[]>([]);
	let initialized = $state(false);
	let selectedSource = $state<string | null>(null);
	let pollError = $state(false);
	let pollTimer: ReturnType<typeof setInterval> | null = null;

	// Merge daemon-reported processes with CLI process manager tracked processes.
	// Daemon reports itself + LSP + MCP. PM tracker reports search, app, storybook, etc.
	const processes = $derived.by(() => {
		const pmProcesses = getTrackedProcesses();
		const daemonSources = new Set(daemonProcesses.map((p) => p.source));
		const extra = pmProcesses.filter((p) => !daemonSources.has(p.source));
		return [...daemonProcesses, ...extra];
	});

	/** Fetch process status from the daemon health endpoint. */
	async function fetchStatus(): Promise<void> {
		try {
			const result = await invoke<ProcessInfo[]>("devtools_process_status");
			daemonProcesses = result;
			pollError = false;
		} catch {
			pollError = true;
		} finally {
			initialized = true;
		}
	}

	/**
	 * Toggle the selected process source filter for log highlighting.
	 * @param source - The process source string to select or deselect.
	 */
	function handleSelect(source: string): void {
		selectedSource = selectedSource === source ? null : source;
	}

	const connectionState = $derived(
		pollError
			? ("disconnected" as const)
			: initialized
				? ("connected" as const)
				: ("waiting" as const),
	);

	const connectionLabel = $derived(pollError ? "Daemon unreachable" : "Running");

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

<ScrollArea full>
	<Panel padding="normal">
		<Stack gap={4}>
			<HStack justify="between">
				<Heading level={5}>Processes</Heading>
				{#if initialized}
					<ConnectionIndicator state={connectionState} label={connectionLabel} />
				{/if}
			</HStack>

			{#if !initialized}
				<Center full>
					<Text variant="body-muted">Loading process status...</Text>
				</Center>
			{:else if processes.length === 0}
				<Center full>
					<EmptyState
						title="No processes detected"
						description="Waiting for the daemon health endpoint to report process status."
					/>
				</Center>
			{:else}
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
					<Callout tone="muted" align="start">
						<Text variant="body-muted" block>
							Showing logs for source <Code>{selectedSource}</Code>. Click the card again to clear
							the filter. Navigate to the
							<Text variant="body">Stream</Text> tab to see filtered entries.
						</Text>
					</Callout>
				{/if}
			{/if}
		</Stack>
	</Panel>
</ScrollArea>
