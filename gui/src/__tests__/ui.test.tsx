import { describe, it, expect, beforeAll } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import App from '../App';
import { Header } from '../components/Header';
import { HomeView } from '../components/HomeView';
import { EcosystemView } from '../components/EcosystemView';
import { GovernanceView } from '../components/GovernanceView';
import { AcademyView } from '../components/AcademyView';
import { ResearchView } from '../components/ResearchView';

beforeAll(() => {
  (globalThis as unknown as { ResizeObserver: unknown }).ResizeObserver = class ResizeObserver {
    observe() {}
    unobserve() {}
    disconnect() {}
  };
});

describe('traces-sm GUI UI Test Suite', () => {
  describe('Header Component', () => {
    it('renders logo, title, and all navigation buttons', () => {
      let currentTab = 'home';
      const setTab = (tab: string) => { currentTab = tab; };

      render(<Header activeTab={currentTab} setActiveTab={setTab} />);

      expect(screen.getByText('CyberShield')).toBeDefined();
      expect(screen.getByText('Ecosystem Hub')).toBeDefined();
      expect(screen.getByText('Grants & Funding')).toBeDefined();
      expect(screen.getByText('Bounties & CVEs')).toBeDefined();
      expect(screen.getByText('Governance & SIGs')).toBeDefined();
      expect(screen.getByText('Academy & Tools')).toBeDefined();
    });

    it('triggers tab switch on button click', () => {
      let selectedTab = 'home';
      const handleTabChange = (tab: string) => { selectedTab = tab; };

      render(<Header activeTab="home" setActiveTab={handleTabChange} />);
      const governanceBtn = screen.getByText('Governance & SIGs');
      fireEvent.click(governanceBtn);
      expect(selectedTab).toBe('governance');
    });
  });

  describe('HomeView Component', () => {
    it('renders primary hero, key statistics and action buttons', () => {
      render(<HomeView onNavigate={() => {}} />);
      expect(screen.getByText(/Non-Profit Foundation for Open-Source Security/i)).toBeDefined();
      expect(screen.getByText(/Decentralized Public Goods Funding/i)).toBeDefined();
    });
  });

  describe('EcosystemView Component', () => {
    it('renders ecosystem integrations and grant categories', () => {
      render(<EcosystemView />);
      expect(screen.getAllByText('Infrastructure Defense').length).toBeGreaterThan(0);
      expect(screen.getAllByText('Cryptographic Research').length).toBeGreaterThan(0);
    });
  });

  describe('GovernanceView Component', () => {
    it('renders DAO governance proposals and special interest groups', () => {
      render(<GovernanceView />);
      expect(screen.getAllByText('Infrastructure Protection').length).toBeGreaterThan(0);
      expect(screen.getAllByText('Crypto & Privacy Protocols').length).toBeGreaterThan(0);
    });
  });

  describe('AcademyView Component', () => {
    it('renders curriculum tracks and enclave cryptographic tutorials', () => {
      render(<AcademyView />);
      expect(screen.getByText(/CyberShield Academy & Sandboxes/i)).toBeDefined();
      expect(screen.getByText('Cryptographic Code Auditor')).toBeDefined();
    });
  });

  describe('ResearchView Component', () => {
    it('renders vulnerability bounties and disclosure forms', () => {
      render(<ResearchView />);
      expect(screen.getByText(/Public Bug Bounties & Threat Intelligence/i)).toBeDefined();
      expect(screen.getByText(/Report Vulnerability/i)).toBeDefined();
    });
  });

  describe('Full App Integration', () => {
    it('renders default Home view and transitions through navigation tabs', () => {
      render(<App />);

      // Initially on Home tab
      expect(screen.getByText(/Non-Profit Foundation for Open-Source Security/i)).toBeDefined();

      // Navigate to Grants & Funding
      fireEvent.click(screen.getByText('Grants & Funding'));
      expect(screen.getAllByText('Infrastructure Defense').length).toBeGreaterThan(0);

      // Navigate to Governance & SIGs
      fireEvent.click(screen.getByText('Governance & SIGs'));
      expect(screen.getAllByText('Infrastructure Protection').length).toBeGreaterThan(0);

      // Navigate to Academy & Tools
      fireEvent.click(screen.getByText('Academy & Tools'));
      expect(screen.getByText(/CyberShield Academy & Sandboxes/i)).toBeDefined();

      // Navigate to Bounties & CVEs
      fireEvent.click(screen.getByText('Bounties & CVEs'));
      expect(screen.getByText(/Public Bug Bounties & Threat Intelligence/i)).toBeDefined();
    });
  });
});
