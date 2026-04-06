<script lang="ts">
	import {
		Icon,
		Badge,
		Button,
		Panel,
		SectionHeader,
		HighlightWrapper,
	} from "@orqastudio/svelte-components/pure";
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
		bash,
		sh: bash,
		shell: bash,
		zsh: bash,
		rust,
		rs: rust,
		typescript,
		ts: typescript,
		javascript,
		js: javascript,
		json,
		jsonc: json,
		yaml,
		yml: yaml,
		css,
		scss: css,
		sql,
		markdown,
		md: markdown,
		html: xml,
		xml,
		svelte: xml,
		text: plaintext,
		plaintext,
		txt: plaintext,
	};

	const displayLang = $derived(lang || "text");
	const resolvedLang = $derived(LANGUAGES[displayLang.toLowerCase()] ?? plaintext);

	let copied = $state(false);

	/** Copy the code block text to the clipboard and briefly show a confirmation state. */
	function copyToClipboard() {
		navigator.clipboard.writeText(text).then(() => {
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		});
	}
</script>

<Panel background="muted-subtle" border="all" rounded="sm" padding="none">
	<SectionHeader variant="compact">
		{#snippet start()}
			<Badge variant="secondary" size="xs">{displayLang.toUpperCase()}</Badge>
		{/snippet}
		{#snippet end()}
			<Button
				variant="ghost"
				size="sm"
				onclick={copyToClipboard}
				aria-label={copied ? "Copied to clipboard" : "Copy code to clipboard"}
			>
				{#if copied}
					<Icon name="check" size="sm" />
					Copied
				{:else}
					<Icon name="copy" size="sm" />
				{/if}
			</Button>
		{/snippet}
	</SectionHeader>
	<HighlightWrapper>
		<Highlight language={resolvedLang} code={text} />
	</HighlightWrapper>
</Panel>
