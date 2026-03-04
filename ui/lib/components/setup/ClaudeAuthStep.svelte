<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import CheckCircleIcon from "@lucide/svelte/icons/circle-check";
	import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
	import { setupStore } from "$lib/stores/setup.svelte";

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
	<ShieldCheckIcon class="mx-auto h-10 w-10 text-muted-foreground" />
	<h3 class="text-lg font-semibold">Authentication</h3>
	<p class="text-sm text-muted-foreground">Verifying Claude CLI authentication</p>

	{#if checking}
		<LoadingSpinner size="md" />
		<p class="text-xs text-muted-foreground">Checking authentication...</p>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.cliInfo?.authenticated}
		<div class="space-y-2">
			<CheckCircleIcon class="mx-auto h-8 w-8 text-green-500" />
			<p class="text-sm font-medium text-green-600 dark:text-green-400">Authenticated</p>
			{#if setupStore.cliInfo.subscription_type}
				<p class="text-xs text-muted-foreground">
					Plan: {setupStore.cliInfo.subscription_type}
				</p>
			{/if}
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-amber-600 dark:text-amber-400">Not authenticated</p>
			<p class="text-xs text-muted-foreground">
				Run <code class="rounded bg-muted px-1 py-0.5">claude</code> in your terminal and
				follow the login prompts to authenticate.
			</p>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</div>
	{/if}
</div>
