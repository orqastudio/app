<script lang="ts">
	import {
		Icon,
		CardRoot,
		CardHeader,
		CardTitle,
		CardContent,
		CardAction,
		Heading,
		Button,
		HStack,
		Stack,
		Text,
		Caption,
		ProgressBar,
	} from "@orqastudio/svelte-components/pure";
	import { Panel } from "@orqastudio/svelte-components/pure";
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
			<HStack gap={2}>
				<Icon name="target" size="md" />
				Active Milestone
			</HStack>
		</CardTitle>
		<CardAction>
			{#if activeMilestone}
				<Button variant="ghost" size="sm" onclick={openMilestone}>
					<Icon name="kanban" size="sm" />
					View Roadmap
				</Button>
			{:else}
				<Button variant="ghost" size="sm" onclick={openRoadmap}>
					<Icon name="map" size="sm" />
					Roadmap
				</Button>
			{/if}
		</CardAction>
	</CardHeader>

	<CardContent>
		{#if !graphReady}
			<Text variant="body-muted" block>Loading artifact graph&hellip;</Text>
		{:else if !activeMilestone}
			<Text variant="body-muted" block>
				No active milestone.
				<Button variant="ghost" size="sm" onclick={openRoadmap}>Open Roadmap</Button> to plan one.
			</Text>
		{:else}
			<!-- Title + deadline row + gate question wrapped in a Stack for uniform spacing -->
			<Stack gap={3}>
				<HStack justify="between" align="start" gap={4}>
					<Stack gap={0} flex={1}>
						<Heading level={2}>{activeMilestone.node.title}</Heading>
						{#if activeMilestone.node.description}
							<Text variant="body-muted" lineClamp={2}>{activeMilestone.node.description}</Text>
						{/if}
					</Stack>
					{#if activeMilestone.deadline}
						<HStack gap={1} align="center">
							<Icon name="calendar" size="sm" />
							<Caption>{activeMilestone.deadline}</Caption>
						</HStack>
					{/if}
				</HStack>

				<!-- Gate question -->
				{#if activeMilestone.gate}
					<Panel padding="tight" background="muted" rounded="md">
						<Text variant="overline-muted" block>Gate question</Text>
						<Text variant="body" block>"{activeMilestone.gate}"</Text>
					</Panel>
				{/if}
			</Stack>

			<!-- P1 epic progress -->
			{#if activeMilestone.p1Total > 0}
				<ProgressBar
					label="P1 Epics"
					current={activeMilestone.p1Done}
					total={activeMilestone.p1Total}
				/>
			{:else}
				<Caption>No P1 epics defined for this milestone.</Caption>
			{/if}
		{/if}
	</CardContent>
</CardRoot>
