// Tests for the Badge component — variant rendering and element type.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import Badge from "../../src/pure/badge/badge.svelte";

describe("Badge", () => {
	it("renders as a <span> element by default", () => {
		const { container } = render(Badge);
		expect(container.querySelector("span[data-slot='badge']")).not.toBeNull();
	});

	it("renders as an <a> element when href is provided", () => {
		const { container } = render(Badge, { props: { href: "/somewhere" } });
		const anchor = container.querySelector("a[data-slot='badge']");
		expect(anchor).not.toBeNull();
		expect((anchor as HTMLAnchorElement).href).toContain("/somewhere");
	});

	it("sets data-slot='badge' on the element", () => {
		const { container } = render(Badge);
		expect(container.querySelector("[data-slot='badge']")).not.toBeNull();
	});

	it("applies a class for each variant without throwing", () => {
		const variants = ["default", "secondary", "destructive", "outline", "warning"] as const;
		for (const variant of variants) {
			expect(() => render(Badge, { props: { variant } })).not.toThrow();
		}
	});

	it("accepts additional class props", () => {
		const { container } = render(Badge, { props: { class: "my-custom-class" } });
		const el = container.querySelector("[data-slot='badge']")!;
		expect(el.className).toContain("my-custom-class");
	});
});
