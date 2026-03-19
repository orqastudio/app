// Composed single-export (default usage)
export { default as Resizable, type ResizableProps } from "./SimpleResizable.svelte";

// Parts for custom composition
export { default as ResizablePaneGroup } from "./resizable-pane-group.svelte";
export { default as ResizableHandle } from "./resizable-handle.svelte";
export { Pane as ResizablePane } from "paneforge";
