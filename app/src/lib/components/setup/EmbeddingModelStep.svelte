<script lang="ts">
	import { Icon, Button, Heading, Text, Caption, Stack } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import { EMBEDDING_MODEL_NAME } from "$lib/config/search-config";

	const { setupStore, settingsStore } = getStores();

	interface Props {
		onComplete: () => void;
	}

	const { onComplete }: Props = $props();

	let checking = $state(true);

	const downloadProgress = $derived(() => {
		const task = settingsStore.startupStatus?.tasks.find((t) => t.id === "embedding_model");
		if (!task || task.status !== "in_progress") return null;
		return task.detail;
	});

	/**
	 *
	 */
	async function check() {
		checking = true;
		setupStore.error = null;
		await setupStore.checkEmbeddingModel();
		checking = false;

		if (setupStore.embeddingStatus?.status === "complete") {
			setTimeout(onComplete, 1000);
		}
	}

	/**
	 *
	 */
	async function waitForDownload() {
		// The embedding model download is started during app init in lib.rs.
		// Poll the startup tracker until it completes.
		const poll = setInterval(async () => {
			await settingsStore.refreshSidecarStatus();
			const task = settingsStore.startupStatus?.tasks.find((t) => t.id === "embedding_model");
			if (task?.status === "done") {
				clearInterval(poll);
				setupStore.embeddingStatus = {
					id: "embedding_model",
					label: "Embedding Model",
					status: "complete",
					detail: `${EMBEDDING_MODEL_NAME} ready`,
				};
				checking = false;
				setTimeout(onComplete, 1000);
			} else if (task?.status === "error") {
				clearInterval(poll);
				setupStore.error = task.detail ?? "Embedding model download failed";
				checking = false;
			}
		}, 1000);
	}

	$effect(() => {
		check().then(() => {
			if (setupStore.embeddingStatus?.status !== "complete") {
				checking = true;
				waitForDownload();
			}
		});
	});
</script>

<Stack gap={4} align="center">
	<Icon name="brain" size="xl" />
	<Heading level={3}>Embedding Model</Heading>
	<Text tone="muted">Preparing semantic search model</Text>

	{#if checking}
		<LoadingSpinner size="md" />
		{#if downloadProgress()}
			<Caption tone="muted">Downloading: {downloadProgress()}</Caption>
		{:else}
			<Caption tone="muted">Checking model...</Caption>
		{/if}
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.embeddingStatus?.status === "complete"}
		<Stack gap={2} align="center">
			<Icon name="circle-check" size="xl" />
			<Text tone="success" variant="body-strong">Model ready</Text>
			<Caption tone="muted">{EMBEDDING_MODEL_NAME}</Caption>
		</Stack>
	{:else}
		<Stack gap={3} align="center">
			<Text tone="warning">Model not available</Text>
			<Caption tone="muted">
				The embedding model will be downloaded automatically when the app starts.
			</Caption>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</Stack>
	{/if}
</Stack>
