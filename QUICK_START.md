# Domain Router - Quick Start Guide

## 🚀 Bắt đầu nhanh trong 5 phút

### Bước 1: Cài đặt (3 phút)

```bash
# Clone project
git clone <your-repo-url>
cd dynamic-routing

# Cài đặt tự động (recommended)
chmod +x install.sh
./install.sh
```

Hoặc chỉ cài dependencies (nếu bạn muốn build thủ công):

```bash
# Install system packages
sudo apt install -y curl wget build-essential libssl-dev \
  libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf

# Install built-in reverse proxy
sudo wget -O /usr/local/bin/ \
  https://github.com///releases/latest/download/_linux_amd64
sudo chmod +x /usr/local/bin/

# Install Node.js (if needed)
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Bước 2: Build và Run (2 phút)

#### Development Mode (Recommended để test)

```bash
# Install dependencies
npm install

# Run in dev mode
npm run tauri dev
```

#### Production Build

```bash
# Build release
npm run tauri build

# Install DEB package
sudo dpkg -i target/release/bundle/deb/domain-router_*.deb

# Run application
domain-router
```

### Bước 3: Sử dụng

#### Example 1: Redirect Domain về Localhost

**Scenario**: Bạn có website `seller-dev.openlive.lotte.vn` đang chạy trên production, muốn test locally trên port 80.

1. **Start your local server**
   ```bash
   # Giả sử bạn chạy service trên port 80
   python3 -m http.server 80
   # hoặc
   npm run dev -- --port 80
   ```

2. **Add Route trong Domain Router**
   - Click **"Add Route"**
   - Select **"Domain Route"**
   - Domain: `seller-dev.openlive.lotte.vn`
   - Target Port: `80`
   - Enable HTTPS: `✓` (recommended)
   - SSL Mode: `Self-Signed (Auto)`
   - Click **"Add Route"**

3. **App sẽ tự động:**
   - Thêm `127.0.0.1 seller-dev.openlive.lotte.vn` vào `/etc/hosts`
   - Generate SSL certificate
   - Configure built-in reverse proxy routing

4. **Test**
   ```bash
   curl http://seller-dev.openlive.lotte.vn
   # or
   curl https://seller-dev.openlive.lotte.vn -k
   ```

5. **Truy cập browser:**
   - Mở `https://seller-dev.openlive.lotte.vn`
   - Accept self-signed certificate warning
   - Done! Website của bạn chạy locally!

#### Example 2: Port Mapping với SSL

**Scenario**: Service chạy trên port 8080, muốn access qua HTTPS port 4000.

1. **Start your service**
   ```bash
   # Service của bạn chạy trên 8080
   node server.js
   # hoặc
   python api.py
   ```

2. **Add Port Mapping**
   - Click **"Add Route"**
   - Select **"Port Mapping"**
   - Source Port: `4000`
   - Target Port: `8080`
   - Enable HTTPS: `✓`
   - SSL Mode: `Passthrough` hoặc `Self-Signed`
   - Click **"Add Route"**

3. **Access**
   ```bash
   curl https://localhost:4000 -k
   ```

## 🎯 Use Cases phổ biến

### Case 1: Test Production Domain Locally

```
Production: https://api.example.com → AWS server
Local: https://api.example.com → localhost:3000
```

**Steps:**
1. Add domain route: `api.example.com` → port `3000`
2. Start local API: `npm start` (port 3000)
3. Test như production: `curl https://api.example.com`

### Case 2: Multiple Microservices

```
https://api.local.dev → localhost:3000 (API)
https://web.local.dev → localhost:8080 (Frontend)
https://admin.local.dev → localhost:4000 (Admin)
```

Add 3 domain routes, mỗi route trỏ về port khác nhau.

### Case 3: SSL Debugging

```
http://localhost:8080 → https://localhost:4000
```

Add port mapping để test SSL behavior của application.

## ⚡ Commands Nhanh

```bash
# Start built-in reverse proxy (nếu chưa chạy)
# Click "Start" button trong app footer

# Check built-in reverse proxy status
/usr/local/bin/ version

# View routes
cat ~/.config/domain-router/config.json

# View built-in reverse proxy config
cat ~/.config/domain-router//dynamic.yml

# Check /etc/hosts entries
grep "Domain Router" /etc/hosts -A 10

# Backup your hosts file
sudo cp /etc/hosts /etc/hosts.backup

# Restore hosts backup
sudo cp ~/.config/domain-router/hosts.backup /etc/hosts
```

## 🐛 Troubleshooting Nhanh

### Port 80/443 in use
```bash
sudo lsof -i :80
sudo lsof -i :443
# Stop service sử dụng port
sudo systemctl stop nginx  # example
```

### built-in reverse proxy not starting
```bash
# Check binary
which 
 version

# Test config
 --configFile ~/.config/domain-router//.yml
```

### Domain không resolve
```bash
# Check /etc/hosts
cat /etc/hosts | grep "your-domain.com"

# Manually add if needed
echo "127.0.0.1 your-domain.com" | sudo tee -a /etc/hosts

# Flush DNS cache
sudo systemd-resolve --flush-caches
```

### SSL Certificate Error
```bash
# Regenerate certs
rm -rf ~/.config/domain-router/certs/
# Then add route lại trong app
```

## 📚 Next Steps

- Đọc [README.md](README.md) đầy đủ để hiểu chi tiết
- Xem [Requirements Document](REQUIREMENTS.md) để hiểu architecture
- Check [Issues](https://github.com/your-repo/issues) nếu có problem

## 💡 Tips

1. **Always start built-in reverse proxy trước** bằng button trong app footer
2. **Check port availability** trước khi add route
3. **Use self-signed SSL** cho local development (đơn giản nhất)
4. **Keep routes organized** - disable routes không dùng thay vì xóa
5. **Backup /etc/hosts** trước khi add nhiều routes

## ✅ Checklist

- [ ] built-in reverse proxy installed (`/usr/local/bin/`)
- [ ] Rust installed (`rustc --version`)
- [ ] Node.js installed (`node --version`)
- [ ] App compiled successfully
- [ ] built-in reverse proxy running (green status in footer)
- [ ] First route added
- [ ] Domain resolves to localhost
- [ ] Service accessible through domain

---

**Happy Routing! 🚀**
