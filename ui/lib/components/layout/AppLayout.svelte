<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import ActivityBar from "./ActivityBar.svelte";
	import NavSubPanel from "./NavSubPanel.svelte";
	import Toolbar from "./Toolbar.svelte";
	import StatusBar from "./StatusBar.svelte";
	import WelcomeScreen from "./WelcomeScreen.svelte";
	import ProjectDashboard from "$lib/components/dashboard/ProjectDashboard.svelte";
	import ArtifactLanding from "$lib/components/artifact/ArtifactLanding.svelte";
	import ArtifactViewer from "$lib/components/artifact/ArtifactViewer.svelte";
	import SettingsView from "$lib/components/settings/SettingsView.svelte";
	import ConversationView from "$lib/components/conversation/ConversationView.svelte";
	import ProjectSetupWizard from "$lib/components/settings/ProjectSetupWizard.svelte";
	import SetupWizard from "$lib/components/setup/SetupWizard.svelte";
	import GovernanceBootstrapWizard from "$lib/components/governance/GovernanceBootstrapWizard.svelte";

	import LessonsPanel from "$lib/components/lessons/LessonsPanel.svelte";
	import * as Resizable from "$lib/components/ui/resizable";
	import setupBackground from "$lib/assets/setup-background.png";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { setupStore } from "$lib/stores/setup.svelte";
	import { governanceStore } from "$lib/stores/governance.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";

	const hasProject = $derived(projectStore.hasProject);
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
	});

	onDestroy(() => {
		settingsStore.destroy();
	});

	// When a project becomes active, switch to the project dashboard
	$effect(() => {
		if (hasProject && !needsSetup && navigationStore.activeActivity === "chat") {
			navigationStore.setActivity("project");
		}
	});


	// Load doc tree when switching to docs activity (and project is loaded)
	$effect(() => {
		if (
			hasProject &&
			!needsSetup &&
			navigationStore.activeActivity === "docs" &&
			artifactStore.docTree.length === 0
		) {
			artifactStore.loadDocTree();
		}
	});

	// Load research tree when switching to research activity
	$effect(() => {
		if (
			hasProject &&
			!needsSetup &&
			navigationStore.activeActivity === "research" &&
			artifactStore.researchTree.length === 0
		) {
			artifactStore.loadResearchTree();
		}
	});

	// Load plan tree when switching to plans activity
	$effect(() => {
		if (
			hasProject &&
			!needsSetup &&
			navigationStore.activeActivity === "plans" &&
			artifactStore.planTree.length === 0
		) {
			artifactStore.loadPlanTree();
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

	// Activity-to-artifact-type mapping
	const activityToArtifactType: Record<string, string> = {
		agents: "agent",
		rules: "rule",
		skills: "skill",
		hooks: "hook",
	};

	// Load governance artifacts when switching to agents/rules/skills/hooks activity
	$effect(() => {
		const activity = navigationStore.activeActivity;
		const artifactType = activityToArtifactType[activity];
		if (hasProject && !needsSetup && artifactType) {
			artifactStore.loadGovernanceList(artifactType);
			if (activity === "rules") {
				enforcementStore.loadRules();
			}
		}
	});

	// Auto-load artifact content when the selected artifact path changes
	$effect(() => {
		const path = navigationStore.selectedArtifactPath;
		const activity = navigationStore.activeActivity;
		if (!path) return;

		if (activity === "docs") {
			artifactStore.loadDoc(path);
		} else if (activity === "research") {
			artifactStore.loadResearch(path);
		} else if (activity === "plans") {
			artifactStore.loadPlan(path);
		} else if (activityToArtifactType[activity]) {
			artifactStore.loadGovernanceArtifact(path);
		}
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

			<!-- Nav Sub-Panel (collapsible, 200px) -->
			{#if navigationStore.showNavPanel}
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
							{:else if navigationStore.activeActivity === "lessons"}
								<LessonsPanel />
							{:else if navigationStore.activeActivity === "chat"}
								<WelcomeScreen />
							{:else if navigationStore.isArtifactActivity}
								{#if navigationStore.explorerView === "artifact-viewer"}
									<ArtifactViewer />
								{:else}
									<ArtifactLanding category={navigationStore.activeActivity} />
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
