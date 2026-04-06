<!-- Session picker: compact bar above the log filter strip that lets the
     developer switch between historical devtools sessions and the live feed.

     When viewingHistorical is false (default), this bar shows a compact
     "Sessions ▼" dropdown trigger. Clicking it opens a list of all sessions
     ordered newest-first. Clicking a historical session loads its events from
     SQLite and disables live streaming. A "Return to live" button appears while
     viewing a historical session and reactivates the ring buffer stream. -->
<script lang="ts">
	import { onMount } from "svelte";
	import { Button, HStack, Caption, Badge, Input } from "@orqastudio/svelte-components/pure";
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
	 * @param e - The document click event used to check the click target.
	 */
	function handleDocumentClick(e: MouseEvent): void {
		const target = e.target as HTMLElement;
		if (!target.closest("[data-session-picker]")) {
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

<!-- Session picker bar: always visible above the filter strip.
     Scoped CSS class provides layout; no Tailwind utility classes used. -->
<div class="session-picker" data-session-picker>
	{#if viewingHistorical.value}
		<!-- Historical banner: shown when a past session is being viewed.
		     HStack provides the flex-row layout. -->
		<HStack justify="between" full gap={2}>
			<!-- Scoped div provides the primary color and italic styling for the label. -->
			<div class="session-picker__historical-label">
				<Caption truncate>
					Viewing: {displayedSession ? sessionDisplayLabel(displayedSession) : "historical session"}
				</Caption>
			</div>
			<!-- Wrapper span with display:contents provides :global() hook for Button override. -->
			<span class="session-picker__return-wrap" style="display: contents;">
				<Button variant="ghost" size="icon-sm" onclick={handleReturnToLive}>Return to live</Button>
			</span>
		</HStack>
	{:else}
		<!-- Compact trigger: shows session count and opens dropdown.
		     Scoped class provides the flex alignment. -->
		<div class="session-picker__trigger-row">
			<span class="session-picker__trigger-wrap" style="display: contents;">
				<Button
					variant="ghost"
					size="icon-sm"
					onclick={toggleDropdown}
					aria-haspopup="listbox"
					aria-expanded={dropdownOpen}
				>
					Sessions ({sessions.length})
					<!-- Chevron rendered inline; no raw span needed. -->
					{dropdownOpen ? "▲" : "▼"}
				</Button>
			</span>
		</div>
	{/if}

	<!-- Dropdown panel: session list. Scoped CSS class provides positioning. -->
	{#if dropdownOpen}
		<div class="session-picker__panel" role="listbox" aria-label="Session list">
			{#if sessions.length === 0}
				<!-- Empty message: scoped div provides padding + centering. -->
				<div class="session-picker__empty">
					<Caption>No sessions yet</Caption>
				</div>
			{:else}
				{#each sessions as session (session.id)}
					<!-- Each session row: click to view, "..." for context menu.
					     Scoped class + data attrs drive row styling. -->
					<div
						class="session-picker__row"
						data-current={session.is_current}
						data-active={activeSessionId.value === session.id}
						role="option"
						aria-selected={activeSessionId.value === session.id ||
							(session.is_current && !viewingHistorical.value)}
					>
						{#if renamingId === session.id}
							<!-- Inline rename: ORQA Input component replaces raw input.
							     Wrapper span provides :global() hook for compact fit override. -->
							<span class="session-picker__rename-wrap">
								<Input
									type="text"
									bind:value={renameValue}
									onblur={commitRename}
									onkeydown={handleRenameKeydown}
									placeholder="Session label…"
									autofocus
								/>
							</span>
						{:else}
							<!-- Clickable session info: wrapper span for Button override. -->
							<span class="session-picker__row-content-wrap" style="display: contents;">
								<Button variant="ghost" onclick={() => handleSelectSession(session)}>
									<div class="session-picker__session-label">
										{sessionDisplayLabel(session)}
										{#if session.is_current}
											<!-- Badge success variant provides the live indicator. -->
											<Badge variant="success" size="xs">live</Badge>
										{:else if session.ended_at === null}
											<!-- Badge warning variant provides the interrupted indicator. -->
											<Badge variant="warning" size="xs">interrupted</Badge>
										{/if}
									</div>
									<Caption>
										{sessionDuration(session)} &middot; {session.event_count} events
									</Caption>
								</Button>
							</span>

							<!-- Context menu trigger: wrapper span for Button override. -->
							<span class="session-picker__menu-wrap" style="display: contents;">
								<Button
									variant="ghost"
									onclick={(e) => toggleContextMenu(session.id, e)}
									aria-label="Session options"
								>
									&hellip;
								</Button>
							</span>

							<!-- Context menu panel: scoped CSS class provides positioning. -->
							{#if contextMenuId === session.id}
								<div class="session-picker__context-menu">
									<!-- Wrapper span for context item Button override. -->
									<span class="session-picker__context-item-wrap" style="display: contents;">
										<Button variant="ghost" onclick={(e) => startRename(session, e)}>Rename</Button>
									</span>
									{#if !session.is_current}
										<!-- Danger item gets its own wrapper for color override. -->
										<span
											class="session-picker__context-item-wrap session-picker__context-item-danger-wrap"
											style="display: contents;"
										>
											<Button variant="ghost" onclick={(e) => handleDelete(session, e)}>
												Delete
											</Button>
										</span>
									{/if}
								</div>
							{/if}
						{/if}
					</div>
				{/each}
			{/if}
		</div>
	{/if}
</div>

<style>
	/* Picker bar: compact strip with a subtle separator. */
	.session-picker {
		position: relative;
		border-bottom: 1px solid var(--color-border);
		background-color: var(--color-surface-base);
		display: flex;
		flex-shrink: 0;
		align-items: center;
		padding: 0 var(--spacing-2);
		min-height: 28px;
	}

	/* Trigger row: left-aligned. */
	.session-picker__trigger-row {
		display: flex;
		align-items: center;
	}

	/* Trigger button: compact font and height.
	   Targets Button inside the trigger wrapper span. */
	:global(.session-picker__trigger-wrap button) {
		font-size: 11px !important;
		height: 22px !important;
		padding: 0 var(--spacing-1-5) !important;
		gap: var(--spacing-1) !important;
	}

	/* Return to live button: primary text to pair with the banner.
	   Targets Button inside the return wrapper span. */
	:global(.session-picker__return-wrap button) {
		font-size: 11px !important;
		height: 22px !important;
		padding: 0 var(--spacing-1-5) !important;
		flex-shrink: 0;
		color: var(--color-primary) !important;
	}

	/* Dropdown panel. */
	.session-picker__panel {
		position: absolute;
		left: var(--spacing-2);
		top: 100%;
		z-index: 20;
		min-width: 320px;
		max-width: 480px;
		max-height: 16rem;
		overflow-y: auto;
		border-radius: var(--radius-md);
		border: 1px solid var(--color-border);
		background-color: var(--color-surface-raised);
		box-shadow: var(--shadow-lg);
	}

	/* Empty message: centred with padding. */
	.session-picker__empty {
		padding: var(--spacing-3);
		text-align: center;
	}

	/* Session row. */
	.session-picker__row {
		position: relative;
		display: flex;
		align-items: center;
		border-bottom: 1px solid var(--color-border);
	}

	.session-picker__row:last-child {
		border-bottom: none;
	}

	/* Current session: subtle primary tint. */
	.session-picker__row[data-current="true"] {
		background-color: color-mix(in srgb, var(--color-primary) 8%, transparent);
	}

	/* Active (being viewed) session: stronger primary tint. */
	.session-picker__row[data-active="true"] {
		background-color: color-mix(in srgb, var(--color-primary) 15%, transparent);
	}

	/* Main clickable content area of a row: column layout, fills available width.
	   Targets Button inside the row-content wrapper span. */
	:global(.session-picker__row-content-wrap button) {
		display: flex !important;
		flex: 1 !important;
		flex-direction: column !important;
		align-items: flex-start !important;
		gap: 2px !important;
		padding: var(--spacing-1-5) var(--spacing-2) !important;
		text-align: left !important;
		min-width: 0 !important;
		height: auto !important;
		border-radius: 0 !important;
		justify-content: flex-start !important;
	}

	/* Session label row: name + badge indicators. */
	.session-picker__session-label {
		display: flex;
		align-items: center;
		gap: var(--spacing-1-5);
		font-size: 11px;
		font-weight: 500;
		color: var(--color-content-base);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 100%;
	}

	/* Historical session label wrapper: primary color + italic to indicate navigation state. */
	.session-picker__historical-label {
		flex: 1;
		min-width: 0;
		overflow: hidden;
		color: var(--color-primary);
		font-style: italic;
	}

	/* "..." context menu trigger: compact icon-sized button.
	   Targets Button inside the menu wrapper span. */
	:global(.session-picker__menu-wrap button) {
		flex-shrink: 0 !important;
		width: 24px !important;
		height: 100% !important;
		font-size: 14px !important;
		padding: 0 !important;
		border-radius: 0 !important;
	}

	/* Context menu floating panel. */
	.session-picker__context-menu {
		position: absolute;
		right: 28px;
		top: 0;
		z-index: 30;
		min-width: 100px;
		border-radius: var(--radius-md);
		border: 1px solid var(--color-border);
		background-color: var(--color-surface-raised);
		box-shadow: var(--shadow-lg);
		overflow: hidden;
	}

	/* Context menu items: full-width compact buttons.
	   Targets Button inside the context item wrapper span. */
	:global(.session-picker__context-item-wrap button) {
		display: flex !important;
		width: 100% !important;
		padding: var(--spacing-1-5) var(--spacing-2) !important;
		font-size: 11px !important;
		text-align: left !important;
		justify-content: flex-start !important;
		height: auto !important;
		border-radius: 0 !important;
	}

	/* Danger variant for the delete action. */
	:global(.session-picker__context-item-danger-wrap button) {
		color: var(--color-destructive) !important;
	}

	:global(.session-picker__context-item-danger-wrap button:hover) {
		background-color: color-mix(in srgb, var(--color-destructive) 10%, transparent) !important;
	}

	/* Inline rename input wrapper: provides compact fit within the session row. */
	.session-picker__rename-wrap {
		flex: 1;
		display: flex;
		align-items: center;
		margin: var(--spacing-1) var(--spacing-2);
	}

	/* Input inside rename wrapper: compact height and font. */
	:global(.session-picker__rename-wrap input) {
		height: 22px !important;
		padding: 0 var(--spacing-1-5) !important;
		font-size: 11px !important;
	}
</style>
