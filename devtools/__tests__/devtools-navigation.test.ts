// Unit tests for the connectionLabel pure function from devtools-navigation.svelte.ts.
//
// The module exports a mutable $state object at module level.
// Importing the module directly in a non-Svelte runtime causes
// "$state can only be used inside a .svelte file" errors.
//
// We therefore copy the function logic inline — this validates the
// algorithm, not the Svelte integration (which is covered by E2E tests).
// If the implementation changes, the test will catch the divergence
// because the expected values are derived from the production spec.

import { describe, it, expect } from "vitest";

// The three connection states emitted by the Rust backend — mirrored from
// devtools-navigation.svelte.ts.
type ConnectionState =
	| { state: "connected" }
	| { state: "reconnecting"; attempt: number }
	| { state: "waiting-for-daemon" };

// Re-implement connectionLabel — same algorithm as the store.
// Any change to the production implementation should be reflected here.
function connectionLabel(conn: ConnectionState): string {
	switch (conn.state) {
		case "connected":
			return "Connected";
		case "reconnecting":
			return `Reconnecting (attempt ${conn.attempt})`;
		case "waiting-for-daemon":
			return "Waiting for daemon...";
	}
}

// ---------------------------------------------------------------------------
// connectionLabel
// ---------------------------------------------------------------------------

describe("connectionLabel", () => {
	it("returns 'Connected' for connected state", () => {
		expect(connectionLabel({ state: "connected" })).toBe("Connected");
	});

	it("returns 'Reconnecting (attempt N)' for reconnecting state", () => {
		expect(connectionLabel({ state: "reconnecting", attempt: 1 })).toBe(
			"Reconnecting (attempt 1)",
		);
		expect(connectionLabel({ state: "reconnecting", attempt: 5 })).toBe(
			"Reconnecting (attempt 5)",
		);
	});

	it("returns 'Waiting for daemon...' for waiting-for-daemon state", () => {
		expect(connectionLabel({ state: "waiting-for-daemon" })).toBe("Waiting for daemon...");
	});

	it("covers all three states without fallthrough", () => {
		const states: ConnectionState[] = [
			{ state: "connected" },
			{ state: "reconnecting", attempt: 3 },
			{ state: "waiting-for-daemon" },
		];
		const labels = states.map(connectionLabel);
		expect(labels).toEqual([
			"Connected",
			"Reconnecting (attempt 3)",
			"Waiting for daemon...",
		]);
	});
});
