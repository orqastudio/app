// Tests for the LoadingSpinner component — renders with size prop.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import LoadingSpinner from "../../src/pure/loading-spinner/LoadingSpinner.svelte";

describe("LoadingSpinner", () => {
	it("renders without throwing", () => {
		expect(() => render(LoadingSpinner)).not.toThrow();
	});

	it("renders the spinner element with a size class", () => {
		const { container } = render(LoadingSpinner, { props: { size: "md" } });
		const spinner = container.querySelector(".animate-spin");
		expect(spinner).not.toBeNull();
		expect(spinner!.className).toContain("h-8");
	});

	it("applies sm size class for size='sm'", () => {
		const { container } = render(LoadingSpinner, { props: { size: "sm" } });
		const spinner = container.querySelector(".animate-spin");
		expect(spinner!.className).toContain("h-4");
	});

	it("applies lg size class for size='lg'", () => {
		const { container } = render(LoadingSpinner, { props: { size: "lg" } });
		const spinner = container.querySelector(".animate-spin");
		expect(spinner!.className).toContain("h-12");
	});

	it("defaults to md size when no size prop is provided", () => {
		const { container } = render(LoadingSpinner);
		const spinner = container.querySelector(".animate-spin");
		expect(spinner!.className).toContain("h-8");
	});
});
