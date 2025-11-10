import { useState } from 'react';
import { X } from 'lucide-react';
import { Route, RouteFormData } from '../types';

interface AddRouteDialogProps {
  onAdd: (route: Partial<Route>) => void;
  onClose: () => void;
}

function AddRouteDialog({ onAdd, onClose }: AddRouteDialogProps) {
  const [formData, setFormData] = useState<RouteFormData>({
    type: 'domain',
    domain: '',
    source_port: '',
    target_host: '127.0.0.1',
    target_port: '',
    ssl_enabled: true,
    ssl_mode: 'self-signed',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const route: Partial<Route> = {
      type: formData.type,
      ssl_enabled: formData.ssl_enabled,
      ssl_mode: formData.ssl_mode,
      enabled: true,
      target_host: formData.target_host || '127.0.0.1',
    };

    if (formData.type === 'domain') {
      route.domain = formData.domain;
      route.target_port = parseInt(formData.target_port);
    } else {
      route.source_port = parseInt(formData.source_port);
      route.target_port = parseInt(formData.target_port);
    }

    onAdd(route);
  };

  const handleChange = (field: keyof RouteFormData, value: string | boolean) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <h2>Add New Route</h2>
          <button onClick={onClose} className="btn-icon">
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="dialog-body">
          <div className="form-group">
            <label>Type:</label>
            <div className="radio-group">
              <label className="radio-label">
                <input
                  type="radio"
                  name="type"
                  value="domain"
                  checked={formData.type === 'domain'}
                  onChange={(e) => handleChange('type', e.target.value)}
                />
                <span>Domain Route</span>
              </label>
              <label className="radio-label">
                <input
                  type="radio"
                  name="type"
                  value="portmapping"
                  checked={formData.type === 'portmapping'}
                  onChange={(e) => handleChange('type', e.target.value)}
                />
                <span>Port Mapping</span>
              </label>
            </div>
          </div>

          {formData.type === 'domain' ? (
            <>
              <div className="form-group">
                <label htmlFor="domain">Domain:</label>
                <input
                  id="domain"
                  type="text"
                  placeholder="example.com or api.local.dev"
                  value={formData.domain}
                  onChange={(e) => handleChange('domain', e.target.value)}
                  required
                />
                <small>The domain name to redirect (e.g., seller-dev.openlive.lotte.vn)</small>
              </div>

              <div className="form-group">
                <label htmlFor="target_host_domain">Target Host:</label>
                <input
                  id="target_host_domain"
                  type="text"
                  placeholder="127.0.0.1"
                  value={formData.target_host}
                  onChange={(e) => handleChange('target_host', e.target.value)}
                  required
                />
                <small>The target host (e.g., 127.0.0.1, 192.168.1.100, server.local)</small>
              </div>

              <div className="form-group">
                <label htmlFor="target_port">Target Port:</label>
                <input
                  id="target_port"
                  type="number"
                  placeholder="80"
                  min="1"
                  max="65535"
                  value={formData.target_port}
                  onChange={(e) => handleChange('target_port', e.target.value)}
                  required
                />
                <small>The target port to forward traffic to</small>
              </div>
            </>
          ) : (
            <>
              <div className="form-group">
                <label htmlFor="source_port">Source Port:</label>
                <input
                  id="source_port"
                  type="number"
                  placeholder="4000"
                  min="1"
                  max="65535"
                  value={formData.source_port}
                  onChange={(e) => handleChange('source_port', e.target.value)}
                  required
                />
                <small>The port to listen on</small>
              </div>

              <div className="form-group">
                <label htmlFor="target_host_pm">Target Host:</label>
                <input
                  id="target_host_pm"
                  type="text"
                  placeholder="127.0.0.1"
                  value={formData.target_host}
                  onChange={(e) => handleChange('target_host', e.target.value)}
                  required
                />
                <small>The target host (e.g., 127.0.0.1, 192.168.1.100, server.local)</small>
              </div>

              <div className="form-group">
                <label htmlFor="target_port_pm">Target Port:</label>
                <input
                  id="target_port_pm"
                  type="number"
                  placeholder="80"
                  min="1"
                  max="65535"
                  value={formData.target_port}
                  onChange={(e) => handleChange('target_port', e.target.value)}
                  required
                />
                <small>The port to forward to</small>
              </div>
            </>
          )}

          <div className="form-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={formData.ssl_enabled}
                onChange={(e) => handleChange('ssl_enabled', e.target.checked)}
              />
              <span>Enable HTTPS/SSL</span>
            </label>
          </div>

          {formData.ssl_enabled && (
            <div className="form-group">
              <label htmlFor="ssl_mode">SSL Mode:</label>
              <select
                id="ssl_mode"
                value={formData.ssl_mode}
                onChange={(e) => handleChange('ssl_mode', e.target.value)}
              >
                <option value="self-signed">Self-Signed Certificate (Auto)</option>
                <option value="letsencrypt">Let's Encrypt (Future)</option>
                <option value="passthrough">SSL Passthrough</option>
              </select>
              <small>How SSL certificates should be handled</small>
            </div>
          )}

          <div className="dialog-footer">
            <button type="button" onClick={onClose} className="btn btn-secondary">
              Cancel
            </button>
            <button type="submit" className="btn btn-primary">
              Add Route
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default AddRouteDialog;
