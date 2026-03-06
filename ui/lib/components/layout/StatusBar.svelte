<script lang="ts">
	import { LoaderCircle } from "lucide-svelte";
	import BrainIcon from "@lucide/svelte/icons/brain";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { sessionStore } from "$lib/stores/session.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import logoPulse from "$lib/assets/logo-pulse.svg";

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
	const session = $derived(sessionStore.activeSession);
	const hasTokens = $derived(
		session !== null &&
			(session.total_input_tokens > 0 || session.total_output_tokens > 0),
	);

	function formatTokens(count: number): string {
		if (count >= 1_000_000) {
			return `${(count / 1_000_000).toFixed(1)}M`;
		}
		if (count >= 1000) {
			return `${(count / 1000).toFixed(1)}k`;
		}
		return String(count);
	}

	function openModelSettings() {
		settingsStore.setActiveSection("model");
		navigationStore.setActivity("settings");
	}
</script>

<div
	class="flex h-8 items-center border-t border-border bg-muted/30 px-4 pb-1 text-xs text-muted-foreground"
>
	<!-- Left: Connection status -->
	<div class="flex items-center gap-1.5">
		<span class="inline-block h-2 w-2 rounded-full {statusColor}"></span>
		<span>{settingsStore.sidecarStateLabel}</span>
	</div>

	<!-- Center: Project path -->
	<div class="flex min-w-0 flex-1 items-center justify-center gap-2">
		<span class="truncate">{projectPath}</span>
	</div>

	<!-- Startup task indicator -->
	{#if settingsStore.activeStartupTask}
		<div class="mr-4 flex items-center gap-1.5">
			<LoaderCircle class="h-3 w-3 animate-spin text-muted-foreground" />
			<span>
				{settingsStore.activeStartupTask.label}{settingsStore.activeStartupTask.detail
					? `: ${settingsStore.activeStartupTask.detail}`
					: "..."}
			</span>
		</div>
	{/if}

	<!-- Right: Token usage + Model + Branding -->
	<div class="flex items-center gap-3">
		{#if hasTokens && session}
			<span class="tabular-nums text-muted-foreground/70">
				{formatTokens(session.total_input_tokens)}↑ {formatTokens(session.total_output_tokens)}↓
			</span>
		{/if}

		<button
			class="flex items-center gap-1 rounded px-1.5 py-0.5 transition-colors hover:bg-accent hover:text-accent-foreground"
			onclick={openModelSettings}
			title="Change model"
		>
			<BrainIcon class="h-3 w-3" />
			<span>{settingsStore.modelDisplayName}</span>
		</button>

		<span class="text-muted-foreground/30">|</span>

		<div class="flex items-center gap-1.5">
			<img src={logoPulse} class="h-3.5 w-3.5" alt="" />
			<span>Orqa Studio</span>
		</div>
	</div>
</div>
