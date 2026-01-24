#!/bin/bash
set -e

# Get version from Cargo.toml if not provided
if [ -z "$1" ]; then
    VERSION=$(grep '^version =' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
else
    VERSION="$1"
fi

PACKAGE_NAME="rcmdb-${VERSION}-macos-arm64"
BUILD_DIR="dist/$PACKAGE_NAME"

echo "📦 Packaging rcmdb v$VERSION..."

# Clean previous build
rm -rf dist
mkdir -p "$BUILD_DIR"

# Build and sign
echo "🔨 Building release binary..."
cargo build --release

echo "🔐 Signing binary..."
./scripts/sign.sh

# Copy files
echo "📄 Copying files..."
cp target/release/rcmdb "$BUILD_DIR/"
cp README.md "$BUILD_DIR/"
cp LICENSE "$BUILD_DIR/" 2>/dev/null || echo "# MIT License" > "$BUILD_DIR/LICENSE"

# Create install script in package
cat > "$BUILD_DIR/install.sh" << 'EOF'
#!/bin/bash
set -e

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Installing rcmdb to $INSTALL_DIR..."
cp rcmdb "$INSTALL_DIR/rcmdb"
chmod +x "$INSTALL_DIR/rcmdb"

if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  Add this to your ~/.zshrc or ~/.bashrc:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "✅ Installed! Run: rcmdb"
EOF
chmod +x "$BUILD_DIR/install.sh"

# Create tarball
echo "🗜️  Creating tarball..."
cd dist
tar -czf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"
cd ..

# Calculate checksum
echo "🔍 Calculating checksum..."
shasum -a 256 "dist/${PACKAGE_NAME}.tar.gz" > "dist/${PACKAGE_NAME}.tar.gz.sha256"

# GPG sign the tarball
echo "🔐 GPG signing tarball..."
if ! gpg --list-secret-keys &> /dev/null; then
    echo "⚠️  Warning: No GPG key found. Skipping GPG signature."
    echo "   Run ./scripts/setup-gpg.sh to set up GPG signing."
else
    gpg --detach-sign --armor "dist/${PACKAGE_NAME}.tar.gz"
fi

echo ""
echo "✅ Package created successfully!"
echo ""
echo "📦 Package: dist/${PACKAGE_NAME}.tar.gz"
echo "📊 Size: $(du -h dist/${PACKAGE_NAME}.tar.gz | cut -f1)"
echo "🔐 SHA256: $(cat dist/${PACKAGE_NAME}.tar.gz.sha256)"
echo "🔏 GPG signature: dist/${PACKAGE_NAME}.tar.gz.asc"
echo ""
echo "To install:"
echo "  tar -xzf ${PACKAGE_NAME}.tar.gz"
echo "  cd $PACKAGE_NAME"
echo "  ./install.sh"
