import type { Meta, StoryObj } from "@storybook/svelte";
import Stack from "./Stack.svelte";

// Stack is a structural-only primitive. Visual decoration (padding, border, margin,
// overflow) must be applied via Panel, SectionHeader, or SectionFooter.
const stackMeta = {
	title: "Pure/Layout/Stack",
	component: Stack,
	tags: ["autodocs"],
	argTypes: {
		gap: {
			control: "select",
			options: [0, 0.5, 1, 1.5, 2, 3, 4, 6, 8],
		},
		align: {
			control: "select",
			options: ["start", "center", "end", "stretch"],
		},
		height: {
			control: "select",
			options: [undefined, "full", "screen"],
		},
		flex: {
			control: "select",
			options: [undefined, 0, 1],
		},
		full: { control: "boolean" },
	},
} satisfies Meta;

export default stackMeta;
type Story = StoryObj;

export const DefaultStack: Story = { args: { gap: 2 } };
export const TightStack: Story = { args: { gap: 1 } };
export const WideStack: Story = { args: { gap: 6 } };
export const CenteredStack: Story = { args: { gap: 2, align: "center" } };

/** Replaces <div class="flex h-full flex-col">. */
export const FullHeightStack: Story = { args: { gap: 2, height: "full" } };

/** Replaces <div class="flex flex-col min-h-0 flex-1"> in split-pane layouts. */
export const FillStack: Story = { args: { gap: 2, minHeight: 0, flex: 1 } };
