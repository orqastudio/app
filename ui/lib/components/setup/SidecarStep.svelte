<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { extractErrorMessage } from "$lib/ipc/invoke";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import CheckCircleIcon from "@lucide/svelte/icons/circle-check";
	import CpuIcon from "@lucide/svelte/icons/cpu";
	import { setupStore } from "$lib/stores/setup.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";

	interface Props {
		onComplete: () => void;
	}

	const { onComplete }: Props = $props();

	let starting = $state(true);

	async function start() {
		starting = true;
		setupStore.error = null;

		try {
			await settingsStore.refreshSidecarStatus();

			if (settingsStore.sidecarConnected) {
				setupStore.sidecarStarted = true;
				starting = false;
				setTimeout(onComplete, 1000);
				return;
			}

			await settingsStore.restartSidecar();
			await settingsStore.refreshSidecarStatus();

			if (settingsStore.sidecarConnected) {
				setupStore.sidecarStarted = true;
				starting = false;
				setTimeout(onComplete, 1000);
			} else {
				setupStore.error = settingsStore.sidecarStatus.error_message ?? "Sidecar failed to start";
				starting = false;
			}
		} catch (err) {
			setupStore.error = extractErrorMessage(err);
			starting = false;
		}
	}

	$effect(() => {
		start();
	});
</script>

<div class="space-y-4 text-center">
	<CpuIcon class="mx-auto h-10 w-10 text-muted-foreground" />
	<h3 class="text-lg font-semibold">Sidecar Process</h3>
	<p class="text-sm text-muted-foreground">Starting the Agent SDK sidecar</p>

	{#if starting}
		<LoadingSpinner size="md" />
		<p class="text-xs text-muted-foreground">Starting sidecar...</p>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={start} />
	{:else if setupStore.sidecarStarted}
		<div class="space-y-2">
			<CheckCircleIcon class="mx-auto h-8 w-8 text-green-500" />
			<p class="text-sm font-medium text-green-600 dark:text-green-400">Sidecar connected</p>
			{#if settingsStore.sidecarStatus.pid}
				<p class="text-xs text-muted-foreground">PID: {settingsStore.sidecarStatus.pid}</p>
			{/if}
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-amber-600 dark:text-amber-400">Sidecar not running</p>
			<Button variant="outline" onclick={start}>Retry</Button>
		</div>
	{/if}
</div>
