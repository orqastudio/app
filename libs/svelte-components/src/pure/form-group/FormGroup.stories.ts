import type { Meta, StoryObj } from "@storybook/svelte";
import FormGroup from "./FormGroup.svelte";

const meta = {
	title: "Pure/FormGroup",
	component: FormGroup,
	tags: ["autodocs"],
	argTypes: {
		label: { control: "text" },
		description: { control: "text" },
		error: { control: "text" },
		required: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { label: "Email", description: "Enter your email address" },
};

export const Required: Story = {
	args: { label: "Name", required: true },
};

export const WithError: Story = {
	args: { label: "Password", error: "Password must be at least 8 characters" },
};

export const WithDescription: Story = {
	args: { label: "API Key", description: "Found in your provider settings dashboard" },
};
