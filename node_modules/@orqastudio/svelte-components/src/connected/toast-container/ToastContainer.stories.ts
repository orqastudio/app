import type { Meta, StoryObj } from "@storybook/svelte";
import ToastContainer from "./ToastContainer.svelte";

const meta = {
	title: "Connected/ToastContainer",
	component: ToastContainer,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };
