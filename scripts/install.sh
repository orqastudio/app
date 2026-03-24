#!/usr/bin/env bash
# install.sh — Zero to orqa on PATH.
#
# Handles a completely fresh machine:
#   1. Ensure Node.js 22+ is available (install via fnm if missing)
#   2. npm install
#   3. npm link @orqastudio/cli (puts orqa on PATH)
#   4. orqa install (sets up everything else)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

NODE_VERSION=22

echo "=== OrqaStudio Bootstrap ==="
echo ""

# ── Ensure Node.js 22+ ──────────────────────────────────────────────────────

ensure_node() {
  # Already have a suitable node?
  if command -v node &>/dev/null; then
    local major
    major=$(node --version | sed 's/v//' | cut -d. -f1)
    if [ "$major" -ge "$NODE_VERSION" ]; then
      echo "  ✓ node $(node --version)"
      return 0
    fi
    echo "  Node.js $(node --version) found but ${NODE_VERSION}+ required."
  else
    echo "  Node.js not found — installing..."
  fi

  # Try fnm first (already installed?)
  if command -v fnm &>/dev/null; then
    echo "  Installing Node ${NODE_VERSION} via fnm..."
    fnm install "$NODE_VERSION" && fnm use "$NODE_VERSION"
    return 0
  fi

  # Try nvm (already installed?)
  if command -v nvm &>/dev/null; then
    echo "  Installing Node ${NODE_VERSION} via nvm..."
    nvm install "$NODE_VERSION" && nvm use "$NODE_VERSION"
    return 0
  fi

  # Neither fnm nor nvm — install fnm then node
  echo "  Installing fnm (fast node manager)..."
  case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*|Windows*)
      if command -v winget &>/dev/null; then
        winget install --id Schniz.fnm --accept-source-agreements --accept-package-agreements
      else
        echo "ERROR: Cannot auto-install Node on Windows without winget."
        echo "Install Node.js ${NODE_VERSION}+ manually: https://nodejs.org/en/download"
        exit 1
      fi
      ;;
    Darwin*)
      if command -v brew &>/dev/null; then
        brew install fnm
      else
        curl -fsSL https://fnm.vercel.app/install | bash
      fi
      ;;
    *)
      curl -fsSL https://fnm.vercel.app/install | bash
      ;;
  esac

  # Source fnm into this shell
  export PATH="$HOME/.local/share/fnm:$HOME/.fnm:$PATH"
  if command -v fnm &>/dev/null; then
    eval "$(fnm env)"
    fnm install "$NODE_VERSION" && fnm use "$NODE_VERSION"
  else
    echo "ERROR: fnm installation failed. Install Node.js ${NODE_VERSION}+ manually:"
    echo "  https://nodejs.org/en/download"
    exit 1
  fi
}

ensure_node

# Verify node is now available
if ! command -v node &>/dev/null; then
  echo "ERROR: Node.js still not available after installation attempt."
  echo "Install Node.js ${NODE_VERSION}+ manually: https://nodejs.org/en/download"
  exit 1
fi

# ── Init submodules (needed to access libs/cli) ─────────────────────────────

echo "  Initialising submodules..."
git submodule update --init --recursive
echo "  ✓ submodules"

# ── Bootstrap the CLI ────────────────────────────────────────────────────────

echo "  Building CLI..."
cd "$ROOT/libs/types" && npm install --ignore-scripts && npx tsc
cd "$ROOT/libs/cli" && npm install --ignore-scripts && npm link @orqastudio/types && npx tsc && npm link

# ── Hand off to orqa install ─────────────────────────────────────────────────

cd "$ROOT"
echo ""
orqa install
