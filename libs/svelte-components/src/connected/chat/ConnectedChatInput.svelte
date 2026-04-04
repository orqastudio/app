<!--
  Wraps ChatInput with store bindings for sending messages.
  Reads from the conversation store for streaming state and delegates send/stop to it.
-->
<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import ChatInput from "../../pure/chat/ChatInput.svelte";

	let {
		sessionId,
	}: {
		/** ID of the active session to send messages into. */
		sessionId: number;
	} = $props();

	const { conversationStore } = getStores();

	const isStreaming = $derived(conversationStore.isStreaming);

	function handleSubmit(content: string) {
		conversationStore.sendMessage(sessionId, content);
	}

	function handleStop() {
		conversationStore.stopStreaming(sessionId);
	}
</script>

<ChatInput
	{isStreaming}
	onsubmit={handleSubmit}
	onstop={handleStop}
/>
