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
	variant="ghost"
	class="group h-auto w-full rounded-xl border border-border bg-card p-4 text-left hover:border-border/80 hover:bg-accent/40 hover:shadow-sm"
	onclick={onClick}
>
	<Stack gap={3}>
		<!-- Header -->
		<HStack gap={3} align="start">
			<Stack gap={1} class="min-w-0 flex-1">
				<Text size="sm" class="truncate font-semibold leading-tight">{milestone.title}</Text>
				{#if milestone.description}
					<Caption class="line-clamp-2">{milestone.description}</Caption>
				{/if}
			</Stack>
			<div class="shrink-0">
				<StatusIndicator status={milestone.status ?? "planning"} mode="badge" />
			</div>
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
			<Stack gap={1} class="border-t border-border/50 pt-2">
				<Caption class="uppercase tracking-wide">Now</Caption>
				{#each inProgressEpics.slice(0, 2) as epic (epic.id)}
					<HStack gap={1} align="center">
						<span class="block h-1.5 w-1.5 shrink-0 rounded-full bg-blue-500"></span>
						<Caption class="truncate">{epic.title}</Caption>
					</HStack>
				{/each}
				{#if inProgressEpics.length > 2}
					<Caption class="opacity-60">+{inProgressEpics.length - 2} more</Caption>
				{/if}
			</Stack>
		{/if}

		<!-- Critical P1 epics not done -->
		{#if criticalEpics.length > 0}
			<Stack gap={1} class="border-t border-border/50 pt-2">
				<Caption class="uppercase tracking-wide">Critical</Caption>
				{#each criticalEpics.slice(0, 2) as epic (epic.id)}
					<HStack gap={1} align="center">
						<SmallBadge variant="destructive">P1</SmallBadge>
						<Caption class="truncate">{epic.title}</Caption>
					</HStack>
				{/each}
				{#if criticalEpics.length > 2}
					<Caption class="opacity-60">+{criticalEpics.length - 2} more</Caption>
				{/if}
			</Stack>
		{/if}
	</Stack>
</Button>
