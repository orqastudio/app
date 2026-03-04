<script lang="ts">
	import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
	import ChevronUpIcon from "@lucide/svelte/icons/chevron-up";
	import CheckIcon from "@lucide/svelte/icons/check";
	import XIcon from "@lucide/svelte/icons/x";
	import PlayIcon from "@lucide/svelte/icons/play";
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import SmallBadge from "$lib/components/shared/SmallBadge.svelte";
	import type { BadgeVariant } from "$lib/components/ui/badge";
	import type { Recommendation, RecommendationPriority } from "$lib/types/governance";

	interface Props {
		recommendation: Recommendation;
		onApprove: (id: number) => void;
		onReject: (id: number) => void;
		onApply: (id: number) => void;
	}

	const { recommendation, onApprove, onReject, onApply }: Props = $props();

	let expanded = $state(false);

	function priorityVariant(priority: RecommendationPriority): BadgeVariant {
		switch (priority) {
			case "critical":
				return "destructive";
			case "recommended":
				return "default";
			case "optional":
				return "secondary";
			default:
				return "secondary";
		}
	}

	function priorityLabel(priority: RecommendationPriority): string {
		switch (priority) {
			case "critical":
				return "Critical";
			case "recommended":
				return "Recommended";
			case "optional":
				return "Optional";
			default:
				return priority;
		}
	}

	const isApproved = $derived(recommendation.status === "approved");
	const isRejected = $derived(recommendation.status === "rejected");
	const isApplied = $derived(recommendation.status === "applied");
	const isPending = $derived(recommendation.status === "pending");
</script>

<Card.Root
	class="transition-opacity {isRejected ? 'opacity-50' : 'opacity-100'} {isApplied
		? 'border-green-500/40'
		: ''} {isApproved ? 'border-primary/40' : ''}"
>
	<Card.Content class="p-4">
		<div class="space-y-3">
			<!-- Header row -->
			<div class="flex items-start justify-between gap-3">
				<div class="min-w-0 flex-1 space-y-1">
					<div class="flex flex-wrap items-center gap-2">
						<span class="text-sm font-semibold">{recommendation.title}</span>
						<SmallBadge variant={priorityVariant(recommendation.priority)}>
							{priorityLabel(recommendation.priority)}
						</SmallBadge>
						<SmallBadge variant="secondary">
							{recommendation.category}
						</SmallBadge>
					</div>
					<p class="text-sm text-muted-foreground">{recommendation.description}</p>
				</div>

				<!-- Status indicator -->
				<div class="flex-shrink-0">
					{#if isApplied}
						<div class="flex items-center gap-1 text-xs text-green-600">
							<CheckIcon class="h-4 w-4" />
							<span>Applied</span>
						</div>
					{:else if isApproved}
						<div class="flex items-center gap-1 text-xs text-primary">
							<CheckIcon class="h-4 w-4" />
							<span>Approved</span>
						</div>
					{:else if isRejected}
						<div class="flex items-center gap-1 text-xs text-muted-foreground">
							<XIcon class="h-4 w-4" />
							<span>Rejected</span>
						</div>
					{/if}
				</div>
			</div>

			<!-- Action buttons -->
			{#if !isApplied}
				<div class="flex items-center gap-2">
					{#if isPending || isRejected}
						<Button
							size="sm"
							variant="outline"
							class="border-green-500 text-green-700 hover:bg-green-50 dark:text-green-400 dark:hover:bg-green-900/20"
							onclick={() => onApprove(recommendation.id)}
						>
							<CheckIcon class="mr-1 h-3 w-3" />
							Approve
						</Button>
					{/if}

					{#if isPending || isApproved}
						<Button
							size="sm"
							variant="outline"
							class="border-destructive text-destructive hover:bg-destructive/10"
							onclick={() => onReject(recommendation.id)}
						>
							<XIcon class="mr-1 h-3 w-3" />
							Reject
						</Button>
					{/if}

					{#if isApproved}
						<Button size="sm" onclick={() => onApply(recommendation.id)}>
							<PlayIcon class="mr-1 h-3 w-3" />
							Apply
						</Button>
					{/if}

					<button
						class="ml-auto flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground"
						onclick={() => (expanded = !expanded)}
					>
						{expanded ? "Hide" : "Show"} details
						{#if expanded}
							<ChevronUpIcon class="h-3 w-3" />
						{:else}
							<ChevronDownIcon class="h-3 w-3" />
						{/if}
					</button>
				</div>
			{:else}
				<div class="flex items-center">
					<button
						class="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground"
						onclick={() => (expanded = !expanded)}
					>
						{expanded ? "Hide" : "Show"} details
						{#if expanded}
							<ChevronUpIcon class="h-3 w-3" />
						{:else}
							<ChevronDownIcon class="h-3 w-3" />
						{/if}
					</button>
				</div>
			{/if}

			<!-- Expandable details -->
			{#if expanded}
				<div class="space-y-3 border-t pt-3">
					<div>
						<p class="mb-1 text-xs font-medium text-muted-foreground uppercase">Target path</p>
						<p class="font-mono text-xs">{recommendation.target_path}</p>
					</div>

					<div>
						<p class="mb-1 text-xs font-medium text-muted-foreground uppercase">Rationale</p>
						<p class="text-sm text-muted-foreground">{recommendation.rationale}</p>
					</div>

					<div>
						<p class="mb-1 text-xs font-medium text-muted-foreground uppercase">
							Content preview
						</p>
						<pre
							class="max-h-48 overflow-y-auto rounded-md bg-muted p-3 font-mono text-xs whitespace-pre-wrap"
						>{recommendation.content.slice(0, 800)}{recommendation.content.length > 800
								? "\n..."
								: ""}</pre>
					</div>
				</div>
			{/if}
		</div>
	</Card.Content>
</Card.Root>
