<script lang="ts">
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import { CLAUDE_MODELS } from "./model-options";

	function getModelLabel(modelId: string): string {
		const found = CLAUDE_MODELS.find((m) => m.value === modelId);
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
	items={CLAUDE_MODELS}
	selected={value}
	onSelect={onchange}
	{triggerLabel}
	align="start"
/>
