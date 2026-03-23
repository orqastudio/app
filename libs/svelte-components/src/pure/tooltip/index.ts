// Composed single-export (default usage)
export { default as Tooltip } from "./SimpleTooltip.svelte";

// Parts for edge cases needing custom composition
export { default as TooltipRoot } from "./tooltip.svelte";
export { default as TooltipTrigger } from "./tooltip-trigger.svelte";
export { default as TooltipContent } from "./tooltip-content.svelte";
export { default as TooltipProvider } from "./tooltip-provider.svelte";
export { default as TooltipPortal } from "./tooltip-portal.svelte";
