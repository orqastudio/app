<script lang="ts">
	import { onMount } from "svelte";

	let { text }: { text: string } = $props();

	let svgContent = $state<string | null>(null);
	let error = $state<string | null>(null);
	let loading = $state(true);

	/** Simple SVG cache keyed by diagram source text. */
	const svgCache: Record<string, string> = {};

	/** Detect whether the app is currently in dark mode. */
	function isDark(): boolean {
		if (typeof document === "undefined") return false;
		return document.documentElement.classList.contains("dark");
	}

	/**
	 * Encode PlantUML text for the server URL using the deflate + base64 approach
	 * described at https://plantuml.com/text-encoding
	 *
	 * Uses the browser's CompressionStream API (deflate-raw) and custom base64
	 * encoding with PlantUML's 64-character alphabet.
	 */
	async function encodePlantUml(source: string): Promise<string> {
		const data = new TextEncoder().encode(source);

		// Deflate using the browser's built-in CompressionStream
		const cs = new CompressionStream("deflate-raw");
		const writer = cs.writable.getWriter();
		writer.write(data);
		writer.close();

		const reader = cs.readable.getReader();
		const chunks: Uint8Array[] = [];
		let done = false;
		while (!done) {
			const result = await reader.read();
			if (result.value) chunks.push(result.value);
			done = result.done;
		}

		// Concatenate chunks
		const totalLength = chunks.reduce((acc, c) => acc + c.length, 0);
		const compressed = new Uint8Array(totalLength);
		let offset = 0;
		for (const chunk of chunks) {
			compressed.set(chunk, offset);
			offset += chunk.length;
		}

		return encode64(compressed);
	}

	/**
	 * PlantUML's custom base64 encoding.
	 * Uses the alphabet: 0-9, A-Z, a-z, -, _
	 */
	function encode64(data: Uint8Array): string {
		let result = "";
		const len = data.length;
		for (let i = 0; i < len; i += 3) {
			if (i + 2 === len) {
				result += append3bytes(data[i], data[i + 1], 0);
			} else if (i + 1 === len) {
				result += append3bytes(data[i], 0, 0);
			} else {
				result += append3bytes(data[i], data[i + 1], data[i + 2]);
			}
		}
		return result;
	}

	function append3bytes(b1: number, b2: number, b3: number): string {
		const c1 = b1 >> 2;
		const c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
		const c3 = ((b2 & 0xf) << 2) | (b3 >> 6);
		const c4 = b3 & 0x3f;
		return encode6bit(c1) + encode6bit(c2) + encode6bit(c3) + encode6bit(c4);
	}

	function encode6bit(b: number): string {
		if (b < 10) return String.fromCharCode(48 + b); // 0-9
		b -= 10;
		if (b < 26) return String.fromCharCode(65 + b); // A-Z
		b -= 26;
		if (b < 26) return String.fromCharCode(97 + b); // a-z
		b -= 26;
		if (b === 0) return "-";
		if (b === 1) return "_";
		return "?";
	}

	/**
	 * Inject dark-mode skin params if the app is in dark mode
	 * and the source does not already contain skinparam directives.
	 */
	function withTheme(source: string): string {
		const trimmed = source.trim();
		if (!isDark()) return trimmed;

		// Only inject if the user hasn't set their own skinparam
		if (trimmed.toLowerCase().includes("skinparam")) return trimmed;

		const darkSkin = [
			"skinparam backgroundColor #1e1e2e",
			"skinparam defaultFontColor #cdd6f4",
			"skinparam arrowColor #89b4fa",
			"skinparam classBorderColor #585b70",
			"skinparam classBackgroundColor #313244",
			"skinparam noteBorderColor #585b70",
			"skinparam noteBackgroundColor #313244",
			"skinparam sequenceLifeLineBorderColor #585b70",
			"skinparam sequenceParticipantBackgroundColor #313244",
			"skinparam sequenceParticipantBorderColor #585b70",
		].join("\n");

		// Insert skin params after the @startuml line (or at the beginning)
		const startTag = /@start\w+/;
		const match = trimmed.match(startTag);
		if (match && match.index !== undefined) {
			const insertPos = match.index + match[0].length;
			return trimmed.slice(0, insertPos) + "\n" + darkSkin + "\n" + trimmed.slice(insertPos);
		}

		return darkSkin + "\n" + trimmed;
	}

	async function fetchDiagram(): Promise<void> {
		loading = true;
		error = null;

		const themed = withTheme(text);
		const cacheKey = themed;

		const cached = svgCache[cacheKey];
		if (cached) {
			svgContent = cached;
			loading = false;
			return;
		}

		try {
			const encoded = await encodePlantUml(themed);
			const url = `https://www.plantuml.com/plantuml/svg/${encoded}`;
			const response = await fetch(url);

			if (!response.ok) {
				throw new Error(`PlantUML server returned ${response.status}`);
			}

			const svg = await response.text();
			svgCache[cacheKey] = svg;
			svgContent = svg;
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : String(err);
			svgContent = null;
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		fetchDiagram();
	});

	// Re-fetch when text changes.
	$effect(() => {
		void text;
		fetchDiagram();
	});

	// Re-render when theme changes.
	onMount(() => {
		const observer = new MutationObserver(() => {
			fetchDiagram();
		});
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["class"],
		});
		return () => observer.disconnect();
	});
</script>

<div class="plantuml-diagram overflow-x-auto rounded border border-border bg-muted/30 p-4">
	{#if loading}
		<div class="flex items-center justify-center py-8 text-xs text-muted-foreground">
			Loading PlantUML diagram...
		</div>
	{:else if error}
		<div class="rounded bg-destructive/10 p-3 text-xs text-destructive">
			<p class="font-semibold">PlantUML render error</p>
			<pre class="mt-1 whitespace-pre-wrap">{error}</pre>
		</div>
	{:else if svgContent}
		<div class="flex justify-center [&_svg]:max-w-full">
			<!-- eslint-disable-next-line svelte/no-at-html-tags -- PlantUML server returns SVG content that must be rendered as HTML -->
			{@html svgContent}
		</div>
	{/if}
</div>
