<script lang="ts">
	import ShieldIcon from "@lucide/svelte/icons/shield";
	import AlertTriangleIcon from "@lucide/svelte/icons/alert-triangle";
	import FolderIcon from "@lucide/svelte/icons/folder";
	import GlobeIcon from "@lucide/svelte/icons/globe";
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import CircleDashedIcon from "@lucide/svelte/icons/circle-dashed";
	import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import { Badge } from "$lib/components/ui/badge";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import { enforcementStore } from "$lib/stores/enforcement.svelte";

	let { content, ruleName }: { content: string; ruleName: string } = $props();

	// Match this rule name against loaded enforcement rules
	const matchedRule = $derived(
		enforcementStore.rules.find((r) => r.name === ruleName),
	);
	const isLoaded = $derived(matchedRule !== null && matchedRule !== undefined);

	// Violations for this specific rule
	const ruleViolations = $derived(
		enforcementStore.violations.filter((v) => v.rule_name === ruleName),
	);
	const ruleBlockCount = $derived(
		ruleViolations.filter((v) => v.action === "Block").length,
	);
	const ruleWarnCount = $derived(
		ruleViolations.filter((v) => v.action === "Warn").length,
	);

	let violationsExpanded = $state(true);
</script>

<div class="space-y-4">
	<!-- Enforcement status bar -->
	<div class="flex flex-wrap items-center gap-2 rounded-md border border-border bg-muted/30 px-3 py-2">
		{#if isLoaded}
			<div class="flex items-center gap-1 text-xs text-success">
				<CheckCircleIcon class="h-3.5 w-3.5" />
				<span>Loaded</span>
			</div>
		{:else}
			<div class="flex items-center gap-1 text-xs text-muted-foreground">
				<CircleDashedIcon class="h-3.5 w-3.5" />
				<span>Not loaded</span>
			</div>
		{/if}

		{#if matchedRule}
			<span class="text-muted-foreground">|</span>
			<div class="flex items-center gap-1 text-xs text-muted-foreground">
				{#if matchedRule.scope === "system"}
					<GlobeIcon class="h-3 w-3" />
					<span>System</span>
				{:else}
					<FolderIcon class="h-3 w-3" />
					<span>Project</span>
				{/if}
			</div>
			<span class="text-muted-foreground">|</span>
			<span class="text-xs text-muted-foreground">
				{matchedRule.entries.length} {matchedRule.entries.length === 1 ? "entry" : "entries"}
			</span>
		{/if}

		{#if ruleBlockCount > 0}
			<Badge variant="destructive" class="h-5 px-1.5 text-xs">
				{ruleBlockCount} blocked
			</Badge>
		{/if}
		{#if ruleWarnCount > 0}
			<Badge variant="warning" class="h-5 px-1.5 text-xs">
				{ruleWarnCount} warned
			</Badge>
		{/if}
	</div>

	<!-- Violation details (collapsible) -->
	{#if ruleViolations.length > 0}
		<div class="rounded-md border border-border">
			<button
				class="flex w-full items-center gap-2 px-3 py-2 text-left text-xs font-medium text-muted-foreground uppercase tracking-wide hover:bg-muted/50"
				onclick={() => (violationsExpanded = !violationsExpanded)}
			>
				{#if violationsExpanded}
					<ChevronDownIcon class="h-3 w-3" />
				{:else}
					<ChevronRightIcon class="h-3 w-3" />
				{/if}
				Session Violations ({ruleViolations.length})
			</button>
			{#if violationsExpanded}
				<div class="space-y-1 border-t border-border px-3 py-2">
					{#each ruleViolations as violation (violation.timestamp)}
						<div class="flex items-start gap-2">
							{#if violation.action === "Block"}
								<ShieldIcon class="mt-0.5 h-3 w-3 shrink-0 text-destructive" />
							{:else}
								<AlertTriangleIcon class="mt-0.5 h-3 w-3 shrink-0 text-warning" />
							{/if}
							<div class="min-w-0 flex-1">
								<span class="block truncate font-mono text-xs text-muted-foreground">{violation.tool_name}</span>
								<span class="text-xs text-muted-foreground">{violation.detail}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Rule content -->
	<MarkdownRenderer {content} />
</div>
