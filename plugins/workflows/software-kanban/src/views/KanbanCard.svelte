<!-- KanbanCard: displays a single artifact as a draggable/clickable kanban card. -->
<script lang="ts">
	import type { ArtifactNode } from "@orqastudio/types";
	import { StatusIndicator } from "@orqastudio/svelte-components/connected";
	import {
		SmallBadge,
		CardRoot,
		Button,
		Text,
		Caption,
		ProgressBar,
		HStack,
		Stack,
	} from "@orqastudio/svelte-components/pure";
	import type { BadgeVariant } from "@orqastudio/svelte-components/pure";

	let {
		node,
		taskCount,
		onClick,
		onDragStart,
	}: {
		node: ArtifactNode;
		taskCount?: { done: number; total: number };
		onClick?: () => void;
		onDragStart?: (e: DragEvent) => void;
	} = $props();

	/**
	 * Maps a priority string to the corresponding badge variant for visual distinction.
	 * @param priority - The artifact priority string (e.g. "P1", "P2") or null if unset.
	 * @returns The BadgeVariant to use for the priority badge.
	 */
	function priorityVariant(priority: string | null): BadgeVariant {
		if (priority === "P1") return "destructive";
		if (priority === "P2") return "default";
		return "secondary";
	}
</script>

{#snippet cardContent()}
	<Stack gap={2}>
		<!-- Title row -->
		<HStack gap={2} align="start">
			<HStack gap={2} align="center" minHeight={0} flex={1}>
				<StatusIndicator status={node.status ?? "captured"} mode="dot" />
				<Text variant="body-strong" truncate>{node.title}</Text>
			</HStack>
			{#if node.priority}
				<SmallBadge variant={priorityVariant(node.priority)}>
					{node.priority}
				</SmallBadge>
			{/if}
		</HStack>

		<!-- Description -->
		{#if node.description}
			<Caption lineClamp={2}>{node.description}</Caption>
		{/if}

		<!-- Task progress bar -->
		{#if taskCount && taskCount.total > 0}
			<ProgressBar
				label=""
				current={taskCount.done}
				total={taskCount.total}
				colorClass="bg-emerald-500"
			/>
		{/if}

		<!-- ID chip + project badge -->
		<HStack gap={1} align="center">
			{#if node.project}
				<SmallBadge variant="secondary">{node.project}</SmallBadge>
			{/if}
			<Caption variant="caption-mono">{node.id}</Caption>
		</HStack>
	</Stack>
{/snippet}

{#if onClick}
	<Button
		variant="card"
		draggable={onDragStart !== undefined}
		ondragstart={onDragStart}
		onclick={onClick}
	>
		{@render cardContent()}
	</Button>
{:else}
	<!-- Raw div wrapper provides drag cursor and drag events; CardRoot provides visual card appearance. -->
	<div
		class="drag-wrapper"
		draggable={onDragStart !== undefined}
		ondragstart={onDragStart}
		role="listitem"
	>
		<CardRoot>
			{@render cardContent()}
		</CardRoot>
	</div>
{/if}

<style>
	/* Cursor styling for draggable card list items. The drag-wrapper is a raw div
	   because HTML5 drag API requires a native element, and cursor-grab cannot be
	   expressed as a typed prop on CardRoot. */
	.drag-wrapper {
		cursor: grab;
		width: 100%;
	}
	.drag-wrapper:active {
		cursor: grabbing;
	}
</style>
