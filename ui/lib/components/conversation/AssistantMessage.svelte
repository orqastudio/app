<script lang="ts">
	import type { Message } from "$lib/types";
	import { Badge } from "$lib/components/ui/badge";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import StreamingIndicator from "./StreamingIndicator.svelte";

	let { message, streamingContent }: { message: Message; streamingContent?: string } = $props();

	const isStreaming = $derived(message.stream_status === "pending");

	const displayContent = $derived(
		isStreaming && streamingContent ? streamingContent : (message.content ?? "")
	);

	const formattedTime = $derived(
		new Date(message.created_at).toLocaleTimeString(undefined, {
			hour: "2-digit",
			minute: "2-digit",
		})
	);
</script>

<div class="flex justify-start">
	<div class="max-w-[85%] space-y-1">
		<div class="rounded-2xl rounded-tl-sm border border-border bg-muted/50 px-4 py-2.5">
			{#if displayContent}
				<MarkdownRenderer content={displayContent} />
			{:else if isStreaming}
				<StreamingIndicator />
			{/if}
		</div>
		<div class="flex items-center gap-2">
			<p class="text-xs text-muted-foreground">{formattedTime}</p>
			{#if message.input_tokens || message.output_tokens}
				<Badge variant="outline" class="text-[10px]">
					{message.input_tokens ?? 0}↑ {message.output_tokens ?? 0}↓
				</Badge>
			{/if}
		</div>
	</div>
</div>
