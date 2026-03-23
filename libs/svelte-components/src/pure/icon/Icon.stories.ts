import type { Meta, StoryObj } from "@storybook/svelte";
import Icon from "./Icon.svelte";

const meta = {
	title: "Pure/Icon",
	component: Icon,
	tags: ["autodocs"],
	argTypes: {
		name: {
			control: "select",
			options: ["target", "shield", "zap", "lightbulb", "flag", "bot", "brain", "rocket", "workflow", "network"],
		},
		size: {
			control: "select",
			options: ["xs", "sm", "md", "lg", "xl"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Default: Story = {
	args: { name: "target", size: "md" },
};

export const ExtraSmall: Story = {
	args: { name: "shield", size: "xs" },
};

export const Small: Story = {
	args: { name: "shield", size: "sm" },
};

export const Large: Story = {
	args: { name: "rocket", size: "lg" },
};

export const ExtraLarge: Story = {
	args: { name: "brain", size: "xl" },
};

export const UnknownFallback: Story = {
	args: { name: "nonexistent-icon" },
};
