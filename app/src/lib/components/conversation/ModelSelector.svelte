<script lang="ts">
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { CLAUDE_MODEL_OPTIONS } from "./model-options";

	/**
	 *
	 * @param modelId
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
