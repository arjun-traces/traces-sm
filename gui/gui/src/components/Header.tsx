import { Shield, Coins, Bug, Vote, BookOpen, PlusCircle, ArrowUpRight } from 'lucide-react';

interface HeaderProps {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

export function Header({ activeTab, setActiveTab }: HeaderProps) {
  const tabs = [
    { id: 'home', label: 'Ecosystem Hub', icon: Shield },
    { id: 'ecosystem', label: 'Grants & Funding', icon: Coins },
    { id: 'research', label: 'Bounties & CVEs', icon: Bug },
    { id: 'governance', label: 'Governance & SIGs', icon: Vote },
    { id: 'academy', label: 'Academy & Tools', icon: BookOpen },
  ];

  return (
    <header className="border-b border-slate-800 bg-slate-900/90 backdrop-blur sticky top-0 z-50">
      {/* Top Banner Ticker */}
      <div className="bg-slate-950 border-b border-slate-800/80 px-6 py-1.5 text-xs text-slate-400 flex items-center justify-between">
        <div className="flex items-center space-x-6">
          <span className="flex items-center space-x-1.5 text-emerald-400 font-semibold">
            <span className="relative flex h-2 w-2">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
            </span>
            <span>Network Status: Normal Operational Defense</span>
          </span>
          <span className="hidden md:inline text-slate-500">|</span>
          <span className="hidden md:inline">Treasury Pool: <strong className="text-slate-200">$33,840,000 USD</strong></span>
          <span className="hidden md:inline text-slate-500">|</span>
          <span className="hidden lg:inline">Active RPGF Round 4: <strong className="text-emerald-400">$500,000 Match</strong></span>
        </div>
        <div className="flex items-center space-x-4">
          <a href="#github" className="hover:text-emerald-400 transition-colors flex items-center gap-1">
            <span>GitHub</span>
            <ArrowUpRight size={12} />
          </a>
          <a href="#discord" className="hover:text-emerald-400 transition-colors flex items-center gap-1">
            <span>Community Matrix</span>
            <ArrowUpRight size={12} />
          </a>
        </div>
      </div>

      {/* Main Header Nav */}
      <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
        {/* Brand Logo */}
        <div 
          onClick={() => setActiveTab('home')}
          className="flex items-center space-x-3 cursor-pointer group"
        >
          <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-emerald-500 to-cyan-600 flex items-center justify-center shadow-lg shadow-emerald-950/50 group-hover:scale-105 transition-transform">
            <Shield className="text-slate-950" size={24} strokeWidth={2.5} />
          </div>
          <div>
            <div className="flex items-center space-x-2">
              <span className="font-bold text-lg text-slate-100 tracking-tight">CyberShield</span>
              <span className="px-2 py-0.5 text-[10px] uppercase font-mono font-bold tracking-wider rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                NGO DAO
              </span>
            </div>
            <p className="text-[11px] text-slate-400 font-medium">Decentralized Security Public Goods</p>
          </div>
        </div>

        {/* Navigation Tabs */}
        <nav className="hidden md:flex items-center space-x-1">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            const isActive = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center space-x-2 px-3.5 py-2 rounded-lg text-sm font-medium transition-all ${
                  isActive
                    ? 'bg-slate-800 text-emerald-400 shadow-sm border border-slate-700/60'
                    : 'text-slate-300 hover:text-slate-100 hover:bg-slate-800/50'
                }`}
              >
                <Icon size={17} className={isActive ? 'text-emerald-400' : 'text-slate-400'} />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </nav>

        {/* Action Buttons */}
        <div className="flex items-center space-x-3">
          <button 
            onClick={() => setActiveTab('ecosystem')}
            className="hidden sm:flex items-center space-x-1.5 text-xs font-semibold px-3 py-2 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-slate-950 transition-colors shadow-md shadow-emerald-950/30"
          >
            <PlusCircle size={15} />
            <span>Apply for Grant</span>
          </button>
          <button 
            onClick={() => setActiveTab('research')}
            className="flex items-center space-x-1.5 text-xs font-semibold px-3 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-emerald-400 border border-emerald-500/30 transition-colors"
          >
            <Bug size={15} />
            <span>Submit Vulnerability</span>
          </button>
        </div>
      </div>
    </header>
  );
}
