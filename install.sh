#!/usr/bin/env bash
set -euo pipefail

echo "rebound - 0xClumzZy's Arch Linux Setup"
echo "======================================="

if ! command -v pacman &>/dev/null; then
    echo "Error: This tool requires Arch Linux (pacman not found)"
    exit 1
fi

if ! command -v rustup &>/dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "Building rebound..."
TMPDIR=$(mktemp -d)
git clone https://github.com/0xClumzzy/rebound-.git "$TMPDIR"
cd "$TMPDIR"
cargo build --release

echo "Installing..."
mkdir -p "$HOME/.local/bin"
cp target/release/rebound "$HOME/.local/bin/"
chmod +x "$HOME/.local/bin/rebound"

rm -rf "$TMPDIR"

echo ""
echo "Done! Run 'rebound' to get started."
echo "Make sure ~/.local/bin is in your PATH."
