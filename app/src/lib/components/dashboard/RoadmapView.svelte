<!-- Roadmap view — shows the active milestone and a list of all milestones,
     each expanded to show its child epics.
     Full roadmap planning (multi-milestone timeline, drag-drop sequencing)
     is post-migration work tracked in IDEA-a3f7c912. -->
<script lang="ts">
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import MilestoneContextCard from "./MilestoneContextCard.svelte";
	import type { ArtifactNode } from "@orqastudio/types";

	const { artifactGraphSDK, navigationStore, projectStore } = getStores();

	/**
	 * Default dot colors by status key for milestone status indicators.
	 * Used when no color is declared on the StatusDefinition.
	 */
	const DEFAULT_STATUS_DOT_COLORS: Record<string, string> = {
		active: "bg-green-500",
		completed: "bg-muted-foreground",
		planned: "bg-blue-400",
	};

	/**
	 * Default badge colors by status key for milestone status badges.
	 * Used when no color is declared on the StatusDefinition.
	 */
	const DEFAULT_STATUS_BADGE_COLORS: Record<string, string> = {
		active: "bg-green-500/20 text-green-700 dark:text-green-400",
	};

	/**
	 * Returns the Tailwind dot color class for a milestone status.
	 * Prefers the color from the project's StatusDefinition when available.
	 * @param status - The milestone status key.
	 * @returns A Tailwind background class string.
	 */
	function statusDotClass(status: string): string {
		const def = projectStore.projectSettings?.statuses?.find((s) => s.key === status);
		if (def?.color) return `bg-[${def.color}]`;
		return DEFAULT_STATUS_DOT_COLORS[status] ?? "bg-muted-foreground";
	}

	/**
	 * Returns the Tailwind badge class string for a milestone status.
	 * Uses default badge colors; extensible when plugin manifests declare status colors.
	 * @param status - The milestone status key.
	 * @returns A Tailwind class string for badge background and text.
	 */
	function statusBadgeClass(status: string): string {
		return DEFAULT_STATUS_BADGE_COLORS[status] ?? "bg-muted text-muted-foreground";
	}

	// Derive all milestones sorted by deadline then title
	const milestones = $derived.by(() => {
		return artifactGraphSDK.byType("milestone").sort((a, b) => {
			const da = (a.frontmatter as Record<string, unknown>)?.deadline as string | undefined;
			const db = (b.frontmatter as Record<string, unknown>)?.deadline as string | undefined;
			if (da && db) return da.localeCompare(db);
			if (da) return -1;
			if (db) return 1;
			return a.title.localeCompare(b.title);
		});
	});

	/**
	 * Return the epics that fulfil a given milestone.
	 *
	 * Epics carry an outgoing "fulfils" edge pointing at their parent milestone.
	 * `traverseIncoming` follows those edges in reverse — from milestone to the
	 * epics that fulfil it.
	 * @param milestoneId - The ID of the milestone artifact.
	 * @returns Epics that fulfil this milestone, sorted by title.
	 */
	function getEpicsForMilestone(milestoneId: string): ArtifactNode[] {
		return artifactGraphSDK
			.traverseIncoming(milestoneId, "fulfils")
			.sort((a, b) => a.title.localeCompare(b.title));
	}

	/**
	 * Opens a milestone artifact in the editor panel.
	 * @param path - The file path of the milestone artifact to open.
	 */
	function openMilestone(path: string) {
		navigationStore.openArtifact(path, []);
	}

	/**
	 * Navigates to an epic artifact by its ID.
	 * @param id - The artifact ID of the epic to navigate to.
	 */
	function openEpic(id: string) {
		navigationStore.navigateToArtifact(id);
	}
</script>

<ScrollArea class="h-full">
	<div class="space-y-6 p-6">
		<div>
			<h1 class="text-2xl font-bold">Roadmap</h1>
			<p class="text-sm text-muted-foreground">Active milestone and delivery plan</p>
		</div>

		<!-- Active milestone card -->
		<MilestoneContextCard />

		<!-- All milestones list -->
		{#if milestones.length > 0}
			<div>
				<h2 class="mb-3 text-sm font-semibold uppercase tracking-wide text-muted-foreground">
					All Milestones
				</h2>
				<div class="space-y-2">
					{#each milestones as ms (ms.path)}
						{@const status = ms.status ?? "planned"}
						{@const deadline = (ms.frontmatter as Record<string, unknown>)?.deadline as string | undefined}
						{@const epics = getEpicsForMilestone(ms.id)}
						<div class="rounded-md border border-border bg-card">
							<!-- Milestone row -->
							<button
								class="flex w-full items-center gap-3 p-3 text-left transition-colors hover:bg-accent/50"
								onclick={() => openMilestone(ms.path)}
							>
								<div class="mt-0.5 h-2.5 w-2.5 shrink-0 rounded-full {statusDotClass(status)}"></div>
								<div class="min-w-0 flex-1">
									<div class="truncate text-sm font-medium">{ms.title}</div>
									{#if ms.description}
										<div class="truncate text-xs text-muted-foreground">{ms.description}</div>
									{/if}
								</div>
								{#if deadline}
									<div class="flex shrink-0 items-center gap-1 text-xs text-muted-foreground">
										<Icon name="calendar" size="sm" />
										{deadline}
									</div>
								{/if}
								<span class="shrink-0 rounded px-1.5 py-0.5 text-xs capitalize {statusBadgeClass(status)}">
									{status}
								</span>
							</button>

							<!-- Child epics -->
							{#if epics.length > 0}
								<div class="border-t border-border">
									{#each epics as epic (epic.id)}
										{@const epicStatus = epic.status ?? "planned"}
										<button
											class="flex w-full items-center gap-3 px-3 py-2 pl-7 text-left text-sm transition-colors hover:bg-accent/50"
											onclick={() => openEpic(epic.id)}
										>
											<span class="shrink-0 text-muted-foreground"><Icon name="layers" size="sm" /></span>
											<div class="min-w-0 flex-1">
												<div class="truncate font-medium">{epic.title}</div>
											</div>
											<span class="shrink-0 rounded px-1.5 py-0.5 text-xs capitalize {statusBadgeClass(epicStatus)}">
												{epicStatus}
											</span>
										</button>
									{/each}
								</div>
							{:else}
								<div class="border-t border-border px-3 py-2 pl-7 text-xs text-muted-foreground">
									No epics yet
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{:else}
			<div class="rounded-md border border-dashed p-8 text-center text-sm text-muted-foreground">
				<Icon name="map" size="xl" />
				<p class="mt-2">No milestones yet.</p>
				<p class="mt-1">Create milestone artifacts in your delivery tree to build a roadmap.</p>
			</div>
		{/if}
	</div>
</ScrollArea>
