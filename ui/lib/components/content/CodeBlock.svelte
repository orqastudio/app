<script lang="ts">
	import CopyIcon from "@lucide/svelte/icons/copy";
	import CheckIcon from "@lucide/svelte/icons/check";
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
	};

	const resolvedLang = $derived(LANGUAGES[lang.toLowerCase()] ?? undefined);

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

<div class="group relative rounded-md border border-border bg-muted/30">
	<div class="flex items-center justify-between border-b border-border px-3 py-1.5">
		{#if lang}
			<span class="text-xs text-muted-foreground">{lang}</span>
		{:else}
			<span></span>
		{/if}
		<button
			class="flex items-center gap-1 rounded px-1.5 py-0.5 text-xs text-muted-foreground opacity-0 transition-opacity hover:bg-accent group-hover:opacity-100"
			onclick={copyToClipboard}
		>
			{#if copied}
				<CheckIcon class="h-3.5 w-3.5 text-success" />
				<span>Copied</span>
			{:else}
				<CopyIcon class="h-3.5 w-3.5" />
				<span>Copy</span>
			{/if}
		</button>
	</div>
	{#if resolvedLang}
		<div class="codeblock-highlight overflow-x-auto text-sm [&_pre]:!bg-transparent [&_pre]:!p-3 [&_code]:!bg-transparent">
			<Highlight language={resolvedLang} code={text} />
		</div>
	{:else}
		<pre class="overflow-x-auto p-3 text-sm"><code>{text}</code></pre>
	{/if}
</div>
