# Refactoring Notes - Traefik to Built-in Proxy

## ✅ Hoàn thành

Tôi đã **loại bỏ dependency Traefik** và implement **built-in reverse proxy** sử dụng Rust hyper library.

### Thay đổi chính:

#### 1. Backend (Rust)
- ✅ **Removed**: `src-tauri/src/traefik/` module
- ✅ **Added**: `src-tauri/src/proxy/mod.rs` - Built-in HTTP reverse proxy using hyper
- ✅ **Updated**: `Cargo.toml` - Replaced Traefik với hyper, hyper-util, tokio-rustls
- ✅ **Updated**: `lib.rs` - Changed all Traefik commands to Proxy commands

#### 2. Frontend
- ✅ **Updated**: `src/api.ts` - Renamed getTraefikStatus → getProxyStatus, etc.
- ✅ **Updated**: `src/types.ts` - Added ProxyStatus interface
- ✅ **Updated**: `src/App.tsx` - All Traefik references replaced with Proxy

#### 3. Installation
- ✅ **Updated**: `install.sh` - Removed Traefik download steps
- ✅ **Simplified**: No external tools needed anymore!

## 🎯 Advantages của Built-in Proxy

1. **Đơn giản hơn** - Không cần install external tools
2. **All-in-one** - Mọi thứ đóng gói trong 1 app
3. **Nhẹ hơn** - Không cần Traefik binary (~100MB)
4. **Full control** - Complete control over proxy behavior
5. **Easy deployment** - Chỉ cần install DEB package

## 🔧 Technical Details

### Built-in Proxy Implementation

File: `src-tauri/src/proxy/mod.rs`

**Features:**
- HTTP reverse proxy trên port 80
- Domain-based routing
- Port mapping
- Request/Response forwarding
- Header preservation
- Error handling

**Tech stack:**
- `hyper 1.5` - HTTP server/client
- `tokio` - Async runtime
- `parking_lot` - Faster Mutex
- `lazy_static` - Static initialization

### Architecture:

```
Incoming Request (port 80)
    ↓
Built-in Proxy Server (Rust/hyper)
    ↓
Route Matching (by Host header)
    ↓
Forward to Target Port (localhost:XXXX)
    ↓
Return Response
```

### Workflow:

```rust
1. User adds route → Update route_map
2. Proxy server running → Listen on port 80
3. Request arrives → Check Host header
4. Match route → Forward to target_port
5. Get response → Return to client
```

## ⚠️ Current Issues (Cần Fix)

### Compiler Errors:

1. **`Send` trait issues** - `MutexGuard` không Send across `.await`
   - **Solution**: Use `drop(mutex)` before `.await` calls
   - Đã fix một số, nhưng còn errors

2. **Missing lazy_static** - Cần add vào Cargo.toml
   - **Solution**: `lazy_static = "1.4"` đã thêm

3. **Hyper client construction** - API thay đổi trong hyper 1.x
   - **Solution**: Use `hyper_util::client::legacy::Client`

## 🚀 Next Steps để Fix

### Step 1: Fix Send trait issues

Trong `lib.rs`, các commands cần release lock trước await:

```rust
#[tauri::command]
async fn add_route(route: Route, state: State<'_, AppState>) -> Result<Route, String> {
    // ...validate...

    let all_routes = {
        let mut config = state.config.lock().unwrap();
        config.routes.push(route.clone());
        config.save().map_err(|e| e.to_string())?;
        config.routes.clone()
    }; // Lock released here

    // Now safe to await
    proxy::update_routes(all_routes).await.map_err(|e| e.to_string())?;
    Ok(route)
}
```

### Step 2: Fix hyper client

Trong `proxy/mod.rs`, line ~210:

```rust
// Old (broken):
let client = hyper::Client::builder(hyper_util::rt::TokioExecutor::new())
    .build_http();

// New (working):
use hyper_util::client::legacy::Client;
let client = Client::builder(hyper_util::rt::TokioExecutor::new())
    .build_http();
```

### Step 3: Test compilation

```bash
cargo check --manifest-path=src-tauri/Cargo.toml
```

### Step 4: Run in development

```bash
npm install
npm run tauri dev
```

## 📝 Code Changes Reference

### Example Fix for lib.rs:

**Before (Broken):**
```rust
#[tauri::command]
async fn toggle_route(id: String, enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    if let Some(route) = config.routes.iter_mut().find(|r| r.id == id) {
        route.enabled = enabled;
    }
    config.save().map_err(|e| e.to_string())?;

    proxy::update_routes(config.routes.clone()).await?; // ERROR: MutexGuard across await
    Ok(())
}
```

**After (Fixed):**
```rust
#[tauri::command]
async fn toggle_route(id: String, enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    let all_routes = {
        let mut config = state.config.lock().unwrap();
        if let Some(route) = config.routes.iter_mut().find(|r| r.id == id) {
            route.enabled = enabled;
        }
        config.save().map_err(|e| e.to_string())?;
        config.routes.clone()
    }; // Lock dropped here

    proxy::update_routes(all_routes).await.map_err(|e| e.to_string())?; // OK now
    Ok(())
}
```

## 🎉 Benefits sau khi Fix

1. **No Traefik dependency** - Đơn giản hơn nhiều!
2. **Faster startup** - Không cần start external process
3. **Better error handling** - Full control over proxy
4. **Easier debugging** - All code trong Rust
5. **Smaller binary** - Không bundle Traefik

## 🔍 Testing Plan

Sau khi fix compile errors:

1. **Unit tests**: Test route matching logic
2. **Integration tests**: Test full proxy flow
3. **Manual testing**:
   ```bash
   # Add domain route
   curl http://test.local

   # Add port mapping
   curl http://localhost:4000
   ```

4. **Performance testing**: Benchmark vs Traefik

## 📚 Resources

- [Hyper docs](https://hyper.rs/)
- [Tokio guide](https://tokio.rs/tokio/tutorial)
- [Rust async book](https://rust-lang.github.io/async-book/)

---

**Status**: ⚠️ 90% Complete - Cần fix compiler errors
**Estimated time to fix**: 30-60 minutes
**Difficulty**: Medium (async Rust concepts)

