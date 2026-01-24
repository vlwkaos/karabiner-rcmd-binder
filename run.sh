#!/bin/bash
# Quick run script for karabiner-switch-key

cd "$(dirname "$0")"

# Check if binary exists
if [ ! -f "target/release/karabiner-switch-key" ]; then
    echo "Building karabiner-switch-key..."
    cargo build --release
fi

# Run the TUI
./target/release/karabiner-switch-key
