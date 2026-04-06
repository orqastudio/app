<!-- Status bar rendered at the bottom of the app shell. Displays brand, active model,
     token counters, artifact index state, sidecar status, and daemon health. -->
<script lang="ts">
	import {
		Icon,
		ConnectionIndicator,
		Text,
		Spacer,
		TooltipRoot,
		TooltipTrigger,
		TooltipContent,
		type ConnectionState,
	} from "@orqastudio/svelte-components/pure";
	import { getStores, fmt } from "@orqastudio/sdk";
	import { assertNever } from "@orqastudio/types";

	const { settingsStore, sessionStore, navigationStore, artifactGraphSDK, pluginRegistry } =
		getStores();
	import finMark from "$lib/assets/fin-mark.svg";

	/** Map the sidecar process state to a ConnectionIndicator state. */
	const sidecarConnectionState = $derived.by((): ConnectionState => {
		switch (settingsStore.sidecarStatus.state) {
			case "connected":
				return "connected";
			case "starting":
				return "reconnecting";
			case "error":
				return "disconnected";
			case "stopped":
				return "waiting";
			case "not_started":
				return "waiting";
			default:
				return assertNever(settingsStore.sidecarStatus.state);
		}
	});

	/** Map the daemon health state to a ConnectionIndicator state. */
	const daemonConnectionState = $derived.by((): ConnectionState => {
		switch (settingsStore.daemonHealth.state) {
			case "connected":
				return "connected";
			case "degraded":
				return "reconnecting";
			case "disconnected":
				return "disconnected";
			default:
				return assertNever(settingsStore.daemonHealth.state);
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
				return assertNever(status.state);
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
		session !== null && (session.total_input_tokens > 0 || session.total_output_tokens > 0),
	);

	const artifactCount = $derived(
		Math.max(artifactGraphSDK.graph.size, settingsStore.daemonHealth.artifacts ?? 0),
	);

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

<div class="status-bar">
	<!-- Left: Brand | Model -->
	<div class="flex items-center gap-3">
		<div class="flex items-center gap-1">
			<img src={finMark} class="h-3.5 w-3.5" alt="" />
			<span class="brand-label">OrqaStudio</span>
		</div>

		<div class="bg-border h-3 w-px"></div>

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button {...props} class="status-btn" onclick={openModelSettings}>
						<Icon name="brain" size="xs" />
						<span>{settingsStore.modelDisplayName}</span>
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<Text variant="body">Change model</Text>
			</TooltipContent>
		</TooltipRoot>
	</div>

	<Spacer />

	<!-- Startup task indicator -->
	{#if settingsStore.activeStartupTask}
		<div class="mr-4 flex items-center gap-1">
			<Icon name="loader-circle" size="xs" />
			<span>
				{settingsStore.activeStartupTask.label}{settingsStore.activeStartupTask.detail
					? `: ${settingsStore.activeStartupTask.detail}`
					: "..."}
			</span>
		</div>
	{/if}

	<!-- Right: Tokens | Index | Sidecar status | Daemon health -->
	<div class="flex items-center gap-3">
		{#if hasTokens && session}
			<span class="token-counter">
				{formatTokens(session.total_input_tokens)}↑ {formatTokens(session.total_output_tokens)}↓
			</span>
			<div class="bg-border h-3 w-px"></div>
		{/if}

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button
						{...props}
						class="status-btn {artifactGraphSDK.error ? 'text-destructive' : ''}"
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
				<Text variant="body"
					>{artifactGraphSDK.error
						? `Index error: ${artifactGraphSDK.error}`
						: "Rebuild artifact graph index"}</Text
				>
			</TooltipContent>
		</TooltipRoot>

		<div class="bg-border h-3 w-px"></div>

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button {...props} class="status-btn" onclick={openPluginSettings}>
						<ConnectionIndicator
							state={sidecarConnectionState}
							label={settingsStore.sidecarStateLabel}
						/>
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<Text variant="body">{sidecarTooltip}</Text>
			</TooltipContent>
		</TooltipRoot>

		<div class="bg-border h-3 w-px"></div>

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<button {...props} class="status-btn" onclick={() => settingsStore.refreshDaemonHealth()}>
						<ConnectionIndicator
							state={daemonConnectionState}
							label={settingsStore.daemonStateLabel}
						/>
					</button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<Text variant="body">{daemonTooltip}</Text>
			</TooltipContent>
		</TooltipRoot>
	</div>
</div>

<style>
	/* Status bar container — fixed height, full-width footer strip. */
	.status-bar {
		display: flex;
		height: 2rem;
		align-items: center;
		border-top: 1px solid var(--color-border);
		background-color: color-mix(in srgb, var(--color-muted) 30%, transparent);
		padding: 0 1rem 0.25rem;
		font-size: 0.75rem;
		color: var(--color-muted-foreground);
	}

	/* Brand name — slightly subdued foreground to avoid competing with content. */
	.brand-label {
		font-weight: 500;
		color: color-mix(in srgb, var(--color-foreground) 70%, transparent);
	}

	/* Compact ghost button sized for the 2rem status bar row. */
	:global(.status-btn) {
		height: 1.5rem;
		padding-inline: 0.375rem;
		font-size: 0.75rem;
		gap: 0.25rem;
	}

	/* Token counter — tabular numerics prevent layout shift as values change. */
	.token-counter {
		font-variant-numeric: tabular-nums;
		color: color-mix(in srgb, var(--color-muted-foreground) 70%, transparent);
	}
</style>
