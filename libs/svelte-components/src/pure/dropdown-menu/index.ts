// Composed single-export (default usage)
export {
	default as DropdownMenu,
	type DropdownMenuProps,
	type DropdownMenuItem as DropdownMenuItemDef,
	type DropdownMenuEntry,
	type DropdownMenuSeparator as DropdownMenuSeparatorDef,
} from "./SimpleDropdownMenu.svelte";

// Parts for custom composition
export { default as DropdownMenuRoot } from "./dropdown-menu.svelte";
export { default as DropdownMenuContent } from "./dropdown-menu-content.svelte";
export { default as DropdownMenuTrigger } from "./dropdown-menu-trigger.svelte";
export { default as DropdownMenuItem } from "./dropdown-menu-item.svelte";
export { default as DropdownMenuSeparator } from "./dropdown-menu-separator.svelte";
export { default as DropdownMenuGroup } from "./dropdown-menu-group.svelte";
export { default as DropdownMenuGroupHeading } from "./dropdown-menu-group-heading.svelte";
export { default as DropdownMenuLabel } from "./dropdown-menu-label.svelte";
export { default as DropdownMenuShortcut } from "./dropdown-menu-shortcut.svelte";
export { default as DropdownMenuSub } from "./dropdown-menu-sub.svelte";
export { default as DropdownMenuSubContent } from "./dropdown-menu-sub-content.svelte";
export { default as DropdownMenuSubTrigger } from "./dropdown-menu-sub-trigger.svelte";
export { default as DropdownMenuCheckboxGroup } from "./dropdown-menu-checkbox-group.svelte";
export { default as DropdownMenuCheckboxItem } from "./dropdown-menu-checkbox-item.svelte";
export { default as DropdownMenuRadioGroup } from "./dropdown-menu-radio-group.svelte";
export { default as DropdownMenuRadioItem } from "./dropdown-menu-radio-item.svelte";
export { default as DropdownMenuPortal } from "./dropdown-menu-portal.svelte";
