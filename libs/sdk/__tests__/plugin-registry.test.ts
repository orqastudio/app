/**
 * Tests for PluginRegistry — schema registration, relationship resolution,
 * conflict detection, alias management, and viewer routing.
 *
 * PluginRegistry has no IPC or Tauri dependencies, so these tests run
 * against the class directly without any mocking.
 */
import { describe, it, expect, beforeEach } from "vitest";
import { PluginRegistry } from "../src/plugins/plugin-registry.svelte.js";
import type { PluginManifest, RelationshipType, ArtifactSchema } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

function makeSchema(overrides: Partial<ArtifactSchema> = {}): ArtifactSchema {
	return {
		key: "task",
		label: "Task",
		icon: "check-square",
		defaultPath: ".orqa/implementation/tasks",
		idPrefix: "TASK",
		frontmatter: { type: "object" },
		statusTransitions: { todo: ["in_progress"], in_progress: ["done"], done: [] },
		...overrides,
	};
}

function makeRelationship(overrides: Partial<RelationshipType> = {}): RelationshipType {
	return {
		key: "delivers",
		inverse: "delivered-by",
		label: "delivers",
		inverseLabel: "delivered by",
		from: ["task"],
		to: ["epic"],
		description: "Task delivers an Epic",
		semantic: "lineage",
		...overrides,
	};
}

function makeManifest(overrides: Partial<PluginManifest> = {}): PluginManifest {
	return {
		name: "test-plugin",
		version: "0.1.0",
		provides: {
			schemas: [],
			relationships: [],
			views: [],
			widgets: [],
		},
		...overrides,
	} as PluginManifest;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("PluginRegistry", () => {
	let registry: PluginRegistry;

	beforeEach(() => {
		registry = new PluginRegistry();
	});

	describe("initial state", () => {
		it("starts with no plugins", () => {
			expect(registry.plugins.size).toBe(0);
		});

		it("allSchemas is empty initially", () => {
			expect(registry.allSchemas).toEqual([]);
		});

		it("allRelationships is empty initially", () => {
			expect(registry.allRelationships).toEqual([]);
		});

		it("pluginNames is empty initially", () => {
			expect(registry.pluginNames).toEqual([]);
		});
	});

	describe("registerPlatformRelationships", () => {
		it("registers platform relationships and their inverses", () => {
			const rel = makeRelationship({ key: "enforces", inverse: "enforced-by", semantic: "governance" });
			registry.registerPlatformRelationships([rel]);

			const forward = registry.getRelationship("enforces");
			const inverse = registry.getRelationship("enforced-by");

			expect(forward).not.toBeNull();
			expect(forward?.key).toBe("enforces");
			expect(inverse).not.toBeNull();
			expect(inverse?.key).toBe("enforced-by");
			expect(inverse?.inverse).toBe("enforces");
		});

		it("inverse relationship has swapped from/to", () => {
			const rel = makeRelationship({
				key: "delivers",
				inverse: "delivered-by",
				from: ["task"],
				to: ["epic"],
			});
			registry.registerPlatformRelationships([rel]);

			const inverse = registry.getRelationship("delivered-by");
			expect(inverse?.from).toEqual(["epic"]);
			expect(inverse?.to).toEqual(["task"]);
		});

		it("platform relationships are owned by 'platform'", () => {
			const rel = makeRelationship();
			registry.registerPlatformRelationships([rel]);

			expect(registry.getRelationshipOwner("delivers")).toBe("platform");
			expect(registry.getRelationshipOwner("delivered-by")).toBe("platform");
		});
	});

	describe("register", () => {
		it("registers a plugin with schemas and relationships", () => {
			const schema = makeSchema({ key: "epic", label: "Epic", defaultPath: ".orqa/planning/epics" });
			const rel = makeRelationship({ key: "contains", inverse: "contained-by" });
			const manifest = makeManifest({
				name: "planning-plugin",
				provides: {
					schemas: [schema],
					relationships: [rel],
					views: [],
					widgets: [],
				},
			});

			registry.register(manifest, {});

			expect(registry.plugins.has("planning-plugin")).toBe(true);
			expect(registry.allSchemas).toHaveLength(1);
			expect(registry.allSchemas[0].key).toBe("epic");
			expect(registry.getRelationship("contains")).not.toBeNull();
		});

		it("makes the plugin accessible by name", () => {
			const manifest = makeManifest({ name: "my-plugin" });
			registry.register(manifest, {});

			expect(registry.getPlugin("my-plugin")).not.toBeNull();
			expect(registry.isPluginActive("my-plugin")).toBe(true);
			expect(registry.isPluginActive("other-plugin")).toBe(false);
		});

		it("throws when a required dependency is not loaded", () => {
			const manifest = makeManifest({
				name: "child-plugin",
				requires: ["parent-plugin"],
			});

			expect(() => registry.register(manifest, {})).toThrow("parent-plugin");
		});

		it("succeeds when all required dependencies are loaded", () => {
			const parent = makeManifest({ name: "parent-plugin" });
			const child = makeManifest({ name: "child-plugin", requires: ["parent-plugin"] });

			registry.register(parent, {});
			expect(() => registry.register(child, {})).not.toThrow();
		});

		it("throws on duplicate schema key from different plugin", () => {
			const schema = makeSchema({ key: "task" });
			const plugin1 = makeManifest({ name: "plugin-a", provides: { schemas: [schema], relationships: [], views: [], widgets: [] } });
			const plugin2 = makeManifest({ name: "plugin-b", provides: { schemas: [schema], relationships: [], views: [], widgets: [] } });

			registry.register(plugin1, {});
			expect(() => registry.register(plugin2, {})).toThrow("schema key");
		});

		it("adds plugin name to pluginNames list", () => {
			registry.register(makeManifest({ name: "plugin-alpha" }), {});
			registry.register(makeManifest({ name: "plugin-beta" }), {});

			expect(registry.pluginNames).toContain("plugin-alpha");
			expect(registry.pluginNames).toContain("plugin-beta");
		});
	});

	describe("unregister", () => {
		it("removes plugin and its schema ownership", () => {
			const schema = makeSchema({ key: "task" });
			const manifest = makeManifest({
				name: "my-plugin",
				provides: { schemas: [schema], relationships: [], views: [], widgets: [] },
			});

			registry.register(manifest, {});
			expect(registry.getSchema("task")).not.toBeNull();

			registry.unregister("my-plugin");
			expect(registry.plugins.has("my-plugin")).toBe(false);
			expect(registry.getSchema("task")).toBeNull();
		});

		it("removes relationship definitions on unregister", () => {
			const rel = makeRelationship({ key: "links-to", inverse: "linked-by" });
			const manifest = makeManifest({
				name: "my-plugin",
				provides: { schemas: [], relationships: [rel], views: [], widgets: [] },
			});

			registry.register(manifest, {});
			expect(registry.getRelationship("links-to")).not.toBeNull();

			registry.unregister("my-plugin");
			expect(registry.getRelationship("links-to")).toBeNull();
			expect(registry.getRelationship("linked-by")).toBeNull();
		});

		it("is a no-op for unregistered plugin names", () => {
			expect(() => registry.unregister("nonexistent-plugin")).not.toThrow();
		});
	});

	describe("getSchema", () => {
		it("returns the schema for a registered type key", () => {
			const schema = makeSchema({ key: "milestone", label: "Milestone" });
			registry.register(makeManifest({
				name: "planning",
				provides: { schemas: [schema], relationships: [], views: [], widgets: [] },
			}), {});

			const result = registry.getSchema("milestone");
			expect(result).not.toBeNull();
			expect(result?.label).toBe("Milestone");
		});

		it("returns null for unknown type key", () => {
			expect(registry.getSchema("unknown-type")).toBeNull();
		});
	});

	describe("allSchemas", () => {
		it("aggregates schemas across all plugins", () => {
			const schemaA = makeSchema({ key: "epic", label: "Epic", defaultPath: ".orqa/planning/epics" });
			const schemaB = makeSchema({ key: "task", label: "Task", defaultPath: ".orqa/implementation/tasks" });

			registry.register(makeManifest({ name: "plugin-a", provides: { schemas: [schemaA], relationships: [], views: [], widgets: [] } }), {});
			registry.register(makeManifest({ name: "plugin-b", provides: { schemas: [schemaB], relationships: [], views: [], widgets: [] } }), {});

			const all = registry.allSchemas;
			expect(all).toHaveLength(2);
			expect(all.map((s) => s.key)).toContain("epic");
			expect(all.map((s) => s.key)).toContain("task");
		});
	});

	describe("allRelationships", () => {
		it("returns forward keys without duplicates (no inverse-only entries)", () => {
			const rel = makeRelationship({ key: "observes", inverse: "observed-by" });
			registry.register(makeManifest({
				name: "discovery",
				provides: { schemas: [], relationships: [rel], views: [], widgets: [] },
			}), {});

			const all = registry.allRelationships;
			// Should have exactly one entry (the forward definition), not two
			expect(all).toHaveLength(1);
			expect(all[0].key).toBe("observes");
		});

		it("includes platform relationships alongside plugin relationships", () => {
			const platformRel = makeRelationship({ key: "enforces", inverse: "enforced-by", semantic: "governance" });
			registry.registerPlatformRelationships([platformRel]);

			const pluginRel = makeRelationship({ key: "observes", inverse: "observed-by", semantic: "observation" });
			registry.register(makeManifest({
				name: "learning",
				provides: { schemas: [], relationships: [pluginRel], views: [], widgets: [] },
			}), {});

			const all = registry.allRelationships;
			expect(all).toHaveLength(2);
		});
	});

	describe("validateRelationship", () => {
		beforeEach(() => {
			const rel = makeRelationship({
				key: "delivers",
				inverse: "delivered-by",
				from: ["task"],
				to: ["epic"],
			});
			registry.register(makeManifest({
				name: "delivery-plugin",
				provides: { schemas: [], relationships: [rel], views: [], widgets: [] },
			}), {});
		});

		it("returns null when the relationship is valid", () => {
			expect(registry.validateRelationship("delivers", "task", "epic")).toBeNull();
		});

		it("rejects unknown relationship key", () => {
			const err = registry.validateRelationship("unknown-rel", "task", "epic");
			expect(err).toContain("unknown relationship key");
		});

		it("rejects wrong from type", () => {
			const err = registry.validateRelationship("delivers", "epic", "epic");
			expect(err).toContain("cannot originate from");
		});

		it("rejects wrong to type", () => {
			const err = registry.validateRelationship("delivers", "task", "rule");
			expect(err).toContain("cannot target type");
		});

		it("allows any from type when from array is empty", () => {
			const openRel = makeRelationship({ key: "relates-to", inverse: "related-by", from: [], to: [] });
			registry.register(makeManifest({
				name: "generic-plugin",
				provides: { schemas: [], relationships: [openRel], views: [], widgets: [] },
			}), {});

			expect(registry.validateRelationship("relates-to", "any-type", "another-type")).toBeNull();
		});
	});

	describe("alias resolution", () => {
		it("resolveKey returns canonical key for alias", () => {
			registry.loadPluginConfigs({
				"my-plugin": {
					installed: true,
					enabled: true,
					path: "plugins/my-plugin",
					schemaAliases: {
						"task": { alias: "ticket", label: "Ticket" },
					},
				},
			});

			expect(registry.resolveKey("ticket")).toBe("task");
		});

		it("resolveKey returns the key itself when no alias exists", () => {
			expect(registry.resolveKey("task")).toBe("task");
		});

		it("getAlias returns alias for canonical key", () => {
			registry.loadPluginConfigs({
				"my-plugin": {
					installed: true,
					enabled: true,
					path: "plugins/my-plugin",
					schemaAliases: {
						"task": { alias: "ticket", label: "Ticket" },
					},
				},
			});

			expect(registry.getAlias("task")).toBe("ticket");
		});

		it("getAlias returns the key itself when no alias set", () => {
			expect(registry.getAlias("epic")).toBe("epic");
		});

		it("setAlias updates both directions", () => {
			registry.setAlias("my-plugin", "schema", "epic", "initiative");

			expect(registry.resolveKey("initiative")).toBe("epic");
			expect(registry.getAlias("epic")).toBe("initiative");
		});
	});

	describe("allPluginConfigs", () => {
		it("returns configs loaded via loadPluginConfigs", () => {
			registry.loadPluginConfigs({
				"plugin-a": { installed: true, enabled: true, path: "plugins/plugin-a" },
			});

			const configs = registry.allPluginConfigs;
			expect(configs["plugin-a"]).toBeDefined();
			expect(configs["plugin-a"].enabled).toBe(true);
		});
	});

	describe("getViewerForArtifactType", () => {
		it("returns the view key when a plugin registers a custom viewer", () => {
			const manifest = makeManifest({
				name: "kanban-plugin",
				provides: {
					schemas: [],
					relationships: [],
					views: [],
					widgets: [],
					artifact_viewers: [{ artifact_type: "task", view_key: "kanban-board" }],
				},
			});
			registry.register(manifest, {});

			expect(registry.getViewerForArtifactType("task")).toBe("kanban-board");
		});

		it("returns null when no plugin claims the artifact type", () => {
			registry.register(makeManifest({ name: "plain-plugin" }), {});
			expect(registry.getViewerForArtifactType("task")).toBeNull();
		});
	});

	describe("getIconForType", () => {
		it("returns the icon from the schema", () => {
			const schema = makeSchema({ key: "rule", icon: "shield", defaultPath: ".orqa/learning/rules" });
			registry.register(makeManifest({
				name: "learning",
				provides: { schemas: [schema], relationships: [], views: [], widgets: [] },
			}), {});

			expect(registry.getIconForType("rule")).toBe("shield");
		});

		it("returns 'file-text' fallback for unknown type", () => {
			expect(registry.getIconForType("mystery-type")).toBe("file-text");
		});
	});

	describe("getSchemaCategories", () => {
		it("returns categories from the schema", () => {
			const schema = makeSchema({
				key: "lesson",
				defaultPath: ".orqa/learning/lessons",
				categories: [
					{ key: "process", label: "Process", color: "#3b82f6" },
					{ key: "technical", label: "Technical", color: "#10b981" },
				],
			});
			registry.register(makeManifest({
				name: "learning",
				provides: { schemas: [schema], relationships: [], views: [], widgets: [] },
			}), {});

			const cats = registry.getSchemaCategories("lesson");
			expect(cats).toHaveLength(2);
			expect(cats[0].key).toBe("process");
		});

		it("returns empty array for unknown schema key", () => {
			expect(registry.getSchemaCategories("unknown")).toEqual([]);
		});
	});

	describe("governanceSchemas", () => {
		it("returns only schemas with semantic governance", () => {
			const govSchema = makeSchema({ key: "rule", defaultPath: ".orqa/learning/rules", semantic: "governance" });
			const plainSchema = makeSchema({ key: "task", defaultPath: ".orqa/implementation/tasks" });

			registry.register(makeManifest({
				name: "mixed-plugin",
				provides: { schemas: [govSchema, plainSchema], relationships: [], views: [], widgets: [] },
			}), {});

			const gov = registry.governanceSchemas;
			expect(gov).toHaveLength(1);
			expect(gov[0].key).toBe("rule");
		});
	});

	describe("getPipelineStages", () => {
		it("returns pipeline stages from workflow registration", () => {
			const manifest = makeManifest({
				name: "lesson-plugin",
				provides: {
					schemas: [],
					relationships: [],
					views: [],
					widgets: [],
					workflows: [
						{
							artifact_type: "lesson",
							pipeline_stages: [
								{ key: "captured", label: "Captured", color: "#3b82f6" },
								{ key: "validated", label: "Validated", color: "#10b981" },
							],
						},
					],
				},
			});
			registry.register(manifest, {});

			const stages = registry.getPipelineStages("lesson");
			expect(stages).toHaveLength(2);
			expect(stages[0].key).toBe("captured");
		});

		it("returns empty array when no workflow is registered for the type", () => {
			expect(registry.getPipelineStages("unknown-type")).toEqual([]);
		});
	});

	describe("resolveNavigationItem", () => {
		it("resolves label and icon from plugin view registration", () => {
			const manifest = makeManifest({
				name: "roadmap-plugin",
				provides: {
					schemas: [],
					relationships: [],
					views: [{ key: "roadmap", label: "Roadmap", icon: "map" }],
					widgets: [],
				},
			});
			registry.register(manifest, {});

			// icon not in item — falls back to view registration's icon
			const result = registry.resolveNavigationItem({
				key: "roadmap",
				type: "plugin",
				icon: "map",
				pluginSource: "roadmap-plugin",
			});

			expect(result.label).toBe("Roadmap");
			expect(result.icon).toBe("map");
		});

		it("falls back to item key as label when plugin not found", () => {
			const result = registry.resolveNavigationItem({
				key: "my-view",
				type: "plugin",
				icon: "circle",
				pluginSource: "nonexistent-plugin",
			});

			expect(result.label).toBe("my-view");
		});
	});

	describe("conflict detection", () => {
		it("checkConflicts detects schema key conflicts", () => {
			const schema = makeSchema({ key: "task" });
			registry.register(makeManifest({
				name: "plugin-a",
				provides: { schemas: [schema], relationships: [], views: [], widgets: [] },
			}), {});

			const manifest = makeManifest({
				name: "plugin-b",
				provides: { schemas: [schema], relationships: [], views: [], widgets: [] },
			});

			const conflicts = registry.checkConflicts(manifest);
			expect(conflicts).toHaveLength(1);
			expect(conflicts[0].type).toBe("schema");
			expect(conflicts[0].key).toBe("task");
			expect(conflicts[0].existingPlugin).toBe("plugin-a");
		});

		it("checkConflicts detects relationship key conflicts with different inverse", () => {
			const rel = makeRelationship({ key: "links-to", inverse: "linked-by" });
			registry.register(makeManifest({
				name: "plugin-a",
				provides: { schemas: [], relationships: [rel], views: [], widgets: [] },
			}), {});

			const conflictingRel = makeRelationship({ key: "links-to", inverse: "references" });
			const manifest = makeManifest({
				name: "plugin-b",
				provides: { schemas: [], relationships: [conflictingRel], views: [], widgets: [] },
			});

			const conflicts = registry.checkConflicts(manifest);
			expect(conflicts.some((c) => c.type === "relationship-key")).toBe(true);
		});

		it("no conflict when two plugins extend same relationship with different type pairs", () => {
			const rel1 = makeRelationship({ key: "evolves-to", inverse: "evolved-from", from: ["research"], to: ["idea"] });
			registry.register(makeManifest({
				name: "plugin-a",
				provides: { schemas: [], relationships: [rel1], views: [], widgets: [] },
			}), {});

			const rel2 = makeRelationship({ key: "evolves-to", inverse: "evolved-from", from: ["idea"], to: ["epic"] });
			const manifest = makeManifest({
				name: "plugin-b",
				provides: { schemas: [], relationships: [rel2], views: [], widgets: [] },
			});

			// Should merge from/to without error
			const conflicts = registry.checkConflicts(manifest);
			expect(conflicts.filter((c) => c.type === "relationship-key")).toHaveLength(0);
		});
	});

	describe("provider management", () => {
		it("activeSidecarKey is null when no sidecar registered", () => {
			expect(registry.activeSidecarKey).toBeNull();
		});

		it("setActiveSidecar updates providerConfig", () => {
			registry.setActiveSidecar("claude-code");
			expect(registry.providerConfig.activeSidecar).toBe("claude-code");
		});

		it("activeSidecar returns null when key does not match any provider", () => {
			registry.setActiveSidecar("nonexistent");
			expect(registry.activeSidecar).toBeNull();
		});
	});
});
