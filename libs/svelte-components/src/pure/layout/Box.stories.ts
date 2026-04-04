import type { Meta, StoryObj } from "@storybook/svelte";
import Box from "./Box.svelte";

const meta = {
	title: "Pure/Layout/Box",
	component: Box,
	tags: ["autodocs"],
	argTypes: {
		padding: {
			control: "select",
			options: [undefined, 0, 0.5, 1, 1.5, 2, 3, 4, 6, 8],
		},
		paddingX: {
			control: "select",
			options: [undefined, 0, 0.5, 1, 1.5, 2, 3, 4, 6, 8],
		},
		paddingY: {
			control: "select",
			options: [undefined, 0, 0.5, 1, 1.5, 2, 3, 4, 6, 8],
		},
		width: {
			control: "select",
			options: [undefined, "full", "auto"],
		},
		height: {
			control: "select",
			options: [undefined, "full", "screen"],
		},
		overflow: {
			control: "select",
			options: [undefined, "hidden", "auto", "scroll", "visible"],
		},
		position: {
			control: "select",
			options: [undefined, "relative", "absolute", "fixed", "sticky"],
		},
		zIndex: {
			control: "select",
			options: [undefined, 10, 20, 30, 40, 50],
		},
		rounded: {
			control: "select",
			options: [undefined, "none", "sm", "md", "lg", "xl", "full"],
		},
		background: {
			control: "select",
			options: [undefined, "card", "muted", "surface", "transparent"],
		},
		flex: {
			control: "select",
			options: [undefined, 0, 1],
		},
		border: { control: "boolean" },
		borderTop: { control: "boolean" },
		borderBottom: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

/** Basic padded box. */
export const Padded: Story = { args: { padding: 3 } };

/** Bordered box with rounded corners. */
export const BorderedRounded: Story = { args: { border: true, rounded: "md", padding: 4 } };

/** Muted background card-style box. */
export const MutedBackground: Story = { args: { background: "muted", padding: 4, rounded: "lg" } };

/** Bottom-bordered header row (replaces <div class="border-b px-3 py-2">). */
export const BorderedHeader: Story = { args: { borderBottom: true, paddingX: 3, paddingY: 2 } };

/** Overlay positioned box (replaces <div class="fixed inset-0 z-40">). */
export const FixedOverlay: Story = { args: { position: "fixed", inset: 0, zIndex: 40 } };

/** flex-1 min-h-0 fill panel (replaces <div class="min-h-0 flex-1">). */
export const FillPanel: Story = { args: { flex: 1, minHeight: 0 } };
