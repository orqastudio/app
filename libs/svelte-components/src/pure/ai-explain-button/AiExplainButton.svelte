<!-- AiExplainButton — a button that builds an AI explanation prompt from event context
     and emits it to the consumer via the onexplain callback. Disabled when no event
     is selected or when the event has no message. -->
<script lang="ts">
	import { Button } from "../button/index.js";
	import { Icon } from "../icon/index.js";
	import { buildExplainPrompt, type ExplainEvent } from "./explain-prompt.js";

	/** Props for AiExplainButton. */
	export interface AiExplainButtonProps {
		/** The log event to explain, or null when nothing is selected. */
		event: ExplainEvent | null;
		/** When true, forces the button into the disabled state regardless of event presence. */
		disabled?: boolean;
		/** Called with the built prompt string when the user clicks the button. */
		onexplain?: (prompt: string) => void;
	}

	let { event, disabled = false, onexplain }: AiExplainButtonProps = $props();

	/**
	 * Determines whether the button should be disabled.
	 * Disabled when event is null or has no message, or when disabled prop is set.
	 */
	const isDisabled = $derived(disabled || event === null || !event.message);

	/**
	 * Handle click: build the explain prompt from the current event and fire onexplain.
	 * No-ops when event is null (should not occur given disabled state, but guards anyway).
	 */
	function handleClick(): void {
		if (!event || !event.message) return;
		const prompt = buildExplainPrompt(event);
		onexplain?.(prompt);
	}
</script>

<Button
	variant="ghost"
	size="sm"
	disabled={isDisabled}
	onclick={handleClick}
	aria-label="Explain with AI"
>
	<Icon name="brain" size="sm" />
	Explain with AI
</Button>
