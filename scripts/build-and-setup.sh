#!/bin/bash

set -e  # Exit on error

echo "================================================"
echo "  Building Domain Router (Frontend + Backend)"
echo "================================================"

# Step 1: Build frontend
echo ""
echo "📦 Step 1/4: Building frontend..."
npm run build

# Step 2: Build backend with Tauri
echo ""
echo "🦀 Step 2/4: Building backend (Rust + Tauri)..."
npm run tauri build

# Step 3: Grant capabilities
echo ""
echo "🔐 Step 3/4: Granting capabilities to binary..."
BINARY_PATH="src-tauri/target/release/domain-router"

if [ -f "$BINARY_PATH" ]; then
    sudo setcap 'cap_net_bind_service=+ep' "$BINARY_PATH"
    echo "✓ Capabilities granted to $BINARY_PATH"
else
    echo "⚠️  Binary not found at $BINARY_PATH"
    exit 1
fi

# Step 4: Summary
echo ""
echo "================================================"
echo "✅ Build Complete!"
echo "================================================"
echo ""
echo "Binary location:"
echo "  $BINARY_PATH"
echo ""
echo "Debian package:"
DEB_PATH=$(ls -1 src-tauri/target/release/bundle/deb/*.deb 2>/dev/null | head -1)
if [ -n "$DEB_PATH" ]; then
    echo "  $DEB_PATH"
    echo ""
    echo "To install .deb package:"
    echo "  sudo dpkg -i \"$DEB_PATH\""
    echo "  sudo setcap 'cap_net_bind_service=+ep' /usr/bin/domain-router"
fi
echo ""
echo "To run directly:"
echo "  ./$BINARY_PATH"
echo ""
echo "To test with Quick Setup:"
echo "  1. Start your backend: npm run start (on port 4000)"
echo "  2. Run: ./$BINARY_PATH"
echo "  3. Click 'Quick Setup (80 & 443)'"
echo "  4. Enter target port: 4000"
echo "  5. Click 'Start Proxy'"
echo "  6. Test:"
echo "     curl http://localhost"
echo "     curl -k https://localhost"
echo ""
