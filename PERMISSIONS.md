# Permission Handling

## Overview

Domain Router needs elevated permissions for two operations:

1. **Binding to privileged ports** (80, 443) - Linux only
2. **Modifying the hosts file** - Both Linux and Windows

The app handles these securely without requiring you to run as root/administrator.

## How It Works

### Linux

When you start the proxy with ports < 1024:

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

**What happens behind the scenes:**
1. App detects ports < 1024 are requested
2. Checks if binary has `CAP_NET_BIND_SERVICE` capability
3. If not, shows GUI dialog via `pkexec`
4. Runs: `setcap 'cap_net_bind_service=+ep' /path/to/binary`
5. App can now bind to privileged ports

### Windows

When modifying the hosts file:

```
┌─────────────────────────────────────────┐
│  User Account Control                   │
├─────────────────────────────────────────┤
│  Do you want to allow this app to       │
│  make changes to your device?           │
│                                         │
│  [Yes]                      [No]        │
└─────────────────────────────────────────┘
```

**What happens:**
1. App needs to modify `C:\Windows\System32\drivers\etc\hosts`
2. UAC prompt appears automatically
3. User clicks "Yes"
4. Hosts file is updated

**Note:** On Windows, binding to ports < 1024 does NOT require elevation.

## Security

### Linux Capabilities

Instead of running as root, we use Linux capabilities:

| Approach | Security Risk |
|----------|---------------|
| `sudo ./app` | Full root access - HIGH risk |
| `CAP_NET_BIND_SERVICE` | Only port binding - MINIMAL risk |

The capability grants:
- ✅ Bind to ports below 1024 (80, 443)
- ❌ Does NOT give root access
- ❌ Does NOT allow reading other users' files
- ❌ Does NOT allow modifying system files

### Windows UAC

Windows User Account Control provides:
- ✅ Granular permission for specific actions
- ✅ User sees exactly what's being requested
- ✅ No persistent elevation
- ❌ Does NOT run entire app as admin

## Permission Lifecycle

### Linux

```
First Run (ports 80/443):
  → Show pkexec dialog
  → Enter password
  → Grant capability
  → Capability persists on binary file

Subsequent Runs:
  → Check capability
  → Already granted
  → No dialog needed

After Rebuild:
  → Binary changed
  → Capability lost
  → Show dialog again
```

### Windows

```
Every Hosts File Change:
  → Show UAC prompt
  → User clicks Yes
  → Change applied
  → No persistent elevation
```

## Manual Control

### Linux

**Grant capability manually:**
```bash
sudo setcap 'cap_net_bind_service=+ep' /path/to/domain-router
```

**Check current capability:**
```bash
getcap /path/to/domain-router
# Output: cap_net_bind_service=ep (if granted)
```

**Revoke capability:**
```bash
sudo setcap -r /path/to/domain-router
```

### Windows

**Run as Administrator (alternative):**
1. Right-click the app
2. Select "Run as administrator"
3. No UAC prompts during session

**Disable UAC prompts (not recommended):**
- Not recommended for security reasons

## Troubleshooting

### Linux: Dialog not appearing

**Cause:** PolicyKit agent not running

**Solution:**
```bash
# Install policykit
sudo apt install policykit-1

# Ensure desktop environment has PolicyKit agent
# (Most desktop environments include this)
```

### Linux: Permission denied after dialog

**Verify capability:**
```bash
getcap /path/to/domain-router
```

**Re-grant if missing:**
```bash
sudo setcap 'cap_net_bind_service=+ep' /path/to/domain-router
```

### Windows: UAC not appearing

**Cause:** UAC might be disabled

**Solution:**
1. Open "User Account Control settings"
2. Ensure slider is not at "Never notify"
3. Restart computer

### Windows: Access denied to hosts file

**Solution 1:** Allow UAC prompt when it appears

**Solution 2:** Run app as administrator

**Solution 3:** Manually edit hosts file:
```
C:\Windows\System32\drivers\etc\hosts
```

## Implementation Details

### Linux (privilege.rs)

```rust
// Check if we need privilege
privilege::needs_privilege(&[80, 443]) // → true

// Check if we have it
privilege::has_capability() // → true/false

// Request it (shows GUI dialog)
privilege::request_capability() // → pkexec setcap ...

// All-in-one helper
privilege::ensure_capability_for_ports(&[80, 443])
```

### Windows (hosts/mod.rs)

```rust
// Check if elevated
is_elevated_windows() // Uses GetTokenInformation

// Copy with elevation
copy_with_elevation_windows(source, dest)
// Uses PowerShell Start-Process -Verb RunAs
```

## Files Involved

- `src-tauri/src/privilege.rs` - Linux capability handling
- `src-tauri/src/hosts/mod.rs` - Cross-platform hosts file management

## Best Practices

1. **Don't run as root/admin** - Let the app request specific permissions
2. **Grant capabilities after build** - Run post-install script
3. **Allow UAC prompts** - Don't disable UAC on Windows
4. **Review what's requested** - App only asks for what it needs
