<!-- Session picker: compact bar above the log filter strip that lets the
     developer switch between historical devtools sessions and the live feed.

     When viewingHistorical is false (default), this bar shows a compact
     "Sessions ▼" dropdown trigger. Clicking it opens a list of all sessions
     ordered newest-first. Clicking a historical session loads its events from
     SQLite and disables live streaming. A "Return to live" button appears while
     viewing a historical session and reactivates the ring buffer stream. -->
<script lang="ts">
	import { onMount } from "svelte";
	import { Button, HStack, Caption } from "@orqastudio/svelte-components/pure";
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
	import {
		enterHistoricalMode,
		exitHistoricalMode,
	} from "../../stores/log-store.svelte.js";

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
			viewingHistorical.value
				? s.id === activeSessionId.value
				: s.is_current
		) ?? sessions[0] ?? null
	);

	// Toggle the dropdown open/closed.
	function toggleDropdown(): void {
		dropdownOpen = !dropdownOpen;
		contextMenuId = null;
	}

	// Close all overlays when the user clicks outside the picker.
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

	// Switch to a session and close the dropdown. Updates session-store state
	// then delegates to log-store to load events from SQLite.
	async function handleSelectSession(session: DevToolsSession): Promise<void> {
		if (session.is_current && !viewingHistorical.value) {
			dropdownOpen = false;
			return;
		}
		dropdownOpen = false;
		await switchToSession(session.id);
		await enterHistoricalMode(session);
	}

	// Start the inline rename flow for a session.
	function startRename(session: DevToolsSession, e: MouseEvent): void {
		e.stopPropagation();
		contextMenuId = null;
		renamingId = session.id;
		renameValue = session.label ?? "";
	}

	// Commit the rename on Enter or blur.
	async function commitRename(): Promise<void> {
		if (renamingId === null) return;
		const id = renamingId;
		const label = renameValue.trim();
		renamingId = null;
		if (label.length > 0) {
			await renameSession(id, label);
		}
	}

	function handleRenameKeydown(e: KeyboardEvent): void {
		if (e.key === "Enter") {
			e.preventDefault();
			commitRename();
		} else if (e.key === "Escape") {
			renamingId = null;
		}
	}

	// Confirm and delete a session.
	async function handleDelete(session: DevToolsSession, e: MouseEvent): Promise<void> {
		e.stopPropagation();
		contextMenuId = null;
		await deleteSession(session.id);
	}

	// Toggle context menu for a session row.
	function toggleContextMenu(sessionId: string, e: MouseEvent): void {
		e.stopPropagation();
		contextMenuId = contextMenuId === sessionId ? null : sessionId;
	}

	// Return to the live feed. Exits historical mode in both session-store and
	// log-store so the ring buffer stream is re-activated.
	async function handleReturnToLive(): Promise<void> {
		await switchToCurrentSession();
		await exitHistoricalMode();
	}
</script>

<svelte:document onclick={handleDocumentClick} />

<!-- Session picker bar: always visible above the filter strip. -->
<div
	class="session-picker"
	style="min-height: 28px;"
	data-session-picker
>
	{#if viewingHistorical.value}
		<!-- Historical banner: shown when a past session is being viewed. -->
		<HStack justify="between" class="w-full gap-2">
			<Caption class="overflow-hidden text-ellipsis whitespace-nowrap italic session-picker__historical-label">
				Viewing: {displayedSession ? sessionDisplayLabel(displayedSession) : "historical session"}
			</Caption>
			<Button
				variant="ghost"
				size="icon-sm"
				class="session-picker__return-btn"
				onclick={handleReturnToLive}
			>
				Return to live
			</Button>
		</HStack>
	{:else}
		<!-- Compact trigger: shows session count and opens dropdown. -->
		<div class="session-picker__trigger-row">
			<Button
				variant="ghost"
				size="icon-sm"
				class="session-picker__trigger"
				onclick={toggleDropdown}
				aria-haspopup="listbox"
				aria-expanded={dropdownOpen}
			>
				Sessions ({sessions.length})
				<span class="session-picker__chevron">{dropdownOpen ? "▲" : "▼"}</span>
			</Button>
		</div>
	{/if}

	<!-- Dropdown panel: session list. -->
	{#if dropdownOpen}
		<div
			class="session-picker__panel"
			role="listbox"
			aria-label="Session list"
		>
			{#if sessions.length === 0}
				<div class="session-picker__empty">No sessions yet</div>
			{:else}
				{#each sessions as session (session.id)}
					<!-- Each session row: click to view, "..." for context menu. -->
					<div
						class="session-picker__row {session.is_current ? 'session-picker__row--current' : ''} {activeSessionId.value === session.id ? 'session-picker__row--active' : ''}"
						role="option"
						aria-selected={activeSessionId.value === session.id || (session.is_current && !viewingHistorical.value)}
					>
						{#if renamingId === session.id}
							<!-- Inline rename input. -->
							<input
								class="session-picker__rename-input"
								type="text"
								bind:value={renameValue}
								onblur={commitRename}
								onkeydown={handleRenameKeydown}
								placeholder="Session label…"
								autofocus
							/>
						{:else}
							<!-- Clickable session info. -->
							<Button
								variant="ghost"
								class="session-picker__row-content"
								onclick={() => handleSelectSession(session)}
							>
								<span class="session-picker__session-label">
									{sessionDisplayLabel(session)}
									{#if session.is_current}
										<span class="session-picker__current-badge">live</span>
									{:else if session.ended_at === null}
										<span class="session-picker__interrupted-badge">interrupted</span>
									{/if}
								</span>
								<Caption>
									{sessionDuration(session)} &middot; {session.event_count} events
								</Caption>
							</Button>

							<!-- Context menu trigger. -->
							<Button
								variant="ghost"
								class="session-picker__menu-btn"
								onclick={(e) => toggleContextMenu(session.id, e)}
								aria-label="Session options"
							>
								&hellip;
							</Button>

							<!-- Context menu panel. -->
							{#if contextMenuId === session.id}
								<div class="session-picker__context-menu">
									<Button
										variant="ghost"
										class="session-picker__context-item"
										onclick={(e) => startRename(session, e)}
									>
										Rename
									</Button>
									{#if !session.is_current}
										<Button
											variant="ghost"
											class="session-picker__context-item session-picker__context-item--danger"
											onclick={(e) => handleDelete(session, e)}
										>
											Delete
										</Button>
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
	}

	/* Trigger row: left-aligned. */
	.session-picker__trigger-row {
		display: flex;
		align-items: center;
	}

	:global(.session-picker__trigger) {
		font-size: 11px !important;
		height: 22px !important;
		padding: 0 var(--spacing-1-5) !important;
		gap: var(--spacing-1) !important;
	}

	:global(.session-picker__return-btn) {
		font-size: 11px !important;
		height: 22px !important;
		padding: 0 var(--spacing-1-5) !important;
		flex-shrink: 0;
		color: var(--color-primary) !important;
	}

	.session-picker__chevron {
		font-size: 9px;
		color: var(--color-content-muted);
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

	.session-picker__empty {
		padding: var(--spacing-3) var(--spacing-3);
		font-size: 11px;
		color: var(--color-content-muted);
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
	.session-picker__row--current {
		background-color: color-mix(in srgb, var(--color-primary) 8%, transparent);
	}

	/* Active (being viewed) session: stronger primary tint. */
	.session-picker__row--active {
		background-color: color-mix(in srgb, var(--color-primary) 15%, transparent);
	}

	/* Main clickable content area of a row — overrides Button defaults for column layout. */
	:global(.session-picker__row-content) {
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

	.session-picker__current-badge {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-success);
		border: 1px solid color-mix(in srgb, var(--color-success) 40%, transparent);
		border-radius: 3px;
		padding: 0 3px;
		flex-shrink: 0;
	}

	.session-picker__interrupted-badge {
		font-size: 9px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-warning);
		border: 1px solid color-mix(in srgb, var(--color-warning) 40%, transparent);
		border-radius: 3px;
		padding: 0 3px;
		flex-shrink: 0;
	}

	/* "..." context menu trigger — overrides Button defaults for icon-sized trigger. */
	:global(.session-picker__menu-btn) {
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

	/* Context menu items — overrides Button defaults for compact menu item display. */
	:global(.session-picker__context-item) {
		display: flex !important;
		width: 100% !important;
		padding: var(--spacing-1-5) var(--spacing-2) !important;
		font-size: 11px !important;
		text-align: left !important;
		justify-content: flex-start !important;
		height: auto !important;
		border-radius: 0 !important;
	}

	:global(.session-picker__context-item--danger) {
		color: var(--color-destructive) !important;
	}

	:global(.session-picker__context-item--danger:hover) {
		background-color: color-mix(in srgb, var(--color-destructive) 10%, transparent) !important;
	}

	/* Historical session label: uses primary color to indicate "info/navigation" state. */
	:global(.session-picker__historical-label) {
		color: var(--color-primary) !important;
	}

	/* Inline rename input. */
	.session-picker__rename-input {
		flex: 1;
		margin: var(--spacing-1) var(--spacing-2);
		height: 22px;
		padding: 0 var(--spacing-1-5);
		font-size: 11px;
		background-color: var(--color-surface-base);
		border: 1px solid var(--color-accent-base);
		border-radius: var(--radius-sm);
		color: var(--color-content-base);
		outline: none;
	}
</style>
