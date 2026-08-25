import { CheckCircle2 } from 'lucide-react';
import { LineChart, Line, YAxis, CartesianGrid, ResponsiveContainer } from 'recharts';

const entropyData = Array.from({ length: 50 }, (_, i) => ({
  time: i,
  value: 0.9 + Math.random() * 0.09
}));

export function EntropyView() {
  return (
    <div className="space-y-6">
      <h2 className="text-2xl font-bold">NIST Entropy & Health</h2>
      <p className="text-muted">Real-time SP 800-90B DRBG status.</p>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="card">
          <div className="flex justify-between items-start mb-4">
            <div>
              <h3 className="font-semibold text-lg">Repetition Count Test</h3>
              <p className="text-sm text-slate-400">Continuous Health Test</p>
            </div>
            <CheckCircle2 className="text-success" size={24} />
          </div>
          <div className="p-4 bg-slate-800 rounded-lg flex justify-between items-center">
            <span className="text-sm">Status:</span>
            <span className="font-medium text-success">PASS</span>
          </div>
        </div>

        <div className="card">
          <div className="flex justify-between items-start mb-4">
            <div>
              <h3 className="font-semibold text-lg">Adaptive Proportion Test</h3>
              <p className="text-sm text-slate-400">Continuous Health Test</p>
            </div>
            <CheckCircle2 className="text-success" size={24} />
          </div>
          <div className="p-4 bg-slate-800 rounded-lg flex justify-between items-center">
            <span className="text-sm">Status:</span>
            <span className="font-medium text-success">PASS</span>
          </div>
        </div>
      </div>

      <div className="card h-80">
        <div className="flex justify-between items-center mb-6">
          <h3 className="text-lg font-medium">Min-Entropy Estimate (bits/byte)</h3>
          <div className="px-3 py-1 bg-slate-800 rounded text-sm font-mono text-primary">
            Current: 7.98 bits
          </div>
        </div>
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={entropyData}>
            <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
            <YAxis domain={[0, 8]} stroke="#94a3b8" />
            <Line type="monotone" dataKey="value" stroke="#10b981" strokeWidth={2} dot={false} isAnimationActive={false} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
