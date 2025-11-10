use anyhow::Result;
use std::net::TcpListener;

pub fn check_port(port: u16) -> Result<bool> {
    match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(_) => Ok(true),  // Port is available
        Err(_) => Ok(false), // Port is in use
    }
}

pub fn get_available_port(start: u16) -> Result<u16> {
    for port in start..65535 {
        if check_port(port)? {
            return Ok(port);
        }
    }

    anyhow::bail!("No available ports found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_port() {
        // Port 0 should always be available (let OS choose)
        let result = check_port(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_available_port() {
        let port = get_available_port(10000).unwrap();
        assert!(port >= 10000);
        assert!(port < 65535);
    }
}
