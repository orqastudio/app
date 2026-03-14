<script lang="ts">
	import type { Snippet } from "svelte";
	import { untrack } from "svelte";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import * as ScrollArea from "$lib/components/ui/scroll-area";
	import { Badge } from "$lib/components/ui/badge";
	import { cn } from "$lib/utils";

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

	// isCollapsed is seeded from the `collapsed` prop on mount.
	// Subsequent prop changes do not re-sync — the component owns collapse state.
	// untrack() prevents the Svelte state_referenced_locally warning.
	let isCollapsed = $state(untrack(() => collapsed));
	let isDragOver = $state(false);

	function handleToggle() {
		isCollapsed = !isCollapsed;
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragOver = true;
		onDragOver?.(e);
	}

	function handleDragLeave() {
		isDragOver = false;
	}

	function handleDrop(e: DragEvent) {
		isDragOver = false;
		onDrop?.(e);
	}
</script>

{#if isCollapsed}
	<!-- Collapsed: thin vertical bar -->
	<div
		class={cn(
			"flex w-10 shrink-0 flex-col items-center rounded-lg border border-dashed border-border bg-muted/30 transition-colors cursor-pointer hover:bg-muted/50",
			isDragOver && "border-primary bg-primary/10",
		)}
		onclick={handleToggle}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
		role="button"
		tabindex="0"
		onkeydown={(e) => e.key === "Enter" && handleToggle()}
		aria-label="Expand {title} column"
	>
		<!-- Rotated title + count -->
		<div class="flex flex-1 flex-col items-center justify-center gap-2 py-4">
			<span
				class="text-xs font-medium text-muted-foreground select-none"
				style="writing-mode: vertical-rl; transform: rotate(180deg);"
			>
				{title}
			</span>
			{#if count > 0}
				<span
					class={cn(
						"flex h-5 w-5 items-center justify-center rounded-full text-[10px] font-semibold tabular-nums",
						isDone ? "bg-emerald-500/20 text-emerald-700 dark:text-emerald-400" : "bg-muted text-muted-foreground",
					)}
				>
					{count}
				</span>
			{/if}
		</div>
	</div>
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
					<span class="text-[10px] tabular-nums text-muted-foreground">
						{doneCount}/{totalCount} Done
					</span>
				{/if}
			</div>
			{#if isDone}
				<button
					class="rounded p-0.5 text-muted-foreground hover:text-foreground transition-colors"
					onclick={handleToggle}
					aria-label="Collapse {title} column"
				>
					<ChevronRightIcon class="h-3.5 w-3.5" />
				</button>
			{/if}
		</div>

		<!-- Column content -->
		<ScrollArea.Root class="min-h-0 flex-1" orientation="vertical">
			<div
				class="flex flex-col gap-2 p-2"
				role="list"
				ondragover={handleDragOver}
				ondragleave={handleDragLeave}
				ondrop={handleDrop}
			>
				{@render children()}
			</div>
		</ScrollArea.Root>
	</div>
{/if}
