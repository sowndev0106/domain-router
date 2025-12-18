# Build Guide

Complete instructions for building Domain Router from source.

## System Requirements

### Hardware
- **CPU**: 2 cores minimum
- **RAM**: 2GB minimum (4GB recommended)
- **Disk**: 2GB free space
- **Network**: Internet connection (for downloading dependencies)

### Operating Systems
- **Linux**: Ubuntu 20.04+, Debian 11+, Fedora 38+
- **Windows**: Windows 10/11 (for cross-compilation from Linux, or native build)

## Prerequisites

### Linux

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install system dependencies
sudo apt install -y \
    curl wget build-essential \
    libssl-dev libwebkit2gtk-4.0-dev \
    libgtk-3-dev libayatana-appindicator3-dev \
    librsvg2-dev patchelf pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Verify Rust installation
rustc --version  # Should be 1.75+
cargo --version

# Install Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Verify Node.js installation
node --version   # Should be v20.x.x
npm --version    # Should be 10.x.x
```

### Windows (Native Build)

1. Install [Rust](https://rustup.rs/)
2. Install [Node.js 20 LTS](https://nodejs.org/)
3. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
4. Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Windows (Cross-Compile from Linux)

```bash
# Install MinGW toolchain
sudo apt install -y mingw-w64

# Add Windows target to Rust
rustup target add x86_64-pc-windows-gnu
```

## Building

### Quick Build (Recommended)

```bash
# Clone and enter directory
cd domain-router

# Install npm dependencies
npm install

# Build everything (frontend + backend + permissions)
npm run build:all
```

This will:
1. Build the React frontend
2. Compile the Rust backend
3. Create installers (.deb, .AppImage)
4. Grant necessary Linux capabilities

**Build time**: 5-15 minutes (first build), 1-3 minutes (subsequent builds)

### Manual Build Steps

#### Step 1: Install npm dependencies

```bash
npm install
```

#### Step 2: Build frontend

```bash
npm run build
```

Creates `dist/` folder with HTML, CSS, JS.

#### Step 3: Build backend

**Development (with debug info):**
```bash
npm run tauri:dev
```

**Production (optimized):**
```bash
npm run tauri:build
```

#### Step 4: Grant capabilities (Linux only)

```bash
sudo setcap 'cap_net_bind_service=+ep' src-tauri/target/release/domain-router
```

## Build Outputs

After successful build:

```
src-tauri/target/release/
├── domain-router                           # Binary executable
└── bundle/
    ├── deb/
    │   └── Domain Router_1.0.0_amd64.deb   # Debian package
    └── appimage/
        └── Domain Router_1.0.0_amd64.AppImage  # Portable Linux
```

## Cross-Compilation

### Build for Windows (from Linux)

```bash
# Ensure Windows target is installed
rustup target add x86_64-pc-windows-gnu

# Build
npm run tauri:build -- --target x86_64-pc-windows-gnu
```

Output: `src-tauri/target/x86_64-pc-windows-gnu/release/bundle/nsis/Domain Router_1.0.0_x64-setup.exe`

## Installation

### From .deb package (Debian/Ubuntu)

```bash
sudo dpkg -i "Domain Router_1.0.0_amd64.deb"

# If dependency errors occur:
sudo apt install -f
```

### From AppImage

```bash
chmod +x "Domain Router_1.0.0_amd64.AppImage"
./"Domain Router_1.0.0_amd64.AppImage"
```

### From binary

```bash
# Run directly
./src-tauri/target/release/domain-router

# Or install to PATH
sudo cp src-tauri/target/release/domain-router /usr/local/bin/
domain-router
```

## Development

### Hot Reload Development

```bash
npm run tauri:dev
```

Changes to React code will hot-reload. Rust changes require restart.

### Running Tests

```bash
# Rust tests
cargo test

# Check Rust code
cargo check

# Format code
cargo fmt
```

### Debug Build

```bash
npm run tauri:build -- --debug
```

## Troubleshooting

### Build fails with "linker error"

```bash
# Reinstall development libraries
sudo apt install -y build-essential libssl-dev \
    libwebkit2gtk-4.0-dev libgtk-3-dev

# Clean and rebuild
cargo clean
npm run tauri:build
```

### "WebKit not found"

```bash
sudo apt install -y libwebkit2gtk-4.0-dev libjavascriptcoregtk-4.0-dev
```

### npm install fails

```bash
# Clear npm cache
npm cache clean --force

# Remove existing modules
rm -rf node_modules package-lock.json

# Reinstall
npm install
```

### Windows cross-compile fails with "aws-lc-sys" error

The TLS library requires NASM for Windows builds. The project is configured to use `ring` backend instead, which doesn't require NASM.

If you still encounter issues:
```bash
# Ensure ring backend is used (already configured in Cargo.toml)
# Check that rustls features use "ring" not "aws_lc_rs"
```

## Uninstallation

### Remove .deb package

```bash
sudo dpkg -r domain-router
```

### Remove binary installation

```bash
sudo rm /usr/local/bin/domain-router
rm -rf ~/.config/domain-router
```

### Clean hosts file

```bash
# Backup first
sudo cp /etc/hosts /etc/hosts.backup

# Remove Domain Router entries
sudo sed -i '/# === Domain Router/,/# === Domain Router END ===/d' /etc/hosts
```

## Build Optimization

The project is configured with optimized release settings in `Cargo.toml`:

```toml
[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

This provides:
- Smaller binary size
- Better performance
- Stripped debug symbols

## CI/CD Example

GitHub Actions workflow for automated builds:

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev build-essential \
            libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npm run tauri:build
      - uses: actions/upload-artifact@v4
        with:
          name: linux-builds
          path: |
            src-tauri/target/release/bundle/deb/*.deb
            src-tauri/target/release/bundle/appimage/*.AppImage

  build-windows:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev build-essential \
            libssl-dev libgtk-3-dev mingw-w64
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-gnu
      - run: npm install
      - run: npm run tauri:build -- --target x86_64-pc-windows-gnu
      - uses: actions/upload-artifact@v4
        with:
          name: windows-builds
          path: src-tauri/target/x86_64-pc-windows-gnu/release/bundle/nsis/*.exe
```

## Tips

1. **First build is slow**: Initial compilation downloads and builds all dependencies. Subsequent builds are faster.

2. **Use SSD**: Building on SSD is significantly faster than HDD.

3. **Sufficient RAM**: Ensure at least 2GB free RAM during build.

4. **Update regularly**:
   ```bash
   rustup update
   npm update
   ```

5. **Backup config before updates**:
   ```bash
   cp -r ~/.config/domain-router ~/domain-router-backup
   ```
