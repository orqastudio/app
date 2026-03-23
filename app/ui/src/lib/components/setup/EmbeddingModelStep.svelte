<script lang="ts">
	import { Icon, Button } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

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
					detail: "bge-small-en-v1.5 ready",
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

<div class="space-y-4 text-center">
	<Icon name="brain" size="xl" />
	<h3 class="text-lg font-semibold">Embedding Model</h3>
	<p class="text-sm text-muted-foreground">Preparing semantic search model</p>

	{#if checking}
		<LoadingSpinner size="md" />
		{#if downloadProgress()}
			<p class="text-xs text-muted-foreground">Downloading: {downloadProgress()}</p>
		{:else}
			<p class="text-xs text-muted-foreground">Checking model...</p>
		{/if}
	{:else if setupStore.error}
		<ErrorDisplay message={setupStore.error} onRetry={check} />
	{:else if setupStore.embeddingStatus?.status === "complete"}
		<div class="space-y-2">
			<Icon name="circle-check" size="xl" />
			<p class="text-sm font-medium text-success">Model ready</p>
			<p class="text-xs text-muted-foreground">bge-small-en-v1.5</p>
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-warning">Model not available</p>
			<p class="text-xs text-muted-foreground">
				The embedding model will be downloaded automatically when the app starts.
			</p>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</div>
	{/if}
</div>
