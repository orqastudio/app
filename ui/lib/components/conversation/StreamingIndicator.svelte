<script lang="ts">
	let { isThinking = false }: { isThinking?: boolean } = $props();
</script>

<div class="flex items-center gap-2 px-4 py-2 text-sm text-muted-foreground">
	<div class="flex gap-1">
		<span class="h-1.5 w-1.5 animate-bounce rounded-full bg-muted-foreground [animation-delay:0ms]"
		></span>
		<span
			class="h-1.5 w-1.5 animate-bounce rounded-full bg-muted-foreground [animation-delay:150ms]"
		></span>
		<span
			class="h-1.5 w-1.5 animate-bounce rounded-full bg-muted-foreground [animation-delay:300ms]"
		></span>
	</div>
	<span>{isThinking ? "Thinking..." : "Typing..."}</span>
</div>
