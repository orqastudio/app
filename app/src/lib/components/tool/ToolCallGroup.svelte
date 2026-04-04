<script lang="ts">
	import { Icon, HStack, Caption, Stack,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
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
	<CollapsibleTrigger
		class="w-full rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
	>
		<HStack gap={2}>
			<Icon name="chevron-right" size="sm" />
			{@const ToolIcon = displayInfo.icon}
			<ToolIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
			<!-- flex-1 is structural (fills trigger row); caption-mono provides the text styling -->
			<span class="flex-1"><Caption variant="caption-mono" truncate>{label}</Caption></span>
			{#if errorCount > 0}
				<HStack gap={1}>
					<Icon name="x-circle" size="sm" />
					<Caption tone="destructive">{errorCount} {errorCount === 1 ? "error" : "errors"}</Caption>
				</HStack>
			{:else}
				<Icon name="check-circle" size="sm" />
			{/if}
		</HStack>
	</CollapsibleTrigger>
	<CollapsibleContent>
		<!-- border-l-2 and ml-3 are structural indentation; no ORQA primitive supports border-left -->
		<div class="ml-3 mt-1 border-l-2 border-border pl-4">
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
		</div>
	</CollapsibleContent>
</Collapsible>
