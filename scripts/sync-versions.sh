#!/usr/bin/env bash
# sync-versions.sh — Synchronise the canonical VERSION across all submodules.
#
# Reads VERSION from the dev repo root and updates every package.json,
# orqa-plugin.json, Cargo.toml, and plugin.json in every submodule.
#
# Usage:
#   bash scripts/sync-versions.sh              # Use VERSION file
#   bash scripts/sync-versions.sh 0.2.0-dev    # Override version

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="${1:-$(cat "$ROOT/VERSION" | tr -d '[:space:]')}"

echo "=== Syncing version: $VERSION ==="

# Helper: update "version" field in a JSON file
update_json_version() {
  local file="$1"
  if [ ! -f "$file" ]; then return; fi
  node -e "
    const fs = require('fs');
    const f = process.argv[1];
    const v = process.argv[2];
    const data = JSON.parse(fs.readFileSync(f, 'utf-8'));
    if (data.version !== undefined && data.version !== v) {
      data.version = v;
      fs.writeFileSync(f, JSON.stringify(data, null, 2) + '\n');
      console.log('  Updated: ' + f);
    }
  " "$file" "$VERSION"
}

# Helper: update version in Cargo.toml
update_cargo_version() {
  local file="$1"
  if [ ! -f "$file" ]; then return; fi
  sed -i "s/^version = \".*\"/version = \"$VERSION\"/" "$file"
  echo "  Updated: $file"
}

# Helper: update @orqastudio/* dependency versions in package.json
update_orqa_deps() {
  local file="$1"
  if [ ! -f "$file" ]; then return; fi
  node -e "
    const fs = require('fs');
    const f = process.argv[1];
    const v = process.argv[2];
    const data = JSON.parse(fs.readFileSync(f, 'utf-8'));
    let changed = false;
    for (const section of ['dependencies', 'devDependencies', 'peerDependencies']) {
      if (!data[section]) continue;
      for (const [key, val] of Object.entries(data[section])) {
        if (key.startsWith('@orqastudio/') && val !== v) {
          data[section][key] = v;
          changed = true;
        }
      }
    }
    if (changed) {
      fs.writeFileSync(f, JSON.stringify(data, null, 2) + '\n');
      console.log('  Updated deps: ' + f);
    }
  " "$file" "$VERSION"
}

echo ""
echo "--- Updating library versions ---"

# TypeScript libraries — find all package.json under libs/
for pkg in "$ROOT"/libs/*/package.json; do
  [ -f "$pkg" ] || continue
  update_json_version "$pkg"
  update_orqa_deps "$pkg"
done

echo ""
echo "--- Updating app version ---"
update_json_version "$ROOT/app/package.json"
update_orqa_deps "$ROOT/app/package.json"
update_cargo_version "$ROOT/app/src-tauri/Cargo.toml"

echo ""
echo "--- Updating plugin versions ---"
for plugin_dir in "$ROOT"/plugins/*/; do
  [ -d "$plugin_dir" ] || continue
  update_json_version "${plugin_dir}orqa-plugin.json"
  update_json_version "${plugin_dir}package.json"
  update_json_version "${plugin_dir}.claude-plugin/plugin.json"
  update_orqa_deps "${plugin_dir}package.json"
done

echo ""
echo "=== Version sync complete: $VERSION ==="
echo ""
echo "Next steps:"
echo "  1. Review changes: make submodule-status"
echo "  2. Commit in each submodule: make commit-all"
echo "  3. Push everything: make push-all"
