#!/usr/bin/env bash
# Artifact schema validation for pre-commit hook.
# Delegates to validate-schema.mjs which calls the orqa-validation daemon.
#
# Passes through all arguments including --warn-rules=RULE-032,...

set -euo pipefail

HOOK_DIR="$(cd "$(dirname "$0")" && pwd)"

node "$HOOK_DIR/validate-schema.mjs" "$@"
