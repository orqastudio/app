<script lang="ts">
	import { Badge } from "$lib/components/ui/badge";
	import ArtifactLink from "./ArtifactLink.svelte";

	let {
		metadata,
		artifactType,
	}: {
		metadata: Record<string, string | string[]>;
		artifactType: string;
	} = $props();

	/** Returns the badge variant for a status value. */
	function statusVariant(
		status: string,
	): "default" | "secondary" | "destructive" | "outline" | "warning" {
		const s = status.toLowerCase();
		if (["draft", "captured", "todo", "proposed", "planning"].includes(s)) return "secondary";
		if (["active", "in-progress", "exploring", "ready"].includes(s)) return "default";
		if (["done", "complete", "accepted", "shaped"].includes(s)) return "outline";
		if (["review"].includes(s)) return "warning";
		if (["superseded", "deprecated", "archived"].includes(s)) return "destructive";
		return "secondary";
	}

	/** Returns a Tailwind class suffix for priority badges. */
	function priorityClass(priority: string): string {
		if (priority === "P1") return "bg-destructive/15 text-destructive border-destructive/30";
		if (priority === "P2") return "bg-warning/15 text-warning border-warning/30";
		if (priority === "P3") return "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border-emerald-500/30";
		return "";
	}

	/** Format an ISO date string to a readable date. */
	function formatDate(value: string): string {
		try {
			return new Date(value).toLocaleDateString(undefined, {
				year: "numeric",
				month: "short",
				day: "numeric",
			});
		} catch {
			return value;
		}
	}

	/** Pattern that matches artifact IDs like EPIC-005, MS-001, AD-017, IMPL-003. */
	const ARTIFACT_ID_RE = /^(MS|EPIC|TASK|IDEA|AD|IMPL)-\d+$/;

	function isArtifactId(value: string): boolean {
		return ARTIFACT_ID_RE.test(value.trim());
	}

	/**
	 * Fields rendered with a dedicated display or shown outside the metadata card.
	 * Skip them in the generic loop.
	 */
	const HANDLED_FIELDS = new Set([
		"id",
		"title",
		"description",
		"status",
		"priority",
		"milestone",
		"epic",
		"plan",
		"depends-on",
		"blocks",
		"supersedes",
		"superseded-by",
		"promoted-to",
		"research-refs",
		"docs-required",
		"docs-produced",
		"tags",
		"created",
		"updated",
		"deadline",
		"gate",
		"epic-count",
		"completed-epics",
		"scoring",
	]);

	/** Fields that contain artifact ID references. */
	const ARTIFACT_LINK_FIELDS = [
		"milestone",
		"epic",
		"depends-on",
		"blocks",
		"supersedes",
		"superseded-by",
		"promoted-to",
	];

	/** File-path list fields. */
	const FILE_LIST_FIELDS = ["docs-required", "docs-produced", "research-refs", "plan"];

	/** Date fields. */
	const DATE_FIELDS = ["created", "updated", "deadline"];

	const id = $derived(metadata["id"] as string | undefined);
	const title = $derived(metadata["title"] as string | undefined);
	const description = $derived(metadata["description"] as string | undefined);
	const status = $derived(metadata["status"] as string | undefined);
	const priority = $derived(metadata["priority"] as string | undefined);
	const gate = $derived(metadata["gate"] as string | undefined);
	const epicCount = $derived(metadata["epic-count"] as string | undefined);
	const completedEpics = $derived(metadata["completed-epics"] as string | undefined);
	const tags = $derived(metadata["tags"] as string | string[] | undefined);
	const tagList = $derived(
		tags === undefined
			? []
			: Array.isArray(tags)
				? tags
				: tags
						.split(",")
						.map((t) => t.trim())
						.filter(Boolean),
	);

	/** Generic remaining fields to render as label/value rows. */
	const extraFields = $derived(
		Object.entries(metadata).filter(
			([key]) => !HANDLED_FIELDS.has(key) && !ARTIFACT_LINK_FIELDS.includes(key),
		),
	);

	function asArray(value: string | string[]): string[] {
		return Array.isArray(value) ? value : [value];
	}
</script>

<!-- Title — from YAML `title` field -->
{#if title}
	<h1 class="mb-1 text-2xl font-bold leading-snug">{title}</h1>
{/if}

<!-- Description — from YAML `description` field -->
{#if description}
	<p class="mb-4 text-sm leading-relaxed text-muted-foreground">{description}</p>
{:else if title}
	<div class="mb-4"></div>
{/if}

<!-- Metadata card — shows all YAML fields except title and description -->
<div class="mb-4 space-y-3 rounded-lg border border-border bg-muted/30 px-4 py-3">
	<!-- ID + Status/Priority row -->
	<div class="flex items-start justify-between gap-3">
		<div class="space-y-0.5">
			{#if id}
				<p class="font-mono text-xs font-semibold uppercase tracking-widest text-muted-foreground">
					{artifactType} · {id}
				</p>
			{/if}
		</div>

		<div class="flex shrink-0 flex-col items-end gap-1.5">
			{#if status}
				<Badge variant={statusVariant(status)}>{status}</Badge>
			{/if}
			{#if priority}
				<span
					class="inline-flex items-center rounded-full border px-2 py-0.5 text-xs font-medium {priorityClass(priority)}"
				>
					{priority}
				</span>
			{/if}
		</div>
	</div>

	<!-- Artifact link fields -->
	{#each ARTIFACT_LINK_FIELDS as field (field)}
		{#if metadata[field] !== undefined}
			{@const values = asArray(metadata[field])}
			<div class="flex flex-wrap items-center gap-2">
				<span class="min-w-[7rem] text-xs font-medium capitalize text-muted-foreground">
					{field.replace(/-/g, " ")}
				</span>
				<div class="flex flex-wrap gap-1">
					{#each values as val (val)}
						{#if isArtifactId(val.trim())}
							<ArtifactLink id={val.trim()} />
						{:else}
							<span class="text-xs text-foreground">{val}</span>
						{/if}
					{/each}
				</div>
			</div>
		{/if}
	{/each}

	<!-- File list fields -->
	{#each FILE_LIST_FIELDS as field (field)}
		{#if metadata[field] !== undefined}
			{@const values = asArray(metadata[field])}
			<div class="flex flex-wrap items-start gap-2">
				<span class="min-w-[7rem] text-xs font-medium capitalize text-muted-foreground">
					{field.replace(/-/g, " ")}
				</span>
				<div class="flex flex-wrap gap-1">
					{#each values as val (val)}
						<span
							class="rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-[11px] text-foreground"
						>
							{val}
						</span>
					{/each}
				</div>
			</div>
		{/if}
	{/each}

	<!-- Date fields -->
	{#each DATE_FIELDS as field (field)}
		{#if metadata[field] !== undefined}
			<div class="flex items-center gap-2">
				<span class="min-w-[7rem] text-xs font-medium capitalize text-muted-foreground">
					{field.replace(/-/g, " ")}
				</span>
				<span class="text-xs text-foreground">{formatDate(metadata[field] as string)}</span>
			</div>
		{/if}
	{/each}

	<!-- Epic progress -->
	{#if epicCount !== undefined || completedEpics !== undefined}
		<div class="flex items-center gap-2">
			<span class="min-w-[7rem] text-xs font-medium text-muted-foreground">Progress</span>
			<span class="text-xs text-foreground">
				{completedEpics ?? "0"} / {epicCount ?? "?"} epics
			</span>
		</div>
	{/if}

	<!-- Gate -->
	{#if gate}
		<div class="rounded border border-border bg-muted/40 px-3 py-2">
			<p class="text-xs font-medium text-muted-foreground">Gate question</p>
			<p class="mt-0.5 text-sm italic text-foreground">"{gate}"</p>
		</div>
	{/if}

	<!-- Tags -->
	{#if tagList.length > 0}
		<div class="flex flex-wrap items-center gap-2">
			<span class="min-w-[7rem] text-xs font-medium text-muted-foreground">Tags</span>
			<div class="flex flex-wrap gap-1">
				{#each tagList as tag (tag)}
					<span
						class="rounded-full border border-border bg-secondary px-2 py-0.5 text-[11px] text-secondary-foreground"
					>
						{tag}
					</span>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Generic extra fields -->
	{#each extraFields as [key, value] (key)}
		<div class="flex items-start gap-2">
			<span class="min-w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
				{key.replace(/-/g, " ")}
			</span>
			{#if Array.isArray(value)}
				<div class="flex flex-wrap gap-1">
					{#each value as v (v)}
						<span class="text-xs text-foreground">{v}</span>
					{/each}
				</div>
			{:else}
				<span class="text-xs text-foreground">{value}</span>
			{/if}
		</div>
	{/each}
</div>

