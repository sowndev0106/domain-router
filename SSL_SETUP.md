# SSL/TLS Configuration

## Overview

Domain Router supports HTTPS with automatic SSL/TLS termination. When you enable SSL for a route:

1. Port 443 listener is created automatically
2. Self-signed certificate is generated (or loaded from cache)
3. TLS termination happens at the proxy
4. Decrypted HTTP traffic is forwarded to your backend

## How It Works

```
Client (HTTPS) → Port 443 (TLS Termination) → Backend (Plain HTTP)
Client (HTTP)  → Port 80                     → Backend (Plain HTTP)
```

## Quick Setup

1. Click "Quick Setup (80 & 443)" in the UI
2. Enter your backend target port (e.g., `3000`)
3. Click "Add Routes"
4. Click "Start Proxy"

This automatically:
- Creates `localhost:80 → 127.0.0.1:3000` (HTTP)
- Creates `localhost:443 → 127.0.0.1:3000` (HTTPS with TLS)
- Generates a self-signed certificate for "localhost"

## Testing

**Test HTTP:**
```bash
curl http://localhost
```

**Test HTTPS:**
```bash
# -k flag ignores SSL validation (needed for self-signed certs)
curl -k https://localhost
```

**Test with certificate validation:**
```bash
# Use the generated certificate
curl --cacert ~/.config/domain-router/certs/localhost.crt https://localhost
```

## Certificate Management

### Storage Location

Certificates are stored in:
```
~/.config/domain-router/certs/
├── localhost.crt  # Certificate
└── localhost.key  # Private Key
```

On Windows:
```
%APPDATA%\domain-router\certs\
```

### Certificate Details

Generated certificates include:
- **Common Name (CN)**: The domain name
- **Subject Alternative Names (SANs)**:
  - The exact domain
  - Wildcard for subdomains (e.g., `*.localhost`)
  - `localhost`

### Regenerating Certificates

If you need fresh certificates:
```bash
# Linux
rm ~/.config/domain-router/certs/localhost.*

# Windows (PowerShell)
Remove-Item "$env:APPDATA\domain-router\certs\localhost.*"
```

Restart the proxy - new certificates will be generated automatically.

## Browser Warnings

Self-signed certificates will show browser warnings like "Your connection is not private". This is expected behavior.

**To proceed:**
1. Click "Advanced"
2. Click "Proceed to localhost (unsafe)"

**Or add certificate to trusted store:**

**Linux:**
```bash
sudo cp ~/.config/domain-router/certs/localhost.crt /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

**Windows:**
1. Double-click the `.crt` file
2. Click "Install Certificate"
3. Select "Local Machine"
4. Select "Trusted Root Certification Authorities"

**macOS:**
```bash
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ~/.config/domain-router/certs/localhost.crt
```

## Troubleshooting

### Certificate errors

```bash
# Check if certificate exists
ls -la ~/.config/domain-router/certs/

# View certificate details
openssl x509 -in ~/.config/domain-router/certs/localhost.crt -text -noout

# Test TLS connection
openssl s_client -connect localhost:443 -servername localhost
```

### HTTPS not working

1. Verify proxy is running:
   ```bash
   lsof -i :443
   ```

2. Check logs for TLS errors

3. Verify certificate files exist

4. Try regenerating certificates

### Connection refused on port 443

- Ensure the proxy is started
- Check if another service is using port 443:
  ```bash
  sudo lsof -i :443
  ```

## Technical Details

- **TLS Library**: `tokio-rustls` with `rustls` backend
- **Certificate Generation**: `rcgen` library
- **Supported TLS Versions**: TLS 1.2, TLS 1.3
- **Key Size**: 2048-bit RSA

## Let's Encrypt (Future)

Let's Encrypt support is planned for production use with real domains. The foundation is in place but requires:
- Public domain with valid DNS
- Port 80 accessible from internet
- HTTP-01 challenge handler implementation

For now, self-signed certificates are perfect for:
- Development
- Internal networks
- Docker container access
- Testing

## Security Notes

- TLS termination happens at the proxy
- Backend communication is plain HTTP (no double encryption)
- Private keys are stored with user-only permissions
- Self-signed certificates are regenerated if deleted
- For production with real domains, use Let's Encrypt or provide your own certificates
