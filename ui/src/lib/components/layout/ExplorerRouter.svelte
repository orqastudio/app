<script lang="ts">
	import { getStores } from "@orqastudio/sdk";
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import FullGraphView from "$lib/components/graph/FullGraphView.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import ArtifactMasterDetail from "$lib/components/artifact/ArtifactMasterDetail.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import PluginViewContainer from "$lib/components/plugin/PluginViewContainer.svelte";

	const { navigationStore } = getStores();

	const activePluginView = $derived.by(() => {
		const navItem = navigationStore.activeNavItem;
		if (!navItem || navItem.type !== "plugin" || !navItem.pluginSource) return null;
		return { pluginName: navItem.pluginSource, viewKey: navItem.key };
	});

	const viewType = $derived.by((): string => {
		if (activePluginView) return "plugin";
		if (navigationStore.activeActivity === "project") return "project";
		if (navigationStore.activeActivity === "artifact-graph") return "graph";
		if (navigationStore.explorerView === "artifact-viewer") return "artifact-viewer";
		if (navigationStore.activeGroup !== null || navigationStore.isArtifactActivity) return "artifact-list";
		return "welcome";
	});
</script>

<div class="h-full w-full">
	{#if viewType === "plugin" && activePluginView}
		<PluginViewContainer
			pluginName={activePluginView.pluginName}
			viewKey={activePluginView.viewKey}
		/>
	{:else if viewType === "project"}
		<ProjectDashboard />
	{:else if viewType === "graph"}
		<FullGraphView />
	{:else if viewType === "artifact-viewer"}
		<ArtifactViewer />
	{:else if viewType === "artifact-list"}
		<ArtifactMasterDetail activity={navigationStore.activeActivity} />
	{:else}
		<WelcomeScreen />
	{/if}
</div>
