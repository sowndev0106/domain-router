# Domain Router

A cross-platform reverse proxy and SSL/TLS manager for localhost and Docker containers.

## Downloads

| Platform | File | Size |
|----------|------|------|
| **Windows** | [Domain Router_1.0.0_x64-setup.exe](releases/Domain%20Router_1.0.0_x64-setup.exe) | ~2 MB |
| **Linux (AppImage)** | [Domain Router_1.0.0_amd64.AppImage](releases/Domain%20Router_1.0.0_amd64.AppImage) | ~77 MB |
| **Linux (Deb)** | [Domain Router_1.0.0_amd64.deb](releases/Domain%20Router_1.0.0_amd64.deb) | ~3.5 MB |

### Quick Install

**Windows:**
```
Download and run Domain Router_1.0.0_x64-setup.exe
```

**Linux (AppImage):**
```bash
chmod +x "Domain Router_1.0.0_amd64.AppImage"
./"Domain Router_1.0.0_amd64.AppImage"
```

**Linux (Debian/Ubuntu):**
```bash
sudo dpkg -i "Domain Router_1.0.0_amd64.deb"
```

## Features

- **HTTP Reverse Proxy** - Port forwarding and domain routing
- **HTTPS with Self-Signed Certificates** - Automatic TLS termination
- **Quick Setup (80 & 443)** - One-click configuration for HTTP + HTTPS
- **Cross-Platform** - Works on Windows and Linux
- **Privilege Escalation** - GUI dialog for elevated permissions (no manual sudo)
- **Let's Encrypt Foundation** - Ready for production (partial implementation)

## Quick Start

### Using Pre-built Downloads

1. Download the installer for your platform from the table above
2. Install and run the application
3. Click "Quick Setup (80 & 443)"
4. Enter your backend target port (e.g., `3000`)
5. Click "Start Proxy"

### Building from Source

```bash
# Clone and enter the directory
cd domain-router

# Install dependencies
npm install

# Build everything
npm run build:all

# Run
./src-tauri/target/release/domain-router
```

## Usage Example

1. Start your backend service:
   ```bash
   # Example: Node.js app on port 3000
   npm run start
   ```

2. Open Domain Router

3. Click "Quick Setup (80 & 443)"

4. Enter target port: `3000`

5. Click "Start Proxy"

6. Test:
   ```bash
   curl http://localhost          # HTTP works!
   curl -k https://localhost      # HTTPS works!
   ```

## How It Works

```
                    ┌─────────────────────────────────────┐
                    │          Domain Router              │
                    │                                     │
Browser ─────────── │  Port 80  ──────────────────────── │ ───► Backend
(HTTP)              │  (HTTP Proxy)                       │      (Port 3000)
                    │                                     │
Browser ─────────── │  Port 443 ──────────────────────── │ ───► Backend
(HTTPS)             │  (TLS Termination + Proxy)          │      (Port 3000)
                    │                                     │
                    └─────────────────────────────────────┘
```

- **Port 80**: Plain HTTP passthrough
- **Port 443**: TLS termination with auto-generated self-signed certificates

## Technology Stack

**Frontend:**
- React 19
- TypeScript
- Vite
- Tauri (Desktop UI)

**Backend:**
- Rust
- Tokio (Async runtime)
- tokio-rustls (TLS)
- rcgen (Certificate generation)

## Requirements

### For Users (Pre-built downloads):
- **Windows**: Windows 10/11 (x64)
- **Linux**: Ubuntu 20.04+ / Debian 11+ (x64)

### For Developers (Building from source):
- Node.js 20+
- Rust 1.75+
- 2GB RAM minimum

## Platform-Specific Notes

### Windows
- UAC prompt will appear when modifying the hosts file
- No special permissions needed for port binding

### Linux
- Uses `pkexec` for privilege escalation (GUI password dialog)
- Requires `CAP_NET_BIND_SERVICE` capability for ports < 1024
- The app will automatically request permissions when needed

## Security

- **Linux Capabilities** - Only grants `CAP_NET_BIND_SERVICE`, not full root
- **pkexec Integration** - GUI password dialog for privilege escalation
- **Self-signed Certificates** - Automatic generation and secure storage
- **TLS 1.2/1.3** - Modern encryption standards

## Troubleshooting

### Port already in use
```bash
# Check what's using port 80
sudo lsof -i :80

# Stop nginx/apache if running
sudo systemctl stop nginx
```

### Permission denied on ports 80/443 (Linux)
```bash
sudo setcap 'cap_net_bind_service=+ep' /path/to/domain-router
```

### SSL certificate errors
```bash
# Delete old certificates (they will regenerate)
rm -rf ~/.config/domain-router/certs
```

### Hosts file not updating (Windows)
- Run the application as Administrator, or
- Allow the UAC prompt when it appears

## Project Structure

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
│   │   ├── hosts/           # Hosts file management
│   │   ├── acme/            # Let's Encrypt (foundation)
│   │   └── privilege.rs     # Permission handling
│   └── Cargo.toml
├── releases/                 # Pre-built binaries
└── docs/                     # Documentation
```

## Available Scripts

| Command | Description |
|---------|-------------|
| `npm install` | Install dependencies |
| `npm run build` | Build frontend only |
| `npm run build:all` | Build everything (frontend + backend + permissions) |
| `npm run tauri:dev` | Development mode with hot reload |
| `npm run tauri:build` | Production build |

## Documentation

- [BUILD.md](BUILD.md) - Detailed build instructions
- [SSL_SETUP.md](SSL_SETUP.md) - SSL/TLS configuration
- [PERMISSIONS.md](PERMISSIONS.md) - Permission handling explained

## Contributing

Contributions welcome! Please:
1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - See LICENSE file

---

**Built with Rust + React + Tauri**
