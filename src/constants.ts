/** Relationship type pairs for bidirectional enforcement. */
export const INVERSE_MAP: ReadonlyMap<string, string> = new Map([
  ["observes", "observed-by"],
  ["observed-by", "observes"],
  ["grounded", "grounded-by"],
  ["grounded-by", "grounded"],
  ["practices", "practiced-by"],
  ["practiced-by", "practices"],
  ["enforces", "enforced-by"],
  ["enforced-by", "enforces"],
  ["verifies", "verified-by"],
  ["verified-by", "verifies"],
  ["informs", "informed-by"],
  ["informed-by", "informs"],
  ["scoped-to", "scoped-by"],
  ["scoped-by", "scoped-to"],
  ["documents", "documented-by"],
  ["documented-by", "documents"],
]);

/** Frontmatter fields that hold a single artifact reference. */
export const SINGLE_REF_FIELDS = [
  "milestone",
  "epic",
  "promoted-to",
  "supersedes",
  "superseded-by",
  "surpassed-by",
  "promoted-from",
  "assignee",
] as const;

/** Frontmatter fields that hold an array of artifact references. */
export const ARRAY_REF_FIELDS = [
  "depends-on",
  "blocks",
  "pillars",
  "research-refs",
  "docs-required",
  "docs-produced",
  "skills",
] as const;
