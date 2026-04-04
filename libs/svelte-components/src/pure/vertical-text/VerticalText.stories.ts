import type { Meta, StoryObj } from "@storybook/svelte";
import VerticalText from "./VerticalText.svelte";

const meta = {
	title: "Pure/VerticalText",
	component: VerticalText,
	tags: ["autodocs"],
	argTypes: {
		variant: {
			control: "select",
			options: [
				"body", "body-muted", "body-strong", "body-strong-muted",
				"caption", "caption-strong", "caption-mono", "caption-tabular",
				"label", "overline", "overline-muted", "mono", "tabular",
			],
		},
		tone: {
			control: "select",
			options: ["warning", "destructive", "success", "muted"],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

// Default: caption variant used for Kanban column labels.
export const Default: Story = { args: { variant: "caption" } };
export const Overline: Story = { args: { variant: "overline" } };
export const OverlineMuted: Story = { args: { variant: "overline-muted" } };
export const BodyStrong: Story = { args: { variant: "body-strong" } };
export const CaptionSuccessTone: Story = { args: { variant: "caption", tone: "success" } };
export const CaptionWarningTone: Story = { args: { variant: "caption", tone: "warning" } };
