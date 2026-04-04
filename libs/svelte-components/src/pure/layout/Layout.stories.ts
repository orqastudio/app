import type { Meta, StoryObj } from "@storybook/svelte";
import Stack from "./Stack.svelte";
import HStack from "./HStack.svelte";
import Grid from "./Grid.svelte";
import Center from "./Center.svelte";

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
		borderTop: { control: "boolean" },
		borderBottom: { control: "boolean" },
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

/** Replaces <div class="flex flex-col p-4">. */
export const PaddedStack: Story = { args: { gap: 2, padding: 4 } };

/** Replaces <div class="flex flex-col min-h-0 flex-1"> in split-pane layouts. */
export const FillStack: Story = { args: { gap: 2, minHeight: 0, flex: 1 } };

/** Stack with a top border separator. */
export const BorderedStack: Story = { args: { gap: 2, borderTop: true, paddingTop: 3, marginTop: 3 } };
