<script lang="ts">
	import { Icon, Button, Heading, Text, Caption, Stack } from "@orqastudio/svelte-components/pure";
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

	/**
	 *
	 */
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

<Stack gap={4} align="center">
	<Icon name="cpu" size="xl" />
	<Heading level={3}>Sidecar Process</Heading>
	<Text tone="muted">Starting the Agent SDK sidecar</Text>

	{#if starting}
		<LoadingSpinner size="md" />
		<Caption tone="muted">Starting sidecar...</Caption>
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={start} />
	{:else if setupStore.sidecarStarted}
		<Stack gap={2} align="center">
			<Icon name="circle-check" size="xl" />
			<Text tone="success" variant="body-strong">Sidecar connected</Text>
			{#if settingsStore.sidecarStatus.pid}
				<Caption tone="muted">PID: {settingsStore.sidecarStatus.pid}</Caption>
			{/if}
		</Stack>
	{:else}
		<Stack gap={3} align="center">
			<Text tone="warning">Sidecar not running</Text>
			<Button variant="outline" onclick={start}>Retry</Button>
		</Stack>
	{/if}
</Stack>
