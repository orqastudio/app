#!/usr/bin/env bash
#
# monorepo-merge.sh — Merge all submodule repos into a monorepo using git subtree.
#
# This script:
# 1. Removes all submodule references from the current repo
# 2. Imports each repo via git subtree add (preserving full history)
# 3. Each repo's content lands at its current submodule path
#
# Prerequisites:
# - Run from the dev repo root
# - All submodules must be committed and pushed
# - Working tree must be clean
#
# Usage:
#   bash scripts/monorepo-merge.sh [--dry-run]

set -euo pipefail

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=true
  echo "[DRY RUN] No changes will be made."
fi

# ---------------------------------------------------------------------------
# Submodule definitions: path → GitHub URL
# Order: dependencies first (leaf libs → consuming libs → app → plugins → meta)
# ---------------------------------------------------------------------------

declare -a REPOS=(
  # Tier 1: Leaf libraries
  "libs/types|git@github.com:orqastudio/orqastudio-lib-types.git"
  "libs/logger|git@github.com:orqastudio/orqastudio-lib-logger.git"
  "libs/brand|git@github.com:orqastudio/orqastudio-lib-brand.git"
  "libs/validation|git@github.com:orqastudio/orqastudio-lib-validation.git"
  "libs/search|git@github.com:orqastudio/orqastudio-lib-search.git"

  # Tier 2: Libraries with Tier 1 deps
  "plugins/typescript|git@github.com:orqastudio/orqastudio-plugin-typescript.git"
  "libs/cli|git@github.com:orqastudio/orqastudio-lib-cli.git"
  "libs/sdk|git@github.com:orqastudio/orqastudio-lib-sdk.git"
  "libs/mcp-server|git@github.com:orqastudio/orqastudio-lib-mcp-server.git"
  "libs/lsp-server|git@github.com:orqastudio/orqastudio-lib-lsp-server.git"

  # Tier 3: Libraries with Tier 2 deps
  "libs/svelte-components|git@github.com:orqastudio/orqastudio-lib-svelte-components.git"
  "libs/graph-visualiser|git@github.com:orqastudio/orqastudio-lib-graph-visualiser.git"

  # Tier 4: App
  "app|git@github.com:orqastudio/orqastudio-app.git"

  # Tier 5: Connectors and integrations
  "connectors/claude-code|git@github.com:orqastudio/orqastudio-connector-claude-code.git"
  "integrations/claude-agent-sdk|git@github.com:orqastudio/orqastudio-plugin-claude.git"

  # Tier 6: Content plugins
  "plugins/core|git@github.com:orqastudio/orqastudio-plugin-core-framework.git"
  "plugins/software|git@github.com:orqastudio/orqastudio-plugin-software.git"
  "plugins/cli|git@github.com:orqastudio/orqastudio-plugin-cli.git"
  "plugins/svelte|git@github.com:orqastudio/orqastudio-plugin-svelte.git"
  "plugins/tauri|git@github.com:orqastudio/orqastudio-plugin-tauri.git"
  "plugins/rust|git@github.com:orqastudio/orqastudio-plugin-rust.git"
  "plugins/coding-standards|git@github.com:orqastudio/orqastudio-plugin-coding-standards.git"
  "plugins/agile-governance|git@github.com:orqastudio/orqastudio-plugin-agile-governance.git"
  "plugins/systems-thinking|git@github.com:orqastudio/orqastudio-plugin-systems-thinking.git"
  "plugins/githooks|git@github.com:orqastudio/orqastudio-plugin-githooks.git"

  # Tier 7: Meta
  "registry/official|git@github.com:orqastudio/orqastudio-registry-official.git"
  "registry/community|git@github.com:orqastudio/orqastudio-registry-community.git"
  "templates|git@github.com:orqastudio/orqastudio-templates.git"
  "tools/debug|git@github.com:orqastudio/orqastudio-tool-debug.git"
  ".github-org|git@github.com:orqastudio/.github.git"
)

echo "=== Monorepo Merge: ${#REPOS[@]} repos ==="
echo ""

# ---------------------------------------------------------------------------
# Step 1: Verify clean working tree
# ---------------------------------------------------------------------------

if [[ -n "$(git status --porcelain)" ]]; then
  echo "ERROR: Working tree is not clean. Commit or stash changes first."
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 2: Remove all submodules
# ---------------------------------------------------------------------------

echo "--- Step 1: Removing submodules ---"

for entry in "${REPOS[@]}"; do
  IFS='|' read -r subpath url <<< "$entry"

  if [[ ! -f ".gitmodules" ]] || ! grep -q "path = $subpath" .gitmodules 2>/dev/null; then
    echo "  SKIP (not a submodule): $subpath"
    continue
  fi

  echo "  Removing submodule: $subpath"

  if [[ "$DRY_RUN" == true ]]; then
    continue
  fi

  # Deinit the submodule (removes working tree content and .git/config entry)
  git submodule deinit -f "$subpath" 2>/dev/null || true

  # Remove from index and .gitmodules
  git rm -f "$subpath" 2>/dev/null || true

  # Clean up .git/modules/<path>
  rm -rf ".git/modules/$subpath" 2>/dev/null || true
done

if [[ "$DRY_RUN" == false ]]; then
  # Remove .gitmodules if it's now empty
  if [[ -f ".gitmodules" ]]; then
    git rm -f .gitmodules 2>/dev/null || true
  fi

  # Commit the submodule removal
  git commit -m "Remove all submodules in preparation for monorepo merge

This removes all 30 submodule references. The next commits will import
each repo's content and history via git subtree add.

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"

  echo "  Submodules removed and committed."
fi

echo ""

# ---------------------------------------------------------------------------
# Step 3: Import each repo via git subtree add
# ---------------------------------------------------------------------------

echo "--- Step 2: Importing repos via git subtree ---"

imported=0
failed=0

for entry in "${REPOS[@]}"; do
  IFS='|' read -r subpath url <<< "$entry"

  echo "  [$((imported + failed + 1))/${#REPOS[@]}] $subpath <- $url"

  if [[ "$DRY_RUN" == true ]]; then
    imported=$((imported + 1))
    continue
  fi

  if git subtree add --prefix="$subpath" "$url" main 2>&1; then
    imported=$((imported + 1))
  else
    echo "    FAILED: $subpath"
    failed=$((failed + 1))
  fi
done

echo ""
echo "=== Done: $imported imported, $failed failed ==="

if [[ $failed -gt 0 ]]; then
  echo "WARNING: Some repos failed to import. Check the output above."
  exit 1
fi
