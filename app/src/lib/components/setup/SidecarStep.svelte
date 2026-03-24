<script lang="ts">
	import { Icon, Button } from "@orqastudio/svelte-components/pure";
	import { extractErrorMessage, logger } from "@orqastudio/sdk";

	const log = logger("setup");
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { setupStore, settingsStore } = getStores();

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
			log.error("Sidecar start failed", { err });
			setupStore.error = extractErrorMessage(err);
			starting = false;
		}
	}

	$effect(() => {
		start();
	});
</script>

<div class="space-y-4 text-center">
	<Icon name="cpu" size="xl" />
	<h3 class="text-lg font-semibold">Sidecar Process</h3>
	<p class="text-sm text-muted-foreground">Starting the Agent SDK sidecar</p>

	{#if starting}
		<LoadingSpinner size="md" />
		<p class="text-xs text-muted-foreground">Starting sidecar...</p>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={start} />
	{:else if setupStore.sidecarStarted}
		<div class="space-y-2">
			<Icon name="circle-check" size="xl" />
			<p class="text-sm font-medium text-success">Sidecar connected</p>
			{#if settingsStore.sidecarStatus.pid}
				<p class="text-xs text-muted-foreground">PID: {settingsStore.sidecarStatus.pid}</p>
			{/if}
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-warning">Sidecar not running</p>
			<Button variant="outline" onclick={start}>Retry</Button>
		</div>
	{/if}
</div>
