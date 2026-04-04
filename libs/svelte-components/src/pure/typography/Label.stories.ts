import type { Meta, StoryObj } from "@storybook/svelte";
import Label from "./Label.svelte";

const meta = {
	title: "Pure/Typography/Label",
	component: Label,
	tags: ["autodocs"],
	argTypes: {
		required: { control: "boolean" },
		htmlFor: { control: "text" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = { args: { required: false } };
export const Required: Story = { args: { required: true } };
export const WithHtmlFor: Story = { args: { htmlFor: "my-input", required: false } };
