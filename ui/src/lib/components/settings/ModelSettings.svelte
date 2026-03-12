<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import { settingsStore, type DefaultModel } from "$lib/stores/settings.svelte";

	const modelOptions: { value: DefaultModel; label: string; description: string }[] = [
		{ value: "auto", label: "Auto (recommended)", description: "Automatically selects the best model" },
		{ value: "claude-opus-4-6", label: "Opus", description: "Most capable, slower" },
		{ value: "claude-sonnet-4-6", label: "Sonnet", description: "Balanced performance" },
		{ value: "claude-haiku-4-5", label: "Haiku", description: "Fastest responses" },
	];

	function handleModelChange(value: string): void {
		settingsStore.setDefaultModel(value as DefaultModel);
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Model</Card.Title>
		<Card.Description>Select the default Claude model for new sessions</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-4">
		<div>
			<span class="text-sm font-medium">Default Model</span>
			<div class="mt-1">
				<SelectMenu
					items={modelOptions}
					selected={settingsStore.defaultModel}
					onSelect={handleModelChange}
					triggerLabel={modelOptions.find((o) => o.value === settingsStore.defaultModel)?.label ?? "Auto"}
					triggerSize="default"
					align="start"
				/>
			</div>
			<p class="mt-1.5 text-xs text-muted-foreground">
				{modelOptions.find((o) => o.value === settingsStore.defaultModel)?.description ?? ""}
			</p>
		</div>
	</Card.Content>
</Card.Root>
