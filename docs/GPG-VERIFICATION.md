# How to Verify GPG Signatures

All `rcmdb` releases are signed with GPG to ensure authenticity and integrity.

## Quick Verify

```bash
# Import public key (one time)
curl -sL https://github.com/vlwkaos.gpg | gpg --import

# Verify release
gpg --verify rcmdb-0.2.1-macos-arm64.tar.gz.asc
```

You should see:
```
gpg: Good signature from "vlwkaos (karabiner-rcmd-binder releases) <vlwkaos@users.noreply.github.com>"
```

## Manual Verification

### 1. Download Release Files

From https://github.com/vlwkaos/karabiner-rcmd-binder/releases:

- `rcmdb-0.2.1-macos-arm64.tar.gz` (the package)
- `rcmdb-0.2.1-macos-arm64.tar.gz.asc` (GPG signature)
- `rcmdb-0.2.1-macos-arm64.tar.gz.sha256` (checksum)

### 2. Import Public Key

**Option A:** From GitHub
```bash
curl -sL https://github.com/vlwkaos.gpg | gpg --import
```

**Option B:** From project repository
```bash
gpg --import GPG-PUBLIC-KEY.asc
```

### 3. Verify Signature

```bash
gpg --verify rcmdb-0.2.1-macos-arm64.tar.gz.asc rcmdb-0.2.1-macos-arm64.tar.gz
```

**Expected output:**
```
gpg: Signature made Fri Jan 24 18:00:00 2026 KST
gpg:                using RSA key 87C20CDEFAF201FD579E8BAEA7679D75266DD44D
gpg: Good signature from "vlwkaos (karabiner-rcmd-binder releases) <vlwkaos@users.noreply.github.com>" [unknown]
```

**Warning about trust is normal:**
```
gpg: WARNING: This key is not certified with a trusted signature!
```

This just means you haven't personally marked the key as trusted. The signature is still valid.

### 4. Verify Checksum (Optional)

```bash
shasum -a 256 -c rcmdb-0.2.1-macos-arm64.tar.gz.sha256
```

Expected: `rcmdb-0.2.1-macos-arm64.tar.gz: OK`

## Key Information

**Key ID:** `A7679D75266DD44D`

**Fingerprint:**
```
87C2 0CDE FAF2 01FD 579E  8BAE A767 9D75 266D D44D
```

**Owner:** vlwkaos (karabiner-rcmd-binder releases)

**Email:** vlwkaos@users.noreply.github.com

**Expiration:** Never

## Trust This Key

If you want to suppress the "not certified" warning:

```bash
gpg --edit-key A7679D75266DD44D
> trust
> 5 (I trust ultimately)
> quit
```

## What GPG Signature Proves

✅ **Authenticity** - The release was created by vlwkaos (holder of private key)
✅ **Integrity** - The file hasn't been tampered with since signing
✅ **Non-repudiation** - Only the key owner could have created this signature

❌ **Does NOT prove** - The software is safe/bug-free (review source code for that)

## Troubleshooting

**"gpg: Can't check signature: No public key"**
```bash
# Import the key first
curl -sL https://github.com/vlwkaos.gpg | gpg --import
```

**"gpg: BAD signature"**
⚠️ **DO NOT INSTALL!** The file has been tampered with.

**"gpg command not found"**
```bash
brew install gnupg
```

## Additional Security

Verify the commit that created the release is also GPG signed:

```bash
git verify-tag v0.2.1
```

Or check on GitHub - look for the "Verified" badge next to commits.
