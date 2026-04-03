// Tests for ProgressBar — renders label, current/total values, and calculates percentage.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import ProgressBar from "../../src/pure/progress-bar/ProgressBar.svelte";

describe("ProgressBar", () => {
	it("renders the label text", () => {
		const { getByText } = render(ProgressBar, {
			props: { label: "Tasks", current: 3, total: 10 },
		});
		expect(getByText("Tasks")).toBeInTheDocument();
	});

	it("renders the current/total fraction", () => {
		const { getByText } = render(ProgressBar, {
			props: { label: "Tasks", current: 3, total: 10 },
		});
		expect(getByText("3/10")).toBeInTheDocument();
	});

	it("sets the fill bar width to the correct percentage", () => {
		const { container } = render(ProgressBar, {
			props: { label: "Progress", current: 50, total: 100 },
		});
		// The inner fill div should have style="width: 50%"
		const fill = container.querySelector("[style*='width: 50%']");
		expect(fill).not.toBeNull();
	});

	it("shows 0% width when current is 0", () => {
		const { container } = render(ProgressBar, {
			props: { label: "Progress", current: 0, total: 10 },
		});
		const fill = container.querySelector("[style*='width: 0%']");
		expect(fill).not.toBeNull();
	});

	it("shows 0% width when total is 0 to avoid division by zero", () => {
		const { container } = render(ProgressBar, {
			props: { label: "Progress", current: 0, total: 0 },
		});
		const fill = container.querySelector("[style*='width: 0%']");
		expect(fill).not.toBeNull();
	});

	it("rounds the percentage to a whole number", () => {
		// 1/3 = 33.33%, should round to 33%
		const { container } = render(ProgressBar, {
			props: { label: "Progress", current: 1, total: 3 },
		});
		const fill = container.querySelector("[style*='width: 33%']");
		expect(fill).not.toBeNull();
	});
});
