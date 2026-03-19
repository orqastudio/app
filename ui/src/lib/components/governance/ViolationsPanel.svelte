<script lang="ts">
	import { Icon, ScrollArea } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";
	import { SearchInput } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { logger } from "@orqastudio/sdk";
	import type { StoredEnforcementViolation } from "@orqastudio/types";

	const log = logger("governance");

	let {
		violations,
		loading = false,
		error = null,
		onRetry,
	}: {
		violations: StoredEnforcementViolation[];
		loading?: boolean;
		error?: string | null;
		onRetry?: () => void;
	} = $props();

	let ruleFilter = $state("");
	let actionFilter = $state<"all" | "block" | "warn">("all");

	const filtered = $derived(
		violations.filter((v) => {
			const matchesRule =
				ruleFilter.trim() === "" ||
				v.rule_name.toLowerCase().includes(ruleFilter.toLowerCase());
			const matchesAction =
				actionFilter === "all" ||
				v.action.toLowerCase() === actionFilter;
			return matchesRule && matchesAction;
		}),
	);

	const blockCount = $derived(violations.filter((v) => v.action.toLowerCase() === "block").length);
	const warnCount = $derived(violations.filter((v) => v.action.toLowerCase() === "warn").length);

	function formatTimestamp(iso: string): string {
		try {
			const d = new Date(iso.endsWith("Z") ? iso : `${iso}Z`);
			return d.toLocaleString(undefined, {
				month: "short",
				day: "numeric",
				hour: "2-digit",
				minute: "2-digit",
			});
		} catch (err) {
			log.error("Failed to format violation timestamp", { iso, err });
			return iso;
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header with summary counts -->
	<div class="border-b border-border px-4 py-3">
		<div class="flex items-center justify-between">
			<h2 class="text-sm font-semibold">Violation History</h2>
			<div class="flex items-center gap-2">
				{#if blockCount > 0}
					<Badge variant="destructive" class="h-5 px-1.5 text-xs">
						{blockCount} blocked
					</Badge>
				{/if}
				{#if warnCount > 0}
					<Badge variant="warning" class="h-5 px-1.5 text-xs">
						{warnCount} warned
					</Badge>
				{/if}
			</div>
		</div>

		<!-- Filters -->
		<div class="mt-2 flex items-center gap-2">
			<div class="flex-1">
				<SearchInput
					bind:value={ruleFilter}
					placeholder="Filter by rule name..."
					size="xs"
				/>
			</div>
			<div class="flex items-center gap-1">
				{#each (["all", "block", "warn"] as const) as opt (opt)}
					<button
						class="rounded px-2 py-1 text-xs transition-colors"
						class:bg-accent={actionFilter === opt}
						class:text-accent-foreground={actionFilter === opt}
						class:text-muted-foreground={actionFilter !== opt}
						onclick={() => { actionFilter = opt; }}
					>
						{opt === "all" ? "All" : opt === "block" ? "Blocks" : "Warns"}
					</button>
				{/each}
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="min-h-0 flex-1">
		{#if loading}
			<div class="flex h-full items-center justify-center">
				<LoadingSpinner />
			</div>
		{:else if error}
			<div class="flex h-full items-center justify-center px-4">
				<ErrorDisplay message={error} onRetry={onRetry} />
			</div>
		{:else if filtered.length === 0}
			<div class="flex h-full items-center justify-center">
				{#if violations.length === 0}
					<EmptyState
						icon="shield"
						title="No violations recorded"
						description="Enforcement violations will appear here when rules block or warn on tool calls."
					/>
				{:else}
					<EmptyState
						title="No matches"
						description="No violations match your current filters."
					/>
				{/if}
			</div>
		{:else}
			<ScrollArea class="h-full">
				<div class="divide-y divide-border">
					{#each filtered as v (v.id)}
						<div class="flex items-start gap-3 px-4 py-2.5 hover:bg-muted/30">
							<!-- Action icon -->
							<div class="mt-0.5 shrink-0">
								{#if v.action.toLowerCase() === "block"}
									<Icon name="shield" size="sm" />
								{:else}
									<Icon name="alert-triangle" size="sm" />
								{/if}
							</div>

							<!-- Details -->
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-2">
									<span class="truncate text-xs font-medium">{v.rule_name}</span>
									<Badge
										variant={v.action.toLowerCase() === "block" ? "destructive" : "warning"}
										class="h-4 shrink-0 px-1 py-0 text-xs"
									>
										{v.action.toLowerCase()}
									</Badge>
								</div>
								<span class="block truncate font-mono text-xs text-muted-foreground">
									{v.tool_name}
								</span>
								{#if v.detail}
									<span class="block truncate text-xs text-muted-foreground">
										{v.detail}
									</span>
								{/if}
							</div>

							<!-- Timestamp -->
							<div class="flex shrink-0 items-center gap-1 text-xs text-muted-foreground">
								<Icon name="clock" size="xs" />
								<span>{formatTimestamp(v.created_at)}</span>
							</div>
						</div>
					{/each}
				</div>
			</ScrollArea>
		{/if}
	</div>

	<!-- Footer with result count -->
	{#if !loading && !error && violations.length > 0}
		<div class="border-t border-border px-4 py-2 text-xs text-muted-foreground">
			{filtered.length} of {violations.length} {violations.length === 1 ? "violation" : "violations"}
		</div>
	{/if}
</div>
