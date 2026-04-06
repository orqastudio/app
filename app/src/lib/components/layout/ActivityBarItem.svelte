<script lang="ts">
	import {
		Icon,
		Text,
		TooltipRoot,
		TooltipTrigger,
		TooltipContent,
		VisuallyHidden,
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
			<button
				{...props}
				class="mb-1 flex h-10 w-10 items-center justify-center rounded-md transition-colors {active
					? 'bg-accent text-accent-foreground'
					: 'text-muted-foreground hover:bg-accent/50 hover:text-foreground'}"
				{onclick}
			>
				<Icon name={icon} size="lg" />
				<VisuallyHidden>{label}</VisuallyHidden>
			</button>
		{/snippet}
	</TooltipTrigger>
	<TooltipContent side="right">
		<Text variant="body">{label}</Text>
	</TooltipContent>
</TooltipRoot>
