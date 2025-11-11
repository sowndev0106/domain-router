# Let's Encrypt Integration

## Overview

Let's Encrypt support has been **partially implemented** with the foundation in place. Full implementation requires additional work due to complexity.

## Current Status

### ✅ Completed:
- ACME client module (`src-tauri/src/acme/mod.rs`)
- `rustls-acme` dependency added
- Certificate caching infrastructure
- Staging/Production server configuration

### ⏳ Remaining Work:
1. **HTTP-01 Challenge Handler** - Server to respond to Let's Encrypt validation
2. **Auto-renewal Logic** - Background task to renew before expiry
3. **Integration with Proxy** - Use ACME certificates in HTTPS server
4. **UI Configuration** - Email & domain input in frontend

## Why Let's Encrypt is Complex

### Requirements:
1. **Public Domain** - Must have a real domain (not localhost)
2. **Public IP** - Domain must resolve to server's public IP
3. **Port 80 Access** - Let's Encrypt must reach http://yourdomain/.well-known/acme-challenge/
4. **DNS Configuration** - Domain A record pointing to your server

### How It Works:

```
┌─────────────┐                           ┌──────────────────┐
│ Your Server │                           │  Let's Encrypt   │
│             │                           │   CA Server      │
└──────┬──────┘                           └────────┬─────────┘
       │                                            │
       │  1. Request cert for "api.example.com"    │
       ├──────────────────────────────────────────>│
       │                                            │
       │  2. Challenge: "Put file at /.well-known/ │
       │     with content: abc123xyz"               │
       │<───────────────────────────────────────────┤
       │                                            │
       │  3. Create file + start HTTP server       │
       │                                            │
       │           4. Validate via HTTP             │
       │<───────────────────────────────────────────┤
       │     GET http://api.example.com/.well-known/acme-challenge/abc123xyz
       │                                            │
       │  5. Return challenge response             │
       ├──────────────────────────────────────────>│
       │                                            │
       │  6. Issue Certificate (valid 90 days)     │
       │<───────────────────────────────────────────┤
       │                                            │
```

## Implementation Plan

### Phase 1: Foundation (✅ DONE)
- ACME module structure
- Configuration types
- Certificate cache

### Phase 2: Challenge Handler (TODO)
```rust
// Need to implement in proxy/mod.rs
async fn handle_acme_challenge(
    req: Request,
    challenge_store: Arc<RwLock<HashMap<String, String>>>
) -> Response {
    // Serve /.well-known/acme-challenge/{token}
}
```

### Phase 3: Certificate Acquisition (TODO)
```rust
use rustls_acme::AcmeConfig;

// In ssl/mod.rs
pub async fn get_letsencrypt_certificate(
    domain: String,
    email: String,
) -> Result<Arc<ServerConfig>> {
    let acme_manager = AcmeManager::new(email, false)?;
    let config = acme_manager.get_acme_config(vec![domain])?;
    
    // Start ACME state machine
    let mut state = config.state();
    let acceptor = state.acceptor();
    
    // Handle challenges...
    // Wait for certificate...
    
    Ok(Arc::new(acceptor.into()))
}
```

### Phase 4: Auto-Renewal (TODO)
```rust
// Background task
tokio::spawn(async move {
    loop {
        tokio::time::sleep(Duration::from_days(60)).await;
        
        // Check certificate expiry
        if cert_expires_soon() {
            renew_certificate().await?;
        }
    }
});
```

### Phase 5: UI Integration (TODO)
- Add email input field
- Add domain input field  
- "Use Let's Encrypt" checkbox
- Staging/Production toggle

## Testing

### ⚠️ IMPORTANT: Always test with STAGING first!

Let's Encrypt has **rate limits**:
- **Production**: 50 certificates per week per domain
- **Staging**: Much higher limits (for testing)

```rust
// Use staging for testing
let manager = AcmeManager::new(email, true); // true = staging

// Switch to production only when confident
let manager = AcmeManager::new(email, false); // false = production
```

### Testing Checklist:
1. ✅ Have a real domain
2. ✅ Domain points to your server's public IP
3. ✅ Port 80 is accessible from internet
4. ✅ Firewall allows inbound port 80
5. ✅ Start with staging server
6. ✅ Verify certificate obtained successfully
7. ✅ Switch to production

## Current Solution

For now, the app uses **self-signed certificates** which is perfect for:
- ✅ Development on localhost
- ✅ Internal networks
- ✅ Docker containers
- ✅ Testing

## Future: Completing Let's Encrypt

If you need Let's Encrypt for production, here's what to do:

1. **Ensure Requirements**:
   - Real domain pointing to public IP
   - Port 80 accessible from internet

2. **Implement Challenge Handler**:
   ```rust
   // Add to proxy/mod.rs
   if req.path().starts_with("/.well-known/acme-challenge/") {
       return serve_acme_challenge(req).await;
   }
   ```

3. **Use `rustls-acme` State Machine**:
   ```rust
   let mut state = acme_config.state();
   while let Some(event) = state.next().await {
       match event {
           Ok(Event::ChallengeReady { .. }) => { /* serve challenge */ }
           Ok(Event::CertReady) => { /* use certificate */ }
           Err(e) => { /* handle error */ }
       }
   }
   ```

4. **Add Auto-Renewal**:
   - Check certificate expiry every day
   - Renew 30 days before expiry
   - Restart HTTPS server with new cert

## References

- [Let's Encrypt Documentation](https://letsencrypt.org/docs/)
- [rustls-acme Crate](https://docs.rs/rustls-acme/)
- [ACME Protocol RFC 8555](https://tools.ietf.org/html/rfc8555)

## Summary

Let's Encrypt support is **foundational work is done**, but full implementation is complex and requires:
- Real domain with public IP
- Challenge handler implementation
- Certificate renewal logic
- UI for configuration

For local development, **self-signed certificates** are the right choice and are already working perfectly!
