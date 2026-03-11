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

# Determine which files to scan: args or all staged production files.
if [ "$#" -gt 0 ]; then
  FILES=("$@")
else
  mapfile -t FILES < <(git diff --cached --name-only --diff-filter=ACMR -- '*.rs' '*.ts' '*.svelte' '*.js' 2>/dev/null || true)
fi

# Returns 0 (true in bash) if the file should be EXCLUDED.
is_excluded() {
  local file="$1"
  # Exclude test files by name
  [[ "$file" == *test* ]] && return 0
  [[ "$file" == *spec* ]] && return 0
  [[ "$file" == *.test.* ]] && return 0
  # Exclude test directories
  [[ "$file" == src-tauri/src/*/tests/* ]] && return 0
  [[ "$file" == src-tauri/src/tests/* ]] && return 0
  [[ "$file" == tests/* ]] && return 0
  # Exclude governance and hook files
  [[ "$file" == .orqa/* ]] && return 0
  [[ "$file" == .githooks/* ]] && return 0
  # Exclude build artifacts and deps
  [[ "$file" == node_modules/* ]] && return 0
  [[ "$file" == target/* ]] && return 0
  return 1
}

# Scan a file for a pattern, reporting violations.
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

# Like scan_pattern but skips lines that fall inside Rust test modules.
# Rust test modules begin with "#[cfg(test)]" or "mod tests {" / "mod test {".
# They end when the matching closing brace for the module is reached.
# We use a simple heuristic: skip lines between a test-module start and a
# line that is just "}" at the same or lesser indentation.
scan_pattern_rust_skiptest() {
  local file="$1"
  local pattern="$2"
  local label="$3"

  [ -f "$file" ] || return 0

  # Collect test-module line ranges using awk.
  # Handles both single-line and two-line forms:
  #   Single:  #[cfg(test)]
  #            mod tests {
  #   Two-line is the common pattern in this codebase.
  # Also handles: mod tests { (without prior cfg annotation).
  # Counts braces to find the end of the module.
  # Output: "start_line end_line" pairs, one per test module.
  local test_ranges
  test_ranges=$(awk '
    BEGIN { pending_cfg=0; in_mod=0; depth=0; start=0 }
    /#\[cfg\(test\)\]/ { pending_cfg=1 }
    /^[[:space:]]*mod[[:space:]]+(tests?|test_[a-zA-Z0-9_]*)[[:space:]]*(;|\{)/ {
      if (!in_mod) {
        in_mod=1; start=NR; depth=0
        # Count opening braces on this line
        n=split($0,chars,"")
        for(i=1;i<=n;i++){
          if(chars[i]=="{") depth++
          if(chars[i]=="}") depth--
        }
        if(depth==0 && index($0,"{")==0){ in_mod=0 }
        pending_cfg=0
        next
      }
    }
    !/^[[:space:]]*#/ && !/^[[:space:]]*$/ && pending_cfg { pending_cfg=0 }
    in_mod {
      n=split($0,chars,"")
      for(i=1;i<=n;i++){
        if(chars[i]=="{") depth++
        if(chars[i]=="}") depth--
      }
      if(depth<=0){ print start " " NR; in_mod=0; start=0; depth=0 }
    }
  ' "$file" 2>/dev/null || true)

  # Now scan for the pattern, skipping lines in test ranges.
  while IFS=: read -r lineno content; do
    local in_test=0
    while IFS= read -r range; do
      [ -z "$range" ] && continue
      local rs re
      rs=$(echo "$range" | cut -d' ' -f1)
      re=$(echo "$range" | cut -d' ' -f2)
      if [ "$lineno" -ge "$rs" ] && [ "$lineno" -le "$re" ]; then
        in_test=1
        break
      fi
    done <<< "$test_ranges"
    if [ "$in_test" -eq 0 ]; then
      echo "STUB: $file:$lineno — $label found: $content"
      VIOLATIONS=$((VIOLATIONS + 1))
    fi
  done < <(grep -nP "$pattern" "$file" 2>/dev/null || true)
}

for file in "${FILES[@]}"; do
  # Normalise Windows paths (MINGW64 may give backslashes)
  file="${file//\\//}"

  is_excluded "$file" && continue

  case "$file" in
    *.rs)
      # Rust production files — TODO/FIXME/HACK in comments (case-insensitive)
      scan_pattern "$file" '//[[:space:]]*(TODO|FIXME|HACK)\b' 'TODO/FIXME/HACK comment'
      scan_pattern_rust_skiptest "$file" '\bdbg!\s*\(' 'dbg! macro'
      scan_pattern_rust_skiptest "$file" '\.unwrap\s*\(' 'unwrap()'
      ;;
    *.ts|*.svelte|*.js)
      # TypeScript/Svelte/JS production files
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
