// Composed single-export (default usage)
export { default as Popover } from "./SimplePopover.svelte";

// Parts for edge cases needing custom composition
export { default as PopoverRoot } from "./popover.svelte";
export { default as PopoverTrigger } from "./popover-trigger.svelte";
export { default as PopoverContent } from "./popover-content.svelte";
export { default as PopoverClose } from "./popover-close.svelte";
export { default as PopoverPortal } from "./popover-portal.svelte";
