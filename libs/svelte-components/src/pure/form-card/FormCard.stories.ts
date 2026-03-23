import type { Meta, StoryObj } from "@storybook/svelte";
import FormCard from "./FormCard.svelte";

const meta = {
	title: "Pure/FormCard",
	component: FormCard,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const WithDescription: Story = {
	args: {
		title: "General Settings",
		description: "Configure your project's basic information.",
	},
};

export const TitleOnly: Story = {
	args: { title: "Model Configuration" },
};
