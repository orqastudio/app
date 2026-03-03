<script lang="ts">
	import type { Session, SessionSummary } from "$lib/types";
	import PlusIcon from "@lucide/svelte/icons/plus";
	import PencilIcon from "@lucide/svelte/icons/pencil";
	import CheckIcon from "@lucide/svelte/icons/check";
	import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
	import HistoryIcon from "@lucide/svelte/icons/history";
	import { Button } from "$lib/components/ui/button";
	import {
		DropdownMenu,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuTrigger,
	} from "$lib/components/ui/dropdown-menu";
	import { Badge } from "$lib/components/ui/badge";
	import SessionDropdown from "./SessionDropdown.svelte";

	let {
		session,
		sessions,
		resolvedModel,
		onNewSession,
		onUpdateTitle,
		onSelectModel,
		onSelectSession,
		onDeleteSession,
	}: {
		session: Session;
		sessions: SessionSummary[];
		resolvedModel: string | null;
		onNewSession: () => void;
		onUpdateTitle: (title: string) => void;
		onSelectModel: (model: string) => void;
		onSelectSession: (sessionId: number) => void;
		onDeleteSession: (sessionId: number) => void;
	} = $props();

	let isEditing = $state(false);
	let editTitle = $state("");
	let inputRef = $state<HTMLInputElement | null>(null);

	const displayTitle = $derived(session.title ?? "New Session");
	const modelLabel = $derived(getModelLabel(session.model));
	const resolvedLabel = $derived(resolvedModel ? getModelLabel(resolvedModel) : null);

	const models: { value: string; label: string }[] = [
		{ value: "auto", label: "Auto" },
		{ value: "opus", label: "Opus" },
		{ value: "sonnet", label: "Sonnet" },
		{ value: "haiku", label: "Haiku" },
	];

	function getModelLabel(model: string): string {
		const found = models.find((m) => m.value === model);
		return found ? found.label : model;
	}

	function startEditing() {
		isEditing = true;
		editTitle = session.title ?? "";
		// Focus input after it renders
		setTimeout(() => inputRef?.focus(), 0);
	}

	function finishEditing() {
		isEditing = false;
		const trimmed = editTitle.trim();
		if (trimmed.length > 0 && trimmed !== session.title) {
			onUpdateTitle(trimmed);
		}
	}

	function handleTitleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter") {
			event.preventDefault();
			finishEditing();
		} else if (event.key === "Escape") {
			isEditing = false;
		}
	}

	function formatTokens(count: number): string {
		if (count >= 1000) {
			return `${(count / 1000).toFixed(1)}k`;
		}
		return String(count);
	}
</script>

<div class="flex items-center gap-2 border-b border-border px-3 py-2">
	<!-- Session dropdown trigger -->
	<SessionDropdown
		{sessions}
		activeSessionId={session.id}
		onSelect={onSelectSession}
		onNewSession={onNewSession}
		onDelete={onDeleteSession}
	>
		<Button variant="ghost" size="icon-sm" aria-label="Session history" title="Session history">
			<HistoryIcon class="h-4 w-4" />
		</Button>
	</SessionDropdown>

	<!-- Session title -->
	<div class="flex min-w-0 flex-1 items-center gap-1.5">
		{#if isEditing}
			<input
				bind:this={inputRef}
				bind:value={editTitle}
				onblur={finishEditing}
				onkeydown={handleTitleKeydown}
				class="min-w-0 flex-1 rounded border border-input bg-transparent px-1.5 py-0.5 text-sm outline-none focus-visible:border-ring"
			/>
			<Button variant="ghost" size="icon-sm" onclick={finishEditing} aria-label="Save title">
				<CheckIcon class="h-3.5 w-3.5" />
			</Button>
		{:else}
			<h2 class="min-w-0 flex-1 truncate text-sm font-medium">{displayTitle}</h2>
			<Button variant="ghost" size="icon-sm" onclick={startEditing} aria-label="Edit title">
				<PencilIcon class="h-3.5 w-3.5" />
			</Button>
		{/if}
	</div>

	<!-- Model selector -->
	<DropdownMenu>
		<DropdownMenuTrigger>
			<Button variant="outline" size="sm" class="gap-1 text-xs">
				{modelLabel}
				{#if resolvedLabel && resolvedLabel !== modelLabel}
					<span class="text-muted-foreground">({resolvedLabel})</span>
				{/if}
				<ChevronDownIcon class="h-3 w-3" />
			</Button>
		</DropdownMenuTrigger>
		<DropdownMenuContent align="end">
			{#each models as model}
				<DropdownMenuItem onclick={() => onSelectModel(model.value)}>
					{model.label}
					{#if model.value === session.model}
						<CheckIcon class="ml-auto h-3.5 w-3.5" />
					{/if}
				</DropdownMenuItem>
			{/each}
		</DropdownMenuContent>
	</DropdownMenu>

	<!-- Token usage -->
	{#if session.total_input_tokens > 0 || session.total_output_tokens > 0}
		<Badge variant="secondary" class="text-[10px]">
			{formatTokens(session.total_input_tokens)}↑ {formatTokens(session.total_output_tokens)}↓
		</Badge>
	{/if}

	<!-- New session -->
	<Button variant="ghost" size="icon-sm" onclick={onNewSession} aria-label="New session">
		<PlusIcon class="h-4 w-4" />
	</Button>
</div>
