// Tests for fixture data — ensures fixtures are well-formed and useful for tests.
import { describe, it, expect } from "vitest";
import {
	FIXTURE_ARTIFACTS,
	FIXTURE_STATUSES,
	FIXTURE_ARTIFACT_TYPES,
	FIXTURE_PROJECT_SETTINGS,
} from "../../src/testing/fixtures.js";

describe("FIXTURE_STATUSES", () => {
	it("contains at least 10 status entries", () => {
		expect(FIXTURE_STATUSES.length).toBeGreaterThanOrEqual(10);
	});

	it("each entry has key, label, and icon", () => {
		for (const s of FIXTURE_STATUSES) {
			expect(s.key).toBeTruthy();
			expect(s.label).toBeTruthy();
			expect(s.icon).toBeTruthy();
		}
	});
});

describe("FIXTURE_ARTIFACT_TYPES", () => {
	it("contains the core artifact types", () => {
		const keys = FIXTURE_ARTIFACT_TYPES.map((t) => t.key);
		expect(keys).toContain("epic");
		expect(keys).toContain("task");
		expect(keys).toContain("milestone");
	});

	it("each entry has key, label, and plural", () => {
		for (const t of FIXTURE_ARTIFACT_TYPES) {
			expect(t.key).toBeTruthy();
			expect(t.label).toBeTruthy();
			expect(t.plural).toBeTruthy();
		}
	});
});

describe("FIXTURE_ARTIFACTS", () => {
	it("contains at least 5 artifacts", () => {
		expect(FIXTURE_ARTIFACTS.length).toBeGreaterThanOrEqual(5);
	});

	it("each artifact has id, type, title, and status", () => {
		for (const a of FIXTURE_ARTIFACTS) {
			expect(a.id).toBeTruthy();
			expect(a.type).toBeTruthy();
			expect(a.title).toBeTruthy();
			expect(a.status).toBeTruthy();
		}
	});

	it("each artifact has a relationships array", () => {
		for (const a of FIXTURE_ARTIFACTS) {
			expect(Array.isArray(a.relationships)).toBe(true);
		}
	});
});

describe("FIXTURE_PROJECT_SETTINGS", () => {
	it("has a name", () => {
		expect(FIXTURE_PROJECT_SETTINGS.name).toBeTruthy();
	});

	it("has statuses and artifactTypes arrays", () => {
		expect(Array.isArray(FIXTURE_PROJECT_SETTINGS.statuses)).toBe(true);
		expect(Array.isArray(FIXTURE_PROJECT_SETTINGS.artifactTypes)).toBe(true);
	});
});
