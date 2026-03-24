<script lang="ts">
	import { Icon, DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@orqastudio/svelte-components/pure";
	import { TabsRoot as Tabs, TabsContent, TabsList, TabsTrigger } from "@orqastudio/svelte-components/pure";
	import { ScrollArea } from "@orqastudio/svelte-components/pure";
	import {
		CollapsibleRoot as Collapsible,
		CollapsibleContent,
		CollapsibleTrigger,
	} from "@orqastudio/svelte-components/pure";
	import { logger } from "@orqastudio/sdk";
	import type { ContextEntry as ContextEntryType } from "@orqastudio/sdk";

	const log = logger("conversation");

	let {
		entry,
		open = $bindable(false),
	}: { entry: ContextEntryType; open: boolean } = $props();

	let customPromptOpen = $state(true);
	let governancePromptOpen = $state(false);

	interface ParsedMessage {
		role: string;
		content: string;
	}

	const parsedMessages = $derived.by((): ParsedMessage[] => {
		if (entry.type !== "context_injected") return [];
		try {
			const parsed = JSON.parse(entry.messages);
			if (!Array.isArray(parsed)) return [];
			return parsed.map((m: unknown) => {
				if (typeof m === "object" && m !== null) {
					const obj = m as Record<string, unknown>;
					return {
						role: typeof obj.role === "string" ? obj.role : "unknown",
						content:
							typeof obj.content === "string" ? obj.content : JSON.stringify(obj.content),
					};
				}
				return { role: "unknown", content: String(m) };
			});
		} catch (err) {
			log.warn("Failed to parse injected context messages", { err });
			return [];
		}
	});

	const dialogTitle = $derived(
		entry.type === "system_prompt_sent" ? "System Prompt Details" : "Injected Context Details"
	);

	const dialogDescription = $derived(
		entry.type === "system_prompt_sent"
			? `${entry.totalChars.toLocaleString()} characters sent to the model`
			: `${entry.messageCount} messages, ${entry.totalChars.toLocaleString()} characters injected`
	);

	const rawText = $derived.by(() => {
		if (entry.type === "system_prompt_sent") {
			const parts: string[] = [];
			if (entry.customPrompt) {
				parts.push("=== CUSTOM PROMPT ===\n" + entry.customPrompt);
			}
			parts.push("=== GOVERNANCE PROMPT ===\n" + entry.governancePrompt);
			return parts.join("\n\n");
		}
		return entry.messages;
	});

	function roleLabel(role: string): string {
		if (role === "user") return "User";
		if (role === "assistant") return "Assistant";
		return role.charAt(0).toUpperCase() + role.slice(1);
	}
</script>

<DialogRoot bind:open>
	<DialogContent class="flex max-h-[80vh] flex-col gap-0 p-0 sm:max-w-2xl">
		<DialogHeader class="border-b border-border px-6 py-4">
			<DialogTitle>{dialogTitle}</DialogTitle>
			<DialogDescription>{dialogDescription}</DialogDescription>
		</DialogHeader>

		<Tabs value="structured" class="flex min-h-0 flex-1 flex-col">
			<TabsList class="mx-6 mt-4 w-fit shrink-0">
				<TabsTrigger value="structured">Structured</TabsTrigger>
				<TabsTrigger value="raw">Raw</TabsTrigger>
			</TabsList>

			<TabsContent value="structured" class="min-h-0 flex-1 overflow-hidden px-6 pb-6 pt-4">
				<ScrollArea class="h-full">
					{#if entry.type === "system_prompt_sent"}
						<div class="space-y-3">
							{#if entry.customPrompt}
								<Collapsible bind:open={customPromptOpen}>
									<CollapsibleTrigger
										class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
									>
										<Icon name="chevron-right" size="sm" />
										<span class="flex-1 text-xs font-medium text-foreground">Custom Prompt</span>
										<span class="text-xs text-muted-foreground">
											{entry.customPrompt.length.toLocaleString()} chars
										</span>
									</CollapsibleTrigger>
									<CollapsibleContent>
										<div class="ml-3 mt-1 border-l-2 border-border pl-4">
											<pre
												class="whitespace-pre-wrap break-words rounded-md bg-muted/20 p-3 font-mono text-xs text-foreground">{entry.customPrompt}</pre>
										</div>
									</CollapsibleContent>
								</Collapsible>
							{:else}
								<div
									class="rounded-lg border border-border bg-muted/20 px-3 py-2 text-xs text-muted-foreground"
								>
									No custom prompt — using governance prompt only.
								</div>
							{/if}

							<Collapsible bind:open={governancePromptOpen}>
								<CollapsibleTrigger
									class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
								>
									<Icon name="chevron-right" size="sm" />
									<span class="flex-1 text-xs font-medium text-foreground"
										>Governance Prompt</span
									>
									<span class="text-xs text-muted-foreground">
										{entry.governancePrompt.length.toLocaleString()} chars
									</span>
								</CollapsibleTrigger>
								<CollapsibleContent>
									<div class="ml-3 mt-1 border-l-2 border-border pl-4">
										<pre
											class="whitespace-pre-wrap break-words rounded-md bg-muted/20 p-3 font-mono text-xs text-foreground">{entry.governancePrompt}</pre>
									</div>
								</CollapsibleContent>
							</Collapsible>
						</div>
					{:else if entry.type === "context_injected"}
						<div class="space-y-2">
							{#if parsedMessages.length === 0}
								<div
									class="rounded-lg border border-border bg-muted/20 px-3 py-2 text-xs text-muted-foreground"
								>
									Unable to parse injected messages.
								</div>
							{:else}
								{#each parsedMessages as msg, i (i)}
									<div class="rounded-lg border border-border bg-muted/20 p-3 text-xs">
										<div
											class="mb-1.5 text-[11px] font-semibold uppercase tracking-wide text-muted-foreground"
										>
											{roleLabel(msg.role)}
										</div>
										<div class="line-clamp-6 whitespace-pre-wrap break-words text-foreground">
											{msg.content}
										</div>
									</div>
								{/each}
							{/if}
						</div>
					{/if}
				</ScrollArea>
			</TabsContent>

			<TabsContent value="raw" class="min-h-0 flex-1 overflow-hidden px-6 pb-6 pt-4">
				<ScrollArea class="h-full">
					<pre
						class="whitespace-pre-wrap break-words rounded-md bg-muted/20 p-4 font-mono text-xs text-foreground">{rawText}</pre>
				</ScrollArea>
			</TabsContent>
		</Tabs>
	</DialogContent>
</DialogRoot>
