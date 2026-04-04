import type { Meta, StoryObj } from "@storybook/svelte";
import Text from "./Text.svelte";

const meta = {
	title: "Pure/Typography/Text",
	component: Text,
	tags: ["autodocs"],
	argTypes: {
		variant: {
			control: "select",
			options: [
				"body",
				"body-muted",
				"caption",
				"label",
				"overline",
				"mono",
				"tabular",
				"heading-xl",
				"heading-lg",
				"heading-base",
				"heading-sm",
			],
		},
		tone: {
			control: "select",
			options: ["warning", "destructive", "success", "muted"],
		},
		truncate: { control: "boolean" },
		block: { control: "boolean" },
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Body: Story = { args: { variant: "body" } };
export const BodyMuted: Story = { args: { variant: "body-muted" } };
export const Caption: Story = { args: { variant: "caption" } };
export const Label: Story = { args: { variant: "label" } };
export const Overline: Story = { args: { variant: "overline" } };
export const Mono: Story = { args: { variant: "mono" } };
export const Tabular: Story = { args: { variant: "tabular" } };
export const HeadingXL: Story = { args: { variant: "heading-xl" } };
export const HeadingLG: Story = { args: { variant: "heading-lg" } };
export const HeadingBase: Story = { args: { variant: "heading-base" } };
export const HeadingSM: Story = { args: { variant: "heading-sm" } };
export const WithToneWarning: Story = { args: { variant: "body", tone: "warning" } };
export const WithToneDestructive: Story = { args: { variant: "body", tone: "destructive" } };
export const Truncated: Story = { args: { variant: "body", truncate: true } };
export const Block: Story = { args: { variant: "body", block: true } };
