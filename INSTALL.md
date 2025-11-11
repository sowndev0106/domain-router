# Domain Router - Installation Guide

## Prerequisites

- Ubuntu/Debian Linux
- Node.js 18+ and npm
- Rust and cargo (for building from source)

## Installation

### Option 1: Build and Install

```bash
# Build the application
npm install
npm run tauri build

# Run post-install script to set up port binding capabilities
./scripts/post-install.sh src-tauri/target/release/domain-router

# Now you can run the app without sudo
./src-tauri/target/release/domain-router
```

### Option 2: Install from .deb package (Coming soon)

```bash
sudo dpkg -i domain-router_*.deb
# Capabilities will be set automatically during installation
```

## Port Binding Permissions

This application needs to bind to ports 80 and 443 for HTTP/HTTPS proxying. On Linux, ports below 1024 are "privileged" and normally require root access.

### How we handle this securely:

Instead of running the entire app as root (which is a security risk), we use Linux capabilities to grant only the specific permission needed:

```bash
sudo setcap 'cap_net_bind_service=+ep' /path/to/domain-router
```

This command grants the `CAP_NET_BIND_SERVICE` capability, which allows the binary to:
- ✅ Bind to ports below 1024 (like 80 and 443)
- ❌ Does NOT give full root access
- ❌ Does NOT compromise system security

### Verify capabilities:

```bash
getcap src-tauri/target/release/domain-router
# Output: cap_net_bind_service=ep
```

## Development

For development, you have two options:

### Option 1: Run with sudo (Quick & Easy)
```bash
sudo npm run tauri dev
```

### Option 2: Use non-privileged ports for testing
- Use port 8080 instead of 80
- Use port 8443 instead of 443
- Test without sudo, then build and install for production use

```bash
npm run tauri dev
# In the app, create port mappings like:
# - 8080 → your-service:4000
# - 8443 → your-service:4000
```

## Troubleshooting

### "Permission denied" errors when binding to port 80/443

**Cause:** The binary doesn't have the required capability.

**Solution:**
```bash
./scripts/post-install.sh src-tauri/target/release/domain-router
```

### Capabilities lost after rebuild

**Cause:** Capabilities are tied to the specific binary. When you rebuild, the new binary doesn't have them.

**Solution:** Run the post-install script again after each build:
```bash
npm run tauri build
./scripts/post-install.sh src-tauri/target/release/domain-router
```

### Still having issues?

1. Check if `libcap` is installed:
   ```bash
   sudo apt-get install libcap2-bin
   ```

2. Verify your user has permission to use `setcap`:
   ```bash
   sudo setcap -v 'cap_net_bind_service=+ep' src-tauri/target/release/domain-router
   ```

3. As a last resort, you can run with sudo:
   ```bash
   sudo ./src-tauri/target/release/domain-router
   ```

## Security Notes

- **CAP_NET_BIND_SERVICE** is a minimal capability that only allows binding to privileged ports
- The app runs as your regular user (not root)
- Only the necessary network permissions are granted
- This is the recommended approach by Linux security best practices
- Much safer than running the entire app with sudo

## Uninstallation

To remove the capabilities:

```bash
sudo setcap -r src-tauri/target/release/domain-router
```

Or simply delete the binary:

```bash
rm src-tauri/target/release/domain-router
```
