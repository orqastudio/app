<script lang="ts">
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import { Icon,
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "@orqastudio/svelte-components/pure";
	import ViolationBadge from "$lib/components/enforcement/ViolationBadge.svelte";
	import { getToolDisplay } from "$lib/utils/tool-display";

	// Parses "Rule 'rule-name' blocked..." text to extract rule name
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
		isError && isComplete && toolOutput ? parseEnforcementRuleName(toolOutput) : null
	);
	const isEnforcementBlock = $derived(enforcementRuleName !== null);

	const inputIsTruncated = $derived(toolInput !== null && toolInput.length > MAX_DISPLAY_CHARS);
	const displayInput = $derived(
		toolInput === null
			? null
			: showFullInput || !inputIsTruncated
				? toolInput
				: toolInput.slice(0, MAX_DISPLAY_CHARS)
	);

	const outputIsTruncated = $derived(toolOutput !== null && toolOutput.length > MAX_DISPLAY_CHARS);
	const displayOutput = $derived(
		toolOutput === null
			? null
			: showFullOutput || !outputIsTruncated
				? toolOutput
				: toolOutput.slice(0, MAX_DISPLAY_CHARS)
	);
</script>

<Collapsible bind:open>
	<CollapsibleTrigger
		class="flex w-full items-center gap-2 rounded-lg border {isEnforcementBlock ? 'border-destructive/50 bg-destructive/5' : 'border-border bg-muted/30'} px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
	>
		<Icon name="chevron-right" size="sm" />
		{@const ToolIcon = displayInfo.icon}
		<ToolIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		<span class="flex-1 truncate font-mono text-xs">{displayInfo.label}</span>
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
		<div class="ml-3 mt-1 space-y-2 border-l-2 border-border pl-4">
			{#if displayInput}
				<div>
					<p class="mb-1 text-xs font-medium text-muted-foreground">Input</p>
					<CodeBlock text={displayInput} lang="json" />
					{#if inputIsTruncated}
						<button
							class="mt-1 text-xs text-muted-foreground hover:text-foreground"
							onclick={() => (showFullInput = !showFullInput)}
						>
							{#if showFullInput}
								Show less
							{:else}
								Show full input ({Math.round(toolInput!.length / 1000)}K chars)
							{/if}
						</button>
					{/if}
				</div>
			{/if}
			{#if displayOutput}
				<div>
					<p class="mb-1 text-xs font-medium text-muted-foreground">
						{isError ? "Error" : "Output"}
					</p>
					<CodeBlock text={displayOutput} lang={isError ? "" : "json"} />
					{#if outputIsTruncated}
						<button
							class="mt-1 text-xs text-muted-foreground hover:text-foreground"
							onclick={() => (showFullOutput = !showFullOutput)}
						>
							{#if showFullOutput}
								Show less
							{:else}
								Show full output ({Math.round(toolOutput!.length / 1000)}K chars)
							{/if}
						</button>
					{/if}
				</div>
			{/if}
			{#if !isComplete}
				<p class="text-xs italic text-muted-foreground">Running...</p>
			{/if}
		</div>
	</CollapsibleContent>
</Collapsible>
