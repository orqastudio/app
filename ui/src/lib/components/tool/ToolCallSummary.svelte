<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import XCircleIcon from "@lucide/svelte/icons/x-circle";
	import type { Message } from "$lib/types";
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";
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
			return { name, count, label: display.label, icon: display.icon };
		});
	});
</script>

{#if totalTools > 0}
	<Collapsible bind:open>
		<CollapsibleTrigger
			class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
		>
			<ChevronRightIcon
				class="h-3.5 w-3.5 shrink-0 text-muted-foreground transition-transform {open
					? 'rotate-90'
					: ''}"
			/>
			<WrenchIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
			<span class="flex-1 text-xs text-muted-foreground">{summaryLabel}</span>
			{#if errorCount > 0}
				<span class="flex items-center gap-1 text-xs text-destructive">
					<XCircleIcon class="h-3.5 w-3.5 shrink-0" />
					{errorCount} {errorCount === 1 ? "error" : "errors"}
				</span>
			{/if}
		</CollapsibleTrigger>
		<CollapsibleContent>
			<div class="ml-3 mt-1 space-y-1 border-l-2 border-border pl-4">
				{#if summaryParts.length > 0}
					<div class="flex flex-wrap gap-2 py-1">
						{#each summaryParts as part (part.name)}
							{@const Icon = part.icon}
							<span class="flex items-center gap-1 rounded-md bg-muted px-2 py-0.5 text-xs text-muted-foreground">
								<Icon class="h-3 w-3" />
								{part.label} ({part.count})
							</span>
						{/each}
					</div>
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
			</div>
		</CollapsibleContent>
	</Collapsible>
{/if}
