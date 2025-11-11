# Hướng dẫn Build, Đóng gói và Chạy ứng dụng Domain Router trên Ubuntu

## 📋 Mục lục
1. [Yêu cầu hệ thống](#yêu-cầu-hệ-thống)
2. [Cài đặt Dependencies](#cài-đặt-dependencies)
3. [Build ứng dụng](#build-ứng-dụng)
4. [Đóng gói ứng dụng](#đóng-gói-ứng-dụng)
5. [Cài đặt và Chạy](#cài-đặt-và-chạy)
6. [Kiểm tra lỗi](#kiểm-tra-lỗi)

---

## 🖥️ Yêu cầu hệ thống

### Phần cứng tối thiểu:
- **CPU**: 2 cores
- **RAM**: 2GB
- **Ổ cứng**: 2GB trống
- **Mạng**: Internet (để tải dependencies)

### Hệ điều hành:
- Ubuntu 20.04 LTS hoặc mới hơn
- Ubuntu 22.04 LTS (khuyên dùng)
- Ubuntu 24.04 LTS

### Quyền truy cập:
- Quyền sudo
- Ports 80 và 443 phải available (không bị chiếm dụng)

---

## 🔧 Cài đặt Dependencies

### Phương pháp 1: Cài đặt tự động (Khuyên dùng)

```bash
# Di chuyển vào thư mục project
cd /home/sown/workplace/docker-app/dynamic-routing

# Cấp quyền thực thi cho script
chmod +x install.sh

# Chạy script cài đặt
./install.sh
```

Script này sẽ tự động:
- ✅ Cài đặt system dependencies
- ✅ Cài đặt Rust toolchain
- ✅ Cài đặt Node.js và npm
- ✅ Build ứng dụng
- ✅ Tạo DEB package
- ✅ Cài đặt ứng dụng

**Sau khi chạy xong, ứng dụng đã sẵn sàng sử dụng!**

---

### Phương pháp 2: Cài đặt thủ công từng bước

#### Bước 1: Cập nhật hệ thống

```bash
sudo apt update
sudo apt upgrade -y
```

#### Bước 2: Cài đặt các gói hệ thống cần thiết

```bash
sudo apt install -y \
    curl \
    wget \
    build-essential \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf \
    pkg-config
```

**Giải thích các gói:**
- `build-essential`: Compiler và build tools (gcc, g++, make)
- `libssl-dev`: SSL/TLS library cho HTTPS
- `libwebkit2gtk-4.0-dev`: WebKit engine cho Tauri UI
- `libgtk-3-dev`: GTK3 cho giao diện
- `libayatana-appindicator3-dev`: System tray support
- `librsvg2-dev`: SVG rendering
- `patchelf`: Binary patching tool

#### Bước 3: Cài đặt Rust

```bash
# Tải và cài đặt Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Load Rust environment
source $HOME/.cargo/env

# Kiểm tra cài đặt
rustc --version
cargo --version
```

Kết quả mong đợi:
```
rustc 1.75.0 (hoặc mới hơn)
cargo 1.75.0 (hoặc mới hơn)
```

#### Bước 4: Cài đặt Node.js (v20 LTS)

```bash
# Thêm NodeSource repository
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -

# Cài đặt Node.js
sudo apt install -y nodejs

# Kiểm tra cài đặt
node --version
npm --version
```

Kết quả mong đợi:
```
v20.x.x
10.x.x
```

---

## 🏗️ Build ứng dụng

### ⚡ Cách nhanh nhất: Build tất cả cùng lúc (Khuyên dùng)

```bash
# Di chuyển vào thư mục project
cd /home/sown/workplace/docker-app/dynamic-routing

# Cài đặt dependencies (chỉ cần làm 1 lần)
npm install

# Build frontend + backend + grant capabilities
npm run build:all
```

Script này sẽ tự động:
1. ✅ Build frontend (React + TypeScript + Vite)
2. ✅ Build backend (Rust + Tauri)
3. ✅ Grant capabilities cho binary (để bind ports 80/443)
4. ✅ Tạo .deb package
5. ✅ Hiển thị hướng dẫn chạy app

**Thời gian**: 5-15 phút (lần đầu tiên)

**Output**:
- Binary: `src-tauri/target/release/domain-router`
- DEB package: `src-tauri/target/release/bundle/deb/Domain Router_1.0.0_amd64.deb`

Sau khi hoàn thành, bạn có thể chạy ngay:
```bash
./src-tauri/target/release/domain-router
```

---

### 🔧 Build từng bước (Advanced)

Nếu bạn muốn control từng bước:

#### Bước 1: Cài đặt npm dependencies

```bash
npm install
```

Quá trình này sẽ:
- Tải các package từ npm registry
- Cài đặt React, Vite, TypeScript
- Cài đặt Tauri CLI
- Tạo folder `node_modules/`

**Thời gian ước tính**: 2-5 phút (tùy tốc độ mạng)

#### Bước 2: Build frontend (React + Vite)

```bash
npm run build
```

Lệnh này sẽ:
- Compile TypeScript → JavaScript
- Bundle React components với Vite
- Minify và optimize code
- Tạo folder `dist/` với static files

**Output**: `dist/` folder chứa HTML, CSS, JS

#### Bước 3: Build backend (Rust + Tauri)

**Development build (nhanh hơn, có debug info):**

```bash
npm run tauri:dev
```

**Production build (optimize, nhỏ gọn):**

```bash
npm run tauri:build
```

Quá trình build sẽ:
- Compile Rust code → binary
- Link với system libraries
- Embed frontend vào binary
- Tạo installers (DEB, AppImage, etc.)
- Strip symbols (production mode)

**Thời gian ước tính**: 5-15 phút (lần đầu tiên)

**Output location**:
```
src-tauri/target/release/bundle/
├── deb/
│   └── Domain Router_1.0.0_amd64.deb
├── appimage/
│   └── Domain Router_1.0.0_amd64.AppImage
└── ...
```

#### Bước 4: Grant capabilities (cho ports 80/443)

```bash
sudo setcap 'cap_net_bind_service=+ep' src-tauri/target/release/domain-router
```

**Lưu ý**: Bước này cần thiết để app có thể bind vào ports < 1024 mà không cần chạy với sudo.

---

## 📦 Đóng gói ứng dụng

Sau khi build thành công, bạn có các định dạng package:

### 1. DEB Package (cho Ubuntu/Debian)

**File location**:
```bash
target/release/bundle/deb/domain-router_1.0.0_amd64.deb
```

**Kích thước**: ~10-15MB

**Nội dung package**:
- Binary executable: `/usr/bin/domain-router`
- Desktop entry: `/usr/share/applications/domain-router.desktop`
- Icon: `/usr/share/icons/hicolor/*/apps/domain-router.png`
- License & docs: `/usr/share/doc/domain-router/`

### 2. AppImage (Portable, chạy trên mọi distro)

**File location**:
```bash
target/release/bundle/appimage/domain-router_1.0.0_amd64.AppImage
```

**Ưu điểm**:
- Không cần cài đặt
- Chạy trực tiếp
- Portable giữa các máy

### 3. Binary thuần (không installer)

**File location**:
```bash
target/release/domain-router
```

**Sử dụng**: Chạy trực tiếp hoặc copy vào `/usr/local/bin/`

---

## 🚀 Cài đặt và Chạy

### Cách 1: Cài đặt từ DEB package (Khuyên dùng)

```bash
# Cài đặt package
sudo dpkg -i target/release/bundle/deb/domain-router_1.0.0_amd64.deb

# Nếu có lỗi dependencies, chạy:
sudo apt install -f

# Chạy ứng dụng
domain-router
```

**Sau khi cài đặt**:
- Ứng dụng xuất hiện trong Applications menu
- Có thể search "Domain Router"
- Shortcut desktop (optional)

### Cách 2: Chạy AppImage

```bash
# Cấp quyền thực thi
chmod +x target/release/bundle/appimage/domain-router_1.0.0_amd64.AppImage

# Chạy trực tiếp
./target/release/bundle/appimage/domain-router_1.0.0_amd64.AppImage
```

### Cách 3: Chạy binary trực tiếp

```bash
# Từ thư mục build
./target/release/domain-router

# Hoặc copy vào PATH
sudo cp target/release/domain-router /usr/local/bin/
domain-router
```

---

## 🎮 Sử dụng ứng dụng

### Khởi động

**Từ Applications menu:**
1. Nhấn Super key (Windows key)
2. Gõ "Domain Router"
3. Click vào icon

**Từ Terminal:**
```bash
domain-router
```

### Giao diện chính

Khi ứng dụng mở, bạn sẽ thấy:
- **Danh sách routes**: Hiển thị tất cả domain và port mappings
- **Add Route button**: Thêm route mới
- **Status indicators**: Trạng thái của mỗi route
- **Footer**: Proxy status và controls

### Thêm Domain Route

**Ví dụ**: Redirect `api.local.dev` → `localhost:3000`

1. Click **"Add Route"**
2. Chọn **"Domain Route"**
3. Điền thông tin:
   ```
   Domain: api.local.dev
   Target Port: 3000
   Enable HTTPS: ✓
   SSL Mode: Self-Signed (Auto)
   ```
4. Click **"Add Route"**

**Kết quả**:
- `/etc/hosts` được cập nhật: `127.0.0.1 api.local.dev`
- SSL certificate được tạo tự động
- Reverse proxy được cấu hình
- Truy cập qua: `https://api.local.dev`

### Thêm Port Mapping

**Ví dụ**: Forward `localhost:4000` → `localhost:8080` với HTTPS

1. Click **"Add Route"**
2. Chọn **"Port Mapping"**
3. Điền thông tin:
   ```
   Source Port: 4000
   Target Port: 8080
   Enable HTTPS: ✓
   SSL Mode: Passthrough
   ```
4. Click **"Add Route"**

**Kết quả**:
- Traffic từ port 4000 → 8080
- HTTPS tự động được xử lý
- Service trên 8080 accessible qua `https://localhost:4000`

### Quản lý Routes

- **Enable/Disable**: Click icon ⚡
- **Delete**: Click icon 🗑️
- **View logs**: Check terminal output

---

## ⚠️ Kiểm tra lỗi

### Lỗi 1: "Port 80 or 443 already in use"

**Nguyên nhân**: Port đã được service khác sử dụng

**Kiểm tra**:
```bash
# Xem process nào đang dùng port 80
sudo lsof -i :80

# Xem process nào đang dùng port 443
sudo lsof -i :443
```

**Giải quyết**:
```bash
# Ví dụ: Dừng nginx
sudo systemctl stop nginx

# Hoặc Apache
sudo systemctl stop apache2

# Vô hiệu hóa không tự động start
sudo systemctl disable nginx
```

### Lỗi 2: "Permission denied when modifying /etc/hosts"

**Nguyên nhân**: Thiếu quyền sudo

**Giải quyết**:
```bash
# Cài đặt policykit
sudo apt install policykit-1

# Hoặc thêm user vào sudoers
sudo usermod -aG sudo $USER

# Logout và login lại để apply
```

### Lỗi 3: Build fails với "linker error"

**Nguyên nhân**: Thiếu development libraries

**Giải quyết**:
```bash
# Cài đặt lại tất cả dependencies
sudo apt install -y \
    build-essential \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Clean và rebuild
cargo clean
npm run tauri build
```

### Lỗi 4: "WebKit not found"

**Nguyên nhân**: Thiếu WebKit2GTK

**Giải quyết**:
```bash
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    libjavascriptcoregtk-4.0-dev
```

### Lỗi 5: "npm install fails"

**Nguyên nhân**: Network hoặc npm cache issues

**Giải quyết**:
```bash
# Clear npm cache
npm cache clean --force

# Xóa node_modules và package-lock.json
rm -rf node_modules package-lock.json

# Cài đặt lại
npm install
```

### Lỗi 6: SSL Certificate không hoạt động

**Giải quyết**:
```bash
# Xóa certificates cũ
rm -rf ~/.config/domain-router/certs

# Restart ứng dụng và add route lại
```

---

## 📂 Cấu trúc Files sau khi cài đặt

### System files:
```
/usr/bin/domain-router                          # Binary chính
/usr/share/applications/domain-router.desktop   # Desktop entry
/usr/share/icons/hicolor/*/apps/domain-router.* # Icons
```

### User config files:
```
~/.config/domain-router/
├── config.json           # Main config
├── routes.json           # Routes configuration
├── certs/                # SSL certificates
│   ├── *.crt
│   └── *.key
└── hosts.backup          # Backup of /etc/hosts
```

---

## 🔍 Logs và Debug

### Xem logs runtime:

```bash
# Chạy từ terminal để xem logs
domain-router

# Hoặc xem system logs
journalctl -u domain-router -f
```

### Debug mode:

```bash
# Build với debug symbols
npm run tauri build -- --debug

# Chạy dev mode với hot reload
npm run tauri dev
```

### Check configuration:

```bash
# Xem config hiện tại
cat ~/.config/domain-router/config.json

# Xem routes
cat ~/.config/domain-router/routes.json

# Xem /etc/hosts
cat /etc/hosts | grep "# Managed by Domain Router"
```

---

## 🎯 Các lệnh hữu ích

### Build commands:

```bash
# Development build (có debug info)
npm run tauri build -- --debug

# Production build (optimize)
npm run tauri build

# Build chỉ frontend
npm run build

# Clean build cache
cargo clean
rm -rf dist/ node_modules/
```

### Testing:

```bash
# Run ở dev mode
npm run tauri dev

# Check Rust code
cargo check

# Run Rust tests
cargo test

# Format code
cargo fmt
```

### Distribution:

```bash
# Tạo DEB package
npm run tauri build

# Chỉ build binary (không tạo installer)
cargo build --release

# Cross-compile cho arch khác (advanced)
cargo build --release --target x86_64-unknown-linux-gnu
```

---

## 📦 Uninstall

### Nếu cài từ DEB:

```bash
sudo dpkg -r domain-router
```

### Nếu cài thủ công:

```bash
# Xóa binary
sudo rm /usr/local/bin/domain-router

# Xóa config
rm -rf ~/.config/domain-router

# Xóa desktop entry (nếu có)
rm ~/.local/share/applications/domain-router.desktop
```

### Clean /etc/hosts:

```bash
# Backup trước
sudo cp /etc/hosts /etc/hosts.backup

# Xóa entries do app tạo
sudo sed -i '/# Managed by Domain Router/d' /etc/hosts
```

---

## 🚀 Quick Start Summary

**Cách nhanh nhất để chạy app:**

```bash
# 1. Clone hoặc cd vào folder
cd /home/sown/workplace/docker-app/dynamic-routing

# 2. Chạy script cài đặt
chmod +x install.sh
./install.sh

# 3. Đợi 10-15 phút (tải và build)

# 4. Chạy app
domain-router
```

**Hoặc build thủ công:**

```bash
# 1. Cài dependencies
sudo apt update
sudo apt install -y curl wget build-essential libssl-dev \
    libwebkit2gtk-4.0-dev libgtk-3-dev \
    libayatana-appindicator3-dev librsvg2-dev patchelf

# 2. Cài Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# 3. Cài Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# 4. Build app
npm install
npm run tauri build

# 5. Cài đặt
sudo dpkg -i target/release/bundle/deb/domain-router_*.deb

# 6. Chạy
domain-router
```

---

## 💡 Tips và Best Practices

1. **Build lần đầu chậm**: Lần build đầu tiên sẽ mất 10-15 phút vì phải compile tất cả dependencies. Các lần sau nhanh hơn.

2. **Sử dụng SSD**: Build trên SSD sẽ nhanh hơn rất nhiều so với HDD.

3. **RAM đủ**: Đảm bảo có ít nhất 2GB RAM free khi build.

4. **Internet ổn định**: Quá trình tải dependencies cần internet tốt.

5. **Update thường xuyên**:
   ```bash
   rustup update
   npm update
   ```

6. **Backup config**: Trước khi update app, backup config:
   ```bash
   cp -r ~/.config/domain-router ~/domain-router-backup
   ```

---

## 📞 Support

Nếu gặp vấn đề:

1. Check [Troubleshooting section](#kiểm-tra-lỗi)
2. Xem logs: `journalctl -u domain-router -f`
3. Search issues trên GitHub
4. Tạo issue mới với thông tin:
   - Ubuntu version
   - Error message đầy đủ
   - Output của `rustc --version`, `node --version`
   - Các bước đã thử

---

**Chúc bạn build thành công! 🎉**
