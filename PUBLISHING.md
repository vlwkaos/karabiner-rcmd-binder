# Publishing Guide for karabiner-rcmd-binder

## Quick Publish Workflow

```bash
# 1. Update version in Cargo.toml
# 2. Build, sign, and package
make package

# 3. Upload to GitHub Releases
# dist/rcmdb-{version}-macos-arm64.tar.gz
# dist/rcmdb-{version}-macos-arm64.tar.gz.sha256
```

## Step-by-Step

### 1. Update Version

Edit `Cargo.toml`:
```toml
version = "0.2.2"  # Increment version
```

### 2. Build and Sign

```bash
# Clean build
make clean
make build

# Sign binary
make sign
```

Verify signature:
```bash
codesign -dv target/release/rcmdb
```

### 3. Create Distribution Package

```bash
make package
```

This creates:
- `dist/rcmdb-{version}-macos-arm64.tar.gz` (signed binary + docs)
- `dist/rcmdb-{version}-macos-arm64.tar.gz.sha256` (checksum)

### 4. Test Package Locally

```bash
cd /tmp
tar -xzf ~/path/to/rcmdb-{version}-macos-arm64.tar.gz
cd rcmdb-{version}-macos-arm64
./install.sh
rcmdb  # Test it works
```

### 5. Create GitHub Release

1. Go to GitHub → Releases → New Release
2. Tag: `v0.2.2`
3. Title: `v0.2.2 - Feature description`
4. Description:
   ```markdown
   ## Changes
   - Feature 1
   - Feature 2
   
   ## Installation
   \`\`\`bash
   tar -xzf rcmdb-0.2.2-macos-arm64.tar.gz
   cd rcmdb-0.2.2-macos-arm64
   ./install.sh
   \`\`\`
   
   **SHA256:** (paste from .sha256 file)
   ```
5. Attach files:
   - `rcmdb-{version}-macos-arm64.tar.gz`
   - `rcmdb-{version}-macos-arm64.tar.gz.sha256`
6. Publish

### 6. Update Repository

```bash
git add .
git commit -m "chore: release v0.2.2"
git tag v0.2.2
git push origin main --tags
```

## Homebrew Formula (Future)

Create `Formula/rcmdb.rb`:
```ruby
class Switchkey < Formula
  desc "TUI for Karabiner-Elements rcmd key bindings"
  homepage "https://github.com/yourusername/karabiner-rcmd-binder"
  url "https://github.com/yourusername/karabiner-rcmd-binder/releases/download/v0.2.2/rcmdb-0.2.2-macos-arm64.tar.gz"
  sha256 "PASTE_SHA256_HERE"
  version "0.2.2"
  
  def install
    bin.install "rcmdb"
  end
  
  test do
    system "#{bin}/rcmdb", "--version"
  end
end
```

## Code Signing Certificate

Your signing identity:
```
Apple Development: mgparkprint@gmail.com (DKTF556333)
TeamIdentifier: 3DPNMFCFVA
```

To renew or update:
1. Go to developer.apple.com
2. Certificates → Download new certificate
3. Install in Keychain
4. Update `scripts/sign.sh` with new identity if changed

## Notarization (Optional, for Gatekeeper)

For wider distribution, notarize with Apple:

```bash
# Create app bundle
mkdir -p Switchkey.app/Contents/MacOS
cp target/release/rcmdb Switchkey.app/Contents/MacOS/

# Create Info.plist
# ... (template needed)

# Sign app bundle
codesign --deep --force --sign "Apple Development: ..." Switchkey.app

# Create ZIP
ditto -c -k --keepParent Switchkey.app Switchkey.zip

# Notarize
xcrun notarytool submit Switchkey.zip \
  --apple-id "mgparkprint@gmail.com" \
  --team-id "3DPNMFCFVA" \
  --wait

# Staple
xcrun stapler staple Switchkey.app
```

## Pre-Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` (if exists)
- [ ] Run `cargo test`
- [ ] Run `make clean && make build`
- [ ] Run `make sign` successfully
- [ ] Test binary manually
- [ ] Run `make package`
- [ ] Test installation from package
- [ ] Verify SHA256 checksum
- [ ] Create git tag
- [ ] Push to GitHub
- [ ] Create GitHub Release
- [ ] Attach distribution files
- [ ] Announce (if applicable)

## Version Numbering

Follow Semantic Versioning (semver):
- **MAJOR**: Breaking changes (e.g., config format changes)
- **MINOR**: New features, backwards compatible
- **PATCH**: Bug fixes, no new features

Examples:
- `0.2.1` → `0.2.2`: Bug fix
- `0.2.2` → `0.3.0`: New feature (dynamic app discovery)
- `0.3.0` → `1.0.0`: Stable release, no breaking changes planned
