#!/bin/bash
set -e

echo "🔧 Installing karabiner-rcmd-binder..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed"
    echo "Install Rust from https://rustup.rs/ and try again"
    exit 1
fi

# Build release binary
echo "📦 Building release binary..."
cargo build --release

# Install to ~/.local/bin
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "📥 Installing rcmdb to $INSTALL_DIR..."
cp target/release/rcmdb "$INSTALL_DIR/rcmdb"
chmod +x "$INSTALL_DIR/rcmdb"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  $HOME/.local/bin is not in your PATH"
    echo ""
    echo "Add this to your shell config (~/.zshrc or ~/.bashrc):"
    echo ""
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "Then reload your shell:"
    echo "    source ~/.zshrc  # or source ~/.bashrc"
else
    echo "✅ $HOME/.local/bin is already in PATH"
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "Usage:"
echo "  rcmdb          # Run the TUI"
echo "  rcmdb --help   # Show help (future)"
echo ""
