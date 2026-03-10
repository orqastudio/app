<script lang="ts" module>
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import ClockIcon from "@lucide/svelte/icons/clock";
	import FileEditIcon from "@lucide/svelte/icons/file-edit";
	import AlertCircleIcon from "@lucide/svelte/icons/alert-circle";
	import ArchiveIcon from "@lucide/svelte/icons/archive";
	import ArrowUpCircleIcon from "@lucide/svelte/icons/arrow-up-circle";
	import type { Component } from "svelte";
	import type { BadgeVariant } from "$lib/components/ui/badge";

	type StatusGroup = "active" | "success" | "draft" | "warning" | "promoted" | "inactive";

	const STATUS_GROUP_MAP: Record<string, StatusGroup> = {
		active: "active",
		"in-progress": "active",
		exploring: "active",
		ready: "active",
		done: "success",
		complete: "success",
		accepted: "success",
		shaped: "success",
		draft: "draft",
		captured: "draft",
		todo: "draft",
		proposed: "draft",
		planning: "draft",
		review: "warning",
		recurring: "warning",
		promoted: "promoted",
		inactive: "inactive",
		superseded: "inactive",
		deprecated: "inactive",
		archived: "inactive",
		surpassed: "inactive",
	};

	const GROUP_ICONS: Record<StatusGroup, Component> = {
		active: ClockIcon,
		success: CheckCircleIcon,
		draft: FileEditIcon,
		warning: AlertCircleIcon,
		promoted: ArrowUpCircleIcon,
		inactive: ArchiveIcon,
	};

	/** Dot colour classes per status group (Tailwind bg-* utility). */
	const GROUP_DOT_CLASSES: Record<StatusGroup, string> = {
		active: "bg-blue-500",
		success: "bg-emerald-500",
		draft: "bg-muted-foreground/50",
		warning: "bg-amber-500",
		promoted: "bg-purple-500",
		inactive: "bg-destructive/60",
	};

	/** Badge variant per status group. */
	const GROUP_BADGE_VARIANTS: Record<StatusGroup, BadgeVariant> = {
		active: "default",
		success: "outline",
		draft: "secondary",
		warning: "warning",
		promoted: "secondary",
		inactive: "destructive",
	};

	/** Extra Tailwind classes applied to badge for promoted group (no built-in variant). */
	const GROUP_BADGE_EXTRA: Partial<Record<StatusGroup, string>> = {
		promoted: "bg-purple-500/15 text-purple-700 dark:text-purple-300 border-purple-500/30",
		success: "border-emerald-500/30 text-emerald-700 dark:text-emerald-400",
	};

	function resolveGroup(status: string): StatusGroup {
		return STATUS_GROUP_MAP[status.toLowerCase()] ?? "draft";
	}

	/** Returns the Badge variant for the given status string. */
	export function statusVariant(status: string): BadgeVariant {
		return GROUP_BADGE_VARIANTS[resolveGroup(status)];
	}

	/** Returns Tailwind dot colour classes for the given status string. */
	export function statusColor(status: string): string {
		return GROUP_DOT_CLASSES[resolveGroup(status)];
	}
</script>

<script lang="ts">
	import { Badge } from "$lib/components/ui/badge";
	import { cn } from "$lib/utils";

	let {
		status,
		mode = "badge",
	}: {
		status: string;
		mode?: "badge" | "dot" | "inline";
	} = $props();

	const group = $derived(resolveGroup(status));
	const Icon = $derived(GROUP_ICONS[group]);
	const dotClass = $derived(GROUP_DOT_CLASSES[group]);
	const badgeVariant = $derived(GROUP_BADGE_VARIANTS[group]);
	const badgeExtra = $derived(GROUP_BADGE_EXTRA[group] ?? "");
</script>

{#if mode === "dot"}
	<span
		class={cn("inline-block h-2 w-2 shrink-0 rounded-full", dotClass)}
		title={status}
	></span>
{:else if mode === "inline"}
	<span class="inline-flex items-center gap-1 text-xs">
		<Icon class="h-3.5 w-3.5 shrink-0" />
		<span class="capitalize">{status}</span>
	</span>
{:else}
	<Badge variant={badgeVariant} class="{badgeExtra} capitalize">
		<Icon class="h-3 w-3 shrink-0" />{status}
	</Badge>
{/if}
