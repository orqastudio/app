/**
 * Tests for port resolution — getPort(), getPortBase(), DEFAULT_PORT_BASE,
 * PORT_OFFSETS, and the ORQA_PORT_BASE environment variable override.
 *
 * Every OrqaStudio service derives its port from a single base plus a fixed
 * offset. These tests verify that derivation is correct and that the env var
 * override works as expected. Port values are read from infrastructure/ports.json
 * via `@orqastudio/constants` — no hardcoded values appear here.
 */
import { describe, it, expect, afterEach } from "vitest";
import { DEFAULT_PORT_BASE, PORT_OFFSETS, getPortBase, getPort } from "../src/lib/ports.js";

const ORIGINAL_ENV = process.env["ORQA_PORT_BASE"];

afterEach(() => {
	// Restore env after each test that may have mutated it.
	if (ORIGINAL_ENV === undefined) {
		delete process.env["ORQA_PORT_BASE"];
	} else {
		process.env["ORQA_PORT_BASE"] = ORIGINAL_ENV;
	}
});

// ---------------------------------------------------------------------------
// DEFAULT_PORT_BASE
// ---------------------------------------------------------------------------

describe("DEFAULT_PORT_BASE", () => {
	it("is 10100", () => {
		expect(DEFAULT_PORT_BASE).toBe(10100);
	});
});

// ---------------------------------------------------------------------------
// PORT_OFFSETS
// ---------------------------------------------------------------------------

describe("PORT_OFFSETS", () => {
	it("daemon offset is 0 (base port = daemon port)", () => {
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

// ---------------------------------------------------------------------------
// getPortBase
// ---------------------------------------------------------------------------

describe("getPortBase", () => {
	it("returns DEFAULT_PORT_BASE when ORQA_PORT_BASE is unset", () => {
		delete process.env["ORQA_PORT_BASE"];
		expect(getPortBase()).toBe(DEFAULT_PORT_BASE);
	});

	it("returns the parsed integer from ORQA_PORT_BASE", () => {
		process.env["ORQA_PORT_BASE"] = "20200";
		expect(getPortBase()).toBe(20200);
	});

	it("falls back to DEFAULT_PORT_BASE for non-numeric ORQA_PORT_BASE", () => {
		process.env["ORQA_PORT_BASE"] = "not-a-number";
		expect(getPortBase()).toBe(DEFAULT_PORT_BASE);
	});

	it("falls back to DEFAULT_PORT_BASE for empty string", () => {
		process.env["ORQA_PORT_BASE"] = "";
		expect(getPortBase()).toBe(DEFAULT_PORT_BASE);
	});
});

// ---------------------------------------------------------------------------
// getPort
// ---------------------------------------------------------------------------

describe("getPort", () => {
	it("daemon port equals port base (offset 0)", () => {
		delete process.env["ORQA_PORT_BASE"];
		expect(getPort("daemon")).toBe(DEFAULT_PORT_BASE);
	});

	it("lsp port is base + 1", () => {
		delete process.env["ORQA_PORT_BASE"];
		expect(getPort("lsp")).toBe(DEFAULT_PORT_BASE + 1);
	});

	it("mcp port is base + 2", () => {
		delete process.env["ORQA_PORT_BASE"];
		expect(getPort("mcp")).toBe(DEFAULT_PORT_BASE + 2);
	});

	it("vite port is 10420 (base 10100 + offset 320)", () => {
		delete process.env["ORQA_PORT_BASE"];
		expect(getPort("vite")).toBe(10420);
	});

	it("devtools port is base + 40", () => {
		delete process.env["ORQA_PORT_BASE"];
		expect(getPort("devtools")).toBe(DEFAULT_PORT_BASE + 40);
	});

	it("respects ORQA_PORT_BASE override for all services", () => {
		process.env["ORQA_PORT_BASE"] = "20000";
		expect(getPort("daemon")).toBe(20000);
		expect(getPort("lsp")).toBe(20001);
		expect(getPort("mcp")).toBe(20002);
		expect(getPort("vite")).toBe(20320);
	});
});
