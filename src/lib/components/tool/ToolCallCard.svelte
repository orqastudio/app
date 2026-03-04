<script lang="ts">
	import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
	import WrenchIcon from "@lucide/svelte/icons/wrench";
	import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
	import XCircleIcon from "@lucide/svelte/icons/x-circle";
	import LoaderIcon from "@lucide/svelte/icons/loader";
	import FileTextIcon from "@lucide/svelte/icons/file-text";
	import FilePenIcon from "@lucide/svelte/icons/file-pen";
	import PencilIcon from "@lucide/svelte/icons/pencil";
	import TerminalIcon from "@lucide/svelte/icons/terminal";
	import FolderSearchIcon from "@lucide/svelte/icons/folder-search";
	import FileSearchIcon from "@lucide/svelte/icons/file-search";
	import RegexIcon from "@lucide/svelte/icons/regex";
	import BrainIcon from "@lucide/svelte/icons/brain";
	import BookOpenIcon from "@lucide/svelte/icons/book-open";
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import {
		Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "$lib/components/ui/collapsible";

	const TOOL_DISPLAY: Record<string, { label: string; icon: typeof WrenchIcon }> = {
		read_file: { label: "Read File", icon: FileTextIcon },
		write_file: { label: "Write File", icon: FilePenIcon },
		edit_file: { label: "Edit File", icon: PencilIcon },
		bash: { label: "Run Command", icon: TerminalIcon },
		glob: { label: "Find Files", icon: FolderSearchIcon },
		grep: { label: "Search Content", icon: FileSearchIcon },
		search_regex: { label: "Regex Search", icon: RegexIcon },
		search_semantic: { label: "Semantic Search", icon: BrainIcon },
		code_research: { label: "Code Research", icon: BookOpenIcon },
	};

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

	let open = $state(false);

	const displayInfo = $derived(TOOL_DISPLAY[toolName] ?? { label: toolName, icon: WrenchIcon });

	const statusColor = $derived(
		isComplete ? (isError ? "text-destructive" : "text-green-500") : "text-muted-foreground"
	);
</script>

<Collapsible bind:open>
	<CollapsibleTrigger
		class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
	>
		<ChevronRightIcon
			class="h-3.5 w-3.5 shrink-0 text-muted-foreground transition-transform {open
				? 'rotate-90'
				: ''}"
		/>
		{@const Icon = displayInfo.icon}
		<Icon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
		<span class="flex-1 truncate font-mono text-xs">{displayInfo.label}</span>
		{#if isComplete && isError}
			<XCircleIcon class="h-3.5 w-3.5 shrink-0 {statusColor}" />
		{:else if isComplete}
			<CheckCircleIcon class="h-3.5 w-3.5 shrink-0 {statusColor}" />
		{:else}
			<LoaderIcon class="h-3.5 w-3.5 shrink-0 animate-spin {statusColor}" />
		{/if}
	</CollapsibleTrigger>
	<CollapsibleContent>
		<div class="ml-3 mt-1 space-y-2 border-l-2 border-border pl-4">
			{#if toolInput}
				<div>
					<p class="mb-1 text-xs font-medium text-muted-foreground">Input</p>
					<CodeBlock code={toolInput} language="json" />
				</div>
			{/if}
			{#if toolOutput}
				<div>
					<p class="mb-1 text-xs font-medium text-muted-foreground">
						{isError ? "Error" : "Output"}
					</p>
					<CodeBlock code={toolOutput} language={isError ? "" : "json"} />
				</div>
			{/if}
			{#if !isComplete}
				<p class="text-xs italic text-muted-foreground">Running...</p>
			{/if}
		</div>
	</CollapsibleContent>
</Collapsible>
