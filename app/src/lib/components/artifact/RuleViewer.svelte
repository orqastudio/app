<!-- Renders an enforcement rule: load status, scope, violation counts, and full markdown content. -->
<script lang="ts">
	import {
		Icon,
		Badge,
		HStack,
		Stack,
		Box,
		Text,
		Button,
		Panel,
		Separator,
	} from "@orqastudio/svelte-components/pure";
	import { MarkdownRenderer } from "@orqastudio/svelte-components/connected";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";
	import { getStores } from "@orqastudio/sdk";

	const { enforcementStore } = getStores();

	let { content, ruleName }: { content: string; ruleName: string } = $props();

	// Match this rule name against loaded enforcement rules
	const matchedRule = $derived(enforcementStore.rules.find((r) => r.name === ruleName));
	const isLoaded = $derived(matchedRule !== null && matchedRule !== undefined);

	// Violations for this specific rule
	const ruleViolations = $derived(
		enforcementStore.violations.filter((v) => v.rule_name === ruleName),
	);
	const ruleBlockCount = $derived(ruleViolations.filter((v) => v.action === "Block").length);
	const ruleWarnCount = $derived(ruleViolations.filter((v) => v.action === "Warn").length);

	let violationsExpanded = $state(true);
</script>

<Stack gap={4}>
	<!-- Enforcement status bar -->
	<Panel background="muted-subtle" border="all" rounded="md" padding="tight">
		<HStack wrap gap={2}>
			{#if isLoaded}
				<HStack gap={1}>
					<Icon name="check-circle" size="sm" />
					<Text variant="caption" tone="success">Loaded</Text>
				</HStack>
			{:else}
				<HStack gap={1}>
					<Icon name="circle-dashed" size="sm" />
					<Text variant="caption">Not loaded</Text>
				</HStack>
			{/if}

			{#if matchedRule}
				<Separator orientation="vertical" />
				<HStack gap={1}>
					{#if matchedRule.scope === "system"}
						<Icon name="globe" size="xs" />
						<Text variant="caption">System</Text>
					{:else}
						<Icon name="folder" size="xs" />
						<Text variant="caption">Project</Text>
					{/if}
				</HStack>
				<Separator orientation="vertical" />
				<Text variant="caption">
					{matchedRule.entries.length}
					{matchedRule.entries.length === 1 ? "entry" : "entries"}
				</Text>
			{/if}

			{#if ruleBlockCount > 0}
				<Badge variant="destructive" size="sm">
					{ruleBlockCount} blocked
				</Badge>
			{/if}
			{#if ruleWarnCount > 0}
				<Badge variant="warning" size="sm">
					{ruleWarnCount} warned
				</Badge>
			{/if}
		</HStack>
	</Panel>

	<!-- Violation details (collapsible) -->
	{#if ruleViolations.length > 0}
		<Panel padding="none" border="all" rounded="md">
			<Button variant="ghost" full onclick={() => (violationsExpanded = !violationsExpanded)}>
				<HStack gap={1}>
					{#if violationsExpanded}
						<Icon name="chevron-down" size="xs" />
					{:else}
						<Icon name="chevron-right" size="xs" />
					{/if}
					<Text variant="overline">Session Violations ({ruleViolations.length})</Text>
				</HStack>
			</Button>
			{#if violationsExpanded}
				<Panel padding="tight" border="top">
					<Stack gap={1}>
						{#each ruleViolations as violation (violation.timestamp)}
							<HStack gap={2} align="start">
								{#if violation.action === "Block"}
									<Icon name="shield" size="xs" />
								{:else}
									<Icon name="alert-triangle" size="xs" />
								{/if}
								<Box flex={1} minWidth={0}>
									<Text variant="caption-mono" truncate block>{violation.tool_name}</Text>
									<Text variant="caption" block>{violation.detail}</Text>
								</Box>
							</HStack>
						{/each}
					</Stack>
				</Panel>
			{/if}
		</Panel>
	{/if}

	<!-- Rule content -->
	<MarkdownRenderer {content} codeRenderer={DiagramCodeBlock} linkRenderer={MarkdownLink} />
</Stack>
