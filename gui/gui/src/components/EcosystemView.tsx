import React, { useState } from 'react';
import { MOCK_GRANTS } from '../api/mockData';
import { GrantProposal } from '../types/ngo';
import { Coins, Search, PlusCircle, Users } from 'lucide-react';

export function EcosystemView() {
  const [selectedCategory, setSelectedCategory] = useState<string>('All');
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [showApplyModal, setShowApplyModal] = useState<boolean>(false);
  const [grantsList, setGrantsList] = useState<GrantProposal[]>(MOCK_GRANTS);

  // New Grant Form state
  const [newTitle, setNewTitle] = useState('');
  const [newApplicant, setNewApplicant] = useState('');
  const [newCategory, setNewCategory] = useState<GrantProposal['category']>('Infrastructure Defense');
  const [newAmount, setNewAmount] = useState('$50,000');
  const [newDesc, setNewDesc] = useState('');

  const categories = ['All', 'Infrastructure Defense', 'Cryptographic Research', 'Defensive Tooling', 'Auditing & Verification'];

  const filteredGrants = grantsList.filter(g => {
    const matchesCategory = selectedCategory === 'All' || g.category === selectedCategory;
    const matchesSearch = g.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
                          g.applicant.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          g.tags.some(t => t.toLowerCase().includes(searchQuery.toLowerCase()));
    return matchesCategory && matchesSearch;
  });

  const handleCreateGrant = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle || !newApplicant || !newDesc) return;

    const newGrant: GrantProposal = {
      id: `GRANT-2026-${Math.floor(100 + Math.random() * 900)}`,
      title: newTitle,
      applicant: newApplicant,
      category: newCategory,
      amountRequested: newAmount,
      status: 'Active Round',
      quadraticContributors: 1,
      tags: ['Open-Source', 'Community', 'Public-Goods'],
      description: newDesc,
      impactScore: 88,
    };

    setGrantsList([newGrant, ...grantsList]);
    setShowApplyModal(false);
    setNewTitle('');
    setNewApplicant('');
    setNewDesc('');
  };

  return (
    <div className="space-y-8 pb-12">
      {/* Header Banner */}
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-8 flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
        <div>
          <div className="inline-flex items-center space-x-2 text-xs font-semibold text-emerald-400 uppercase tracking-wider mb-2">
            <Coins size={14} />
            <span>Public Goods Funding Engine</span>
          </div>
          <h1 className="text-2xl md:text-3xl font-bold text-slate-100">Cybersecurity Ecosystem Grants</h1>
          <p className="text-slate-400 text-sm mt-1 max-w-2xl">
            Supporting open-source maintainers, cryptographic researchers, and defensive security architects through milestone grants and quadratic community matching.
          </p>
        </div>

        <button 
          onClick={() => setShowApplyModal(true)}
          className="flex items-center space-x-2 px-5 py-3 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold shadow-lg shadow-emerald-950/40 transition-colors shrink-0"
        >
          <PlusCircle size={18} />
          <span>Apply for Security Grant</span>
        </button>
      </div>

      {/* Filter & Search Bar */}
      <div className="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-4">
        {/* Category Pills */}
        <div className="flex items-center space-x-2 overflow-x-auto pb-2 md:pb-0 scrollbar-none">
          {categories.map((cat) => (
            <button
              key={cat}
              onClick={() => setSelectedCategory(cat)}
              className={`px-3.5 py-2 rounded-lg text-xs font-semibold whitespace-nowrap transition-colors ${
                selectedCategory === cat
                  ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/30'
                  : 'bg-slate-900 text-slate-400 hover:text-slate-200 border border-slate-800'
              }`}
            >
              {cat}
            </button>
          ))}
        </div>

        {/* Search Bar */}
        <div className="relative w-full md:w-72">
          <Search size={16} className="absolute left-3.5 top-1/2 -translate-y-1/2 text-slate-500" />
          <input
            type="text"
            placeholder="Search projects, tags, labs..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-900 border border-slate-800 rounded-lg pl-10 pr-4 py-2 text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-emerald-500/50"
          />
        </div>
      </div>

      {/* Grants Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {filteredGrants.map((grant) => (
          <div key={grant.id} className="bg-slate-900 border border-slate-800 hover:border-slate-700 rounded-xl p-6 flex flex-col justify-between space-y-4 transition-all">
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="px-2.5 py-1 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-[11px] font-mono font-bold">
                  {grant.category}
                </span>
                <span className={`px-2.5 py-1 rounded text-[11px] font-mono font-bold ${
                  grant.status === 'Completed' ? 'bg-slate-800 text-slate-300' : 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20'
                }`}>
                  {grant.status}
                </span>
              </div>

              <div>
                <h3 className="text-base font-bold text-slate-100 leading-snug">{grant.title}</h3>
                <p className="text-xs text-slate-400 font-medium mt-1">Applicant: <strong className="text-slate-200">{grant.applicant}</strong></p>
              </div>

              <p className="text-xs text-slate-300 leading-relaxed">{grant.description}</p>

              <div className="flex flex-wrap gap-1.5 pt-1">
                {grant.tags.map(tag => (
                  <span key={tag} className="px-2 py-0.5 rounded bg-slate-950 text-slate-400 text-[10px] font-mono border border-slate-800">
                    #{tag}
                  </span>
                ))}
              </div>
            </div>

            <div className="pt-4 border-t border-slate-800/80 flex items-center justify-between text-xs">
              <div>
                <div className="text-[10px] text-slate-500 font-mono">Grant Value</div>
                <div className="text-sm font-extrabold text-emerald-400 font-mono">{grant.amountRequested}</div>
              </div>

              <div className="flex items-center space-x-3">
                <span className="text-slate-400 flex items-center gap-1 text-[11px]">
                  <Users size={14} className="text-cyan-400" />
                  <strong>{grant.quadraticContributors}</strong> Donors
                </span>
                <button 
                  onClick={() => {
                    setGrantsList(grantsList.map(g => g.id === grant.id ? { ...g, quadraticContributors: g.quadraticContributors + 1 } : g));
                  }}
                  className="px-3 py-1.5 rounded-lg bg-emerald-500/10 hover:bg-emerald-500/20 text-emerald-400 font-semibold border border-emerald-500/30 text-xs transition-colors"
                >
                  + Support Grant
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Grant Application Modal */}
      {showApplyModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm">
          <div className="bg-slate-900 border border-slate-800 rounded-2xl max-w-xl w-full p-6 shadow-2xl space-y-6">
            <div className="flex items-center justify-between border-b border-slate-800 pb-4">
              <div>
                <h3 className="text-lg font-bold text-slate-100">Apply for Security Public Goods Grant</h3>
                <p className="text-xs text-slate-400">Submit your project proposal for community voting and matching</p>
              </div>
              <button 
                onClick={() => setShowApplyModal(false)}
                className="text-slate-400 hover:text-slate-200 text-sm font-mono"
              >
                ✕
              </button>
            </div>

            <form onSubmit={handleCreateGrant} className="space-y-4 text-xs">
              <div>
                <label className="block text-slate-300 font-medium mb-1">Project / Proposal Title</label>
                <input
                  type="text"
                  required
                  placeholder="e.g. Memory-Safe Cryptographic Driver for Linux Kernel"
                  value={newTitle}
                  onChange={(e) => setNewTitle(e.target.value)}
                  className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-emerald-500/50"
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-slate-300 font-medium mb-1">Lead Applicant / Org</label>
                  <input
                    type="text"
                    required
                    placeholder="e.g. OpenCrypto Collective"
                    value={newApplicant}
                    onChange={(e) => setNewApplicant(e.target.value)}
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-emerald-500/50"
                  />
                </div>
                <div>
                  <label className="block text-slate-300 font-medium mb-1">Grant Category</label>
                  <select
                    value={newCategory}
                    onChange={(e) => setNewCategory(e.target.value as any)}
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 focus:outline-none focus:border-emerald-500/50"
                  >
                    <option value="Infrastructure Defense">Infrastructure Defense</option>
                    <option value="Cryptographic Research">Cryptographic Research</option>
                    <option value="Defensive Tooling">Defensive Tooling</option>
                    <option value="Auditing & Verification">Auditing & Verification</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="block text-slate-300 font-medium mb-1">Requested Funding Amount (USD)</label>
                <input
                  type="text"
                  required
                  placeholder="e.g. $75,000"
                  value={newAmount}
                  onChange={(e) => setNewAmount(e.target.value)}
                  className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-emerald-500/50 font-mono"
                />
              </div>

              <div>
                <label className="block text-slate-300 font-medium mb-1">Project Description & Milestones</label>
                <textarea
                  required
                  rows={4}
                  placeholder="Outline project objectives, open-source repository link, key milestones, and public benefit..."
                  value={newDesc}
                  onChange={(e) => setNewDesc(e.target.value)}
                  className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-emerald-500/50"
                />
              </div>

              <div className="pt-4 border-t border-slate-800 flex justify-end space-x-3">
                <button
                  type="button"
                  onClick={() => setShowApplyModal(false)}
                  className="px-4 py-2 rounded-lg bg-slate-800 text-slate-300 font-medium hover:bg-slate-700"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-5 py-2 rounded-lg bg-emerald-500 text-slate-950 font-bold hover:bg-emerald-400"
                >
                  Submit Grant Proposal
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
