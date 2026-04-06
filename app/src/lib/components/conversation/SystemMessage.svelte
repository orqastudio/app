<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import { Caption, ChatBubble } from "@orqastudio/svelte-components/pure";

	let { message }: { message: Message } = $props();

	const isError = $derived(message.stream_status === "error");
</script>

<ChatBubble role="system" tone={isError ? "destructive" : "default"}>
	<Caption tone={isError ? "destructive" : undefined}>{message.content ?? ""}</Caption>
</ChatBubble>
