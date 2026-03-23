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

<div class="space-y-4 text-center">
	<Icon name="shield-check" size="xl" />
	<h3 class="text-lg font-semibold">Authentication</h3>
	<p class="text-sm text-muted-foreground">Verifying Claude CLI authentication</p>

	{#if checking}
		<LoadingSpinner size="md" />
		<p class="text-xs text-muted-foreground">Checking authentication...</p>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.cliInfo?.authenticated}
		<div class="space-y-2">
			<Icon name="circle-check" size="xl" />
			<p class="text-sm font-medium text-success">Authenticated</p>
			{#if setupStore.cliInfo.subscription_type}
				<p class="text-xs text-muted-foreground">
					Plan: {setupStore.cliInfo.subscription_type}
				</p>
			{/if}
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-warning">Not authenticated</p>
			<p class="text-xs text-muted-foreground">
				Run <code class="rounded bg-muted px-1 py-0.5">claude</code> in your terminal and
				follow the login prompts to authenticate.
			</p>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</div>
	{/if}
</div>
