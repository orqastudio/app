<script lang="ts">
	import { Icon, Button, Heading } from "@orqastudio/svelte-components/pure";
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

<div class="flex flex-col items-center gap-6 text-center">
	<Icon name="rocket" size="xl" />
	<Heading level={3}>All Set</Heading>
	<span class="text-sm text-muted-foreground">OrqaStudio is configured and ready to use.</span>

	<div class="mx-auto flex max-w-xs flex-col gap-2 text-left">
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
