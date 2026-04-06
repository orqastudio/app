<!-- Textarea with send/stop button. Auto-grows up to 200px. Submits on Enter. -->
<script lang="ts">
	import { Icon } from "../icon/index.js";
	import { Button } from "../button/index.js";
	import { Textarea } from "../textarea/index.js";
	import { HStack, Center } from "../layout/index.js";
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

	/**
	 * Submit the message on Enter (without Shift), preventing the default newline insertion.
	 * @param event - The keyboard event from the textarea
	 */
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			send();
		}
	}

	/** Trim the current value, call the onsubmit callback, then clear the field and reset its height. Does nothing when streaming, disabled, or the input is blank. */
	function send() {
		const content = value.trim();
		if (content.length === 0 || isStreaming || disabled) return;
		onsubmit(content);
		value = "";
		resetTextareaHeight();
	}

	/** Trigger auto-resize whenever the textarea content changes. */
	function handleInput() {
		autoResize();
	}

	/** Expand the textarea to fit its content up to a 200px maximum height. */
	function autoResize() {
		if (!textareaRef) return;
		textareaRef.style.height = "auto";
		const maxHeight = 200;
		textareaRef.style.height = `${Math.min(textareaRef.scrollHeight, maxHeight)}px`;
	}

	/** Reset the textarea height back to auto after a message is sent. */
	function resetTextareaHeight() {
		if (!textareaRef) return;
		textareaRef.style.height = "auto";
	}
</script>

<div class="border-border bg-background border-t p-3">
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
	<Center>
		<Caption block>
			<Kbd>Enter</Kbd> to send, <Kbd>Shift+Enter</Kbd> for newline
		</Caption>
	</Center>
</div>
