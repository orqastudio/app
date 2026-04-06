<script lang="ts">
	import {
		ActivityBarButton,
		TooltipRoot,
		TooltipTrigger,
		TooltipContent,
		Text,
	} from "@orqastudio/svelte-components/pure";

	let {
		icon,
		label,
		active = false,
		onclick,
	}: {
		icon: string;
		label: string;
		active?: boolean;
		onclick: () => void;
	} = $props();
</script>

<TooltipRoot>
	<TooltipTrigger>
		{#snippet child({ props })}
			<ActivityBarButton {...props} {icon} {label} {active} {onclick} />
		{/snippet}
	</TooltipTrigger>
	<TooltipContent side="right">
		<Text variant="body">{label}</Text>
	</TooltipContent>
</TooltipRoot>
