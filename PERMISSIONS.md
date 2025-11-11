# How Permission Handling Works

## TL;DR
Khi bạn start proxy với ports 80/443, app sẽ **tự động hiện dialog xin quyền** và nhập password. Bạn không cần chạy sudo!

## Chi tiết

### 1. Automatic Permission Request

Khi bạn click "Start Proxy" với port mappings sử dụng ports < 1024 (ví dụ: 80, 443):

```
┌─────────────────────────────────────────┐
│  Authentication Required                │
├─────────────────────────────────────────┤
│  Domain Router needs to bind to         │
│  privileged ports (80, 443)             │
│                                         │
│  Password: [__________________]         │
│                                         │
│  [Cancel]              [Authenticate]   │
└─────────────────────────────────────────┘
```

### 2. What Happens Behind the Scenes

1. App detects bạn muốn bind ports < 1024
2. Check xem binary đã có `CAP_NET_BIND_SERVICE` capability chưa
3. Nếu chưa có:
   - Hiện GUI dialog (qua `pkexec`)
   - Yêu cầu password
   - Chạy: `setcap 'cap_net_bind_service=+ep' /path/to/binary`
4. Sau đó app bind ports bình thường

### 3. Security

- ✅ **An toàn**: Chỉ cấp quyền bind ports < 1024
- ✅ **Không cần root**: App chạy như user bình thường
- ✅ **Transparent**: Bạn thấy dialog khi cần quyền
- ✅ **One-time**: Chỉ cần nhập password MỘT LẦN
- ✅ **Revokable**: Có thể thu hồi bất cứ lúc nào

### 4. Permission Lifecycle

```
First Run (ports 80/443):
  → Show dialog → Enter password → Grant capability
  → Capability persists on binary

Subsequent Runs:
  → Check capability → Already has it → No dialog needed

After Rebuild:
  → Binary changed → Capability lost → Show dialog again
```

### 5. Manual Control

Nếu muốn grant trước:
```bash
npm run tauri build
./scripts/post-install.sh src-tauri/target/release/domain-router
```

Nếu muốn revoke:
```bash
sudo setcap -r src-tauri/target/release/domain-router
```

Check hiện tại:
```bash
getcap src-tauri/target/release/domain-router
# Output: cap_net_bind_service=ep (if granted)
```

### 6. Development Workflow

**Development (npm run tauri dev):**
- Dialog sẽ xuất hiện LẦN ĐẦU bạn start proxy với ports < 1024
- Sau đó không cần nhập password nữa (trong cùng dev session)
- Nếu rebuild, dialog hiện lại (binary thay đổi)

**Production (after build & install):**
- Dialog xuất hiện LẦN ĐẦU user chạy app
- Sau đó không cần nhập password nữa
- Capability persist qua các lần chạy app

### 7. Troubleshooting

**Dialog không hiện?**
- Đảm bảo `policykit-1` installed: `sudo apt install policykit-1`
- Check desktop environment có PolicyKit agent running

**Dialog bị cancel/fail?**
- App sẽ show error message với hướng dẫn
- Có thể chạy manual: `sudo setcap 'cap_net_bind_service=+ep' <binary-path>`
- Hoặc chạy với sudo (không recommended): `sudo ./domain-router`

**Permission denied ngay cả sau khi grant?**
- Verify: `getcap src-tauri/target/release/domain-router`
- Re-grant: `./scripts/post-install.sh src-tauri/target/release/domain-router`

## Implementation Details

File: [`src-tauri/src/privilege.rs`](src-tauri/src/privilege.rs)

```rust
// Check if we need privilege
privilege::needs_privilege(&[80, 443]) // → true

// Check if we have it
privilege::has_capability() // → false

// Request it (shows GUI dialog)
privilege::request_capability() // → pkexec setcap ...

// All-in-one helper
privilege::ensure_capability_for_ports(&[80, 443])
```

Called automatically in [`src-tauri/src/proxy/mod.rs`](src-tauri/src/proxy/mod.rs#L58-L63) when starting proxy.
