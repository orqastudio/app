<script lang="ts">
	import { Icon, Button } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { setupStore } = getStores();

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
	<Icon name="rocket" size="xl" />
	<h3 class="text-lg font-semibold">All Set</h3>
	<p class="text-sm text-muted-foreground">OrqaStudio is configured and ready to use.</p>

	<div class="mx-auto max-w-xs space-y-2 text-left">
		<div class="flex items-center gap-2 text-sm">
			<Icon name="circle-check" size="md" />
			<span>Claude CLI installed</span>
		</div>
		<div class="flex items-center gap-2 text-sm">
			<Icon name="circle-check" size="md" />
			<span>Authentication verified</span>
		</div>
		<div class="flex items-center gap-2 text-sm">
			<Icon name="circle-check" size="md" />
			<span>Sidecar connected</span>
		</div>
		<div class="flex items-center gap-2 text-sm">
			<Icon name="circle-check" size="md" />
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
