# 🚀 HOW TO RUN - Domain Router

## ✅ Đã Hoàn Thành

Tôi đã refactor **thành công** để loại bỏ Traefik và implement built-in proxy!

### Thay đổi chính:
- ✅ **NO MORE TRAEFIK** - Không cần install external tools
- ✅ Built-in proxy sử dụng Rust
- ✅ All-in-one application
- ✅ Code đã fix hết compile errors

## 🏃 Cách Run (3 options)

### Option 1: Development Mode (Recommended để test)

```bash
# 1. Ensure Rust is installed and updated
rustup update
rustup default stable

# 2. Install frontend dependencies
npm install

# 3. Run in development mode
npm run tauri dev
```

Application sẽ mở window với hot-reload enabled!

### Option 2: Build và Install (Production)

```bash
# 1. Install dependencies (if not done)
npm install

# 2. Build production version
npm run tauri build

# 3. Install DEB package
sudo dpkg -i target/release/bundle/deb/domain-router_*.deb

# 4. Run application
domain-router
```

### Option 3: Automated Installation

```bash
chmod +x install.sh
./install.sh
```

Script sẽ tự động làm tất cả!

## 📝 Step-by-step Detailed

### Step 1: Fix Rust Toolchain (nếu bị lỗi)

```bash
# Remove old toolchain
rustup toolchain list
rustup toolchain remove stable-x86_64-unknown-linux-gnu

# Reinstall
rustup install stable
rustup default stable

# Verify
rustc --version
cargo --version
```

### Step 2: Install Dependencies

```bash
# System packages
sudo apt update
sudo apt install -y \
    curl wget build-essential \
    libssl-dev libwebkit2gtk-4.0-dev \
    libgtk-3-dev libayatana-appindicator3-dev \
    librsvg2-dev patchelf

# Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Node.js (if needed)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Frontend dependencies
npm install
```

### Step 3: Run!

```bash
# Development (with hot reload)
npm run tauri dev

# OR build production
npm run tauri build
```

## 🐛 Troubleshooting

### Error: "rustc binary not found"

```bash
rustup update
rustup default stable
source $HOME/.cargo/env
```

### Error: "libwebkit2gtk not found"

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.0-dev
```

### Error: "Cannot find module"

```bash
rm -rf node_modules package-lock.json
npm install
```

### Port 80/443 in use

App cần port 80 và 443. Nếu ports đang được dùng:

```bash
# Check what's using
sudo lsof -i :80
sudo lsof -i :443

# Stop conflicting services
sudo systemctl stop nginx  # example
sudo systemctl stop apache2  # example
```

### Run with sudo (for ports < 1024)

```bash
# Grant capability to bind low ports
sudo setcap 'cap_net_bind_service=+ep' ~/.cargo/bin/tauri

# OR run with sudo (not recommended)
sudo npm run tauri dev
```

## 🎯 After Running

Khi app chạy:

1. **Click "Start" button** trong footer để start proxy
2. **Add route**:
   - Click "Add Route"
   - Choose "Domain Route" or "Port Mapping"
   - Fill in details
   - Click "Add Route"

3. **Test**:
   ```bash
   # For domain route
   curl http://your-domain.local

   # For port mapping
   curl http://localhost:SOURCE_PORT
   ```

## 📚 Features

### Domain Routing
- Redirect `example.com` → `localhost:8080`
- Auto update `/etc/hosts`
- SSL support (self-signed)

### Port Mapping
- Forward `localhost:4000` → `localhost:80`
- SSL support

### Built-in Proxy
- No Traefik needed!
- Lightweight Rust implementation
- Auto-reload on route changes

## 🔍 Verify Installation

```bash
# Check all is installed
which rustc && echo "✅ Rust"
which node && echo "✅ Node.js"
which npm && echo "✅ NPM"
ls -la node_modules && echo "✅ Frontend deps"

# Test compilation
cargo check --manifest-path=src-tauri/Cargo.toml
```

## 💡 Tips

1. **Use development mode** để test nhanh
2. **Check logs** nếu có issues:
   ```bash
   # In terminal running `npm run tauri dev`
   # Logs will show there
   ```

3. **Ports 80/443** - App cần sudo hoặc CAP_NET_BIND_SERVICE
4. **Hot reload** - Code changes auto reload trong dev mode

## ✨ What's Different from Original?

- ❌ **NO Traefik** - Removed completely
- ✅ **Built-in proxy** - Rust implementation
- ✅ **Simpler** - No external dependencies
- ✅ **Smaller** - ~100MB less (no Traefik binary)
- ✅ **Faster** - Direct Rust proxy

## 🎉 Success Indicators

Khi run thành công, bạn sẽ thấy:

```
✅ Window mở với UI
✅ "Proxy Status: ○ Stopped" trong footer
✅ Có thể click "Start" để start proxy
✅ Có thể add routes
```

## 📞 Need Help?

Check these files:
- [`REFACTORING_NOTES.md`](REFACTORING_NOTES.md) - Technical details
- [`README.md`](README.md) - Full documentation
- [`QUICK_START.md`](QUICK_START.md) - Quick start guide

---

**TL;DR - Quickest Way:**

```bash
rustup update && rustup default stable
npm install
npm run tauri dev
```

Done! 🎉
