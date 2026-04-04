<!-- MilestoneCard: displays a milestone with epic progress, in-progress, and critical epics. -->
<script lang="ts">
	import type { ArtifactNode } from "@orqastudio/types";
	import { StatusIndicator } from "@orqastudio/svelte-components/connected";
	import {
		SmallBadge,
		Button,
		Text,
		Caption,
		ProgressBar,
		HStack,
		Stack,
		Box,
		Dot,
	} from "@orqastudio/svelte-components/pure";

	let {
		milestone,
		epicCount,
		doneEpicCount,
		inProgressEpics,
		criticalEpics,
		epicLabel = "Epic",
		onClick,
	}: {
		milestone: ArtifactNode;
		epicCount: number;
		doneEpicCount: number;
		inProgressEpics: ArtifactNode[];
		criticalEpics: ArtifactNode[];
		/** Display label for the level-1 type (e.g. "Epic"). Used in progress text. */
		epicLabel?: string;
		onClick: () => void;
	} = $props();

	const epicLabelPlural = $derived(`${epicLabel.toLowerCase()}s`);
</script>

<Button
	variant="card"
	onclick={onClick}
>
	<Stack gap={3}>
		<!-- Header -->
		<HStack gap={3} align="start">
			<Stack gap={1} minHeight={0} flex={1}>
				<Text variant="body-strong" truncate>{milestone.title}</Text>
				{#if milestone.description}
					<Caption lineClamp={2}>{milestone.description}</Caption>
				{/if}
			</Stack>
			<Box flex={0}>
				<StatusIndicator status={milestone.status ?? "planning"} mode="badge" />
			</Box>
		</HStack>

		<!-- Progress -->
		{#if epicCount > 0}
			<ProgressBar
				label={`${doneEpicCount}/${epicCount} ${epicLabelPlural}`}
				current={doneEpicCount}
				total={epicCount}
				colorClass="bg-emerald-500"
			/>
		{:else}
			<Caption>No {epicLabelPlural} yet</Caption>
		{/if}

		<!-- In-progress epics -->
		{#if inProgressEpics.length > 0}
			<Stack gap={1} borderTop paddingTop={2}>
				<Text variant="overline-muted">Now</Text>
				{#each inProgressEpics.slice(0, 2) as epic (epic.id)}
					<HStack gap={1} align="center">
						<Dot size="sm" color="info" />
						<Caption truncate>{epic.title}</Caption>
					</HStack>
				{/each}
				{#if inProgressEpics.length > 2}
					<Caption tone="muted">+{inProgressEpics.length - 2} more</Caption>
				{/if}
			</Stack>
		{/if}

		<!-- Critical P1 epics not done -->
		{#if criticalEpics.length > 0}
			<Stack gap={1} borderTop paddingTop={2}>
				<Text variant="overline-muted">Critical</Text>
				{#each criticalEpics.slice(0, 2) as epic (epic.id)}
					<HStack gap={1} align="center">
						<SmallBadge variant="destructive">P1</SmallBadge>
						<Caption truncate>{epic.title}</Caption>
					</HStack>
				{/each}
				{#if criticalEpics.length > 2}
					<Caption tone="muted">+{criticalEpics.length - 2} more</Caption>
				{/if}
			</Stack>
		{/if}
	</Stack>
</Button>
