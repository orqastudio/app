import type { DocNode, SortConfig, FilterableField } from "@orqastudio/types";

/** Filter nodes by frontmatter values. */
export function applyFilters(
	nodes: readonly DocNode[],
	filters: Readonly<Record<string, readonly string[]>>,
): readonly DocNode[] {
	// If no active filters, return all nodes unchanged
	const activeFilters = Object.entries(filters).filter(([, vals]) => vals.length > 0);
	if (activeFilters.length === 0) return nodes;

	return nodes.filter((node) => {
		// Skip directory nodes (no frontmatter)
		if (node.children !== null) return true;

		for (const [field, allowed] of activeFilters) {
			const raw = node.frontmatter?.[field];
			const value = raw !== null && raw !== undefined ? String(raw) : null;
			if (value === null || !allowed.includes(value)) return false;
		}
		return true;
	});
}

/** Sort nodes by a frontmatter field. */
export function applySort(nodes: readonly DocNode[], sort: Readonly<SortConfig>): DocNode[] {
	if (!sort.field || sort.field === "title") {
		// Sort by label
		const sorted = [...nodes].sort((a, b) => {
			const cmp = a.label.localeCompare(b.label, undefined, { sensitivity: "base" });
			return sort.direction === "desc" ? -cmp : cmp;
		});
		return sorted;
	}

	const { field, direction } = sort;

	return [...nodes].sort((a, b) => {
		const rawA = a.frontmatter?.[field];
		const rawB = b.frontmatter?.[field];

		// Nodes without the field go to the end regardless of direction
		const missingA = rawA === null || rawA === undefined;
		const missingB = rawB === null || rawB === undefined;
		if (missingA && missingB) return 0;
		if (missingA) return 1;
		if (missingB) return -1;

		const strA = String(rawA);
		const strB = String(rawB);

		// Try date comparison for likely date fields
		const dateA = Date.parse(strA);
		const dateB = Date.parse(strB);
		if (!isNaN(dateA) && !isNaN(dateB)) {
			const cmp = dateA - dateB;
			return direction === "desc" ? -cmp : cmp;
		}

		// Fallback: locale string comparison
		const cmp = strA.localeCompare(strB, undefined, { sensitivity: "base" });
		return direction === "desc" ? -cmp : cmp;
	});
}

/** Group nodes by a frontmatter field value. */
export function applyGrouping(
	nodes: readonly DocNode[],
	groupField: string,
	groupOrder: readonly string[] | undefined,
	filterableFields: readonly FilterableField[],
): { label: string; nodes: DocNode[] }[] {
	// Partition nodes into groups
	const groups = new Map<string, DocNode[]>();
	const otherNodes: DocNode[] = [];

	for (const node of nodes) {
		// Directory nodes: include in every group? No — put them in Other
		if (node.children !== null) {
			otherNodes.push(node);
			continue;
		}
		const raw = node.frontmatter?.[groupField];
		if (raw === null || raw === undefined) {
			otherNodes.push(node);
		} else {
			const value = String(raw);
			if (!groups.has(value)) groups.set(value, []);
			groups.get(value)!.push(node);
		}
	}

	// Build ordered keys
	let orderedKeys: string[];

	if (groupOrder && groupOrder.length > 0) {
		// 1. groupOrder from _navigation.json
		orderedKeys = [
			...groupOrder.filter((k) => groups.has(k)),
			...[...groups.keys()].filter((k) => !groupOrder.includes(k)).sort(),
		];
	} else {
		// 2. Schema enum order from matching FilterableField
		const field = filterableFields.find((f) => f.name === groupField);
		if (field && field.values.length > 0) {
			orderedKeys = [
				...field.values.filter((v) => groups.has(v)),
				...[...groups.keys()].filter((k) => !field.values.includes(k)).sort(),
			];
		} else {
			// 3. Alphabetical fallback
			orderedKeys = [...groups.keys()].sort();
		}
	}

	const result: { label: string; nodes: DocNode[] }[] = orderedKeys.map((key) => ({
		label: humanizeValue(key),
		nodes: groups.get(key) ?? [],
	}));

	if (otherNodes.length > 0) {
		result.push({ label: "Other", nodes: otherNodes });
	}

	return result.filter((g) => g.nodes.length > 0);
}

/** Count how many nodes match each value of a filterable field. */
export function countFieldValues(
	nodes: readonly DocNode[],
	fieldName: string,
): Record<string, number> {
	return nodes.reduce<Record<string, number>>((counts, node) => {
		if (node.children !== null) return counts;
		const raw = node.frontmatter?.[fieldName];
		if (raw !== null && raw !== undefined) {
			const value = String(raw);
			return { ...counts, [value]: (counts[value] ?? 0) + 1 };
		}
		return counts;
	}, {});
}

/** Humanize a field value for display (replace hyphens/underscores with spaces, title-case). */
function humanizeValue(value: string): string {
	return value
		.replace(/[-_]/g, " ")
		.replace(/\b\w/g, (c) => c.toUpperCase());
}
