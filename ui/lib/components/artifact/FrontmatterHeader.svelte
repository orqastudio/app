<script lang="ts">
	import ArtifactLink from "./ArtifactLink.svelte";
	import GateQuestions from "./GateQuestions.svelte";
	import StatusIndicator from "$lib/components/shared/StatusIndicator.svelte";
	import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

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

	/**
	 * Returns true when the SDK can resolve the value to a known artifact node.
	 * This is authoritative — no regex needed; the graph is the source of truth.
	 */
	function isArtifactId(value: string): boolean {
		return artifactGraphSDK.resolve(value.trim()) !== undefined;
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
		"id", "title", "description", "status", "priority", "scoring",
	]);

	const DATE_FIELDS = new Set(["created", "updated", "deadline"]);

	/** FILE_LIST_FIELDS: path-like values rendered as monospace chips. */
	const FILE_LIST_FIELDS = new Set([
		"docs-required", "docs-produced",
	]);

	/**
	 * LINK_FIELDS: values that are artifact IDs and should render as clickable ArtifactLink chips.
	 * research-refs added here (RES-NNN IDs).
	 */
	const LINK_FIELDS = new Set([
		"milestone", "epic", "pillars", "promoted-to", "promoted_to",
		"surpassed-by", "supersedes", "superseded-by",
		"depends-on", "blocks", "research-refs",
	]);

	/**
	 * CHIP_FIELDS: rendered as styled chips but NOT clickable links.
	 */
	const CHIP_FIELDS = new Set([
		"assignee", "skills",
	]);

	/** Classify a field key into its render type. */
	type FieldType = "date" | "file-list" | "link" | "chip" | "generic";

	function fieldType(key: string): FieldType {
		if (DATE_FIELDS.has(key)) return "date";
		if (FILE_LIST_FIELDS.has(key)) return "file-list";
		if (LINK_FIELDS.has(key)) return "link";
		if (CHIP_FIELDS.has(key)) return "chip";
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
/**
	 * Gate question — extracted and rendered last,
	 * separated from the main body entries loop.
	 */
	/** Gate — supports both a single string (milestones) and an array (pillars). */
	const gateQuestions = $derived(
		isPresent(metadata["gate"]) ? asArray(metadata["gate"]).filter(Boolean) : [],
	);

	/**
	 * The ordered body entries from the metadata object, skipping:
	 * - Fixed header fields (SKIP_FIELDS)
	 * - Progress fields (rendered as a combined row)
	 * - Gate field (rendered separately at the end)
	 * - Entries without a present value
	 */
	const bodyEntries = $derived(
		Object.entries(metadata).filter(([key, value]) => {
			if (SKIP_FIELDS.has(key)) return false;
			if (key === "gate") return false;
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
	<!-- ID + Status/Priority row — only rendered when at least one value is present -->
	{#if id || (status && isPresent(status)) || priority}
		<div class="flex items-start justify-between gap-3">
			<div class="space-y-0.5">
				{#if id}
					<p class="font-mono text-xs font-semibold uppercase tracking-widest text-muted-foreground">
						{artifactType} · {id}
					</p>
				{/if}
			</div>

			<div class="flex shrink-0 items-center gap-1.5">
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
					<span class="min-w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
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
				<div class="flex flex-wrap items-start gap-2">
					<span class="min-w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
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

		{:else if type === "chip"}
			{@const items = asArray(value).filter(Boolean)}
			{#if items.length > 0}
				<div class="flex flex-wrap items-start gap-2">
					<span class="min-w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex flex-wrap gap-1">
						{#each items as item, i (i)}
							<span class="rounded-full border border-border bg-secondary px-2 py-0.5 text-[11px] capitalize text-secondary-foreground">
								{item}
							</span>
						{/each}
					</div>
				</div>
			{/if}

		{:else}
			<!-- generic -->
			<div class="flex items-start gap-2">
				<span class="min-w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
					{humanizeKey(key)}
				</span>
				{#if Array.isArray(value)}
					<div class="flex flex-wrap gap-1">
						{#each value as v, i (i)}
							<span class="rounded-full border border-border bg-secondary px-2 py-0.5 text-[11px] capitalize text-secondary-foreground">
								{v}
							</span>
						{/each}
					</div>
				{:else}
					<span class="text-xs capitalize text-foreground">{String(value)}</span>
				{/if}
			</div>
		{/if}
	{/each}

	<!-- Gate question(s) — always last -->
	<GateQuestions questions={gateQuestions} />
</div>
