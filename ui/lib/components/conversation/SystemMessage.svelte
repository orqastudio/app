<script lang="ts">
	import type { Message } from "$lib/types";

	let { message }: { message: Message } = $props();

	const isError = $derived(message.stream_status === "error");
</script>

<div class="flex justify-center">
	<div
		class="max-w-[90%] rounded-lg px-4 py-2 text-center text-xs {isError
			? 'bg-destructive/10 text-destructive'
			: 'bg-muted/30 text-muted-foreground'}"
	>
		<p>{message.content ?? ""}</p>
	</div>
</div>
