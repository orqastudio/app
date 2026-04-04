<script lang="ts">
	import { Icon, Button, Heading, Text } from "@orqastudio/svelte-components/pure";
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

	async function check() {
		checking = true;
		setupStore.error = null;
		await setupStore.checkEmbeddingModel();
		checking = false;

		if (setupStore.embeddingStatus?.status === "complete") {
			setTimeout(onComplete, 1000);
		}
	}

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

<div class="flex flex-col items-center gap-4 text-center">
	<Icon name="brain" size="xl" />
	<Heading level={3}>Embedding Model</Heading>
	<Text tone="muted">Preparing semantic search model</Text>

	{#if checking}
		<LoadingSpinner size="md" />
		{#if downloadProgress()}
			<span class="text-xs text-muted-foreground">Downloading: {downloadProgress()}</span>
		{:else}
			<span class="text-xs text-muted-foreground">Checking model...</span>
		{/if}
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.embeddingStatus?.status === "complete"}
		<div class="flex flex-col items-center gap-2">
			<Icon name="circle-check" size="xl" />
			<p class="text-sm font-medium text-success">Model ready</p>
			<span class="text-xs text-muted-foreground">{EMBEDDING_MODEL_NAME}</span>
		</div>
	{:else}
		<div class="flex flex-col items-center gap-3">
			<p class="text-sm text-warning">Model not available</p>
			<span class="text-xs text-muted-foreground">
				The embedding model will be downloaded automatically when the app starts.
			</span>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</div>
	{/if}
</div>
