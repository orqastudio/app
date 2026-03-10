<script lang="ts">
	import SvelteMarkdown from "@humanspeak/svelte-markdown";
	import { parseFrontmatter } from "$lib/utils/frontmatter";
	import { navigationStore } from "$lib/stores/navigation.svelte";

	let { content }: { content: string } = $props();

	// Strip YAML frontmatter defensively so callers that pass raw file content
	// don't render the --- block as markdown text.
	const body = $derived(parseFrontmatter(content).body);

	let containerEl = $state<HTMLDivElement | undefined>(undefined);

	/** Regex matching artifact IDs like EPIC-005, MS-001, AD-017, IMPL-003, RES-032, PILLAR-001, RULE-031. */
	const ARTIFACT_ID_RE = /\b(EPIC|TASK|AD|MS|IDEA|IMPL|RES|PILLAR|RULE)-(\d{3,})\b/g;

	/**
	 * After the markdown renders, walk text nodes in the container
	 * and replace artifact ID patterns with clickable buttons.
	 * Skips content inside <code> and <pre> elements.
	 */
	function processArtifactLinks(root: HTMLDivElement) {
		const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, {
			acceptNode(node) {
				// Skip code/pre blocks
				let parent = node.parentElement;
				while (parent && parent !== root) {
					const tag = parent.tagName.toLowerCase();
					if (tag === "code" || tag === "pre") return NodeFilter.FILTER_REJECT;
					parent = parent.parentElement;
				}
				return ARTIFACT_ID_RE.test(node.nodeValue ?? "") ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT;
			},
		});

		const nodesToReplace: Text[] = [];
		let textNode: Text | null;
		while ((textNode = walker.nextNode() as Text | null)) {
			nodesToReplace.push(textNode);
		}

		for (const node of nodesToReplace) {
			const text = node.nodeValue ?? "";
			ARTIFACT_ID_RE.lastIndex = 0;
			const fragment = document.createDocumentFragment();
			let lastIndex = 0;
			let match: RegExpExecArray | null;

			while ((match = ARTIFACT_ID_RE.exec(text)) !== null) {
				if (match.index > lastIndex) {
					fragment.appendChild(document.createTextNode(text.slice(lastIndex, match.index)));
				}

				const id = match[0];
				const btn = document.createElement("button");
				btn.textContent = id;
				btn.className =
					"text-primary hover:underline cursor-pointer";
				btn.title = `Navigate to ${id}`;
				btn.addEventListener("click", (e) => {
					e.stopPropagation();
					navigationStore.navigateToArtifact(id);
				});

				fragment.appendChild(btn);
				lastIndex = match.index + id.length;
			}

			if (lastIndex < text.length) {
				fragment.appendChild(document.createTextNode(text.slice(lastIndex)));
			}

			node.parentNode?.replaceChild(fragment, node);
		}
	}

	$effect(() => {
		if (!containerEl) return;
		// Re-process whenever body changes. Use a microtask to let SvelteMarkdown finish rendering.
		// Guard with a cancelled flag so that if body changes again before the microtask fires,
		// the stale microtask is a no-op and only the latest one processes the DOM.
		const el = containerEl;
		const _body = body; // track dependency
		void _body;
		let cancelled = false;
		queueMicrotask(() => {
			if (!cancelled) processArtifactLinks(el);
		});
		return () => {
			cancelled = true;
		};
	});
</script>

<div class="prose prose-sm dark:prose-invert max-w-none" bind:this={containerEl}>
	<SvelteMarkdown source={body} />
</div>
