/**
 * Tests for port constants and resolution functions.
 *
 * Verifies that the single-source-of-truth port constants (read from
 * infrastructure/ports.json) have the expected values and that
 * getPort/getPortBase correctly apply environment overrides.
 */

import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { DEFAULT_PORT_BASE, PORT_OFFSETS, getPortBase, getPort } from "../src/ports.js";

describe("DEFAULT_PORT_BASE", () => {
	it("is 10100", () => {
		expect(DEFAULT_PORT_BASE).toBe(10100);
	});
});

describe("PORT_OFFSETS", () => {
	it("daemon offset is 0 (daemon IS the base port)", () => {
		expect(PORT_OFFSETS.daemon).toBe(0);
	});

	it("lsp offset is 1", () => {
		expect(PORT_OFFSETS.lsp).toBe(1);
	});

	it("mcp offset is 2", () => {
		expect(PORT_OFFSETS.mcp).toBe(2);
	});

	it("vite offset is 320 (10100 + 320 = 10420)", () => {
		expect(PORT_OFFSETS.vite).toBe(320);
	});

	it("dashboard offset is 30", () => {
		expect(PORT_OFFSETS.dashboard).toBe(30);
	});

	it("sync offset is 31", () => {
		expect(PORT_OFFSETS.sync).toBe(31);
	});

	it("devtools offset is 40", () => {
		expect(PORT_OFFSETS.devtools).toBe(40);
	});

	it("storybook offset is 50", () => {
		expect(PORT_OFFSETS.storybook).toBe(50);
	});
});

describe("getPortBase", () => {
	const original = process.env["ORQA_PORT_BASE"];

	beforeEach(() => {
		delete process.env["ORQA_PORT_BASE"];
	});

	afterEach(() => {
		if (original !== undefined) {
			process.env["ORQA_PORT_BASE"] = original;
		} else {
			delete process.env["ORQA_PORT_BASE"];
		}
	});

	it("returns DEFAULT_PORT_BASE when env var is absent", () => {
		expect(getPortBase()).toBe(DEFAULT_PORT_BASE);
	});

	it("returns the parsed integer when ORQA_PORT_BASE is set to a valid number", () => {
		process.env["ORQA_PORT_BASE"] = "20000";
		expect(getPortBase()).toBe(20000);
	});

	it("returns DEFAULT_PORT_BASE when ORQA_PORT_BASE is not a valid integer", () => {
		process.env["ORQA_PORT_BASE"] = "not-a-number";
		expect(getPortBase()).toBe(DEFAULT_PORT_BASE);
	});

	it("returns DEFAULT_PORT_BASE when ORQA_PORT_BASE is an empty string", () => {
		process.env["ORQA_PORT_BASE"] = "";
		expect(getPortBase()).toBe(DEFAULT_PORT_BASE);
	});

	it("parses a leading-zero numeric string correctly", () => {
		process.env["ORQA_PORT_BASE"] = "09999";
		// parseInt("09999", 10) === 9999
		expect(getPortBase()).toBe(9999);
	});
});

describe("getPort", () => {
	const original = process.env["ORQA_PORT_BASE"];

	beforeEach(() => {
		delete process.env["ORQA_PORT_BASE"];
	});

	afterEach(() => {
		if (original !== undefined) {
			process.env["ORQA_PORT_BASE"] = original;
		} else {
			delete process.env["ORQA_PORT_BASE"];
		}
	});

	it("daemon port equals DEFAULT_PORT_BASE (offset 0)", () => {
		expect(getPort("daemon")).toBe(10100);
	});

	it("lsp port equals DEFAULT_PORT_BASE + 1", () => {
		expect(getPort("lsp")).toBe(10101);
	});

	it("mcp port equals DEFAULT_PORT_BASE + 2", () => {
		expect(getPort("mcp")).toBe(10102);
	});

	it("vite port equals 10420 (base 10100 + offset 320)", () => {
		expect(getPort("vite")).toBe(10420);
	});

	it("dashboard port equals DEFAULT_PORT_BASE + 30", () => {
		expect(getPort("dashboard")).toBe(10130);
	});

	it("sync port equals DEFAULT_PORT_BASE + 31", () => {
		expect(getPort("sync")).toBe(10131);
	});

	it("devtools port equals DEFAULT_PORT_BASE + 40", () => {
		expect(getPort("devtools")).toBe(10140);
	});

	it("storybook port equals DEFAULT_PORT_BASE + 50", () => {
		expect(getPort("storybook")).toBe(10150);
	});

	it("respects ORQA_PORT_BASE override for all services", () => {
		process.env["ORQA_PORT_BASE"] = "20000";
		expect(getPort("daemon")).toBe(20000);
		expect(getPort("lsp")).toBe(20001);
		expect(getPort("mcp")).toBe(20002);
		expect(getPort("dashboard")).toBe(20030);
	});
});
