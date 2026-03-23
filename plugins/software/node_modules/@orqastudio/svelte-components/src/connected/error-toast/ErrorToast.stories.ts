import type { Meta, StoryObj } from "@storybook/svelte";
import ErrorToast from "./ErrorToast.svelte";

const meta = {
	title: "Connected/ErrorToast",
	component: ErrorToast,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };
