<!--
  Thin wrapper — delegates to the shared ChatInput primitive.
  Keeps the existing props interface (onsend/onstop) for compatibility with ConversationView.
-->
<script lang="ts">
	import { ChatInput } from "@orqastudio/svelte-components/pure";

	let {
		isStreaming = false,
		onsend,
		onstop,
	}: {
		isStreaming?: boolean;
		onsend: (content: string) => void;
		onstop?: () => void;
	} = $props();
</script>

<ChatInput {isStreaming} onsubmit={onsend} {onstop} />
