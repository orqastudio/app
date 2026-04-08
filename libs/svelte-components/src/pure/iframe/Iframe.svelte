<!-- Iframe — a typed library wrapper around the native <iframe> element.
     Raw HTML is only permitted inside library components; app/devtools code
     must use this component rather than writing <iframe> directly.

     The fill prop sets height and width to 100% with flex: 1 so the iframe
     expands to fill its parent container — this is the standard pattern for
     embedding full-height web views (e.g. Storybook, documentation pages).

     sandbox controls the permission set passed to the sandboxed iframe. -->
<script lang="ts">
	let {
		src,
		title,
		fill = false,
		sandbox,
	}: {
		/** URL to load in the frame. */
		src: string;
		/** Accessible title for the frame (required for screen readers). */
		title: string;
		/** When true, sets height/width to 100% and flex: 1 to fill the parent. */
		fill?: boolean;
		/** Sandbox permission string passed verbatim to the iframe element. */
		sandbox?: string;
	} = $props();
</script>

<iframe
	{src}
	{title}
	{sandbox}
	style:height={fill ? "100%" : undefined}
	style:width={fill ? "100%" : undefined}
	style:flex={fill ? "1" : undefined}
	style:border="none"
></iframe>
