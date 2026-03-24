<script lang="ts">
	import type { SessionSummary, SessionStatus } from "@orqastudio/types";
	import { Icon,
		PopoverRoot as Popover,
		PopoverContent,
		PopoverTrigger,
	} from "@orqastudio/svelte-components/pure";
	import { Button } from "@orqastudio/svelte-components/pure";
	import { Badge } from "@orqastudio/svelte-components/pure";
	import { Separator } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import { SearchInput } from "@orqastudio/svelte-components/pure";
	import { ConfirmDialog as ConfirmDeleteDialog } from "@orqastudio/svelte-components/pure";
	import { EmptyState } from "@orqastudio/svelte-components/pure";
	import { ErrorDisplay } from "@orqastudio/svelte-components/pure";
	import { LoadingSpinner } from "@orqastudio/svelte-components/pure";

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
				return "secondary";
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
				return status;
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
	<PopoverContent align="start" class="w-80 p-0">
		<!-- Header with New Session -->
		<div class="flex items-center justify-between p-3 pb-2">
			<h3 class="text-sm font-semibold">Sessions</h3>
			<Button variant="ghost" size="sm" onclick={handleNewSession} class="h-7 gap-1 px-2 text-xs">
				<Icon name="plus" size="sm" />
				New Session
			</Button>
		</div>

		<!-- Search -->
		<div class="px-3 pb-2">
			<SearchInput bind:value={searchQuery} placeholder="Search sessions..." size="sm" />
		</div>

		<Separator />

		<!-- Session list -->
		<ScrollArea class="max-h-64">
			{#if loading}
				<div class="flex items-center justify-center py-6">
					<LoadingSpinner />
				</div>
			{:else if error}
				<div class="px-3 py-4">
					<ErrorDisplay message={error} onRetry={onRetry} />
				</div>
			{:else if filteredSessions.length === 0}
				<EmptyState
					icon="message-square"
					title={searchQuery.trim().length > 0 ? "No matching sessions" : "No sessions yet"}
				/>
			{:else}
				<div class="p-1">
					{#each filteredSessions as session (session.id)}
						{@const isActive = session.id === activeSessionId}
						<div
							class="group flex w-full cursor-pointer items-start gap-2 rounded-md px-2 py-2 text-left transition-colors hover:bg-accent {isActive ? 'bg-accent/50' : ''}"
							onclick={() => handleSelect(session.id)}
							onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleSelect(session.id); } }}
							role="option"
							aria-selected={isActive}
							tabindex="0"
						>
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-1.5">
									<span class="truncate text-sm font-medium {isActive ? 'text-foreground' : 'text-foreground/80'}">
										{session.title ?? "Untitled"}
									</span>
									<Badge variant={statusVariant(session.status)} class="shrink-0 text-[10px] px-1.5 py-0">
										{statusLabel(session.status)}
									</Badge>
								</div>
								<div class="mt-0.5 flex items-center gap-2 text-[11px] text-muted-foreground">
									<span>{session.message_count} messages</span>
									<span class="text-muted-foreground/50">|</span>
									<span>{formatRelativeTime(session.updated_at)}</span>
								</div>
								{#if session.preview}
									<p class="mt-0.5 truncate text-[11px] text-muted-foreground/70">
										{session.preview}
									</p>
								{/if}
							</div>
							<!-- Delete button -->
							<button
								class="mt-0.5 shrink-0 rounded p-1 text-muted-foreground opacity-0 transition-opacity hover:bg-destructive/10 hover:text-destructive group-hover:opacity-100"
								onclick={(e) => handleDeleteClick(e, session.id, session.title ?? "Untitled")}
								aria-label="Delete session"
								title="Delete session"
							>
								<Icon name="trash-2" size="sm" />
							</button>
						</div>
					{/each}
				</div>
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
