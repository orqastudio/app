#!/usr/bin/env bash
# OrqaStudio Git Hooks Plugin — pre-commit enforcement
#
# Schema-driven validation for governance artifacts. Runs orqa validate
# on any staged .orqa/ or plugin artifact files. Blocks commits with
# validation errors.
#
# Install: git config core.hooksPath plugins/githooks/hooks
# Or: copy/symlink this file to .git/hooks/pre-commit

set -euo pipefail

# Find project root (walk up to find .orqa/)
find_root() {
  local dir="$PWD"
  while [ "$dir" != "/" ]; do
    if [ -d "$dir/.orqa" ]; then
      echo "$dir"
      return
    fi
    dir="$(dirname "$dir")"
  done
  echo "$PWD"
}

PROJECT_ROOT=$(find_root)

# Check if orqa CLI is available
if ! command -v orqa &> /dev/null; then
  # Try node-based fallback
  if [ -f "$PROJECT_ROOT/libs/cli/dist/cli.js" ]; then
    ORQA="node $PROJECT_ROOT/libs/cli/dist/cli.js"
  else
    echo "WARNING: orqa CLI not available — skipping artifact validation"
    exit 0
  fi
else
  ORQA="orqa"
fi

# Check if any governance artifacts are staged
ARTIFACT_CHANGED=$(git diff --cached --name-only --diff-filter=ACMR -- \
  '*.md' \
  | grep -E '^(\.orqa/|plugins/[^/]+/(agents|rules|knowledge|documentation)/|connectors/[^/]+/knowledge/)' \
  | head -1 || true)

if [ -z "$ARTIFACT_CHANGED" ]; then
  exit 0
fi

echo "=== Pre-commit: Validating governance artifacts ==="

# Run schema-driven validation
RESULT=$($ORQA validate "$PROJECT_ROOT" --json 2>/dev/null || echo '{"errors":0}')
ERRORS=$(echo "$RESULT" | node -e "process.stdin.on('data',d=>{try{console.log(JSON.parse(d).errors||0)}catch{console.log(0)}})" 2>/dev/null || echo "0")

if [ "$ERRORS" -gt 0 ]; then
  echo ""
  echo "BLOCKED: $ERRORS validation error(s) found in governance artifacts."
  echo ""
  echo "Run 'orqa validate' to see details."
  echo "Run 'orqa validate --fix' to auto-fix where possible."
  echo ""
  echo "To bypass (not recommended): git commit --no-verify"
  exit 1
fi

# JSON Schema validation of frontmatter
PLUGIN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [ -f "$PLUGIN_DIR/hooks/validate-frontmatter.mjs" ]; then
  echo "--- Frontmatter schema validation ---"
  if ! node "$PLUGIN_DIR/hooks/validate-frontmatter.mjs"; then
    echo ""
    echo "Fix frontmatter errors before committing."
    exit 1
  fi
fi

# Check filename-to-ID alignment on staged artifacts
MISMATCHED=""
for file in $(git diff --cached --name-only --diff-filter=ACMR -- '*.md' | grep -E '^(\.orqa/|plugins/|connectors/)' || true); do
  [ -f "$file" ] || continue
  ID=$(head -10 "$file" | grep '^id:' | sed 's/id: *"*//' | sed 's/"*$//' | head -1)
  [ -z "$ID" ] && continue
  BASENAME=$(basename "$file" .md)
  if [ "$BASENAME" != "$ID" ]; then
    MISMATCHED="$MISMATCHED  $file (filename: $BASENAME, id: $ID)\n"
  fi
done

if [ -n "$MISMATCHED" ]; then
  echo ""
  echo "WARNING: Filename-to-ID mismatches found:"
  echo -e "$MISMATCHED"
  echo "Filenames should match their frontmatter ID."
  echo "Rename with: mv <file> <directory>/<ID>.md"
  echo ""
  # Warning only, not blocking — rename is a separate step
fi

echo "=== Pre-commit: Artifact validation passed ==="
