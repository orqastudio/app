<script lang="ts">
	import type { Session, SessionSummary } from "@orqastudio/types";
	import {
		Icon,
		Button,
		Heading,
		HStack,
		Box,
		SectionHeader,
		Input,
	} from "@orqastudio/svelte-components/pure";
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
	 * Commit the title edit on Enter, or cancel it on Escape.
	 * @param event - The keyboard event from the title input field.
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

		<HStack gap={1} flex={1} minWidth={0}>
			{#if isEditing}
				<Input
					bind:ref={inputRef}
					bind:value={editTitle}
					size="compact"
					onblur={finishEditing}
					onkeydown={handleTitleKeydown}
				/>
				<Button variant="ghost" size="icon-sm" onclick={finishEditing} aria-label="Save title">
					<Icon name="check" size="sm" />
				</Button>
			{:else}
				<Box flex={1} minWidth={0} truncate><Heading level={5}>{displayTitle}</Heading></Box>
				<Button variant="ghost" size="icon-sm" onclick={startEditing} aria-label="Edit title">
					<Icon name="pencil" size="sm" />
				</Button>
			{/if}
		</HStack>
	{/snippet}
	{#snippet end()}
		<!-- New session -->
		<Button variant="ghost" size="icon-sm" onclick={onNewSession} aria-label="New session">
			<Icon name="plus" size="md" />
		</Button>
	{/snippet}
</SectionHeader>
