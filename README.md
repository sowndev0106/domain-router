# Domain Router

<div align="center">
  <h3>🌐 Quản lý Domain Routing và Port Mapping trên Ubuntu</h3>
  <p>Ứng dụng desktop mạnh mẽ để quản lý domain routing local với built-in reverse proxy reverse proxy</p>
</div>

---

## ✨ Tính năng chính

### 🔀 Domain Routing (Case 1)
- Redirect domain từ internet về localhost
- Tự động cập nhật `/etc/hosts`
- Cấu hình built-in reverse proxy reverse proxy tự động
- Hỗ trợ HTTP và HTTPS
- SSL certificate tự động (self-signed)

**Ví dụ:**
```
Domain: https://seller-dev.openlive.lotte.vn
→ /etc/hosts: 127.0.0.1 seller-dev.openlive.lotte.vn
→ built-in reverse proxy routes traffic đến localhost:80
```

### 🔌 Port Mapping với SSL (Case 2)
- Map port local sang port khác
- Tự động xử lý HTTPS
- SSL passthrough hoặc tự động generate certificate
- Conflict detection cho ports

**Ví dụ:**
```
Source: http://localhost:4000
→ built-in reverse proxy proxies sang https://localhost:80
→ SSL tự động hoặc passthrough
```

## 🚀 Cài đặt

### Yêu cầu hệ thống
- Ubuntu 20.04 hoặc mới hơn
- Sudo privileges
- 2GB RAM minimum
- Ports 80 và 443 available

### Cài đặt tự động

```bash
# Clone repository
git clone <repository-url>
cd domain-router

# Run installation script
chmod +x install.sh
./install.sh
```

Script sẽ tự động cài đặt:
- System dependencies
- Rust toolchain
- built-in reverse proxy binary
- Node.js và npm
- Build và install application

### Cài đặt thủ công

#### 1. Cài đặt Dependencies

```bash
# System packages
sudo apt update
sudo apt install -y curl wget build-essential libssl-dev \
  libwebkit2gtk-4.0-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev patchelf

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# built-in reverse proxy
sudo wget -O /usr/local/bin/ \
  https://github.com///releases/latest/download/_linux_amd64
sudo chmod +x /usr/local/bin/

# Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
```

#### 2. Build Application

```bash
# Install npm dependencies
npm install

# Build for production
npm run tauri build

# Install DEB package
sudo dpkg -i target/release/bundle/deb/domain-router_*.deb
```

## 📖 Sử dụng

### Khởi chạy ứng dụng

```bash
# From applications menu
Search "Domain Router" và click

# Or from terminal
domain-router
```

### Thêm Domain Route

1. Click nút **"Add Route"**
2. Chọn **"Domain Route"**
3. Nhập:
   - Domain: `seller-dev.openlive.lotte.vn`
   - Target Port: `80`
   - Enable HTTPS: `✓`
   - SSL Mode: `Self-Signed (Auto)`
4. Click **"Add Route"**

App sẽ:
- Thêm entry vào `/etc/hosts` (yêu cầu sudo)
- Generate SSL certificate
- Cấu hình built-in reverse proxy router
- Start routing ngay lập tức

### Thêm Port Mapping

1. Click **"Add Route"**
2. Chọn **"Port Mapping"**
3. Nhập:
   - Source Port: `4000`
   - Target Port: `80`
   - Enable HTTPS: `✓`
   - SSL Mode: `Passthrough` hoặc `Self-Signed`
4. Click **"Add Route"**

### Quản lý Routes

- **Enable/Disable**: Click icon ⚡ để bật/tắt route
- **Delete**: Click icon 🗑️ để xóa route
- **Status**: Xem trạng thái real-time của mỗi route

### Quản lý built-in reverse proxy

Footer hiển thị built-in reverse proxy status:
- **⬤ Running**: built-in reverse proxy đang chạy
- **○ Stopped**: built-in reverse proxy đã dừng
- **Start/Stop buttons**: Điều khiển built-in reverse proxy service

## ⚙️ Configuration

### Config File Location
```
~/.config/domain-router/config.json
```

### built-in reverse proxy Config
```
~/.config/domain-router//
├── .yml      # Static configuration
└── dynamic.yml      # Dynamic routes (auto-generated)
```

### SSL Certificates
```
~/.config/domain-router/certs/
├── example.com.crt
└── example.com.key
```

### Hosts Backup
```
~/.config/domain-router/hosts.backup
```

## 🔧 Development

### Setup Development Environment

```bash
# Clone repo
git clone <repo-url>
cd domain-router

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Project Structure

```
domain-router/
├── src/                      # Frontend (React + TypeScript)
│   ├── components/           # React components
│   │   ├── RouteList.tsx
│   │   └── AddRouteDialog.tsx
│   ├── App.tsx               # Main app component
│   ├── App.css               # Styles
│   ├── api.ts                # Tauri API wrapper
│   └── types.ts              # TypeScript types
├── src-tauri/                # Backend (Rust)
│   ├── src/
│   │   ├── routes/           # Route management
│   │   ├── hosts/            # /etc/hosts manager
│   │   ├── /          # built-in reverse proxy controller
│   │   ├── ssl/              # SSL certificate manager
│   │   ├── utils/            # Utilities
│   │   ├── lib.rs            # Main library
│   │   └── main.rs           # Entry point
│   ├── Cargo.toml
│   └── tauri.conf.json
└── install.sh                # Installation script
```

### Building

```bash
# Development build
npm run tauri build -- --debug

# Production build
npm run tauri build

# Output location
target/release/bundle/
├── deb/                      # Debian package
│   └── domain-router_*.deb
├── appimage/                 # AppImage
└── rpm/                      # RPM package
```

## 🐛 Troubleshooting

### Port 80/443 already in use

```bash
# Check what's using the ports
sudo lsof -i :80
sudo lsof -i :443

# Stop conflicting services (example: nginx)
sudo systemctl stop nginx
```

### built-in reverse proxy not starting

```bash
# Check built-in reverse proxy logs
journalctl -u domain-router -f

# Manually test built-in reverse proxy
/usr/local/bin/ --configFile ~/.config/domain-router//.yml
```

### /etc/hosts permission denied

Ứng dụng sẽ tự động yêu cầu sudo bằng `pkexec`. Nếu gặp lỗi:

```bash
# Ensure pkexec is installed
sudo apt install policykit-1

# Or manually add entry
sudo nano /etc/hosts
# Add: 127.0.0.1 your-domain.com
```

### SSL certificate errors

```bash
# Regenerate certificates
rm -rf ~/.config/domain-router/certs
# Then add route again in app
```

## 📝 Examples

### Example 1: Local Development Domain

Redirect `api.local.dev` → `localhost:3000`

1. Add Domain Route:
   - Domain: `api.local.dev`
   - Target Port: `3000`
   - HTTPS: Enabled

2. Your API server running on `localhost:3000` sẽ accessible tại:
   - `http://api.local.dev` (nếu HTTPS disabled)
   - `https://api.local.dev` (nếu HTTPS enabled)

### Example 2: Port Forwarding với SSL

Forward `localhost:4000` → `localhost:8080` với HTTPS

1. Add Port Mapping:
   - Source Port: `4000`
   - Target Port: `8080`
   - HTTPS: Enabled
   - SSL Mode: Passthrough

2. Service trên port 8080 sẽ accessible qua HTTPS trên port 4000

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Desktop framework
- [built-in reverse proxy](https://.io/) - Reverse proxy
- [React](https://react.dev/) - UI framework
- [Rust](https://www.rust-lang.org/) - Systems programming language

## 🔮 Roadmap (v2.0)

- [ ] Docker container routing
- [ ] Wildcard domain support (`*.dev.local`)
- [ ] Import/export configurations
- [ ] Real Let's Encrypt integration
- [ ] Middleware support (auth, rate limiting)
- [ ] Multi-profile support (dev/staging/prod)
- [ ] System tray icon
- [ ] Auto-start on boot
- [ ] macOS and Windows support

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Troubleshooting](#-troubleshooting) section
2. Search existing [Issues](https://github.com/your-repo/issues)
3. Create a new issue with:
   - OS version
   - Error message
   - Steps to reproduce

---

**Made with ❤️ for Ubuntu developers**
