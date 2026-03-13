#!/usr/bin/env bash
# Blocks deletion of research and task files (RULE-014: historical preservation).
# Research and tasks are historical records — mark as surpassed, never delete.
#
# Called from pre-commit hook.

set -euo pipefail

DELETED_HISTORICAL=$(git diff --cached --name-only --diff-filter=D -- \
  '.orqa/delivery/research/RES-*.md' \
  '.orqa/delivery/tasks/TASK-*.md' \
  '.orqa/delivery/ideas/IDEA-*.md' \
  '.orqa/process/lessons/IMPL-*.md' 2>/dev/null || true)

if [ -n "$DELETED_HISTORICAL" ]; then
  echo ""
  echo "  ERROR: Historical artifacts cannot be deleted (RULE-014):"
  echo ""
  echo "$DELETED_HISTORICAL" | while IFS= read -r f; do
    echo "    $f"
  done
  echo ""
  echo "  Mark as surpassed/archived instead of deleting."
  echo "  Use 'status: surpassed' and 'surpassed-by: <ID>' in frontmatter."
  exit 1
fi
