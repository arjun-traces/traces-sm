import { Fingerprint, Calculator, Target } from 'lucide-react';

export default function ZkpSandboxView() {
  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">ZKP & Homomorphic Sandbox</h2>
      <p className="text-muted">Interactive proofs and homomorphic operations.</p>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        
        {/* Schnorr PoK */}
        <div className="card">
          <div className="flex items-center space-x-3 mb-4">
            <Fingerprint className="text-primary" />
            <h3 className="text-lg font-bold">Schnorr Proof of Knowledge</h3>
          </div>
          <p className="text-sm text-slate-400 mb-4">Prove possession of a private key without revealing it.</p>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Public Key (Target)</label>
              <input type="text" className="w-full bg-slate-800 border border-slate-700 rounded p-2 text-sm font-mono" defaultValue="0x..." />
            </div>
            <button className="btn btn-primary w-full">Generate Proof</button>
            <div className="p-3 bg-slate-900 border border-slate-800 rounded text-xs font-mono break-all text-slate-500">
              Proof Output...
            </div>
          </div>
        </div>

        {/* Bulletproofs */}
        <div className="card">
          <div className="flex items-center space-x-3 mb-4">
            <Target className="text-secondary" />
            <h3 className="text-lg font-bold">Bulletproof Range Proof</h3>
          </div>
          <p className="text-sm text-slate-400 mb-4">Prove a committed value lies in a specific range [0, 2^64-1].</p>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Secret Value</label>
              <input type="number" className="w-full bg-slate-800 border border-slate-700 rounded p-2 text-sm" placeholder="e.g., 42" />
            </div>
            <button className="btn btn-secondary w-full">Generate Range Proof</button>
          </div>
        </div>

        {/* Paillier HE */}
        <div className="card lg:col-span-2">
          <div className="flex items-center space-x-3 mb-4">
            <Calculator className="text-accent" />
            <h3 className="text-lg font-bold">Paillier Homomorphic Addition</h3>
          </div>
          <p className="text-sm text-slate-400 mb-4">Compute E(m1 + m2) = E(m1) * E(m2) without decrypting.</p>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
            <div>
              <label className="block text-sm font-medium mb-1">Value 1 (m1)</label>
              <input type="number" className="w-full bg-slate-800 border border-slate-700 rounded p-2 text-sm" placeholder="10" />
            </div>
            <div className="flex items-center justify-center font-bold text-2xl pt-6">+</div>
            <div>
              <label className="block text-sm font-medium mb-1">Value 2 (m2)</label>
              <input type="number" className="w-full bg-slate-800 border border-slate-700 rounded p-2 text-sm" placeholder="20" />
            </div>
          </div>
          
          <button className="btn bg-accent text-white hover:bg-fuchsia-500 w-full mb-4">Homomorphic Add Ciphertexts</button>
          
          <div className="p-4 bg-slate-900 border border-slate-800 rounded">
            <p className="text-sm font-medium mb-2">Encrypted Result E(m1 + m2):</p>
            <p className="text-xs font-mono break-all text-slate-400">waiting for computation...</p>
          </div>
        </div>

      </div>
    </div>
  );
}
