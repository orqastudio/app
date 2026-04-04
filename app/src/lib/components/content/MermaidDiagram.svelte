<script lang="ts">
	import { onMount } from "svelte";
	import mermaid from "mermaid";
	import { Caption, Center } from "@orqastudio/svelte-components/pure";


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

<!-- Inline style required: background opacity and overflow-x cannot be expressed via Box typed props -->
<div style="border-radius: 0.375rem; border: 1px solid hsl(var(--border)); background: hsl(var(--muted) / 0.3); padding: 1rem; overflow-x: auto;">
	{#if rendering && !error && !svgContent}
		<Center padding={8}>
			<Caption tone="muted">Rendering diagram...</Caption>
		</Center>
	{/if}
	{#if error}
		<div style="border-radius: 0.375rem; background: hsl(var(--destructive) / 0.1); padding: 0.75rem;">
			<Caption variant="caption-strong" tone="destructive">Mermaid render error</Caption>
			<pre style="margin-top: 0.25rem; white-space: pre-wrap; font-size: 0.75rem; color: hsl(var(--destructive));">{error}</pre>
		</div>
	{:else if svgContent}
		<div style="display: flex; justify-content: center;">
			<!-- eslint-disable-next-line svelte/no-at-html-tags -- Mermaid produces sanitized SVG via its own securityLevel setting -->
			{@html svgContent}
		</div>
	{/if}
</div>
