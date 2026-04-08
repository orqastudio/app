import type { Meta, StoryObj } from "@storybook/svelte";
import SearchCard from "./SearchCard.svelte";
import SearchBarInput from "./SearchBarInput.svelte";
import SearchResultItem from "./SearchResultItem.svelte";

// ── SearchCard ────────────────────────────────────────────────────────────────

export const CardMeta = {
	title: "Pure/SearchOverlay/SearchCard",
	component: SearchCard,
	tags: ["autodocs"],
} satisfies Meta;

export default CardMeta;
type CardStory = StoryObj<typeof CardMeta>;

export const CardDefault: CardStory = {
	args: {},
};

// ── SearchBarInput ────────────────────────────────────────────────────────────

export const InputMeta = {
	title: "Pure/SearchOverlay/SearchBarInput",
	component: SearchBarInput,
	tags: ["autodocs"],
	argTypes: {
		value: { control: "text" },
		placeholder: { control: "text" },
	},
} satisfies Meta;

type InputStory = StoryObj<typeof InputMeta>;

export const InputDefault: InputStory = {
	args: { value: "", placeholder: "Search artifacts..." },
};

// ── SearchResultItem ──────────────────────────────────────────────────────────

export const ResultMeta = {
	title: "Pure/SearchOverlay/SearchResultItem",
	component: SearchResultItem,
	tags: ["autodocs"],
	argTypes: {
		active: { control: "boolean" },
	},
} satisfies Meta;

type ResultStory = StoryObj<typeof ResultMeta>;

export const ResultDefault: ResultStory = {
	args: {
		iconName: "file-text",
		id: "EPIC-001",
		title: "Build the authentication system",
		artifactType: "epic",
		active: false,
	},
};

export const ResultActive: ResultStory = {
	args: {
		iconName: "file-text",
		id: "TASK-042",
		title: "Write unit tests for login flow",
		artifactType: "task",
		active: true,
	},
};

export const ResultWithProject: ResultStory = {
	args: {
		iconName: "check-circle",
		project: "orqa",
		id: "EPIC-048",
		title: "Platform migration epic",
		artifactType: "epic",
		active: false,
	},
};
