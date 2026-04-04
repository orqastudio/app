<!-- Small circular count indicator for lists and columns. Displays a numeric count in a fixed-size pill. -->
<script lang="ts">
	type CountBadgeVariant = "default" | "success" | "muted" | "warning";

	let { count, variant = "default" }: { count: number; variant?: CountBadgeVariant } = $props();

	// Maps semantic variant names to Tailwind background and text color pairs.
	const variantMap: Record<CountBadgeVariant, string> = {
		default: "bg-primary/10 text-primary",
		success: "bg-success/15 text-success",
		muted: "bg-muted text-muted-foreground",
		warning: "bg-warning/15 text-warning",
	};
</script>

<span class="inline-flex size-5 items-center justify-center rounded-full text-xs font-semibold {variantMap[variant]}" aria-hidden="true">{count}</span>
