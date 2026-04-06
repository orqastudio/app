<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import {
		Badge,
		Caption,
		Stack,
		HStack,
		ChatBubble,
		StreamingText,
	} from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
	import StreamingIndicator from "./StreamingIndicator.svelte";

	let { message, streamingContent }: { message: Message; streamingContent?: string } = $props();

	const isStreaming = $derived(message.stream_status === "pending");
	const isActivelyStreaming = $derived(isStreaming && !!streamingContent);

	const displayContent = $derived(
		isStreaming && streamingContent ? streamingContent : (message.content ?? ""),
	);

	const formattedTime = $derived(
		new Date(message.created_at).toLocaleTimeString(undefined, {
			hour: "2-digit",
			minute: "2-digit",
		}),
	);
</script>

<Stack gap={1}>
	<ChatBubble role="assistant">
		{#if isActivelyStreaming}
			<StreamingText content={displayContent} />
		{:else if displayContent}
			<MarkdownRenderer
				content={displayContent}
				codeRenderer={DiagramCodeBlock}
				linkRenderer={MarkdownLink}
			/>
		{:else if isStreaming}
			<StreamingIndicator />
		{/if}
	</ChatBubble>
	<HStack gap={2}>
		<Caption>{formattedTime}</Caption>
		{#if message.input_tokens || message.output_tokens}
			<Badge variant="outline" size="xs">
				{message.input_tokens ?? 0}↑ {message.output_tokens ?? 0}↓
			</Badge>
		{/if}
	</HStack>
</Stack>
