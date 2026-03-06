<script lang="ts">
	import type { Session, SessionSummary } from "$lib/types";
	import PlusIcon from "@lucide/svelte/icons/plus";
	import PencilIcon from "@lucide/svelte/icons/pencil";
	import CheckIcon from "@lucide/svelte/icons/check";
	import HistoryIcon from "@lucide/svelte/icons/history";
	import { Button } from "$lib/components/ui/button";
	import SessionDropdown from "./SessionDropdown.svelte";

	let {
		session,
		sessions,
		onNewSession,
		onUpdateTitle,
		onSelectSession,
		onDeleteSession,
	}: {
		session: Session;
		sessions: SessionSummary[];
		onNewSession: () => void;
		onUpdateTitle: (title: string) => void;
		onSelectSession: (sessionId: number) => void;
		onDeleteSession: (sessionId: number) => void;
	} = $props();

	let isEditing = $state(false);
	let editTitle = $state("");
	let inputRef = $state<HTMLInputElement | null>(null);

	const displayTitle = $derived(session.title ?? "New Session");

	function startEditing() {
		isEditing = true;
		editTitle = session.title ?? "";
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

	<!-- New session -->
	<Button variant="ghost" size="icon-sm" onclick={onNewSession} aria-label="New session">
		<PlusIcon class="h-4 w-4" />
	</Button>
</div>
