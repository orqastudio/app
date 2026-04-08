<script lang="ts" module>
	import { type WithElementRef } from "../../utils/cn.js";
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements";
	import { type VariantProps, tv } from "tailwind-variants";

	export const buttonVariants = tv({
		base: "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-all outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			variant: {
				default: "bg-primary text-primary-foreground hover:bg-primary/90 shadow-xs",
				destructive:
					"bg-destructive hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 text-white shadow-xs",
				outline:
					"bg-background hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border shadow-xs",
				secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-xs",
				ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
				// Ghost button with destructive text — used for danger actions in context menus.
				"ghost-destructive":
					"text-destructive hover:bg-destructive/10 hover:text-destructive dark:hover:bg-destructive/20",
				link: "text-primary underline-offset-4 hover:underline",
				card: "w-full text-left h-auto p-3 rounded-lg border border-border bg-card hover:bg-accent/50 hover:border-border/80 shadow-none",
				// Full-width, left-justified row button for virtualised log table rows.
				// Zero gap and zero radius keep columns tightly aligned.
				row: "w-full justify-start gap-0 rounded-none px-2 text-left hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
			},
			size: {
				default: "h-9 px-4 py-2 has-[>svg]:px-3",
				sm: "h-8 gap-2 rounded-md px-3 has-[>svg]:px-3",
				lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
				icon: "size-9",
				"icon-sm": "size-8",
				"icon-lg": "size-10",
				// Compact size for dense devtools toolbars: 20px height, 10px font.
				xs: "h-5 px-2 text-[10px] gap-1",
				// Log row button: 24px (h-6) to match ROW_HEIGHT in the log virtualiser. Leading-6
				// ensures text is vertically centred at this fixed height without an inline style.
				"log-row": "h-6 leading-6 text-[10px] gap-0",
				// Status bar size: 24px height, 12px font, tight padding for the 2rem footer strip.
				status: "h-6 px-2 text-xs gap-2",
				// Column item size: auto height, column flex layout, for multi-line session/option rows.
				"col-item":
					"h-auto flex-col items-start justify-start gap-1 px-2 py-2 rounded-none text-[11px] min-w-0",
				// List item size: auto height, left-justified, 0.5rem padding — for sidebar list buttons.
				"list-item": "h-auto justify-start text-left px-2 py-2 w-full",
			},
			full: {
				true: "w-full",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	});

	export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
	export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

	export type ButtonProps = WithElementRef<Omit<HTMLButtonAttributes, "class">> &
		WithElementRef<Omit<HTMLAnchorAttributes, "class">> & {
			variant?: ButtonVariant;
			size?: ButtonSize;
			/** When true, expands to full width. */
			full?: boolean;
			/** When true, renders at reduced opacity (0.6) — used for de-emphasised list items. */
			faded?: boolean;
			/** Icon name rendered before children. Resolves via the design system icon map. */
			startIcon?: string;
			/** Icon name rendered after children. Resolves via the design system icon map. */
			endIcon?: string;
		};
</script>

<script lang="ts">
	import { Icon } from "../icon/index.js";

	let {
		variant = "default",
		size = "default",
		full = false,
		faded = false,
		startIcon,
		endIcon,
		ref = $bindable(null),
		href = undefined,
		type = "button",
		disabled,
		children,
		...restProps
	}: ButtonProps = $props();

	// Icon size derived from button size — smaller buttons get smaller icons.
	const ICON_SIZE_MAP: Record<string, "xs" | "sm" | "md" | "lg"> = {
		xs: "xs",
		status: "xs",
		"log-row": "xs",
		sm: "sm",
		"icon-sm": "sm",
		default: "md",
		lg: "md",
		icon: "md",
		"icon-lg": "lg",
	};
	const iconSizeResolved = $derived(ICON_SIZE_MAP[size ?? "default"] ?? "md");
</script>

{#if href}
	<a
		bind:this={ref}
		data-slot="button"
		{...restProps}
		class="{buttonVariants({ variant, size, full: full || undefined })}{faded ? ' opacity-60' : ''}"
		style={undefined}
		href={disabled ? undefined : href}
		aria-disabled={disabled}
		role={disabled ? "link" : undefined}
		tabindex={disabled ? -1 : undefined}
	>
		{#if startIcon}<Icon name={startIcon} size={iconSizeResolved} />{/if}
		{@render children?.()}
		{#if endIcon}<Icon name={endIcon} size={iconSizeResolved} />{/if}
	</a>
{:else}
	<button
		bind:this={ref}
		data-slot="button"
		{...restProps}
		class="{buttonVariants({ variant, size, full: full || undefined })}{faded ? ' opacity-60' : ''}"
		style={undefined}
		{type}
		{disabled}
	>
		{#if startIcon}<Icon name={startIcon} size={iconSizeResolved} />{/if}
		{@render children?.()}
		{#if endIcon}<Icon name={endIcon} size={iconSizeResolved} />{/if}
	</button>
{/if}
