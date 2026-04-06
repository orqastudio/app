<script lang="ts">
	import BrainIcon from "@lucide/svelte/icons/brain";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import {
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "../collapsible/index.js";
	import { Text } from "../typography/index.js";

	let { content, isStreaming = false }: { content: string; isStreaming?: boolean } = $props();

	let open = $state(true);

	$effect(() => {
		if (!isStreaming && content.length > 0) {
			open = false;
		}
	});
</script>

{#if content.length > 0}
	<Collapsible bind:open>
		<CollapsibleTrigger
			class="border-warning/30 bg-warning/5 hover:bg-warning/10 flex w-full items-center gap-2 rounded-lg border px-3 py-2 text-left text-sm transition-colors"
		>
			<ChevronRightIcon
				class="text-warning h-3.5 w-3.5 shrink-0 transition-transform {open ? 'rotate-90' : ''}"
			/>
			<BrainIcon class="text-warning h-3.5 w-3.5 shrink-0" />
			<Text variant="caption" tone="warning">{isStreaming ? "Reasoning..." : "Reasoning"}</Text>
			{#if isStreaming}
				<span class="bg-warning h-2 w-2 animate-pulse rounded-full"></span>
			{/if}
		</CollapsibleTrigger>
		<CollapsibleContent>
			<div class="border-warning/20 bg-warning/5 mt-1 rounded-lg border px-3 py-2">
				<pre class="text-warning font-mono text-xs whitespace-pre-wrap">{content}</pre>
			</div>
		</CollapsibleContent>
	</Collapsible>
{/if}
