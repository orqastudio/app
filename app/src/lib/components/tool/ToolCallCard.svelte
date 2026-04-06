<script lang="ts">
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import {
		Icon,
		Button,
		Caption,
		Stack,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "@orqastudio/svelte-components/pure";
	import ViolationBadge from "$lib/components/enforcement/ViolationBadge.svelte";
	import { getToolDisplay } from "$lib/utils/tool-display";
	import { fmt } from "@orqastudio/sdk";

	/**
	 * Parse enforcement violation text to extract the rule name from the standard message format.
	 * @param text - The enforcement violation message string (e.g. "Rule 'rule-name' blocked...").
	 * @returns The extracted rule name, or null if the text does not match the expected format.
	 */
	function parseEnforcementRuleName(text: string): string | null {
		const match = /^Rule '([^']+)'/.exec(text);
		return match ? match[1] : null;
	}

	let {
		toolName,
		toolInput,
		toolOutput,
		isError = false,
		isComplete = false,
	}: {
		toolName: string;
		toolInput: string | null;
		toolOutput: string | null;
		isError: boolean;
		isComplete?: boolean;
	} = $props();

	const MAX_DISPLAY_CHARS = 10_000;

	let open = $state(false);
	let showFullInput = $state(false);
	let showFullOutput = $state(false);

	const displayInfo = $derived(getToolDisplay(toolName));

	// Detect if this is an enforcement block — error output starts with "Rule '"
	const enforcementRuleName = $derived(
		isError && isComplete && toolOutput ? parseEnforcementRuleName(toolOutput) : null,
	);
	const isEnforcementBlock = $derived(enforcementRuleName !== null);

	const inputIsTruncated = $derived(toolInput !== null && toolInput.length > MAX_DISPLAY_CHARS);
	const displayInput = $derived(
		toolInput === null
			? null
			: showFullInput || !inputIsTruncated
				? toolInput
				: toolInput.slice(0, MAX_DISPLAY_CHARS),
	);

	const outputIsTruncated = $derived(toolOutput !== null && toolOutput.length > MAX_DISPLAY_CHARS);
	const displayOutput = $derived(
		toolOutput === null
			? null
			: showFullOutput || !outputIsTruncated
				? toolOutput
				: toolOutput.slice(0, MAX_DISPLAY_CHARS),
	);
</script>

<Collapsible bind:open>
	<CollapsibleTrigger
		class="flex w-full items-center gap-2 rounded-lg border {isEnforcementBlock
			? 'border-destructive/50 bg-destructive/5'
			: 'border-border bg-muted/30'} hover:bg-muted/50 px-3 py-2 text-left text-sm transition-colors"
	>
		<Icon name="chevron-right" size="sm" />
		{@const ToolIcon = displayInfo.icon}
		<ToolIcon class="text-muted-foreground h-3.5 w-3.5 shrink-0" />
		<!-- flex-1 is structural (fills trigger row); caption-mono provides the text styling -->
		<span class="flex-1"
			><Caption variant="caption-mono" truncate>{displayInfo.label}</Caption></span
		>
		{#if isEnforcementBlock && enforcementRuleName}
			<ViolationBadge action="Block" ruleName={enforcementRuleName} />
		{:else if isComplete && isError}
			<Icon name="x-circle" size="sm" />
		{:else if isComplete}
			<Icon name="check-circle" size="sm" />
		{:else}
			<Icon name="loader" size="sm" />
		{/if}
	</CollapsibleTrigger>
	<CollapsibleContent>
		<!-- border-l-2 and ml-3 are structural indentation; no ORQA primitive supports border-left -->
		<div class="border-border mt-1 ml-3 border-l-2 pl-4">
			<Stack gap={2}>
				{#if displayInput}
					<Stack gap={1}>
						<Caption variant="caption-strong">Input</Caption>
						<CodeBlock text={displayInput} lang="json" />
						{#if inputIsTruncated}
							<Button variant="ghost" size="sm" onclick={() => (showFullInput = !showFullInput)}>
								{#if showFullInput}
									Show less
								{:else}
									Show full input ({fmt(toolInput!.length / 1000, 0)}K chars)
								{/if}
							</Button>
						{/if}
					</Stack>
				{/if}
				{#if displayOutput}
					<Stack gap={1}>
						<Caption variant="caption-strong">{isError ? "Error" : "Output"}</Caption>
						<CodeBlock text={displayOutput} lang={isError ? "" : "json"} />
						{#if outputIsTruncated}
							<Button variant="ghost" size="sm" onclick={() => (showFullOutput = !showFullOutput)}>
								{#if showFullOutput}
									Show less
								{:else}
									Show full output ({fmt(toolOutput!.length / 1000, 0)}K chars)
								{/if}
							</Button>
						{/if}
					</Stack>
				{/if}
				{#if !isComplete}
					<Caption variant="caption-strong">Running...</Caption>
				{/if}
			</Stack>
		</div>
	</CollapsibleContent>
</Collapsible>
