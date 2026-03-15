<script lang="ts" module>
	import CircleIcon from "@lucide/svelte/icons/circle";
	import CompassIcon from "@lucide/svelte/icons/compass";
	import CircleDotIcon from "@lucide/svelte/icons/circle-dot";
	import CircleArrowRightIcon from "@lucide/svelte/icons/circle-arrow-right";
	import CircleUserRoundIcon from "@lucide/svelte/icons/circle-user-round";
	import CircleCheckBigIcon from "@lucide/svelte/icons/circle-check-big";
	import CircleMinusIcon from "@lucide/svelte/icons/circle-minus";
	import CircleFadingArrowUpIcon from "@lucide/svelte/icons/circle-fading-arrow-up";
	import CircleStarIcon from "@lucide/svelte/icons/circle-star";
	import CirclePauseIcon from "@lucide/svelte/icons/circle-pause";
	import CircleStopIcon from "@lucide/svelte/icons/circle-stop";
	import type { Component } from "svelte";

	type StatusGroup =
		| "unshaped"
		| "exploring"
		| "prioritised"
		| "waiting"
		| "active"
		| "human-gate"
		| "complete"
		| "on-hold"
		| "blocked"
		| "closed"
		| "recurring";

	const STATUS_GROUP_MAP: Record<string, StatusGroup> = {
		// Unshaped — draft/initial states
		draft: "unshaped",
		captured: "unshaped",
		proposed: "unshaped",
		planning: "unshaped",
		// Exploring — investigation
		exploring: "exploring",
		// Prioritised — human-marked as important
		prioritised: "prioritised",
		// Waiting — staged/ready states
		todo: "waiting",
		ready: "waiting",
		shaped: "waiting",
		// Active — in-flight
		"in-progress": "active",
		// Human gate — needs human attention
		review: "human-gate",
		"action-needed": "human-gate",
		// Complete — positive/done states
		done: "complete",
		complete: "complete",
		accepted: "complete",
		promoted: "complete",
		active: "complete",
		// On hold — paused
		"on-hold": "on-hold",
		// Blocked — can't proceed
		blocked: "blocked",
		// Closed — inactive/historical states
		inactive: "closed",
		archived: "closed",
		surpassed: "closed",
		superseded: "closed",
		deprecated: "closed",
		// Recurring
		recurring: "recurring",
	};

	const GROUP_ICONS: Record<StatusGroup, Component> = {
		unshaped: CircleIcon,
		exploring: CompassIcon,
		prioritised: CircleStarIcon,
		waiting: CircleDotIcon,
		active: CircleArrowRightIcon,
		"human-gate": CircleUserRoundIcon,
		complete: CircleCheckBigIcon,
		"on-hold": CirclePauseIcon,
		blocked: CircleStopIcon,
		closed: CircleMinusIcon,
		recurring: CircleFadingArrowUpIcon,
	};

	const GROUP_LABELS: Record<StatusGroup, string> = {
		unshaped: "Unshaped",
		exploring: "Exploring",
		prioritised: "Prioritised",
		waiting: "Queued",
		active: "Active",
		"human-gate": "Needs review",
		complete: "Complete",
		"on-hold": "On Hold",
		blocked: "Blocked",
		closed: "Closed",
		recurring: "Recurring",
	};

	function resolveGroup(status: string): StatusGroup {
		return STATUS_GROUP_MAP[status.toLowerCase()] ?? "unshaped";
	}

	/** Returns the Lucide icon component for the given status string. */
	export function statusIcon(status: string): Component {
		return GROUP_ICONS[resolveGroup(status)];
	}

	/** Returns the group label (e.g. "Active", "Complete") for the given status string. */
	export function statusLabel(status: string): string {
		return GROUP_LABELS[resolveGroup(status)];
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
</script>

{#if mode === "dot"}
	<Icon class={cn("inline-block h-3.5 w-3.5 shrink-0 text-muted-foreground")} />
{:else if mode === "inline"}
	<span class="inline-flex items-center gap-1 text-xs text-muted-foreground">
		<Icon class="h-3.5 w-3.5 shrink-0" />
		<span class="capitalize">{status}</span>
	</span>
{:else}
	<span class="inline-flex items-center gap-1.5 rounded border border-border bg-muted/30 px-1.5 py-0.5 text-xs capitalize text-muted-foreground">
		<Icon class="h-3 w-3 shrink-0" />{status}
	</span>
{/if}
