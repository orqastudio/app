<script lang="ts">
	import SvelteMarkdown from "@humanspeak/svelte-markdown";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import DiagramCodeBlock from "$lib/components/content/DiagramCodeBlock.svelte";
	import MarkdownLink from "$lib/components/content/MarkdownLink.svelte";

	let { content }: { content: string } = $props();

	// Strip YAML frontmatter defensively so callers that pass raw file content
	// don't render the --- block as markdown text.
	const rawBody = $derived(parseFrontmatter(content).body);

	/**
	 * Preprocess custom directives into fenced code blocks that DiagramCodeBlock
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
</script>

<div class="prose prose-sm dark:prose-invert max-w-none [&_:not(pre)>code]:rounded [&_:not(pre)>code]:bg-muted [&_:not(pre)>code]:px-1.5 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:font-mono [&_:not(pre)>code]:text-[11px] [&_:not(pre)>code]:text-foreground [&_:not(pre)>code]:before:content-none [&_:not(pre)>code]:after:content-none [&_:not(pre)>code]:font-normal">
	<SvelteMarkdown source={body} renderers={{ code: DiagramCodeBlock, link: MarkdownLink }} />
</div>
