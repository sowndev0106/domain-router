# SSL/TLS Support

## Overview

Domain Router now supports HTTPS with SSL/TLS termination! When you enable SSL for port 80, the proxy automatically:
1. Creates a port 443 (HTTPS) listener
2. Generates or loads a self-signed certificate
3. Terminates TLS/SSL on port 443
4. Forwards decrypted HTTP traffic to your backend

## How It Works

```
Client (HTTPS) → Port 443 (TLS Termination) → Backend (Plain HTTP)
Client (HTTP)  → Port 80                     → Backend (Plain HTTP)
```

### Quick Setup

1. Click "Quick Setup (80 & 443)" button in the UI
2. Enter your backend target port (e.g., 4000)
3. Click "Add Routes"
4. Click "Start Proxy"

This will automatically:
- Create route: `localhost:80 → 127.0.0.1:4000` (HTTP)
- Create route: `localhost:443 → 127.0.0.1:4000` (HTTPS with TLS termination)
- Generate a self-signed certificate for "localhost"

### Testing

**Test HTTP:**
```bash
curl http://localhost:80
```

**Test HTTPS:**
```bash
curl -k https://localhost:443
# -k flag ignores SSL certificate validation (needed for self-signed certs)
```

**Test with full certificate validation:**
```bash
# First, get the certificate location
ls ~/.config/domain-router/certs/

# Then use curl with the certificate
curl --cacert ~/.config/domain-router/certs/localhost.crt https://localhost
```

## Certificate Management

### Self-Signed Certificates

Certificates are automatically generated on first use and stored in:
```
~/.config/domain-router/certs/
├── localhost.crt  (Certificate)
└── localhost.key  (Private Key)
```

### Certificate Details

The self-signed certificates include:
- Common Name (CN): The domain name (e.g., "localhost")
- Subject Alternative Names (SANs):
  - The exact domain
  - Wildcard for subdomains (e.g., *.localhost)
  - "localhost"

### Regenerating Certificates

If you need to regenerate certificates:
```bash
rm ~/.config/domain-router/certs/localhost.*
# Restart the proxy - new certificates will be generated
```

## Let's Encrypt (Future)

Let's Encrypt support is planned for production use with real domains. Currently, the system uses self-signed certificates which are perfect for:
- Development
- Internal networks
- Testing
- Docker container access

For production with real domains, you'll be able to:
1. Configure your domain in the UI
2. Enable "Let's Encrypt" mode
3. The proxy will automatically obtain and renew valid SSL certificates

## Security Notes

- ✅ TLS termination happens at the proxy
- ✅ Backend communication is plain HTTP (fast, no double encryption)
- ✅ Private keys are stored securely in `~/.config/domain-router/certs/`
- ✅ Self-signed certificates are regenerated if deleted
- ⚠️ Self-signed certificates will show browser warnings (this is normal)
- ⚠️ For production, use Let's Encrypt or provide your own certificates

## Troubleshooting

### Browser shows "Your connection is not private"

This is expected with self-signed certificates. You can:
1. Click "Advanced" → "Proceed to localhost (unsafe)" 
2. Or use curl with `-k` flag
3. Or add the certificate to your system's trusted certificates

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

1. Make sure proxy is running (`lsof -i :443`)
2. Check logs for TLS errors
3. Verify certificate files exist
4. Try regenerating certificates

## Technical Implementation

- **TLS Library**: `tokio-rustls` with `rustls` backend
- **Certificate Generation**: `rcgen` library
- **TLS Termination**: Port 443 accepts TLS connections, decrypts, forwards to backend
- **HTTP Passthrough**: Port 80 forwards traffic as-is (no TLS)

## Architecture

[src-tauri/src/ssl/mod.rs](src-tauri/src/ssl/mod.rs):
- Certificate generation
- Certificate storage/loading
- TLS configuration

[src-tauri/src/proxy/mod.rs](src-tauri/src/proxy/mod.rs):
- `start_https_server()`: HTTPS listener on port 443
- `handle_https_connection()`: TLS termination and forwarding
- `start_port_server()`: HTTP listener for other ports
