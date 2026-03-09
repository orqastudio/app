/** README frontmatter for navigation discovery */
export interface NavReadme {
	role: string | null;
	label: string | null;
	description: string | null;
	icon: string | null;
	sort: number | null;
}

/** The full navigation tree returned by artifact_scan_tree */
export interface NavTree {
	groups: NavGroup[];
}

/** A group folder (e.g. Planning, Governance) */
export interface NavGroup {
	label: string;
	description: string;
	icon: string;
	sort: number;
	path: string;
	readme_content: string;
	types: NavType[];
}

/** An artifact type folder (e.g. Epics, Rules) */
export interface NavType {
	label: string;
	description: string;
	icon: string;
	sort: number;
	path: string;
	readme_content: string;
	nodes: DocNode[];
}

/** A node in a NavType's file list */
export interface DocNode {
	/** Display name: filename without .md, hyphens replaced with spaces, title-cased. */
	label: string;
	/** Relative path from project root without .md extension. Null for directories. */
	path: string | null;
	/** Child nodes for directories. Null for leaf files. */
	children: DocNode[] | null;
	/** Status value from YAML frontmatter (e.g. "draft", "in-progress", "done"). Null for directories. */
	status?: string | null;
	/** Optional short description shown below the label for flat-list items. */
	description?: string | null;
}
