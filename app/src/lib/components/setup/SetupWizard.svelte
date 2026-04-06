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
		BackgroundImage,
		Box,
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

<!-- Full-screen background with cover image and centered card -->
<BackgroundImage src={setupBackground} overlay>
	<Box maxWidth="sm" width="full">
		<CardRoot>
			<CardHeader>
				<Stack gap={2} align="center">
					<CardTitle>Welcome to OrqaStudio</CardTitle>
					<CardDescription>
						Let's make sure everything is set up for managed agentic development.
					</CardDescription>

					<!-- Step indicator dots -->
					<HStack gap={2} justify="center">
						{#each Array.from({ length: setupStore.totalSteps }, (_, idx) => idx) as i (i)}
							<Dot color={i <= setupStore.currentStep ? "primary" : "muted"} size="md" />
						{/each}
					</HStack>
					<Caption tone="muted"
						>Step {setupStore.currentStep + 1} of {setupStore.totalSteps}</Caption
					>
				</Stack>
			</CardHeader>

			<CardContent>
				<!-- min-height prevents card from collapsing during step transitions -->
				<Box minHeight="step">
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
				</Box>
			</CardContent>
		</CardRoot>
	</Box>
</BackgroundImage>
