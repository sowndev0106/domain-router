import { useState, useEffect } from 'react';
import { Plus, RefreshCw, Settings, Play, Square, Activity } from 'lucide-react';
import { api } from './api';
import { Route, ProxyStatus } from './types';
import RouteList from './components/RouteList';
import HostsList from './components/HostsList';
import AddDomainRouteDialog from './components/AddDomainRouteDialog';
import AddPortMappingDialog from './components/AddPortMappingDialog';
import './App.css';

type TabType = 'domain' | 'port';

function App() {
  const [routes, setRoutes] = useState<Route[]>([]);
  const [proxyStatus, setTraefikStatus] = useState<ProxyStatus>({ running: false, http_port: 80, https_port: 443, active_routes: 0 });
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<TabType>('domain');
  const [hostsEntries, setHostsEntries] = useState<[string, boolean][]>([]);

  useEffect(() => {
    loadData();
    // Poll Proxy status and hosts every 3 seconds
    const interval = setInterval(() => {
      loadProxyStatus();
      loadHostsEntries();
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      await Promise.all([loadRoutes(), loadProxyStatus()]);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load data');
    } finally {
      setLoading(false);
    }
  };

  const loadRoutes = async () => {
    try {
      const data = await api.getRoutes();
      setRoutes(data);
    } catch (err) {
      console.error('Failed to load routes:', err);
    }
  };

  const loadProxyStatus = async () => {
    try {
      const status = await api.getProxyStatus();
      setTraefikStatus(status);
    } catch (err) {
      console.error('Failed to load Proxy status:', err);
    }
  };

  const loadHostsEntries = async () => {
    try {
      const entries = await api.getHostsEntries();
      setHostsEntries(entries);
    } catch (err) {
      console.error('Failed to load hosts entries:', err);
    }
  };

  const handleToggleHostsEntry = async (domain: string, enabled: boolean) => {
    try {
      setError(null);
      await api.toggleHostsEntry(domain, enabled);
      await loadHostsEntries();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to toggle hosts entry');
    }
  };

  const handleDeleteHostsEntry = async (domain: string) => {
    try {
      setError(null);
      await api.deleteHostsEntry(domain);
      await loadHostsEntries();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete hosts entry');
    }
  };

  const handleAddRoute = async (route: Partial<Route>) => {
    try {
      setError(null);
      await api.addRoute(route);
      await loadRoutes();
      setShowAddDialog(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add route');
    }
  };

  const handleToggleRoute = async (id: string, enabled: boolean) => {
    try {
      setError(null);
      await api.toggleRoute(id, enabled);
      await loadRoutes();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to toggle route');
    }
  };

  const handleDeleteRoute = async (id: string) => {
    if (!confirm('Are you sure you want to delete this route?')) return;

    try {
      setError(null);
      await api.removeRoute(id);
      await loadRoutes();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete route');
    }
  };

  const handleStartProxy = async () => {
    try {
      setError(null);
      await api.startProxy();
      await loadProxyStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to start Proxy');
    }
  };

  const handleStopProxy = async () => {
    try {
      setError(null);
      await api.stopProxy();
      await loadProxyStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to stop Proxy');
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-content">
          <h1>Domain Router</h1>
          <div className="header-actions">
            <button className="btn btn-secondary">
              <Settings size={18} />
            </button>
          </div>
        </div>
      </header>

      <main className="app-main">
        {error && (
          <div className="error-banner">
            <span>{error}</span>
            <button onClick={() => setError(null)}>&times;</button>
          </div>
        )}

        <div className="tabs-container">
          <div className="tabs">
            <button
              className={`tab ${activeTab === 'domain' ? 'active' : ''}`}
              onClick={() => setActiveTab('domain')}
            >
              Domain Routes
              <span className="tab-count">{routes.filter(r => r.type === 'domain').length}</span>
            </button>
            <button
              className={`tab ${activeTab === 'port' ? 'active' : ''}`}
              onClick={() => setActiveTab('port')}
            >
              Port Mapping
              <span className="tab-count">{routes.filter(r => r.type === 'portmapping').length}</span>
            </button>
          </div>
        </div>

        {activeTab === 'domain' && (
          <>
            <div className="tab-actions">
              <div className="hosts-sync-info">
                <span className="sync-label">/etc/hosts:</span>
                <span className="sync-count">
                  {hostsEntries.filter(([_, enabled]) => enabled).length} active
                </span>
                <span className="sync-separator">|</span>
                <span className="sync-count">
                  {hostsEntries.filter(([_, enabled]) => !enabled).length} disabled
                </span>
              </div>
              <div className="tab-buttons">
                <button onClick={() => setShowAddDialog(true)} className="btn btn-primary">
                  <Plus size={18} />
                  <span>Add Route</span>
                </button>
                <button onClick={loadData} className="btn btn-secondary" disabled={loading}>
                  <RefreshCw size={18} className={loading ? 'spinning' : ''} />
                  <span>Refresh</span>
                </button>
              </div>
            </div>
            <HostsList
              entries={hostsEntries}
              onToggle={handleToggleHostsEntry}
              onDelete={handleDeleteHostsEntry}
            />
            <RouteList
              routes={routes.filter(r => r.type === 'domain')}
              onToggle={handleToggleRoute}
              onDelete={handleDeleteRoute}
            />
          </>
        )}

        {activeTab === 'port' && (
          <>
            <div className="tab-actions">
              <div className="tab-buttons">
                <button onClick={() => setShowAddDialog(true)} className="btn btn-primary">
                  <Plus size={18} />
                  <span>Add Route</span>
                </button>
                <button onClick={loadData} className="btn btn-secondary" disabled={loading}>
                  <RefreshCw size={18} className={loading ? 'spinning' : ''} />
                  <span>Refresh</span>
                </button>
              </div>
            </div>
            <RouteList
              routes={routes.filter(r => r.type === 'portmapping')}
              onToggle={handleToggleRoute}
              onDelete={handleDeleteRoute}
            />
          </>
        )}
      </main>

      <footer className="app-footer">
        <div className="traefik-status">
          <Activity size={18} />
          <span>Proxy Status:</span>
          <span className={`status-indicator ${proxyStatus.running ? 'running' : 'stopped'}`}>
            {proxyStatus.running ? '⬤ Running' : '○ Stopped'}
          </span>
          {proxyStatus.http_port && <span className="status-detail">HTTP: {proxyStatus.http_port}</span>}
          {proxyStatus.https_port && <span className="status-detail">HTTPS: {proxyStatus.https_port}</span>}
        </div>
        <div className="traefik-actions">
          {proxyStatus.running ? (
            <button onClick={handleStopProxy} className="btn btn-danger btn-sm">
              <Square size={16} />
              <span>Stop</span>
            </button>
          ) : (
            <button onClick={handleStartProxy} className="btn btn-success btn-sm">
              <Play size={16} />
              <span>Start</span>
            </button>
          )}
        </div>
      </footer>

      {showAddDialog && activeTab === 'domain' && (
        <AddDomainRouteDialog
          onAdd={handleAddRoute}
          onClose={() => setShowAddDialog(false)}
        />
      )}

      {showAddDialog && activeTab === 'port' && (
        <AddPortMappingDialog
          onAdd={handleAddRoute}
          onClose={() => setShowAddDialog(false)}
        />
      )}
    </div>
  );
}

export default App;
