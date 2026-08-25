import { Server, Shield } from 'lucide-react';

export function TopologyView() {
  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">DKG & Topology Map</h2>
      <p className="text-muted">Interactive node topology map: Local SGX TEE vs M-of-N threshold peers.</p>

      <div className="card h-[600px] flex items-center justify-center relative overflow-hidden bg-slate-900 border-slate-800">
        {/* Simple static representation of a topology for UI mockup */}
        <div className="absolute inset-0 opacity-20 pointer-events-none" style={{
          backgroundImage: 'radial-gradient(circle at center, #38bdf8 1px, transparent 1px)',
          backgroundSize: '40px 40px'
        }}></div>

        <div className="relative z-10 flex flex-col items-center">
          
          <div className="flex flex-col items-center mb-16">
            <div className="w-24 h-24 bg-primary/20 border-2 border-primary rounded-full flex items-center justify-center shadow-[0_0_30px_rgba(56,189,248,0.3)]">
              <Shield size={40} className="text-primary" />
            </div>
            <h3 className="mt-4 font-bold text-lg text-primary">Local SGX Enclave</h3>
            <p className="text-xs text-slate-400 font-mono">0x7F...3B</p>
          </div>

          {/* Lines */}
          <div className="absolute top-24 bottom-24 left-1/2 w-0.5 bg-gradient-to-b from-primary to-secondary -translate-x-1/2 z-[-1]"></div>

          <div className="grid grid-cols-3 gap-16 mt-8">
            <PeerNode id="Peer 1 (AWS)" active />
            <PeerNode id="Peer 2 (GCP)" active />
            <PeerNode id="Peer 3 (Azure)" active={false} />
          </div>

          <div className="mt-12 text-center p-4 bg-slate-800 rounded-lg border border-slate-700">
            <p className="text-sm font-medium">Threshold Configuration</p>
            <p className="text-2xl font-bold text-secondary mt-1">2-of-3 Quorum</p>
            <p className="text-xs text-success mt-1">Quorum Reached</p>
          </div>

        </div>
      </div>
    </div>
  );
}

function PeerNode({ id, active }: { id: string, active: boolean }) {
  return (
    <div className={`flex flex-col items-center p-4 rounded-xl border ${active ? 'bg-surface border-slate-600' : 'bg-slate-800/50 border-slate-800 opacity-50'}`}>
      <Server size={32} className={active ? 'text-secondary' : 'text-slate-500'} />
      <span className="mt-2 text-sm font-medium">{id}</span>
      <span className={`text-xs mt-1 ${active ? 'text-success' : 'text-danger'}`}>
        {active ? 'Online' : 'Offline'}
      </span>
    </div>
  );
}
