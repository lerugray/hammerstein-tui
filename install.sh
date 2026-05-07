#!/usr/bin/env bash
# install.sh — one-line installer for Hammerstein TUI
# curl -fsSL https://raw.githubusercontent.com/lerugray/hammerstein-tui/main/install.sh | bash
set -euo pipefail

BOLD="\033[1m"
GOLD="\033[33m"
GREEN="\033[32m"
RESET="\033[0m"

echo -e "${BOLD}${GOLD}⚡ Hammerstein TUI installer${RESET}"
echo ""

# 1. Check for deepseek-tui
if ! command -v deepseek-tui &>/dev/null; then
    echo "deepseek-tui is required but not found."
    echo "Install it first: brew install deepseek-tui"
    echo "Or: npm install -g deepseek-tui"
    exit 1
fi
echo -e "  ${GREEN}✓${RESET} deepseek-tui found at $(which deepseek-tui)"

# 2. Determine install dir
INSTALL_DIR="${HAMMERSTEIN_INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"

# 3. Find the repo (where this install script lives)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR" && pwd)"

# 4. Install symlinks
WRAPPER="$REPO_DIR/scripts/hammerstein-tui"
DISPATCH="$REPO_DIR/scripts/hammerstein-dispatch"
DISPATCH_MD="$REPO_DIR/scripts/commands/dispatch.md"

if [ ! -f "$WRAPPER" ]; then
    echo "Error: hammerstein-tui wrapper not found at $WRAPPER"
    echo "Make sure you're running install.sh from within the hammerstein-tui repo."
    exit 1
fi

chmod +x "$WRAPPER" "$DISPATCH" 2>/dev/null || true
ln -sf "$WRAPPER" "$INSTALL_DIR/hamt"
ln -sf "$DISPATCH" "$INSTALL_DIR/hamt-dispatch"
echo -e "  ${GREEN}✓${RESET} hamt → hammerstein-tui"

# 5. Config directory
mkdir -p "$HOME/.hammerstein/tui"
echo -e "  ${GREEN}✓${RESET} Config: ~/.hammerstein/tui/"

# 6. Slash command
mkdir -p "$HOME/.deepseek/commands"
ln -sf "$DISPATCH_MD" "$HOME/.deepseek/commands/dispatch.md"
echo -e "  ${GREEN}✓${RESET} /dispatch slash command installed"

# 7. Check PATH
if ! echo "$PATH" | tr ':' '\n' | grep -qF "$INSTALL_DIR"; then
    echo ""
    echo -e "  ${GOLD}⚠${RESET}  Add $INSTALL_DIR to your PATH:"
    echo "      echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
    echo "      source ~/.zshrc"
fi

# 8. Check API key
if [ -z "${HAMMERSTEIN_API_KEY:-}" ] && [ -z "${DEEPSEEK_API_KEY:-}" ]; then
    echo ""
    echo -e "  ${GOLD}⚠${RESET}  No API key found. Set one:"
    echo "      export HAMMERSTEIN_API_KEY=\"sk-...\""
    echo "  Or create ~/.hammerstein/tui/config.toml"
fi

echo ""
echo -e "${BOLD}${GREEN}Done.${RESET}"
echo ""
echo "  Launch:  ${BOLD}hamt${RESET}"
echo "  Config:  ~/.hammerstein/tui/config.toml"
echo "  Help:    hamt --help"
echo ""
echo "  From inside the TUI:"
echo "    /dispatch fix the auth bug"
echo "    !hamt-dispatch \"fix the auth bug\""
