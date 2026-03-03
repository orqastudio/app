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
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore, type ThemeMode, type DefaultModel } from "$lib/stores/settings.svelte";
	import ProjectSetupWizard from "./ProjectSetupWizard.svelte";
	import ProjectSettingsForm from "./ProjectSettingsForm.svelte";
	import type { ProjectSettings } from "$lib/types";

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

	function handleThemeChange(e: Event): void {
		const target = e.target as HTMLSelectElement;
		settingsStore.setThemeMode(target.value as ThemeMode);
	}

	function handleModelChange(e: Event): void {
		const target = e.target as HTMLSelectElement;
		settingsStore.setDefaultModel(target.value as DefaultModel);
	}

	function handleRestart(): void {
		settingsStore.restartSidecar();
	}
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
						<label class="text-sm font-medium" for="model-select">Default Model</label>
						<select
							id="model-select"
							class="mt-1 flex h-9 w-full max-w-xs rounded-md border border-input bg-background px-3 py-1 text-sm"
							value={settingsStore.defaultModel}
							onchange={handleModelChange}
						>
							{#each modelOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
						<p class="mt-1.5 text-xs text-muted-foreground">
							{modelOptions.find((o) => o.value === settingsStore.defaultModel)?.description ?? ""}
						</p>
					</div>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Project section -->
		{#if settingsStore.activeSection === "project"}
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
				<ProjectSettingsForm
					settings={projectStore.projectSettings}
					projectPath={project.path}
					onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
					onRescan={() => projectStore.scanProject(project.path, projectStore.projectSettings?.excluded_paths)}
					rescanning={projectStore.scanning}
					iconDataUrl={projectStore.iconDataUrl}
					onUploadIcon={(sourcePath) => projectStore.uploadIcon(sourcePath)}
					onRemoveIcon={() => projectStore.removeIcon()}
				/>
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
						<label class="text-sm font-medium" for="theme-select">Theme</label>
						<select
							id="theme-select"
							class="mt-1 flex h-9 w-full max-w-xs rounded-md border border-input bg-background px-3 py-1 text-sm"
							value={settingsStore.themeMode}
							onchange={handleThemeChange}
						>
							{#each themeModeOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
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
