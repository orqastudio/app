<script lang="ts">
	import type { Snippet } from "svelte";

	let {
		backgroundImage,
		logoSrc,
		logoAlt = "",
		title,
		subtitle,
		children,
	}: {
		backgroundImage?: string;
		logoSrc?: string;
		logoAlt?: string;
		title: string;
		subtitle?: string;
		children?: Snippet;
	} = $props();
</script>

<div
	class="relative flex h-full w-full items-center justify-center overflow-hidden"
	style={backgroundImage
		? `background-image: url(${backgroundImage}); background-size: cover; background-position: center;`
		: undefined}
>
	<div class="bg-background/70 absolute inset-0"></div>

	<div class="relative z-10 flex flex-col items-center gap-8">
		{#if logoSrc}
			<img src={logoSrc} alt={logoAlt} class="h-16 w-16 opacity-60" />
		{/if}
		<div class="text-center">
			<h2 class="text-foreground text-xl font-semibold">{title}</h2>
			{#if subtitle}
				<p class="text-muted-foreground mt-2 text-sm">{subtitle}</p>
			{/if}
		</div>
		{#if children}
			{@render children()}
		{/if}
	</div>
</div>
