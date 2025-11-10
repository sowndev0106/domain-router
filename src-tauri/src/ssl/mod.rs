use anyhow::{Context, Result};
use rcgen::{CertificateParams, KeyPair, DistinguishedName, DnType};
use std::fs;
use std::path::PathBuf;

pub fn generate_self_signed_cert(domain: &str) -> Result<(String, String)> {
    // Create certificate params
    let mut params = CertificateParams::new(vec![domain.to_string()]).unwrap();

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
