#!/bin/bash
set -e

echo "🔐 GPG Key Setup for karabiner-rcmd-binder releases"
echo ""

# Check if GPG key already exists
if gpg --list-secret-keys vlwkaos@users.noreply.github.com &> /dev/null; then
    echo "✅ GPG key already exists for vlwkaos@users.noreply.github.com"
    KEY_ID=$(gpg --list-secret-keys --keyid-format=long vlwkaos@users.noreply.github.com | grep sec | awk '{print $2}' | cut -d'/' -f2)
    echo "Key ID: $KEY_ID"
else
    echo "📝 Creating GPG key batch file..."
    
    # Create batch file for non-interactive key generation
    cat > /tmp/gpg-keygen-batch << EOF
%no-protection
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: vlwkaos
Name-Email: vlwkaos@users.noreply.github.com
Name-Comment: karabiner-rcmd-binder releases
Expire-Date: 0
EOF

    echo "🔑 Generating GPG key..."
    gpg --batch --generate-key /tmp/gpg-keygen-batch
    rm /tmp/gpg-keygen-batch
    
    KEY_ID=$(gpg --list-secret-keys --keyid-format=long vlwkaos@users.noreply.github.com | grep sec | awk '{print $2}' | cut -d'/' -f2)
    echo "✅ GPG key generated! Key ID: $KEY_ID"
fi

# Export public key
echo ""
echo "📤 Exporting public key..."
gpg --armor --export vlwkaos@users.noreply.github.com > GPG-PUBLIC-KEY.asc
echo "✅ Public key exported to GPG-PUBLIC-KEY.asc"

# Export for GitHub (to clipboard)
echo ""
echo "📋 Copying public key to clipboard for GitHub..."
gpg --armor --export vlwkaos@users.noreply.github.com | pbcopy
echo "✅ Public key copied to clipboard!"

echo ""
echo "📌 Next steps:"
echo "1. Go to: https://github.com/settings/gpg/new"
echo "2. Paste the key from clipboard (already copied!)"
echo "3. Click 'Add GPG key'"
echo ""
echo "🔍 Your key fingerprint:"
gpg --fingerprint vlwkaos@users.noreply.github.com
echo ""
echo "Press Enter when you've added the key to GitHub..."
read

# Configure git to sign commits
echo "⚙️  Configuring git to use GPG key..."
git config --global user.signingkey "$KEY_ID"
git config --global commit.gpgsign true
git config --global tag.gpgsign true

echo ""
echo "✅ GPG setup complete!"
echo ""
echo "Test signing:"
echo "  echo 'test' | gpg --clearsign"
