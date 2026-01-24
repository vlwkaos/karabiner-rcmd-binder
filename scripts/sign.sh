#!/bin/bash
set -e

echo "🔐 Signing rcmdb binary..."

BINARY="target/release/rcmdb"

# Auto-detect Apple Developer identity from keychain
if [ -z "$CODESIGN_IDENTITY" ]; then
    echo "🔍 Auto-detecting Apple Developer identity..."
    IDENTITY=$(security find-identity -v -p codesigning 2>/dev/null | grep "Apple Development" | head -1 | sed 's/.*"\(.*\)"/\1/')
    
    if [ -z "$IDENTITY" ]; then
        echo "❌ Error: No Apple Developer identity found"
        echo ""
        echo "Either:"
        echo "  1. Install an Apple Developer certificate in Keychain"
        echo "  2. Set CODESIGN_IDENTITY environment variable"
        echo ""
        echo "Example: export CODESIGN_IDENTITY='Apple Development: you@example.com (ABCD123456)'"
        exit 1
    fi
    
    echo "✅ Found identity: $IDENTITY"
else
    IDENTITY="$CODESIGN_IDENTITY"
    echo "✅ Using identity from CODESIGN_IDENTITY: $IDENTITY"
fi

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
