<script lang="ts">
	import { Icon, HStack } from "@orqastudio/svelte-components/pure";
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

<Card variant="warning">
	<CardHeader compact>
		<CardTitle>
			<HStack gap={2}>
				<Icon name="shield-alert" size="md" />
				Approval Required — {toolLabel}
			</HStack>
		</CardTitle>
	</CardHeader>
	<CardContent compact>
		<p class="mb-2 text-xs text-muted-foreground">
			Claude wants to run <span class="font-mono text-foreground">{stripToolName(approval.toolName)}</span> with the
			following parameters. Allow this action?
		</p>
		<CodeBlock text={formattedInput()} lang="json" />
	</CardContent>
	<CardFooter compact>
		<div class="flex items-center gap-2">
			<button
				class="flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-sm text-primary-foreground hover:bg-primary/90"
				onclick={onApprove}
			>
				<Icon name="check" size="sm" />
				Approve
			</button>
			<button
				class="flex items-center gap-1.5 rounded-md border border-border px-3 py-1.5 text-sm hover:bg-accent"
				onclick={onDeny}
			>
				<Icon name="x" size="sm" />
				Deny
			</button>
		</div>
	</CardFooter>
</Card>
