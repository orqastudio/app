<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import { Badge, Caption, Stack, HStack } from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
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

<HStack align="start" justify="start">
	<!-- max-w-[85%] is a responsive sizing constraint with no ORQA primitive equivalent -->
	<div class="max-w-[85%]">
	<Stack gap={1}>
		<!-- rounded-2xl rounded-tl-sm border bg-muted/50 are visual card styles; no ORQA equivalent for asymmetric radius -->
		<div class="rounded-2xl rounded-tl-sm border border-border bg-muted/50 px-4 py-2.5">
			{#if isActivelyStreaming}
				<!-- streaming-text is a scoped style class; pre is structural for whitespace preservation during streaming -->
				<pre class="streaming-text">{displayContent}<span class="cursor-blink" aria-hidden="true"></span></pre>
			{:else if displayContent}
				<MarkdownRenderer content={displayContent} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
			{:else if isStreaming}
				<StreamingIndicator />
			{/if}
		</div>
		<HStack gap={2}>
			<Caption>{formattedTime}</Caption>
			{#if message.input_tokens || message.output_tokens}
				<Badge variant="outline" size="xs">
					{message.input_tokens ?? 0}↑ {message.output_tokens ?? 0}↓
				</Badge>
			{/if}
		</HStack>
	</Stack>
	</div>
</HStack>

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
