<script lang="ts">
	import { CardRoot, CardHeader, CardTitle, CardDescription, CardContent } from "@orqastudio/svelte-components/pure";
	import ClaudeCliStep from "./ClaudeCliStep.svelte";
	import ClaudeAuthStep from "./ClaudeAuthStep.svelte";
	import SidecarStep from "./SidecarStep.svelte";
	import EmbeddingModelStep from "./EmbeddingModelStep.svelte";
	import SetupComplete from "./SetupComplete.svelte";
	import setupBackground from "$lib/assets/setup-background.png";
	import { getStores } from "@orqastudio/sdk";

	const { setupStore } = getStores();

	interface Props {
		onComplete: () => void;
	}

	const { onComplete }: Props = $props();

	function handleStepComplete() {
		setupStore.nextStep();
	}

	function handleSetupComplete() {
		onComplete();
	}
</script>

<div
	class="relative flex h-full w-full items-center justify-center overflow-hidden"
	style="background-image: url({setupBackground}); background-size: cover; background-position: center;"
>
	<div class="absolute inset-0 bg-background/70"></div>

	<div class="relative z-10 w-full max-w-lg px-4">
		<CardRoot>
			<CardHeader class="text-center">
				<CardTitle class="text-xl">Welcome to OrqaStudio</CardTitle>
				<CardDescription>
					Let's make sure everything is set up for managed agentic development.
				</CardDescription>

				<!-- Step indicator -->
				<div class="flex items-center justify-center gap-2 pt-3">
					{#each Array.from({ length: setupStore.totalSteps }, (_, idx) => idx) as i (i)}
						<div
							class="h-2 w-2 rounded-full transition-colors {i < setupStore.currentStep
								? 'bg-primary'
								: i === setupStore.currentStep
									? 'bg-primary'
									: 'bg-muted'}"
						></div>
					{/each}
				</div>
				<p class="text-xs text-muted-foreground">
					Step {setupStore.currentStep + 1} of {setupStore.totalSteps}
				</p>
			</CardHeader>

			<CardContent class="min-h-[200px]">
				{#if setupStore.stepId === "claude_cli"}
					<ClaudeCliStep onComplete={handleStepComplete} />
				{:else if setupStore.stepId === "claude_auth"}
					<ClaudeAuthStep onComplete={handleStepComplete} />
				{:else if setupStore.stepId === "sidecar"}
					<SidecarStep onComplete={handleStepComplete} />
				{:else if setupStore.stepId === "embedding_model"}
					<EmbeddingModelStep onComplete={handleStepComplete} />
				{:else if setupStore.stepId === "complete"}
					<SetupComplete onComplete={handleSetupComplete} />
				{/if}
			</CardContent>
		</CardRoot>
	</div>
</div>
