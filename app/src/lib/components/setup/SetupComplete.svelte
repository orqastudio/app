<script lang="ts">
	import {
		Icon,
		Button,
		Heading,
		Caption,
		Text,
		Stack,
		HStack,
	} from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { setupStore } = getStores();

	interface Props {
		onComplete: () => void;
	}

	const { onComplete }: Props = $props();

	let completing = $state(false);

	/**
	 *
	 */
	async function handleComplete() {
		completing = true;
		await setupStore.completeSetup();
		if (setupStore.setupComplete) {
			onComplete();
		}
		completing = false;
	}
</script>

<Stack gap={6} align="center">
	<Icon name="rocket" size="xl" />
	<Heading level={3}>All Set</Heading>
	<Caption tone="muted">OrqaStudio is configured and ready to use.</Caption>

	<Stack gap={2}>
		<HStack gap={2}>
			<Icon name="circle-check" size="md" />
			<Text>Claude CLI installed</Text>
		</HStack>
		<HStack gap={2}>
			<Icon name="circle-check" size="md" />
			<Text>Authentication verified</Text>
		</HStack>
		<HStack gap={2}>
			<Icon name="circle-check" size="md" />
			<Text>Sidecar connected</Text>
		</HStack>
		<HStack gap={2}>
			<Icon name="circle-check" size="md" />
			<Text>Embedding model ready</Text>
		</HStack>
	</Stack>

	<Button onclick={handleComplete} disabled={completing}>
		{#if completing}
			Getting started...
		{:else}
			Get Started
		{/if}
	</Button>
</Stack>
