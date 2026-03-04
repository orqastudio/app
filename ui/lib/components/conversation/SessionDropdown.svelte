<script lang="ts">
	import type { SessionSummary, SessionStatus } from "$lib/types";
	import PlusIcon from "@lucide/svelte/icons/plus";
	import Trash2Icon from "@lucide/svelte/icons/trash-2";
	import MessageSquareIcon from "@lucide/svelte/icons/message-square";
	import {
		Popover,
		PopoverContent,
		PopoverTrigger,
	} from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import { Badge } from "$lib/components/ui/badge";
	import { Separator } from "$lib/components/ui/separator";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import SearchInput from "$lib/components/shared/SearchInput.svelte";

	let {
		sessions,
		activeSessionId,
		onSelect,
		onNewSession,
		onDelete,
		children,
	}: {
		sessions: SessionSummary[];
		activeSessionId: number | null;
		onSelect: (sessionId: number) => void;
		onNewSession: () => void;
		onDelete: (sessionId: number) => void;
		children: import("svelte").Snippet;
	} = $props();

	let open = $state(false);
	let searchQuery = $state("");
	let confirmDeleteId = $state<number | null>(null);

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
		confirmDeleteId = null;
	}

	function handleNewSession() {
		onNewSession();
		open = false;
		searchQuery = "";
		confirmDeleteId = null;
	}

	function handleDeleteClick(event: MouseEvent, sessionId: number) {
		event.stopPropagation();
		if (confirmDeleteId === sessionId) {
			onDelete(sessionId);
			confirmDeleteId = null;
		} else {
			confirmDeleteId = sessionId;
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
				<PlusIcon class="h-3.5 w-3.5" />
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
			{#if filteredSessions.length === 0}
				<div class="flex flex-col items-center py-6 text-center">
					<MessageSquareIcon class="mb-2 h-8 w-8 text-muted-foreground" />
					<p class="text-xs text-muted-foreground">
						{searchQuery.trim().length > 0
							? "No matching sessions"
							: "No sessions yet"}
					</p>
				</div>
			{:else}
				<div class="p-1">
					{#each filteredSessions as session (session.id)}
						{@const isActive = session.id === activeSessionId}
						<!-- svelte-ignore a11y_no_static_element_interactions -->
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
								class="mt-0.5 shrink-0 rounded p-1 text-muted-foreground opacity-0 transition-opacity hover:bg-destructive/10 hover:text-destructive group-hover:opacity-100 {confirmDeleteId === session.id ? 'opacity-100 text-destructive' : ''}"
								onclick={(e) => handleDeleteClick(e, session.id)}
								aria-label={confirmDeleteId === session.id ? "Confirm delete" : "Delete session"}
								title={confirmDeleteId === session.id ? "Click again to confirm" : "Delete session"}
							>
								<Trash2Icon class="h-3.5 w-3.5" />
							</button>
						</div>
					{/each}
				</div>
			{/if}
		</ScrollArea>
	</PopoverContent>
</Popover>
