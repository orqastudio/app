<script lang="ts">
	import Breadcrumb from "./Breadcrumb.svelte";
	import FrontmatterHeader from "./FrontmatterHeader.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";

	const artifact = $derived(artifactStore.activeArtifact);
	const breadcrumbs = $derived(navigationStore.breadcrumbs);
	const activity = $derived(navigationStore.activeActivity);
	const parsedContent = $derived(artifact ? parseFrontmatter(artifact.content) : null);

	/**
	 * Strip a leading `# Heading` line and the first paragraph from body content.
	 * Used when the title and description are shown via FrontmatterHeader so they
	 * are not duplicated in the markdown body.
	 */
	function stripLeadingHeadingAndDescription(body: string): string {
		let text = body.trimStart();

		// Strip leading `# Heading` line (level-1 only)
		text = text.replace(/^#\s[^\n]*\n?/, "");

		// Strip one blank line separator after the heading
		text = text.replace(/^\n/, "");

		// Strip leading paragraph (non-blank lines until the first blank line or heading)
		// Only strip if it looks like a description paragraph (not a heading, list, or code block)
		const firstChar = text.trimStart()[0];
		if (firstChar && firstChar !== "#" && firstChar !== "-" && firstChar !== "`" && firstChar !== "|") {
			text = text.replace(/^[^\n]+(\n[^\n]+)*\n?/, "");
		}

		return text.trimStart();
	}

	/**
	 * Whether this artifact has frontmatter with a title field.
	 * When true, we strip the leading heading/description from the body to avoid duplication.
	 */
	const hasFrontmatterTitle = $derived(
		parsedContent !== null && typeof parsedContent.metadata["title"] === "string",
	);

	/**
	 * Whether the artifact has any metadata fields beyond title and description.
	 * Determines whether to render the metadata card.
	 */
	const hasMetadataFields = $derived(
		parsedContent !== null &&
		Object.keys(parsedContent.metadata).some(
			(k) => k !== "title" && k !== "description",
		),
	);

	/**
	 * Body to render: when frontmatter has a title, strip the duplicate heading/description
	 * from the markdown body since FrontmatterHeader already shows them.
	 */
	const bodyToRender = $derived(
		parsedContent
			? hasFrontmatterTitle
				? stripLeadingHeadingAndDescription(parsedContent.body)
				: parsedContent.body
			: null,
	);

	function handleContentClick(event: MouseEvent) {
		const anchor = (event.target as HTMLElement).closest("a");
		if (!anchor) return;

		const href = anchor.getAttribute("href");
		if (!href) return;

		// Internal doc links start with / and don't have a protocol
		if (href.startsWith("/") && !href.includes("://")) {
			event.preventDefault();
			// Strip leading / and trailing /
			const docPath = href.replace(/^\/+/, "").replace(/\/+$/, "");
			if (!docPath) return;

			// Build breadcrumbs from path segments
			const crumbs = docPath.split("/").map((seg) =>
				seg
					.split("-")
					.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
					.join(" ")
			);
			navigationStore.openArtifact(docPath, crumbs);
		}
	}

</script>

<div class="flex h-full flex-col">
	<!-- Breadcrumb bar (hidden on home/landing pages) -->
	{#if breadcrumbs.length > 0}
		<div class="flex h-10 items-center justify-between border-b border-border px-4">
			<Breadcrumb items={breadcrumbs} />
		</div>
	{/if}

	<!-- Content -->
	{#if artifactStore.loading}
		<div class="flex flex-1 items-center justify-center">
			<LoadingSpinner />
		</div>
	{:else if artifactStore.error}
		<div class="flex flex-1 items-center justify-center px-4">
			<ErrorDisplay message={artifactStore.error} />
		</div>
	{:else if artifact}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="min-h-0 flex-1 overflow-y-auto" onclick={handleContentClick}>
			<div class="p-6">
				{#if parsedContent}
					{#if hasMetadataFields}
						<FrontmatterHeader
							metadata={parsedContent.metadata}
							artifactType={activity}
						/>
					{:else if hasFrontmatterTitle}
						<!-- Title + description only, no metadata card -->
						{@const title = parsedContent.metadata["title"] as string}
						{@const description = parsedContent.metadata["description"] as string | undefined}
						<h1 class="mb-1 text-2xl font-bold leading-snug">{title}</h1>
						{#if description}
							<p class="mb-6 text-sm leading-relaxed text-muted-foreground">{description}</p>
						{:else}
							<div class="mb-6"></div>
						{/if}
					{/if}
					<MarkdownRenderer content={bodyToRender ?? parsedContent.body} />
				{:else}
					<MarkdownRenderer content={artifact.content} />
				{/if}
			</div>
		</div>
	{:else}
		<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
			Select an artifact to view its contents
		</div>
	{/if}
</div>
