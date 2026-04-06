<!-- Renders a horizontal pipeline progress indicator for artifact lifecycle stages.
     Delegates all rendering to the PipelineStepper library component so that no
     raw HTML resides in app code. -->
<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import { PipelineStepper as PipelineStepperLib } from "@orqastudio/svelte-components/pure";

	const { artifactGraphSDK, projectStore } = getStores();

	interface Stage {
		key: string;
		label: string;
	}

	let {
		stages,
		status,
		path = "",
	}: {
		stages: Stage[];
		status: string;
		/** Relative path from project root — required for status transitions. */
		path?: string;
	} = $props();

	/**
	 * Keys reachable from the current status — driven by the `transitions` array
	 * on the matching status definition in project config.
	 */
	const reachableKeys = $derived.by((): string[] => {
		const statusKey = status?.toLowerCase();
		if (!statusKey) return [];
		const def = projectStore.projectSettings?.statuses?.find((s) => s.key === statusKey);
		return def?.transitions ?? [];
	});

	let transitioning = $state(false);

	/**
	 * Trigger a status transition to the given target key via the artifact graph SDK.
	 * @param targetKey - The status key to transition the current artifact to.
	 */
	async function handleTransition(targetKey: string) {
		if (!path || transitioning) return;
		transitioning = true;
		try {
			await artifactGraphSDK.updateField(path, "status", targetKey);
		} finally {
			transitioning = false;
		}
	}
</script>

<PipelineStepperLib
	{stages}
	{status}
	{reachableKeys}
	{transitioning}
	onTransition={handleTransition}
/>
