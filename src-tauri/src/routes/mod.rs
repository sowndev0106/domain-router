use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub traefik: TraefikConfig,
    pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraefikConfig {
    pub binary_path: String,
    pub config_dir: PathBuf,
    pub log_level: String,
    pub dashboard_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    #[serde(flatten)]
    pub route_type: RouteType,
    pub ssl_enabled: bool,
    pub ssl_mode: SslMode,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

/// Input struct for creating a new route (without ID and created_at)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInput {
    #[serde(flatten)]
    pub route_type: RouteType,
    pub ssl_enabled: bool,
    #[serde(default = "default_ssl_mode")]
    pub ssl_mode: SslMode,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_ssl_mode() -> SslMode {
    SslMode::SelfSigned
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RouteType {
    Domain {
        domain: String,
        #[serde(default = "default_target_host")]
        target_host: String,
        target_port: u16,
    },
    PortMapping {
        source_port: u16,
        #[serde(default = "default_target_host")]
        target_host: String,
        target_port: u16,
    },
}

fn default_target_host() -> String {
    "127.0.0.1".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SslMode {
    #[serde(rename = "self-signed")]
    SelfSigned,
    #[serde(rename = "letsencrypt")]
    LetsEncrypt,
    Passthrough,
    Custom { cert_path: PathBuf, key_path: PathBuf },
}

impl Default for Config {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let config_dir = PathBuf::from(format!("{}/.config/domain-router", home));

        Self {
            version: "1.0.0".to_string(),
            traefik: TraefikConfig {
                binary_path: "/usr/local/bin/traefik".to_string(),
                config_dir: config_dir.join("traefik"),
                log_level: "INFO".to_string(),
                dashboard_port: 8080,
            },
            routes: vec![],
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        PathBuf::from(format!("{}/.config/domain-router/config.json", home))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)
            .context("Failed to read config file")?;

        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&path, content)
            .context("Failed to write config file")?;

        Ok(())
    }
}

impl RouteInput {
    /// Convert RouteInput to Route by generating ID and timestamp
    pub fn into_route(self) -> Route {
        Route {
            id: Uuid::new_v4().to_string(),
            route_type: self.route_type,
            ssl_enabled: self.ssl_enabled,
            ssl_mode: self.ssl_mode,
            enabled: self.enabled,
            created_at: Utc::now(),
        }
    }
}

impl Route {
    pub fn new_domain(domain: String, target_port: u16, ssl_enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            route_type: RouteType::Domain {
                domain,
                target_host: "127.0.0.1".to_string(),
                target_port,
            },
            ssl_enabled,
            ssl_mode: SslMode::SelfSigned,
            enabled: true,
            created_at: Utc::now(),
        }
    }

    pub fn new_port_mapping(source_port: u16, target_port: u16, ssl_enabled: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            route_type: RouteType::PortMapping {
                source_port,
                target_host: "127.0.0.1".to_string(),
                target_port,
            },
            ssl_enabled,
            ssl_mode: SslMode::SelfSigned,
            enabled: true,
            created_at: Utc::now(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        match &self.route_type {
            RouteType::Domain { domain, target_host, target_port } => {
                // Validate domain
                if domain.is_empty() {
                    anyhow::bail!("Domain cannot be empty");
                }

                // Validate target host
                if target_host.is_empty() {
                    anyhow::bail!("Target host cannot be empty");
                }

                // Basic domain validation
                let domain_regex = regex::Regex::new(r"^([a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}$")?;
                if !domain_regex.is_match(domain) {
                    anyhow::bail!("Invalid domain format: {}", domain);
                }

                // Validate port
                if *target_port == 0 {
                    anyhow::bail!("Port cannot be 0");
                }
            }
            RouteType::PortMapping { source_port, target_host, target_port } => {
                if *source_port == 0 || *target_port == 0 {
                    anyhow::bail!("Ports cannot be 0");
                }

                // Validate target host
                if target_host.is_empty() {
                    anyhow::bail!("Target host cannot be empty");
                }

                if source_port == target_port && target_host == "localhost" {
                    anyhow::bail!("Source and target ports cannot be the same for localhost");
                }
            }
        }

        Ok(())
    }

    pub fn get_name(&self) -> String {
        match &self.route_type {
            RouteType::Domain { domain, .. } => domain.clone(),
            RouteType::PortMapping { source_port, target_host, target_port } => {
                format!("localhost:{} → {}:{}", source_port, target_host, target_port)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_validation() {
        let route = Route::new_domain("example.com".to_string(), 80, true);
        assert!(route.validate().is_ok());

        let invalid_route = Route::new_domain("".to_string(), 80, true);
        assert!(invalid_route.validate().is_err());
    }

    #[test]
    fn test_port_mapping_validation() {
        let route = Route::new_port_mapping(4000, 80, true);
        assert!(route.validate().is_ok());

        let invalid_route = Route::new_port_mapping(0, 80, true);
        assert!(invalid_route.validate().is_err());
    }
}
