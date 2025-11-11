# Domain Router 🚀

Reverse proxy và SSL/TLS manager cho localhost và Docker containers trên Ubuntu.

## ✨ Features

- ✅ **HTTP Reverse Proxy** - Port forwarding và domain routing
- ✅ **HTTPS với Self-Signed Certificates** - TLS termination tự động
- ✅ **Quick Setup (80 & 443)** - One-click configuration cho HTTP + HTTPS
- ✅ **Privilege Escalation** - GUI dialog để xin quyền (không cần sudo)
- ✅ **Let's Encrypt Foundation** - Sẵn sàng cho production (cần complete)

## 🚀 Quick Start

### Build tất cả cùng lúc (Khuyên dùng):

```bash
# Clone và vào folder
cd /home/sown/workplace/docker-app/dynamic-routing

# Install dependencies (chỉ cần 1 lần)
npm install

# Build frontend + backend + grant capabilities
npm run build:all
```

Sau khi build xong (~10 phút), chạy:

```bash
./src-tauri/target/release/domain-router
```

### Development Mode:

```bash
npm run tauri:dev
```

## 📖 Documentation

- **[HUONG_DAN_BUILD_RUN.md](HUONG_DAN_BUILD_RUN.md)** - Hướng dẫn build chi tiết (tiếng Việt)
- **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - Tổng quan project & architecture
- **[SSL_SETUP.md](SSL_SETUP.md)** - SSL/TLS setup & troubleshooting
- **[LETS_ENCRYPT.md](LETS_ENCRYPT.md)** - Let's Encrypt integration guide
- **[PERMISSIONS.md](PERMISSIONS.md)** - Privilege handling explained
- **[INSTALL.md](INSTALL.md)** - Installation instructions

## 🎯 Usage Example

1. Start your backend service:
   ```bash
   # Ví dụ: NestJS trên port 4000
   npm run start
   ```

2. Open Domain Router UI

3. Click "Quick Setup (80 & 443)"

4. Enter target port: `4000`

5. Click "Start Proxy"

6. Test:
   ```bash
   curl http://localhost          # HTTP works!
   curl -k https://localhost      # HTTPS works!
   ```

## 🛠️ Technology Stack

### Frontend:
- React 19
- TypeScript
- Vite
- Tauri (Desktop UI)

### Backend:
- Rust
- Tokio (Async runtime)
- tokio-rustls (TLS)
- rcgen (Certificate generation)

## 📦 Available Scripts

| Command | Description |
|---------|-------------|
| `npm install` | Install dependencies |
| `npm run build` | Build frontend only |
| `npm run build:all` | **Build everything (frontend + backend + grant caps)** |
| `npm run tauri:dev` | Development mode with hot reload |
| `npm run tauri:build` | Production build |

## 🔐 Security

- **Linux Capabilities** - Only grants `CAP_NET_BIND_SERVICE`, not full root
- **pkexec Integration** - GUI password dialog for privilege escalation
- **Self-signed Certificates** - Automatic generation and secure storage
- **TLS 1.3** - Modern encryption standards

## ⚙️ Requirements

- Ubuntu 20.04+ (tested on 22.04 & 24.04)
- Node.js 20+
- Rust 1.75+
- 2GB RAM minimum
- Ports 80 and 443 available

## 🐛 Troubleshooting

### Port already in use:
```bash
sudo lsof -i :80
sudo systemctl stop nginx  # or apache2
```

### Permission denied on ports 80/443:
```bash
sudo setcap 'cap_net_bind_service=+ep' src-tauri/target/release/domain-router
```

### SSL certificate errors:
```bash
rm -rf ~/.config/domain-router/certs
# Restart app - certificates will be regenerated
```

More help: See [HUONG_DAN_BUILD_RUN.md](HUONG_DAN_BUILD_RUN.md#kiểm-tra-lỗi)

## 📊 Project Structure

```
domain-router/
├── src/                      # Frontend (React + TypeScript)
│   ├── App.tsx
│   ├── components/
│   └── api.ts
├── src-tauri/                # Backend (Rust)
│   ├── src/
│   │   ├── proxy/           # Reverse proxy engine
│   │   ├── ssl/             # Certificate management
│   │   ├── acme/            # Let's Encrypt (foundation)
│   │   └── privilege.rs     # pkexec integration
│   └── Cargo.toml
├── scripts/
│   └── build-and-setup.sh   # All-in-one build script
└── docs/                     # Documentation
```

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📝 License

MIT License - See LICENSE file

## 💬 Support

- Issues: [GitHub Issues](https://github.com/your-repo/issues)
- Docs: See `docs/` folder
- Email: support@example.com

---

**Built with ❤️ using Rust + React + Tauri**
