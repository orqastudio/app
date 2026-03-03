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
	import ToolCallCard from "$lib/components/tool/ToolCallCard.svelte";
	import { conversationStore } from "$lib/stores/conversation.svelte";
	import { sessionStore } from "$lib/stores/session.svelte";
	import { projectStore } from "$lib/stores/project.svelte";
	import { forgeInvoke } from "$lib/ipc/invoke";
	import { onMount } from "svelte";

	let scrollViewportRef = $state<HTMLElement | null>(null);
	let userScrolledUp = $state(false);
	let initialized = $state(false);

	const session = $derived(sessionStore.activeSession);
	const sessions = $derived(sessionStore.sessions);
	const messages = $derived(conversationStore.messages);
	const isStreaming = $derived(conversationStore.isStreaming);
	const isLoading = $derived(conversationStore.isLoading);
	const error = $derived(conversationStore.error);
	const streamingContent = $derived(conversationStore.streamingContent);
	const streamingThinking = $derived(conversationStore.streamingThinking);
	const activeToolCalls = $derived(conversationStore.activeToolCalls);

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
			try {
				const allSettings = await forgeInvoke<Record<string, unknown>>(
					"settings_get_all",
					{ scope: "app" }
				);
				const lastSessionId = allSettings["last_session_id"];
				if (typeof lastSessionId === "number" && lastSessionId > 0) {
					await sessionStore.restoreSession(lastSessionId);
				}
			} catch {
				// Non-critical — proceed without restoring
			}
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

	// Auto-scroll to bottom when new content arrives, unless user scrolled up
	$effect(() => {
		// Track dependencies: messages, streamingContent, and activeToolCalls
		void messages.length;
		void streamingContent;
		void activeToolCalls.size;

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

	function handleSelectModel(_model: string) {
		// Model selection will be wired to backend in a future phase
	}

	// Determine if the last message is a streaming assistant message
	const lastMessage = $derived(messages.length > 0 ? messages[messages.length - 1] : null);
	const isLastMessageStreaming = $derived(
		lastMessage !== null &&
			lastMessage.role === "assistant" &&
			lastMessage.stream_status === "pending"
	);

	// Convert active tool calls map to array for display
	const toolCallsArray = $derived(Array.from(activeToolCalls.values()));
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
			resolvedModel={conversationStore.currentModel}
			onNewSession={handleNewSession}
			onUpdateTitle={handleUpdateTitle}
			onSelectModel={handleSelectModel}
			onSelectSession={handleSelectSession}
			onDeleteSession={handleDeleteSession}
		/>

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
						{#each messages as message (message.id)}
							<MessageBubble
								{message}
								streamingContent={isLastMessageStreaming && message.id === lastMessage?.id
									? streamingContent
									: undefined}
							/>
						{/each}

						<!-- Active tool calls during streaming -->
						{#if isStreaming && toolCallsArray.length > 0}
							<div class="space-y-2 px-4">
								{#each toolCallsArray as toolCall (toolCall.toolCallId)}
									<ToolCallCard
										toolName={toolCall.toolName}
										toolInput={toolCall.input || null}
										toolOutput={toolCall.output}
										isError={toolCall.isError}
										isComplete={toolCall.isComplete}
									/>
								{/each}
							</div>
						{/if}

						<!-- Streaming indicator -->
						{#if isStreaming && !streamingContent && toolCallsArray.length === 0}
							<StreamingIndicator isThinking={streamingThinking.length > 0} />
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
		<MessageInput {isStreaming} onsend={handleSend} onstop={handleStop} />
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
