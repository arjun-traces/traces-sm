import React, { useState } from 'react';
import { MOCK_BOUNTIES, MOCK_ADVISORIES } from '../api/mockData';
import { VulnerabilityBounty } from '../types/ngo';
import { Bug, ShieldAlert, Lock, Send } from 'lucide-react';

export function ResearchView() {
  const [bountiesList, setBountiesList] = useState<VulnerabilityBounty[]>(MOCK_BOUNTIES);
  const [showSubmitModal, setShowSubmitModal] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState<'bounties' | 'advisories'>('bounties');

  // Submit Vulnerability state
  const [vulnTitle, setVulnTitle] = useState('');
  const [targetProj, setTargetProj] = useState('');
  const [severity, setSeverity] = useState<VulnerabilityBounty['severity']>('High');
  const [reporter, setReporter] = useState('');
  const [payload, setPayload] = useState('');
  const [isEncrypted, setIsEncrypted] = useState(true);

  const handleReportSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!vulnTitle || !targetProj) return;

    const newBounty: VulnerabilityBounty = {
      id: `VULN-${Math.floor(8000 + Math.random() * 1000)}`,
      title: vulnTitle,
      targetProject: targetProj,
      severity: severity,
      rewardPool: severity === 'Critical' ? '$75,000' : severity === 'High' ? '$25,000' : '$10,000',
      status: 'Under Audit',
      disclosedDate: '2026-08-25',
      reporter: reporter || 'anonymous_researcher',
    };

    setBountiesList([newBounty, ...bountiesList]);
    setShowSubmitModal(false);
    setVulnTitle('');
    setTargetProj('');
    setPayload('');
  };

  return (
    <div className="space-y-8 pb-12">
      {/* Header Banner */}
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-8 flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
        <div>
          <div className="inline-flex items-center space-x-2 text-xs font-semibold text-cyan-400 uppercase tracking-wider mb-2">
            <Bug size={14} />
            <span>Coordinated Disclosure & Auditing Engine</span>
          </div>
          <h1 className="text-2xl md:text-3xl font-bold text-slate-100">Public Bug Bounties & Threat Intelligence</h1>
          <p className="text-slate-400 text-sm mt-1 max-w-2xl">
            Empowering ethical security researchers to identify zero-days, verify cryptographic codebases, and protect public open-source infrastructure with transparent payout bounties.
          </p>
        </div>

        <button 
          onClick={() => setShowSubmitModal(true)}
          className="flex items-center space-x-2 px-5 py-3 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-bold shadow-lg shadow-cyan-950/40 transition-colors shrink-0"
        >
          <Lock size={18} />
          <span>Report Vulnerability (PGP Encrypted)</span>
        </button>
      </div>

      {/* Navigation Sub-Tabs */}
      <div className="flex items-center space-x-4 border-b border-slate-800 pb-2">
        <button
          onClick={() => setActiveTab('bounties')}
          className={`flex items-center space-x-2 px-4 py-2 rounded-lg text-xs font-bold transition-colors ${
            activeTab === 'bounties'
              ? 'bg-slate-800 text-cyan-400 border border-slate-700'
              : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <Bug size={16} />
          <span>Active Bug Bounties ({bountiesList.length})</span>
        </button>
        <button
          onClick={() => setActiveTab('advisories')}
          className={`flex items-center space-x-2 px-4 py-2 rounded-lg text-xs font-bold transition-colors ${
            activeTab === 'advisories'
              ? 'bg-slate-800 text-red-400 border border-slate-700'
              : 'text-slate-400 hover:text-slate-200'
          }`}
        >
          <ShieldAlert size={16} />
          <span>Threat Advisory Feed ({MOCK_ADVISORIES.length})</span>
        </button>
      </div>

      {/* Bounties Tab View */}
      {activeTab === 'bounties' && (
        <div className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {bountiesList.map((bounty) => (
              <div key={bounty.id} className="bg-slate-900 border border-slate-800 rounded-xl p-6 flex flex-col justify-between space-y-4">
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <span className={`px-2.5 py-0.5 rounded text-[11px] font-mono font-bold ${
                      bounty.severity === 'Critical' ? 'bg-red-500/10 text-red-400 border border-red-500/30' :
                      bounty.severity === 'High' ? 'bg-amber-500/10 text-amber-400 border border-amber-500/30' :
                      'bg-emerald-500/10 text-emerald-400 border border-emerald-500/30'
                    }`}>
                      {bounty.severity} Severity
                    </span>
                    {bounty.cveId && (
                      <span className="text-xs font-mono text-cyan-400 font-semibold">{bounty.cveId}</span>
                    )}
                  </div>

                  <div>
                    <h3 className="text-base font-bold text-slate-100">{bounty.title}</h3>
                    <p className="text-xs text-slate-400 mt-1">Target Project: <strong className="text-slate-200">{bounty.targetProject}</strong></p>
                  </div>

                  <div className="flex items-center justify-between text-xs text-slate-400 pt-2 border-t border-slate-800">
                    <span>Disclosed: <strong className="text-slate-300">{bounty.disclosedDate}</strong></span>
                    <span>Reporter: <strong className="text-slate-300">@{bounty.reporter}</strong></span>
                  </div>
                </div>

                <div className="pt-4 border-t border-slate-800 flex items-center justify-between">
                  <div>
                    <div className="text-[10px] text-slate-500 font-mono">Bounty Payout Pool</div>
                    <div className="text-base font-extrabold text-cyan-400 font-mono">{bounty.rewardPool}</div>
                  </div>

                  <span className={`px-3 py-1 rounded-full text-xs font-mono font-bold ${
                    bounty.status === 'Patched' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' :
                    bounty.status === 'Open Bounty' ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/20' :
                    'bg-slate-800 text-slate-400'
                  }`}>
                    {bounty.status}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Threat Advisories Tab View */}
      {activeTab === 'advisories' && (
        <div className="bg-slate-900 border border-slate-800 rounded-xl p-6 space-y-4">
          <div className="flex items-center justify-between border-b border-slate-800 pb-4">
            <div>
              <h3 className="text-lg font-bold text-slate-100">Live Coordinated Vulnerability Feed</h3>
              <p className="text-xs text-slate-400">Verified security advisories for public infrastructure software</p>
            </div>
          </div>

          <div className="space-y-3">
            {MOCK_ADVISORIES.map((adv) => (
              <div key={adv.id} className="p-4 rounded-xl bg-slate-950 border border-slate-800/80 flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
                <div className="flex items-start space-x-3">
                  <div className="p-2 rounded bg-red-500/10 text-red-400 border border-red-500/20 shrink-0">
                    <ShieldAlert size={18} />
                  </div>
                  <div>
                    <div className="flex items-center space-x-2">
                      <span className="text-xs font-mono font-bold text-slate-400">{adv.id}</span>
                      <span className="text-[10px] text-slate-500">• {adv.timestamp}</span>
                    </div>
                    <h4 className="text-sm font-bold text-slate-100 mt-0.5">{adv.title}</h4>
                    <p className="text-xs text-slate-400">Affected Component: <strong className="text-slate-300">{adv.affectedComponent}</strong></p>
                  </div>
                </div>

                <span className="px-3 py-1 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 shrink-0">
                  {adv.mitigationStatus}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Vulnerability Report Submission Modal */}
      {showSubmitModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm">
          <div className="bg-slate-900 border border-slate-800 rounded-2xl max-w-xl w-full p-6 shadow-2xl space-y-6">
            <div className="flex items-center justify-between border-b border-slate-800 pb-4">
              <div>
                <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
                  <Lock size={18} className="text-cyan-400" />
                  <span>Submit Vulnerability Disclosure</span>
                </h3>
                <p className="text-xs text-slate-400">Secure, end-to-end encrypted submission to NGO Incident Response Team</p>
              </div>
              <button 
                onClick={() => setShowSubmitModal(false)}
                className="text-slate-400 hover:text-slate-200 text-sm font-mono"
              >
                ✕
              </button>
            </div>

            <form onSubmit={handleReportSubmit} className="space-y-4 text-xs">
              <div>
                <label className="block text-slate-300 font-medium mb-1">Vulnerability Title / Summary</label>
                <input
                  type="text"
                  required
                  placeholder="e.g. Unchecked Array Bounds in Cryptographic Enclave Mailbox"
                  value={vulnTitle}
                  onChange={(e) => setVulnTitle(e.target.value)}
                  className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-cyan-500/50"
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-slate-300 font-medium mb-1">Target Project / Repository</label>
                  <input
                    type="text"
                    required
                    placeholder="e.g. OpenSSL / Linux Kernel"
                    value={targetProj}
                    onChange={(e) => setTargetProj(e.target.value)}
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-cyan-500/50"
                  />
                </div>
                <div>
                  <label className="block text-slate-300 font-medium mb-1">Estimated Severity</label>
                  <select
                    value={severity}
                    onChange={(e) => setSeverity(e.target.value as any)}
                    className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 focus:outline-none focus:border-cyan-500/50"
                  >
                    <option value="Critical">Critical ($75,000 Payout Pool)</option>
                    <option value="High">High ($25,000 Payout Pool)</option>
                    <option value="Medium">Medium ($10,000 Payout Pool)</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="block text-slate-300 font-medium mb-1">Researcher Handle / Alias (Optional)</label>
                <input
                  type="text"
                  placeholder="e.g. sec_researcher99"
                  value={reporter}
                  onChange={(e) => setReporter(e.target.value)}
                  className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 focus:outline-none focus:border-cyan-500/50"
                />
              </div>

              <div>
                <div className="flex justify-between items-center mb-1">
                  <label className="text-slate-300 font-medium">Proof of Concept / Technical Details</label>
                  <label className="flex items-center space-x-1.5 cursor-pointer text-cyan-400">
                    <input 
                      type="checkbox" 
                      checked={isEncrypted} 
                      onChange={(e) => setIsEncrypted(e.target.checked)}
                      className="accent-cyan-500"
                    />
                    <span>Encrypt payload with NGO PGP Key (0x92F1A...)</span>
                  </label>
                </div>
                <textarea
                  required
                  rows={4}
                  placeholder="Provide step-by-step reproduction steps, stack trace, or PoC code snippet..."
                  value={payload}
                  onChange={(e) => setPayload(e.target.value)}
                  className="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-slate-100 placeholder-slate-600 font-mono text-[11px] focus:outline-none focus:border-cyan-500/50"
                />
              </div>

              <div className="pt-4 border-t border-slate-800 flex justify-end space-x-3">
                <button
                  type="button"
                  onClick={() => setShowSubmitModal(false)}
                  className="px-4 py-2 rounded-lg bg-slate-800 text-slate-300 font-medium hover:bg-slate-700"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-5 py-2 rounded-lg bg-cyan-500 text-slate-950 font-bold hover:bg-cyan-400 flex items-center gap-1.5"
                >
                  <Send size={14} />
                  <span>Submit Vulnerability</span>
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}
