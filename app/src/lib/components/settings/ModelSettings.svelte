<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import type { DefaultModel } from "@orqastudio/sdk";

	const { settingsStore } = getStores();

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

<CardRoot>
	<CardHeader>
		<CardTitle>Model</CardTitle>
		<CardDescription>Select the default Claude model for new sessions</CardDescription>
	</CardHeader>
	<CardContent class="space-y-4">
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
	</CardContent>
</CardRoot>
