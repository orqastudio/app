// Tests for Separator — renders with horizontal/vertical orientation via data-orientation.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import Separator from "../../src/pure/separator/separator.svelte";

describe("Separator", () => {
	it("renders without throwing", () => {
		expect(() => render(Separator)).not.toThrow();
	});

	it("renders a separator element with data-slot", () => {
		const { container } = render(Separator);
		const el = container.querySelector("[data-slot='separator']");
		expect(el).not.toBeNull();
	});

	it("defaults to horizontal orientation", () => {
		const { container } = render(Separator);
		const el = container.querySelector("[data-slot='separator']")!;
		// bits-ui sets data-orientation based on the orientation prop
		// default is horizontal
		expect(el.getAttribute("data-orientation")).toBe("horizontal");
	});

	it("accepts a vertical orientation prop", () => {
		const { container } = render(Separator, { props: { orientation: "vertical" } });
		const el = container.querySelector("[data-slot='separator']")!;
		expect(el.getAttribute("data-orientation")).toBe("vertical");
	});
});
