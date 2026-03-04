<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import CheckCircleIcon from "@lucide/svelte/icons/circle-check";
	import BrainIcon from "@lucide/svelte/icons/brain";
	import { setupStore } from "$lib/stores/setup.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";

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
	<BrainIcon class="mx-auto h-10 w-10 text-muted-foreground" />
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
			<CheckCircleIcon class="mx-auto h-8 w-8 text-green-500" />
			<p class="text-sm font-medium text-green-600 dark:text-green-400">Model ready</p>
			<p class="text-xs text-muted-foreground">bge-small-en-v1.5</p>
		</div>
	{:else}
		<div class="space-y-3">
			<p class="text-sm text-amber-600 dark:text-amber-400">Model not available</p>
			<p class="text-xs text-muted-foreground">
				The embedding model will be downloaded automatically when the app starts.
			</p>
			<Button variant="outline" onclick={check}>Check Again</Button>
		</div>
	{/if}
</div>
