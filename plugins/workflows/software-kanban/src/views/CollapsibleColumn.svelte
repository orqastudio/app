<!-- CollapsibleColumn: a kanban column that can be collapsed to a thin vertical bar.
     Drag-and-drop events are forwarded to parent via onDragOver/onDrop props. -->
<script lang="ts">
	import type { Snippet } from "svelte";
	import { untrack } from "svelte";
	import { cn } from "@orqastudio/svelte-components";
	import {
		Icon,
		ScrollArea,
		Badge,
		Button,
		Caption,
		CollapsibleRoot,
		CollapsibleTrigger,
		CollapsibleContent,
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

<CollapsibleRoot bind:open={isOpen} class="flex h-full flex-col">
	{#if !isOpen}
		<!-- Collapsed: thin vertical bar acts as the trigger -->
		<CollapsibleTrigger
			class={cn(
				"flex w-10 shrink-0 flex-col items-center rounded-lg border border-dashed border-border bg-muted/30 transition-colors hover:bg-muted/50 cursor-pointer h-full",
				isDragOver && "border-primary bg-primary/10",
			)}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
			aria-label="Expand {title} column"
		>
			<div class="flex flex-1 flex-col items-center justify-center gap-2 py-4">
				<!-- Rotated title using a raw span — writing-mode cannot be set via className -->
				<span
					class="text-xs text-muted-foreground select-none"
					style="writing-mode: vertical-rl; transform: rotate(180deg);"
				>
					{title}
				</span>
				{#if count > 0}
					<span
						class={cn(
							"flex h-5 w-5 items-center justify-center rounded-full text-[10px] font-semibold tabular-nums",
							isDone
								? "bg-emerald-500/20 text-emerald-700 dark:text-emerald-400"
								: "bg-muted text-muted-foreground",
						)}
					>
						{count}
					</span>
				{/if}
			</div>
		</CollapsibleTrigger>
	{:else}
		<!-- Expanded: full column -->
		<div
			class={cn(
				"flex min-w-56 flex-1 flex-col rounded-lg border border-border bg-muted/10 transition-colors",
				isDragOver && "border-primary bg-primary/5",
			)}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
			role="region"
			aria-label="{title} column"
		>
			<!-- Column header -->
			<div class="flex items-center justify-between border-b border-border px-3 py-2">
				<div class="flex items-center gap-2">
					<Badge variant="outline" class="text-xs font-semibold capitalize">
						{title}
					</Badge>
					{#if doneCount !== undefined && totalCount !== undefined}
						<Caption class="tabular-nums">{doneCount}/{totalCount} Done</Caption>
					{/if}
				</div>
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
			</div>

			<!-- Column content -->
			<CollapsibleContent class="min-h-0 flex-1">
				<ScrollArea class="h-full" orientation="vertical">
					<div class="flex flex-col gap-2 p-2" role="list">
						{@render children()}
					</div>
				</ScrollArea>
			</CollapsibleContent>
		</div>
	{/if}
</CollapsibleRoot>
