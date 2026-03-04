<script lang="ts">
	import { LoaderCircle } from "lucide-svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { projectStore } from "$lib/stores/project.svelte";

	const statusColor = $derived.by(() => {
		switch (settingsStore.sidecarStatus.state) {
			case "connected":
				return "bg-green-500";
			case "starting":
				return "bg-yellow-500";
			case "error":
				return "bg-red-500";
			case "stopped":
			case "not_started":
			default:
				return "bg-muted-foreground";
		}
	});

	const projectPath = $derived(projectStore.activeProject?.path ?? "No project");
</script>

<div
	class="flex h-8 items-center border-t border-border bg-muted/30 px-4 pb-1 text-xs text-muted-foreground"
>
	<!-- Left: Active model -->
	<div class="flex items-center">
		<span>Model: {settingsStore.modelDisplayName}</span>
	</div>

	<!-- Center: Project path -->
	<div class="flex min-w-0 flex-1 items-center justify-center">
		<span class="truncate">{projectPath}</span>
	</div>

	<!-- Startup task indicator (between center and right) -->
	{#if settingsStore.activeStartupTask}
		<div class="mr-4 flex items-center gap-1.5">
			<LoaderCircle class="h-3 w-3 animate-spin text-muted-foreground" />
			<span>
				{settingsStore.activeStartupTask.label}{settingsStore.activeStartupTask.detail ? `: ${settingsStore.activeStartupTask.detail}` : '...'}
			</span>
		</div>
	{/if}

	<!-- Right: Sidecar connection status -->
	<div class="flex items-center gap-1.5">
		<span>{settingsStore.sidecarStateLabel}</span>
		<span class="inline-block h-2 w-2 rounded-full {statusColor}"></span>
	</div>
</div>
