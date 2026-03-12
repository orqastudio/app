<script lang="ts">
	import type { Message } from "$lib/types";

	let { message }: { message: Message } = $props();

	const formattedTime = $derived(
		new Date(message.created_at).toLocaleTimeString(undefined, {
			hour: "2-digit",
			minute: "2-digit",
		})
	);
</script>

<div class="flex justify-end">
	<div class="max-w-[80%] space-y-1">
		<div class="rounded-2xl rounded-tr-sm bg-primary px-4 py-2.5 text-primary-foreground">
			<p class="whitespace-pre-wrap text-sm">{message.content ?? ""}</p>
		</div>
		<p class="text-right text-xs text-muted-foreground">{formattedTime}</p>
	</div>
</div>
