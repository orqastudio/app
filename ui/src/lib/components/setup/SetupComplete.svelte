<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import CheckCircleIcon from "@lucide/svelte/icons/circle-check";
	import RocketIcon from "@lucide/svelte/icons/rocket";
	import { setupStore } from "$lib/stores/setup.svelte";

	interface Props {
		onComplete: () => void;
	}

	const { onComplete }: Props = $props();

	let completing = $state(false);

	async function handleComplete() {
		completing = true;
		await setupStore.completeSetup();
		if (setupStore.setupComplete) {
			onComplete();
		}
		completing = false;
	}
</script>

<div class="space-y-6 text-center">
	<RocketIcon class="mx-auto h-12 w-12 text-primary" />
	<h3 class="text-lg font-semibold">All Set</h3>
	<p class="text-sm text-muted-foreground">OrqaStudio is configured and ready to use.</p>

	<div class="mx-auto max-w-xs space-y-2 text-left">
		<div class="flex items-center gap-2 text-sm">
			<CheckCircleIcon class="h-4 w-4 text-success" />
			<span>Claude CLI installed</span>
		</div>
		<div class="flex items-center gap-2 text-sm">
			<CheckCircleIcon class="h-4 w-4 text-success" />
			<span>Authentication verified</span>
		</div>
		<div class="flex items-center gap-2 text-sm">
			<CheckCircleIcon class="h-4 w-4 text-success" />
			<span>Sidecar connected</span>
		</div>
		<div class="flex items-center gap-2 text-sm">
			<CheckCircleIcon class="h-4 w-4 text-success" />
			<span>Embedding model ready</span>
		</div>
	</div>

	<Button onclick={handleComplete} disabled={completing}>
		{#if completing}
			Getting started...
		{:else}
			Get Started
		{/if}
	</Button>
</div>
