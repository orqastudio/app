<script lang="ts">
	import { Icon } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { getStores, fmt } from "@orqastudio/sdk";

	const { settingsStore, sessionStore, navigationStore, artifactGraphSDK, pluginRegistry } = getStores();
	import finMark from "$lib/assets/fin-mark.svg";

	const sidecarColor = $derived.by(() => {
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

	const daemonColor = $derived.by(() => {
		switch (settingsStore.daemonHealth.state) {
			case "connected":
				return "bg-success";
			case "degraded":
				return "bg-warning";
			case "disconnected":
			default:
				return "bg-muted-foreground";
		}
	});

	/**
	 * Display name for the active sidecar provider.
	 * Read from the plugin registry so it reflects the installed plugin,
	 * not a hardcoded package name.
	 */
	const sidecarProviderName = $derived(pluginRegistry.activeSidecar?.label ?? "No sidecar");

	const sidecarTooltip = $derived.by(() => {
		const status = settingsStore.sidecarStatus;
		switch (status.state) {
			case "connected":
				return `Connected via ${sidecarProviderName}`;
			case "starting":
				return `Starting ${sidecarProviderName}...`;
			case "error":
				return `Error: ${status.error_message ?? "Unknown error"}`;
			case "stopped":
				return `Stopped — ${sidecarProviderName}`;
			case "not_started":
				return "No providers configured";
			default:
				return "No providers configured";
		}
	});

	const daemonTooltip = $derived.by(() => {
		const health = settingsStore.daemonHealth;
		if (health.state === "connected") {
			return `Daemon: ${health.artifacts} artifacts, ${health.rules} rules`;
		}
		if (health.state === "degraded") {
			return `Daemon degraded: ${health.error}`;
		}
		return `Daemon offline${health.error ? `: ${health.error}` : ""}`;
	});

	const session = $derived(sessionStore.activeSession);
	const hasTokens = $derived(
		session !== null &&
			(session.total_input_tokens > 0 || session.total_output_tokens > 0),
	);

	const artifactCount = $derived(Math.max(artifactGraphSDK.graph.size, settingsStore.daemonHealth.artifacts ?? 0));

	/**
	 * Format a token count as a human-readable string (e.g. 1.2k, 3.4M).
	 * @param count - The raw token count.
	 * @returns A formatted string with k or M suffix where appropriate.
	 */
	function formatTokens(count: number): string {
		if (count >= 1_000_000) {
			return `${fmt(count / 1_000_000, 1)}M`;
		}
		if (count >= 1000) {
			return `${fmt(count / 1000, 1)}k`;
		}
		return String(count);
	}

	/** Navigate to the model settings section. */
	function openModelSettings() {
		settingsStore.setActiveSection("model");
		navigationStore.setActivity("settings");
	}

	/** Navigate to the project-plugins settings section. */
	function openPluginSettings() {
		settingsStore.setActiveSection("project-plugins");
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

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="flex items-center gap-1 rounded px-1.5 py-0.5 transition-colors hover:bg-accent hover:text-accent-foreground"
						onclick={openModelSettings}
					>
						<Icon name="brain" size="xs" />
						<span>{settingsStore.modelDisplayName}</span>
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<p>Change model</p>
			</TooltipContent>
		</TooltipRoot>
	</div>

	<!-- Center: spacer -->
	<div class="flex-1"></div>

	<!-- Startup task indicator -->
	{#if settingsStore.activeStartupTask}
		<div class="mr-4 flex items-center gap-1.5">
			<Icon name="loader-circle" size="xs" />
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

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="flex items-center gap-1 rounded px-1.5 py-0.5 transition-colors hover:bg-accent hover:text-accent-foreground disabled:cursor-not-allowed disabled:opacity-50 {artifactGraphSDK.error ? 'text-destructive' : ''}"
						onclick={() => artifactGraphSDK.refresh()}
						disabled={artifactGraphSDK.loading}
					>
						{#if artifactGraphSDK.loading}
							<Icon name="loader-circle" size="xs" />
						{:else if artifactGraphSDK.error}
							<Icon name="triangle-alert" size="xs" />
							<span>Index Error</span>
						{:else}
							<Icon name="database" size="xs" />
							<span>{artifactCount}</span>
						{/if}
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<p>{artifactGraphSDK.error ? `Index error: ${artifactGraphSDK.error}` : "Rebuild artifact graph index"}</p>
			</TooltipContent>
		</TooltipRoot>

		<span class="h-3 w-px bg-border"></span>

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="flex items-center gap-1.5 rounded px-1.5 py-0.5 transition-colors hover:bg-accent hover:text-accent-foreground"
						onclick={openPluginSettings}
					>
						<span class="inline-block h-2 w-2 rounded-full {sidecarColor}"></span>
						<span>{settingsStore.sidecarStateLabel}</span>
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<p>{sidecarTooltip}</p>
			</TooltipContent>
		</TooltipRoot>

		<span class="h-3 w-px bg-border"></span>

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="flex items-center gap-1.5 rounded transition-colors hover:bg-accent hover:text-accent-foreground"
						onclick={() => settingsStore.refreshDaemonHealth()}
					>
						<span class="inline-block h-2 w-2 rounded-full {daemonColor}"></span>
						<span>{settingsStore.daemonStateLabel}</span>
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<p>{daemonTooltip}</p>
			</TooltipContent>
		</TooltipRoot>
	</div>
</div>
