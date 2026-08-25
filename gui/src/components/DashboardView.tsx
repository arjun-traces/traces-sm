import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { ShieldCheck, Zap, AlertTriangle, KeySquare } from 'lucide-react';

const data = [
  { name: '00:00', ops: 400 },
  { name: '04:00', ops: 300 },
  { name: '08:00', ops: 200 },
  { name: '12:00', ops: 278 },
  { name: '16:00', ops: 189 },
  { name: '20:00', ops: 239 },
  { name: '24:00', ops: 349 },
];

export default function DashboardView() {
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard title="Active Secrets" value="8,432" icon={<KeySquare className="text-primary" />} trend="+12% today" />
        <StatCard title="Crypto Ops / Sec" value="1,240" icon={<Zap className="text-warning" />} trend="Stable" />
        <StatCard title="Enclave Health" value="100%" icon={<ShieldCheck className="text-success" />} trend="Secure" />
        <StatCard title="Policy Alerts" value="0" icon={<AlertTriangle className="text-danger" />} trend="No alerts" />
      </div>

      <div className="card h-96">
        <h3 className="text-lg font-medium mb-4">Cryptographic Operations Volume</h3>
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#334155" />
            <XAxis dataKey="name" stroke="#94a3b8" />
            <YAxis stroke="#94a3b8" />
            <Tooltip 
              contentStyle={{ backgroundColor: '#1e293b', borderColor: '#334155', color: '#f8fafc' }}
              itemStyle={{ color: '#38bdf8' }}
            />
            <Line type="monotone" dataKey="ops" stroke="#38bdf8" strokeWidth={2} dot={{ fill: '#38bdf8' }} activeDot={{ r: 8 }} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}

function StatCard({ title, value, icon, trend }: { title: string, value: string, icon: React.ReactNode, trend: string }) {
  return (
    <div className="card flex items-center justify-between">
      <div>
        <p className="text-sm font-medium text-slate-400">{title}</p>
        <p className="text-3xl font-bold mt-2">{value}</p>
        <p className="text-xs text-slate-500 mt-1">{trend}</p>
      </div>
      <div className="p-3 bg-slate-800 rounded-lg">
        {icon}
      </div>
    </div>
  );
}
