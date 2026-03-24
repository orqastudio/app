<script lang="ts">
	import { Icon, Button } from "@orqastudio/svelte-components/pure";
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

<div class="space-y-4 text-center">
	<Icon name="terminal" size="xl" />
	<h3 class="text-lg font-semibold">Claude CLI</h3>
	<p class="text-sm text-muted-foreground">Checking for Claude Code CLI installation</p>

	{#if checking}
		<LoadingSpinner size="md" />
		<p class="text-xs text-muted-foreground">Detecting Claude CLI...</p>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.cliInfo?.installed}
		<div class="space-y-2">
			<Icon name="circle-check" size="xl" />
			<p class="text-sm font-medium text-success">Claude CLI found</p>
			{#if setupStore.cliInfo.version}
				<p class="text-xs text-muted-foreground">Version: {setupStore.cliInfo.version}</p>
			{/if}
			{#if setupStore.cliInfo.path}
				<p class="font-mono text-xs text-muted-foreground">{setupStore.cliInfo.path}</p>
			{/if}
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-warning">Claude CLI not found</p>
			<p class="text-xs text-muted-foreground">
				Install Claude Code to continue. Visit
				<a
					href="https://docs.anthropic.com/en/docs/claude-code"
					target="_blank"
					rel="noopener noreferrer"
					class="underline hover:text-foreground"
				>
					docs.anthropic.com
				</a>
				for installation instructions.
			</p>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</div>
	{/if}
</div>
