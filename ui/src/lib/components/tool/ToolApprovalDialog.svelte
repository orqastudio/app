<script lang="ts">
	import { Icon, Button } from "@orqastudio/svelte-components/pure";
	import { CardRoot as Card, CardContent, CardFooter, CardHeader, CardTitle } from "@orqastudio/svelte-components/pure";
	import CodeBlock from "$lib/components/content/CodeBlock.svelte";
	import { logger } from "@orqastudio/sdk";
	import type { PendingApproval } from "@orqastudio/sdk";

	const log = logger("tool-approval");
	import { getToolDisplay, stripToolName } from "$lib/utils/tool-display";

	let {
		approval,
		onApprove,
		onDeny,
	}: {
		approval: PendingApproval;
		onApprove: () => void;
		onDeny: () => void;
	} = $props();

	const toolLabel = $derived(getToolDisplay(approval.toolName).label);

	/** Pretty-print JSON if possible, fall back to raw string. */
	const formattedInput = $derived(() => {
		try {
			return JSON.stringify(JSON.parse(approval.input), null, 2);
		} catch (err) {
			log.error("Failed to parse tool approval input as JSON", { toolName: approval.toolName, err });
			return approval.input;
		}
	});
</script>

<Card class="border-warning/40 bg-warning/5">
	<CardHeader class="pb-2">
		<CardTitle class="flex items-center gap-2 text-sm font-semibold text-warning">
			<Icon name="shield-alert" size="md" />
			Approval Required — {toolLabel}
		</CardTitle>
	</CardHeader>
	<CardContent class="pb-2">
		<p class="mb-2 text-xs text-muted-foreground">
			Claude wants to run <span class="font-mono text-foreground">{stripToolName(approval.toolName)}</span> with the
			following parameters. Allow this action?
		</p>
		<CodeBlock text={formattedInput()} lang="json" />
	</CardContent>
	<CardFooter class="flex gap-2 pt-2">
		<Button variant="default" size="sm" onclick={onApprove} class="gap-1.5">
			<Icon name="check" size="sm" />
			Approve
		</Button>
		<Button variant="outline" size="sm" onclick={onDeny} class="gap-1.5">
			<Icon name="x" size="sm" />
			Deny
		</Button>
	</CardFooter>
</Card>
