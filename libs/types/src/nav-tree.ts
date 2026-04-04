/** README frontmatter for navigation discovery */
export interface NavReadme {
	readonly role: string | null;
	readonly label: string | null;
	readonly description: string | null;
	readonly icon: string | null;
	readonly sort: number | null;
}

/** The full navigation tree returned by artifact_scan_tree */
export interface NavTree {
	readonly groups: readonly NavGroup[];
}

/** A group folder (e.g. Planning, Governance) */
export interface NavGroup {
	readonly label: string;
	readonly description: string;
	readonly icon: string;
	readonly sort: number;
	readonly path: string;
	readonly readme_content: string;
	readonly types: readonly NavType[];
}

/** An artifact type folder (e.g. Epics, Rules) */
export interface NavType {
	readonly label: string;
	readonly description: string;
	readonly icon: string;
	readonly sort: number;
	readonly path: string;
	readonly readme_content: string;
	readonly nodes: readonly NavDocNode[];
	readonly filterable_fields: readonly FilterableField[];
	readonly sortable_fields: readonly SortableField[];
	readonly navigation_config?: NavigationConfig;
}

/** A node in a NavType's file list */
export interface NavDocNode {
	/** Display name: filename without .md, hyphens replaced with spaces, title-cased. */
	readonly label: string;
	/** Relative path from project root without .md extension. Null for directories. */
	readonly path: string | null;
	/** Child nodes for directories. Null for leaf files. */
	readonly children: readonly NavDocNode[] | null;
	/** Status value from YAML frontmatter (e.g. "draft", "in-progress", "done"). Null for directories. */
	readonly status?: string | null;
	/** Optional short description shown below the label for flat-list items. */
	readonly description?: string | null;
	/** Icon name from README frontmatter, for directory nodes only. Null for leaf files. */
	readonly icon?: string | null;
	/** Full YAML frontmatter parsed from the file. Null for directories. */
	readonly frontmatter?: Readonly<Record<string, unknown>>;
}

/** Alias for NavDocNode — used by the frontend nav-tree types. */
export type DocNode = NavDocNode;

/** A filterable field derived from a JSON Schema enum property. */
export interface FilterableField {
	readonly name: string;
	readonly values: readonly string[];
}

/** A sortable field derived from a JSON Schema date or string property. */
export interface SortableField {
	readonly name: string;
	readonly field_type: string;
}

/** Default sort configuration. */
export interface SortConfig {
	readonly field: string;
	readonly direction: string;
}

/** A labelled section in a layout-based navigation view. */
export interface LayoutSection {
	readonly label: string;
	readonly description?: string;
	readonly items: readonly string[];
}

/** Layout configuration for a navigation type. */
export interface NavigationLayout {
	readonly sections: readonly LayoutSection[];
	readonly uncategorized?: string;
}

/** Default navigation behaviour for a type (sort, group, filters). */
export interface NavigationDefaults {
	readonly sort?: SortConfig;
	readonly group?: string;
	readonly group_order?: Readonly<Record<string, readonly string[]>>;
	readonly filters?: Readonly<Record<string, readonly string[]>>;
	readonly collapsed_groups?: readonly string[];
}

/** Navigation configuration loaded from _navigation.json in a type directory. */
export interface NavigationConfig {
	readonly defaults?: NavigationDefaults;
	readonly layout?: NavigationLayout;
}

/** Client-side view state for an artifact type. */
export interface ArtifactViewState {
	readonly sort: SortConfig;
	readonly filters: Readonly<Record<string, readonly string[]>>;
	readonly group: string | null;
}
