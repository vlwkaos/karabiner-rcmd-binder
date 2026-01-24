#!/bin/bash
set -e

VERSION="$1"

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh VERSION"
    echo "Example: ./scripts/release.sh 0.2.2"
    echo "Example: ./scripts/release.sh 0.2.1-beta"
    exit 1
fi

# Check dependencies
echo "🔍 Checking dependencies..."
MISSING_DEPS=()

if ! command -v gh &> /dev/null; then
    MISSING_DEPS+=("gh (GitHub CLI)")
fi

if ! command -v gpg &> /dev/null; then
    MISSING_DEPS+=("gpg")
fi

if ! security find-identity -v -p codesigning 2>/dev/null | grep -q "Apple Development"; then
    if [ -z "$CODESIGN_IDENTITY" ]; then
        MISSING_DEPS+=("Apple Developer certificate (or set CODESIGN_IDENTITY)")
    fi
fi

if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
    echo "❌ Missing dependencies:"
    for dep in "${MISSING_DEPS[@]}"; do
        echo "  - $dep"
    done
    exit 1
fi

echo "✅ All dependencies found"
echo ""
echo "🚀 Starting release process for v$VERSION..."
echo ""

# Step 1: Update version in Cargo.toml
echo "📝 Updating Cargo.toml version..."
sed -i '' "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Step 2: Build
echo "🔨 Building release binary..."
cargo build --release

# Step 3: Sign with Apple cert
echo "🔐 Signing with Apple Developer certificate..."
./scripts/sign.sh

# Step 4: Package (creates tarball + SHA256 + GPG signature)
echo "📦 Creating distribution package..."
./scripts/package.sh "$VERSION"

# Step 5: Git commit and tag
echo "📌 Creating git tag..."
git add Cargo.toml
git add Cargo.lock 2>/dev/null || true
git commit -m "chore: bump version to v$VERSION" || echo "No changes to commit"
git tag -s "v$VERSION" -m "Release v$VERSION"
git push origin main || echo "Already pushed"
git push origin "v$VERSION"

# Step 6: Create GitHub release
PACKAGE_NAME="rcmdb-${VERSION}-macos-arm64"
echo "🎉 Creating GitHub release..."

# Check if this is a beta/pre-release
PRERELEASE_FLAG=""
if [[ "$VERSION" == *"beta"* ]] || [[ "$VERSION" == *"alpha"* ]] || [[ "$VERSION" == *"rc"* ]]; then
    PRERELEASE_FLAG="--prerelease"
    echo "📦 Creating pre-release..."
fi

gh release create "v$VERSION" \
    $PRERELEASE_FLAG \
    --title "v$VERSION" \
    --notes "## Installation

\`\`\`bash
brew tap vlwkaos/tap
brew install rcmdb
\`\`\`

Or download manually:
\`\`\`bash
tar -xzf rcmdb-${VERSION}-macos-arm64.tar.gz
cd rcmdb-${VERSION}-macos-arm64
./install.sh
\`\`\`

## Verification

### GPG Signature
\`\`\`bash
curl -sL https://github.com/vlwkaos.gpg | gpg --import
gpg --verify rcmdb-${VERSION}-macos-arm64.tar.gz.asc
\`\`\`

### SHA256 Checksum
\`\`\`bash
shasum -a 256 -c rcmdb-${VERSION}-macos-arm64.tar.gz.sha256
\`\`\`

## Checksums
\`\`\`
$(cat dist/${PACKAGE_NAME}.tar.gz.sha256)
\`\`\`

See [docs/GPG-VERIFICATION.md](https://github.com/vlwkaos/karabiner-rcmd-binder/blob/main/docs/GPG-VERIFICATION.md) for detailed verification instructions.
" \
    "dist/${PACKAGE_NAME}.tar.gz" \
    "dist/${PACKAGE_NAME}.tar.gz.sha256" \
    "dist/${PACKAGE_NAME}.tar.gz.asc"

echo "✅ GitHub release created!"

# Step 7: Update Homebrew formula (skip for pre-releases)
if [ -z "$PRERELEASE_FLAG" ]; then
    echo "🍺 Updating Homebrew formula..."
    ./scripts/update-formula.sh "$VERSION"
else
    echo "⏭️  Skipping Homebrew formula update (pre-release)"
fi

echo ""
echo "✅ Release v$VERSION complete!"
echo ""
echo "🔗 Release URL: https://github.com/vlwkaos/karabiner-rcmd-binder/releases/tag/v$VERSION"
echo ""
if [ -z "$PRERELEASE_FLAG" ]; then
    echo "Users can now install with:"
    echo "  brew tap vlwkaos/tap"
    echo "  brew install rcmdb"
else
    echo "This is a pre-release. Users can test with:"
    echo "  Download from: https://github.com/vlwkaos/karabiner-rcmd-binder/releases/tag/v$VERSION"
fi
