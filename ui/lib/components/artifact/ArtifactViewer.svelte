<script lang="ts">
	import Breadcrumb from "./Breadcrumb.svelte";
	import FrontmatterHeader from "./FrontmatterHeader.svelte";
	import HookViewer from "./HookViewer.svelte";
	import ReferencesPanel from "./ReferencesPanel.svelte";
	import MarkdownRenderer from "$lib/components/content/MarkdownRenderer.svelte";
	import LoadingSpinner from "$lib/components/shared/LoadingSpinner.svelte";
	import ErrorDisplay from "$lib/components/shared/ErrorDisplay.svelte";
	import { ScrollArea } from "$lib/components/ui/scroll-area";
	import { artifactStore } from "$lib/stores/artifact.svelte";
	import { navigationStore } from "$lib/stores/navigation.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";
	import { parseFrontmatter } from "$lib/utils/frontmatter";

	const content = $derived(artifactStore.activeContent);
	const breadcrumbs = $derived(navigationStore.breadcrumbs);
	const activity = $derived(navigationStore.activeActivity);
	const currentPath = $derived(navigationStore.selectedArtifactPath);

	/**
	 * Graph node for the current artifact. Populated when the artifact is in the
	 * graph (i.e. after the watcher has indexed it). May be undefined for newly
	 * created files that the watcher hasn't processed yet.
	 */
	const graphNode = $derived(
		currentPath ? artifactGraphSDK.resolveByPath(currentPath) : undefined,
	);

	/**
	 * Effective metadata: prefer pre-parsed frontmatter from the graph node when
	 * available; fall back to parsing the raw content string for files not yet
	 * in the graph.
	 */
	const parsedContent = $derived.by(() => {
		if (!content) return null;
		if (graphNode) {
			// Use graph metadata — already parsed by the Rust backend.
			// Cast to the shape expected by FrontmatterHeader (Record<string, string | string[]>).
			const metadata = graphNode.frontmatter as Record<string, string | string[]>;
			// Still parse the body from raw content so we get the stripped-frontmatter markdown.
			const fallback = parseFrontmatter(content);
			return { metadata, body: fallback.body };
		}
		return parseFrontmatter(content);
	});

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
	 * File extension of the selected artifact path (lowercase, without the dot).
	 * Used to route non-markdown file types to their appropriate viewer.
	 */
	const fileExtension = $derived.by(() => {
		const path = navigationStore.selectedArtifactPath;
		if (!path) return "";
		const filename = path.split("/").pop() ?? "";
		const dotIndex = filename.lastIndexOf(".");
		return dotIndex !== -1 ? filename.slice(dotIndex + 1).toLowerCase() : "";
	});

	/**
	 * Whether this is a README file. READMEs are introductory pages —
	 * their YAML frontmatter is not displayed.
	 */
	const isReadme = $derived.by(() => {
		const path = navigationStore.selectedArtifactPath;
		if (!path) return false;
		const filename = path.split("/").pop() ?? "";
		return filename.toUpperCase().startsWith("README");
	});

	/**
	 * Whether this artifact has frontmatter with a title field.
	 * When true, we strip the leading heading/description from the body to avoid duplication.
	 */
	const hasFrontmatterTitle = $derived(
		!isReadme &&
		parsedContent !== null && typeof parsedContent.metadata["title"] === "string",
	);

	/**
	 * Whether the artifact has any metadata fields beyond title and description.
	 * Determines whether to render the metadata card.
	 */
	const hasMetadataFields = $derived(
		!isReadme &&
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

	/**
	 * Artifact type from the graph node. Falls back to activity string.
	 */
	const artifactType = $derived(graphNode?.artifact_type ?? activity);

	/**
	 * Pattern matching artifact IDs like EPIC-048, TASK-001, AD-017, MS-001, etc.
	 * These are all-uppercase prefix + hyphen + digits.
	 */
	const ARTIFACT_ID_RE = /^[A-Z]+-\d+$/;

	function handleContentClick(event: MouseEvent) {
		const anchor = (event.target as HTMLElement).closest("a");
		if (!anchor) return;

		const href = anchor.getAttribute("href");
		if (!href) return;

		// Artifact ID links (e.g. href="EPIC-048") — use SDK-based navigation.
		if (ARTIFACT_ID_RE.test(href.trim())) {
			event.preventDefault();
			navigationStore.navigateToArtifact(href.trim());
			return;
		}

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
	{#if artifactStore.activeContentLoading}
		<div class="flex flex-1 items-center justify-center">
			<LoadingSpinner size="lg" />
		</div>
	{:else if artifactStore.activeContentError}
		<div class="flex flex-1 items-center justify-center px-4">
			<ErrorDisplay message={artifactStore.activeContentError} />
		</div>
	{:else if content}
		{#if currentPath && !isReadme}
			<ReferencesPanel artifactPath={currentPath} />
		{/if}
		<ScrollArea class="min-h-0 flex-1" onclick={handleContentClick}>
			<div class="p-6">
				{#if fileExtension === "sh"}
					<HookViewer {content} />
				{:else if parsedContent}
					{#if hasMetadataFields}
						<FrontmatterHeader
							metadata={parsedContent.metadata}
							{artifactType}
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
					<MarkdownRenderer content={content} />
				{/if}
			</div>
		</ScrollArea>
	{:else}
		<div class="flex flex-1 items-center justify-center text-sm text-muted-foreground">
			Select an artifact to view its contents
		</div>
	{/if}
</div>
