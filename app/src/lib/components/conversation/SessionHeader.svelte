<script lang="ts">
	import type { Session, SessionSummary } from "@orqastudio/types";
	import { Icon, Button, Heading } from "@orqastudio/svelte-components/pure";
	import SessionDropdown from "./SessionDropdown.svelte";

	let {
		session,
		sessions,
		sessionsLoading = false,
		sessionsError = null,
		onNewSession,
		onUpdateTitle,
		onSelectSession,
		onDeleteSession,
		onRetryLoadSessions,
	}: {
		session: Session;
		sessions: SessionSummary[];
		sessionsLoading?: boolean;
		sessionsError?: string | null;
		onNewSession: () => void;
		onUpdateTitle: (title: string) => void;
		onSelectSession: (sessionId: number) => void;
		onDeleteSession: (sessionId: number) => void;
		onRetryLoadSessions?: () => void;
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
		loading={sessionsLoading}
		error={sessionsError}
		onSelect={onSelectSession}
		onNewSession={onNewSession}
		onDelete={onDeleteSession}
		onRetry={onRetryLoadSessions}
	>
		<Button variant="ghost" size="icon-sm" aria-label="Session history" title="Session history">
			<Icon name="history" size="md" />
		</Button>
	</SessionDropdown>

	<!-- Session title -->
	<div class="flex min-w-0 flex-1 items-center gap-1">
		{#if isEditing}
			<input
				bind:this={inputRef}
				bind:value={editTitle}
				onblur={finishEditing}
				onkeydown={handleTitleKeydown}
				class="min-w-0 flex-1 h-7 rounded border border-border bg-background px-2 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
			/>
			<Button variant="ghost" size="icon-sm" onclick={finishEditing} aria-label="Save title">
				<Icon name="check" size="sm" />
			</Button>
		{:else}
			<span class="min-w-0 flex-1 truncate"><Heading level={5}>{displayTitle}</Heading></span>
			<Button variant="ghost" size="icon-sm" onclick={startEditing} aria-label="Edit title">
				<Icon name="pencil" size="sm" />
			</Button>
		{/if}
	</div>

	<!-- New session -->
	<Button variant="ghost" size="icon-sm" onclick={onNewSession} aria-label="New session">
		<Icon name="plus" size="md" />
	</Button>
</div>
