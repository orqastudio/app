<script lang="ts">
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import * as Card from "$lib/components/ui/card";
	import CircleXIcon from "@lucide/svelte/icons/circle-x";
	import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import ProviderSettings from "./ProviderSettings.svelte";
	import ModelSettings from "./ModelSettings.svelte";
	import AppearanceSettings from "./AppearanceSettings.svelte";
	import ShortcutsSettings from "./ShortcutsSettings.svelte";
	import ProjectSetupWizard from "./ProjectSetupWizard.svelte";
	import ProjectGeneralSettings from "./ProjectGeneralSettings.svelte";
	import ProjectScanningSettings from "./ProjectScanningSettings.svelte";
	import ProjectGovernanceSettings from "./ProjectGovernanceSettings.svelte";

	const project = $derived(projectStore.activeProject);
	const isProjectSection = $derived(
		settingsStore.activeSection === "project-general" ||
		settingsStore.activeSection === "project-scanning" ||
		settingsStore.activeSection === "project-governance",
	);
</script>

<ScrollArea.Root class="h-full">
	<div class="space-y-6 p-6">
		{#if settingsStore.activeSection === "provider"}
			<ProviderSettings />
		{/if}

		{#if settingsStore.activeSection === "model"}
			<ModelSettings />
		{/if}

		{#if settingsStore.activeSection === "appearance"}
			<AppearanceSettings />
		{/if}

		{#if settingsStore.activeSection === "shortcuts"}
			<ShortcutsSettings />
		{/if}

		{#if isProjectSection}
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
	</div>
</ScrollArea.Root>
