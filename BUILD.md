# Build Instructions

## Development Build

```bash
# Install dependencies first
npm install

# Run in development mode (hot reload)
npm run tauri dev
```

Ứng dụng sẽ mở trong development window với hot reload enabled.

## Production Build

### Option 1: Automated Installation Script

```bash
chmod +x install.sh
./install.sh
```

Script sẽ:
1. Install tất cả dependencies
2. Build production binary
3. Create DEB package
4. Install application

### Option 2: Manual Build

```bash
# Ensure dependencies are installed
npm install

# Build for production
npm run tauri build
```

Output sẽ ở trong:
```
target/release/bundle/
├── deb/
│   └── domain-router_1.0.0_amd64.deb
├── appimage/
│   └── domain-router_1.0.0_amd64.AppImage
└── rpm/
    └── domain-router-1.0.0-1.x86_64.rpm
```

### Install DEB Package

```bash
sudo dpkg -i target/release/bundle/deb/domain-router_*.deb
```

## Build Optimization

Cargo.toml đã configured với:
```toml
[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

Điều này giúp:
- Giảm binary size
- Tăng performance
- Strip debug symbols

## Troubleshooting

### Build fails with dependency errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cd src-tauri
cargo clean
cd ..
npm run tauri build
```

### Missing system dependencies

```bash
sudo apt install -y libwebkit2gtk-4.0-dev \
  build-essential curl wget libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf
```

### npm install errors

```bash
# Clear cache
npm cache clean --force

# Delete node_modules and reinstall
rm -rf node_modules
npm install
```

## Cross-Compilation (Advanced)

Not yet supported. Currently targets x86_64 Linux only.

## CI/CD

Example GitHub Actions workflow:

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install dependencies
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.0-dev build-essential curl wget \
            libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
      - name: Setup Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '20'
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install dependencies
        run: npm install
      - name: Build
        run: npm run tauri build
      - name: Upload artifacts
        uses: actions/upload-artifact@v2
        with:
          name: domain-router-deb
          path: target/release/bundle/deb/*.deb
```
