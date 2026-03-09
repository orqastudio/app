<script lang="ts">
	import ArtifactLink from "./ArtifactLink.svelte";
	import StatusIndicator from "$lib/components/shared/StatusIndicator.svelte";

	let {
		metadata,
		artifactType,
	}: {
		metadata: Record<string, unknown>;
		artifactType: string;
	} = $props();

	/** Returns Tailwind classes for priority badges. */
	function priorityClass(priority: string): string {
		if (priority === "P1") return "bg-destructive/15 text-destructive border-destructive/30";
		if (priority === "P2") return "bg-warning/15 text-warning border-warning/30";
		if (priority === "P3") return "bg-emerald-500/15 text-emerald-600 dark:text-emerald-400 border-emerald-500/30";
		return "";
	}

	/** Returns human-readable label for priority. */
	function priorityLabel(priority: string): string {
		if (priority === "P1") return "P1 — Critical";
		if (priority === "P2") return "P2 — Important";
		if (priority === "P3") return "P3 — Nice to Have";
		return priority;
	}

	/** Format an ISO date string to a readable date; returns null for invalid/null values. */
	function formatDate(value: unknown): string | null {
		if (value === null || value === undefined || value === "" || value === "null") return null;
		try {
			const d = new Date(String(value));
			if (isNaN(d.getTime())) return null;
			return d.toLocaleDateString(undefined, {
				year: "numeric",
				month: "short",
				day: "numeric",
			});
		} catch {
			return null;
		}
	}

	/** Pattern that matches artifact IDs like EPIC-005, MS-001, AD-017, IMPL-003. */
	const ARTIFACT_ID_RE = /^(MS|EPIC|TASK|IDEA|AD|IMPL)-\d+$/;

	function isArtifactId(value: string): boolean {
		return ARTIFACT_ID_RE.test(value.trim());
	}

	/** Returns true if a value is non-empty (not null, undefined, empty string, or "null"). */
	function isPresent(value: unknown): boolean {
		if (value === null || value === undefined) return false;
		if (value === "" || value === "null") return false;
		if (Array.isArray(value) && value.length === 0) return false;
		return true;
	}

	function asArray(value: unknown): string[] {
		if (Array.isArray(value)) return value.map(String);
		if (typeof value === "string") return [value];
		return [String(value)];
	}

	/**
	 * Fields always rendered in the fixed header row (ID, status, priority)
	 * or handled outside the metadata card (title, description).
	 * These are skipped in the dynamic body loop.
	 */
	const SKIP_FIELDS = new Set([
		"id", "title", "description", "status", "priority", "tags",
		"enforcement", "scoring",
	]);

	const DATE_FIELDS = new Set(["created", "updated", "deadline"]);

	const FILE_LIST_FIELDS = new Set([
		"docs-required", "docs-produced", "research-refs", "plan", "scope",
	]);

	const LINK_FIELDS = new Set([
		"milestone", "epic", "promoted-to", "promoted_to",
		"surpassed-by", "supersedes", "superseded-by",
		"depends-on", "blocks",
	]);

	/** Classify a field key into its render type. */
	type FieldType = "date" | "file-list" | "link" | "gate" | "progress" | "generic";

	function fieldType(key: string): FieldType {
		if (DATE_FIELDS.has(key)) return "date";
		if (FILE_LIST_FIELDS.has(key)) return "file-list";
		if (LINK_FIELDS.has(key)) return "link";
		if (key === "gate") return "gate";
		if (key === "epic-count" || key === "completed-epics") return "progress";
		return "generic";
	}

	/** Humanize a kebab-case field key for display. */
	function humanizeKey(key: string): string {
		return key.replace(/-/g, " ").replace(/_/g, " ");
	}

	// --- Derived header values (always rendered first) ---
	const id = $derived(metadata["id"] as string | undefined);
	const title = $derived(metadata["title"] as string | undefined);
	const description = $derived(metadata["description"] as string | undefined);
	const status = $derived(metadata["status"] as string | undefined);
	const priority = $derived(
		isPresent(metadata["priority"]) ? String(metadata["priority"]) : undefined,
	);
	const tags = $derived(metadata["tags"] as string | string[] | undefined);
	const tagList = $derived(
		tags === undefined || tags === null || tags === ""
			? []
			: Array.isArray(tags)
				? tags.filter(Boolean)
				: String(tags).split(",").map((t) => t.trim()).filter(Boolean),
	);

	/**
	 * Progress fields — rendered together as "X / Y epics".
	 * Tracked separately so the progress row can combine them.
	 */
	const epicCount = $derived(metadata["epic-count"] as string | undefined);
	const completedEpics = $derived(metadata["completed-epics"] as string | undefined);
	const hasProgress = $derived(isPresent(epicCount) || isPresent(completedEpics));

	/**
	 * The ordered body entries from the metadata object, skipping:
	 * - Fixed header fields (SKIP_FIELDS)
	 * - Progress fields (rendered as a combined row)
	 * - Entries without a present value
	 */
	const bodyEntries = $derived(
		Object.entries(metadata).filter(([key, value]) => {
			if (SKIP_FIELDS.has(key)) return false;
			if (key === "epic-count" || key === "completed-epics") return false;
			if (!isPresent(value)) return false;
			return true;
		}),
	);
</script>

<!-- Title -->
{#if title}
	<h1 class="mb-1 text-2xl font-bold leading-snug">{title}</h1>
{/if}

<!-- Description -->
{#if description}
	<p class="mb-4 text-sm leading-relaxed text-muted-foreground">{description}</p>
{:else if title}
	<div class="mb-4"></div>
{/if}

<!-- Metadata card -->
<div class="mb-4 space-y-3 rounded-lg border border-border bg-muted/30 px-4 py-3">
	<!-- ID + Status/Priority row — always first -->
	<div class="flex items-start justify-between gap-3">
		<div class="space-y-0.5">
			{#if id}
				<p class="font-mono text-xs font-semibold uppercase tracking-widest text-muted-foreground">
					{artifactType} · {id}
				</p>
			{/if}
		</div>

		<div class="flex shrink-0 flex-col items-end gap-1.5">
			{#if status && isPresent(status)}
				<StatusIndicator {status} mode="badge" />
			{/if}
			{#if priority}
				<span
					class="inline-flex items-center rounded-full border px-2 py-0.5 text-xs font-medium {priorityClass(priority)}"
				>
					{priorityLabel(priority)}
				</span>
			{/if}
		</div>
	</div>

	<!-- Progress (epic-count + completed-epics combined) -->
	{#if hasProgress}
		<div class="flex items-center gap-2">
			<span class="min-w-[7rem] text-xs font-medium text-muted-foreground">Progress</span>
			<span class="text-xs text-foreground">
				{completedEpics ?? "0"} / {epicCount ?? "?"} epics
			</span>
		</div>
	{/if}

	<!-- Dynamic body — YAML source order, type-dispatched -->
	{#each bodyEntries as [key, value] (key)}
		{@const type = fieldType(key)}

		{#if type === "date"}
			{@const formatted = formatDate(value)}
			{#if formatted}
				<div class="flex items-center gap-2">
					<span class="min-w-[7rem] text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<span class="text-xs text-foreground">{formatted}</span>
				</div>
			{/if}

		{:else if type === "file-list"}
			{@const items = asArray(value).filter(Boolean)}
			{#if items.length > 0}
				<div class="flex flex-wrap items-start gap-2">
					<span class="min-w-[7rem] text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex flex-wrap gap-1">
						{#each items as item, i (i)}
							<span
								class="rounded border border-border bg-muted px-1.5 py-0.5 font-mono text-[11px] text-foreground"
							>
								{item}
							</span>
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "link"}
			{@const vals = asArray(value).filter(Boolean)}
			{#if vals.length > 0}
				<div class="flex flex-wrap items-center gap-2">
					<span class="min-w-[7rem] text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex flex-wrap gap-1">
						{#each vals as val, i (i)}
							{#if isArtifactId(val.trim())}
								<ArtifactLink id={val.trim()} />
							{:else}
								<span class="text-xs text-foreground">{val}</span>
							{/if}
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "gate"}
			<div class="rounded border border-border bg-muted/40 px-3 py-2">
				<p class="text-xs font-medium text-muted-foreground">Gate question</p>
				<p class="mt-0.5 text-sm italic text-foreground">"{String(value)}"</p>
			</div>

		{:else}
			<!-- generic -->
			<div class="flex items-start gap-2">
				<span class="min-w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
					{humanizeKey(key)}
				</span>
				{#if Array.isArray(value)}
					<div class="flex flex-wrap gap-1">
						{#each value as v, i (i)}
							<span class="text-xs text-foreground">{v}</span>
						{/each}
					</div>
				{:else}
					<span class="text-xs text-foreground">{String(value)}</span>
				{/if}
			</div>
		{/if}
	{/each}

	<!-- Tags — always last -->
	{#if tagList.length > 0}
		<div class="flex flex-wrap items-center gap-2">
			<span class="min-w-[7rem] text-xs font-medium text-muted-foreground">Tags</span>
			<div class="flex flex-wrap gap-1">
				{#each tagList as tag, i (i)}
					<span
						class="rounded-full border border-border bg-secondary px-2 py-0.5 text-[11px] text-secondary-foreground"
					>
						{tag}
					</span>
				{/each}
			</div>
		</div>
	{/if}
</div>
