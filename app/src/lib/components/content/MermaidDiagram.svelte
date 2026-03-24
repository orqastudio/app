<script lang="ts">
	import { onMount } from "svelte";
	import mermaid from "mermaid";

	let { text }: { text: string } = $props();

	let svgContent = $state<string | null>(null);
	let error = $state<string | null>(null);
	let rendering = $state(true);

	/** Detect whether the app is currently in dark mode. */
	function isDark(): boolean {
		if (typeof document === "undefined") return false;
		return document.documentElement.classList.contains("dark");
	}

	/** Generate a unique ID for each diagram render. */
	function diagramId(): string {
		return `mermaid-${Math.random().toString(36).slice(2, 10)}`;
	}

	async function renderDiagram(): Promise<void> {
		rendering = true;
		error = null;

		const theme = isDark() ? "dark" : "default";
		mermaid.initialize({
			startOnLoad: false,
			theme,
			securityLevel: "loose",
			fontFamily: "inherit",
		});

		try {
			const { svg } = await mermaid.render(diagramId(), text.trim());
			svgContent = svg;
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
			svgContent = null;
		} finally {
			rendering = false;
		}
	}

	onMount(() => {
		renderDiagram();
	});

	// Re-render when text changes.
	$effect(() => {
		void text;
		renderDiagram();
	});

	// Re-render when theme changes (observe the `dark` class on <html>).
	onMount(() => {
		const observer = new MutationObserver(() => {
			renderDiagram();
		});
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class"],
		});
		return () => observer.disconnect();
	});
</script>

<div class="mermaid-diagram overflow-x-auto rounded border border-border bg-muted/30 p-4">
	{#if rendering && !error && !svgContent}
		<div class="flex items-center justify-center py-8 text-xs text-muted-foreground">
			Rendering diagram...
		</div>
	{/if}
	{#if error}
		<div class="rounded bg-destructive/10 p-3 text-xs text-destructive">
			<p class="font-semibold">Mermaid render error</p>
			<pre class="mt-1 whitespace-pre-wrap">{error}</pre>
		</div>
	{:else if svgContent}
		<div class="flex justify-center [&_svg]:max-w-full">
			<!-- eslint-disable-next-line svelte/no-at-html-tags -- Mermaid produces sanitized SVG via its own securityLevel setting -->
			{@html svgContent}
		</div>
	{/if}
</div>
