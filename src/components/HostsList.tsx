import { useState } from 'react';
import { Globe, CheckCircle, XCircle, Filter, Power, PowerOff, Trash2 } from 'lucide-react';

interface HostsListProps {
  entries: [string, boolean][];
  onToggle: (domain: string, enabled: boolean) => void;
  onDelete: (domain: string) => void;
}

type IPFilter = 'all' | 'ipv4' | 'ipv6';

function HostsList({ entries, onToggle, onDelete }: HostsListProps) {
  const [ipFilter, setIpFilter] = useState<IPFilter>('ipv4');

  // Helper function to check if IP is IPv4 or IPv6
  const isIPv4 = (ip: string): boolean => {
    return /^(\d{1,3}\.){3}\d{1,3}$/.test(ip);
  };

  const isIPv6 = (ip: string): boolean => {
    return ip.includes(':');
  };

  // Filter entries based on IP version
  const filteredEntries = entries.filter(([entry]) => {
    const ip = entry.split(' ')[0];

    if (ipFilter === 'all') return true;
    if (ipFilter === 'ipv4') return isIPv4(ip);
    if (ipFilter === 'ipv6') return isIPv6(ip);

    return true;
  });

  if (entries.length === 0) {
    return (
      <div className="empty-state">
        <Globe size={64} opacity={0.3} />
        <h3>No /etc/hosts entries</h3>
        <p>Domain entries will appear here when you add domain routes</p>
      </div>
    );
  }

  return (
    <div className="hosts-list">
      <div className="hosts-list-header">
        <h3 className="hosts-list-title">
          <Globe size={18} />
          <span>/etc/hosts Entries ({filteredEntries.length}/{entries.length})</span>
        </h3>
        <div className="hosts-filter">
          <Filter size={16} />
          <div className="filter-buttons">
            <button
              className={`filter-btn ${ipFilter === 'ipv4' ? 'active' : ''}`}
              onClick={() => setIpFilter('ipv4')}
            >
              IPv4
            </button>
            <button
              className={`filter-btn ${ipFilter === 'ipv6' ? 'active' : ''}`}
              onClick={() => setIpFilter('ipv6')}
            >
              IPv6
            </button>
            <button
              className={`filter-btn ${ipFilter === 'all' ? 'active' : ''}`}
              onClick={() => setIpFilter('all')}
            >
              ALL
            </button>
          </div>
        </div>
      </div>
      <div className="hosts-table-wrapper">
        <table className="hosts-table">
          <thead>
            <tr>
              <th>Entry</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredEntries.map(([entry, enabled], index) => {
              // Parse "IP domain" format
              const parts = entry.split(' ');
              const ip = parts[0] || '';
              const domain = parts.slice(1).join(' ') || entry;

              return (
                <tr key={`${entry}-${index}`} className={enabled ? '' : 'disabled'}>
                  <td>
                    <div className="hosts-domain">
                      <span className="hosts-ip">{ip}</span>
                      <span className="hosts-arrow">→</span>
                      <span className="hosts-name">{domain}</span>
                    </div>
                  </td>
                  <td>
                    <div className="hosts-status">
                      {enabled ? (
                        <span className="badge badge-success">
                          <CheckCircle size={14} />
                          <span>Active</span>
                        </span>
                      ) : (
                        <span className="badge badge-secondary">
                          <XCircle size={14} />
                          <span>Commented</span>
                        </span>
                      )}
                    </div>
                  </td>
                  <td>
                    <div className="hosts-actions">
                      {enabled ? (
                        <button
                          onClick={() => onToggle(domain, false)}
                          className="btn-icon btn-warning"
                          title="Inactive (Comment)"
                        >
                          <PowerOff size={16} />
                        </button>
                      ) : (
                        <button
                          onClick={() => onToggle(domain, true)}
                          className="btn-icon btn-success"
                          title="Active (Uncomment)"
                        >
                          <Power size={16} />
                        </button>
                      )}
                      <button
                        onClick={() => {
                          if (confirm(`Delete "${domain}" from /etc/hosts?`)) {
                            onDelete(domain);
                          }
                        }}
                        className="btn-icon btn-danger"
                        title="Delete"
                      >
                        <Trash2 size={16} />
                      </button>
                    </div>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default HostsList;
