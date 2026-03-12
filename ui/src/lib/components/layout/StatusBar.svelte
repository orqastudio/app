<script lang="ts">
	import { LoaderCircle } from "lucide-svelte";
	import BrainIcon from "@lucide/svelte/icons/brain";
	import DatabaseIcon from "@lucide/svelte/icons/database";
	import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { sessionStore } from "$lib/stores/session.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import finMark from "$lib/assets/fin-mark.svg";

	const statusColor = $derived.by(() => {
		switch (settingsStore.sidecarStatus.state) {
			case "connected":
				return "bg-success";
			case "starting":
				return "bg-warning";
			case "error":
				return "bg-destructive";
			case "stopped":
			case "not_started":
			default:
				return "bg-muted-foreground";
		}
	});

	const session = $derived(sessionStore.activeSession);
	const hasTokens = $derived(
		session !== null &&
			(session.total_input_tokens > 0 || session.total_output_tokens > 0),
	);

	const artifactCount = $derived(artifactGraphSDK.graph.size);

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
	<!-- Left: Brand | Model -->
	<div class="flex items-center gap-3">
		<div class="flex items-center gap-1.5">
			<img src={finMark} class="h-3.5 w-3.5" alt="" />
			<span class="font-medium text-foreground/70">OrqaStudio</span>
		</div>

		<span class="h-3 w-px bg-border"></span>

		<Tooltip.Root>
			<Tooltip.Trigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="flex items-center gap-1 rounded px-1.5 py-0.5 transition-colors hover:bg-accent hover:text-accent-foreground"
						onclick={openModelSettings}
					>
						<BrainIcon class="h-3 w-3" />
						<span>{settingsStore.modelDisplayName}</span>
					</button>
				{/snippet}
			</Tooltip.Trigger>
			<Tooltip.Content side="top">
				<p>Change model</p>
			</Tooltip.Content>
		</Tooltip.Root>
	</div>

	<!-- Center: spacer -->
	<div class="flex-1"></div>

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

	<!-- Right: Tokens | Index | Sidecar status -->
	<div class="flex items-center gap-3">
		{#if hasTokens && session}
			<span class="tabular-nums text-muted-foreground/70">
				{formatTokens(session.total_input_tokens)}↑ {formatTokens(session.total_output_tokens)}↓
			</span>
			<span class="h-3 w-px bg-border"></span>
		{/if}

		<Tooltip.Root>
			<Tooltip.Trigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="flex items-center gap-1 rounded px-1.5 py-0.5 transition-colors hover:bg-accent hover:text-accent-foreground disabled:cursor-not-allowed disabled:opacity-50 {artifactGraphSDK.error ? 'text-destructive' : ''}"
						onclick={() => artifactGraphSDK.refresh()}
						disabled={artifactGraphSDK.loading}
					>
						{#if artifactGraphSDK.loading}
							<LoaderCircle class="h-3 w-3 animate-spin" />
							<span>Indexing...</span>
						{:else if artifactGraphSDK.error}
							<TriangleAlertIcon class="h-3 w-3" />
							<span>Index Error</span>
						{:else}
							<DatabaseIcon class="h-3 w-3" />
							<span>{artifactCount} Artifacts Indexed</span>
						{/if}
					</button>
				{/snippet}
			</Tooltip.Trigger>
			<Tooltip.Content side="top">
				<p>{artifactGraphSDK.error ? `Index error: ${artifactGraphSDK.error}` : "Rebuild artifact graph index"}</p>
			</Tooltip.Content>
		</Tooltip.Root>

		<span class="h-3 w-px bg-border"></span>

		<div class="flex items-center gap-1.5">
			<span>{settingsStore.sidecarStateLabel}</span>
			<span class="inline-block h-2 w-2 rounded-full {statusColor}"></span>
		</div>
	</div>
</div>
