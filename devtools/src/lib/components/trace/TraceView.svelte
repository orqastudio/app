<!-- TraceView — wires the TraceTimeline library component to the trace store.

Shows the correlation ID being traced as a header badge and renders the
TraceTimeline with the filtered events from trace-store. When no correlation
ID is selected, renders an EmptyState instructing the user to click a
correlation_id value in the Stream tab's Context panel.

Clicking an event node in the timeline opens the EventDrawer so the user can
inspect full event detail without leaving the Trace view. The trace event list
is used as the navigation context so next/prev stays within the trace. -->
<script lang="ts">
	import { TraceTimeline } from "@orqastudio/svelte-components/pure";
	import {
		EmptyState,
		Panel,
		HStack,
		Caption,
		SmallBadge,
		Button,
		Stack,
	} from "@orqastudio/svelte-components/pure";
	import {
		getSelectedCorrelationId,
		traceEvents,
		clearTrace,
	} from "../../stores/trace-store.svelte.js";
	import { openDrawer } from "../../stores/drawer-store.svelte.js";
	import type { LogEvent } from "../../stores/log-store.svelte.js";

	// Reactive derivations from the trace store.
	const correlationId = $derived(getSelectedCorrelationId());
	const events = $derived(traceEvents());

	/**
	 * Handle an event node click in the timeline. Finds the full LogEvent by id
	 * from the trace event list and opens the EventDrawer with the trace list as
	 * navigation context so next/prev stays within the trace.
	 * @param eventId - The id of the event node that was clicked.
	 */
	function handleEventClick(eventId: number): void {
		const found = events.find((ev: LogEvent) => ev.id === eventId);
		if (found) {
			openDrawer(found, events);
		}
	}
</script>

<Panel padding="normal" background="none" height="full">
	<Stack gap={3} height="full">
		{#if correlationId !== null}
			<!-- Trace header: shows the active correlation ID and a clear button. -->
			<HStack gap={2} justify="between" full>
				<HStack gap={2}>
					<Caption tone="muted">Tracing:</Caption>
					<SmallBadge variant="secondary">{correlationId}</SmallBadge>
				</HStack>
				<Button variant="ghost" size="sm" onclick={clearTrace}>Clear</Button>
			</HStack>

			<!-- Timeline rendering events for the selected correlation ID. -->
			<TraceTimeline {events} onEventClick={handleEventClick} />
		{:else}
			<EmptyState
				icon="git-branch"
				title="No trace selected"
				description="Click a correlation_id value in the Context panel to trace it here."
			/>
		{/if}
	</Stack>
</Panel>
