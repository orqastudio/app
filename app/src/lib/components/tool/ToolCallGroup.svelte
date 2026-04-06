<script lang="ts">
	import {
		Icon,
		HStack,
		Caption,
		Stack,
		Box,
		IndentedBlock,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleSection,
	} from "@orqastudio/svelte-components/pure";
	import ToolCallCard from "./ToolCallCard.svelte";
	import { getToolDisplay, groupLabel } from "$lib/utils/tool-display";

	interface ToolCallInfo {
		toolCallId: string;
		toolName: string;
		input: string | null;
		output: string | null;
		isError: boolean;
		isComplete: boolean;
	}

	let {
		toolName,
		toolCalls,
	}: {
		toolName: string;
		toolCalls: ToolCallInfo[];
	} = $props();

	let open = $state(false);

	const displayInfo = $derived(getToolDisplay(toolName));
	const label = $derived(groupLabel(toolName, toolCalls.length));
	const errorCount = $derived(toolCalls.filter((c) => c.isError).length);
</script>

<Collapsible bind:open>
	<CollapsibleSection>
		<HStack gap={2}>
			<Icon name="chevron-right" size="sm" />
			<Icon name={displayInfo.iconName} size="sm" tone="muted" />
			<!-- flex-1 fills the trigger row so the status icon sits flush right -->
			<Box flex={1} minWidth={0}><Caption variant="caption-mono" truncate>{label}</Caption></Box>
			{#if errorCount > 0}
				<HStack gap={1}>
					<Icon name="x-circle" size="sm" />
					<Caption tone="destructive">{errorCount} {errorCount === 1 ? "error" : "errors"}</Caption>
				</HStack>
			{:else}
				<Icon name="check-circle" size="sm" />
			{/if}
		</HStack>
	</CollapsibleSection>
	<CollapsibleContent>
		<IndentedBlock>
			<Stack gap={1}>
				{#each toolCalls as toolCall (toolCall.toolCallId)}
					<ToolCallCard
						toolName={toolCall.toolName}
						toolInput={toolCall.input}
						toolOutput={toolCall.output}
						isError={toolCall.isError}
						isComplete={toolCall.isComplete}
					/>
				{/each}
			</Stack>
		</IndentedBlock>
	</CollapsibleContent>
</Collapsible>
