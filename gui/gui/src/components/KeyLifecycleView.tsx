import { Trash2, RefreshCw } from 'lucide-react';

const keys = [
  { id: 'key-1234', name: 'Master-App-Key', type: 'AES-256-GCM', phase: 'Active', created: '2024-01-15' },
  { id: 'key-5678', name: 'Legacy-API-Key', type: 'RSA-4096', phase: 'Expired', created: '2023-06-20' },
  { id: 'key-9012', name: 'Staging-DB-Key', type: 'Ed25519', phase: 'Pre-Op', created: '2024-03-01' },
  { id: 'key-3456', name: 'Compromised-JWT', type: 'ECDSA-P256', phase: 'Revoked', created: '2023-11-10' },
];

export function KeyLifecycleView() {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-2xl font-bold">Key Lifecycle Matrix</h2>
        <button className="btn btn-primary">Generate New Key</button>
      </div>

      <div className="card overflow-hidden p-0">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-slate-800/50 border-b border-slate-700">
              <th className="p-4 font-medium text-slate-300">Key ID</th>
              <th className="p-4 font-medium text-slate-300">Name</th>
              <th className="p-4 font-medium text-slate-300">Type</th>
              <th className="p-4 font-medium text-slate-300">Phase (NIST SP 800-57)</th>
              <th className="p-4 font-medium text-slate-300">Created</th>
              <th className="p-4 font-medium text-slate-300">Actions</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-700">
            {keys.map((k) => (
              <tr key={k.id} className="hover:bg-slate-800/30 transition-colors">
                <td className="p-4 font-mono text-sm text-slate-400">{k.id}</td>
                <td className="p-4 font-medium">{k.name}</td>
                <td className="p-4 text-sm">{k.type}</td>
                <td className="p-4">
                  <PhaseBadge phase={k.phase} />
                </td>
                <td className="p-4 text-sm text-slate-400">{k.created}</td>
                <td className="p-4 flex space-x-2">
                  <button className="p-2 text-slate-400 hover:text-primary rounded hover:bg-slate-800" title="Rotate Key">
                    <RefreshCw size={16} />
                  </button>
                  <button className="p-2 text-slate-400 hover:text-danger rounded hover:bg-slate-800" title="Crypto Shred">
                    <Trash2 size={16} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

function PhaseBadge({ phase }: { phase: string }) {
  const styles: Record<string, string> = {
    'Active': 'bg-success/10 text-success border-success/20',
    'Pre-Op': 'bg-secondary/10 text-secondary border-secondary/20',
    'Expired': 'bg-warning/10 text-warning border-warning/20',
    'Revoked': 'bg-danger/10 text-danger border-danger/20',
    'Shredded': 'bg-slate-500/10 text-slate-400 border-slate-500/20',
  };
  
  return (
    <span className={`px-2.5 py-1 text-xs font-semibold border rounded-full ${styles[phase] || styles['Shredded']}`}>
      {phase}
    </span>
  );
}
