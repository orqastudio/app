import type { Meta, StoryObj } from "@storybook/svelte";
import ToastContainer from "./ToastContainer.svelte";

const meta: Meta = {
	title: "Connected/ToastContainer",
	component: ToastContainer,
	tags: ["autodocs"],
};

export default meta;
type Story = StoryObj;

export const Default: Story = { args: {} };
