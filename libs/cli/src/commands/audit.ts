/**
 * Audit command — full governance audit.
 *
 * orqa audit [--fix]
 *   Runs: integrity validation (with optional --fix), version drift, license audit, readme audit.
 *   Exits non-zero if any check fails.
 *
 * orqa audit escalation [--json] [--create-tasks]
 *   Scans lessons for escalation candidates and creates CRITICAL task artifacts.
 *
 *   Detection rules:
 *     - Lesson recurrence >= 3 and status != promoted → [PROMOTE] finding
 *     - Lesson recurrence >= 3 and promoted-to rule exists:
 *         - If rule has no enforcement_updated → [STRENGTHEN] finding
 *         - If rule has enforcement_updated and lesson recurrence >= 3 post-update → [STRENGTHEN] finding
 *
 *   --create-tasks always creates artifacts for every finding found.
 *   In the stop hook, --create-tasks is always passed so tasks are created immediately.
 */

import { execSync } from "node:child_process";
import { readFileSync, writeFileSync, readdirSync, existsSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";
import { getRoot } from "../lib/root.js";
import { parseFrontmatterFromFile } from "../lib/frontmatter.js";

// ─── Workflow/schema helpers (read from resolved YAML, not hardcoded) ─────────

/** Read the initial_state for an artifact type from its resolved workflow. */
function getInitialStatus(projectDir: string, artifactType: string): string | null {
	const workflowsDir = join(projectDir, ".orqa", "workflows");
	// Try exact match first, then scan all for matching artifact_type
	for (const name of [`${artifactType}.resolved.yaml`]) {
		const filePath = join(workflowsDir, name);
		if (existsSync(filePath)) {
			try {
				const parsed = parseYaml(readFileSync(filePath, "utf-8"));
				if (parsed?.initial_state) return parsed.initial_state;
			} catch { /* skip */ }
		}
	}
	// Scan all resolved workflows for matching artifact_type
	try {
		const entries = readdirSync(workflowsDir, { encoding: "utf-8" }) as string[];
		for (const entry of entries) {
			if (!entry.endsWith(".resolved.yaml")) continue;
			try {
				const parsed = parseYaml(readFileSync(join(workflowsDir, entry), "utf-8"));
				if (parsed?.artifact_type === artifactType && parsed?.initial_state) {
					return parsed.initial_state;
				}
			} catch { /* skip */ }
		}
	} catch { /* skip */ }
	return null;
}

/** Get all status names that have the "active" category for a given artifact type. */
function getActiveStatuses(projectDir: string, artifactType: string): Set<string> {
	const result = new Set<string>();
	const workflowsDir = join(projectDir, ".orqa", "workflows");
	try {
		const entries = readdirSync(workflowsDir, { encoding: "utf-8" }) as string[];
		for (const entry of entries) {
			if (!entry.endsWith(".resolved.yaml")) continue;
			try {
				const parsed = parseYaml(readFileSync(join(workflowsDir, entry), "utf-8"));
				if (parsed?.artifact_type !== artifactType) continue;
				if (parsed?.states && typeof parsed.states === "object") {
					for (const [stateName, stateDef] of Object.entries(parsed.states)) {
						const sd = stateDef as Record<string, unknown>;
						if (sd.category === "active") {
							result.add(stateName);
						}
					}
				}
			} catch { /* skip */ }
		}
	} catch { /* skip */ }
	return result;
}

/** Read the relationship type that connects tasks to epics from the delivery config. */
function getTaskToEpicRelationship(projectDir: string): string | null {
	for (const container of ["plugins", "connectors", "integrations"]) {
		const containerDir = join(projectDir, container);
		let entries: string[];
		try {
			entries = readdirSync(containerDir, { encoding: "utf-8" }) as string[];
		} catch {
			continue;
		}
		for (const entry of entries) {
			const manifestPath = join(containerDir, entry, "orqa-plugin.json");
			try {
				const manifest = JSON.parse(readFileSync(manifestPath, "utf-8"));
				if (manifest?.delivery?.types && Array.isArray(manifest.delivery.types)) {
					const taskType = manifest.delivery.types.find(
						(dt: Record<string, unknown>) => dt.key === "task",
					);
					if (taskType?.parent?.relationship) {
						return taskType.parent.relationship;
					}
				}
			} catch { /* skip */ }
		}
	}
	return null;
}

// ─── Full audit ───────────────────────────────────────────────────────────────

export async function runAuditCommand(args: string[] = []): Promise<void> {
	const subcommand = args[0];

	if (subcommand === "escalation") {
		await runEscalationCheck(getRoot(), args.slice(1));
		return;
	}

	const root = getRoot();
	let failed = false;

	const fix = args.includes("--fix") ? " --fix" : "";

	const checks = [
		{ name: "integrity", cmd: `orqa enforce .${fix}` },
		{ name: "version", cmd: "orqa version check" },
		{ name: "license", cmd: "orqa repo license" },
		{ name: "readme", cmd: "orqa repo readme" },
	];

	for (const check of checks) {
		try {
			execSync(check.cmd, { cwd: root, stdio: "inherit" });
		} catch {
			failed = true;
		}
	}

	if (failed) {
		process.exit(1);
	}
}

// ─── Types ────────────────────────────────────────────────────────────────────

interface ArtifactRelationship {
	target: string;
	type: string;
	rationale?: string;
}

interface LessonFrontmatter {
	id: string;
	title: string;
	status: string;
	recurrence: number;
	relationships: ArtifactRelationship[];
}

interface RuleFrontmatter {
	id: string;
	title: string;
	enforcement_updated?: string;
	relationships: ArtifactRelationship[];
}

interface EscalationFinding {
	lessonId: string;
	lessonTitle: string;
	recurrence: number;
	lessonStatus: string;
	reason: "promote" | "strengthen";
	/** Rule ID associated with this finding, if applicable */
	ruleId?: string;
	description: string;
}

// ─── Frontmatter helpers ─────────────────────────────────────────────────────

function extractRelationships(fm: Record<string, unknown>): ArtifactRelationship[] {
	const raw = fm.relationships;
	if (!Array.isArray(raw)) return [];
	return (raw as Array<Record<string, unknown>>).map((r) => ({
		target: typeof r.target === "string" ? r.target : "",
		type: typeof r.type === "string" ? r.type : "",
		rationale: typeof r.rationale === "string" ? r.rationale : undefined,
	}));
}

/** Cached initial status for lessons, read from the resolved workflow. */
let lessonInitialStatus: string | undefined;

function parseLessonFrontmatter(filePath: string): LessonFrontmatter | null {
	const fm = parseFrontmatterFromFile(filePath);
	if (!fm) return null;
	if (lessonInitialStatus === undefined) {
		lessonInitialStatus = getInitialStatus(getRoot(), "lesson") ?? "";
	}
	return {
		id: typeof fm.id === "string" ? fm.id : "",
		title: typeof fm.title === "string" ? fm.title : "",
		status: typeof fm.status === "string" ? fm.status : lessonInitialStatus,
		recurrence: typeof fm.recurrence === "number" ? fm.recurrence : 0,
		relationships: extractRelationships(fm),
	};
}

function parseRuleFrontmatter(filePath: string): RuleFrontmatter | null {
	const fm = parseFrontmatterFromFile(filePath);
	if (!fm) return null;
	return {
		id: typeof fm.id === "string" ? fm.id : "",
		title: typeof fm.title === "string" ? fm.title : "",
		enforcement_updated: typeof fm.enforcement_updated === "string" ? fm.enforcement_updated : undefined,
		relationships: extractRelationships(fm),
	};
}

// ─── Rule index ───────────────────────────────────────────────────────────────

function buildRuleIndex(rulesDir: string): Map<string, RuleFrontmatter> {
	const index = new Map<string, RuleFrontmatter>();
	let entries: string[];
	try {
		entries = readdirSync(rulesDir, { encoding: "utf-8" }) as string[];
	} catch {
		return index;
	}
	for (const entry of entries) {
		if (!entry.endsWith(".md")) continue;
		const rule = parseRuleFrontmatter(join(rulesDir, entry));
		if (rule?.id) index.set(rule.id, rule);
	}
	return index;
}

// ─── Escalation scan ──────────────────────────────────────────────────────────

function scanForEscalations(projectDir: string): EscalationFinding[] {
	const findings: EscalationFinding[] = [];

	const lessonsDir = join(projectDir, ".orqa", "learning", "lessons");
	const rulesDir = join(projectDir, ".orqa", "learning", "rules");

	const ruleIndex = buildRuleIndex(rulesDir);

	let lessonEntries: string[];
	try {
		lessonEntries = readdirSync(lessonsDir, { encoding: "utf-8" }) as string[];
	} catch {
		return findings;
	}

	for (const entry of lessonEntries) {
		if (!entry.endsWith(".md")) continue;
		const lesson = parseLessonFrontmatter(join(lessonsDir, entry));
		if (!lesson?.id) continue;

		const recurrence = lesson.recurrence;
		if (recurrence < 3) continue;

		// Find promoted-to rule (if any)
		const promotedToRel = lesson.relationships.find((r) => r.type === "promoted-to");
		const associatedRule = promotedToRel ? ruleIndex.get(promotedToRel.target) : undefined;

		if (lesson.status !== "promoted") {
			// Not yet promoted — flag for promotion
			findings.push({
				lessonId: lesson.id,
				lessonTitle: lesson.title,
				recurrence,
				lessonStatus: lesson.status,
				reason: "promote",
				description: `Lesson ${lesson.id} has recurrence ${recurrence} but status is "${lesson.status}" — needs promoting to a rule`,
			});
		} else if (associatedRule) {
			// Promoted — check if the rule's enforcement needs strengthening.
			// If enforcement_updated is set, recurrence was reset at that point.
			// A recurrence >= 3 post-update means the enforcement isn't working.
			// If enforcement_updated is not set, the rule has never had enforcement strengthened.
			const needsStrengthening = !associatedRule.enforcement_updated || recurrence >= 3;

			if (needsStrengthening) {
				findings.push({
					lessonId: lesson.id,
					lessonTitle: lesson.title,
					recurrence,
					lessonStatus: lesson.status,
					reason: "strengthen",
					ruleId: associatedRule.id,
					description: associatedRule.enforcement_updated
						? `Lesson ${lesson.id} has recurrence ${recurrence} after enforcement was updated on ${associatedRule.enforcement_updated} — enforcement on ${associatedRule.id} is not working`
						: `Lesson ${lesson.id} has recurrence ${recurrence} but ${associatedRule.id} has no enforcement_updated date — enforcement has never been strengthened`,
				});
			}
		} else {
			// Promoted but no associated rule found via promoted-to relationship
			findings.push({
				lessonId: lesson.id,
				lessonTitle: lesson.title,
				recurrence,
				lessonStatus: lesson.status,
				reason: "strengthen",
				description: `Lesson ${lesson.id} has recurrence ${recurrence} and status "promoted" but no associated rule found — check promoted-to relationship`,
			});
		}
	}

	return findings;
}

// ─── Active epic resolution ───────────────────────────────────────────────────

function findActiveEpic(projectDir: string): string | null {
	// Try session state first
	const sessionFile = join(projectDir, ".state", "session-state.md");
	if (existsSync(sessionFile)) {
		const content = readFileSync(sessionFile, "utf-8");
		const match = /EPIC-[a-f0-9]{8}/.exec(content);
		if (match) return match[0];
	}

	// Scan epics directory for first epic in an "active" category state
	const epicsDir = join(projectDir, ".orqa", "delivery", "epics");
	if (!existsSync(epicsDir)) return null;

	const activeStatuses = getActiveStatuses(projectDir, "epic");

	let entries: string[];
	try {
		entries = readdirSync(epicsDir, { encoding: "utf-8" }) as string[];
	} catch {
		return null;
	}

	for (const entry of entries) {
		if (!entry.endsWith(".md")) continue;
		const fm = parseFrontmatterFromFile(join(epicsDir, entry));
		if (typeof fm?.status === "string" && activeStatuses.has(fm.status) && typeof fm.id === "string") {
			return fm.id;
		}
	}

	return null;
}

// ─── Task artifact creation ───────────────────────────────────────────────────

function generateIdFromTitle(prefix: string, title: string): string {
	const hex = createHash("md5").update(title).digest("hex").substring(0, 8);
	return `${prefix.toUpperCase()}-${hex}`;
}

function nextTaskFilename(tasksDir: string): string {
	let entries: string[];
	try {
		entries = readdirSync(tasksDir, { encoding: "utf-8" }) as string[];
	} catch {
		entries = [];
	}
	const numbers = entries
		.filter((e) => /^TASK-\d+\.md$/.test(e))
		.map((e) => parseInt(e.replace("TASK-", "").replace(".md", ""), 10))
		.filter((n) => !isNaN(n));
	const next = numbers.length > 0 ? Math.max(...numbers) + 1 : 1;
	return `TASK-${String(next).padStart(3, "0")}.md`;
}

function createTaskArtifact(projectDir: string, finding: EscalationFinding, epicId: string | null): string {
	const today = new Date().toISOString().slice(0, 10);

	const titleVerb = finding.reason === "promote" ? "Promote" : "Strengthen enforcement for";
	const titleTarget = finding.ruleId ? `${finding.ruleId} (from ${finding.lessonId})` : `lesson ${finding.lessonId}`;
	const title = `ESCALATION: ${titleVerb} ${titleTarget} (recurrence ${finding.recurrence})`;
	const taskId = generateIdFromTitle("TASK", title);

	const relationships: ArtifactRelationship[] = [
		{
			target: finding.lessonId,
			type: "addresses",
			rationale: `Escalation task for lesson with recurrence ${finding.recurrence}`,
		},
	];

	if (finding.ruleId) {
		relationships.push({
			target: finding.ruleId,
			type: "addresses",
			rationale: "Enforcement strengthening needed on this rule",
		});
	}

	if (epicId) {
		const taskToEpicRel = getTaskToEpicRelationship(projectDir);
		if (taskToEpicRel) {
			relationships.push({
				target: epicId,
				type: taskToEpicRel,
				rationale: "Escalation task linked to active epic",
			});
		}
	}

	const whyText =
		finding.reason === "promote"
			? "The lesson must be promoted to a rule so it is mechanically enforced. Recurrence >= 3 means this pattern is established and will continue without a rule."
			: "The rule exists but enforcement is insufficient — recurrence continues post-promotion. Strengthening enforcement means adding mechanical checks (lint rules, hooks, or gates) that catch violations before they reach production.";

	const acceptanceLine =
		finding.reason === "promote"
			? `Rule created and linked to lesson ${finding.lessonId} via promoted-to relationship`
			: `enforcement_updated date added to ${finding.ruleId ?? "associated rule"} and lesson recurrence reset to 0`;

	const fm: Record<string, unknown> = {
		id: taskId,
		title,
		description: finding.description,
		status: "captured",
		priority: "critical",
		created: today,
		updated: today,
		relationships,
	};

	const yamlText = stringifyYaml(fm, { lineWidth: 0 }).trimEnd();
	const body =
		`\n## What\n\n${finding.description}\n\n` +
		`## Why\n\n${whyText}\n\n` +
		`## Acceptance\n\n` +
		`- [ ] ${acceptanceLine}\n` +
		`- [ ] Recurrence does not increase in the next session\n`;

	const content = `---\n${yamlText}\n---${body}`;

	const tasksDir = join(projectDir, ".orqa", "delivery", "tasks");
	mkdirSync(tasksDir, { recursive: true });

	const filePath = join(tasksDir, nextTaskFilename(tasksDir));
	writeFileSync(filePath, content);
	return filePath;
}

// ─── Escalation command ───────────────────────────────────────────────────────

async function runEscalationCheck(projectDir: string, args: string[]): Promise<void> {
	const asJson = args.includes("--json");
	const createTasks = args.includes("--create-tasks");

	const findings = scanForEscalations(projectDir);

	if (asJson) {
		const created: string[] = [];
		if (createTasks && findings.length > 0) {
			const epicId = findActiveEpic(projectDir);
			for (const finding of findings) {
				created.push(createTaskArtifact(projectDir, finding, epicId));
			}
		}
		console.log(JSON.stringify({ findings, tasks_created: created.length }, null, 2));
		if (findings.length > 0) process.exit(1);
		return;
	}

	// Human-readable output
	if (findings.length === 0) {
		console.log("No escalation candidates found.");
		return;
	}

	console.log(`Found ${findings.length} escalation candidate(s):\n`);

	for (const finding of findings) {
		const tag = finding.reason === "promote" ? "[PROMOTE]" : "[STRENGTHEN]";
		const ruleNote = finding.ruleId ? ` → ${finding.ruleId}` : "";
		console.log(`  ${tag} ${finding.lessonId}${ruleNote} — recurrence ${finding.recurrence} (status: ${finding.lessonStatus})`);
		console.log(`    ${finding.lessonTitle}`);
		console.log(`    ${finding.description}`);
		console.log();
	}

	if (createTasks) {
		const epicId = findActiveEpic(projectDir);
		console.log(epicId ? `Active epic: ${epicId}` : "No active epic found — tasks created without epic link.");
		console.log();

		for (const finding of findings) {
			const filePath = createTaskArtifact(projectDir, finding, epicId);
			const rel = filePath.replace(projectDir + "/", "").replace(projectDir + "\\", "");
			console.log(`  Created: ${rel}`);
		}
		console.log();
	} else {
		console.log("Run with --create-tasks to auto-create CRITICAL task artifacts.");
	}

	process.exit(1);
}
