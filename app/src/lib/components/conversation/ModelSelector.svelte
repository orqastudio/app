<script lang="ts">
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { CLAUDE_MODEL_OPTIONS } from "./model-options";

	/**
	 * Look up the display label for the given model ID, falling back to the raw ID if not found.
	 * @param modelId - The Claude model identifier (e.g. "claude-opus-4-5").
	 * @returns The human-readable label for the model, or the raw model ID if unrecognised.
	 */
	function getModelLabel(modelId: string): string {
		const found = CLAUDE_MODEL_OPTIONS.find((m) => m.value === modelId);
		return found ? found.label : modelId;
	}

	let {
		value,
		onchange,
	}: {
		value: string;
		onchange: (model: string) => void;
	} = $props();

	const triggerLabel = $derived(getModelLabel(value));
</script>

<SelectMenu
	items={CLAUDE_MODEL_OPTIONS}
	selected={value}
	onSelect={onchange}
	{triggerLabel}
	align="start"
/>
