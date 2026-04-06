<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import {
		Icon,
		HStack,
		Caption,
		Stack,
		Badge,
		Box,
		IndentedBlock,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleSection,
	} from "@orqastudio/svelte-components/pure";
	import ToolCallCard from "./ToolCallCard.svelte";
	import { getToolDisplay, groupLabel, stripToolName } from "$lib/utils/tool-display";

	let { messages }: { messages: Message[] } = $props();

	let open = $state(false);

	// Pair tool_use messages with their corresponding tool_result
	interface ToolPair {
		toolName: string;
		input: string | null;
		output: string | null;
		isError: boolean;
		id: number;
	}

	const toolPairs = $derived.by(() => {
		const pairs: ToolPair[] = [];
		const pendingUses: Record<string, Message> = {};

		for (const msg of messages) {
			if (msg.content_type === "tool_use" && msg.tool_call_id) {
				pendingUses[msg.tool_call_id] = msg;
			} else if (msg.content_type === "tool_result" && msg.tool_call_id) {
				const useMsg = pendingUses[msg.tool_call_id];
				pairs.push({
					toolName: useMsg?.tool_name ?? msg.tool_name ?? "unknown",
					input: useMsg?.content ?? msg.tool_input ?? null,
					output: msg.content,
					isError: msg.tool_is_error,
					id: msg.id,
				});
				delete pendingUses[msg.tool_call_id];
			}
		}

		// Any unmatched tool_use (no result yet — shouldn't happen for completed turns)
		for (const useMsg of Object.values(pendingUses)) {
			pairs.push({
				toolName: useMsg.tool_name ?? "unknown",
				input: useMsg.content,
				output: null,
				isError: false,
				id: useMsg.id,
			});
		}

		return pairs;
	});

	// Group by stripped tool name for the summary
	const groupedCounts = $derived.by(() => {
		const counts: Record<string, number> = {};
		for (const pair of toolPairs) {
			const stripped = stripToolName(pair.toolName);
			counts[stripped] = (counts[stripped] ?? 0) + 1;
		}
		return counts;
	});

	const totalTools = $derived(toolPairs.length);
	const errorCount = $derived(toolPairs.filter((p) => p.isError).length);

	const summaryLabel = $derived.by(() => {
		const entries = Object.entries(groupedCounts);
		if (entries.length === 1) {
			const [name, count] = entries[0];
			return groupLabel(name, count);
		}
		return `Used ${totalTools} tools`;
	});

	const summaryParts = $derived.by(() => {
		const entries = Object.entries(groupedCounts);
		if (entries.length <= 1) return [];
		return entries.map(([name, count]) => {
			const display = getToolDisplay(name);
			return { name, count, label: display.label, iconName: display.iconName };
		});
	});
</script>

{#if totalTools > 0}
	<Collapsible bind:open>
		<CollapsibleSection>
			<HStack gap={2}>
				<Icon name="chevron-right" size="sm" />
				<Icon name="wrench" size="sm" />
				<!-- flex-1 fills the trigger row so the error indicator sits flush right -->
				<Box flex={1} minWidth={0}><Caption tone="muted">{summaryLabel}</Caption></Box>
				{#if errorCount > 0}
					<HStack gap={1}>
						<Icon name="x-circle" size="sm" />
						<Caption tone="destructive"
							>{errorCount} {errorCount === 1 ? "error" : "errors"}</Caption
						>
					</HStack>
				{/if}
			</HStack>
		</CollapsibleSection>
		<CollapsibleContent>
			<IndentedBlock>
				<Stack gap={1}>
					{#if summaryParts.length > 0}
						<HStack gap={2} wrap>
							{#each summaryParts as part (part.name)}
								<Badge variant="outline" size="xs">
									<Icon name={part.iconName} size="xs" />
									{part.label} ({part.count})
								</Badge>
							{/each}
						</HStack>
					{/if}
					{#each toolPairs as pair (pair.id)}
						<ToolCallCard
							toolName={pair.toolName}
							toolInput={pair.input}
							toolOutput={pair.output}
							isError={pair.isError}
							isComplete={true}
						/>
					{/each}
				</Stack>
			</IndentedBlock>
		</CollapsibleContent>
	</Collapsible>
{/if}
