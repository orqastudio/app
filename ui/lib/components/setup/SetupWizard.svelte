<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import ClaudeCliStep from "./ClaudeCliStep.svelte";
	import ClaudeAuthStep from "./ClaudeAuthStep.svelte";
	import SidecarStep from "./SidecarStep.svelte";
	import EmbeddingModelStep from "./EmbeddingModelStep.svelte";
	import SetupComplete from "./SetupComplete.svelte";
	import setupBackground from "$lib/assets/setup-background.png";
	import { setupStore } from "$lib/stores/setup.svelte";

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
		<Card.Root>
			<Card.Header class="text-center">
				<Card.Title class="text-xl">Welcome to Orqa Studio</Card.Title>
				<Card.Description>
					Let's make sure everything is set up for managed agentic development.
				</Card.Description>

				<!-- Step indicator -->
				<div class="flex items-center justify-center gap-2 pt-3">
					{#each Array(setupStore.totalSteps) as _, i}
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
			</Card.Header>

			<Card.Content class="min-h-[200px]">
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
			</Card.Content>
		</Card.Root>
	</div>
</div>
