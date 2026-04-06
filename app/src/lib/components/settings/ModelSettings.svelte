<script lang="ts">
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		FormGroup,
	} from "@orqastudio/svelte-components/pure";
	import { SelectMenu } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import type { DefaultModel } from "@orqastudio/sdk";
	import { CLAUDE_MODEL_OPTIONS } from "$lib/components/conversation/model-options";

	const { settingsStore } = getStores();

	const modelOptions = CLAUDE_MODEL_OPTIONS as {
		value: DefaultModel;
		label: string;
		description: string;
	}[];

	/**
	 * Applies the selected model to the settings store.
	 * @param value - The model identifier to set as the default.
	 */
	function handleModelChange(value: string): void {
		settingsStore.setDefaultModel(value as DefaultModel);
	}
</script>

<CardRoot>
	<CardHeader>
		<CardTitle>Model</CardTitle>
		<CardDescription>Select the default Claude model for new sessions</CardDescription>
	</CardHeader>
	<CardContent>
		<FormGroup
			label="Default Model"
			description={modelOptions.find((o) => o.value === settingsStore.defaultModel)?.description ??
				""}
		>
			<SelectMenu
				items={modelOptions}
				selected={settingsStore.defaultModel}
				onSelect={handleModelChange}
				triggerLabel={modelOptions.find((o) => o.value === settingsStore.defaultModel)?.label ??
					"Auto"}
				triggerSize="default"
				align="start"
			/>
		</FormGroup>
	</CardContent>
</CardRoot>
