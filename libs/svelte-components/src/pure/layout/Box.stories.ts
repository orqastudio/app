import type { Meta, StoryObj } from "@storybook/svelte";
import Box from "./Box.svelte";

// Box is a structural-only primitive. Visual decoration (padding, background, border,
// rounded, overflow, margin) must be applied via Panel, SectionHeader, or SectionFooter.
const meta = {
	title: "Pure/Layout/Box",
	component: Box,
	tags: ["autodocs"],
	argTypes: {
		width: {
			control: "select",
			options: [undefined, "full", "auto"],
		},
		height: {
			control: "select",
			options: [undefined, "full", "screen"],
		},
		position: {
			control: "select",
			options: [undefined, "relative", "absolute", "fixed", "sticky"],
		},
		zIndex: {
			control: "select",
			options: [undefined, 10, 20, 30, 40, 50],
		},
		flex: {
			control: "select",
			options: [undefined, 0, 1],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Overlay positioned box (replaces <div class="fixed inset-0 z-40">). */
export const FixedOverlay: Story = { args: { position: "fixed", inset: 0, zIndex: 40 } };

/** flex-1 min-h-0 fill panel (replaces <div class="min-h-0 flex-1">). */
export const FillPanel: Story = { args: { flex: 1, minHeight: 0 } };
