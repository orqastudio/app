// Tests for icon resolution utilities — resolveIcon and DEFAULT_ICON_MAP.
import { describe, it, expect } from "vitest";
import { resolveIcon, DEFAULT_ICON_MAP } from "../../src/pure/icon/icon-utils.js";

describe("resolveIcon", () => {
	it("returns the component for a known icon name", () => {
		const Icon = resolveIcon("target");
		expect(Icon).toBe(DEFAULT_ICON_MAP["target"]);
	});

	it("returns CircleDotIcon as fallback for an unknown name", () => {
		const known = resolveIcon("circle-dot");
		const unknown = resolveIcon("not-a-real-icon");
		// Both should resolve to the same fallback icon (CircleDotIcon)
		expect(unknown).toBe(known);
	});

	it("returns fallback for undefined name", () => {
		const fallback = resolveIcon(undefined);
		const circledot = resolveIcon("circle-dot");
		expect(fallback).toBe(circledot);
	});

	it("checks custom registry before the default map", () => {
		const MockIcon = {} as Parameters<typeof resolveIcon>[1] extends Record<string, infer C>
			? C
			: never;
		const customRegistry = { "my-icon": MockIcon as never };
		const Icon = resolveIcon("my-icon", customRegistry);
		expect(Icon).toBe(MockIcon);
	});

	it("falls back to default map when custom registry does not have the name", () => {
		const customRegistry = { "other-icon": {} as never };
		const Icon = resolveIcon("target", customRegistry);
		expect(Icon).toBe(DEFAULT_ICON_MAP["target"]);
	});

	it("exposes the full default icon map as a record", () => {
		expect(typeof DEFAULT_ICON_MAP).toBe("object");
		expect(DEFAULT_ICON_MAP["search"]).toBeDefined();
		expect(DEFAULT_ICON_MAP["shield"]).toBeDefined();
		expect(DEFAULT_ICON_MAP["calendar"]).toBeDefined();
	});
});
