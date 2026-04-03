// Tests for Card composition — Root, Header, Title, Content, Action data-slot presence.
import { describe, it, expect } from "vitest";
import { render } from "@testing-library/svelte";
import Card from "../../src/pure/card/card.svelte";
import CardHeader from "../../src/pure/card/card-header.svelte";
import CardTitle from "../../src/pure/card/card-title.svelte";
import CardContent from "../../src/pure/card/card-content.svelte";
import CardAction from "../../src/pure/card/card-action.svelte";

describe("Card (Root)", () => {
	it("renders with data-slot='card'", () => {
		const { container } = render(Card);
		expect(container.querySelector("[data-slot='card']")).not.toBeNull();
	});

	it("renders as a <div>", () => {
		const { container } = render(Card);
		expect(container.querySelector("div[data-slot='card']")).not.toBeNull();
	});

	it("accepts additional class prop", () => {
		const { container } = render(Card, { props: { class: "custom-card" } });
		const el = container.querySelector("[data-slot='card']")!;
		expect(el.className).toContain("custom-card");
	});
});

describe("CardHeader", () => {
	it("renders with data-slot='card-header'", () => {
		const { container } = render(CardHeader);
		expect(container.querySelector("[data-slot='card-header']")).not.toBeNull();
	});
});

describe("CardTitle", () => {
	it("renders with data-slot='card-title'", () => {
		const { container } = render(CardTitle);
		expect(container.querySelector("[data-slot='card-title']")).not.toBeNull();
	});
});

describe("CardContent", () => {
	it("renders with data-slot='card-content'", () => {
		const { container } = render(CardContent);
		expect(container.querySelector("[data-slot='card-content']")).not.toBeNull();
	});
});

describe("CardAction", () => {
	it("renders with data-slot='card-action'", () => {
		const { container } = render(CardAction);
		expect(container.querySelector("[data-slot='card-action']")).not.toBeNull();
	});
});
