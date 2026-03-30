<script lang="ts">
	import SvelteMarkdown from "@humanspeak/svelte-markdown";
	import { parseFrontmatter } from "@orqastudio/sdk";
	import type { Component } from "svelte";

	let {
		content,
		codeRenderer = undefined,
		linkRenderer = undefined,
	}: {
		content: string;
		/** Optional renderer component for fenced code blocks. Receives {text, lang} props. */
		codeRenderer?: Component<{ text: string; lang?: string }> | undefined;
		/** Optional renderer component for links. Receives {href, title, children} props. */
		linkRenderer?: Component<{ href?: string; title?: string }> | undefined;
	} = $props();

	// Strip YAML frontmatter defensively so callers that pass raw file content
	// don't render the --- block as markdown text.
	const rawBody = $derived(parseFrontmatter(content).body);

	/**
	 * Preprocess custom directives into fenced code blocks that a codeRenderer
	 * can render. Supports:
	 *   :::artifacts{type="task" parent="EPIC-067" field="epic"}
	 * which becomes a code block with lang="artifacts-table".
	 */
	function preprocessDirectives(md: string): string {
		// Match :::artifacts{key="value" ...} (single line directive)
		return md.replace(
			/^:::artifacts\{([^}]+)\}\s*$/gm,
			(_match, attrs: string) => {
				return "```artifacts-table\n" + attrs.trim() + "\n```";
			}
		);
	}

	const body = $derived(preprocessDirectives(rawBody));

	/** Build the renderers map, only including overrides that were provided. */
	const renderers = $derived.by(() => {
		const r: Record<string, Component> = {};
		if (codeRenderer) r["code"] = codeRenderer as Component;
		if (linkRenderer) r["link"] = linkRenderer as Component;
		return r;
	});
</script>

<div class="prose prose-sm dark:prose-invert max-w-none [&_:not(pre)>code]:rounded [&_:not(pre)>code]:bg-muted [&_:not(pre)>code]:px-1.5 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:font-mono [&_:not(pre)>code]:text-[11px] [&_:not(pre)>code]:text-foreground [&_:not(pre)>code]:before:content-none [&_:not(pre)>code]:after:content-none [&_:not(pre)>code]:font-normal">
	<SvelteMarkdown source={body} {renderers} />
</div>
