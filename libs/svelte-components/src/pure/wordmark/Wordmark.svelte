<!-- Wordmark — split-weight brand text: "Orqa" in bold + suffix in light.
     Uses Inter font (the app default). No spacing between the two parts.
     Used in title bars, status bars, and splash screens. -->
<script lang="ts">
	let {
		suffix = "Studio",
		size = "sm",
	}: {
		/** The second word after "Orqa". E.g. "Studio", "DevTools". */
		suffix?: string;
		/** Text size preset. */
		size?: "xs" | "sm" | "md";
	} = $props();

	const sizeClass: Record<string, string> = {
		xs: "text-xs",
		sm: "text-sm",
		md: "text-base",
	};
</script>

<span class="font-sans leading-none tracking-tight {sizeClass[size] ?? sizeClass.sm}">
	<span class="font-bold">Orqa</span><span class="font-light">{suffix}</span>
</span>
