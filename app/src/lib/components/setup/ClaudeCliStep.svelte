<script lang="ts">
	import { Icon, Button, Heading, Text, Caption, Stack, HStack, Link } from "@orqastudio/svelte-components/pure";
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
		await setupStore.checkCli();
		checking = false;

		if (setupStore.cliInfo?.installed) {
			setTimeout(onComplete, 1000);
		}
	}

	$effect(() => {
		check();
	});
</script>

<Stack gap={4} align="center">
	<Icon name="terminal" size="xl" />
	<Heading level={3}>Claude CLI</Heading>
	<Text tone="muted">Checking for Claude Code CLI installation</Text>

	{#if checking}
		<LoadingSpinner size="md" />
		<Caption tone="muted">Detecting Claude CLI...</Caption>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.cliInfo?.installed}
		<Stack gap={2} align="center">
			<Icon name="circle-check" size="xl" />
			<Text tone="success" variant="body-strong">Claude CLI found</Text>
			{#if setupStore.cliInfo.version}
				<Caption tone="muted">Version: {setupStore.cliInfo.version}</Caption>
			{/if}
			{#if setupStore.cliInfo.path}
				<Caption variant="caption-mono">{setupStore.cliInfo.path}</Caption>
			{/if}
		</Stack>
	{:else}
		<Stack gap={3} align="center">
			<Text tone="warning">Claude CLI not found</Text>
			<Caption tone="muted">
				Install Claude Code to continue. Visit
				<Link href="https://docs.anthropic.com/en/docs/claude-code">docs.anthropic.com</Link>
				for installation instructions.
			</Caption>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</Stack>
	{/if}
</Stack>
