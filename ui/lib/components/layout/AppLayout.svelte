<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import type { UnlistenFn } from "@tauri-apps/api/event";
	import ActivityBar from "./ActivityBar.svelte";
	import NavSubPanel from "./NavSubPanel.svelte";
	import Toolbar from "./Toolbar.svelte";
	import StatusBar from "./StatusBar.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import ConversationView from "$lib/components/conversation/ConversationView.svelte";
	import ProjectSetupWizard from "$lib/components/settings/ProjectSetupWizard.svelte";
	import SetupWizard from "$lib/components/setup/SetupWizard.svelte";
	import GovernanceBootstrapWizard from "$lib/components/governance/GovernanceBootstrapWizard.svelte";

	import ArtifactMasterDetail from "$lib/components/artifact/ArtifactMasterDetail.svelte";
	import * as Resizable from "$lib/components/ui/resizable";
	import setupBackground from "$lib/assets/setup-background.png";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { setupStore } from "$lib/stores/setup.svelte";
	import { governanceStore } from "$lib/stores/governance.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";

	/** Unlisten function for the artifact-changed event, cleaned up on destroy. */
	let unlistenArtifactChanged: UnlistenFn | null = null;

	const hasProject = $derived(projectStore.hasProject);
	const groupHasMultipleSubCategories = $derived(
		navigationStore.activeGroup !== null &&
		navigationStore.groupSubCategories[navigationStore.activeGroup].length > 1,
	);
	const needsSetup = $derived(projectStore.settingsLoaded && !projectStore.hasSettings);
	const hideChatPanel = $derived(
		navigationStore.activeActivity === "settings",
	);
	const setupNeeded = $derived(!setupStore.setupComplete);

	onMount(async () => {
		settingsStore.initialize();
		await setupStore.checkSetupStatus();
		if (setupStore.setupComplete) {
			projectStore.loadActiveProject();
		}

		// Listen for backend file-watcher events and refresh the nav tree.
		unlistenArtifactChanged = await listen("artifact-changed", () => {
			artifactStore.invalidateNavTree();
		});
	});

	onDestroy(() => {
		settingsStore.destroy();
		unlistenArtifactChanged?.();
	});

	// When a project becomes active, switch to the project dashboard
	$effect(() => {
		if (hasProject && !needsSetup && navigationStore.activeActivity === "chat") {
			navigationStore.setActivity("project");
		}
	});


	// Load the unified navigation tree once the project is ready
	$effect(() => {
		if (hasProject && !needsSetup && artifactStore.navTree === null) {
			artifactStore.loadNavTree();
		}
	});

	// Start the .orqa/ file watcher when a project becomes active so the nav
	// tree refreshes automatically when files change on disk.
	$effect(() => {
		const project = projectStore.activeProject;
		if (!project || needsSetup) return;
		invoke("artifact_watch_start", { projectPath: project.path }).catch((err: unknown) => {
			console.warn("[artifact-watcher] failed to start:", err);
		});
	});

	// Load enforcement rules when the rules activity is active
	$effect(() => {
		const activity = navigationStore.activeActivity;
		if (hasProject && !needsSetup && activity === "rules") {
			enforcementStore.loadRules();
		}
	});

	// Auto-trigger governance scan when a project is fully loaded
	$effect(() => {
		const project = projectStore.activeProject;
		if (!project || needsSetup) return;
		const projectId = project.id;
		(async () => {
			await governanceStore.scan(projectId);
			await governanceStore.checkExistingAnalysis(projectId);
			if (
				governanceStore.scanResult !== null &&
				governanceStore.scanResult.coverage_ratio < 3 / 7 &&
				governanceStore.analysis === null
			) {
				governanceStore.showWizard();
			}
		})();
	});

	// Auto-load artifact content when the selected artifact path changes
	$effect(() => {
		const path = navigationStore.selectedArtifactPath;
		if (!path || !hasProject || needsSetup) return;
		artifactStore.loadContent(path);
	});
</script>

<div class="flex h-screen flex-col bg-background text-foreground">
	<!-- Toolbar -->
	<Toolbar />

	<!-- Main Content Area -->
	<div class="flex flex-1 overflow-hidden">
		{#if setupNeeded}
			<!-- First-run setup wizard — blocks all other content -->
			<SetupWizard
				onComplete={() => {
					projectStore.loadActiveProject();
				}}
			/>
		{:else if hasProject && needsSetup}
			<!-- Project needs setup — show wizard only, no chat/nav/activity bar -->
			<div
				class="relative flex flex-1 items-center justify-center overflow-hidden"
				style="background-image: url({setupBackground}); background-size: cover; background-position: center;"
			>
				<div class="absolute inset-0 bg-background/70"></div>
				<div class="relative z-10 w-full max-w-lg px-4">
					<ProjectSetupWizard
						projectPath={projectStore.projectPath ?? ""}
						onComplete={() => {}}
					/>
				</div>
			</div>
		{:else if hasProject}
			<!-- Activity Bar (48px fixed width) — project only -->
			<ActivityBar />

			<!-- Nav Sub-Panel (200px) — shown for settings/chat and for groups with
			     more than one sub-category (Planning, Team, Governance). Hidden for
			     Documentation (single sub-category) and single-item views like Orchestrator. -->
			{#if navigationStore.showNavPanel && (navigationStore.activeGroup === null || groupHasMultipleSubCategories)}
				<NavSubPanel />
			{/if}

			<!-- Explorer + Chat (resizable) -->
			{#if hideChatPanel}
				<div class="min-w-0 flex-1 overflow-hidden">
					{#if navigationStore.activeActivity === "settings"}
						<SettingsView />
					{:else}
						<WelcomeScreen />
					{/if}
				</div>
			{:else}
				<Resizable.PaneGroup direction="horizontal" class="flex-1">
					<Resizable.Pane defaultSize={70} minSize={30}>
						<div class="h-full overflow-hidden">
							{#if navigationStore.activeActivity === "project"}
								<ProjectDashboard />
							{:else if navigationStore.activeActivity === "chat"}
								<WelcomeScreen />
							{:else if navigationStore.activeGroup !== null}
								{#if navigationStore.activeActivity === "orchestrator"}
									<ArtifactViewer />
								{:else}
									<ArtifactMasterDetail activity={navigationStore.activeActivity} />
								{/if}
							{:else if navigationStore.isArtifactActivity}
								{#if navigationStore.explorerView === "artifact-viewer"}
									<ArtifactViewer />
								{:else}
									<div class="flex h-full items-center justify-center text-sm text-muted-foreground">
										Select an item to view it
									</div>
								{/if}
							{:else}
								<WelcomeScreen />
							{/if}
						</div>
					</Resizable.Pane>
					<Resizable.Handle />
					<Resizable.Pane defaultSize={30} minSize={20}>
						<div class="flex h-full flex-col bg-chat">
							<ConversationView />
						</div>
					</Resizable.Pane>
				</Resizable.PaneGroup>
			{/if}

			<!-- Governance bootstrap wizard overlay -->
			{#if governanceStore.wizardVisible && projectStore.activeProject}
				<GovernanceBootstrapWizard projectId={projectStore.activeProject.id} />
			{/if}
		{:else}
			<!-- No project loaded — welcome screen, no sidebar -->
			<div class="flex flex-1 overflow-hidden">
				<div class="flex-1 overflow-hidden">
					<WelcomeScreen />
				</div>
			</div>
		{/if}
	</div>

	<!-- Status Bar -->
	<StatusBar />
</div>
