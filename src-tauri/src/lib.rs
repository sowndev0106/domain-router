// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod routes;
mod hosts;
mod proxy;
mod ssl;
mod acme;
mod utils;
mod privilege;

use anyhow::Result;
use log::info;
use std::sync::Mutex;
use tauri::State;

/// Application state shared across all handlers
pub struct AppState {
    config: Mutex<routes::Config>,
}

impl AppState {
    fn new() -> Self {
        Self {
            config: Mutex::new(routes::Config::load().unwrap_or_default()),
        }
    }
}

#[tauri::command]
async fn get_routes(state: State<'_, AppState>) -> Result<Vec<routes::Route>, String> {
    let config = state.config.lock().unwrap();
    Ok(config.routes.clone())
}

#[tauri::command]
async fn add_route(
    route: routes::RouteInput,
    state: State<'_, AppState>,
) -> Result<routes::Route, String> {
    info!("Adding route: {:?}", route);

    // Convert input to route with generated ID
    let route = route.into_route();

    // Validate route
    route.validate().map_err(|e| e.to_string())?;

    // Add to hosts file if domain route
    if let routes::RouteType::Domain { domain, .. } = &route.route_type {
        hosts::add_entry(domain).map_err(|e| e.to_string())?;
    }

    // Update config and get routes snapshot
    let routes_snapshot = {
        let mut config = state.config.lock().unwrap();
        config.routes.push(route.clone());
        config.save().map_err(|e| e.to_string())?;
        config.routes.clone()
    };

    // Update proxy routes (lock released)
    proxy::update_routes(routes_snapshot).await.map_err(|e| e.to_string())?;

    Ok(route)
}

#[tauri::command]
async fn update_route(
    route: routes::Route,
    state: State<'_, AppState>,
) -> Result<routes::Route, String> {
    info!("Updating route: {:?}", route);

    // Validate route
    route.validate().map_err(|e| e.to_string())?;

    let routes_snapshot = {
        let mut config = state.config.lock().unwrap();

        // Find and update route
        if let Some(existing) = config.routes.iter_mut().find(|r| r.id == route.id) {
            *existing = route.clone();
        } else {
            return Err("Route not found".to_string());
        }

        config.save().map_err(|e| e.to_string())?;
        config.routes.clone()
    };

    // Update proxy routes
    proxy::update_routes(routes_snapshot).await.map_err(|e| e.to_string())?;

    Ok(route)
}

#[tauri::command]
async fn remove_route(id: String, state: State<'_, AppState>) -> Result<(), String> {
    info!("Removing route: {}", id);

    let routes_snapshot = {
        let mut config = state.config.lock().unwrap();

        // Find and remove route
        if let Some(pos) = config.routes.iter().position(|r| r.id == id) {
            let route = config.routes.remove(pos);

            // Remove from hosts file
            if let routes::RouteType::Domain { domain, .. } = &route.route_type {
                hosts::remove_entry(domain).map_err(|e| e.to_string())?;
            }
        }

        config.save().map_err(|e| e.to_string())?;
        config.routes.clone()
    };

    // Update proxy routes
    proxy::update_routes(routes_snapshot).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn toggle_route(id: String, enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    info!("Toggling route {}: {}", id, enabled);

    let (routes_snapshot, domain_to_toggle) = {
        let mut config = state.config.lock().unwrap();

        let domain_to_toggle = if let Some(route) = config.routes.iter_mut().find(|r| r.id == id) {
            route.enabled = enabled;

            // Get domain if it's a domain route
            if let routes::RouteType::Domain { domain, .. } = &route.route_type {
                Some(domain.clone())
            } else {
                None
            }
        } else {
            None
        };

        config.save().map_err(|e| e.to_string())?;
        (config.routes.clone(), domain_to_toggle)
    };

    // Toggle hosts entry (comment/uncomment) if it's a domain route
    if let Some(domain) = domain_to_toggle {
        hosts::toggle_entry(&domain, enabled).map_err(|e| e.to_string())?;
    }

    // Update proxy routes
    proxy::update_routes(routes_snapshot).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_proxy_status() -> Result<proxy::ProxyStatus, String> {
    Ok(proxy::get_status())
}

#[tauri::command]
async fn start_proxy(state: State<'_, AppState>) -> Result<(), String> {
    let routes_snapshot = {
        let config = state.config.lock().unwrap();
        config.routes.clone()
    };

    // Start built-in proxy
    proxy::start_proxy(routes_snapshot).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_proxy() -> Result<(), String> {
    proxy::stop_proxy().await.map_err(|e| e.to_string())
}

#[tauri::command]
fn check_port_available(port: u16) -> Result<bool, String> {
    utils::check_port(port).map_err(|e| e.to_string())
}

#[tauri::command]
fn generate_ssl_cert(domain: String) -> Result<(String, String), String> {
    ssl::generate_and_save(&domain)
        .map(|(cert, key)| (cert.to_string_lossy().to_string(), key.to_string_lossy().to_string()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_hosts_entries() -> Result<Vec<(String, bool)>, String> {
    hosts::get_all_entries().map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_hosts_entry(domain: String, enabled: bool) -> Result<(), String> {
    hosts::toggle_any_entry(&domain, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_hosts_entry(domain: String) -> Result<(), String> {
    hosts::delete_any_entry(&domain).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_routes,
            add_route,
            update_route,
            remove_route,
            toggle_route,
            get_proxy_status,
            start_proxy,
            stop_proxy,
            check_port_available,
            generate_ssl_cert,
            get_hosts_entries,
            toggle_hosts_entry,
            delete_hosts_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
