<!-- Status bar rendered at the bottom of the app shell. Displays brand, active model,
     token counters, artifact index state, sidecar status, and daemon health. -->
<script lang="ts">
	import {
		Icon,
		ConnectionIndicator,
		Text,
		Button,
		Separator,
		SectionFooter,
		TooltipRoot,
		TooltipTrigger,
		TooltipContent,
		AppIcon,
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

<SectionFooter variant="status-bar">
	{#snippet start()}
		<!-- Brand -->
		<AppIcon src={finMark} alt="" size="xs" />
		<Text variant="body-strong">OrqaStudio</Text>

		<Separator orientation="vertical" />

		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" size="status" onclick={openModelSettings}>
						<Icon name="brain" size="xs" />
						{settingsStore.modelDisplayName}
					</Button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<Text variant="body">Change model</Text>
			</TooltipContent>
		</TooltipRoot>

		<!-- Startup task indicator -->
		{#if settingsStore.activeStartupTask}
			<Icon name="loader-circle" size="xs" />
			<Text variant="caption">
				{settingsStore.activeStartupTask.label}{settingsStore.activeStartupTask.detail
					? `: ${settingsStore.activeStartupTask.detail}`
					: "..."}
			</Text>
		{/if}
	{/snippet}

	{#snippet end()}
		<!-- Token counters -->
		{#if hasTokens && session}
			<Text variant="caption-tabular">
				{formatTokens(session.total_input_tokens)}↑ {formatTokens(session.total_output_tokens)}↓
			</Text>
			<Separator orientation="vertical" />
		{/if}

		<!-- Artifact index -->
		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<Button
						{...props}
						variant="ghost"
						size="status"
						onclick={() => artifactGraphSDK.refresh()}
						disabled={artifactGraphSDK.loading}
					>
						{#if artifactGraphSDK.loading}
							<Icon name="loader-circle" size="xs" />
						{:else if artifactGraphSDK.error}
							<Icon name="triangle-alert" size="xs" />
							Index Error
						{:else}
							<Icon name="database" size="xs" />
							{artifactCount}
						{/if}
					</Button>
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

		<Separator orientation="vertical" />

		<!-- Sidecar status -->
		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" size="status" onclick={openPluginSettings}>
						<ConnectionIndicator
							state={sidecarConnectionState}
							label={settingsStore.sidecarStateLabel}
						/>
					</Button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<Text variant="body">{sidecarTooltip}</Text>
			</TooltipContent>
		</TooltipRoot>

		<Separator orientation="vertical" />

		<!-- Daemon health -->
		<TooltipRoot>
			<TooltipTrigger>
				{#snippet child({ props })}
					<Button
						{...props}
						variant="ghost"
						size="status"
						onclick={() => settingsStore.refreshDaemonHealth()}
					>
						<ConnectionIndicator
							state={daemonConnectionState}
							label={settingsStore.daemonStateLabel}
						/>
					</Button>
				{/snippet}
			</TooltipTrigger>
			<TooltipContent side="top">
				<Text variant="body">{daemonTooltip}</Text>
			</TooltipContent>
		</TooltipRoot>
	{/snippet}
</SectionFooter>
