<script lang="ts">
	import {
		Icon,
		TooltipRoot as Tooltip,
		TooltipContent,
		TooltipTrigger,
		Caption,
		Code,
		Badge,
	} from "@orqastudio/svelte-components/pure";

	let {
		action,
		ruleName,
	}: {
		action: "Block" | "Warn";
		ruleName: string;
	} = $props();
</script>

<Tooltip>
	<TooltipTrigger>
		{#if action === "Block"}
			<Badge variant="destructive" aria-label="Blocked by rule: {ruleName}">
				<Icon name="shield" size="xs" />
				Block
			</Badge>
		{:else}
			<Badge variant="warning" aria-label="Warning from rule: {ruleName}">
				<Icon name="alert-triangle" size="xs" />
				Warn
			</Badge>
		{/if}
	</TooltipTrigger>
	<TooltipContent>
		<Caption>Rule: <Code>{ruleName}</Code></Caption>
	</TooltipContent>
</Tooltip>
