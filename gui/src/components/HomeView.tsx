import { useState } from 'react';
import { 
  ShieldCheck, Coins, Bug, Vote, ArrowRight, Users, 
  ExternalLink, TrendingUp, AlertTriangle 
} from 'lucide-react';
import { MOCK_GRANTS, MOCK_ADVISORIES } from '../api/mockData';

interface HomeViewProps {
  onNavigate: (tab: string) => void;
}

export function HomeView({ onNavigate }: HomeViewProps) {
  const [calcContributors, setCalcContributors] = useState(25);
  const [calcAvgContrib, setCalcAvgContrib] = useState(50);

  // Quadratic Funding Formula demo calculation
  const totalDirect = calcContributors * calcAvgContrib;
  const quadraticSum = Math.pow(calcContributors * Math.sqrt(calcAvgContrib), 2);
  const matchingFund = Math.max(0, Math.round(quadraticSum - totalDirect));

  return (
    <div className="space-y-10 pb-12">
      {/* Hero Section */}
      <div className="relative rounded-2xl bg-gradient-to-br from-slate-900 via-slate-900 to-emerald-950/40 border border-slate-800 p-8 lg:p-12 overflow-hidden shadow-2xl">
        <div className="absolute top-0 right-0 w-96 h-96 bg-emerald-500/10 rounded-full blur-3xl -z-10 pointer-events-none" />
        <div className="absolute bottom-0 left-1/3 w-80 h-80 bg-cyan-500/10 rounded-full blur-3xl -z-10 pointer-events-none" />

        <div className="max-w-3xl space-y-6">
          <div className="inline-flex items-center space-x-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 text-xs font-semibold uppercase tracking-wider">
            <ShieldCheck size={14} />
            <span>Non-Profit Foundation for Open-Source Security</span>
          </div>

          <h1 className="text-3xl md:text-5xl font-extrabold text-slate-100 tracking-tight leading-tight">
            Protecting Critical Internet Infrastructure Through <span className="bg-gradient-to-r from-emerald-400 to-cyan-400 bg-clip-text text-transparent">Decentralized Public Goods Funding</span>
          </h1>

          <p className="text-slate-300 text-base md:text-lg leading-relaxed">
            Inspired by major Web3 ecosystem foundations, CyberShield mobilizes global security researchers, funds open-source cryptographic defense, operates public bug bounties, and governs security grants transparently.
          </p>

          <div className="flex flex-wrap items-center gap-4 pt-2">
            <button
              onClick={() => onNavigate('ecosystem')}
              className="flex items-center space-x-2 px-6 py-3 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold shadow-lg shadow-emerald-950/50 transition-all hover:scale-[1.02]"
            >
              <span>Explore Grants & Projects</span>
              <ArrowRight size={18} />
            </button>
            <button
              onClick={() => onNavigate('research')}
              className="flex items-center space-x-2 px-6 py-3 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-100 font-semibold border border-slate-700 transition-colors"
            >
              <Bug size={18} className="text-emerald-400" />
              <span>View Bug Bounties & CVEs</span>
            </button>
          </div>
        </div>

        {/* Global Impact Counters */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mt-10 pt-8 border-t border-slate-800/80">
          <div>
            <div className="text-2xl lg:text-3xl font-extrabold text-emerald-400 font-mono">$33.8M</div>
            <div className="text-xs text-slate-400 font-medium">Total Treasury Allocated</div>
          </div>
          <div>
            <div className="text-2xl lg:text-3xl font-extrabold text-cyan-400 font-mono">420+</div>
            <div className="text-xs text-slate-400 font-medium">Open Source Projects Secured</div>
          </div>
          <div>
            <div className="text-2xl lg:text-3xl font-extrabold text-emerald-400 font-mono">12,450</div>
            <div className="text-xs text-slate-400 font-medium">Global Security Researchers</div>
          </div>
          <div>
            <div className="text-2xl lg:text-3xl font-extrabold text-cyan-400 font-mono">1,820+</div>
            <div className="text-xs text-slate-400 font-medium">Vulnerabilities Mitigated</div>
          </div>
        </div>
      </div>

      {/* Live Threat Advisory Ticker */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-xl p-4 flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div className="flex items-center space-x-3">
          <div className="p-2 rounded-lg bg-red-500/10 text-red-400 border border-red-500/20 shrink-0">
            <AlertTriangle size={20} />
          </div>
          <div>
            <div className="flex items-center space-x-2">
              <span className="text-xs font-bold uppercase tracking-wider text-red-400">Live Threat Alert</span>
              <span className="text-xs text-slate-400">{MOCK_ADVISORIES[0].timestamp}</span>
            </div>
            <p className="text-sm font-semibold text-slate-200">{MOCK_ADVISORIES[0].title}</p>
          </div>
        </div>
        <div className="flex items-center space-x-3 w-full md:w-auto justify-between shrink-0">
          <span className="px-2.5 py-1 text-xs font-semibold rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
            Status: {MOCK_ADVISORIES[0].mitigationStatus}
          </span>
          <button 
            onClick={() => onNavigate('research')}
            className="text-xs font-semibold text-cyan-400 hover:text-cyan-300 flex items-center gap-1"
          >
            <span>Full Advisory Feed</span>
            <ExternalLink size={12} />
          </button>
        </div>
      </div>

      {/* Core Ecosystem Pillars Grid */}
      <div>
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-xl font-bold text-slate-100">Ecosystem Pillars</h2>
            <p className="text-xs text-slate-400">Community-driven programs for research, grants, and threat mitigation</p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {/* Pillar 1 */}
          <div 
            onClick={() => onNavigate('ecosystem')}
            className="group cursor-pointer bg-slate-900 border border-slate-800 hover:border-emerald-500/50 rounded-xl p-6 transition-all hover:shadow-lg hover:shadow-emerald-950/30"
          >
            <div className="w-12 h-12 rounded-xl bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
              <Coins size={24} />
            </div>
            <h3 className="text-lg font-bold text-slate-100 group-hover:text-emerald-400 transition-colors">
              Grants & Quadratic Funding
            </h3>
            <p className="text-xs text-slate-400 mt-2 leading-relaxed">
              Funding critical open-source security tools (TLS, SSH, kernels, cryptosystems) with milestone funding and community quadratic matching rounds.
            </p>
            <div className="mt-4 pt-4 border-t border-slate-800/80 flex items-center justify-between text-xs font-semibold text-emerald-400">
              <span>Active Round 4: $500k Matching</span>
              <ArrowRight size={14} className="group-hover:translate-x-1 transition-transform" />
            </div>
          </div>

          {/* Pillar 2 */}
          <div 
            onClick={() => onNavigate('research')}
            className="group cursor-pointer bg-slate-900 border border-slate-800 hover:border-cyan-500/50 rounded-xl p-6 transition-all hover:shadow-lg hover:shadow-cyan-950/30"
          >
            <div className="w-12 h-12 rounded-xl bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
              <Bug size={24} />
            </div>
            <h3 className="text-lg font-bold text-slate-100 group-hover:text-cyan-400 transition-colors">
              Vulnerability Bounties & CVEs
            </h3>
            <p className="text-xs text-slate-400 mt-2 leading-relaxed">
              Public bug bounty registry offering payouts up to $75,000 for zero-day disclosures in public infrastructure software.
            </p>
            <div className="mt-4 pt-4 border-t border-slate-800/80 flex items-center justify-between text-xs font-semibold text-cyan-400">
              <span>$8.5M Bounty Pool Active</span>
              <ArrowRight size={14} className="group-hover:translate-x-1 transition-transform" />
            </div>
          </div>

          {/* Pillar 3 */}
          <div 
            onClick={() => onNavigate('governance')}
            className="group cursor-pointer bg-slate-900 border border-slate-800 hover:border-purple-500/50 rounded-xl p-6 transition-all hover:shadow-lg hover:shadow-purple-950/30"
          >
            <div className="w-12 h-12 rounded-xl bg-purple-500/10 text-purple-400 border border-purple-500/20 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
              <Vote size={24} />
            </div>
            <h3 className="text-lg font-bold text-slate-100 group-hover:text-purple-400 transition-colors">
              Governance & Working Groups
            </h3>
            <p className="text-xs text-slate-400 mt-2 leading-relaxed">
              Special Interest Groups (SIGs) governing research proposals, formal verification standards, and real-time treasury allocations.
            </p>
            <div className="mt-4 pt-4 border-t border-slate-800/80 flex items-center justify-between text-xs font-semibold text-purple-400">
              <span>3 Active Proposals Voting</span>
              <ArrowRight size={14} className="group-hover:translate-x-1 transition-transform" />
            </div>
          </div>
        </div>
      </div>

      {/* Featured Grants & Quadratic Funding Demo */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Active Grants Preview */}
        <div className="lg:col-span-2 bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-6">
          <div className="flex items-center justify-between border-b border-slate-800 pb-4">
            <div>
              <h3 className="text-lg font-bold text-slate-100">Featured Security Grants</h3>
              <p className="text-xs text-slate-400">Milestone-based funding for critical public goods</p>
            </div>
            <button 
              onClick={() => onNavigate('ecosystem')}
              className="text-xs font-semibold text-emerald-400 hover:text-emerald-300"
            >
              View All 42 Grants →
            </button>
          </div>

          <div className="space-y-4">
            {MOCK_GRANTS.slice(0, 3).map((grant) => (
              <div key={grant.id} className="p-4 rounded-xl bg-slate-950/60 border border-slate-800/80 space-y-3 hover:border-slate-700 transition-colors">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <div className="flex items-center space-x-2">
                    <span className="px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-[10px] font-mono font-bold">
                      {grant.category}
                    </span>
                    <span className="text-xs text-slate-400">{grant.applicant}</span>
                  </div>
                  <span className="text-xs font-mono font-bold text-emerald-400 bg-slate-900 px-2 py-1 rounded border border-slate-800">
                    {grant.amountRequested}
                  </span>
                </div>

                <h4 className="font-bold text-slate-100 text-sm">{grant.title}</h4>
                <p className="text-xs text-slate-400 leading-relaxed">{grant.description}</p>

                <div className="flex items-center justify-between text-xs pt-2 border-t border-slate-900">
                  <div className="flex items-center space-x-1.5 text-slate-400">
                    <Users size={14} className="text-emerald-400" />
                    <span><strong>{grant.quadraticContributors}</strong> Community Supporters</span>
                  </div>
                  <span className="text-slate-400 font-mono text-[11px]">Impact Score: <strong className="text-slate-200">{grant.impactScore}/100</strong></span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Interactive Quadratic Funding Simulator */}
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-6 flex flex-col justify-between">
          <div>
            <div className="flex items-center space-x-2 text-emerald-400 font-bold text-sm mb-1">
              <TrendingUp size={16} />
              <span>Gitcoin-Style Quadratic Funding</span>
            </div>
            <h3 className="text-lg font-bold text-slate-100">Quadratic Match Calculator</h3>
            <p className="text-xs text-slate-400 mt-1 leading-relaxed">
              Quadratic funding mathematically amplifies projects supported by many individual community members, regardless of contribution size.
            </p>

            <div className="space-y-4 mt-6">
              <div>
                <div className="flex justify-between text-xs font-medium mb-1">
                  <span className="text-slate-300">Number of Donors</span>
                  <span className="text-emerald-400 font-mono font-bold">{calcContributors} contributors</span>
                </div>
                <input 
                  type="range" 
                  min="5" 
                  max="150" 
                  value={calcContributors}
                  onChange={(e) => setCalcContributors(parseInt(e.target.value))}
                  className="w-full accent-emerald-500 bg-slate-950 h-2 rounded-lg cursor-pointer"
                />
              </div>

              <div>
                <div className="flex justify-between text-xs font-medium mb-1">
                  <span className="text-slate-300">Avg Contribution / Donor</span>
                  <span className="text-emerald-400 font-mono font-bold">${calcAvgContrib} USD</span>
                </div>
                <input 
                  type="range" 
                  min="10" 
                  max="200" 
                  value={calcAvgContrib}
                  onChange={(e) => setCalcAvgContrib(parseInt(e.target.value))}
                  className="w-full accent-emerald-500 bg-slate-950 h-2 rounded-lg cursor-pointer"
                />
              </div>

              <div className="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-2 mt-4">
                <div className="flex justify-between text-xs text-slate-400">
                  <span>Direct Community Contributions:</span>
                  <span className="font-mono font-semibold text-slate-200">${totalDirect.toLocaleString()}</span>
                </div>
                <div className="flex justify-between text-xs text-emerald-400 font-bold">
                  <span>NGO Matching Grant Pool:</span>
                  <span className="font-mono text-sm">+${matchingFund.toLocaleString()}</span>
                </div>
                <div className="pt-2 border-t border-slate-800 flex justify-between text-sm font-extrabold text-slate-100">
                  <span>Total Project Funding:</span>
                  <span className="font-mono text-cyan-400">${(totalDirect + matchingFund).toLocaleString()}</span>
                </div>
              </div>
            </div>
          </div>

          <button 
            onClick={() => onNavigate('ecosystem')}
            className="w-full py-2.5 rounded-lg bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 font-semibold border border-emerald-500/30 text-xs transition-colors"
          >
            Participate in Round 4 Grants →
          </button>
        </div>
      </div>
    </div>
  );
}
