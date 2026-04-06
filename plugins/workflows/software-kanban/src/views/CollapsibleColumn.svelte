<!-- CollapsibleColumn: a kanban column that can be collapsed to a thin vertical bar.
     Drag-and-drop events are forwarded to parent via onDragOver/onDrop props. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { untrack } from "svelte";
	import {
		Icon,
		Panel,
		SectionHeader,
		ScrollArea,
		Badge,
		Button,
		Caption,
		CollapsibleRoot,
		CollapsibleTrigger,
		CollapsibleContent,
		HStack,
		Stack,
		Box,
		CountBadge,
		VerticalText,
		Center,
	} from "@orqastudio/svelte-components/pure";

	let {
		title,
		count,
		doneCount,
		totalCount,
		collapsed = true,
		isDone = false,
		onDragOver,
		onDrop,
		children,
	}: {
		title: string;
		count: number;
		doneCount?: number;
		totalCount?: number;
		collapsed?: boolean;
		isDone?: boolean;
		onDragOver?: (e: DragEvent) => void;
		onDrop?: (e: DragEvent) => void;
		children: Snippet;
	} = $props();

	// isOpen is seeded from the inverse of `collapsed` on mount.
	// Subsequent prop changes do not re-sync — the component owns collapse state.
	// untrack() prevents the Svelte state_referenced_locally warning.
	let isOpen = $state(untrack(() => !collapsed));
	let isDragOver = $state(false);

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragOver = true;
		onDragOver?.(e);
	}

	function handleDragLeave(e: DragEvent) {
		// Only reset isDragOver when the cursor actually leaves the column, not when
		// it moves between child elements. relatedTarget is the element the cursor
		// is entering — if it's still inside the column, ignore the event.
		const related = e.relatedTarget as Node | null;
		if (related && (e.currentTarget as HTMLElement).contains(related)) return;
		isDragOver = false;
	}

	function handleDrop(e: DragEvent) {
		e.stopPropagation();
		isDragOver = false;
		onDrop?.(e);
	}
</script>

<!-- Outer wrapper provides h-full flex-col layout for CollapsibleRoot. -->
<Stack gap={0} height="full">
	<CollapsibleRoot bind:open={isOpen}>
		{#if !isOpen}
			<!-- Collapsed: thin vertical bar acts as the trigger. -->
			<!-- collapsed-bar class applies the column bar appearance via scoped CSS. -->
			<div
				class="collapsed-bar"
				class:drag-over={isDragOver}
				ondragover={handleDragOver}
				ondragleave={handleDragLeave}
				ondrop={handleDrop}
				role="region"
				aria-label="{title} column (collapsed)"
			>
				<CollapsibleTrigger aria-label="Expand {title} column">
					<Center flex={1} gap={2}>
						<VerticalText variant="caption" tone="muted">{title}</VerticalText>
						{#if count > 0}
							<CountBadge {count} variant={isDone ? "success" : "muted"} />
						{/if}
					</Center>
				</CollapsibleTrigger>
			</div>
		{:else}
			<!-- Expanded: full column. column-expanded class applies border/bg/layout via scoped CSS. -->
			<div
				class="column-expanded"
				class:drag-over={isDragOver}
				ondragover={handleDragOver}
				ondragleave={handleDragLeave}
				ondrop={handleDrop}
				role="region"
				aria-label="{title} column"
			>
				<!-- Column header -->
				<SectionHeader>
					{#snippet start()}
						<HStack gap={2} align="center">
							<Badge variant="outline" size="sm" capitalize>
								{title}
							</Badge>
							{#if doneCount !== undefined && totalCount !== undefined}
								<Caption variant="caption-tabular">{doneCount}/{totalCount} Done</Caption>
							{/if}
						</HStack>
					{/snippet}
					{#snippet end()}
						{#if isDone}
							<CollapsibleTrigger>
								<Button
									variant="ghost"
									size="icon-sm"
									aria-label="Collapse {title} column"
								>
									<Icon name="chevron-right" size="sm" />
								</Button>
							</CollapsibleTrigger>
						{/if}
					{/snippet}
				</SectionHeader>

				<!-- Column content -->
				<CollapsibleContent>
					<Box minHeight={0} flex={1}>
						<ScrollArea full orientation="vertical">
							<Panel padding="tight">
							<Stack gap={2} role="list">
								{@render children()}
							</Stack>
							</Panel>
						</ScrollArea>
					</Box>
				</CollapsibleContent>
			</div>
		{/if}
	</CollapsibleRoot>
</Stack>

<style>
	/* Thin vertical bar appearance for collapsed column state.
	   Writing-mode and complex flex layout cannot be expressed as typed props
	   on CollapsibleTrigger, so scoped CSS is used here. */
	.collapsed-bar {
		display: flex;
		width: 2.5rem;
		flex-shrink: 0;
		flex-direction: column;
		align-items: center;
		border-radius: 0.5rem;
		border: 1px dashed var(--color-border);
		background-color: color-mix(in srgb, var(--color-muted) 30%, transparent);
		transition: background-color 0.15s;
		cursor: pointer;
		height: 100%;
	}

	.collapsed-bar:hover {
		background-color: color-mix(in srgb, var(--color-muted) 50%, transparent);
	}

	.collapsed-bar.drag-over {
		border-color: var(--color-primary);
		background-color: color-mix(in srgb, var(--color-primary) 10%, transparent);
	}

	/* Full column container: sets min-width, flex layout, rounded border, and bg. */
	.column-expanded {
		display: flex;
		min-width: 14rem;
		flex: 1;
		flex-direction: column;
		border-radius: 0.5rem;
		border: 1px solid var(--color-border);
		background-color: color-mix(in srgb, var(--color-muted) 10%, transparent);
		transition: border-color 0.15s, background-color 0.15s;
	}

	.column-expanded.drag-over {
		border-color: var(--color-primary);
		background-color: color-mix(in srgb, var(--color-primary) 5%, transparent);
	}
</style>
