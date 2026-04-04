<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import UserMessage from "./UserMessage.svelte";
	import AssistantMessage from "./AssistantMessage.svelte";
	import SystemMessage from "./SystemMessage.svelte";
	import ToolCallCard from "$lib/components/tool/ToolCallCard.svelte";
	import { Box } from "@orqastudio/svelte-components/pure";

	let {
		message,
		streamingContent,
	}: {
		message: Message;
		streamingContent?: string;
	} = $props();

	const isToolUse = $derived(message.content_type === "tool_use");
	const isToolResult = $derived(message.content_type === "tool_result");
</script>

{#if isToolUse || isToolResult}
	<Box paddingX={4}>
		<ToolCallCard
			toolName={message.tool_name ?? "unknown"}
			toolInput={isToolUse ? message.content : message.tool_input}
			toolOutput={isToolResult ? message.content : null}
			isError={message.tool_is_error}
			isComplete={message.stream_status === "complete"}
		/>
	</Box>
{:else if message.role === "user"}
	<UserMessage {message} />
{:else if message.role === "assistant"}
	<AssistantMessage {message} {streamingContent} />
{:else if message.role === "system"}
	<SystemMessage {message} />
{/if}
