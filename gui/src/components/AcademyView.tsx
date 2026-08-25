import { useState } from 'react';
import { BookOpen, Code, Cpu, ArrowRight } from 'lucide-react';
import ZkpSandboxView from './ZkpSandboxView';

export function AcademyView() {
  const [activeTab, setActiveTab] = useState<'guides' | 'goodfirst' | 'zkp'>('guides');

  const pathways = [
    { title: 'Cryptographic Code Auditor', level: 'Intermediate', duration: '4 Weeks', tags: ['ZKP', 'Rust', 'Enclaves'], count: '1,420 Enrolled' },
    { title: 'Kernel & eBPF Security Specialist', level: 'Advanced', duration: '6 Weeks', tags: ['Linux', 'eBPF', 'C'], count: '890 Enrolled' },
    { title: 'Zero-Day Vulnerability Researcher', level: 'Advanced', duration: '8 Weeks', tags: ['Fuzzing', 'Assembly', 'CVE'], count: '2,150 Enrolled' },
    { title: 'Open-Source Security Maintainer', level: 'Beginner', duration: '2 Weeks', tags: ['SBOM', 'Dependencies', 'CI/CD'], count: '3,800 Enrolled' },
  ];

  const goodFirstIssues = [
    { id: 'ISSUE-104', title: 'Add Automated Fuzzing Target for Kyber-768 Decapsulation', repo: 'OpenSSL-PQC-Branch', bounty: '$500', difficulty: 'Good First Issue' },
    { id: 'ISSUE-98', title: 'Implement SPDX SBOM Generation Script in Cargo CI Pipeline', repo: 'KernelShield-Agent', bounty: '$300', difficulty: 'Good First Issue' },
    { id: 'ISSUE-84', title: 'Write Zero-Knowledge Proof Attestation Test suite for SGX Enclave', repo: 'ZKP-Attest-Core', bounty: '$750', difficulty: 'Intermediate' },
  ];

  return (
    <div className="space-y-8 pb-12">
      {/* Header Banner */}
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-8 flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
        <div>
          <div className="inline-flex items-center space-x-2 text-xs font-semibold text-emerald-400 uppercase tracking-wider mb-2">
            <BookOpen size={14} />
            <span>Developer & Security Researcher Academy</span>
          </div>
          <h1 className="text-2xl md:text-3xl font-bold text-slate-100">CyberShield Academy & Sandboxes</h1>
          <p className="text-slate-400 text-sm mt-1 max-w-2xl">
            Free open-source security curricula, hands-on interactive cryptographic sandboxes, and good-first-issues for aspiring security researchers.
          </p>
        </div>
      </div>

      {/* Navigation Sub-Tabs */}
      <div className="flex items-center space-x-4 border-b border-slate-800 pb-2">
        <button
          onClick={() => setActiveTab('guides')}
          className={`flex items-center space-x-2 px-4 py-2 rounded-lg text-xs font-bold transition-colors ${
            activeTab === 'guides'
              ? 'bg-slate-800 text-emerald-400 border border-slate-700'
              : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <BookOpen size={16} />
          <span>Learning Pathways & Guides</span>
        </button>
        <button
          onClick={() => setActiveTab('goodfirst')}
          className={`flex items-center space-x-2 px-4 py-2 rounded-lg text-xs font-bold transition-colors ${
            activeTab === 'goodfirst'
              ? 'bg-slate-800 text-cyan-400 border border-slate-700'
              : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <Code size={16} />
          <span>Good First Issues ({goodFirstIssues.length})</span>
        </button>
        <button
          onClick={() => setActiveTab('zkp')}
          className={`flex items-center space-x-2 px-4 py-2 rounded-lg text-xs font-bold transition-colors ${
            activeTab === 'zkp'
              ? 'bg-slate-800 text-purple-400 border border-slate-700'
              : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <Cpu size={16} />
          <span>Interactive ZKP & Key Sandbox</span>
        </button>
      </div>

      {/* Guides & Pathways View */}
      {activeTab === 'guides' && (
        <div className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {pathways.map((path) => (
              <div key={path.title} className="bg-slate-900 border border-slate-800 rounded-xl p-6 flex flex-col justify-between space-y-4 hover:border-slate-700 transition-colors">
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-[10px] font-mono font-bold">
                      {path.level} • {path.duration}
                    </span>
                    <span className="text-xs text-slate-400 font-medium">{path.count}</span>
                  </div>

                  <h3 className="text-lg font-bold text-slate-100">{path.title}</h3>

                  <div className="flex flex-wrap gap-1.5 pt-1">
                    {path.tags.map(t => (
                      <span key={t} className="px-2 py-0.5 rounded bg-slate-950 text-slate-400 text-[10px] font-mono border border-slate-800">
                        {t}
                      </span>
                    ))}
                  </div>
                </div>

                <div className="pt-4 border-t border-slate-800 flex items-center justify-between">
                  <span className="text-xs text-emerald-400 font-semibold">100% Free & Open-Source</span>
                  <button className="px-3.5 py-1.5 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold text-xs transition-colors flex items-center gap-1">
                    <span>Start Pathway</span>
                    <ArrowRight size={14} />
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Good First Issues View */}
      {activeTab === 'goodfirst' && (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <div className="border-b border-slate-800 pb-3">
            <h3 className="text-lg font-bold text-slate-100">Good First Security Issues</h3>
            <p className="text-xs text-slate-400">Earn entry-level micro-bounties while building your open-source security portfolio</p>
          </div>

          <div className="space-y-3">
            {goodFirstIssues.map(issue => (
              <div key={issue.id} className="p-4 rounded-xl bg-slate-950 border border-slate-800 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                <div>
                  <div className="flex items-center space-x-2">
                    <span className="text-xs font-mono font-bold text-cyan-400">{issue.id}</span>
                    <span className="text-xs text-slate-400">in <strong className="text-slate-300">{issue.repo}</strong></span>
                  </div>
                  <h4 className="text-sm font-bold text-slate-100 mt-1">{issue.title}</h4>
                </div>

                <div className="flex items-center space-x-3 w-full sm:w-auto justify-between shrink-0">
                  <span className="text-sm font-mono font-extrabold text-emerald-400 bg-slate-900 px-3 py-1 rounded border border-slate-800">
                    {issue.bounty}
                  </span>
                  <button className="px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-100 text-xs font-semibold border border-slate-700 transition-colors">
                    Claim Issue
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Interactive ZKP Sandbox Embed View */}
      {activeTab === 'zkp' && (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <div className="border-b border-slate-800 pb-3">
            <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
              <Cpu size={20} className="text-purple-400" />
              <span>Interactive Hardware Enclave & ZKP Sandbox</span>
            </h3>
            <p className="text-xs text-slate-400">Test cryptographic zero-knowledge proof attestations in real time</p>
          </div>

          <ZkpSandboxView />
        </div>
      )}
    </div>
  );
}
