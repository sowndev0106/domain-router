use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::sync::RwLock;
use parking_lot::Mutex;
use log::{info, error};
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;

use crate::routes::{Route, RouteType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStatus {
    pub running: bool,
    pub http_port: u16,
    pub https_port: u16,
    pub active_routes: usize,
}

type RouteMap = Arc<RwLock<HashMap<String, RouteInfo>>>;

#[derive(Clone, Debug)]
struct RouteInfo {
    pub target_host: String,
    pub target_port: u16,
    pub ssl_enabled: bool,
}

lazy_static::lazy_static! {
    static ref PROXY_HANDLE: Mutex<Option<ProxyHandle>> = Mutex::new(None);
}

struct ProxyHandle {
    route_map: RouteMap,
    shutdown_tx: tokio::sync::broadcast::Sender<()>,
    server_tasks: Vec<JoinHandle<()>>,
}

pub async fn start_proxy(routes: Vec<Route>) -> Result<()> {
    info!("Starting built-in reverse proxy...");

    // Stop any existing proxy first
    stop_proxy().await?;

    // Collect ports that need to be bound
    let mut required_ports = Vec::new();
    for route in routes.iter().filter(|r| r.enabled) {
        if let RouteType::PortMapping { source_port, .. } = &route.route_type {
            required_ports.push(*source_port);
            // If SSL enabled and source is 80, we'll also need 443
            if route.ssl_enabled && *source_port == 80 {
                required_ports.push(443);
            }
        }
    }

    // Check and request capability if needed for privileged ports
    // This will show a GUI dialog if we need elevated permissions
    if let Err(e) = crate::privilege::ensure_capability_for_ports(&required_ports) {
        error!("Failed to obtain necessary permissions: {}", e);
        return Err(e);
    }

    // Build route map
    let route_map = build_route_map(&routes);

    // Create shutdown channel
    let (shutdown_tx, _shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    // Collect unique ports for port mappings
    let mut port_mappings: HashMap<u16, (String, u16)> = HashMap::new();
    for route in routes.iter().filter(|r| r.enabled) {
        if let RouteType::PortMapping { source_port, target_host, target_port } = &route.route_type {
            port_mappings.insert(*source_port, (target_host.clone(), *target_port));

            // If SSL is enabled and source port is 80, also add 443 mapping
            if route.ssl_enabled && *source_port == 80 {
                port_mappings.insert(443, (target_host.clone(), *target_port));
                info!("SSL enabled: automatically added port 443 -> {}:{}", target_host, target_port);
            }
        }
    }

    // Start HTTP servers for each port mapping
    let mut server_tasks = Vec::new();

    for (source_port, (target_host, target_port)) in &port_mappings {
        let route_map_clone = route_map.clone();
        let shutdown_rx_clone = shutdown_tx.subscribe();
        let source_port_val = *source_port;
        let target_host_val = target_host.clone();
        let target_port_val = *target_port;

        info!("Started proxy server on port {} -> {}:{}", source_port, target_host, target_port);

        // Use HTTPS server for port 443
        let task = if source_port_val == 443 {
            tokio::spawn(async move {
                if let Err(e) = start_https_server(target_host_val, target_port_val, shutdown_rx_clone).await {
                    error!("Port 443 (HTTPS) server error: {}", e);
                }
            })
        } else {
            tokio::spawn(async move {
                if let Err(e) = start_port_server(source_port_val, target_host_val, target_port_val, route_map_clone, shutdown_rx_clone).await {
                    error!("Port {} server error: {}", source_port_val, e);
                }
            })
        };

        server_tasks.push(task);
    }

    // Store handle
    let handle = ProxyHandle {
        route_map: route_map.clone(),
        shutdown_tx,
        server_tasks,
    };
    *PROXY_HANDLE.lock() = Some(handle);

    info!("Proxy started with {} port mappings", port_mappings.len());
    Ok(())
}

pub async fn stop_proxy() -> Result<()> {
    info!("Stopping proxy...");

    let handle = {
        let mut guard = PROXY_HANDLE.lock();
        guard.take()
    };

    if let Some(handle) = handle {
        // Send shutdown signal
        let _ = handle.shutdown_tx.send(());

        // Wait for all tasks to complete
        for task in handle.server_tasks {
            let _ = task.await;
        }
        info!("All proxy servers stopped");
    }

    Ok(())
}

pub async fn update_routes(routes: Vec<Route>) -> Result<()> {
    let route_map = build_route_map(&routes);

    // Clone the route_map Arc before releasing the lock
    let route_map_clone = {
        let handle_guard = PROXY_HANDLE.lock();
        if let Some(handle) = handle_guard.as_ref() {
            Some(handle.route_map.clone())
        } else {
            None
        }
    }; // Lock released here

    if let Some(target_map) = route_map_clone {
        let mut map = target_map.write().await;
        *map = {
            let new_map = route_map.read().await;
            new_map.clone()
        };
        info!("Routes updated: {} active routes", map.len());
    }

    Ok(())
}

pub fn get_status() -> ProxyStatus {
    let handle = PROXY_HANDLE.lock();

    let (running, active_routes) = if handle.is_some() {
        (true, 0) // Placeholder count
    } else {
        (false, 0)
    };

    ProxyStatus {
        running,
        http_port: 80,
        https_port: 443,
        active_routes,
    }
}

async fn start_port_server(
    source_port: u16,
    target_host: String,
    target_port: u16,
    _route_map: RouteMap,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], source_port).into();
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to port {}: {}", source_port, e))?;

    info!("Listening on {}", addr);

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("Shutting down server on port {}", source_port);
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, client_addr)) => {
                        let target_host = target_host.clone();
                        let target_port = target_port;

                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(stream, client_addr, target_host, target_port).await {
                                error!("Connection error from {}: {}", client_addr, e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection on port {}: {}", source_port, e);
                    }
                }
            }
        }
    }

    Ok(())
}

async fn handle_connection(
    mut client_stream: tokio::net::TcpStream,
    client_addr: SocketAddr,
    target_host: String,
    target_port: u16,
) -> Result<()> {
    info!("Proxying connection from {} to {}:{}", client_addr, target_host, target_port);

    // Connect to target
    let target_addr = format!("{}:{}", target_host, target_port);
    let mut target_stream = match tokio::net::TcpStream::connect(&target_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to connect to {}: {}", target_addr, e);
            return Err(anyhow::anyhow!("Failed to connect to target: {}", e));
        }
    };

    // Bi-directional proxy
    match tokio::io::copy_bidirectional(&mut client_stream, &mut target_stream).await {
        Ok((from_client, from_server)) => {
            info!("Connection closed: {} bytes from client, {} bytes from server", from_client, from_server);
        }
        Err(e) => {
            error!("Proxy error: {}", e);
        }
    }

    Ok(())
}

async fn start_https_server(
    target_host: String,
    target_port: u16,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> Result<()> {
    info!("Starting HTTPS server on port 443");

    // Load or generate TLS config using "localhost" as domain
    let tls_config = crate::ssl::load_or_generate_tls_config("localhost")
        .map_err(|e| anyhow::anyhow!("Failed to load TLS config: {}", e))?;

    let tls_acceptor = TlsAcceptor::from(tls_config);

    let addr: SocketAddr = ([0, 0, 0, 0], 443).into();
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to port 443: {}", e))?;

    info!("HTTPS server listening on {}", addr);

    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("Shutting down HTTPS server on port 443");
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, client_addr)) => {
                        let tls_acceptor = tls_acceptor.clone();
                        let target_host = target_host.clone();
                        let target_port = target_port;

                        tokio::spawn(async move {
                            // Perform TLS handshake
                            match tls_acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    info!("TLS handshake successful from {}", client_addr);
                                    if let Err(e) = handle_https_connection(tls_stream, client_addr, target_host, target_port).await {
                                        error!("HTTPS connection error from {}: {}", client_addr, e);
                                    }
                                }
                                Err(e) => {
                                    error!("TLS handshake failed from {}: {}", client_addr, e);
                                }
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection on port 443: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}

async fn handle_https_connection(
    mut tls_stream: tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
    client_addr: SocketAddr,
    target_host: String,
    target_port: u16,
) -> Result<()> {
    info!("Proxying HTTPS connection from {} to {}:{}", client_addr, target_host, target_port);

    // Connect to target (plain HTTP)
    let target_addr = format!("{}:{}", target_host, target_port);
    let mut target_stream = match tokio::net::TcpStream::connect(&target_addr).await {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to connect to {}: {}", target_addr, e);
            return Err(anyhow::anyhow!("Failed to connect to target: {}", e));
        }
    };

    // Bi-directional proxy: TLS client <-> plain HTTP backend
    match tokio::io::copy_bidirectional(&mut tls_stream, &mut target_stream).await {
        Ok((from_client, from_server)) => {
            info!("HTTPS connection closed: {} bytes from client, {} bytes from server", from_client, from_server);
        }
        Err(e) => {
            error!("HTTPS proxy error: {}", e);
        }
    }

    Ok(())
}

fn build_route_map(routes: &[Route]) -> RouteMap {
    let mut map = HashMap::new();

    for route in routes.iter().filter(|r| r.enabled) {
        match &route.route_type {
            RouteType::Domain { domain, target_host, target_port } => {
                info!("Configuring route: {} -> {}:{}", domain, target_host, target_port);
                map.insert(
                    domain.clone(),
                    RouteInfo {
                        target_host: target_host.clone(),
                        target_port: *target_port,
                        ssl_enabled: route.ssl_enabled,
                    },
                );
            }
            RouteType::PortMapping { source_port, target_host, target_port } => {
                info!("Configuring port mapping: localhost:{} -> {}:{}", source_port, target_host, target_port);
                // For port mapping, use localhost as key
                map.insert(
                    format!("localhost:{}", source_port),
                    RouteInfo {
                        target_host: target_host.clone(),
                        target_port: *target_port,
                        ssl_enabled: route.ssl_enabled,
                    },
                );
            }
        }
    }

    Arc::new(RwLock::new(map))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::Route;

    #[test]
    fn test_build_route_map() {
        let routes = vec![
            Route::new_domain("example.com".to_string(), 8080, false),
        ];

        let _map = build_route_map(&routes);
        // Map created successfully
    }
}
