import type { Meta, StoryObj } from "@storybook/svelte";
import TwoByTwoGrid from "./TwoByTwoGrid.svelte";

const meta = {
	title: "Pure/TwoByTwoGrid",
	component: TwoByTwoGrid,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: {},
};
