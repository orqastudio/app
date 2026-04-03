/**
 * Tests for the daemon-client module — buildBinaryArgs (via the public
 * callDaemonGraph interface is not directly unit-testable without a running
 * daemon, so we test the exported helpers and response types).
 *
 * We test:
 *   - isDaemonRunning() returns false when nothing is running on the port
 *   - callDaemonGraph() throws when neither daemon nor binary are found
 *   - Type shapes for DaemonArtifactRef, DaemonArtifactNode, DaemonHealthResponse
 */
import { describe, it, expect } from "vitest";
import {
	isDaemonRunning,
	callDaemonGraph,
	type DaemonArtifactRef,
	type DaemonArtifactNode,
	type DaemonHealthResponse,
} from "../src/lib/daemon-client.js";

// ---------------------------------------------------------------------------
// isDaemonRunning — should return false when daemon is not running
// ---------------------------------------------------------------------------

describe("isDaemonRunning", () => {
	it("returns false when the daemon is not running", async () => {
		// In the test environment there is no running daemon.
		// This should resolve to false rather than throw.
		const result = await isDaemonRunning();
		expect(result).toBe(false);
	});

	it("returns a boolean", async () => {
		const result = await isDaemonRunning();
		expect(typeof result).toBe("boolean");
	});
});

// ---------------------------------------------------------------------------
// callDaemonGraph — falls back to binary, which is not present in CI
// ---------------------------------------------------------------------------

describe("callDaemonGraph", () => {
	it("throws when daemon is unreachable and no binary is found", async () => {
		// The binary is not present in the CLI source tree, so the fallback
		// will also fail, producing an error about missing binary or daemon.
		await expect(
			callDaemonGraph("GET", "/health"),
		).rejects.toThrow();
	});

	it("throws a descriptive error for unknown endpoint when no binary exists", async () => {
		await expect(
			callDaemonGraph("POST", "/unknown-endpoint", { foo: "bar" }),
		).rejects.toThrow();
	});
});

// ---------------------------------------------------------------------------
// Type contract — interfaces have the expected shape
// ---------------------------------------------------------------------------

describe("DaemonArtifactRef type shape", () => {
	it("can be constructed with all required fields", () => {
		const ref: DaemonArtifactRef = {
			target_id: "EPIC-001",
			field: "delivers",
			source_id: "TASK-001",
			relationship_type: "delivers",
		};
		expect(ref.target_id).toBe("EPIC-001");
		expect(ref.field).toBe("delivers");
		expect(ref.source_id).toBe("TASK-001");
		expect(ref.relationship_type).toBe("delivers");
	});

	it("allows null relationship_type", () => {
		const ref: DaemonArtifactRef = {
			target_id: "EPIC-001",
			field: "epic",
			source_id: "TASK-001",
			relationship_type: null,
		};
		expect(ref.relationship_type).toBeNull();
	});
});

describe("DaemonArtifactNode type shape", () => {
	it("can be constructed with required fields", () => {
		const node: DaemonArtifactNode = {
			id: "TASK-001",
			path: ".orqa/delivery/tasks/TASK-001.md",
			artifact_type: "task",
			title: "Implement the feature",
			description: null,
			status: "active",
			priority: "high",
			frontmatter: { epic: "EPIC-001" },
			references_out: [],
			references_in: [],
		};
		expect(node.id).toBe("TASK-001");
		expect(node.artifact_type).toBe("task");
		expect(node.references_out).toHaveLength(0);
	});
});

describe("DaemonHealthResponse type shape", () => {
	it("can be constructed with required fields", () => {
		const health: DaemonHealthResponse = {
			status: "ok",
			artifacts: 42,
			rules: 7,
		};
		expect(health.status).toBe("ok");
		expect(health.artifacts).toBe(42);
		expect(health.rules).toBe(7);
	});
});
