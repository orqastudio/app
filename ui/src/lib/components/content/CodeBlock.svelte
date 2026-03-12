<script lang="ts">
	import CopyIcon from "@lucide/svelte/icons/copy";
	import CheckIcon from "@lucide/svelte/icons/check";
	import { Badge } from "$lib/components/ui/badge";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { Highlight, type LanguageType } from "svelte-highlight";
	import bash from "svelte-highlight/languages/bash";
	import rust from "svelte-highlight/languages/rust";
	import typescript from "svelte-highlight/languages/typescript";
	import javascript from "svelte-highlight/languages/javascript";
	import json from "svelte-highlight/languages/json";
	import yaml from "svelte-highlight/languages/yaml";
	import css from "svelte-highlight/languages/css";
	import sql from "svelte-highlight/languages/sql";
	import markdown from "svelte-highlight/languages/markdown";
	import xml from "svelte-highlight/languages/xml";
	import plaintext from "svelte-highlight/languages/plaintext";

	let {
		text,
		lang = "",
	}: {
		text: string;
		lang?: string;
	} = $props();

	const LANGUAGES: Record<string, LanguageType<string>> = {
		bash, sh: bash, shell: bash, zsh: bash,
		rust, rs: rust,
		typescript, ts: typescript,
		javascript, js: javascript,
		json, jsonc: json,
		yaml, yml: yaml,
		css, scss: css,
		sql,
		markdown, md: markdown,
		html: xml, xml, svelte: xml,
		text: plaintext, plaintext, txt: plaintext,
	};

	const displayLang = $derived(lang || "text");
	const resolvedLang = $derived(LANGUAGES[displayLang.toLowerCase()] ?? plaintext);

	let copied = $state(false);

	function copyToClipboard() {
		navigator.clipboard.writeText(text).then(() => {
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		});
	}
</script>

<div class="group relative border border-border bg-muted/30">
	<div class="flex items-center justify-between border-b border-border px-1 py-1">
		<Badge variant="secondary" class="font-mono text-[10px] uppercase">{displayLang}</Badge>
		<button
			class="flex items-center gap-1 rounded px-1.5 py-0.5 text-[10px] uppercase tracking-wider text-muted-foreground opacity-0 transition-opacity hover:bg-accent group-hover:opacity-100"
			onclick={copyToClipboard}
		>
			{#if copied}
				<CheckIcon size={10} class="text-success" />
				<span>Copied</span>
			{:else}
				<CopyIcon size={12} />
			{/if}
		</button>
	</div>
	<ScrollArea orientation="horizontal" class="codeblock-highlight text-sm [&_pre]:!bg-transparent [&_pre]:!p-1 [&_pre]:!my-0 [&_code]:!bg-transparent">
		<Highlight language={resolvedLang} code={text} />
	</ScrollArea>
</div>
