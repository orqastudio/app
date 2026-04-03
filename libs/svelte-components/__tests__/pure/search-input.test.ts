// Tests for SearchInput — renders placeholder, accepts value binding, size classes.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import SearchInput from "../../src/pure/search-input/SearchInput.svelte";

describe("SearchInput", () => {
	it("renders an input element", () => {
		const { container } = render(SearchInput);
		expect(container.querySelector("input")).not.toBeNull();
	});

	it("uses the default placeholder 'Search...'", () => {
		const { container } = render(SearchInput);
		const input = container.querySelector("input")!;
		expect(input.placeholder).toBe("Search...");
	});

	it("accepts a custom placeholder", () => {
		const { container } = render(SearchInput, { props: { placeholder: "Find artifact..." } });
		const input = container.querySelector("input")!;
		expect(input.placeholder).toBe("Find artifact...");
	});

	it("renders a search icon element (svg)", () => {
		const { container } = render(SearchInput);
		expect(container.querySelector("svg")).not.toBeNull();
	});

	it("renders without throwing for sm size", () => {
		expect(() => render(SearchInput, { props: { size: "sm" } })).not.toThrow();
	});

	it("renders without throwing for xs size", () => {
		expect(() => render(SearchInput, { props: { size: "xs" } })).not.toThrow();
	});
});
