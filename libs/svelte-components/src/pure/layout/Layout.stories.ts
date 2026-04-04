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
	},
} satisfies Meta;

export default stackMeta;
type Story = StoryObj;

export const DefaultStack: Story = { args: { gap: 2 } };
export const TightStack: Story = { args: { gap: 1 } };
export const WideStack: Story = { args: { gap: 6 } };
export const CenteredStack: Story = { args: { gap: 2, align: "center" } };
