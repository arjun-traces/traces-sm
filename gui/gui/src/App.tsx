import { useState } from 'react';
import { Header } from './components/Header';
import { HomeView } from './components/HomeView';
import { EcosystemView } from './components/EcosystemView';
import { ResearchView } from './components/ResearchView';
import { GovernanceView } from './components/GovernanceView';
import { AcademyView } from './components/AcademyView';

export default function App() {
  const [activeTab, setActiveTab] = useState<string>('home');

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-emerald-500 selection:text-slate-950">
      <Header activeTab={activeTab} setActiveTab={setActiveTab} />
      
      <main className="flex-1 max-w-7xl w-full mx-auto px-6 py-8">
        {activeTab === 'home' && <HomeView onNavigate={(tab) => setActiveTab(tab)} />}
        {activeTab === 'ecosystem' && <EcosystemView />}
        {activeTab === 'research' && <ResearchView />}
        {activeTab === 'governance' && <GovernanceView />}
        {activeTab === 'academy' && <AcademyView />}
      </main>

      <footer className="border-t border-slate-800 bg-slate-900/50 py-8 px-6 text-slate-400 text-xs mt-12">
        <div className="max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
          <div className="flex items-center space-x-3">
            <span className="font-bold text-slate-200">CyberShield Foundation</span>
            <span>•</span>
            <span>Non-Profit Open-Source Security & Public Goods DAO</span>
          </div>
          <div className="flex items-center space-x-6 text-slate-400">
            <a href="#privacy" className="hover:text-emerald-400">Privacy Policy</a>
            <a href="#pgp" className="hover:text-emerald-400">PGP Key (0x92F1A)</a>
            <a href="#terms" className="hover:text-emerald-400">Grant Terms</a>
            <a href="#cve" className="hover:text-emerald-400">CVE Authority</a>
          </div>
        </div>
      </footer>
    </div>
  );
}
