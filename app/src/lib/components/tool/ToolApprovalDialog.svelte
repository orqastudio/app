<script lang="ts">
	import { Icon, HStack, Button, Text, Code } from "@orqastudio/svelte-components/pure";
	import {
		CardRoot as Card,
		CardContent,
		CardFooter,
		CardHeader,
		CardTitle,
	} from "@orqastudio/svelte-components/pure";
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
			log.error("Failed to parse tool approval input as JSON", {
				toolName: approval.toolName,
				err,
			});
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
		<Text variant="caption" block>
			Claude wants to run <Code>{stripToolName(approval.toolName)}</Code> with the following parameters.
			Allow this action?
		</Text>
		<CodeBlock text={formattedInput()} lang="json" />
	</CardContent>
	<CardFooter compact>
		<HStack gap={2}>
			<Button variant="default" size="sm" onclick={onApprove}>
				<Icon name="check" size="sm" />
				Approve
			</Button>
			<Button variant="destructive" size="sm" onclick={onDeny}>
				<Icon name="x" size="sm" />
				Deny
			</Button>
		</HStack>
	</CardFooter>
</Card>
