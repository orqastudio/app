<script lang="ts">
	import type { SessionSummary, SessionStatus } from "@orqastudio/types";
	import { assertNever } from "@orqastudio/types";
	import {
		Icon,
		PopoverRoot as Popover,
		PopoverContent,
		PopoverTrigger,
		Button,
		Badge,
		Separator,
		ScrollArea,
		SearchInput,
		ConfirmDialog as ConfirmDeleteDialog,
		EmptyState,
		ErrorDisplay,
		LoadingSpinner,
		Heading,
		HStack,
		Box,
		Caption,
		Text,
	} from "@orqastudio/svelte-components/pure";

	let {
		sessions,
		activeSessionId,
		loading = false,
		error = null,
		onSelect,
		onNewSession,
		onDelete,
		onRetry,
		children,
	}: {
		sessions: SessionSummary[];
		activeSessionId: number | null;
		loading?: boolean;
		error?: string | null;
		onSelect: (sessionId: number) => void;
		onNewSession: () => void;
		onDelete: (sessionId: number) => void;
		onRetry?: () => void;
		children: import("svelte").Snippet;
	} = $props();

	let open = $state(false);
	let searchQuery = $state("");
	let deleteDialogOpen = $state(false);
	let deleteTargetId = $state<number | null>(null);
	let deleteTargetTitle = $state("");

	const filteredSessions = $derived(
		searchQuery.trim().length === 0
			? sessions
			: sessions.filter((s) => {
					const query = searchQuery.trim().toLowerCase();
					const title = (s.title ?? "Untitled").toLowerCase();
					const preview = (s.preview ?? "").toLowerCase();
					return title.includes(query) || preview.includes(query);
				})
	);

	function handleSelect(sessionId: number) {
		onSelect(sessionId);
		open = false;
		searchQuery = "";
	}

	function handleNewSession() {
		onNewSession();
		open = false;
		searchQuery = "";
	}

	function handleDeleteClick(event: MouseEvent, sessionId: number, title: string) {
		event.stopPropagation();
		deleteTargetId = sessionId;
		deleteTargetTitle = title;
		deleteDialogOpen = true;
	}

	function handleDeleteConfirm() {
		if (deleteTargetId !== null) {
			onDelete(deleteTargetId);
			deleteTargetId = null;
			deleteTargetTitle = "";
		}
	}

	function statusVariant(status: SessionStatus): "default" | "secondary" | "destructive" | "outline" {
		switch (status) {
			case "active":
				return "default";
			case "completed":
				return "secondary";
			case "error":
				return "destructive";
			case "abandoned":
				return "outline";
			default:
				return assertNever(status);
		}
	}

	function statusLabel(status: SessionStatus): string {
		switch (status) {
			case "active":
				return "Active";
			case "completed":
				return "Completed";
			case "error":
				return "Error";
			case "abandoned":
				return "Abandoned";
			default:
				return assertNever(status);
		}
	}

	function formatRelativeTime(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffSec = Math.floor(diffMs / 1000);
		const diffMin = Math.floor(diffSec / 60);
		const diffHour = Math.floor(diffMin / 60);
		const diffDay = Math.floor(diffHour / 24);

		if (diffSec < 60) return "just now";
		if (diffMin < 60) return `${diffMin}m ago`;
		if (diffHour < 24) return `${diffHour}h ago`;
		if (diffDay === 1) return "yesterday";
		if (diffDay < 7) return `${diffDay}d ago`;
		if (diffDay < 30) return `${Math.floor(diffDay / 7)}w ago`;
		return date.toLocaleDateString();
	}
</script>

<Popover bind:open>
	<PopoverTrigger>
		{@render children?.()}
	</PopoverTrigger>
	<PopoverContent align="start">
		<!-- Header with New Session -->
		<HStack justify="between">
			<Heading level={5}>Sessions</Heading>
			<Button variant="ghost" size="sm" onclick={handleNewSession}>
				<Icon name="plus" size="sm" />
				New Session
			</Button>
		</HStack>

		<!-- Search -->
		<Box paddingX={3} paddingBottom={2}>
			<SearchInput bind:value={searchQuery} placeholder="Search sessions..." size="sm" />
		</Box>

		<Separator />

		<!-- Session list -->
		<ScrollArea maxHeight="md">
			{#if loading}
				<HStack justify="center">
					<LoadingSpinner />
				</HStack>
			{:else if error}
				<Box paddingX={3} paddingY={4}>
					<ErrorDisplay message={error} onRetry={onRetry} />
				</Box>
			{:else if filteredSessions.length === 0}
				<EmptyState
					icon="message-square"
					title={searchQuery.trim().length > 0 ? "No matching sessions" : "No sessions yet"}
				/>
			{:else}
				<Box padding={1}>
					{#each filteredSessions as session (session.id)}
						{@const isActive = session.id === activeSessionId}
						<HStack
							align="start"
							gap={2}
							full
							onclick={() => handleSelect(session.id)}
							onkeydown={(e: KeyboardEvent) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleSelect(session.id); } }}
							role="option"
							aria-selected={isActive}
							tabindex={0}
						>
							<div class="min-w-0 flex-1">
								<HStack gap={1} wrap>
									<Text variant="label" truncate>{session.title ?? "Untitled"}</Text>
									<Badge variant={statusVariant(session.status)} size="xs">
										{statusLabel(session.status)}
									</Badge>
								</HStack>
								<HStack gap={2}>
									<Caption>{session.message_count} messages</Caption>
									<Caption tone="muted">|</Caption>
									<Caption>{formatRelativeTime(session.updated_at)}</Caption>
								</HStack>
								{#if session.preview}
									<Caption truncate>
										{session.preview}
									</Caption>
								{/if}
							</div>
							<!-- Delete button -->
							<Button
								variant="ghost"
								size="icon-sm"
								onclick={(e) => handleDeleteClick(e, session.id, session.title ?? "Untitled")}
								aria-label="Delete session"
							>
								<Icon name="trash-2" size="sm" />
							</Button>
						</HStack>
					{/each}
				</Box>
			{/if}
		</ScrollArea>
	</PopoverContent>
</Popover>

<ConfirmDeleteDialog
	bind:open={deleteDialogOpen}
	title="Delete session?"
	description="This will permanently delete &quot;{deleteTargetTitle}&quot; and all its messages."
	onConfirm={handleDeleteConfirm}
/>
