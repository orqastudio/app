<script lang="ts">
	import { Icon, Button, Heading, Text, Caption, Code, Stack, HStack } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { setupStore } = getStores();

	interface Props {
		onComplete: () => void;
	}

	const { onComplete }: Props = $props();

	let checking = $state(true);

	async function check() {
		checking = true;
		setupStore.error = null;
		await setupStore.checkAuth();
		checking = false;

		if (setupStore.cliInfo?.authenticated) {
			setTimeout(onComplete, 1000);
		}
	}

	$effect(() => {
		check();
	});
</script>

<Stack gap={4} align="center">
	<Icon name="shield-check" size="xl" />
	<Heading level={3}>Authentication</Heading>
	<Text tone="muted">Verifying Claude CLI authentication</Text>

	{#if checking}
		<LoadingSpinner size="md" />
		<Caption tone="muted">Checking authentication...</Caption>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.cliInfo?.authenticated}
		<Stack gap={2} align="center">
			<Icon name="circle-check" size="xl" />
			<Text tone="success" variant="body-strong">Authenticated</Text>
			{#if setupStore.cliInfo.subscription_type}
				<Caption tone="muted">Plan: {setupStore.cliInfo.subscription_type}</Caption>
			{/if}
		</Stack>
	{:else}
		<Stack gap={3} align="center">
			<Text tone="warning">Not authenticated</Text>
			<Caption tone="muted">
				Run <Code>claude</Code> in your terminal and
				follow the login prompts to authenticate.
			</Caption>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</Stack>
	{/if}
</Stack>
