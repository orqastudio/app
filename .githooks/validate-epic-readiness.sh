#!/usr/bin/env bash
# Epic readiness (docs-required) gate validator for pre-commit hook.
# Checks that when an epic's status is changed to "ready", all entries in its
# docs-required field exist — either as artifact files or as file paths.
#
# Artifact ID resolution: search .orqa/ for a file with "id: <ID>" in frontmatter.
# File path resolution: check if the path exists relative to repo root.
#
# Usage: bash validate-epic-readiness.sh [epic-files...]
# Exit code 0 = all gates satisfied, 1 = missing required doc found.

set -euo pipefail

ERRORS=0
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Determine which epic files to check: args or all staged epic files.
if [ "$#" -gt 0 ]; then
  FILES=("$@")
else
  mapfile -t FILES < <(git diff --cached --name-only --diff-filter=ACMR -- '.orqa/planning/epics/EPIC-*.md' 2>/dev/null || true)
fi

# AWK program: extract frontmatter block (between first two --- delimiters).
AWK_FM='BEGIN{d=0} /^---$/{d++;if(d==2){exit};next} d==1{print}'

# AWK program: extract status from frontmatter.
AWK_STATUS='BEGIN{d=0} /^---$/{d++;if(d==2){exit};next} d==1 && /^status:/{sub("^status:[[:space:]]*","");gsub("\"","");print;exit}'

# Extract the frontmatter block from a file.
extract_frontmatter() {
  awk "$AWK_FM" "$1"
}

# Extract a YAML list field from frontmatter text on stdin.
# Returns one item per line. Handles inline ([A, B]) and block (- A\n- B) forms.
# Also handles null/~ as empty.
extract_list() {
  local field="$1"
  local fm
  fm=$(cat)

  # null / empty: field: null  or  field: ~  or  field:  (with nothing after)
  local scalar_line
  scalar_line=$(printf '%s\n' "$fm" | grep -P "^${field}:[[:space:]]*(null|~)?$" | head -1 || true)
  if [ -n "$scalar_line" ]; then
    return 0
  fi

  # Inline array: field: [A, B] or field: []
  local inline
  inline=$(printf '%s\n' "$fm" | grep -P "^${field}:\s*\[" | head -1 || true)
  if [ -n "$inline" ]; then
    printf '%s\n' "$inline" \
      | sed "s/^${field}:[[:space:]]*//" \
      | tr -d '[]' \
      | tr ',' '\n' \
      | sed 's/^[[:space:]]*//' \
      | sed 's/[[:space:]]*$//' \
      | grep -v '^$' || true
    return 0
  fi

  # Block list: "  - VALUE" lines following the field key.
  local in_block=0
  while IFS= read -r line; do
    if printf '%s\n' "$line" | grep -qP "^${field}:"; then
      in_block=1
      continue
    fi
    if [ "$in_block" -eq 1 ]; then
      if printf '%s\n' "$line" | grep -qP '^[[:space:]]+-[[:space:]]+'; then
        printf '%s\n' "$line" | sed 's/^[[:space:]]*-[[:space:]]*//'
      elif printf '%s\n' "$line" | grep -qP '^[a-zA-Z]'; then
        break
      fi
    fi
  done <<< "$fm"
  return 0
}

# Check whether a docs-required entry exists.
# Artifact IDs (e.g., DOC-005, RES-012): search .orqa/ for a file with "id: <ID>".
# File paths: check relative to repo root.
check_entry_exists() {
  local entry="$1"

  # Artifact ID: uppercase word(s) + hyphen + digits, no slashes or dots
  if [[ "$entry" =~ ^[A-Z]+-[0-9]+$ ]]; then
    local found
    found=$(grep -rl "^id: ${entry}$" "$REPO_ROOT/.orqa/" 2>/dev/null | head -1 || true)
    [ -n "$found" ]
  else
    [ -f "$REPO_ROOT/$entry" ]
  fi
}

for file in "${FILES[@]}"; do
  # Normalise Windows paths
  file="${file//\\//}"

  # Only process epic files in the epics directory
  [[ "$file" == .orqa/planning/epics/EPIC-*.md ]] || continue

  local_file="$REPO_ROOT/$file"
  [ -f "$local_file" ] || continue

  # Check if the staged version of this file has status: ready
  staged_status=$(git show ":$file" 2>/dev/null | awk "$AWK_STATUS" || true)

  [ "$staged_status" = "ready" ] || continue

  # This epic is moving to ready — check its docs-required list
  epic_id=$(basename "$file" .md)
  fm=$(extract_frontmatter "$local_file")
  mapfile -t docs < <(printf '%s\n' "$fm" | extract_list "docs-required" || true)

  if [ "${#docs[@]}" -eq 0 ]; then
    # Empty or null docs-required — gate automatically satisfied
    continue
  fi

  for doc in "${docs[@]}"; do
    doc="${doc//[[:space:]]/}"
    [ -n "$doc" ] || continue

    if ! check_entry_exists "$doc"; then
      echo "ERROR: $epic_id cannot move to ready — docs-required entry '$doc' does not exist"
      ERRORS=$((ERRORS + 1))
    fi
  done
done

if [ "$ERRORS" -gt 0 ]; then
  echo ""
  echo "Epic readiness gate: $ERRORS missing required document(s) found."
  echo "All entries in docs-required must exist before an epic can be set to ready."
  exit 1
fi

exit 0
