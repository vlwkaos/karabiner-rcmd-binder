#!/bin/bash
set -e

echo "🔐 Signing rcmdb binary..."

BINARY="target/release/rcmdb"
IDENTITY="Apple Development: mgparkprint@gmail.com (DKTF556333)"

if [ ! -f "$BINARY" ]; then
    echo "❌ Error: Binary not found at $BINARY"
    echo "Run 'cargo build --release' first"
    exit 1
fi

# Sign the binary
codesign --sign "$IDENTITY" \
    --force \
    --options runtime \
    --timestamp \
    "$BINARY"

# Verify signature
echo "✅ Verifying signature..."
codesign --verify --verbose "$BINARY"

echo ""
echo "✅ Binary signed successfully!"
echo ""
echo "To verify: codesign -dv target/release/rcmdb"
