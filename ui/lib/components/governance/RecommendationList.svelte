<script lang="ts">
	import ListIcon from "@lucide/svelte/icons/list";
	import PlayIcon from "@lucide/svelte/icons/play";
	import * as Tabs from "$lib/components/ui/tabs";
	import { Button } from "$lib/components/ui/button";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import RecommendationCard from "./RecommendationCard.svelte";
	import type { Recommendation, RecommendationStatus } from "$lib/types/governance";

	interface Props {
		recommendations: Recommendation[];
		loading: boolean;
		onApprove: (id: number) => void;
		onReject: (id: number) => void;
		onApply: (id: number) => void;
		onApplyAll: () => void;
	}

	const { recommendations, loading, onApprove, onReject, onApply, onApplyAll }: Props = $props();

	type FilterTab = "all" | RecommendationStatus;

	let activeTab = $state<FilterTab>("all");

	const tabs: { value: FilterTab; label: string }[] = [
		{ value: "all", label: "All" },
		{ value: "pending", label: "Pending" },
		{ value: "approved", label: "Approved" },
		{ value: "rejected", label: "Rejected" },
		{ value: "applied", label: "Applied" },
	];

	const filtered = $derived(
		activeTab === "all"
			? recommendations
			: recommendations.filter((r) => r.status === activeTab),
	);

	const approvedCount = $derived(recommendations.filter((r) => r.status === "approved").length);
	const hasApproved = $derived(approvedCount > 0);
</script>

<div class="flex flex-col gap-3">
	<!-- Apply all button -->
	{#if hasApproved}
		<div class="flex items-center justify-between rounded-md border bg-muted/30 px-3 py-2">
			<span class="text-sm text-muted-foreground">
				{approvedCount} recommendation{approvedCount === 1 ? "" : "s"} approved and ready to apply
			</span>
			<Button size="sm" onclick={onApplyAll}>
				<PlayIcon class="mr-1 h-3 w-3" />
				Apply All
			</Button>
		</div>
	{/if}

	<!-- Filter tabs -->
	<Tabs.Root value={activeTab} onValueChange={(v) => (activeTab = v as FilterTab)}>
		<Tabs.List class="w-full">
			{#each tabs as tab}
				<Tabs.Trigger value={tab.value} class="flex-1 text-xs">
					{tab.label}
					{#if tab.value !== "all"}
						{@const count = recommendations.filter(
							(r) => tab.value === "all" || r.status === tab.value,
						).length}
						{#if count > 0}
							<span class="ml-1 text-[10px] text-muted-foreground">({count})</span>
						{/if}
					{/if}
				</Tabs.Trigger>
			{/each}
		</Tabs.List>
	</Tabs.Root>

	<!-- Content -->
	{#if loading}
		<LoadingSpinner />
	{:else if filtered.length === 0}
		<EmptyState
			icon={ListIcon}
			title="No recommendations"
			description={activeTab === "all"
				? "No recommendations have been generated yet."
				: `No ${activeTab} recommendations.`}
		/>
	{:else}
		<div class="space-y-2">
			{#each filtered as rec (rec.id)}
				<RecommendationCard
					recommendation={rec}
					{onApprove}
					{onReject}
					{onApply}
				/>
			{/each}
		</div>
	{/if}
</div>
