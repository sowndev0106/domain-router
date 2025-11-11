#!/bin/bash

# Post-install script for Domain Router
# This script sets up the necessary capabilities for the binary to bind to privileged ports

BINARY_PATH="$1"

if [ -z "$BINARY_PATH" ]; then
    echo "Usage: $0 <path-to-binary>"
    exit 1
fi

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "Setting up capabilities for Domain Router..."
echo "This allows the app to bind to ports 80 and 443 without sudo"

# Set CAP_NET_BIND_SERVICE capability
sudo setcap 'cap_net_bind_service=+ep' "$BINARY_PATH"

if [ $? -eq 0 ]; then
    echo "✓ Successfully configured Domain Router"
    echo "✓ The app can now bind to ports 80 and 443"
    echo ""
    echo "You can now run the app normally without sudo"
else
    echo "✗ Failed to set capabilities"
    echo "You may need to run the app with sudo, or manually run:"
    echo "  sudo setcap 'cap_net_bind_service=+ep' $BINARY_PATH"
    exit 1
fi
