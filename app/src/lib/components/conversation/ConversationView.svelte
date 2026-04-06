<script lang="ts">
	import {
		ScrollArea,
		EmptyState,
		LoadingSpinner,
		ErrorDisplay,
		Button,
		Text,
		Stack,
		Box,
		Center,
		Panel,
		SectionHeader,
	} from "@orqastudio/svelte-components/pure";
	import SessionHeader from "./SessionHeader.svelte";
	import MessageBubble from "./MessageBubble.svelte";
	import MessageInput from "./MessageInput.svelte";
	import StreamingIndicator from "./StreamingIndicator.svelte";
	import ToolCallSummary from "$lib/components/tool/ToolCallSummary.svelte";
	import ToolApprovalDialog from "$lib/components/tool/ToolApprovalDialog.svelte";
	import ContextEntryComponent from "./ContextEntry.svelte";
	import { ThinkingBlock } from "@orqastudio/svelte-components/pure";
	import { getStores, logger } from "@orqastudio/sdk";

	const log = logger("conversation");
	const { conversationStore, sessionStore, projectStore, settingsStore } = getStores();
	import type { Message } from "@orqastudio/types";
	import { onMount } from "svelte";

	let scrollViewportRef = $state<HTMLElement | null>(null);
	let userScrolledUp = $state(false);
	let initialized = $state(false);
	let showResumeBanner = $state(false);

	const session = $derived(sessionStore.activeSession);
	const sessions = $derived(sessionStore.sessions);
	const messages = $derived(conversationStore.messages);
	const isStreaming = $derived(conversationStore.isStreaming);
	const isLoading = $derived(conversationStore.isLoading);
	const error = $derived(conversationStore.error);
	const streamingContent = $derived(conversationStore.streamingContent);
	const activeToolCalls = $derived(conversationStore.activeToolCalls);
	const pendingApproval = $derived(conversationStore.pendingApproval);
	const processViolations = $derived(conversationStore.processViolations);
	const contextEntries = $derived(conversationStore.contextEntries);
	const streamingThinking = $derived(conversationStore.streamingThinking);

	// Restore last session on mount
	onMount(() => {
		restoreLastSession();

		// Keyboard shortcut: Ctrl+N for new session
		/**
		 * Handle global keyboard shortcuts registered during the component's lifetime.
		 * @param event - The keyboard event fired on the window.
		 */
		function handleKeydown(event: KeyboardEvent) {
			if ((event.ctrlKey || event.metaKey) && event.key === "n") {
				event.preventDefault();
				handleNewSession();
			}
		}
		window.addEventListener("keydown", handleKeydown);
		return () => window.removeEventListener("keydown", handleKeydown);
	});

	/** Load the session list and restore the last active session, or create a new one if none exists. */
	async function restoreLastSession() {
		const project = projectStore.activeProject;
		if (!project) {
			initialized = true;
			return;
		}

		// Load sessions list for the dropdown
		await sessionStore.loadSessions(project.id);

		// Try to restore the last active session
		if (!sessionStore.hasActiveSession) {
			const lastSessionId = settingsStore.lastSessionId;
			if (lastSessionId !== null) {
				try {
					await sessionStore.restoreSession(lastSessionId);
					if (sessionStore.hasActiveSession) {
						showResumeBanner = true;
					}
				} catch (err) {
					log.error("Failed to restore last session", { lastSessionId, err });
				}
			}
		}

		// Auto-create a session if none was restored
		if (!sessionStore.hasActiveSession) {
			await sessionStore.createSession(project.id);
		}

		initialized = true;
	}

	// Load messages when session changes
	$effect(() => {
		if (session) {
			conversationStore.loadMessages(session.id);
		} else {
			conversationStore.clear();
		}
	});

	// Propagate auto-generated title updates from the conversation store to the session store.
	// This keeps the stores decoupled: conversation store exposes the event data as reactive
	// state, and this component (which already owns both stores) performs the coordination.
	$effect(() => {
		const update = conversationStore.lastTitleUpdate;
		if (update) {
			sessionStore.handleTitleUpdate(update.sessionId, update.title);
		}
	});

	// Auto-scroll to bottom when new content arrives, unless user scrolled up
	$effect(() => {
		// Track dependencies: messages, streamingContent, and activeToolCalls
		void messages.length;
		void streamingContent;
		void activeToolCalls.size;
		void contextEntries.length;

		if (!userScrolledUp && scrollViewportRef) {
			requestAnimationFrame(() => {
				if (scrollViewportRef) {
					scrollViewportRef.scrollTop = scrollViewportRef.scrollHeight;
				}
			});
		}
	});

	/** Update userScrolledUp state based on distance from the bottom of the scroll container. */
	function handleScroll() {
		if (!scrollViewportRef) return;
		const { scrollTop, scrollHeight, clientHeight } = scrollViewportRef;
		const distanceFromBottom = scrollHeight - scrollTop - clientHeight;
		userScrolledUp = distanceFromBottom > 100;
	}

	/** Reset the scroll offset to the bottom of the message list. */
	function scrollToBottom() {
		userScrolledUp = false;
		if (scrollViewportRef) {
			scrollViewportRef.scrollTop = scrollViewportRef.scrollHeight;
		}
	}

	/**
	 * Send a user message to the active session and dismiss the resume banner.
	 * @param content - The text content of the message to send.
	 */
	function handleSend(content: string) {
		if (!session) return;
		userScrolledUp = false;
		showResumeBanner = false;
		conversationStore.sendMessage(session.id, content);
	}

	/** Abort the in-progress streaming response for the active session. */
	function handleStop() {
		if (!session) return;
		conversationStore.stopStreaming(session.id);
	}

	/** Create a new conversation session and clear the current message history. */
	async function handleNewSession() {
		const project = projectStore.activeProject;
		if (!project) return;
		conversationStore.clear();
		await sessionStore.createSession(project.id);
	}

	/**
	 * Switch the active session and clear current messages before loading the selected one.
	 * @param sessionId - The numeric ID of the session to select.
	 */
	async function handleSelectSession(sessionId: number) {
		conversationStore.clear();
		await sessionStore.selectSession(sessionId);
	}

	/**
	 * Delete the specified session from the session store.
	 * @param sessionId - The numeric ID of the session to delete.
	 */
	async function handleDeleteSession(sessionId: number) {
		await sessionStore.deleteSession(sessionId);
	}

	/**
	 * Update the title of the active session.
	 * @param title - The new title string to apply to the session.
	 */
	function handleUpdateTitle(title: string) {
		if (!session) return;
		sessionStore.updateTitle(session.id, title);
	}

	// Determine if the last message is a streaming assistant message
	const lastMessage = $derived(messages.length > 0 ? messages[messages.length - 1] : null);
	const isLastMessageStreaming = $derived(
		lastMessage !== null &&
			lastMessage.role === "assistant" &&
			lastMessage.stream_status === "pending",
	);

	// Convert active tool calls map to array for the streaming indicator
	const toolCallsArray = $derived(Array.from(activeToolCalls.values()));

	// Group messages into display entries: regular messages + tool summary groups
	type DisplayEntry =
		| { kind: "message"; message: Message }
		| { kind: "tool-summary"; messages: Message[]; key: number };

	const displayEntries = $derived.by(() => {
		const entries: DisplayEntry[] = [];
		let i = 0;
		while (i < messages.length) {
			const msg = messages[i];
			if (msg.content_type === "tool_use" || msg.content_type === "tool_result") {
				// Collect consecutive tool messages
				const toolGroup: Message[] = [msg];
				let j = i + 1;
				while (
					j < messages.length &&
					(messages[j].content_type === "tool_use" || messages[j].content_type === "tool_result")
				) {
					toolGroup.push(messages[j]);
					j++;
				}
				entries.push({ kind: "tool-summary", messages: toolGroup, key: toolGroup[0].id });
				i = j;
			} else {
				entries.push({ kind: "message", message: msg });
				i++;
			}
		}
		return entries;
	});
</script>

<Stack height="full">
	{#if !initialized}
		<Center full>
			<LoadingSpinner />
		</Center>
	{:else if session}
		<!-- Session header -->
		<SessionHeader
			{session}
			{sessions}
			sessionsLoading={sessionStore.isLoading}
			sessionsError={sessionStore.error}
			onNewSession={handleNewSession}
			onUpdateTitle={handleUpdateTitle}
			onSelectSession={handleSelectSession}
			onDeleteSession={handleDeleteSession}
			onRetryLoadSessions={() => {
				const project = projectStore.activeProject;
				if (project) sessionStore.loadSessions(project.id);
			}}
		/>

		<!-- Resume notification banner -->
		{#if showResumeBanner}
			<SectionHeader>
				{#snippet start()}
					<Text tone="muted">Session resumed after restart. Send a message to continue.</Text>
				{/snippet}
				{#snippet end()}
					<Button
						variant="ghost"
						size="sm"
						onclick={() => {
							showResumeBanner = false;
						}}
					>
						Dismiss
					</Button>
				{/snippet}
			</SectionHeader>
		{/if}

		<!-- Message area; relative + flex-1 are structural layout constraints -->
		<Box position="relative" flex={1}>
			{#if isLoading}
				<Center full>
					<LoadingSpinner />
				</Center>
			{:else if error}
				<Center full>
					<Panel padding="normal">
						<ErrorDisplay
							message={error}
							onRetry={() => {
								if (session) conversationStore.loadMessages(session.id);
							}}
						/>
					</Panel>
				</Center>
			{:else if messages.length === 0 && !isStreaming}
				<Center full>
					<EmptyState
						icon="message-square"
						title="No messages yet"
						description="Send a message to start the conversation."
					/>
				</Center>
			{:else}
				<ScrollArea full bind:viewportRef={scrollViewportRef}>
					<!-- onscroll requires a raw DOM element; no ORQA primitive supports scroll event handlers -->
					<div class="p-4" onscroll={handleScroll}>
						<Stack gap={4}>
							<!-- Context entries — inline system messages showing what was sent to Claude -->
							{#each contextEntries as entry, i (entry.type + i)}
								<ContextEntryComponent {entry} />
							{/each}

							{#each displayEntries as entry (entry.kind === "message" ? entry.message.id : entry.key)}
								{#if entry.kind === "tool-summary"}
									<Panel padding="normal">
										<ToolCallSummary messages={entry.messages} />
									</Panel>
								{:else}
									<MessageBubble
										message={entry.message}
										streamingContent={isLastMessageStreaming && entry.message.id === lastMessage?.id
											? streamingContent
											: undefined}
									/>
								{/if}
							{/each}

							<!-- Streaming activity indicator -->
							{#if isStreaming}
								<StreamingIndicator
									hasContent={streamingContent.length > 0}
									toolCalls={toolCallsArray}
								/>
							{/if}

							<!-- Thinking block — ephemeral reasoning display below activity -->
							{#if streamingThinking}
								<Panel padding="normal">
									<ThinkingBlock content={streamingThinking} {isStreaming} />
								</Panel>
							{/if}

							<!-- Tool approval dialog — rendered inline in the message stream -->
							{#if pendingApproval}
								<Panel padding="normal">
									<ToolApprovalDialog
										approval={pendingApproval}
										onApprove={() => conversationStore.respondToApproval(true)}
										onDeny={() => conversationStore.respondToApproval(false)}
									/>
								</Panel>
							{/if}

							<!-- Process violation warnings -->
							{#if processViolations.length > 0}
								<Stack gap={1}>
									{#each processViolations as violation (violation.check)}
										<!-- warning/10 bg and warning/30 border are design tokens; Box only supports named bg tokens -->
										<div class="border-warning/30 bg-warning/10 rounded-md border px-3 py-2">
											<Text tone="warning"
												><Text variant="body-strong">Process:</Text> {violation.message}</Text
											>
										</div>
									{/each}
								</Stack>
							{/if}
						</Stack>
					</div>
				</ScrollArea>

				<!-- Scroll to bottom button; absolute + translate-x positioning has no ORQA primitive -->
				{#if userScrolledUp}
					<div class="absolute bottom-2 left-1/2 -translate-x-1/2">
						<Button variant="outline" size="sm" onclick={scrollToBottom}>Scroll to bottom</Button>
					</div>
				{/if}
			{/if}
		</Box>

		<!-- Input area -->
		<MessageInput {isStreaming} onsend={handleSend} onstop={handleStop} />
	{:else}
		<!-- No session selected -->
		<Center full>
			<EmptyState
				icon="message-square"
				title="No session active"
				description="Select or create a session to begin chatting."
				action={{
					label: "New Session",
					onclick: handleNewSession,
				}}
			/>
		</Center>
	{/if}
</Stack>
