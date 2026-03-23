import type { Meta, StoryObj } from "@storybook/svelte";
import ArtifactLink from "./ArtifactLink.svelte";

const meta = {
	title: "Connected/ArtifactLink",
	component: ArtifactLink,
	tags: ["autodocs"],
} satisfies Meta;

export default meta;
type Story = StoryObj;

export const KnownArtifact: Story = {
	args: { artifactId: "EPIC-001" },
};

export const UnknownArtifact: Story = {
	args: { artifactId: "UNKNOWN-999" },
};
