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
				"body-strong",
				"body-strong-muted",
				"caption",
				"caption-strong",
				"caption-mono",
				"caption-tabular",
				"label",
				"overline",
				"overline-muted",
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
		lineClamp: {
			control: "select",
			options: [undefined, 1, 2, 3, 4],
		},
	},
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const Body: Story = { args: { variant: "body" } };
export const BodyMuted: Story = { args: { variant: "body-muted" } };

// body-strong: emphasized body text that stays at foreground color, e.g. column headers, active nav labels.
export const BodyStrong: Story = { args: { variant: "body-strong" } };

// body-strong-muted: emphasized but secondary — e.g. metric labels, section sub-headers.
export const BodyStrongMuted: Story = { args: { variant: "body-strong-muted" } };

export const Caption: Story = { args: { variant: "caption" } };

// caption-strong: bold caption for key-value labels, property names in metadata rows.
export const CaptionStrong: Story = { args: { variant: "caption-strong" } };

// caption-mono: monospace caption for paths, IDs, hashes in secondary metadata.
export const CaptionMono: Story = { args: { variant: "caption-mono" } };

// caption-tabular: monospace tabular-nums caption for timestamps, durations, and numeric columns.
export const CaptionTabular: Story = { args: { variant: "caption-tabular" } };

export const Label: Story = { args: { variant: "label" } };
export const Overline: Story = { args: { variant: "overline" } };

// overline-muted: muted overline for section dividers and category labels in sidebars.
export const OverlineMuted: Story = { args: { variant: "overline-muted" } };

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

// lineClamp stories — truncate wins when both are set.
export const LineClamp2: Story = { args: { variant: "body", lineClamp: 2 } };
export const LineClamp3: Story = { args: { variant: "body", lineClamp: 3 } };
export const TruncateWinsOverLineClamp: Story = {
	args: { variant: "body", truncate: true, lineClamp: 2 },
};
