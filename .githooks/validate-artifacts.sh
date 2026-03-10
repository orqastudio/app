#!/usr/bin/env bash
# Artifact schema validation for pre-commit hook
# Validates YAML frontmatter of .orqa/ markdown files against the canonical schema.
# Source of truth: .orqa/documentation/product/artifact-framework.md

set -euo pipefail

ERRORS=0

# Extract ONLY the first frontmatter block (between first and second --- lines)
# Uses awk to stop after the closing --- to avoid matching YAML code blocks in the body
get_frontmatter() {
  awk 'NR==1 && /^---$/ { found=1; next } found && /^---$/ { exit } found { print }' "$1"
}

# Extract frontmatter field names from a markdown file (keys only)
get_frontmatter_keys() {
  get_frontmatter "$1" | grep '^[a-z_-]*:' | cut -d: -f1
}

# Get a specific frontmatter field value
get_field() {
  get_frontmatter "$1" | grep "^$2:" | head -1 | sed "s/^$2: *//"
}

# Check that all required fields exist
check_required() {
  local file=$1; shift
  local keys=$(get_frontmatter_keys "$file")
  for field in "$@"; do
    if ! echo "$keys" | grep -qx "$field"; then
      echo "ERROR: $file — missing required field '$field'"
      ERRORS=$((ERRORS + 1))
    fi
  done
}

# Check that status value is valid
check_status() {
  local file=$1; shift
  local status=$(get_field "$file" "status" | tr -d '"' | tr -d "'")
  if [ -z "$status" ]; then return; fi
  local valid=0
  for allowed in "$@"; do
    [ "$status" = "$allowed" ] && valid=1
  done
  if [ $valid -eq 0 ]; then
    echo "ERROR: $file — invalid status '$status' (allowed: $*)"
    ERRORS=$((ERRORS + 1))
  fi
}

# Check for forbidden fields
check_forbidden() {
  local file=$1; shift
  local keys=$(get_frontmatter_keys "$file")
  for field in "$@"; do
    if echo "$keys" | grep -qx "$field"; then
      echo "ERROR: $file — forbidden field '$field' (not in schema)"
      ERRORS=$((ERRORS + 1))
    fi
  done
}

# Check for duplicate keys in frontmatter
check_duplicates() {
  local file=$1
  local dupes=$(get_frontmatter "$file" | grep '^[a-z_-]*:' | cut -d: -f1 | sort | uniq -d)
  if [ -n "$dupes" ]; then
    echo "ERROR: $file — duplicate frontmatter keys: $dupes"
    ERRORS=$((ERRORS + 1))
  fi
}

# Common forbidden fields (removed from all types)
COMMON_FORBIDDEN="tags category type date enforcement"

# Validate a single file based on its ID prefix
validate_file() {
  local file=$1
  local basename=$(basename "$file" .md)

  # Skip READMEs
  [ "$basename" = "README" ] && return

  # Check for duplicates on all files
  check_duplicates "$file"

  # Check for common forbidden fields
  check_forbidden "$file" $COMMON_FORBIDDEN

  case "$basename" in
    PILLAR-*)
      check_required "$file" id title status description test-questions created updated
      check_status "$file" active inactive
      ;;
    MS-*)
      check_required "$file" id title status description created updated gate
      check_status "$file" planning active complete
      ;;
    EPIC-*)
      check_required "$file" id title status priority milestone pillars description created updated scoring
      check_status "$file" draft ready in-progress review done
      check_forbidden "$file" assignee roadmap-ref score pillar
      ;;
    TASK-*)
      check_required "$file" id title status epic description created updated
      check_status "$file" todo in-progress done
      check_forbidden "$file" phase
      ;;
    IDEA-*)
      check_required "$file" id title status pillars description created updated
      check_status "$file" captured exploring shaped promoted archived
      check_forbidden "$file" pillar
      ;;
    IMPL-*)
      check_required "$file" id title status description created updated recurrence
      check_status "$file" active recurring promoted
      ;;
    RES-*)
      check_required "$file" id title status description created updated
      check_status "$file" draft complete surpassed
      check_forbidden "$file" milestone owner pillar priority scope research produces produces_decisions informs_phases informs_features questions open_questions phases completed_phases blocks depends-on epic epic-ref informs_epics roadmap-ref research-refs
      ;;
    AD-*)
      check_required "$file" id title status description created updated
      check_status "$file" proposed accepted superseded deprecated
      check_forbidden "$file" team test-questions priority
      ;;
    RULE-*)
      check_required "$file" id slug layer status scope title description created updated
      check_status "$file" active inactive
      ;;
    *)
      # Unknown artifact type — skip
      ;;
  esac
}

# Main: validate all files passed as arguments
for file in "$@"; do
  [ -f "$file" ] && validate_file "$file"
done

if [ $ERRORS -gt 0 ]; then
  echo ""
  echo "Schema validation failed: $ERRORS error(s) found."
  echo "See .orqa/documentation/product/artifact-framework.md for the canonical schema."
  exit 1
fi
