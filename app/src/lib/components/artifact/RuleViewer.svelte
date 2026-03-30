<script lang="ts">
	import { Icon, Badge } from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
	import { getStores } from "@orqastudio/sdk";

	const { enforcementStore } = getStores();

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
				<Icon name="check-circle" size="sm" />
				<span>Loaded</span>
			</div>
		{:else}
			<div class="flex items-center gap-1 text-xs text-muted-foreground">
				<Icon name="circle-dashed" size="sm" />
				<span>Not loaded</span>
			</div>
		{/if}

		{#if matchedRule}
			<span class="text-muted-foreground">|</span>
			<div class="flex items-center gap-1 text-xs text-muted-foreground">
				{#if matchedRule.scope === "system"}
					<Icon name="globe" size="xs" />
					<span>System</span>
				{:else}
					<Icon name="folder" size="xs" />
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
					<Icon name="chevron-down" size="xs" />
				{:else}
					<Icon name="chevron-right" size="xs" />
				{/if}
				Session Violations ({ruleViolations.length})
			</button>
			{#if violationsExpanded}
				<div class="space-y-1 border-t border-border px-3 py-2">
					{#each ruleViolations as violation (violation.timestamp)}
						<div class="flex items-start gap-2">
							{#if violation.action === "Block"}
								<Icon name="shield" size="xs" />
							{:else}
								<Icon name="alert-triangle" size="xs" />
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
	<MarkdownRenderer {content} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
</div>
