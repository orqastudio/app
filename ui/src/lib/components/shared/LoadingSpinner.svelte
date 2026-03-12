<script lang="ts">
	import logoPulse from "$lib/assets/logo-pulse.svg";

	let {
		size = "md",
		variant = "branded",
	}: {
		size?: "sm" | "md" | "lg";
		variant?: "branded" | "minimal";
	} = $props();

	const sizeClasses: Record<string, string> = {
		sm: "h-4 w-4",
		md: "h-8 w-8",
		lg: "h-12 w-12",
	};

	const useBranded = $derived(variant === "branded" && size !== "sm");
</script>

<div class="flex items-center justify-center">
	{#if useBranded}
		<img
			src={logoPulse}
			alt="Loading…"
			class={sizeClasses[size]}
		/>
	{:else}
		<div
			class="animate-spin rounded-full border-2 border-muted border-t-primary {sizeClasses[size]}"
		></div>
	{/if}
</div>
