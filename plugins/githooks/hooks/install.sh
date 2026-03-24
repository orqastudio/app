#!/usr/bin/env bash
# Install OrqaStudio git hooks into the project.
#
# Sets git config core.hooksPath to point at this plugin's hooks directory.
# Preserves any existing hooks by checking for conflicts first.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check for existing hooks path
CURRENT=$(git config core.hooksPath 2>/dev/null || echo "")

if [ -n "$CURRENT" ] && [ "$CURRENT" != "$SCRIPT_DIR" ]; then
  echo "WARNING: git hooks path already set to: $CURRENT"
  echo "Overwrite with OrqaStudio hooks? (y/N)"
  read -r response
  if [ "$response" != "y" ] && [ "$response" != "Y" ]; then
    echo "Aborted."
    exit 1
  fi
fi

git config core.hooksPath "$SCRIPT_DIR"
chmod +x "$SCRIPT_DIR/pre-commit"
chmod +x "$SCRIPT_DIR/post-commit"

echo "Git hooks installed: $SCRIPT_DIR"
echo "Pre-commit hook will validate governance artifacts on every commit."
echo "Post-commit hook will auto-push to remote after every commit."
