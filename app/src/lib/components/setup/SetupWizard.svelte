<script lang="ts">
	import {
		CardRoot,
		CardHeader,
		CardTitle,
		CardDescription,
		CardContent,
		Caption,
		HStack,
		Dot,
		Stack,
	} from "@orqastudio/svelte-components/pure";
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

	/**
	 *
	 */
	function handleStepComplete() {
		setupStore.nextStep();
	}

	/**
	 *
	 */
	function handleSetupComplete() {
		onComplete();
	}
</script>

<!-- Full-screen background with cover image -->
<div
	style="position: relative; display: flex; height: 100%; width: 100%; align-items: center; justify-content: center; overflow: hidden; background-image: url({setupBackground}); background-size: cover; background-position: center;"
>
	<!-- Backdrop overlay -->
	<div style="position: absolute; inset: 0; background: hsl(var(--background) / 0.7);"></div>

	<!-- Centered card container -->
	<div style="position: relative; z-index: 10; width: 100%; max-width: 32rem; padding: 0 1rem;">
		<CardRoot>
			<CardHeader>
				<div style="text-align: center;">
					<CardTitle>Welcome to OrqaStudio</CardTitle>
					<CardDescription>
						Let's make sure everything is set up for managed agentic development.
					</CardDescription>

					<!-- Step indicator dots; gap provides spacing between description and dots -->
					<Stack gap={3}>
						<HStack gap={2} justify="center">
							{#each Array.from({ length: setupStore.totalSteps }, (_, idx) => idx) as i (i)}
								<Dot color={i <= setupStore.currentStep ? "primary" : "muted"} size="md" />
							{/each}
						</HStack>
					</Stack>
					<Caption tone="muted"
						>Step {setupStore.currentStep + 1} of {setupStore.totalSteps}</Caption
					>
				</div>
			</CardHeader>

			<CardContent>
				<div style="min-height: 12.5rem;">
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
				</div>
			</CardContent>
		</CardRoot>
	</div>
</div>
