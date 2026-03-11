#!/usr/bin/env bash
# Task dependency gate validator for pre-commit hook.
# Checks that when a task's status is changed to "in-progress", all tasks
# listed in its depends-on field have status: done.
#
# Usage: bash validate-task-deps.sh [task-files...]
# Exit code 0 = all gates satisfied, 1 = unmet dependency found.

set -euo pipefail

ERRORS=0
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TASKS_DIR="$REPO_ROOT/.orqa/planning/tasks"

# Determine which task files to check: args or all staged task files.
if [ "$#" -gt 0 ]; then
  FILES=("$@")
else
  mapfile -t FILES < <(git diff --cached --name-only --diff-filter=ACMR -- '.orqa/planning/tasks/TASK-*.md' 2>/dev/null || true)
fi

# AWK program: extract frontmatter block (between first two --- delimiters).
AWK_FM='BEGIN{d=0} /^---$/{d++;if(d==2){stop=1;next}} stop{next} d==1{print}'

# AWK program: extract status from frontmatter.
AWK_STATUS='BEGIN{d=0} /^---$/{d++;if(d==2){stop=1;next}} stop{next} d==1 && /^status:/{sub("^status:[[:space:]]*","");gsub("\"","");print;stop=1}'

# Extract the frontmatter block from a file.
extract_frontmatter() {
  awk "$AWK_FM" "$1"
}

# Extract a YAML scalar field value from frontmatter text on stdin.
extract_field() {
  local field="$1"
  grep -P "^${field}:" | head -1 | sed "s/^${field}:[[:space:]]*//" | tr -d "\"'"
}

# Extract a YAML list field from frontmatter text on stdin.
# Returns one item per line. Handles inline ([A, B]) and block (- A\n- B) forms.
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
  # Use grep-based approach to avoid complex awk quoting issues.
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

for file in "${FILES[@]}"; do
  # Normalise Windows paths
  file="${file//\\//}"

  # Only process task files in the tasks directory
  [[ "$file" == .orqa/planning/tasks/TASK-*.md ]] || continue

  local_file="$REPO_ROOT/$file"
  [ -f "$local_file" ] || continue

  # Check if the staged version of this file has status: in-progress
  staged_status=$(git show ":$file" 2>/dev/null | awk "$AWK_STATUS" || true)

  [ "$staged_status" = "in-progress" ] || continue

  # This task is moving to in-progress — check its depends-on list
  task_id=$(basename "$file" .md)
  fm=$(extract_frontmatter "$local_file")
  mapfile -t deps < <(printf '%s\n' "$fm" | extract_list "depends-on" || true)

  if [ "${#deps[@]}" -eq 0 ]; then
    continue
  fi

  for dep in "${deps[@]}"; do
    dep="${dep//[[:space:]]/}"
    [ -n "$dep" ] || continue

    dep_file="$TASKS_DIR/${dep}.md"
    if [ ! -f "$dep_file" ]; then
      echo "WARNING: $task_id depends on $dep but $dep.md does not exist — skipping dependency check"
      continue
    fi

    dep_fm=$(extract_frontmatter "$dep_file")
    dep_status=$(printf '%s\n' "$dep_fm" | extract_field "status")
    if [ "$dep_status" != "done" ]; then
      echo "ERROR: $task_id cannot move to in-progress — dependency $dep has status: ${dep_status:-unknown} (must be done)"
      ERRORS=$((ERRORS + 1))
    fi
  done
done

if [ "$ERRORS" -gt 0 ]; then
  echo ""
  echo "Task dependency gate: $ERRORS unmet dependency(ies) found."
  echo "All tasks listed in depends-on must have status: done before a task can be set to in-progress."
  exit 1
fi

exit 0
