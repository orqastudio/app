<script lang="ts" module>
	import CircleIcon from "@lucide/svelte/icons/circle";
	import CompassIcon from "@lucide/svelte/icons/compass";
	import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
	import CircleDotDashedIcon from "@lucide/svelte/icons/circle-dot-dashed";
	import CircleUserRoundIcon from "@lucide/svelte/icons/circle-user-round";
	import CircleCheckBigIcon from "@lucide/svelte/icons/circle-check-big";
	import CircleMinusIcon from "@lucide/svelte/icons/circle-minus";
	import CircleFadingArrowUpIcon from "@lucide/svelte/icons/circle-fading-arrow-up";
	import CircleStarIcon from "@lucide/svelte/icons/circle-star";
	import CirclePauseIcon from "@lucide/svelte/icons/circle-pause";
	import CircleStopIcon from "@lucide/svelte/icons/circle-stop";
	import type { Component } from "svelte";

	/** Canonical status groups — the universal vocabulary for artifact state. */
	export type StatusGroup =
		| "captured"
		| "exploring"
		| "prioritised"
		| "queued"
		| "active"
		| "review"
		| "complete"
		| "hold"
		| "blocked"
		| "closed"
		| "recurring";

	const STATUS_GROUP_MAP: Record<string, StatusGroup> = {
		// Captured — exists but not shaped yet
		draft: "captured",
		captured: "captured",
		proposed: "captured",
		planning: "captured",
		// Exploring — under investigation
		exploring: "exploring",
		// Prioritised — human-marked as important
		prioritised: "prioritised",
		// Queued — shaped and waiting
		todo: "queued",
		ready: "queued",
		shaped: "queued",
		// Active — work in progress
		"in-progress": "active",
		// Review — needs human attention
		review: "review",
		"action-needed": "review",
		// Complete — done
		done: "complete",
		complete: "complete",
		accepted: "complete",
		promoted: "complete",
		active: "complete",
		// Hold — paused
		hold: "hold",
		"on-hold": "hold",
		// Blocked — can't proceed
		blocked: "blocked",
		// Closed — no longer active
		inactive: "closed",
		archived: "closed",
		surpassed: "closed",
		superseded: "closed",
		deprecated: "closed",
		// Recurring — pattern detected
		recurring: "recurring",
	};

	const GROUP_ICONS: Record<StatusGroup, Component> = {
		unshaped: CircleIcon,
		exploring: CompassIcon,
		prioritised: CircleStarIcon,
		queued: CircleDotIcon,
		active: CircleDotDashedIcon,
		review: CircleUserRoundIcon,
		complete: CircleCheckBigIcon,
		hold: CirclePauseIcon,
		blocked: CircleStopIcon,
		closed: CircleMinusIcon,
		recurring: CircleFadingArrowUpIcon,
	};

	const GROUP_LABELS: Record<StatusGroup, string> = {
		unshaped: "Captured",
		exploring: "Exploring",
		prioritised: "Prioritised",
		queued: "Queued",
		active: "Active",
		review: "Needs Review",
		complete: "Complete",
		hold: "On Hold",
		blocked: "Blocked",
		closed: "Closed",
		recurring: "Recurring",
	};

	function resolveGroup(status: string): StatusGroup {
		return STATUS_GROUP_MAP[status.toLowerCase()] ?? "captured";
	}

	/** Returns the Lucide icon component for the given status string. */
	export function statusIcon(status: string): Component {
		return GROUP_ICONS[resolveGroup(status)];
	}

	/** Returns the group label (e.g. "Active", "Complete") for the given status string. */
	export function statusLabel(status: string): string {
		return GROUP_LABELS[resolveGroup(status)];
	}

	/** Returns true if this status should show a spinning animation. */
	export function statusIsSpinning(status: string): boolean {
		return resolveGroup(status) === "active";
	}
</script>

<script lang="ts">
	import { cn } from "$lib/utils";

	let {
		status,
		mode = "badge",
	}: {
		status: string;
		mode?: "badge" | "dot" | "inline";
	} = $props();

	const Icon = $derived(statusIcon(status));
	const group = $derived(resolveGroup(status));
	const isSpinning = $derived(group === "active");
</script>

{#if mode === "dot"}
	<Icon class={cn("inline-block h-3.5 w-3.5 shrink-0 text-muted-foreground", isSpinning && "status-spin")} />
{:else if mode === "inline"}
	<span class="inline-flex items-center gap-1 text-xs text-muted-foreground">
		<Icon class={cn("h-3.5 w-3.5 shrink-0", isSpinning && "status-spin")} />
		<span class="capitalize">{status}</span>
	</span>
{:else}
	<span class="inline-flex items-center gap-1.5 rounded border border-border bg-muted/30 px-1.5 py-0.5 text-xs capitalize text-muted-foreground">
		<Icon class={cn("h-3 w-3 shrink-0", isSpinning && "status-spin")} />{status}
	</span>
{/if}

<style>
	:global(.status-spin) {
		animation: status-spin 4s linear infinite;
	}
	@keyframes status-spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>
