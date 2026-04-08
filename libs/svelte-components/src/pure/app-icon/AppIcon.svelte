<!-- AppIcon — displays an application logo image with a fixed size and pointer-events disabled.
     Used in toolbar and status bar areas where a brand mark or project icon is shown.
     The `rounded` prop adds border-radius for project icon thumbnails.
     The `objectContain` prop uses object-contain so arbitrary project icons scale correctly.
     The `border` prop adds a subtle border for use as a settings thumbnail. -->
<script lang="ts">
	import { cn } from "../../utils/cn.js";

	let {
		src,
		alt = "",
		size = "md",
		rounded = false,
		objectContain = false,
		border = false,
	}: {
		/** Image source URL or data URL. */
		src: string;
		/** Accessible alt text. Empty string is acceptable for decorative icons. */
		alt?: string;
		/** Visual size: "xs" = 14px, "sm" = 20px, "toolbar" = 28px, "md" = 40px. */
		size?: "xs" | "sm" | "toolbar" | "md";
		/** Apply rounded-md border-radius (for project icon thumbnails). */
		rounded?: boolean;
		/** Apply object-contain so the image scales without cropping. */
		objectContain?: boolean;
		/** Apply a 1px border using the design token border color. */
		border?: boolean;
	} = $props();

	const SIZE_MAP: Record<string, string> = {
		xs: "h-3.5 w-3.5",
		sm: "h-5 w-5",
		toolbar: "h-7 w-7",
		md: "h-10 w-10",
	};
	const sizeClass = $derived(SIZE_MAP[size] ?? SIZE_MAP.md);
</script>

<img
	{src}
	{alt}
	class={cn(
		"pointer-events-none",
		sizeClass,
		rounded && "rounded-md",
		objectContain && "object-contain",
		border && "border-border border",
	)}
/>
