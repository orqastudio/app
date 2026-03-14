<script lang="ts">
	import ArtifactLink from "./ArtifactLink.svelte";
	import GateQuestions from "./GateQuestions.svelte";
	import RelationshipsList from "./RelationshipsList.svelte";
	import StatusIndicator from "$lib/components/shared/StatusIndicator.svelte";
	import { Badge } from "$lib/components/ui/badge";
	import { getCapabilityLabel } from "$lib/utils/tool-display";
	import CalendarPlusIcon from "@lucide/svelte/icons/calendar-plus";
	import CalendarCheckIcon from "@lucide/svelte/icons/calendar-check";
	import CheckIcon from "@lucide/svelte/icons/check";
	import XIcon from "@lucide/svelte/icons/x";
	import ScaleIcon from "@lucide/svelte/icons/scale";

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
		"bodyTemplate", "tools", "capabilities", "created", "updated",
		"relationships", "enforcement", "rule-overrides",
		"acceptance",
	]);

	const DATE_FIELDS = new Set(["created", "updated", "deadline"]);

	/** FILE_LIST_FIELDS: path-like values rendered as monospace chips. */
	const FILE_LIST_FIELDS = new Set<string>([]);

	/**
	 * LINK_FIELDS: values that are artifact IDs and should render as clickable ArtifactLink chips.
	 * research-refs added here (RES-NNN IDs).
	 */
	const LINK_FIELDS = new Set([
		"milestone", "epic", "pillars",
		"surpassed-by", "supersedes", "superseded-by",
		"depends-on", "blocks", "research-refs",
		"docs-required", "docs-produced",
		"assignee", "scope", "skills",
	]);

	/**
	 * CHIP_FIELDS: rendered as styled chips but NOT clickable links.
	 */
	const CHIP_FIELDS = new Set<string>(["layer", "model", "maturity", "recurrence", "category", "version", "horizon"]);

	/** CODE_FIELDS: rendered as monospace inline code badges (e.g. file paths). */
	const CODE_FIELDS = new Set<string>([]);

	/** BOOLEAN_FIELDS: rendered as check/x icons instead of "true"/"false" text. */
	const BOOLEAN_FIELDS = new Set<string>(["user-invocable"]);

	/** Classify a field key into its render type. */
	type FieldType = "date" | "file-list" | "link" | "code" | "chip" | "boolean" | "generic";

	function fieldType(key: string, value: unknown): FieldType {
		if (DATE_FIELDS.has(key)) return "date";
		if (FILE_LIST_FIELDS.has(key)) return "file-list";
		if (LINK_FIELDS.has(key)) return "link";
		if (CODE_FIELDS.has(key) && Array.isArray(value)) return "code";
		if (BOOLEAN_FIELDS.has(key)) return "boolean";
		if (CHIP_FIELDS.has(key)) return "chip";
		return "generic";
	}

	/** Humanize a kebab-case field key for display. */
	function humanizeKey(key: string): string {
		return key
			.replace(/-/g, " ")
			.replace(/_/g, " ")
			.replace(/\b\w/g, (c) => c.toUpperCase());
	}

	// --- Derived header values (always rendered first) ---
	const id = $derived(metadata["id"] as string | undefined);
	const title = $derived(metadata["title"] as string | undefined);
	const description = $derived(metadata["description"] as string | undefined);
	const status = $derived(metadata["status"] as string | undefined);
	const priority = $derived(
		isPresent(metadata["priority"]) ? String(metadata["priority"]) : undefined,
	);

	/** Scoring dimensions as key-value pairs for inline display. */
	const scoringEntries = $derived.by(() => {
		const raw = metadata["scoring"];
		if (raw === null || raw === undefined || typeof raw !== "object" || Array.isArray(raw)) return [];
		return Object.entries(raw as Record<string, unknown>).filter(
			([, v]) => v !== null && v !== undefined,
		);
	});

	/** Short date format for the header chip (e.g. "Jan 5"). */
	function shortDate(value: unknown): string | null {
		if (value === null || value === undefined || value === "" || value === "null") return null;
		try {
			const d = new Date(String(value));
			if (isNaN(d.getTime())) return null;
			return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
		} catch {
			return null;
		}
	}

	const createdShort = $derived(shortDate(metadata["created"]));
	const updatedShort = $derived(shortDate(metadata["updated"]));
	const dateChip = $derived(
		createdShort && updatedShort && createdShort !== updatedShort
			? `${createdShort} → ${updatedShort}`
			: createdShort ?? updatedShort,
	);

	/** Gate — supports both a single string (milestones) and an array (pillars). */
	const gateQuestions = $derived(
		isPresent(metadata["gate"]) ? asArray(metadata["gate"]).filter(Boolean) : [],
	);

	/** Capabilities (or legacy tools) with human-friendly names for display. */
	const appTools = $derived.by(() => {
		// Prefer capabilities field (current); fall back to tools (legacy)
		if (isPresent(metadata["capabilities"])) {
			return asArray(metadata["capabilities"]).map(getCapabilityLabel);
		}
		if (isPresent(metadata["tools"])) {
			return asArray(metadata["tools"])
				.filter((t) => !t.startsWith("mcp__"))
				.map(getCapabilityLabel);
		}
		return [];
	});

	/**
	 * Explicit field display order. Fields listed here are sorted to the
	 * front in the given order; unlisted fields appear after them in their
	 * original YAML source order.
	 */
	const FIELD_ORDER: string[] = [
		"layer", "maturity", "recurrence", "category", "version", "horizon",
		"milestone", "epic", "pillars", "depends-on", "blocks",
	];

	/**
	 * The ordered body entries from the metadata object, skipping:
	 * - Fixed header fields (SKIP_FIELDS)
	 * - Progress fields (rendered as a combined row)
	 * - Gate field (rendered separately at the end)
	 * - Entries without a present value
	 */
	const bodyEntries = $derived.by(() => {
		const filtered = Object.entries(metadata).filter(([key, value]) => {
			if (SKIP_FIELDS.has(key)) return false;
			if (key === "gate") return false;
			if (!isPresent(value)) return false;
			return true;
		});
		return filtered.sort(([a], [b]) => {
			const ai = FIELD_ORDER.indexOf(a);
			const bi = FIELD_ORDER.indexOf(b);
			// Both in order list: sort by position
			if (ai !== -1 && bi !== -1) return ai - bi;
			// Only one in order list: it comes first
			if (ai !== -1) return -1;
			if (bi !== -1) return 1;
			// Neither: preserve original order (stable sort)
			return 0;
		});
	});

	/** Relationships array from frontmatter (array of {type, target, rationale, intended?}). */
	const relationships = $derived(
		Array.isArray(metadata["relationships"]) ? metadata["relationships"] : [],
	);

	/** True when the card has content below the header row. */
	const hasBody = $derived(bodyEntries.length > 0 || appTools.length > 0 || gateQuestions.length > 0 || relationships.length > 0 || scoringEntries.length > 0);
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
	{#if id || (status && isPresent(status)) || priority || dateChip}
		<div class="flex justify-between gap-3" class:items-center={!hasBody} class:items-start={hasBody}>
			<div class="space-y-0.5">
				{#if id}
					<p class="font-mono text-xs font-semibold uppercase tracking-widest text-muted-foreground">
						{artifactType} · {id}
					</p>
				{/if}
			</div>

			<div class="flex shrink-0 items-center gap-2">
				{#if createdShort}
					<Badge variant="secondary" class="text-muted-foreground">
						<CalendarPlusIcon class="h-3 w-3" />{createdShort}
					</Badge>
				{/if}
				{#if updatedShort && updatedShort !== createdShort}
					<Badge variant="secondary" class="text-muted-foreground">
						<CalendarCheckIcon class="h-3 w-3" />{updatedShort}
					</Badge>
				{/if}
				{#if priority}
					<Badge variant="outline" class={priorityClass(priority)}>
						{priorityLabel(priority)}
					</Badge>
				{/if}
				{#if status && isPresent(status)}
					<StatusIndicator {status} mode="badge" />
				{/if}
			</div>
		</div>
	{/if}

	<!-- Scoring dimensions (shown near priority when present) -->
	{#if priority && scoringEntries.length > 0}
		<div class="flex items-baseline gap-2">
			<span class="w-[7rem] shrink-0 text-xs font-medium text-muted-foreground">
				<span class="inline-flex items-center gap-1">
					<ScaleIcon class="h-3 w-3" />Scoring
				</span>
			</span>
			<div class="flex min-w-0 flex-1 flex-wrap gap-1">
				{#each scoringEntries as [key, val] (key)}
					<Badge variant="secondary" class="font-normal">
						<span class="text-muted-foreground">{humanizeKey(key)}:</span> {String(val)}
					</Badge>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Dynamic body — YAML source order, type-dispatched -->
	{#each bodyEntries as [key, value] (key)}
		{@const type = fieldType(key, value)}

		{#if type === "date"}
			{@const formatted = formatDate(value)}
			{#if formatted}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<span class="text-xs text-foreground">{formatted}</span>
				</div>
			{/if}

		{:else if type === "file-list"}
			{@const items = asArray(value).filter(Boolean)}
			{#if items.length > 0}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each items as item, i (i)}
							<ArtifactLink path={item.trim()} />
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "link"}
			{@const vals = asArray(value).filter(Boolean)}
			{#if vals.length > 0}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each vals as val, i (i)}
							<ArtifactLink id={val.trim()} />
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "code"}
			{@const items = asArray(value).filter(Boolean)}
			{#if items.length > 0}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each items as item, i (i)}
							<code class="rounded bg-muted px-1.5 py-0.5 font-mono text-[11px] text-foreground">{item}</code>
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "chip"}
			{@const items = asArray(value).filter(Boolean)}
			{#if items.length > 0}
				<div class="flex items-baseline gap-2">
					<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
						{humanizeKey(key)}
					</span>
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each items as item, i (i)}
							<Badge variant="secondary" class="capitalize">{item}</Badge>
						{/each}
					</div>
				</div>
			{/if}

		{:else if type === "boolean"}
			<div class="flex items-center gap-2">
				<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
					{humanizeKey(key)}
				</span>
				{#if value}
					<CheckIcon class="h-4 w-4 text-green-500" />
				{:else}
					<XIcon class="h-4 w-4 text-muted-foreground" />
				{/if}
			</div>

		{:else}
			<!-- generic -->
			<div class="flex items-baseline gap-2">
				<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
					{humanizeKey(key)}
				</span>
				{#if Array.isArray(value)}
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each value as v, i (i)}
							<Badge variant="secondary" class="capitalize">{v}</Badge>
						{/each}
					</div>
				{:else if typeof value === "object" && value !== null}
					<div class="flex min-w-0 flex-1 flex-wrap gap-1">
						{#each Object.entries(value as Record<string, unknown>) as [k, v], i (i)}
							<Badge variant="secondary">
								<span class="text-muted-foreground">{humanizeKey(k)}:</span> {String(v)}
							</Badge>
						{/each}
					</div>
				{:else}
					<span class="min-w-0 flex-1 text-xs capitalize text-foreground">{String(value)}</span>
				{/if}
			</div>
		{/if}
	{/each}

	<!-- Capabilities (human-friendly names) -->
	{#if appTools.length > 0}
		<div class="flex items-baseline gap-2">
			<span class="w-[7rem] shrink-0 text-xs font-medium capitalize text-muted-foreground">
				Capabilities
			</span>
			<div class="flex min-w-0 flex-1 flex-wrap gap-1">
				{#each appTools as tool, i (i)}
					<Badge variant="secondary">{tool}</Badge>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Relationships -->
	<RelationshipsList {relationships} />

	<!-- Gate question(s) — always last -->
	<GateQuestions questions={gateQuestions} />
</div>
