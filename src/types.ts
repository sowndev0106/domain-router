export interface Route {
  id: string;
  type: 'domain' | 'portmapping';
  domain?: string;
  source_port?: number;
  target_host?: string;
  target_port: number;
  ssl_enabled: boolean;
  ssl_mode: 'self-signed' | 'letsencrypt' | 'passthrough' | 'custom';
  enabled: boolean;
  created_at: string;
}

export interface ProxyStatus {
  running: boolean;
  http_port: number;
  https_port: number;
  active_routes: number;
}

// Alias for backward compatibility with old UI code
export type TraefikStatus = ProxyStatus;

export interface RouteFormData {
  type: 'domain' | 'portmapping';
  domain: string;
  source_port: string;
  target_host: string;
  target_port: string;
  ssl_enabled: boolean;
  ssl_mode: 'self-signed' | 'letsencrypt' | 'passthrough' | 'custom';
}
