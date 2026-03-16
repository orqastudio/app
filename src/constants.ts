/**
 * Canonical bidirectional inverse relationship pairs.
 *
 * 10 canonical pairs + self-inverse `synchronised-with`.
 * Project relationships (e.g. `depends-on`/`depended-on-by`) are defined in
 * `project.json` and merged at runtime — they are NOT included here.
 */
export const INVERSE_MAP: ReadonlyMap<string, string> = new Map([
  ["informs", "informed-by"],
  ["informed-by", "informs"],
  ["evolves-into", "evolves-from"],
  ["evolves-from", "evolves-into"],
  ["drives", "driven-by"],
  ["driven-by", "drives"],
  ["governs", "governed-by"],
  ["governed-by", "governs"],
  ["delivers", "delivered-by"],
  ["delivered-by", "delivers"],
  ["enforces", "enforced-by"],
  ["enforced-by", "enforces"],
  ["grounded", "grounded-by"],
  ["grounded-by", "grounded"],
  ["observes", "observed-by"],
  ["observed-by", "observes"],
  ["merged-into", "merged-from"],
  ["merged-from", "merged-into"],
  ["synchronised-with", "synchronised-with"],
]);

/**
 * @deprecated Standalone reference fields are removed in the graph-first model.
 * All references now use the `relationships` frontmatter array.
 * Kept temporarily for downstream migration — will be removed in a future release.
 */
export const SINGLE_REF_FIELDS = [] as const;

/**
 * @deprecated Standalone reference fields are removed in the graph-first model.
 * All references now use the `relationships` frontmatter array.
 * Kept temporarily for downstream migration — will be removed in a future release.
 */
export const ARRAY_REF_FIELDS = [] as const;
