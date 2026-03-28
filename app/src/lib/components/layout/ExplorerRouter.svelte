<script lang="ts">
	// Explorer router — resolves which component to render in the main explorer panel
	// based on the current navigation state (activity, plugin views, artifact selection).
	import { getStores } from "@orqastudio/sdk";
	import { type Component } from "svelte";

	// Core view components — registered by route key
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import FullGraphView from "$lib/components/graph/FullGraphView.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import PluginViewContainer from "$lib/components/plugin/PluginViewContainer.svelte";
	import PluginBrowser from "$lib/components/settings/PluginBrowser.svelte";

	const { navigationStore } = getStores();

	/**
	 * Core view registry — maps route keys to components.
	 * Plugin views are handled separately via PluginViewContainer.
	 * New core views are added here, not as if/else branches.
	 *
	 * Note: The artifact LIST lives in NavSubPanel (level 2/3 navigation).
	 * The explorer only shows the artifact DETAIL when one is selected,
	 * or a placeholder when nothing is selected.
	 */
	const CORE_VIEWS: Record<string, Component> = {
		"project": ProjectDashboard,
		"artifact-graph": FullGraphView,
		"welcome": WelcomeScreen,
		"plugins": PluginBrowser,
	};

	// Resolve what to render in the explorer panel
	const resolved = $derived.by(() => {
		const navItem = navigationStore.activeNavItem;

		// Plugin view — loaded at runtime from plugin bundle
		if (navItem?.type === "plugin" && navItem.pluginSource) {
			return {
				type: "plugin" as const,
				pluginName: navItem.pluginSource,
				viewKey: navItem.key,
			};
		}

		// Core view by activity key
		const activity = navigationStore.activeActivity;
		if (CORE_VIEWS[activity]) {
			return { type: "core" as const, component: CORE_VIEWS[activity] };
		}

		// Artifact detail — show ArtifactViewer as soon as a path is selected.
		// ArtifactViewer handles its own loading spinner internally.
		if (navigationStore.selectedArtifactPath) {
			return { type: "core" as const, component: ArtifactViewer };
		}

		// Artifact area active but nothing selected — show placeholder
		if (navigationStore.activeGroup !== null || navigationStore.isArtifactActivity) {
			return { type: "placeholder" as const };
		}

		// Default
		return { type: "core" as const, component: WelcomeScreen };
	});
</script>

<div class="h-full w-full">
	{#if resolved.type === "plugin"}
		<PluginViewContainer
			pluginName={resolved.pluginName}
			viewKey={resolved.viewKey}
		/>
	{:else if resolved.type === "placeholder"}
		<div class="flex h-full items-center justify-center text-sm text-muted-foreground">
			Select an item to view it
		</div>
	{:else}
		{@const ViewComponent = resolved.component}
		<ViewComponent />
	{/if}
</div>
