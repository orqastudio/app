<script lang="ts">
	import { Icon, ScrollArea, HStack, Stack, Caption } from "@orqastudio/svelte-components/pure";
	import { Panel } from "@orqastudio/svelte-components/pure";
	import { CardRoot, CardContent } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { projectStore, settingsStore, pluginRegistry } = getStores();
	import ProviderSettings from "./ProviderSettings.svelte";
	import ModelSettings from "./ModelSettings.svelte";
	import AppearanceSettings from "./AppearanceSettings.svelte";
	import ShortcutsSettings from "./ShortcutsSettings.svelte";
	import ProjectSetupWizard from "./ProjectSetupWizard.svelte";
	import ProjectGeneralSettings from "./ProjectGeneralSettings.svelte";
	import ProjectScanningSettings from "./ProjectScanningSettings.svelte";
	import ProjectArtifactLinksSettings from "./ProjectArtifactLinksSettings.svelte";
	import ProjectDeliverySettings from "./ProjectDeliverySettings.svelte";
	import ProjectStatusSettings from "./ProjectStatusSettings.svelte";
	import RelationshipSettings from "./RelationshipSettings.svelte";
	import PluginBrowser from "./PluginBrowser.svelte";
	import PluginViewContainer from "$lib/components/plugin/PluginViewContainer.svelte";

	interface Props {
		activeSection?: string;
	}

	const { activeSection }: Props = $props();

	const section = $derived(activeSection ?? settingsStore.activeSection);
	const project = $derived(projectStore.activeProject);

	// A plugin settings section ID has the form "plugin:<pluginName>:<pageId>".
	// Resolve it against the registry to find the view key to render.
	const pluginPageMatch = $derived.by(() => {
		if (!section.startsWith("plugin:")) return null;
		const pages = pluginRegistry.getSettingsPages();
		const page = pages.find((p) => `plugin:${p.pluginName}:${p.id}` === section);
		return page ?? null;
	});

	const isProjectSection = $derived(
		section === "project-general" ||
			section === "project-scanning" ||
			section === "project-relationships" ||
			section === "project-artifact-links" ||
			section === "project-delivery" ||
			section === "project-status" ||
			section === "project-plugins",
	);
</script>

<ScrollArea maxHeight="viewport">
	<Panel padding="loose">
		<Stack gap={6}>
			{#if section === "provider"}
				<ProviderSettings />
			{/if}

			{#if section === "model"}
				<ModelSettings />
			{/if}

			{#if section === "appearance"}
				<AppearanceSettings />
			{/if}

			{#if section === "shortcuts"}
				<ShortcutsSettings />
			{/if}

			{#if pluginPageMatch}
				<PluginViewContainer
					pluginName={pluginPageMatch.pluginName}
					viewKey={pluginPageMatch.view_key}
				/>
			{/if}

			{#if isProjectSection}
				{#if !project}
					<CardRoot>
						<CardContent>
							<Panel padding="loose">
								<HStack gap={2}>
									<Icon name="circle-x" size="md" />
									<Caption tone="muted">No project loaded</Caption>
								</HStack>
							</Panel>
						</CardContent>
					</CardRoot>
				{:else if !projectStore.settingsLoaded}
					<CardRoot>
						<CardContent>
							<Panel padding="loose">
								<HStack gap={2}>
									<Icon name="loader-circle" size="md" />
									<Caption tone="muted">Loading project settings...</Caption>
								</HStack>
							</Panel>
						</CardContent>
					</CardRoot>
				{:else if projectStore.hasSettings && projectStore.projectSettings}
					{#if section === "project-general"}
						<ProjectGeneralSettings
							settings={projectStore.projectSettings}
							onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
							iconDataUrl={projectStore.iconDataUrl}
							onUploadIcon={(sourcePath) => projectStore.uploadIcon(sourcePath)}
							onRemoveIcon={() => projectStore.removeIcon()}
						/>
					{:else if section === "project-scanning"}
						<ProjectScanningSettings
							settings={projectStore.projectSettings}
							onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
							onRescan={() =>
								projectStore.scanProject(
									project.path,
									projectStore.projectSettings?.excluded_paths,
								)}
							rescanning={projectStore.scanning}
						/>
					{:else if section === "project-relationships"}
						<RelationshipSettings />
					{:else if section === "project-artifact-links"}
						<ProjectArtifactLinksSettings
							settings={projectStore.projectSettings}
							onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						/>
					{:else if section === "project-delivery"}
						<ProjectDeliverySettings
							settings={projectStore.projectSettings}
							onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						/>
					{:else if section === "project-status"}
						<ProjectStatusSettings
							settings={projectStore.projectSettings}
							onSave={(s) => projectStore.saveProjectSettings(project.path, s)}
						/>
					{:else if section === "project-plugins"}
						<PluginBrowser />
					{/if}
				{:else}
					<ProjectSetupWizard
						projectPath={project.path}
						onComplete={(s) => {
							projectStore.projectSettings = s;
						}}
					/>
				{/if}
			{/if}
		</Stack>
	</Panel>
</ScrollArea>
