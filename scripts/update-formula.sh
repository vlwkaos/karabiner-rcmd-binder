#!/bin/bash
set -e

VERSION="$1"
TAP_DIR="../homebrew-tap"

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/update-formula.sh VERSION"
    echo "Example: ./scripts/update-formula.sh 0.2.1"
    exit 1
fi

if [ ! -d "$TAP_DIR" ]; then
    echo "❌ Error: Homebrew tap not found at $TAP_DIR"
    echo "Expected: /Users/eliot/ws-ps/homebrew-tap"
    exit 1
fi

PACKAGE_NAME="rcmdb-${VERSION}-macos-arm64"
TARBALL_PATH="dist/${PACKAGE_NAME}.tar.gz"

if [ ! -f "$TARBALL_PATH" ]; then
    echo "❌ Error: Tarball not found at $TARBALL_PATH"
    echo "Run 'make package' first"
    exit 1
fi

# Calculate SHA256
echo "🔍 Calculating SHA256..."
SHA256=$(shasum -a 256 "$TARBALL_PATH" | cut -d' ' -f1)

echo "📝 Updating formula..."
echo "  Version: $VERSION"
echo "  SHA256: $SHA256"

# Update formula
FORMULA_PATH="$TAP_DIR/Formula/rcmdb.rb"

# Update version
sed -i '' "s|version \".*\"|version \"$VERSION\"|" "$FORMULA_PATH"

# Update URL - match from 'releases/download/' to end
sed -i '' "s|releases/download/v[^/]*/rcmdb-[^/]*-macos-arm64.tar.gz|releases/download/v$VERSION/rcmdb-$VERSION-macos-arm64.tar.gz|" "$FORMULA_PATH"

# Update SHA256
sed -i '' "s|sha256 \".*\"|sha256 \"$SHA256\"|" "$FORMULA_PATH"

echo "✅ Formula updated!"

# Commit and push
echo "📌 Committing to homebrew-tap..."
cd "$TAP_DIR"
git add Formula/rcmdb.rb
git commit -m "chore: update rcmdb to v$VERSION

SHA256: $SHA256"
git push origin main

echo ""
echo "✅ Homebrew formula updated and pushed!"
echo ""
echo "Users can update with:"
echo "  brew update"
echo "  brew upgrade rcmdb"
