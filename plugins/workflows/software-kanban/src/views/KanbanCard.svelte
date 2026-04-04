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
			<HStack gap={2} align="center" class="min-w-0 flex-1">
				<StatusIndicator status={node.status ?? "captured"} mode="dot" />
				<Text size="sm" class="truncate font-medium">{node.title}</Text>
			</HStack>
			{#if node.priority}
				<SmallBadge variant={priorityVariant(node.priority)}>
					{node.priority}
				</SmallBadge>
			{/if}
		</HStack>

		<!-- Description -->
		{#if node.description}
			<Caption class="line-clamp-2">{node.description}</Caption>
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
			<Caption class="font-mono opacity-60">{node.id}</Caption>
		</HStack>
	</Stack>
{/snippet}

{#if onClick}
	<Button
		variant="ghost"
		class="h-auto w-full rounded-lg border border-border bg-card p-3 text-left hover:bg-accent/50 hover:border-border/80"
		draggable={onDragStart !== undefined}
		ondragstart={onDragStart}
		onclick={onClick}
	>
		{@render cardContent()}
	</Button>
{:else}
	<CardRoot
		class="w-full cursor-grab rounded-lg p-3 active:cursor-grabbing"
		draggable={onDragStart !== undefined}
		ondragstart={onDragStart}
		role="listitem"
	>
		{@render cardContent()}
	</CardRoot>
{/if}
