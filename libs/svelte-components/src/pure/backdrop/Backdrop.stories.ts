import type { Meta, StoryObj } from "@storybook/svelte";
import Backdrop from "./Backdrop.svelte";

const meta = {
	title: "Pure/Backdrop",
	component: Backdrop,
	tags: ["autodocs"],
	argTypes: {
		label: { control: "text" },
		zIndex: { control: "number" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { label: "Example dialog", zIndex: 50 },
};
