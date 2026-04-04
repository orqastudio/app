<script lang="ts">
	import type { Message } from "@orqastudio/types";
	import { Caption } from "@orqastudio/svelte-components/pure";

	let { message }: { message: Message } = $props();

	const isError = $derived(message.stream_status === "error");
</script>

<div class="flex justify-center">
	<div
		class="max-w-[90%] rounded-lg px-4 py-2 text-center {isError
			? 'bg-destructive/10'
			: 'bg-muted/30'}"
	>
		<Caption tone={isError ? "destructive" : undefined}>{message.content ?? ""}</Caption>
	</div>
</div>
