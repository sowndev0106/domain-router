#!/bin/bash

# Domain Router Installation Script for Ubuntu
# This script installs all dependencies and sets up the application

set -e

echo "===================================="
echo " Domain Router - Installation"
echo "===================================="
echo ""

# Check if running on Ubuntu
if [ ! -f /etc/os-release ] || ! grep -q "ubuntu" /etc/os-release; then
    echo "⚠️  Warning: This script is designed for Ubuntu."
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "❌ Please do not run this script as root"
    exit 1
fi

echo "📦 Step 1: Installing system dependencies..."
sudo apt update
sudo apt install -y \
    curl \
    wget \
    build-essential \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf

echo ""
echo "🦀 Step 2: Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust already installed"
fi

echo ""
echo "📦 Step 3: Installing Node.js (for building UI)..."
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt install -y nodejs
else
    echo "✅ Node.js already installed"
fi

echo ""
echo "🔧 Step 4: Installing npm dependencies..."
npm install

echo ""
echo "🏗️  Step 5: Building application..."
npm run tauri build

echo ""
echo "📦 Step 6: Installing DEB package..."
DEB_FILE=$(find target/release/bundle/deb -name "*.deb" | head -n 1)
if [ -f "$DEB_FILE" ]; then
    sudo dpkg -i "$DEB_FILE"
    echo "✅ Application installed successfully"
else
    echo "❌ DEB file not found. Build may have failed."
    exit 1
fi

echo ""
echo "===================================="
echo "✅ Installation Complete!"
echo "===================================="
echo ""
echo "You can now run 'domain-router' from your applications menu"
echo "or from the command line."
echo ""
echo "⚠️  Important Notes:"
echo "  - You'll need to grant sudo access when modifying /etc/hosts"
echo "  - Built-in reverse proxy will use ports 80 and 443"
echo "  - Make sure these ports are not in use"
echo "  - No external tools required - everything is built-in!"
echo ""
echo "📚 Documentation: See README.md for usage instructions"
echo ""
