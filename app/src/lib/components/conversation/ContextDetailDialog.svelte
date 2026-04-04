<script lang="ts">
	import { Icon, DialogRoot, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@orqastudio/svelte-components/pure";
	import { TabsRoot as Tabs, TabsContent, TabsList, TabsTrigger } from "@orqastudio/svelte-components/pure";
	import { ScrollArea, Stack, HStack, Box, Code, Caption, Text } from "@orqastudio/svelte-components/pure";
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
	<DialogContent>
		<DialogHeader>
			<DialogTitle>{dialogTitle}</DialogTitle>
			<DialogDescription>{dialogDescription}</DialogDescription>
		</DialogHeader>

		<Tabs value="structured">
			<TabsList>
				<TabsTrigger value="structured">Structured</TabsTrigger>
				<TabsTrigger value="raw">Raw</TabsTrigger>
			</TabsList>

			<TabsContent value="structured">
				<ScrollArea full>
					{#if entry.type === "system_prompt_sent"}
						<Stack gap={3}>
							{#if entry.customPrompt}
								<Collapsible bind:open={customPromptOpen}>
									<CollapsibleTrigger
										class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
									>
										<Icon name="chevron-right" size="sm" />
										<!-- flex-1 is structural (fills trigger row) -->
										<span class="flex-1"><Text variant="body-strong">Custom Prompt</Text></span>
										<Caption>{entry.customPrompt.length.toLocaleString()} chars</Caption>
									</CollapsibleTrigger>
									<CollapsibleContent>
										<!-- border-l-2 and ml-3 are structural indentation; no ORQA primitive supports border-left -->
										<div class="ml-3 mt-1 border-l-2 border-border pl-4">
											<Code block>{entry.customPrompt}</Code>
										</div>
									</CollapsibleContent>
								</Collapsible>
							{:else}
								<Box padding={2} border rounded="lg">
									<Caption>No custom prompt — using governance prompt only.</Caption>
								</Box>
							{/if}

							<Collapsible bind:open={governancePromptOpen}>
								<CollapsibleTrigger
									class="flex w-full items-center gap-2 rounded-lg border border-border bg-muted/30 px-3 py-2 text-left text-sm transition-colors hover:bg-muted/50"
								>
									<Icon name="chevron-right" size="sm" />
									<!-- flex-1 is structural (fills trigger row) -->
									<span class="flex-1"><Text variant="body-strong">Governance Prompt</Text></span>
									<Caption>{entry.governancePrompt.length.toLocaleString()} chars</Caption>
								</CollapsibleTrigger>
								<CollapsibleContent>
									<!-- border-l-2 and ml-3 are structural indentation; no ORQA primitive supports border-left -->
									<div class="ml-3 mt-1 border-l-2 border-border pl-4">
										<Code block>{entry.governancePrompt}</Code>
									</div>
								</CollapsibleContent>
							</Collapsible>
						</Stack>
					{:else if entry.type === "context_injected"}
						<Stack gap={2}>
							{#if parsedMessages.length === 0}
								<Box padding={2} border rounded="lg">
									<Caption>Unable to parse injected messages.</Caption>
								</Box>
							{:else}
								{#each parsedMessages as msg, i (i)}
									<Box padding={3} border rounded="lg">
										<Caption variant="caption-strong">{roleLabel(msg.role)}</Caption>
										<Text variant="body">{msg.content}</Text>
									</Box>
								{/each}
							{/if}
						</Stack>
					{/if}
				</ScrollArea>
			</TabsContent>

			<TabsContent value="raw">
				<ScrollArea full>
					<Code block>{rawText}</Code>
				</ScrollArea>
			</TabsContent>
		</Tabs>
	</DialogContent>
</DialogRoot>
