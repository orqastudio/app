// Composed single-export (default usage)
export { default as Dialog, type DialogProps } from "./SimpleDialog.svelte";

// Parts for custom composition
export { default as DialogRoot } from "./dialog.svelte";
export { default as DialogContent } from "./dialog-content.svelte";
export { default as DialogHeader } from "./dialog-header.svelte";
export { default as DialogTitle } from "./dialog-title.svelte";
export { default as DialogDescription } from "./dialog-description.svelte";
export { default as DialogFooter } from "./dialog-footer.svelte";
export { default as DialogOverlay } from "./dialog-overlay.svelte";
export { default as DialogPortal } from "./dialog-portal.svelte";
