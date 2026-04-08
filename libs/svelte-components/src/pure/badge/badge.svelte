<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const badgeVariants = tv({
		base: "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex w-fit shrink-0 items-center justify-center gap-2 overflow-hidden rounded border px-2 py-1 font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:ring-[3px] [&>svg]:pointer-events-none [&>svg]:size-3",
		variants: {
			variant: {
				default: "bg-primary text-primary-foreground [a&]:hover:bg-primary/90 border-transparent",
				secondary:
					"bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90 border-border",
				destructive:
					"bg-destructive [a&]:hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/70 border-transparent text-white",
				outline: "text-foreground [a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
				warning: "bg-warning/15 text-warning border-warning/30 [a&]:hover:bg-warning/25",
				success: "bg-success/15 text-success border-success/30 [a&]:hover:bg-success/25",
			},
			size: {
				default: "text-xs",
				sm: "text-[11px] leading-none",
				xs: "text-[10px] leading-none px-1",
				// Log table column badge: fixed 42px width, centered, uppercase monospace.
				log: "w-[42px] justify-center font-mono text-[10px] uppercase leading-none px-1",
			},
			capitalize: {
				true: "capitalize",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	});

	export type BadgeVariant = VariantProps<typeof badgeVariants>["variant"];
	export type BadgeSize = VariantProps<typeof badgeVariants>["size"];
	export type BadgeCapitalize = VariantProps<typeof badgeVariants>["capitalize"];
</script>

<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		href,
		variant = "default",
		size = "default",
		capitalize = false,
		children,
		...restProps
	}: Omit<HTMLAttributes<HTMLElement>, "class" | "style"> & {
		ref?: HTMLElement | null;
		href?: string;
		variant?: BadgeVariant;
		size?: BadgeSize;
		capitalize?: boolean;
		children?: Snippet;
	} = $props();
</script>

<svelte:element
	this={href ? "a" : "span"}
	bind:this={ref}
	data-slot="badge"
	{href}
	class={badgeVariants({ variant, size, capitalize: capitalize || undefined })}
	{...restProps}
>
	{@render children?.()}
</svelte:element>
