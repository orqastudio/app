<!-- Session picker: compact bar above the log filter strip that lets the
     developer switch between historical devtools sessions and the live feed.

     When viewingHistorical is false (default), this bar shows a compact
     "Sessions ▼" dropdown trigger. Clicking it opens a list of all sessions
     ordered newest-first. Clicking a historical session loads its events from
     SQLite and disables live streaming. A "Return to live" button appears while
     viewing a historical session and reactivates the ring buffer stream. -->
<script lang="ts">
	import { onMount } from "svelte";
	import {
		Button,
		Caption,
		Badge,
		Input,
		HStack,
		Box,
		Center,
		PickerShell,
		SelectPanel,
		SelectRow,
		ContextMenu,
	} from "@orqastudio/svelte-components/pure";
	import {
		sessions,
		viewingHistorical,
		activeSessionId,
		loadSessions,
		switchToSession,
		switchToCurrentSession,
		renameSession,
		deleteSession,
		sessionDisplayLabel,
		sessionDuration,
		type DevToolsSession,
	} from "../../stores/session-store.svelte.js";
	import { enterHistoricalMode, exitHistoricalMode } from "../../stores/log-store.svelte.js";

	// Whether the session dropdown is open.
	let dropdownOpen = $state(false);

	// The session currently being renamed (null when not renaming).
	let renamingId = $state<string | null>(null);

	// Draft label value during an inline rename.
	let renameValue = $state("");

	// The session for which the context menu is open.
	let contextMenuId = $state<string | null>(null);

	// Load sessions when the picker mounts so the list is ready before the user
	// opens the dropdown.
	onMount(() => {
		loadSessions();
	});

	// The currently displayed session — the one being viewed or the current session.
	const displayedSession = $derived(
		sessions.find((s) =>
			viewingHistorical.value ? s.id === activeSessionId.value : s.is_current,
		) ??
			sessions[0] ??
			null,
	);

	/** Toggle the session dropdown open or closed. */
	function toggleDropdown(): void {
		dropdownOpen = !dropdownOpen;
		contextMenuId = null;
	}

	/**
	 * Close all overlays when the user clicks outside the session picker.
	 * Uses [data-slot="picker-shell"] to identify the picker boundary.
	 * @param e - The document click event used to check the click target.
	 */
	function handleDocumentClick(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (!target.closest('[data-slot="picker-shell"]')) {
			dropdownOpen = false;
			contextMenuId = null;
			if (renamingId !== null) {
				renamingId = null;
			}
		}
	}

	/**
	 * Switch to a session and load its events from SQLite, then close the dropdown.
	 * @param session - The session to activate and load events for.
	 * @returns Resolves after the session switch and event load complete.
	 */
	async function handleSelectSession(session: DevToolsSession): Promise<void> {
		if (session.is_current && !viewingHistorical.value) {
			dropdownOpen = false;
			return;
		}
		dropdownOpen = false;
		await switchToSession(session.id);
		await enterHistoricalMode(session);
	}

	/**
	 * Start the inline rename flow for the given session by setting the rename state.
	 * @param session - The session to rename.
	 * @param e - The click event; propagation is stopped to avoid row selection.
	 */
	function startRename(session: DevToolsSession, e: MouseEvent): void {
		e.stopPropagation();
		contextMenuId = null;
		renamingId = session.id;
		renameValue = session.label ?? "";
	}

	/** Commit the pending inline rename when Enter is pressed or the input blurs. */
	async function commitRename(): Promise<void> {
		if (renamingId === null) return;
		const id = renamingId;
		const label = renameValue.trim();
		renamingId = null;
		if (label.length > 0) {
			await renameSession(id, label);
		}
	}

	/**
	 * Handle keydown in the rename input: Enter commits the rename, Escape cancels it.
	 * @param e - The keyboard event from the rename input field.
	 */
	function handleRenameKeydown(e: KeyboardEvent): void {
		if (e.key === "Enter") {
			e.preventDefault();
			commitRename();
		} else if (e.key === "Escape") {
			renamingId = null;
		}
	}

	/**
	 * Confirm and delete a session and all its stored events.
	 * @param session - The session to permanently delete.
	 * @param e - The click event; propagation is stopped to avoid row selection.
	 * @returns Resolves after the deletion and list reload complete.
	 */
	async function handleDelete(session: DevToolsSession, e: MouseEvent): Promise<void> {
		e.stopPropagation();
		contextMenuId = null;
		await deleteSession(session.id);
	}

	/**
	 * Toggle the context menu open or closed for the given session row.
	 * @param sessionId - The ID of the session whose context menu to toggle.
	 * @param e - The click event; propagation is stopped to avoid row selection.
	 */
	function toggleContextMenu(sessionId: string, e: MouseEvent): void {
		e.stopPropagation();
		contextMenuId = contextMenuId === sessionId ? null : sessionId;
	}

	/**
	 * Return to the live feed by exiting historical mode in both the session and log stores.
	 * @returns Resolves after the stores have switched back to live mode.
	 */
	async function handleReturnToLive(): Promise<void> {
		await switchToCurrentSession();
		await exitHistoricalMode();
	}
</script>

<svelte:document onclick={handleDocumentClick} />

<!-- PickerShell: positioned strip with border-b and bg-surface. position:relative
     anchors the SelectPanel dropdown via position:absolute. -->
<PickerShell>
	{#if viewingHistorical.value}
		<!-- Historical mode: label on left, return button on right. -->
		<HStack justify="between" full gap={2}>
			<Box flex={1} minWidth={0}>
				<Caption truncate tone="primary" italic>
					Viewing: {displayedSession ? sessionDisplayLabel(displayedSession) : "historical session"}
				</Caption>
			</Box>
			<Button variant="ghost" size="xs" onclick={handleReturnToLive}>Return to live</Button>
		</HStack>
	{:else}
		<!-- Live mode: trigger button opens the session list dropdown. -->
		<Button
			variant="ghost"
			size="xs"
			onclick={toggleDropdown}
			aria-haspopup="listbox"
			aria-expanded={dropdownOpen}
		>
			Sessions ({sessions.length})
			{dropdownOpen ? "▲" : "▼"}
		</Button>
	{/if}

	<!-- Session dropdown panel: absolutely positioned below the picker strip. -->
	{#if dropdownOpen}
		<SelectPanel role="listbox" aria-label="Session list">
			{#if sessions.length === 0}
				<!-- Empty list: centered caption in a non-interactive row. -->
				<Center>
					<Caption>No sessions yet</Caption>
				</Center>
			{:else}
				{#each sessions as session (session.id)}
					<!-- Each row: SelectRow provides the relative position context for the
					     context menu panel and the tinted current/active backgrounds. -->
					<SelectRow
						current={session.is_current}
						active={activeSessionId.value === session.id}
						role="option"
						aria-selected={activeSessionId.value === session.id ||
							(session.is_current && !viewingHistorical.value)}
					>
						{#if renamingId === session.id}
							<!-- Inline rename: compact Input fills the row. -->
							<Box flex={1} minWidth={0}>
								<Input
									size="compact"
									type="text"
									bind:value={renameValue}
									onblur={commitRename}
									onkeydown={handleRenameKeydown}
									placeholder="Session label…"
									autofocus
								/>
							</Box>
						{:else}
							<!-- Main content button: column layout, fills available width. -->
							<Button
								variant="ghost"
								size="col-item"
								full
								onclick={() => handleSelectSession(session)}
							>
								<!-- Session name row: label text + live/interrupted badge. -->
								<HStack gap={1}>
									<Caption truncate>{sessionDisplayLabel(session)}</Caption>
									{#if session.is_current}
										<Badge variant="success" size="xs">live</Badge>
									{:else if session.ended_at === null}
										<Badge variant="warning" size="xs">interrupted</Badge>
									{/if}
								</HStack>
								<!-- Duration and event count sub-row. -->
								<Caption>
									{sessionDuration(session)} &middot; {session.event_count} events
								</Caption>
							</Button>

							<!-- Context menu trigger: icon-sm ghost button at the row end. -->
							<Button
								variant="ghost"
								size="icon-sm"
								onclick={(e) => toggleContextMenu(session.id, e)}
								aria-label="Session options"
							>
								&hellip;
							</Button>

							<!-- Context menu: floats above the row right edge when open. -->
							{#if contextMenuId === session.id}
								<ContextMenu>
									<Button
										variant="ghost"
										size="col-item"
										full
										onclick={(e) => startRename(session, e)}
									>
										Rename
									</Button>
									{#if !session.is_current}
										<Button
											variant="ghost-destructive"
											size="col-item"
											full
											onclick={(e) => handleDelete(session, e)}
										>
											Delete
										</Button>
									{/if}
								</ContextMenu>
							{/if}
						{/if}
					</SelectRow>
				{/each}
			{/if}
		</SelectPanel>
	{/if}
</PickerShell>
