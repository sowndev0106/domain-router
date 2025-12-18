use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

#[cfg(target_os = "windows")]
const HOSTS_FILE: &str = r"C:\Windows\System32\drivers\etc\hosts";

#[cfg(not(target_os = "windows"))]
const HOSTS_FILE: &str = "/etc/hosts";

const MARKER_START: &str = "# === Domain Router START ===";
const MARKER_END: &str = "# === Domain Router END ===";

pub fn add_entry(domain: &str) -> Result<()> {
    // Check if we need sudo
    if !is_root() {
        return add_entry_with_sudo(domain);
    }

    let current = read_hosts()?;
    let new_content = add_domain_to_content(&current, domain)?;
    write_hosts(&new_content)?;

    Ok(())
}

pub fn remove_entry(domain: &str) -> Result<()> {
    if !is_root() {
        return remove_entry_with_sudo(domain);
    }

    let current = read_hosts()?;
    let new_content = remove_domain_from_content(&current, domain)?;
    write_hosts(&new_content)?;

    Ok(())
}

/// Toggle entry by commenting/uncommenting instead of removing
pub fn toggle_entry(domain: &str, enabled: bool) -> Result<()> {
    if !is_root() {
        return toggle_entry_with_sudo(domain, enabled);
    }

    let current = read_hosts()?;
    let new_content = toggle_domain_in_content(&current, domain, enabled)?;
    write_hosts(&new_content)?;

    Ok(())
}

/// Get all domain entries from /etc/hosts (both active and commented)
pub fn get_all_entries() -> Result<Vec<(String, bool)>> {
    let content = read_hosts()?;
    parse_domain_entries(&content)
}

/// Toggle any entry in /etc/hosts (comment/uncomment)
pub fn toggle_any_entry(domain: &str, enabled: bool) -> Result<()> {
    if !is_root() {
        return toggle_any_entry_with_sudo(domain, enabled);
    }

    let current = read_hosts()?;
    let new_content = toggle_any_domain_in_content(&current, domain, enabled)?;
    write_hosts(&new_content)?;

    Ok(())
}

/// Delete any entry from /etc/hosts completely
pub fn delete_any_entry(domain: &str) -> Result<()> {
    if !is_root() {
        return delete_any_entry_with_sudo(domain);
    }

    let current = read_hosts()?;
    let new_content = remove_any_domain_from_content(&current, domain)?;
    write_hosts(&new_content)?;

    Ok(())
}

fn read_hosts() -> Result<String> {
    fs::read_to_string(HOSTS_FILE)
        .context("Failed to read /etc/hosts")
}

fn write_hosts(content: &str) -> Result<()> {
    // Backup first
    backup_hosts()?;

    fs::write(HOSTS_FILE, content)
        .context("Failed to write /etc/hosts")
}

fn backup_hosts() -> Result<()> {
    let backup_dir = get_backup_dir()?;
    let backup_path = format!("{}/hosts.backup", backup_dir);
    fs::copy(HOSTS_FILE, backup_path)
        .context("Failed to backup hosts file")?;

    Ok(())
}

fn add_domain_to_content(content: &str, domain: &str) -> Result<String> {
    // Check if our section exists
    if content.contains(MARKER_START) {
        // Update existing section
        let lines: Vec<&str> = content.lines().collect();
        let mut result = String::new();
        let mut in_section = false;
        let mut section_lines = Vec::new();

        for line in lines {
            if line.trim() == MARKER_START {
                in_section = true;
                result.push_str(line);
                result.push('\n');
            } else if line.trim() == MARKER_END {
                in_section = false;

                // Add new domain if not exists
                let domain_entry = format!("127.0.0.1 {}", domain);
                if !section_lines.contains(&domain_entry) {
                    section_lines.push(domain_entry);
                }

                // Write all section lines
                for entry in &section_lines {
                    result.push_str(entry);
                    result.push('\n');
                }
                section_lines.clear();

                result.push_str(line);
                result.push('\n');
            } else if in_section {
                section_lines.push(line.to_string());
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result)
    } else {
        // Create new section
        let mut result = content.to_string();
        if !result.ends_with('\n') {
            result.push('\n');
        }

        result.push_str(&format!(
            "\n{}\n127.0.0.1 {}\n{}\n",
            MARKER_START, domain, MARKER_END
        ));

        Ok(result)
    }
}

fn remove_domain_from_content(content: &str, domain: &str) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    let mut in_section = false;

    for line in lines {
        if line.trim() == MARKER_START {
            in_section = true;
            result.push_str(line);
            result.push('\n');
        } else if line.trim() == MARKER_END {
            in_section = false;
            result.push_str(line);
            result.push('\n');
        } else if in_section {
            // Skip the line if it contains the domain
            if !line.contains(domain) {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

fn is_root() -> bool {
    #[cfg(target_os = "windows")]
    {
        is_elevated_windows()
    }
    #[cfg(not(target_os = "windows"))]
    {
        unsafe { libc::geteuid() == 0 }
    }
}

#[cfg(target_os = "windows")]
fn is_elevated_windows() -> bool {
    use std::mem;
    use std::ptr;
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token_handle = ptr::null_mut();
        let current_process = GetCurrentProcess();

        if OpenProcessToken(current_process, TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation: TOKEN_ELEVATION = mem::zeroed();
        let mut size: u32 = mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut std::ffi::c_void,
            size,
            &mut size,
        );

        CloseHandle(token_handle);

        result != 0 && elevation.TokenIsElevated != 0
    }
}

/// Get cross-platform backup directory
fn get_backup_dir() -> Result<String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
    let backup_dir = config_dir.join("domain-router");
    fs::create_dir_all(&backup_dir)?;
    Ok(backup_dir.to_string_lossy().to_string())
}

/// Copy file with elevated privileges (cross-platform)
fn copy_with_elevation(source: &str, dest: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        copy_with_elevation_windows(source, dest)
    }
    #[cfg(not(target_os = "windows"))]
    {
        copy_with_elevation_unix(source, dest)
    }
}

#[cfg(not(target_os = "windows"))]
fn copy_with_elevation_unix(source: &str, dest: &str) -> Result<()> {
    let status = Command::new("pkexec")
        .arg("cp")
        .arg(source)
        .arg(dest)
        .status()
        .context("Failed to execute pkexec")?;

    if !status.success() {
        anyhow::bail!("Failed to copy file with elevated privileges");
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn copy_with_elevation_windows(source: &str, dest: &str) -> Result<()> {
    // On Windows, use PowerShell with Start-Process -Verb RunAs to trigger UAC
    let script = format!(
        "Copy-Item -Path '{}' -Destination '{}' -Force",
        source.replace("'", "''"),
        dest.replace("'", "''")
    );

    let status = Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Start-Process powershell -Verb RunAs -Wait -ArgumentList '-Command', '{}'",
                script.replace("'", "''")
            ),
        ])
        .status()
        .context("Failed to execute PowerShell with elevation")?;

    if !status.success() {
        anyhow::bail!("Failed to copy file with elevated privileges");
    }
    Ok(())
}

fn add_entry_with_sudo(domain: &str) -> Result<()> {
    let backup_dir = get_backup_dir()?;

    // Read current hosts
    let current = fs::read_to_string(HOSTS_FILE)
        .context("Failed to read hosts file")?;

    // Generate new content
    let new_content = add_domain_to_content(&current, domain)?;

    // Write to temp file
    let temp_file = format!("{}/hosts.tmp", backup_dir);
    fs::write(&temp_file, &new_content)?;

    // Copy with elevated privileges
    copy_with_elevation(&temp_file, HOSTS_FILE)?;

    // Cleanup
    let _ = fs::remove_file(&temp_file);

    Ok(())
}

fn remove_entry_with_sudo(domain: &str) -> Result<()> {
    let backup_dir = get_backup_dir()?;

    // Read current hosts
    let current = fs::read_to_string(HOSTS_FILE)
        .context("Failed to read hosts file")?;

    // Generate new content
    let new_content = remove_domain_from_content(&current, domain)?;

    // Write to temp file
    let temp_file = format!("{}/hosts.tmp", backup_dir);
    fs::write(&temp_file, &new_content)?;

    // Copy with elevated privileges
    copy_with_elevation(&temp_file, HOSTS_FILE)?;

    // Cleanup
    let _ = fs::remove_file(&temp_file);

    Ok(())
}

fn toggle_domain_in_content(content: &str, domain: &str, enabled: bool) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    let mut in_section = false;

    for line in lines {
        if line.trim() == MARKER_START {
            in_section = true;
            result.push_str(line);
            result.push('\n');
        } else if line.trim() == MARKER_END {
            in_section = false;
            result.push_str(line);
            result.push('\n');
        } else if in_section {
            // Check if this line contains the domain
            if line.contains(domain) {
                if enabled {
                    // Uncomment: remove leading # and whitespace
                    let uncommented = line.trim_start_matches('#').trim_start();
                    result.push_str(uncommented);
                } else {
                    // Comment: add # prefix if not already commented
                    if !line.trim_start().starts_with('#') {
                        result.push_str("# ");
                        result.push_str(line.trim_start());
                    } else {
                        result.push_str(line);
                    }
                }
            } else {
                result.push_str(line);
            }
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

fn parse_domain_entries(content: &str) -> Result<Vec<(String, bool)>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut entries = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // Skip empty lines and marker lines
        if trimmed.is_empty() || trimmed == MARKER_START || trimmed == MARKER_END {
            continue;
        }

        let is_enabled = !trimmed.starts_with('#');
        let domain_line = if trimmed.starts_with('#') {
            trimmed.trim_start_matches('#').trim()
        } else {
            trimmed
        };

        // Parse "127.0.0.1 domain.com" or "::1 localhost"
        // Split by whitespace and get IP and domain
        let parts: Vec<&str> = domain_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let ip = parts[0];
            let domain = parts[1];

            // Skip localhost entries for cleaner display (optional)
            // if domain == "localhost" || domain == "ip6-localhost" || domain == "ip6-loopback" {
            //     continue;
            // }

            entries.push((format!("{} {}", ip, domain), is_enabled));
        }
    }

    Ok(entries)
}

fn toggle_entry_with_sudo(domain: &str, enabled: bool) -> Result<()> {
    let backup_dir = get_backup_dir()?;

    // Read current hosts
    let current = fs::read_to_string(HOSTS_FILE)
        .context("Failed to read hosts file")?;

    // Generate new content
    let new_content = toggle_domain_in_content(&current, domain, enabled)?;

    // Write to temp file
    let temp_file = format!("{}/hosts.tmp", backup_dir);
    fs::write(&temp_file, &new_content)?;

    // Copy with elevated privileges
    copy_with_elevation(&temp_file, HOSTS_FILE)?;

    // Cleanup
    let _ = fs::remove_file(&temp_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_domain_to_content() {
        let content = "127.0.0.1 localhost\n";
        let result = add_domain_to_content(content, "example.com").unwrap();

        assert!(result.contains(MARKER_START));
        assert!(result.contains(MARKER_END));
        assert!(result.contains("127.0.0.1 example.com"));
    }

    #[test]
    fn test_remove_domain_from_content() {
        let content = format!(
            "127.0.0.1 localhost\n{}\n127.0.0.1 example.com\n{}\n",
            MARKER_START, MARKER_END
        );

        let result = remove_domain_from_content(&content, "example.com").unwrap();
        assert!(!result.contains("example.com"));
    }
}

// Helper functions for toggling and deleting any entry (not just in Domain Router section)

fn toggle_any_domain_in_content(content: &str, domain: &str, enabled: bool) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();

    for line in lines {
        if line.contains(domain) {
            if enabled {
                // Uncomment: remove leading # and whitespace
                let uncommented = line.trim_start_matches('#').trim_start();
                result.push_str(uncommented);
            } else {
                // Comment: add # prefix if not already commented
                if !line.trim_start().starts_with('#') {
                    result.push_str("# ");
                    result.push_str(line.trim_start());
                } else {
                    result.push_str(line);
                }
            }
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    Ok(result)
}

fn remove_any_domain_from_content(content: &str, domain: &str) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();

    for line in lines {
        // Skip lines that contain the domain
        if !line.contains(domain) {
            result.push_str(line);
            result.push('\n');
        }
    }

    Ok(result)
}

fn toggle_any_entry_with_sudo(domain: &str, enabled: bool) -> Result<()> {
    let backup_dir = get_backup_dir()?;

    let current = fs::read_to_string(HOSTS_FILE)
        .context("Failed to read hosts file")?;

    let new_content = toggle_any_domain_in_content(&current, domain, enabled)?;

    let temp_file = format!("{}/hosts.tmp", backup_dir);
    fs::write(&temp_file, &new_content)?;

    // Copy with elevated privileges
    copy_with_elevation(&temp_file, HOSTS_FILE)?;

    let _ = fs::remove_file(&temp_file);

    Ok(())
}

fn delete_any_entry_with_sudo(domain: &str) -> Result<()> {
    let backup_dir = get_backup_dir()?;

    let current = fs::read_to_string(HOSTS_FILE)
        .context("Failed to read hosts file")?;

    let new_content = remove_any_domain_from_content(&current, domain)?;

    let temp_file = format!("{}/hosts.tmp", backup_dir);
    fs::write(&temp_file, &new_content)?;

    // Copy with elevated privileges
    copy_with_elevation(&temp_file, HOSTS_FILE)?;

    let _ = fs::remove_file(&temp_file);

    Ok(())
}
