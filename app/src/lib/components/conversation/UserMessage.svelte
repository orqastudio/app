<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import { Caption, Stack, HStack, ChatBubble, Text } from "@orqastudio/svelte-components/pure";

	let { message }: { message: Message } = $props();

	const formattedTime = $derived(
		new Date(message.created_at).toLocaleTimeString(undefined, {
			hour: "2-digit",
			minute: "2-digit",
		}),
	);
</script>

<Stack gap={1}>
	<ChatBubble role="user">
		<Text variant="body" block>{message.content ?? ""}</Text>
	</ChatBubble>
	<HStack justify="end">
		<Caption block>{formattedTime}</Caption>
	</HStack>
</Stack>
