<script lang="ts">
	import MessageBubbleIcon from "@lucide/svelte/icons/message-square";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import EmptyState from "$lib/components/shared/EmptyState.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import SessionHeader from "./SessionHeader.svelte";
	import MessageBubble from "./MessageBubble.svelte";
	import MessageInput from "./MessageInput.svelte";
	import StreamingIndicator from "./StreamingIndicator.svelte";
	import ToolCallSummary from "$lib/components/tool/ToolCallSummary.svelte";
	import ToolApprovalDialog from "$lib/components/tool/ToolApprovalDialog.svelte";
	import ContextEntryComponent from "./ContextEntry.svelte";
	import ThinkingBlock from "$lib/components/shared/ThinkingBlock.svelte";
	import { conversationStore } from "$lib/stores/conversation.svelte";
import { sessionStore } from "$lib/stores/session.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import type { Message } from "$lib/types";
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
		function handleKeydown(event: KeyboardEvent) {
			if ((event.ctrlKey || event.metaKey) && event.key === "n") {
				event.preventDefault();
				handleNewSession();
			}
		}
		window.addEventListener("keydown", handleKeydown);
		return () => window.removeEventListener("keydown", handleKeydown);
	});

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
				} catch {
					// Non-critical — proceed without restoring
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

	function handleScroll() {
		if (!scrollViewportRef) return;
		const { scrollTop, scrollHeight, clientHeight } = scrollViewportRef;
		const distanceFromBottom = scrollHeight - scrollTop - clientHeight;
		userScrolledUp = distanceFromBottom > 100;
	}

	function scrollToBottom() {
		userScrolledUp = false;
		if (scrollViewportRef) {
			scrollViewportRef.scrollTop = scrollViewportRef.scrollHeight;
		}
	}

	function handleSend(content: string) {
		if (!session) return;
		userScrolledUp = false;
		showResumeBanner = false;
		conversationStore.sendMessage(session.id, content);
	}

	function handleStop() {
		if (!session) return;
		conversationStore.stopStreaming(session.id);
	}

	async function handleNewSession() {
		const project = projectStore.activeProject;
		if (!project) return;
		conversationStore.clear();
		await sessionStore.createSession(project.id);
	}

	async function handleSelectSession(sessionId: number) {
		conversationStore.clear();
		await sessionStore.selectSession(sessionId);
	}

	async function handleDeleteSession(sessionId: number) {
		await sessionStore.deleteSession(sessionId);
	}

	function handleUpdateTitle(title: string) {
		if (!session) return;
		sessionStore.updateTitle(session.id, title);
	}

	// Determine if the last message is a streaming assistant message
	const lastMessage = $derived(messages.length > 0 ? messages[messages.length - 1] : null);
	const isLastMessageStreaming = $derived(
		lastMessage !== null &&
			lastMessage.role === "assistant" &&
			lastMessage.stream_status === "pending"
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
					(messages[j].content_type === "tool_use" ||
						messages[j].content_type === "tool_result")
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

<div class="flex h-full flex-col">
	{#if !initialized}
		<div class="flex h-full items-center justify-center">
			<LoadingSpinner />
		</div>
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
			<div class="flex items-center justify-between border-b border-border bg-muted/50 px-4 py-2 text-sm text-muted-foreground">
				<span>Session resumed after restart. Send a message to continue.</span>
				<button
					class="ml-2 text-xs hover:text-foreground"
					onclick={() => { showResumeBanner = false; }}
				>
					Dismiss
				</button>
			</div>
		{/if}

		<!-- Message area -->
		<div class="relative flex-1 overflow-hidden">
			{#if isLoading}
				<div class="flex h-full items-center justify-center">
					<LoadingSpinner />
				</div>
			{:else if error}
				<div class="flex h-full items-center justify-center p-4">
					<ErrorDisplay
						message={error}
						onRetry={() => {
							if (session) conversationStore.loadMessages(session.id);
						}}
					/>
				</div>
			{:else if messages.length === 0 && !isStreaming}
				<div class="flex h-full items-center justify-center">
					<EmptyState
						icon={MessageBubbleIcon}
						title="No messages yet"
						description="Send a message to start the conversation."
					/>
				</div>
			{:else}
				<ScrollArea class="h-full" bind:viewportRef={scrollViewportRef}>
					<div class="space-y-4 p-4" onscroll={handleScroll}>
						<!-- Context entries — inline system messages showing what was sent to Claude -->
						{#each contextEntries as entry, i (entry.type + i)}
							<ContextEntryComponent {entry} />
						{/each}

						{#each displayEntries as entry (entry.kind === "message" ? entry.message.id : entry.key)}
							{#if entry.kind === "tool-summary"}
								<div class="px-4">
									<ToolCallSummary messages={entry.messages} />
								</div>
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
							<div class="px-4">
								<ThinkingBlock content={streamingThinking} isStreaming={isStreaming} />
							</div>
						{/if}

						<!-- Tool approval dialog — rendered inline in the message stream -->
						{#if pendingApproval}
							<div class="px-4">
								<ToolApprovalDialog
									approval={pendingApproval}
									onApprove={() => conversationStore.respondToApproval(true)}
									onDeny={() => conversationStore.respondToApproval(false)}
								/>
							</div>
						{/if}

						<!-- Process violation warnings -->
						{#if processViolations.length > 0}
							<div class="space-y-1 px-4">
								{#each processViolations as violation (violation.check)}
									<div class="rounded-md border border-warning/30 bg-warning/10 px-3 py-2 text-sm text-warning">
										<span class="font-medium">Process:</span> {violation.message}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</ScrollArea>

				<!-- Scroll to bottom button -->
				{#if userScrolledUp}
					<button
						class="absolute bottom-2 left-1/2 -translate-x-1/2 rounded-full border border-border bg-background px-3 py-1 text-xs text-muted-foreground shadow-md transition-colors hover:bg-muted"
						onclick={scrollToBottom}
					>
						Scroll to bottom
					</button>
				{/if}
			{/if}
		</div>

		<!-- Input area -->
		<MessageInput
			{isStreaming}
			onsend={handleSend}
			onstop={handleStop}
		/>
	{:else}
		<!-- No session selected -->
		<div class="flex h-full items-center justify-center">
			<EmptyState
				icon={MessageBubbleIcon}
				title="No session active"
				description="Select or create a session to begin chatting."
				action={{
					label: "New Session",
					onclick: handleNewSession,
				}}
			/>
		</div>
	{/if}
</div>
