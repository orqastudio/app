<script lang="ts">
	import ShieldAlertIcon from "@lucide/svelte/icons/shield-alert";
	import CheckIcon from "@lucide/svelte/icons/check";
	import XIcon from "@lucide/svelte/icons/x";
	import { Button } from "$lib/components/ui/button";
	import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "$lib/components/ui/card";
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import type { PendingApproval } from "$lib/stores/conversation.svelte";

	const TOOL_DISPLAY: Record<string, string> = {
		write_file: "Write File",
		edit_file: "Edit File",
		bash: "Run Command",
	};

	let {
		approval,
		onApprove,
		onDeny,
	}: {
		approval: PendingApproval;
		onApprove: () => void;
		onDeny: () => void;
	} = $props();

	const toolLabel = $derived(TOOL_DISPLAY[approval.toolName] ?? approval.toolName);

	/** Pretty-print JSON if possible, fall back to raw string. */
	const formattedInput = $derived(() => {
		try {
			return JSON.stringify(JSON.parse(approval.input), null, 2);
		} catch {
			return approval.input;
		}
	});
</script>

<Card class="border-amber-500/40 bg-amber-950/10">
	<CardHeader class="pb-2">
		<CardTitle class="flex items-center gap-2 text-sm font-semibold text-amber-400">
			<ShieldAlertIcon class="h-4 w-4 shrink-0" />
			Approval Required — {toolLabel}
		</CardTitle>
	</CardHeader>
	<CardContent class="pb-2">
		<p class="mb-2 text-xs text-muted-foreground">
			Claude wants to run <span class="font-mono text-foreground">{approval.toolName}</span> with the
			following parameters. Allow this action?
		</p>
		<CodeBlock code={formattedInput()} language="json" />
	</CardContent>
	<CardFooter class="flex gap-2 pt-2">
		<Button variant="default" size="sm" onclick={onApprove} class="gap-1.5">
			<CheckIcon class="h-3.5 w-3.5" />
			Approve
		</Button>
		<Button variant="outline" size="sm" onclick={onDeny} class="gap-1.5">
			<XIcon class="h-3.5 w-3.5" />
			Deny
		</Button>
	</CardFooter>
</Card>
