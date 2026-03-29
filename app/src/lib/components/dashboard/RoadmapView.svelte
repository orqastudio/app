<!-- Roadmap view — shows the active milestone and a list of all milestones.
     This is a stub that surfaces MilestoneContextCard in a full-page layout.
     Full roadmap planning (multi-milestone timeline, drag-drop sequencing)
     is post-migration work tracked in IDEA-a3f7c912. -->
<script lang="ts">
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";
	import MilestoneContextCard from "./MilestoneContextCard.svelte";
	import { ARTIFACT_TYPES } from "$lib/config/governance-types";
	import {
		MILESTONE_STATUS_DOT_COLORS,
		MILESTONE_STATUS_DOT_DEFAULT,
		MILESTONE_STATUS_BADGE_COLORS,
		MILESTONE_STATUS_BADGE_DEFAULT,
	} from "$lib/config/milestone-config";

	const { artifactGraphSDK, navigationStore } = getStores();

	// Derive all milestones sorted by deadline then title
	const milestones = $derived.by(() => {
		return artifactGraphSDK.byType(ARTIFACT_TYPES.milestone).sort((a, b) => {
			const da = (a.frontmatter as Record<string, unknown>)?.deadline as string | undefined;
			const db = (b.frontmatter as Record<string, unknown>)?.deadline as string | undefined;
			if (da && db) return da.localeCompare(db);
			if (da) return -1;
			if (db) return 1;
			return a.title.localeCompare(b.title);
		});
	});

	/**
	 * Opens a milestone artifact in the editor panel.
	 * @param path - The file path of the milestone artifact to open.
	 */
	function openMilestone(path: string) {
		navigationStore.openArtifact(path, []);
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
						<button
							class="flex w-full items-center gap-3 rounded-md border border-border bg-card p-3 text-left transition-colors hover:bg-accent/50"
							onclick={() => openMilestone(ms.path)}
						>
							<div class="mt-0.5 h-2.5 w-2.5 shrink-0 rounded-full {MILESTONE_STATUS_DOT_COLORS[status] ?? MILESTONE_STATUS_DOT_DEFAULT}"></div>
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
							<span class="shrink-0 rounded px-1.5 py-0.5 text-xs capitalize {MILESTONE_STATUS_BADGE_COLORS[status] ?? MILESTONE_STATUS_BADGE_DEFAULT}">
								{status}
							</span>
						</button>
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
