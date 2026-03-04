<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
	import SelectMenu from "$lib/components/shared/SelectMenu.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore, type ThemeMode, type DefaultModel } from "$lib/stores/settings.svelte";
	import { setupStore } from "$lib/stores/setup.svelte";
	import ProjectSetupWizard from "./ProjectSetupWizard.svelte";
	import ProjectGeneralSettings from "./ProjectGeneralSettings.svelte";
	import ProjectScanningSettings from "./ProjectScanningSettings.svelte";
	import ProjectGovernanceSettings from "./ProjectGovernanceSettings.svelte";
	const project = $derived(projectStore.activeProject);

	const shortcuts: { key: string; action: string }[] = [
		{ key: "Ctrl+B", action: "Toggle Nav Sub-Panel" },
		{ key: "Ctrl+N", action: "New session" },
		{ key: "Ctrl+1-5", action: "Switch activity view" },
		{ key: "Ctrl+,", action: "Open settings" },
	];

	const themeModeOptions: { value: ThemeMode; label: string }[] = [
		{ value: "system", label: "System (default)" },
		{ value: "light", label: "Light" },
		{ value: "dark", label: "Dark" },
	];

	const modelOptions: { value: DefaultModel; label: string; description: string }[] = [
		{ value: "auto", label: "Auto (recommended)", description: "Automatically selects the best model" },
		{ value: "claude-opus-4-6", label: "Opus", description: "Most capable, slower" },
		{ value: "claude-sonnet-4-6", label: "Sonnet", description: "Balanced performance" },
		{ value: "claude-haiku-4-5", label: "Haiku", description: "Fastest responses" },
	];

	function sidecarStatusColor(state: string): string {
		switch (state) {
			case "connected":
				return "text-green-500";
			case "starting":
				return "text-yellow-500";
			case "error":
				return "text-red-500";
			case "stopped":
			case "not_started":
			default:
				return "text-muted-foreground";
		}
	}

	function handleThemeChange(value: string): void {
		settingsStore.setThemeMode(value as ThemeMode);
	}

	function handleModelChange(value: string): void {
		settingsStore.setDefaultModel(value as DefaultModel);
	}

	let cliChecking = $state(false);

	async function handleCheckCli(): Promise<void> {
		cliChecking = true;
		await setupStore.checkCli();
		await setupStore.checkAuth();
		cliChecking = false;
	}

	function handleRestart(): void {
		settingsStore.restartSidecar();
	}

	// Auto-check CLI info when provider section is viewed
	$effect(() => {
		if (settingsStore.activeSection === "provider" && !setupStore.cliInfo) {
			handleCheckCli();
		}
	});
</script>

<ScrollArea.Root class="h-full">
	<div class="space-y-6 p-6">
		<!-- Provider section -->
		{#if settingsStore.activeSection === "provider"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Provider</Card.Title>
					<Card.Description>Claude Code CLI connection and sidecar status</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div class="space-y-3">
						<div class="flex items-center gap-2 text-sm">
							<span class="w-32 text-muted-foreground">Sidecar Status:</span>
							<div class="flex items-center gap-1.5">
								{#if settingsStore.sidecarStatus.state === "connected"}
									<CircleCheckIcon class="h-4 w-4 text-green-500" />
								{:else if settingsStore.sidecarStatus.state === "starting"}
									<LoaderCircleIcon class="h-4 w-4 animate-spin text-yellow-500" />
								{:else if settingsStore.sidecarStatus.state === "error"}
									<CircleXIcon class="h-4 w-4 text-red-500" />
								{:else}
									<CircleDotIcon class="h-4 w-4 text-muted-foreground" />
								{/if}
								<span class={sidecarStatusColor(settingsStore.sidecarStatus.state)}>
									{settingsStore.sidecarStateLabel}
								</span>
							</div>
						</div>

						{#if settingsStore.sidecarStatus.pid !== null}
							<div class="flex items-center gap-2 text-sm">
								<span class="w-32 text-muted-foreground">Process ID:</span>
								<span>{settingsStore.sidecarStatus.pid}</span>
							</div>
						{/if}

						{#if settingsStore.sidecarStatus.uptime_seconds !== null}
							<div class="flex items-center gap-2 text-sm">
								<span class="w-32 text-muted-foreground">Uptime:</span>
								<span>{Math.floor(settingsStore.sidecarStatus.uptime_seconds)}s</span>
							</div>
						{/if}

						<div class="flex items-center gap-2 text-sm">
							<span class="w-32 text-muted-foreground">CLI Detected:</span>
							{#if settingsStore.sidecarStatus.cli_detected}
								<div class="flex items-center gap-1.5">
									<CircleCheckIcon class="h-4 w-4 text-green-500" />
									<span>{settingsStore.sidecarStatus.cli_version ?? "Unknown version"}</span>
								</div>
							{:else}
								<div class="flex items-center gap-1.5">
									<CircleXIcon class="h-4 w-4 text-muted-foreground" />
									<span class="text-muted-foreground">Not found</span>
								</div>
							{/if}
						</div>

						{#if settingsStore.sidecarStatus.error_message}
							<div class="rounded-md border border-red-200 bg-red-50 px-3 py-2 text-sm text-red-700 dark:border-red-800 dark:bg-red-950 dark:text-red-300">
								{settingsStore.sidecarStatus.error_message}
							</div>
						{/if}
					</div>

					<Separator />

					<Button variant="outline" size="sm" onclick={handleRestart}>
						<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
						Restart Sidecar
					</Button>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header>
					<Card.Title>Claude CLI</Card.Title>
					<Card.Description>Claude Code CLI version and authentication status</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					{#if cliChecking}
						<div class="flex items-center gap-2 text-sm">
							<LoaderCircleIcon class="h-4 w-4 animate-spin text-muted-foreground" />
							<span class="text-muted-foreground">Checking CLI status...</span>
						</div>
					{:else if setupStore.cliInfo}
						<div class="space-y-3">
							<div class="flex items-center gap-2 text-sm">
								<span class="w-32 text-muted-foreground">Installed:</span>
								{#if setupStore.cliInfo.installed}
									<div class="flex items-center gap-1.5">
										<CircleCheckIcon class="h-4 w-4 text-green-500" />
										<span>Yes</span>
									</div>
								{:else}
									<div class="flex items-center gap-1.5">
										<CircleXIcon class="h-4 w-4 text-red-500" />
										<span class="text-red-600 dark:text-red-400">Not found</span>
									</div>
								{/if}
							</div>

							{#if setupStore.cliInfo.version}
								<div class="flex items-center gap-2 text-sm">
									<span class="w-32 text-muted-foreground">Version:</span>
									<span class="font-mono text-xs">{setupStore.cliInfo.version}</span>
								</div>
							{/if}

							{#if setupStore.cliInfo.path}
								<div class="flex items-center gap-2 text-sm">
									<span class="w-32 text-muted-foreground">Path:</span>
									<span class="font-mono text-xs">{setupStore.cliInfo.path}</span>
								</div>
							{/if}

							<div class="flex items-center gap-2 text-sm">
								<span class="w-32 text-muted-foreground">Authenticated:</span>
								{#if setupStore.cliInfo.authenticated}
									<div class="flex items-center gap-1.5">
										<ShieldCheckIcon class="h-4 w-4 text-green-500" />
										<span>Yes</span>
										{#if setupStore.cliInfo.subscription_type}
											<Badge variant="secondary" class="text-xs">{setupStore.cliInfo.subscription_type}</Badge>
										{/if}
									</div>
								{:else}
									<div class="flex items-center gap-1.5">
										<CircleXIcon class="h-4 w-4 text-amber-500" />
										<span class="text-amber-600 dark:text-amber-400">Not authenticated</span>
									</div>
								{/if}
							</div>
						</div>
					{:else}
						<p class="text-sm text-muted-foreground">CLI status not checked yet.</p>
					{/if}

					<Separator />

					<Button variant="outline" size="sm" onclick={handleCheckCli} disabled={cliChecking}>
						<RefreshCwIcon class="mr-1.5 h-3.5 w-3.5" />
						Re-check Status
					</Button>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Model section -->
		{#if settingsStore.activeSection === "model"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Model</Card.Title>
					<Card.Description>Select the default Claude model for new sessions</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div>
						<span class="text-sm font-medium">Default Model</span>
						<div class="mt-1">
							<SelectMenu
								items={modelOptions}
								selected={settingsStore.defaultModel}
								onSelect={handleModelChange}
								triggerLabel={modelOptions.find((o) => o.value === settingsStore.defaultModel)?.label ?? "Auto"}
								triggerSize="default"
								align="start"
							/>
						</div>
						<p class="mt-1.5 text-xs text-muted-foreground">
							{modelOptions.find((o) => o.value === settingsStore.defaultModel)?.description ?? ""}
						</p>
					</div>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Project sections (General / Scanning / Governance) -->
		{#if settingsStore.activeSection === "project-general" || settingsStore.activeSection === "project-scanning" || settingsStore.activeSection === "project-governance"}
			{#if !project}
				<Card.Root>
					<Card.Content class="py-8">
						<div class="flex items-center gap-2 text-sm text-muted-foreground">
							<CircleXIcon class="h-4 w-4" />
							No project loaded
						</div>
					</Card.Content>
				</Card.Root>
			{:else if !projectStore.settingsLoaded}
				<Card.Root>
					<Card.Content class="flex items-center gap-2 py-8">
						<LoaderCircleIcon class="h-4 w-4 animate-spin" />
						<span class="text-sm text-muted-foreground">Loading project settings...</span>
					</Card.Content>
				</Card.Root>
			{:else if projectStore.hasSettings && projectStore.projectSettings}
				{#if settingsStore.activeSection === "project-general"}
					<ProjectGeneralSettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						iconDataUrl={projectStore.iconDataUrl}
						onUploadIcon={(sourcePath) => projectStore.uploadIcon(sourcePath)}
						onRemoveIcon={() => projectStore.removeIcon()}
					/>
				{:else if settingsStore.activeSection === "project-scanning"}
					<ProjectScanningSettings
						settings={projectStore.projectSettings}
						onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						onRescan={() => projectStore.scanProject(project.path, projectStore.projectSettings?.excluded_paths)}
						rescanning={projectStore.scanning}
					/>
				{:else if settingsStore.activeSection === "project-governance"}
					<ProjectGovernanceSettings
						governance={projectStore.projectSettings.governance}
					/>
				{/if}
			{:else}
				<ProjectSetupWizard
					projectPath={project.path}
					onComplete={(s) => { projectStore.projectSettings = s; }}
				/>
			{/if}
		{/if}

		<!-- Appearance section -->
		{#if settingsStore.activeSection === "appearance"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Appearance</Card.Title>
					<Card.Description>Theme and display preferences</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-4">
					<div>
						<span class="text-sm font-medium">Theme</span>
						<div class="mt-1">
							<SelectMenu
								items={themeModeOptions}
								selected={settingsStore.themeMode}
								onSelect={handleThemeChange}
								triggerLabel={themeModeOptions.find((o) => o.value === settingsStore.themeMode)?.label ?? "System"}
								triggerSize="default"
								align="start"
							/>
						</div>
					</div>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Keyboard shortcuts section -->
		{#if settingsStore.activeSection === "shortcuts"}
			<Card.Root>
				<Card.Header>
					<Card.Title>Keyboard Shortcuts</Card.Title>
					<Card.Description>Reference card for available shortcuts</Card.Description>
				</Card.Header>
				<Card.Content>
					<div class="space-y-2">
						{#each shortcuts as shortcut}
							<div class="flex items-center justify-between rounded px-2 py-1.5 text-sm hover:bg-muted/50">
								<span class="text-muted-foreground">{shortcut.action}</span>
								<Badge variant="outline" class="font-mono text-xs">{shortcut.key}</Badge>
							</div>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>
		{/if}
	</div>
</ScrollArea.Root>
