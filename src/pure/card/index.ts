// Composed single-export (default usage)
export { default as Card, type CardProps } from "./SimpleCard.svelte";

// Parts for custom composition
export { default as CardRoot } from "./card.svelte";
export { default as CardHeader } from "./card-header.svelte";
export { default as CardTitle } from "./card-title.svelte";
export { default as CardDescription } from "./card-description.svelte";
export { default as CardContent } from "./card-content.svelte";
export { default as CardFooter } from "./card-footer.svelte";
export { default as CardAction } from "./card-action.svelte";
