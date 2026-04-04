<!-- Textarea with send/stop button. Auto-grows up to 200px. Submits on Enter. -->
<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { Button } from "../button/index.js";
	import { Textarea } from "../textarea/index.js";
	import { HStack } from "../layout/index.js";
	import { Caption } from "../typography/index.js";
	import { Kbd } from "../kbd/index.js";

	let {
		value = $bindable(""),
		placeholder = "Type a message...",
		disabled = false,
		isStreaming = false,
		onsubmit,
		onstop,
	}: {
		/** The current input value, bindable. */
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		/** When true, shows a stop button instead of send. */
		isStreaming?: boolean;
		/** Called with the trimmed message content when the user sends. Clears the field afterwards. */
		onsubmit: (content: string) => void;
		/** Called when the user clicks the stop button during streaming. */
		onstop?: () => void;
	} = $props();

	let textareaRef = $state<HTMLTextAreaElement | null | undefined>(null);

	const canSend = $derived(value.trim().length > 0 && !isStreaming && !disabled);

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			send();
		}
	}

	function send() {
		const content = value.trim();
		if (content.length === 0 || isStreaming || disabled) return;
		onsubmit(content);
		value = "";
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
	<HStack gap={2}>
		<Textarea
			bind:ref={textareaRef}
			bind:value
			oninput={handleInput}
			onkeydown={handleKeydown}
			{placeholder}
			rows={1}
			disabled={isStreaming || disabled}
		/>

		{#if isStreaming}
			<Button variant="destructive" size="icon-sm" onclick={onstop} aria-label="Stop generating">
				<Icon name="square" size="md" />
			</Button>
		{:else}
			<Button
				variant="default"
				size="icon-sm"
				onclick={send}
				disabled={!canSend}
				aria-label="Send message"
			>
				<Icon name="send" size="md" />
			</Button>
		{/if}
	</HStack>
	<div class="mt-1 text-center">
		<Caption block>
			<Kbd>Enter</Kbd> to send, <Kbd>Shift+Enter</Kbd> for newline
		</Caption>
	</div>
</div>
