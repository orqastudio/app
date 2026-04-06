import type { Meta, StoryObj } from "@storybook/svelte";
import Heading from "./Heading.svelte";
import Caption from "./Caption.svelte";
import Code from "./Code.svelte";

const headingMeta = {
	title: "Pure/Typography/Heading",
	component: Heading,
	tags: ["autodocs"],
	argTypes: {
		level: {
			control: "select",
			options: [1, 2, 3, 4, 5, 6],
		},
	},
} satisfies Meta;

export default headingMeta;
type Story = StoryObj;

export const H1: Story = { args: { level: 1 } };
export const H2: Story = { args: { level: 2 } };
export const H3: Story = { args: { level: 3 } };
export const H4: Story = { args: { level: 4 } };
export const H5: Story = { args: { level: 5 } };
export const H6: Story = { args: { level: 6 } };

// Caption variants — all restricted to the caption-family.
export const CaptionDefault: StoryObj<typeof Caption> = { args: { variant: "caption" } };
export const CaptionStrong: StoryObj<typeof Caption> = { args: { variant: "caption-strong" } };
export const CaptionMono: StoryObj<typeof Caption> = { args: { variant: "caption-mono" } };
export const CaptionTabular: StoryObj<typeof Caption> = { args: { variant: "caption-tabular" } };
export const CaptionLineClamp2: StoryObj<typeof Caption> = {
	args: { variant: "caption", lineClamp: 2 },
};

// Code block mode — for multi-line code and log output.
export const CodeInline: StoryObj<typeof Code> = { args: { block: false } };
export const CodeBlock: StoryObj<typeof Code> = { args: { block: true } };
