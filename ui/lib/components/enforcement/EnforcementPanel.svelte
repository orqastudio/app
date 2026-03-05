<script lang="ts">
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import AlertTriangleIcon from "@lucide/svelte/icons/alert-triangle";
	import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
	import FolderIcon from "@lucide/svelte/icons/folder";
	import GlobeIcon from "@lucide/svelte/icons/globe";
	import * as Card from "$lib/components/ui/card";
	import { Badge } from "$lib/components/ui/badge";
	import { Button } from "$lib/components/ui/button";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";
	import { onMount } from "svelte";

	onMount(() => {
		enforcementStore.loadRules();
	});

	const rules = $derived(enforcementStore.rules);
	const violations = $derived(enforcementStore.violations);
	const loading = $derived(enforcementStore.loading);
	const error = $derived(enforcementStore.error);
	const blockCount = $derived(enforcementStore.blockCount);
	const warnCount = $derived(enforcementStore.warnCount);

	// Group violations by rule name
	const violationsByRule = $derived(
		violations.reduce<Record<string, typeof violations>>(
			(acc, v) => {
				if (!acc[v.rule_name]) acc[v.rule_name] = [];
				acc[v.rule_name].push(v);
				return acc;
			},
			{},
		),
	);
</script>

<div class="flex h-full flex-col">
	<!-- Header with summary counts -->
	<div class="flex items-center justify-between border-b border-border px-3 py-2">
		<div class="flex items-center gap-2">
			<ShieldIcon class="h-4 w-4 text-muted-foreground" />
			<span class="text-sm font-medium">Enforcement</span>
		</div>
		<div class="flex items-center gap-1.5">
			{#if blockCount > 0}
				<Badge variant="destructive" class="text-xs px-1.5 py-0.5">
					{blockCount} blocked
				</Badge>
			{/if}
			{#if warnCount > 0}
				<Badge variant="outline" class="text-xs px-1.5 py-0.5 border-amber-500/50 text-amber-600 dark:text-amber-400">
					{warnCount} warned
				</Badge>
			{/if}
			<Button
				variant="ghost"
				size="sm"
				class="h-6 w-6 p-0"
				onclick={() => enforcementStore.reloadRules()}
				disabled={loading}
				aria-label="Reload rules"
			>
				<RefreshCwIcon class="h-3 w-3 {loading ? 'animate-spin' : ''}" />
			</Button>
		</div>
	</div>

	<ScrollArea class="flex-1">
		<div class="p-3 space-y-4">
			{#if loading && rules.length === 0}
				<div class="flex justify-center py-8">
					<LoadingSpinner />
				</div>
			{:else if error}
				<ErrorDisplay
					message="Failed to load rules: {error}"
					onRetry={() => enforcementStore.loadRules()}
				/>
			{:else if rules.length === 0}
				<EmptyState
					icon={ShieldIcon}
					title="No rules found"
					description="No enforcement rules found in this project."
				/>
			{:else}
				<!-- Rule inventory -->
				<div>
					<p class="mb-2 text-xs font-medium text-muted-foreground uppercase tracking-wide">
						Loaded Rules ({rules.length})
					</p>
					<div class="space-y-1.5">
						{#each rules as rule}
							<Card.Root class="overflow-hidden">
								<Card.Content class="px-3 py-2">
									<div class="flex items-start justify-between gap-2">
										<div class="min-w-0 flex-1">
											<div class="flex items-center gap-1.5">
												<span class="truncate text-xs font-mono font-medium">{rule.name}</span>
											</div>
											<div class="mt-0.5 flex items-center gap-1.5">
												{#if rule.scope === "system"}
													<GlobeIcon class="h-3 w-3 text-muted-foreground" />
													<span class="text-xs text-muted-foreground">system</span>
												{:else}
													<FolderIcon class="h-3 w-3 text-muted-foreground" />
													<span class="text-xs text-muted-foreground">project</span>
												{/if}
												<span class="text-xs text-muted-foreground">·</span>
												<span class="text-xs text-muted-foreground">
													{rule.entries.length} {rule.entries.length === 1 ? "entry" : "entries"}
												</span>
											</div>
										</div>
										<!-- Violation count badge for this rule -->
										{#if violationsByRule[rule.name]?.length}
											{@const ruleViolations = violationsByRule[rule.name]}
											{@const ruleBlocks = ruleViolations.filter((v) => v.action === "Block").length}
											{@const ruleWarns = ruleViolations.filter((v) => v.action === "Warn").length}
											<div class="flex items-center gap-1 shrink-0">
												{#if ruleBlocks > 0}
													<Badge variant="destructive" class="text-xs px-1 py-0 h-4">
														{ruleBlocks}
													</Badge>
												{/if}
												{#if ruleWarns > 0}
													<Badge variant="outline" class="text-xs px-1 py-0 h-4 border-amber-500/50 text-amber-600 dark:text-amber-400">
														{ruleWarns}
													</Badge>
												{/if}
											</div>
										{/if}
									</div>
								</Card.Content>
							</Card.Root>
						{/each}
					</div>
				</div>

				<!-- Session violations -->
				{#if violations.length > 0}
					<div>
						<p class="mb-2 text-xs font-medium text-muted-foreground uppercase tracking-wide">
							Session Violations ({violations.length})
						</p>
						<div class="space-y-1.5">
							{#each Object.entries(violationsByRule) as [ruleName, ruleViolations]}
								<Card.Root class="overflow-hidden">
									<Card.Content class="px-3 py-2">
										<p class="mb-1.5 text-xs font-mono font-medium">{ruleName}</p>
										<div class="space-y-1">
											{#each ruleViolations as violation}
												<div class="flex items-start gap-2">
													{#if violation.action === "Block"}
														<ShieldIcon class="mt-0.5 h-3 w-3 shrink-0 text-destructive" />
													{:else}
														<AlertTriangleIcon class="mt-0.5 h-3 w-3 shrink-0 text-amber-500" />
													{/if}
													<div class="min-w-0 flex-1">
														<span class="text-xs text-muted-foreground font-mono truncate block">{violation.tool_name}</span>
														<span class="text-xs text-muted-foreground">{violation.detail}</span>
													</div>
												</div>
											{/each}
										</div>
									</Card.Content>
								</Card.Root>
							{/each}
						</div>
					</div>
				{/if}
			{/if}
		</div>
	</ScrollArea>
</div>
