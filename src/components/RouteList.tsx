import { Globe, ArrowRight, Shield, ShieldOff, Power, PowerOff, Trash2 } from 'lucide-react';
import { Route } from '../types';

interface RouteListProps {
  routes: Route[];
  onToggle: (id: string, enabled: boolean) => void;
  onDelete: (id: string) => void;
}

function RouteList({ routes, onToggle, onDelete }: RouteListProps) {
  const getRouteName = (route: Route) => {
    if (route.type === 'domain') {
      return route.domain || 'Unknown';
    } else {
      return `localhost:${route.source_port} → :${route.target_port}`;
    }
  };

  const getRouteIcon = (route: Route) => {
    return route.type === 'domain' ? <Globe size={18} /> : <ArrowRight size={18} />;
  };

  if (routes.length === 0) {
    return (
      <div className="empty-state">
        <Globe size={64} opacity={0.3} />
        <h3>No routes configured</h3>
        <p>Click "Add Route" to create your first routing rule</p>
      </div>
    );
  }

  return (
    <div className="route-list">
      <table className="route-table">
        <thead>
          <tr>
            <th>Type</th>
            <th>Domain/Port</th>
            <th>Target</th>
            <th>SSL</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {routes.map((route) => (
            <tr key={route.id} className={route.enabled ? '' : 'disabled'}>
              <td>
                <div className="route-type">
                  {getRouteIcon(route)}
                  <span>{route.type === 'domain' ? 'Domain' : 'Port'}</span>
                </div>
              </td>
              <td>
                <div className="route-name">{getRouteName(route)}</div>
              </td>
              <td>
                <div className="route-port">{route.target_host || '127.0.0.1'}:{route.target_port}</div>
              </td>
              <td>
                <div className="route-ssl">
                  {route.ssl_enabled ? (
                    <span className="badge badge-success">
                      <Shield size={14} />
                      <span>{route.ssl_mode}</span>
                    </span>
                  ) : (
                    <span className="badge badge-secondary">
                      <ShieldOff size={14} />
                      <span>None</span>
                    </span>
                  )}
                </div>
              </td>
              <td>
                <div className="route-status">
                  {route.enabled ? (
                    <span className="badge badge-success">
                      <Power size={14} />
                      <span>ON</span>
                    </span>
                  ) : (
                    <span className="badge badge-secondary">
                      <PowerOff size={14} />
                      <span>OFF</span>
                    </span>
                  )}
                </div>
              </td>
              <td>
                <div className="route-actions">
                  <button
                    onClick={() => onToggle(route.id, !route.enabled)}
                    className={`btn-icon ${route.enabled ? 'btn-warning' : 'btn-success'}`}
                    title={route.enabled ? 'Disable' : 'Enable'}
                  >
                    {route.enabled ? <PowerOff size={16} /> : <Power size={16} />}
                  </button>
                  <button
                    onClick={() => onDelete(route.id)}
                    className="btn-icon btn-danger"
                    title="Delete"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default RouteList;
