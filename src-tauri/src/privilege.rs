use anyhow::Result;
use log::{info, error};

#[cfg(not(target_os = "windows"))]
use std::process::Command;

/// Check if we need elevated privileges for the given ports
pub fn needs_privilege(ports: &[u16]) -> bool {
    // On Windows, binding to ports < 1024 doesn't require special privileges
    #[cfg(target_os = "windows")]
    {
        let _ = ports;
        false
    }
    #[cfg(not(target_os = "windows"))]
    {
        ports.iter().any(|&port| port < 1024)
    }
}

/// Check if the current binary has CAP_NET_BIND_SERVICE capability (Linux only)
pub fn has_capability() -> bool {
    #[cfg(target_os = "windows")]
    {
        // Windows doesn't use Linux capabilities - always return true
        true
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Get the current executable path
        let exe_path = match std::env::current_exe() {
            Ok(path) => path,
            Err(_) => return false,
        };

        // Check capabilities using getcap
        let output = Command::new("getcap")
            .arg(&exe_path)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.contains("cap_net_bind_service")
            }
            Err(_) => false,
        }
    }
}

/// Grant CAP_NET_BIND_SERVICE capability to the current binary using pkexec
/// This will show a GUI dialog asking for the user's password (Linux only)
pub fn request_capability() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        // Windows doesn't need special capabilities for port binding
        info!("Windows: No special capability needed for port binding");
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        info!("Requesting privilege to bind to ports 80 and 443...");

        // Get the current executable path
        let exe_path = std::env::current_exe()
            .map_err(|e| anyhow::anyhow!("Failed to get executable path: {}", e))?;

        let exe_path_str = exe_path.to_string_lossy();

        // Use pkexec to run setcap with elevated privileges
        // This will show a GUI dialog
        let status = Command::new("pkexec")
            .arg("setcap")
            .arg("cap_net_bind_service=+ep")
            .arg(exe_path_str.as_ref())
            .status()
            .map_err(|e| anyhow::anyhow!("Failed to execute pkexec: {}", e))?;

        if status.success() {
            info!("Successfully granted capability to bind to privileged ports");
            Ok(())
        } else {
            error!("Failed to grant capability - user may have cancelled or entered wrong password");
            Err(anyhow::anyhow!("Failed to grant capability"))
        }
    }
}

/// Check and request capability if needed for the given ports
/// Returns true if we have the necessary privileges, false otherwise
pub fn ensure_capability_for_ports(ports: &[u16]) -> Result<bool> {
    // Check if we need elevated privileges
    if !needs_privilege(ports) {
        info!("No privileged ports requested, no special permissions needed");
        return Ok(true);
    }

    info!("Privileged ports requested: {:?}", ports);

    // Check if we already have the capability
    if has_capability() {
        info!("Binary already has CAP_NET_BIND_SERVICE capability");
        return Ok(true);
    }

    info!("Binary does not have capability, requesting via GUI dialog...");

    // Request capability via GUI dialog
    match request_capability() {
        Ok(_) => {
            info!("Capability granted successfully");
            Ok(true)
        }
        Err(e) => {
            error!("Failed to obtain capability: {}", e);
            // Return the error but don't crash - let the user know they need to manually grant it
            Err(anyhow::anyhow!(
                "Failed to obtain permission to bind to ports 80/443. \
                 You can either:\n\
                 1. Run this command manually: sudo setcap 'cap_net_bind_service=+ep' {}\n\
                 2. Or run the application with sudo",
                std::env::current_exe().unwrap_or_default().display()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needs_privilege() {
        assert!(needs_privilege(&[80, 443]));
        assert!(needs_privilege(&[22, 8080]));
        assert!(!needs_privilege(&[8080, 8443]));
        assert!(!needs_privilege(&[3000]));
    }
}
