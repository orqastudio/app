<!-- Process diagnostics view for OrqaDev. Shows the dependency graph with
     build/process status per node. Falls back to a flat card grid when the
     graph topology isn't available (e.g. production attach mode). -->
<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import {
		ConnectionIndicator,
		EmptyState,
		Panel,
		Stack,
		HStack,
		Grid,
		Heading,
		Text,
		ScrollArea,
		Center,
	} from "@orqastudio/svelte-components/pure";
	import ProcessCard from "./ProcessCard.svelte";
	import type { ProcessInfo } from "./ProcessCard.svelte";
	import ProcessGraphView from "./ProcessGraphView.svelte";
	import {
		topologyLoaded,
		initTopology,
		updateNodeStatus,
	} from "../../stores/graph-topology.svelte.js";
	import { getTrackedProcesses } from "../../stores/process-tracker.svelte.js";

	const POLL_INTERVAL_MS = 2000;

	let daemonProcesses = $state<ProcessInfo[]>([]);
	let initialized = $state(false);
	let pollError = $state(false);
	let pollTimer: ReturnType<typeof setInterval> | null = null;

	// Merge daemon-reported processes with CLI process manager tracked processes.
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
			// Sync daemon-reported process statuses into the topology graph.
			for (const p of result) {
				updateNodeStatus(p.source, p.status);
			}
			// Also read PM-written process statuses from disk for nodes the
			// daemon doesn't manage (search, app, libraries).
			try {
				const pmStatuses = await invoke<Record<string, string>>("devtools_process_statuses");
				for (const [nodeId, status] of Object.entries(pmStatuses)) {
					updateNodeStatus(nodeId, status);
				}
			} catch {
				// Not available — PM may not have written status yet.
			}
		} catch {
			pollError = true;
		} finally {
			initialized = true;
		}
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
		initTopology();
	});

	onDestroy(() => {
		if (pollTimer !== null) {
			clearInterval(pollTimer);
		}
	});
</script>

{#if topologyLoaded.value}
	<!-- Graph topology available — show the dependency graph view. -->
	<Stack gap={0} height="full">
		<Panel padding="tight" border="bottom">
			<HStack justify="between">
				<Heading level={5}>System Health</Heading>
				{#if initialized}
					<ConnectionIndicator state={connectionState} label={connectionLabel} />
				{/if}
			</HStack>
		</Panel>
		<ProcessGraphView />
	</Stack>
{:else}
	<!-- No topology — fallback to flat card grid (production attach mode). -->
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
							<ProcessCard {process} />
						{/each}
					</Grid>
				{/if}
			</Stack>
		</Panel>
	</ScrollArea>
{/if}
