use anyhow::{Context, Result};
use rcgen::{CertificateParams, KeyPair, DistinguishedName, DnType};
use rustls::pki_types::CertificateDer;
use rustls::ServerConfig;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use log::info;

pub fn generate_self_signed_cert(domain: &str) -> Result<(String, String)> {
    // Create certificate params with SANs (pass strings directly)
    let mut params = CertificateParams::new(vec![
        domain.to_string(),
        format!("*.{}", domain),
        "localhost".to_string(),
    ]).unwrap();

    // Set certificate details
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, domain);
    dn.push(DnType::OrganizationName, "Domain Router");
    params.distinguished_name = dn;

    // Generate key pair
    let key_pair = KeyPair::generate().unwrap();

    // Generate certificate
    let cert = params.self_signed(&key_pair)
        .context("Failed to generate certificate")?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    Ok((cert_pem, key_pem))
}

pub fn save_cert(domain: &str, cert_pem: &str, key_pem: &str) -> Result<(PathBuf, PathBuf)> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let cert_dir = PathBuf::from(format!("{}/.config/domain-router/certs", home));
    fs::create_dir_all(&cert_dir)?;

    let cert_path = cert_dir.join(format!("{}.crt", domain));
    let key_path = cert_dir.join(format!("{}.key", domain));

    fs::write(&cert_path, cert_pem)
        .context("Failed to write certificate")?;

    fs::write(&key_path, key_pem)
        .context("Failed to write private key")?;

    Ok((cert_path, key_path))
}

pub fn generate_and_save(domain: &str) -> Result<(PathBuf, PathBuf)> {
    let (cert_pem, key_pem) = generate_self_signed_cert(domain)?;
    save_cert(domain, &cert_pem, &key_pem)
}

/// Load certificate and key from disk and create a rustls ServerConfig
pub fn load_or_generate_tls_config(domain: &str) -> Result<Arc<ServerConfig>> {
    info!("Loading or generating TLS config for domain: {}", domain);

    // Install default crypto provider (ring)
    let _ = rustls::crypto::ring::default_provider().install_default();

    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let cert_dir = PathBuf::from(format!("{}/.config/domain-router/certs", home));
    let cert_path = cert_dir.join(format!("{}.crt", domain));
    let key_path = cert_dir.join(format!("{}.key", domain));

    // Generate if doesn't exist
    if !cert_path.exists() || !key_path.exists() {
        info!("Certificates not found, generating new ones");
        generate_and_save(domain)?;
    }

    // Load certificate and key
    let cert_pem = fs::read(&cert_path)
        .context("Failed to read certificate file")?;
    let key_pem = fs::read(&key_path)
        .context("Failed to read private key file")?;

    // Parse certificates
    let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut cert_pem.as_slice())
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse certificates")?;

    // Parse private key
    let key = rustls_pemfile::private_key(&mut key_pem.as_slice())
        .context("Failed to parse private key")?
        .ok_or_else(|| anyhow::anyhow!("No private key found"))?;

    // Create ServerConfig
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .context("Failed to create TLS config")?;

    info!("TLS config loaded successfully");
    Ok(Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_self_signed_cert() {
        let result = generate_self_signed_cert("example.com");
        assert!(result.is_ok());

        let (cert, key) = result.unwrap();
        assert!(cert.contains("BEGIN CERTIFICATE"));
        assert!(key.contains("BEGIN PRIVATE KEY"));
    }
}
