import type { Meta, StoryObj } from "@storybook/svelte";
import Heading from "./Heading.svelte";
import Caption from "./Caption.svelte";
import Code from "./Code.svelte";

const headingMeta = {
	title: "Pure/Typography/Heading",
	component: Heading,
	tags: ["autodocs"],
	argTypes: {
		level: {
			control: "select",
			options: [1, 2, 3, 4, 5, 6],
		},
	},
} satisfies Meta;

export default headingMeta;
type Story = StoryObj;

export const H1: Story = { args: { level: 1 } };
export const H2: Story = { args: { level: 2 } };
export const H3: Story = { args: { level: 3 } };
export const H4: Story = { args: { level: 4 } };
export const H5: Story = { args: { level: 5 } };
export const H6: Story = { args: { level: 6 } };
