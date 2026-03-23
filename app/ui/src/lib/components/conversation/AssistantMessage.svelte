<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import StreamingIndicator from "./StreamingIndicator.svelte";

	let { message, streamingContent }: { message: Message; streamingContent?: string } = $props();

	const isStreaming = $derived(message.stream_status === "pending");
	const isActivelyStreaming = $derived(isStreaming && !!streamingContent);

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
			{#if isActivelyStreaming}
				<pre class="streaming-text whitespace-pre-wrap font-[inherit] text-sm">{displayContent}<span class="cursor-blink" aria-hidden="true"></span></pre>
			{:else if displayContent}
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

<style>
	.streaming-text {
		margin: 0;
		line-height: 1.625;
	}

	.cursor-blink {
		display: inline-block;
		width: 2px;
		height: 1em;
		background-color: currentColor;
		vertical-align: text-bottom;
		margin-left: 1px;
		animation: blink 1s step-start infinite;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
	}
</style>
