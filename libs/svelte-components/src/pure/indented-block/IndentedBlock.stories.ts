import type { Meta, StoryObj } from "@storybook/svelte";
import IndentedBlock from "./IndentedBlock.svelte";

const meta = {
	title: "Pure/IndentedBlock",
	component: IndentedBlock,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {};
