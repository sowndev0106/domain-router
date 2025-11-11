# Domain Router - Project Summary

## Overview
Domain Router là ứng dụng desktop (Tauri + React) để quản lý reverse proxy và SSL/TLS cho localhost và Docker containers.

## Features Implemented

### ✅ 1. HTTP Reverse Proxy
- TCP-level proxying
- Port mapping (e.g., 8080 → 4000)
- Domain routing
- Bidirectional traffic forwarding

### ✅ 2. HTTPS with Self-Signed Certificates
- Automatic certificate generation
- TLS termination on port 443
- Certificate caching in `~/.config/domain-router/certs/`
- Forward decrypted traffic to backend

### ✅ 3. Quick Setup (80 & 443)
- One-click setup for HTTP + HTTPS
- Automatically creates both port 80 and 443 mappings
- SSL enabled by default

### ✅ 4. Privilege Escalation (pkexec)
- GUI dialog for password when binding ports < 1024
- Uses Linux capabilities (`CAP_NET_BIND_SERVICE`)
- No need to run entire app as root
- Secure and follows Linux best practices

### ✅ 5. Let's Encrypt Foundation
- ACME client module structure
- Configuration types
- Certificate caching
- Ready for future implementation

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Frontend (React)                    │
│  - Route Management UI                                  │
│  - Quick Setup Dialog                                   │
│  - Proxy Status Display                                 │
└────────────────────┬────────────────────────────────────┘
                     │ Tauri IPC
┌────────────────────▼────────────────────────────────────┐
│                  Backend (Rust/Tauri)                   │
│                                                          │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │   Routes    │  │    Proxy     │  │  SSL/TLS      │  │
│  │  Management │  │   Engine     │  │  Manager      │  │
│  └─────────────┘  └──────────────┘  └───────────────┘  │
│                                                          │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │  Privilege  │  │     ACME     │  │    Hosts      │  │
│  │   Handler   │  │   (Future)   │  │   Manager     │  │
│  └─────────────┘  └──────────────┘  └───────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## How It Works

### HTTP Proxy (Port 80)
```
Browser → http://localhost:80 → Proxy (Port 80) → Backend (Port 4000)
                                  ↓ Plain TCP forwarding
                         {"message": "Hello"} ← Backend Response
```

### HTTPS Proxy (Port 443)
```
Browser → https://localhost:443 → Proxy (Port 443)
                                     ↓
                              TLS Handshake
                           (rustls + self-signed cert)
                                     ↓
                              Decrypt HTTPS
                                     ↓
                      → Backend (Port 4000) [Plain HTTP]
                                     ↓
                      ← Response (Plain HTTP)
                                     ↓
                           Encrypt with TLS
                                     ↓
Browser ← Encrypted Response ← Proxy
```

## File Structure

```
domain-router/
├── src/                          # Frontend (React + TypeScript)
│   ├── App.tsx                   # Main component
│   ├── components/
│   │   ├── QuickPortMappingDialog.tsx   # Quick setup UI
│   │   ├── RouteList.tsx
│   │   └── ...
│   └── api.ts                    # Tauri API calls
│
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs                # Main library
│   │   ├── routes/               # Route management
│   │   ├── proxy/                # Reverse proxy engine
│   │   │   └── mod.rs            # HTTP + HTTPS servers
│   │   ├── ssl/                  # Certificate management
│   │   │   └── mod.rs            # Self-signed cert generation
│   │   ├── acme/                 # Let's Encrypt (foundation)
│   │   │   └── mod.rs
│   │   ├── privilege.rs          # pkexec + capabilities
│   │   └── hosts/                # /etc/hosts management
│   └── Cargo.toml                # Dependencies
│
├── scripts/
│   └── post-install.sh           # Grant capabilities
│
├── PERMISSIONS.md                # Privilege handling docs
├── SSL_SETUP.md                  # SSL/TLS documentation
├── LETS_ENCRYPT.md               # Let's Encrypt guide
└── PROJECT_SUMMARY.md            # This file
```

## Key Technologies

### Frontend:
- **React** - UI framework
- **TypeScript** - Type safety
- **Tauri** - Desktop app framework

### Backend:
- **Rust** - System programming language
- **Tokio** - Async runtime
- **tokio-rustls** - TLS implementation
- **rustls** - Pure-Rust TLS library
- **rcgen** - Certificate generation
- **rustls-acme** - ACME protocol (Let's Encrypt)

## Security Features

1. **Linux Capabilities** - Only grant `CAP_NET_BIND_SERVICE`, not full root
2. **pkexec Integration** - GUI password dialog for privilege escalation
3. **Self-signed Certificates** - Automatic generation, secure storage
4. **TLS 1.3** - Modern encryption standards
5. **No Plain Text Passwords** - All privilege requests via OS mechanisms

## Current Limitations

1. **Self-signed Certificates Only** - Browser warnings expected (use `-k` with curl)
2. **Let's Encrypt Not Complete** - Foundation in place, needs:
   - Challenge handler
   - Auto-renewal
   - UI integration
3. **Linux Only** - Capabilities and pkexec are Linux-specific
4. **HTTP-01 Challenge Only** - DNS-01 not implemented

## Usage

### Development:
```bash
npm install
npm run tauri dev
```

### Production:
```bash
npm run tauri build
sudo setcap 'cap_net_bind_service=+ep' src-tauri/target/release/domain-router
./src-tauri/target/release/domain-router
```

### Quick Test:
1. Start NestJS on port 4000: `npm run start`
2. Open Domain Router UI
3. Click "Quick Setup (80 & 443)"
4. Enter target port: `4000`
5. Click "Start Proxy"
6. Test:
   ```bash
   curl http://localhost          # HTTP works
   curl -k https://localhost      # HTTPS works (ignore cert warning)
   ```

## Next Steps (If Needed)

### For Let's Encrypt:
1. Implement HTTP-01 challenge handler
2. Add certificate auto-renewal
3. Create UI for email/domain config
4. Test with staging server
5. Deploy to production with real domain

### For Production:
1. Package as .deb installer
2. Add systemd service
3. Create man pages
4. Add logging to file
5. Implement metrics/monitoring

## Documentation

- [PERMISSIONS.md](PERMISSIONS.md) - How privilege escalation works
- [SSL_SETUP.md](SSL_SETUP.md) - SSL/TLS setup and usage
- [LETS_ENCRYPT.md](LETS_ENCRYPT.md) - Let's Encrypt integration guide
- [INSTALL.md](INSTALL.md) - Installation instructions

## Conclusion

Domain Router hiện đã có:
- ✅ HTTP reverse proxy hoạt động
- ✅ HTTPS với self-signed certificates
- ✅ Quick setup cho ports 80 & 443
- ✅ Privilege escalation an toàn
- ✅ Foundation cho Let's Encrypt

Perfect cho **development và internal networks**!

Cho **production với domain thật**, cần complete Let's Encrypt implementation (có documentation chi tiết trong [LETS_ENCRYPT.md](LETS_ENCRYPT.md)).
