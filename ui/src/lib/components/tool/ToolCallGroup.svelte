<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import XCircleIcon from "@lucide/svelte/icons/x-circle";
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";
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
		class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
	>
		<ChevronRightIcon
			class="h-3.5 w-3.5 shrink-0 text-muted-foreground transition-transform {open
				? 'rotate-90'
				: ''}"
		/>
		{@const Icon = displayInfo.icon}
		<Icon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		<span class="flex-1 truncate font-mono text-xs">{label}</span>
		{#if errorCount > 0}
			<span class="flex items-center gap-1 text-xs text-destructive">
				<XCircleIcon class="h-3.5 w-3.5 shrink-0" />
				{errorCount}
				{errorCount === 1 ? "error" : "errors"}
			</span>
		{:else}
			<CheckCircleIcon class="h-3.5 w-3.5 shrink-0 text-success" />
		{/if}
	</CollapsibleTrigger>
	<CollapsibleContent>
		<div class="ml-3 mt-1 space-y-1 border-l-2 border-border pl-4">
			{#each toolCalls as toolCall (toolCall.toolCallId)}
				<ToolCallCard
					toolName={toolCall.toolName}
					toolInput={toolCall.input}
					toolOutput={toolCall.output}
					isError={toolCall.isError}
					isComplete={toolCall.isComplete}
				/>
			{/each}
		</div>
	</CollapsibleContent>
</Collapsible>
