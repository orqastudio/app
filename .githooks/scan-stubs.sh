#!/usr/bin/env bash
# Stub scanner for pre-commit hook.
# Scans staged production files for forbidden patterns:
#   - TODO, FIXME, HACK comments
#   - console.log in TypeScript/Svelte production files
#   - dbg! macro in Rust production files
#   - unwrap() in Rust production files (excludes embedded test modules)
#
# Excludes: test files, .orqa/, .githooks/, node_modules/, target/
#
# Usage: bash scan-stubs.sh [files...]
# Exit code 0 = clean, 1 = violations found.

set -euo pipefail

VIOLATIONS=0
HOOK_DIR="$(cd "$(dirname "$0")" && pwd)"

# AWK program stored in a variable to avoid bash history-expansion of '!'.
# Detects the start line of each Rust test module (marked with #[cfg(test)]).
# The test module is assumed to run to EOF (standard Rust convention: test
# modules are always the last thing in a file). If a file has multiple test
# modules, the earliest start line is used — everything from there to EOF is
# treated as test code.
# Outputs a single line number: the line where the test module begins.
read -r -d '' AWK_RUST_TEST_START <<'AWKEOF' || true
BEGIN { pending=0; found=0 }
/\[cfg\(test\)\]/ { pending=1; next }
pending && /^[[:space:]]*mod[[:space:]]/ {
    if (!found) { print NR; found=1 }
    pending=0; next
}
pending && /^[[:space:]]*#/ { next }
pending { pending=0 }
AWKEOF

# Determine which files to scan: args or all staged production files.
if [ "$#" -gt 0 ]; then
  FILES=("$@")
else
  mapfile -t FILES < <(git diff --cached --name-only --diff-filter=ACMR -- '*.rs' '*.ts' '*.svelte' '*.js' 2>/dev/null || true)
fi

# Returns 0 (true in bash) if the file should be EXCLUDED.
is_excluded() {
  local file="$1"
  [[ "$file" == *test* ]] && return 0
  [[ "$file" == *spec* ]] && return 0
  [[ "$file" == *.test.* ]] && return 0
  [[ "$file" == backend/src-tauri/src/*/tests/* ]] && return 0
  [[ "$file" == backend/src-tauri/src/tests/* ]] && return 0
  [[ "$file" == tests/* ]] && return 0
  [[ "$file" == .orqa/* ]] && return 0
  [[ "$file" == .githooks/* ]] && return 0
  [[ "$file" == node_modules/* ]] && return 0
  [[ "$file" == target/* ]] && return 0
  return 1
}

# Scan a file for a pattern, report violations.
# Arguments: file grep-pattern label
scan_pattern() {
  local file="$1"
  local pattern="$2"
  local label="$3"

  [ -f "$file" ] || return 0

  while IFS=: read -r lineno content; do
    echo "STUB: $file:$lineno — $label found: $content"
    VIOLATIONS=$((VIOLATIONS + 1))
  done < <(grep -nP "$pattern" "$file" 2>/dev/null || true)
}

# Like scan_pattern but skips lines at or after the start of a Rust test module.
# Rust test modules (marked with #[cfg(test)]) are conventionally placed at
# the end of the file, so we find the first test module start line and skip
# everything from that line onwards.
scan_pattern_rust_skiptest() {
  local file="$1"
  local pattern="$2"
  local label="$3"

  [ -f "$file" ] || return 0

  # Find the first test module start line (0 means no test module found)
  local test_start
  test_start=$(awk "$AWK_RUST_TEST_START" "$file" 2>/dev/null | head -1 || true)
  test_start="${test_start:-0}"

  while IFS=: read -r lineno content; do
    if [ "$test_start" -gt 0 ] && [ "$lineno" -ge "$test_start" ]; then
      continue
    fi
    echo "STUB: $file:$lineno — $label found: $content"
    VIOLATIONS=$((VIOLATIONS + 1))
  done < <(grep -nP "$pattern" "$file" 2>/dev/null || true)
}

for file in "${FILES[@]}"; do
  # Normalise Windows paths (MINGW64 may give backslashes)
  file="${file//\\//}"

  is_excluded "$file" && continue

  case "$file" in
    *.rs)
      scan_pattern "$file" '//[[:space:]]*(TODO|FIXME|HACK)\b' 'TODO/FIXME/HACK comment'
      scan_pattern_rust_skiptest "$file" '\bdbg!\s*\(' 'dbg! macro'
      scan_pattern_rust_skiptest "$file" '\.unwrap\s*\(' 'unwrap()'
      ;;
    *.ts|*.svelte|*.js)
      scan_pattern "$file" '//[[:space:]]*(TODO|FIXME|HACK)\b' 'TODO/FIXME/HACK comment'
      scan_pattern "$file" '\bconsole\.log\s*\(' 'console.log'
      ;;
  esac
done

if [ "$VIOLATIONS" -gt 0 ]; then
  echo ""
  echo "Stub scanner: $VIOLATIONS violation(s) found in production code."
  echo "Fix the violations or move them to TODO.md before committing."
  exit 1
fi

exit 0
