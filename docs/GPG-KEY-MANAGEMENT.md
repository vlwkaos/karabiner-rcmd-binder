# GPG Key Management

## Your Keys

**Public Key:** `GPG-PUBLIC-KEY.asc` (safe to share)
**Private Key:** `GPG-PRIVATE-KEY-BACKUP.asc` (⚠️ KEEP SECRET!)

## Key ID
```
A7679D75266DD44D
```

## Fingerprint
```
87C2 0CDE FAF2 01FD 579E  8BAE A767 9D75 266D D44D
```

---

## ⚠️ CRITICAL: Back Up Your Private Key

The private key is stored in:
1. `~/.gnupg/` (your computer)
2. `GPG-PRIVATE-KEY-BACKUP.asc` (this project)

**You MUST back this up securely:**

### Option 1: Encrypted USB Drive (Recommended)
```bash
# Copy to encrypted USB
cp GPG-PRIVATE-KEY-BACKUP.asc /Volumes/YourUSB/secure-backup/

# Verify
cat /Volumes/YourUSB/secure-backup/GPG-PRIVATE-KEY-BACKUP.asc
```

### Option 2: Password Manager
- 1Password / Bitwarden: Store as "Secure Note"
- Paste contents of `GPG-PRIVATE-KEY-BACKUP.asc`

### Option 3: Encrypted Cloud Storage
- Use encrypted storage (Cryptomator, etc.)
- **DO NOT** upload to Google Drive/Dropbox unencrypted!

---

## Restore on New Computer

### 1. Install GPG
```bash
brew install gnupg
```

### 2. Import Private Key
```bash
# From backup file
gpg --import GPG-PRIVATE-KEY-BACKUP.asc

# Trust the key
gpg --edit-key A7679D75266DD44D
> trust
> 5 (I trust ultimately)
> quit
```

### 3. Configure Git
```bash
git config --global user.signingkey A7679D75266DD44D
git config --global commit.gpgsign true
git config --global tag.gpgsign true
```

### 4. Test
```bash
echo "test" | gpg --clearsign
```

You should see "Good signature from vlwkaos"

---

## Security Best Practices

### ✅ DO:
- Keep `GPG-PRIVATE-KEY-BACKUP.asc` in multiple secure locations
- Use a strong passphrase (you chose this during setup)
- Store in encrypted storage only
- Back up to USB drive kept in safe place

### ❌ DON'T:
- **NEVER commit GPG-PRIVATE-KEY-BACKUP.asc to git** (it's in .gitignore)
- Don't email it
- Don't store in plain text on cloud storage
- Don't share it with anyone

---

## If You Lose Your Private Key

You can still:
- ✅ Clone and work on the project
- ✅ Build releases

You CANNOT:
- ❌ Sign new releases with the same key
- ❌ Prove authenticity of new releases

**Solution:** Generate a new key and announce the change in releases.

---

## Revocation Certificate

GPG automatically created a revocation certificate at:
```
~/.gnupg/openpgp-revocs.d/87C20CDEFAF201FD579E8BAEA7679D75266DD44D.rev
```

**Back this up too!** Use it to revoke the key if it's compromised.

---

## Current Status

- ✅ Private key backed up to: `GPG-PRIVATE-KEY-BACKUP.asc`
- ✅ Public key uploaded to GitHub
- ✅ Git configured to sign commits
- ⚠️ **Action Required:** Move `GPG-PRIVATE-KEY-BACKUP.asc` to secure storage NOW!

**Next steps:**
1. Copy `GPG-PRIVATE-KEY-BACKUP.asc` to USB drive or password manager
2. Verify the backup works (import on another machine)
3. Keep the file in this project as a working backup (it's gitignored)
