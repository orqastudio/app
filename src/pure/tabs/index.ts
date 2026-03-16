// Composed single-export (default usage)
export { default as Tabs, type TabsProps, type TabDef } from "./SimpleTabs.svelte";

// Parts for custom composition
export { default as TabsRoot } from "./tabs.svelte";
export { default as TabsList } from "./tabs-list.svelte";
export { default as TabsTrigger } from "./tabs-trigger.svelte";
export { default as TabsContent } from "./tabs-content.svelte";
