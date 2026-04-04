<script lang="ts">
	import { Icon, CardRoot, CardHeader, CardTitle, CardContent, CardAction, Heading } from "@orqastudio/svelte-components/pure";
	import { TooltipRoot, TooltipTrigger, TooltipContent } from "@orqastudio/svelte-components/pure";
	import { getStores } from "@orqastudio/sdk";

	const { artifactGraphSDK, navigationStore } = getStores();
	import type { ArtifactNode } from "@orqastudio/types";

	// -------------------------------------------------------------------------
	// Derive active milestone and its P1 epic progress
	// -------------------------------------------------------------------------

	interface MilestoneProgress {
		node: ArtifactNode;
		gate: string | null;
		deadline: string | null;
		p1Total: number;
		p1Done: number;
	}

	const activeMilestone = $derived.by((): MilestoneProgress | null => {
		const milestones = artifactGraphSDK.byType("milestone");
		const active = milestones.find((m) => m.status === "active");
		if (!active) return null;

		// Collect epic IDs referenced by this milestone via "contains" relationships
		const epicIds = active.references_out
			.filter((ref) => ref.relationship_type === "contains")
			.map((ref) => ref.target_id);

		// For each epic ID, resolve and count P1 epics
		let p1Total = 0;
		let p1Done = 0;

		for (const epicId of epicIds) {
			const epic = artifactGraphSDK.resolve(epicId);
			if (!epic || epic.artifact_type !== "epic") continue;
			if (epic.priority === "P1") {
				p1Total++;
				if (epic.status === "completed") p1Done++;
			}
		}

		const fm = active.frontmatter as Record<string, unknown>;
		const gate = typeof fm.gate === "string" ? fm.gate : null;
		const deadline = typeof fm.deadline === "string" ? fm.deadline : null;

		return { node: active, gate, deadline, p1Total, p1Done };
	});

	const graphReady = $derived(artifactGraphSDK.graph.size > 0);

	// -------------------------------------------------------------------------
	// Progress bar helpers
	// -------------------------------------------------------------------------

	const progressPercent = $derived.by((): number => {
		if (!activeMilestone || activeMilestone.p1Total === 0) return 0;
		return Math.round((activeMilestone.p1Done / activeMilestone.p1Total) * 100);
	});

	// -------------------------------------------------------------------------
	// Navigation
	// -------------------------------------------------------------------------

	/** Navigate to the roadmap activity to view the active milestone. */
	function openMilestone() {
		navigationStore.setActivity("roadmap");
	}

	/** Navigate to the roadmap activity overview. */
	function openRoadmap() {
		navigationStore.setActivity("roadmap");
	}
</script>

<CardRoot>
	<CardHeader compact>
		<CardTitle>
			<div class="flex items-center gap-2">
				<Icon name="target" size="md" />
				Active Milestone
			</div>
		</CardTitle>
		<CardAction>
			{#if activeMilestone}
				<button class="flex h-7 items-center gap-1 rounded px-2 text-xs hover:bg-accent" onclick={openMilestone}>
					<Icon name="kanban" size="sm" />
					View Roadmap
				</button>
			{:else}
				<button class="flex h-7 items-center gap-1 rounded px-2 text-xs hover:bg-accent" onclick={openRoadmap}>
					<Icon name="map" size="sm" />
					Roadmap
				</button>
			{/if}
		</CardAction>
	</CardHeader>

	<CardContent>
		{#if !graphReady}
			<p class="text-sm text-muted-foreground">Loading artifact graph&hellip;</p>
		{:else if !activeMilestone}
			<p class="text-sm text-muted-foreground">
				No active milestone.
				<button
					class="ml-1 h-auto p-0 text-sm text-muted-foreground underline underline-offset-2"
					onclick={openRoadmap}
				>Open Roadmap</button> to plan one.
			</p>
		{:else}
			<!-- Title + deadline row -->
			<div class="mb-3 flex items-start justify-between gap-4">
				<div>
					<Heading level={2}>{activeMilestone.node.title}</Heading>
					{#if activeMilestone.node.description}
						<p class="mt-0.5 text-sm text-muted-foreground line-clamp-2">
							{activeMilestone.node.description}
						</p>
					{/if}
				</div>
				{#if activeMilestone.deadline}
					<div class="flex shrink-0 items-center gap-1 text-xs text-muted-foreground">
						<Icon name="calendar" size="sm" />
						{activeMilestone.deadline}
					</div>
				{/if}
			</div>

			<!-- Gate question -->
			{#if activeMilestone.gate}
				<div class="mb-4 rounded-md bg-muted/50 py-2">
					<p class="text-xs font-medium text-muted-foreground uppercase tracking-wide mb-1">Gate question</p>
					<p class="text-sm italic">"{activeMilestone.gate}"</p>
				</div>
			{/if}

			<!-- P1 epic progress -->
			{#if activeMilestone.p1Total > 0}
				<div class="space-y-1.5">
					<div class="flex items-center justify-between text-xs">
						<span class="text-muted-foreground">P1 Epics</span>
						<TooltipRoot>
							<TooltipTrigger>
								{#snippet child({ props })}
									<span {...props} class="font-medium tabular-nums">
										{activeMilestone.p1Done}/{activeMilestone.p1Total} done
									</span>
								{/snippet}
							</TooltipTrigger>
							<TooltipContent side="top">
								<p>{progressPercent}% of P1 epics complete</p>
							</TooltipContent>
						</TooltipRoot>
					</div>
					<!-- Progress bar (custom — no shadcn Progress component installed) -->
					<div class="h-2 w-full overflow-hidden rounded-full bg-muted">
						<div
							class="h-full rounded-full bg-primary transition-all duration-500"
							style="width: {progressPercent}%"
						></div>
					</div>
				</div>
			{:else}
				<p class="text-xs text-muted-foreground">No P1 epics defined for this milestone.</p>
			{/if}
		{/if}
	</CardContent>
</CardRoot>
