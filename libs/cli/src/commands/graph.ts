/**
 * Graph browsing command — scan and query the artifact graph from CLI.
 *
 * orqa graph [options]
 *
 * Delegates to the orqa-validation daemon for all graph operations.
 * Falls back to the orqa-validation binary when the daemon is unreachable.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { parse as parseYaml } from "yaml";
import { scanArtifactGraph, queryGraph, getGraphStats } from "../lib/graph.js";
import type { GraphNode, GraphQueryOptions, GraphStats } from "../lib/graph.js";
import { getRoot } from "../lib/root.js";
import type { StateCategory } from "@orqastudio/types";

const USAGE = `
Usage: orqa graph [options]

Browse the artifact graph from the command line.

Options:
  --type <type>          Filter by artifact type (e.g. epic, task, decision)
  --status <status>      Filter by status (e.g. active, completed)
  --related-to <id>      Show artifacts related to the given ID
  --rel-type <type>      Filter by relationship type (e.g. delivers, informs)
  --search <query>       Text search in titles
  --limit <n>            Limit results (default: 50)
  --stats                Show aggregate statistics only
  --json                 Output as JSON
  --tree                 Show as delivery tree (hierarchy view)
  --id <id>              Show details for a specific artifact
  --help, -h             Show this help message
`.trim();

export async function runGraphCommand(args: string[]): Promise<void> {
	if (args.includes("--help") || args.includes("-h")) {
		console.log(USAGE);
		return;
	}

	const nodes = await scanArtifactGraph();

	if (nodes.length === 0) {
		console.log("No artifacts found. Is there a .orqa/ directory in the current project?");
		return;
	}

	// --stats mode
	if (args.includes("--stats")) {
		const stats = await getGraphStats(nodes);
		if (args.includes("--json")) {
			console.log(JSON.stringify(stats, null, 2));
		} else {
			printStats(stats);
		}
		return;
	}

	// --id mode: show single artifact details
	const idIdx = args.indexOf("--id");
	if (idIdx >= 0 && args[idIdx + 1]) {
		const id = args[idIdx + 1];
		const node = nodes.find((n) => n.id === id);
		if (!node) {
			console.error(`Artifact not found: ${id}`);
			process.exit(1);
		}
		if (args.includes("--json")) {
			console.log(JSON.stringify(node, null, 2));
		} else {
			printArtifactDetail(node, nodes);
		}
		return;
	}

	// Build query options from args
	const options: GraphQueryOptions = {};

	const typeIdx = args.indexOf("--type");
	if (typeIdx >= 0 && args[typeIdx + 1]) options.type = args[typeIdx + 1];

	const statusIdx = args.indexOf("--status");
	if (statusIdx >= 0 && args[statusIdx + 1]) options.status = args[statusIdx + 1];

	const relatedIdx = args.indexOf("--related-to");
	if (relatedIdx >= 0 && args[relatedIdx + 1]) options.relatedTo = args[relatedIdx + 1];

	const relTypeIdx = args.indexOf("--rel-type");
	if (relTypeIdx >= 0 && args[relTypeIdx + 1]) options.relationshipType = args[relTypeIdx + 1];

	const searchIdx = args.indexOf("--search");
	if (searchIdx >= 0 && args[searchIdx + 1]) options.search = args[searchIdx + 1];

	const limitIdx = args.indexOf("--limit");
	options.limit = limitIdx >= 0 ? parseInt(args[limitIdx + 1], 10) : 50;

	const results = await queryGraph(nodes, options);

	if (args.includes("--json")) {
		console.log(JSON.stringify(results, null, 2));
	} else if (args.includes("--tree")) {
		printTree(results);
	} else {
		printResults(results);
	}
}

function printStats(stats: GraphStats): void {
	console.log(`Artifact Graph Statistics\n`);
	console.log(`  Total artifacts: ${stats.totalNodes}`);
	console.log(`  Total relationships: ${stats.totalRelationships}\n`);

	console.log("  By type:");
	const sortedTypes = Object.entries(stats.byType).sort((a, b) => b[1] - a[1]);
	for (const [type, count] of sortedTypes) {
		console.log(`    ${type}: ${count}`);
	}

	console.log("\n  By status:");
	const sortedStatuses = Object.entries(stats.byStatus).sort((a, b) => b[1] - a[1]);
	for (const [status, count] of sortedStatuses) {
		console.log(`    ${status}: ${count}`);
	}
}

function printResults(nodes: GraphNode[]): void {
	if (nodes.length === 0) {
		console.log("No artifacts match the query.");
		return;
	}

	console.log(`Found ${nodes.length} artifact(s):\n`);

	// Group by type
	const byType = new Map<string, GraphNode[]>();
	for (const node of nodes) {
		const list = byType.get(node.type) ?? [];
		list.push(node);
		byType.set(node.type, list);
	}

	for (const [type, typeNodes] of byType) {
		console.log(`  ${type} (${typeNodes.length})`);
		for (const node of typeNodes) {
			const statusBadge = formatStatus(node.status);
			console.log(`    ${node.id}  ${statusBadge}  ${node.title}`);
		}
		console.log();
	}
}

function printArtifactDetail(node: GraphNode, allNodes: GraphNode[]): void {
	console.log(`\n${node.id}: ${node.title}`);
	console.log(`${"─".repeat(60)}`);
	console.log(`  Type:   ${node.type}`);
	console.log(`  Status: ${formatStatus(node.status)}`);
	console.log(`  Path:   ${node.path}`);

	if (node.relationships.length > 0) {
		console.log(`\n  Relationships (${node.relationships.length}):`);
		for (const rel of node.relationships) {
			const target = allNodes.find((n) => n.id === rel.target);
			const targetTitle = target ? ` — ${target.title}` : "";
			console.log(`    ${rel.type} → ${rel.target}${targetTitle}`);
		}
	}

	// Show reverse relationships (what points to this artifact)
	const incoming = allNodes.filter((n) =>
		n.relationships.some((r) => r.target === node.id),
	);
	if (incoming.length > 0) {
		console.log(`\n  Referenced by (${incoming.length}):`);
		for (const src of incoming) {
			const rels = src.relationships.filter((r) => r.target === node.id);
			for (const rel of rels) {
				console.log(`    ${src.id} (${rel.type}) — ${src.title}`);
			}
		}
	}

	console.log();
}

// ---------------------------------------------------------------------------
// Delivery hierarchy (loaded from plugin manifests, not hardcoded)
// ---------------------------------------------------------------------------

interface DeliveryLevel {
	type: string;
	parentType: string | null;
	parentRelationship: string | null;
}

/** Load delivery type hierarchy from installed plugin manifests. */
function loadDeliveryHierarchy(): DeliveryLevel[] {
	const root = getRoot();
	const levels: DeliveryLevel[] = [];

	for (const container of ["plugins", "connectors", "sidecars"]) {
		const containerDir = path.join(root, container);
		let entries: string[];
		try {
			entries = fs.readdirSync(containerDir, { encoding: "utf-8" }) as string[];
		} catch {
			continue;
		}

		for (const entry of entries) {
			const manifestPath = path.join(containerDir, entry, "orqa-plugin.json");
			try {
				const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf-8"));
				if (manifest?.delivery?.types && Array.isArray(manifest.delivery.types)) {
					for (const dt of manifest.delivery.types) {
						levels.push({
							type: dt.key,
							parentType: dt.parent?.type ?? null,
							parentRelationship: dt.parent?.relationship ?? null,
						});
					}
					return levels; // First plugin with delivery config wins
				}
			} catch {
				// Skip missing/invalid manifests
			}
		}
	}

	return levels;
}

function printTree(results: GraphNode[]): void {
	const hierarchy = loadDeliveryHierarchy();

	if (hierarchy.length === 0) {
		console.log("No delivery hierarchy configured. Use --type to filter.");
		printResults(results);
		return;
	}

	// Group results by delivery type
	const nodesByType = new Map<string, GraphNode[]>();
	for (const level of hierarchy) {
		nodesByType.set(level.type, results.filter((n) => n.type === level.type));
	}

	// Find the root level (no parent)
	const rootLevel = hierarchy.find((l) => l.parentType === null);
	if (!rootLevel) {
		console.log("No root delivery type found. Use --type to filter.");
		printResults(results);
		return;
	}

	const rootNodes = nodesByType.get(rootLevel.type) ?? [];
	if (rootNodes.length === 0) {
		// Try showing from the first level that has nodes
		const firstWithNodes = hierarchy.find((l) => (nodesByType.get(l.type) ?? []).length > 0);
		if (!firstWithNodes) {
			console.log("No delivery hierarchy found. Use --type to filter.");
			printResults(results);
			return;
		}
	}

	console.log("Delivery Tree:\n");

	/** Recursively print children at increasing indent depth. */
	function printLevel(parentNode: GraphNode, depth: number): void {
		// Find child levels whose parentType matches the current node's type
		const childLevels = hierarchy.filter((l) => l.parentType === parentNode.type);

		for (const childLevel of childLevels) {
			const childNodes = (nodesByType.get(childLevel.type) ?? []).filter((n) =>
				n.relationships.some(
					(r) => r.target === parentNode.id && r.type === childLevel.parentRelationship,
				),
			);

			for (const child of childNodes) {
				const indent = "  ".repeat(depth);
				console.log(`${indent}${formatStatus(child.status)} ${child.id}: ${child.title}`);
				printLevel(child, depth + 1);
			}
		}
	}

	for (const rootNode of rootNodes) {
		console.log(`${formatStatus(rootNode.status)} ${rootNode.id}: ${rootNode.title}`);
		printLevel(rootNode, 1);
		console.log();
	}

	// Show orphans: nodes at non-root levels that don't link to any parent
	for (const level of hierarchy) {
		if (level.parentType === null) continue;
		const parentNodes = nodesByType.get(level.parentType) ?? [];
		const nodes = nodesByType.get(level.type) ?? [];
		const orphans = nodes.filter(
			(n) => !n.relationships.some(
				(r) => r.type === level.parentRelationship && parentNodes.some((p) => p.id === r.target),
			),
		);
		if (orphans.length > 0) {
			console.log(`Unlinked ${level.type}s:`);
			for (const node of orphans) {
				console.log(`  ${formatStatus(node.status)} ${node.id}: ${node.title}`);
			}
		}
	}
}

// ---------------------------------------------------------------------------
// Category-based status icons (derived from resolved workflows, not hardcoded)
// ---------------------------------------------------------------------------

const CATEGORY_ICONS: Record<StateCategory, string> = {
	planning: "[.]",
	active: "[*]",
	review: "[?]",
	completed: "[+]",
	terminal: "[x]",
};

let statusCategoryCache: Map<string, StateCategory> | null = null;

/** Build a status-name → category map from resolved workflow YAML files. */
function loadStatusCategories(): Map<string, StateCategory> {
	if (statusCategoryCache) return statusCategoryCache;

	const map = new Map<string, StateCategory>();
	const root = getRoot();
	const workflowsDir = path.join(root, ".orqa", "workflows");

	let entries: string[];
	try {
		entries = fs.readdirSync(workflowsDir, { encoding: "utf-8" }) as string[];
	} catch {
		statusCategoryCache = map;
		return map;
	}

	for (const entry of entries) {
		if (!entry.endsWith(".resolved.yaml")) continue;
		try {
			const content = fs.readFileSync(path.join(workflowsDir, entry), "utf-8");
			const parsed = parseYaml(content);
			if (parsed?.states && typeof parsed.states === "object") {
				for (const [stateName, stateDef] of Object.entries(parsed.states)) {
					const sd = stateDef as Record<string, unknown>;
					if (typeof sd.category === "string") {
						map.set(stateName, sd.category as StateCategory);
					}
				}
			}
		} catch {
			// Skip unparseable files
		}
	}

	statusCategoryCache = map;
	return map;
}

function formatStatus(status: string): string {
	const categories = loadStatusCategories();
	const category = categories.get(status);
	if (category && CATEGORY_ICONS[category]) {
		return CATEGORY_ICONS[category];
	}
	return `[${status}]`;
}
