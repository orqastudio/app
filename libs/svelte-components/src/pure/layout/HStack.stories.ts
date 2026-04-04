import type { Meta, StoryObj } from "@storybook/svelte";
import HStack from "./HStack.svelte";

const meta = {
	title: "Pure/Layout/HStack",
	component: HStack,
	tags: ["autodocs"],
	argTypes: {
		gap: {
			control: "select",
			options: [0, 0.5, 1, 1.5, 2, 3, 4, 6, 8],
		},
		align: {
			control: "select",
			options: ["start", "center", "end", "baseline", "stretch"],
		},
		justify: {
			control: "select",
			options: ["start", "center", "end", "between", "around"],
		},
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
		height: {
			control: "select",
			options: [undefined, "full", "screen"],
		},
		overflow: {
			control: "select",
			options: [undefined, "hidden", "auto", "scroll", "visible"],
		},
		flex: {
			control: "select",
			options: [undefined, 0, 1],
		},
		marginTop: {
			control: "select",
			options: [undefined, 0, 1, 2, 3, 4, 6, 8],
		},
		wrap: { control: "boolean" },
		full: { control: "boolean" },
		borderTop: { control: "boolean" },
		borderBottom: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { gap: 2 } };
export const SpaceBetween: Story = { args: { gap: 2, justify: "between" } };
export const Wrapped: Story = { args: { gap: 2, wrap: true } };

/** Replaces <div class="border-b px-3 py-2"> header row. */
export const BorderedHeader: Story = { args: { gap: 2, borderBottom: true, paddingX: 3, paddingY: 2 } };

/** Full-width spaced row with padding. */
export const ToolbarRow: Story = { args: { gap: 2, justify: "between", full: true, paddingX: 3, paddingY: 2 } };
