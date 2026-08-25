import { useState } from 'react';
import { MOCK_PROPOSALS, MOCK_TREASURY } from '../api/mockData';
import { GovProposal } from '../types/ngo';
import { Vote, Users, PieChart as PieIcon } from 'lucide-react';
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip, Legend } from 'recharts';

export function GovernanceView() {
  const [proposals, setProposals] = useState<GovProposal[]>(MOCK_PROPOSALS);
  const [votedMap, setVotedMap] = useState<Record<string, 'FOR' | 'AGAINST'>>({});

  const handleVote = (id: string, choice: 'FOR' | 'AGAINST') => {
    if (votedMap[id]) return;
    setVotedMap({ ...votedMap, [id]: choice });

    setProposals(proposals.map(p => {
      if (p.id === id) {
        return {
          ...p,
          votesFor: choice === 'FOR' ? p.votesFor + 1000 : p.votesFor,
          votesAgainst: choice === 'AGAINST' ? p.votesAgainst + 1000 : p.votesAgainst,
        };
      }
      return p;
    }));
  };

  const sigGroups = [
    { id: 'SIG-01', name: 'Infrastructure Protection', lead: 'Dr. Aris Thorne', members: 1420, activeGrants: 14 },
    { id: 'SIG-02', name: 'Crypto & Privacy Protocols', lead: 'Elena Rostova', members: 890, activeGrants: 9 },
    { id: 'SIG-03', name: 'Rapid Defense & SecOps', lead: 'Kaito Tanaka', members: 640, activeGrants: 7 },
    { id: 'SIG-04', name: 'Security Education & Talent', lead: 'Maya Lin', members: 2100, activeGrants: 12 },
  ];

  return (
    <div className="space-y-8 pb-12">
      {/* Header Banner */}
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-8 flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
        <div>
          <div className="inline-flex items-center space-x-2 text-xs font-semibold text-purple-400 uppercase tracking-wider mb-2">
            <Vote size={14} />
            <span>Decentralized Council & Special Interest Groups</span>
          </div>
          <h1 className="text-2xl md:text-3xl font-bold text-slate-100">Governance & Treasury Allocation</h1>
          <p className="text-slate-400 text-sm mt-1 max-w-2xl">
            Empowering ecosystem participants to vote on grant matching pools, vote on formal verification standards, and audit real-time treasury expenditures.
          </p>
        </div>

        <div className="flex items-center space-x-3 bg-slate-950 px-4 py-3 rounded-xl border border-slate-800">
          <div>
            <div className="text-[10px] text-slate-500 font-mono">Total Treasury Reserve</div>
            <div className="text-lg font-extrabold text-emerald-400 font-mono">$33,840,000 USD</div>
          </div>
        </div>
      </div>

      {/* Treasury Breakdown & Special Interest Groups */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Treasury Allocation Chart */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 flex flex-col justify-between space-y-4">
          <div>
            <div className="flex items-center justify-between border-b border-slate-800 pb-3">
              <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
                <PieIcon size={18} className="text-purple-400" />
                <span>Treasury Capital Allocation</span>
              </h3>
              <span className="text-xs font-mono text-emerald-400 font-semibold">Audited Quarterly</span>
            </div>

            <div className="h-64 mt-4">
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={MOCK_TREASURY}
                    cx="50%"
                    cy="50%"
                    innerRadius={55}
                    outerRadius={85}
                    paddingAngle={4}
                    dataKey="amountUsd"
                  >
                    {MOCK_TREASURY.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={entry.color} stroke="#0f172a" strokeWidth={2} />
                    ))}
                  </Pie>
                  <Tooltip 
                    formatter={(value: any) => [`$${(value as number).toLocaleString()} USD`, 'Allocation']}
                    contentStyle={{ backgroundColor: '#020617', borderColor: '#334155', borderRadius: '0.5rem', color: '#f8fafc' }}
                  />
                  <Legend 
                    wrapperStyle={{ fontSize: '11px', color: '#94a3b8' }}
                  />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-2 pt-2 border-t border-slate-800/80">
            {MOCK_TREASURY.slice(0, 4).map(item => (
              <div key={item.name} className="flex justify-between items-center text-xs p-2 rounded bg-slate-950/60 border border-slate-800">
                <span className="text-slate-400 font-medium truncate max-w-[120px]">{item.name}</span>
                <span className="font-mono font-bold text-slate-200">${(item.amountUsd / 1000000).toFixed(1)}M</span>
              </div>
            ))}
          </div>
        </div>

        {/* Special Interest Groups (SIGs) */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <div className="border-b border-slate-800 pb-3">
            <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
              <Users size={18} className="text-purple-400" />
              <span>Special Interest Groups (SIGs)</span>
            </h3>
            <p className="text-xs text-slate-400">Decentralized working groups driving research standards</p>
          </div>

          <div className="space-y-3">
            {sigGroups.map(sig => (
              <div key={sig.id} className="p-3.5 rounded-xl bg-slate-950 border border-slate-800 flex items-center justify-between hover:border-slate-700 transition-colors">
                <div>
                  <div className="flex items-center space-x-2">
                    <span className="text-xs font-mono font-bold text-purple-400">{sig.id}</span>
                    <span className="text-xs font-bold text-slate-200">{sig.name}</span>
                  </div>
                  <div className="text-[11px] text-slate-400 mt-0.5">
                    Lead: <strong className="text-slate-300">{sig.lead}</strong> • {sig.members} Members
                  </div>
                </div>

                <div className="text-right">
                  <span className="px-2.5 py-1 rounded bg-purple-500/10 text-purple-400 border border-purple-500/20 text-[11px] font-mono font-semibold">
                    {sig.activeGrants} Active Grants
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Governance Proposals Section */}
      <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-6">
        <div className="flex items-center justify-between border-b border-slate-800 pb-4">
          <div>
            <h3 className="text-lg font-bold text-slate-100">Ecosystem Governance Proposals</h3>
            <p className="text-xs text-slate-400">Community votes deciding policy, RPGF allocations, and standards</p>
          </div>
        </div>

        <div className="space-y-6">
          {proposals.map((prop) => {
            const totalVotes = prop.votesFor + prop.votesAgainst;
            const percentFor = totalVotes > 0 ? Math.round((prop.votesFor / totalVotes) * 100) : 0;
            const userVote = votedMap[prop.id];

            return (
              <div key={prop.id} className="bg-slate-950 border border-slate-800 rounded-xl p-6 space-y-4">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <div className="flex items-center space-x-3">
                    <span className="text-xs font-mono font-bold text-purple-400">{prop.id}</span>
                    <span className="px-2.5 py-0.5 rounded bg-purple-500/10 text-purple-400 border border-purple-500/20 text-[10px] font-mono font-semibold">
                      {prop.sigGroup}
                    </span>
                  </div>

                  <span className={`px-2.5 py-1 rounded text-xs font-mono font-bold ${
                    prop.status === 'Active Voting' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' :
                    prop.status === 'Passed' ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20' :
                    'bg-slate-800 text-slate-400'
                  }`}>
                    {prop.status} {prop.endsInDays > 0 ? `(${prop.endsInDays}d left)` : ''}
                  </span>
                </div>

                <div>
                  <h4 className="text-base font-bold text-slate-100">{prop.title}</h4>
                  <p className="text-xs text-slate-400 mt-1">Proposed by: <strong className="text-slate-300">{prop.author}</strong></p>
                </div>

                <p className="text-xs text-slate-300 leading-relaxed">{prop.summary}</p>

                {/* Vote Progress Bar */}
                <div className="space-y-2 pt-2">
                  <div className="flex justify-between text-xs font-mono">
                    <span className="text-emerald-400 font-bold">FOR: {prop.votesFor.toLocaleString()} ({percentFor}%)</span>
                    <span className="text-red-400 font-bold">AGAINST: {prop.votesAgainst.toLocaleString()} ({100 - percentFor}%)</span>
                  </div>
                  <div className="w-full h-2.5 bg-slate-900 rounded-full overflow-hidden flex">
                    <div style={{ width: `${percentFor}%` }} className="bg-emerald-500 h-full" />
                    <div style={{ width: `${100 - percentFor}%` }} className="bg-red-500 h-full" />
                  </div>
                </div>

                {/* Voting Action Buttons */}
                <div className="pt-3 border-t border-slate-900 flex items-center justify-between">
                  <span className="text-xs text-slate-500">1 Membership Token = 1 Vote</span>

                  {prop.status === 'Active Voting' && (
                    <div className="flex items-center space-x-2">
                      <button
                        onClick={() => handleVote(prop.id, 'FOR')}
                        disabled={!!userVote}
                        className={`px-4 py-1.5 rounded-lg text-xs font-bold transition-colors ${
                          userVote === 'FOR'
                            ? 'bg-emerald-500 text-slate-950'
                            : 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 hover:bg-emerald-500/20'
                        }`}
                      >
                        {userVote === 'FOR' ? 'Voted FOR ✓' : 'Vote FOR'}
                      </button>
                      <button
                        onClick={() => handleVote(prop.id, 'AGAINST')}
                        disabled={!!userVote}
                        className={`px-4 py-1.5 rounded-lg text-xs font-bold transition-colors ${
                          userVote === 'AGAINST'
                            ? 'bg-red-500 text-slate-950'
                            : 'bg-red-500/10 text-red-400 border border-red-500/30 hover:bg-red-500/20'
                        }`}
                      >
                        {userVote === 'AGAINST' ? 'Voted AGAINST ✓' : 'Vote AGAINST'}
                      </button>
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
