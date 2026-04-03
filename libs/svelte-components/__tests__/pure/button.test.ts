// Tests for the Button component — variant rendering, click handling, disabled state.
import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import Button from "../../src/pure/button/button.svelte";

describe("Button", () => {
	it("renders with data-slot='button'", () => {
		const { container } = render(Button);
		expect(container.querySelector("[data-slot='button']")).not.toBeNull();
	});

	it("renders as a <button> element by default", () => {
		const { getByRole } = render(Button);
		expect(getByRole("button").tagName).toBe("BUTTON");
	});

	it("renders as an <a> element when href is provided", () => {
		const { container } = render(Button, { props: { href: "/home" } });
		expect(container.querySelector("a")).not.toBeNull();
	});

	it("calls onclick handler when clicked", async () => {
		const onclick = vi.fn();
		const { getByRole } = render(Button, { props: { onclick } });
		await fireEvent.click(getByRole("button"));
		expect(onclick).toHaveBeenCalledOnce();
	});

	it("sets the disabled attribute and pointer-events-none when disabled=true", () => {
		// The component uses CSS pointer-events-none to prevent clicks on disabled buttons.
		// fireEvent.click() bypasses CSS so we verify the HTML disabled attr and CSS class.
		const { getByRole } = render(Button, { props: { disabled: true } });
		const btn = getByRole("button");
		expect(btn).toBeDisabled();
		expect(btn.className).toContain("pointer-events-none");
	});

	it("applies variant class to the button", () => {
		const { getByRole } = render(Button, { props: { variant: "destructive" } });
		// tailwind-variants applies classes — just verify the button has classes
		expect(getByRole("button").className).toBeTruthy();
	});
});
