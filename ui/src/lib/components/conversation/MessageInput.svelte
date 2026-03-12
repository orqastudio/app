<script lang="ts">
	import SendIcon from "@lucide/svelte/icons/send";
	import SquareIcon from "@lucide/svelte/icons/square";
	import { Button } from "$lib/components/ui/button";

	let {
		isStreaming = false,
		onsend,
		onstop,
	}: {
		isStreaming?: boolean;
		onsend: (content: string) => void;
		onstop?: () => void;
	} = $props();

	let inputValue = $state("");
	let textareaRef = $state<HTMLTextAreaElement | null>(null);

	const canSend = $derived(inputValue.trim().length > 0 && !isStreaming);

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			send();
		}
	}

	function send() {
		const content = inputValue.trim();
		if (content.length === 0 || isStreaming) return;
		onsend(content);
		inputValue = "";
		resetTextareaHeight();
	}

	function handleInput() {
		autoResize();
	}

	function autoResize() {
		if (!textareaRef) return;
		textareaRef.style.height = "auto";
		const maxHeight = 200;
		textareaRef.style.height = `${Math.min(textareaRef.scrollHeight, maxHeight)}px`;
	}

	function resetTextareaHeight() {
		if (!textareaRef) return;
		textareaRef.style.height = "auto";
	}
</script>

<div class="border-t border-border bg-background p-3">
	<div class="flex items-center gap-2">
		<textarea
			bind:this={textareaRef}
			bind:value={inputValue}
			oninput={handleInput}
			onkeydown={handleKeydown}
			placeholder="Type a message..."
			rows={1}
			disabled={isStreaming}
			class="flex-1 resize-none rounded-lg border border-input bg-transparent px-3 py-2 text-sm shadow-xs outline-none transition-colors placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
		></textarea>

		{#if isStreaming}
			<Button variant="destructive" size="icon-sm" onclick={onstop} aria-label="Stop generating">
				<SquareIcon class="h-4 w-4" />
			</Button>
		{:else}
			<Button
				variant="default"
				size="icon-sm"
				onclick={send}
				disabled={!canSend}
				aria-label="Send message"
			>
				<SendIcon class="h-4 w-4" />
			</Button>
		{/if}
	</div>
	<p class="mt-1 text-center text-[10px] text-muted-foreground">
		Enter to send, Shift+Enter for newline
	</p>
</div>
