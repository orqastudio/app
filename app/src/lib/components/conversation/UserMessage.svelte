<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import { Caption, Stack, HStack, Text } from "@orqastudio/svelte-components/pure";

	let { message }: { message: Message } = $props();

	const formattedTime = $derived(
		new Date(message.created_at).toLocaleTimeString(undefined, {
			hour: "2-digit",
			minute: "2-digit",
		})
	);
</script>

<HStack justify="end">
	<!-- max-w-[80%] is a responsive sizing constraint with no ORQA primitive equivalent -->
	<div class="max-w-[80%]">
		<Stack gap={1}>
			<!-- rounded-2xl rounded-tr-sm bg-primary are chat bubble visual styles; no ORQA equivalent for asymmetric radius or primary bg -->
			<div class="rounded-2xl rounded-tr-sm bg-primary px-4 py-2.5 text-primary-foreground">
				<Text variant="body" block>{message.content ?? ""}</Text>
			</div>
			<Caption block>{formattedTime}</Caption>
		</Stack>
	</div>
</HStack>
