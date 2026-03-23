import { describe, it, expect } from "vitest";
import { extractErrorMessage } from "../src/ipc/invoke.js";

describe("extractErrorMessage", () => {
	it("extracts message from an Error instance", () => {
		const err = new Error("Something went wrong");
		expect(extractErrorMessage(err)).toBe("Something went wrong");
	});

	it("returns a string error directly", () => {
		expect(extractErrorMessage("Network failure")).toBe("Network failure");
	});

	it("extracts message from an OrqaError-shaped object", () => {
		const err = { code: "not_found", message: "Session not found" };
		expect(extractErrorMessage(err)).toBe("Session not found");
	});

	it("stringifies unknown error types", () => {
		expect(extractErrorMessage(42)).toBe("42");
		expect(extractErrorMessage(null)).toBe("null");
		expect(extractErrorMessage(undefined)).toBe("undefined");
	});

	it("handles object without message property", () => {
		const err = { code: "database" };
		expect(extractErrorMessage(err)).toBe("[object Object]");
	});

	it("handles object with empty message", () => {
		const err = { code: "validation", message: "" };
		expect(extractErrorMessage(err)).toBe("");
	});
});
