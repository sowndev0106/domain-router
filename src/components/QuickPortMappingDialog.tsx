import { useState } from 'react';
import { X, Zap } from 'lucide-react';
import { Route } from '../types';

interface QuickPortMappingDialogProps {
  onAdd: (routes: Partial<Route>[]) => void;
  onClose: () => void;
}

interface QuickFormData {
  target_host: string;
  target_port: string;
}

function QuickPortMappingDialog({ onAdd, onClose }: QuickPortMappingDialogProps) {
  const [formData, setFormData] = useState<QuickFormData>({
    target_host: '127.0.0.1',
    target_port: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const targetPort = parseInt(formData.target_port);
    const routes: Partial<Route>[] = [
      {
        type: 'portmapping',
        source_port: 80,
        target_host: formData.target_host || '127.0.0.1',
        target_port: targetPort,
        ssl_enabled: true,  // Enable SSL to also create 443 mapping
        ssl_mode: 'self-signed',
        enabled: true,
      },
    ];

    onAdd(routes);
  };

  const handleChange = (field: keyof QuickFormData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  return (
    <div className="dialog-overlay" onClick={onClose}>
      <div className="dialog" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <div className="dialog-title-with-icon">
            <Zap size={20} className="quick-icon" />
            <h2>Quick Port Mapping (80 & 443)</h2>
          </div>
          <button onClick={onClose} className="btn-icon">
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="dialog-body">
          <div className="quick-info">
            <p>This will create two port mappings:</p>
            <ul>
              <li><strong>Port 80</strong> (HTTP) → Your service</li>
              <li><strong>Port 443</strong> (HTTPS) → Your service</li>
            </ul>
          </div>

          <div className="form-group">
            <label htmlFor="target_host">Target Host:</label>
            <input
              id="target_host"
              type="text"
              placeholder="127.0.0.1"
              value={formData.target_host}
              onChange={(e) => handleChange('target_host', e.target.value)}
              required
            />
            <small>The target host where your service is running</small>
          </div>

          <div className="form-group">
            <label htmlFor="target_port">Target Port:</label>
            <input
              id="target_port"
              type="number"
              placeholder="4000"
              min="1"
              max="65535"
              value={formData.target_port}
              onChange={(e) => handleChange('target_port', e.target.value)}
              required
            />
            <small>The port where your service is listening (e.g., 4000, 3000, 8080)</small>
          </div>

          <div className="quick-preview">
            <strong>Result:</strong>
            <div className="preview-items">
              <div className="preview-item">
                <span className="preview-label">HTTP:</span>
                <span className="preview-value">
                  localhost:80 → {formData.target_host || '127.0.0.1'}:{formData.target_port || 'XXXX'}
                </span>
              </div>
              <div className="preview-item">
                <span className="preview-label">HTTPS:</span>
                <span className="preview-value">
                  localhost:443 → {formData.target_host || '127.0.0.1'}:{formData.target_port || 'XXXX'}
                </span>
              </div>
            </div>
          </div>

          <div className="dialog-footer">
            <button type="button" onClick={onClose} className="btn btn-secondary">
              Cancel
            </button>
            <button type="submit" className="btn btn-primary">
              <Zap size={16} />
              <span>Create Quick Mapping</span>
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default QuickPortMappingDialog;
