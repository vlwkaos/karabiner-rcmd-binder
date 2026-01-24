#!/bin/bash
# Quick run script for karabiner-rcmd-binder

cd "$(dirname "$0")"

# Check if binary exists
if [ ! -f "target/release/rcmdb" ]; then
    echo "Building rcmdb..."
    cargo build --release
fi

# Run the TUI
./target/release/rcmdb
