<script lang="ts">
	import type { Session, SessionSummary } from "@orqastudio/types";
	import { Icon, Button, Heading, SectionHeader } from "@orqastudio/svelte-components/pure";
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

	/**
	 *
	 */
	function startEditing() {
		isEditing = true;
		editTitle = session.title ?? "";
		setTimeout(() => inputRef?.focus(), 0);
	}

	/**
	 *
	 */
	function finishEditing() {
		isEditing = false;
		const trimmed = editTitle.trim();
		if (trimmed.length > 0 && trimmed !== session.title) {
			onUpdateTitle(trimmed);
		}
	}

	/**
	 *
	 * @param event
	 */
	function handleTitleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter") {
			event.preventDefault();
			finishEditing();
		} else if (event.key === "Escape") {
			isEditing = false;
		}
	}
</script>

<SectionHeader>
	{#snippet start()}
		<!-- Session dropdown trigger -->
		<SessionDropdown
			{sessions}
			activeSessionId={session.id}
			loading={sessionsLoading}
			error={sessionsError}
			onSelect={onSelectSession}
			{onNewSession}
			onDelete={onDeleteSession}
			onRetry={onRetryLoadSessions}
		>
			<Button variant="ghost" size="icon-sm" aria-label="Session history" title="Session history">
				<Icon name="history" size="md" />
			</Button>
		</SessionDropdown>

		<!-- Session title; min-w-0 flex-1 are structural layout constraints -->
		<div class="flex min-w-0 flex-1 items-center gap-1">
			{#if isEditing}
				<!-- input requires direct DOM binding; no ORQA Input has bind:this + onblur -->
				<input
					bind:this={inputRef}
					bind:value={editTitle}
					onblur={finishEditing}
					onkeydown={handleTitleKeydown}
					class="border-border bg-background focus:ring-ring h-7 min-w-0 flex-1 rounded border px-2 text-sm focus:ring-1 focus:outline-none"
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
	{/snippet}
	{#snippet end()}
		<!-- New session -->
		<Button variant="ghost" size="icon-sm" onclick={onNewSession} aria-label="New session">
			<Icon name="plus" size="md" />
		</Button>
	{/snippet}
</SectionHeader>
