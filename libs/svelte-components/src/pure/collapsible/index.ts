// Composed single-export (default usage)
export { default as Collapsible } from "./SimpleCollapsible.svelte";

// Parts for edge cases needing custom composition
export { default as CollapsibleRoot } from "./collapsible.svelte";
export { default as CollapsibleTrigger } from "./collapsible-trigger.svelte";
export { default as CollapsibleContent } from "./collapsible-content.svelte";
// Tree-specialised trigger with depth-based padding-left
export { default as TreeCollapsibleTrigger } from "./TreeCollapsibleTrigger.svelte";
// Group header trigger for flat-list sections (chevron + label + optional count)
export { default as CollapsibleGroupHeader } from "./CollapsibleGroupHeader.svelte";
// Bordered card-style section trigger for expandable content panels
export { default as CollapsibleSection } from "./CollapsibleSection.svelte";
