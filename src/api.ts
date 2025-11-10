import { invoke } from '@tauri-apps/api/core';
import { Route, ProxyStatus } from './types';

export const api = {
  // Routes
  async getRoutes(): Promise<Route[]> {
    return await invoke('get_routes');
  },

  async addRoute(route: Partial<Route>): Promise<Route> {
    return await invoke('add_route', { route });
  },

  async updateRoute(route: Route): Promise<Route> {
    return await invoke('update_route', { route });
  },

  async removeRoute(id: string): Promise<void> {
    return await invoke('remove_route', { id });
  },

  async toggleRoute(id: string, enabled: boolean): Promise<void> {
    return await invoke('toggle_route', { id, enabled });
  },

  // Built-in Proxy
  async getProxyStatus(): Promise<ProxyStatus> {
    return await invoke('get_proxy_status');
  },

  async startProxy(): Promise<void> {
    return await invoke('start_proxy');
  },

  async stopProxy(): Promise<void> {
    return await invoke('stop_proxy');
  },

  // Utilities
  async checkPortAvailable(port: number): Promise<boolean> {
    return await invoke('check_port_available', { port });
  },

  async generateSslCert(domain: string): Promise<[string, string]> {
    return await invoke('generate_ssl_cert', { domain });
  },

  // Hosts file sync
  async getHostsEntries(): Promise<[string, boolean][]> {
    return await invoke('get_hosts_entries');
  },

  async toggleHostsEntry(domain: string, enabled: boolean): Promise<void> {
    return await invoke('toggle_hosts_entry', { domain, enabled });
  },

  async deleteHostsEntry(domain: string): Promise<void> {
    return await invoke('delete_hosts_entry', { domain });
  },
};
