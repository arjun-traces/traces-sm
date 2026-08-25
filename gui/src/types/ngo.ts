export interface GrantProposal {
  id: string;
  title: string;
  applicant: string;
  category: 'Infrastructure Defense' | 'Cryptographic Research' | 'Defensive Tooling' | 'Auditing & Verification';
  amountRequested: string;
  status: 'Active Round' | 'Milestone 2/3' | 'Completed' | 'Under Review';
  quadraticContributors: number;
  tags: string[];
  description: string;
  impactScore: number;
}

export interface VulnerabilityBounty {
  id: string;
  cveId?: string;
  title: string;
  targetProject: string;
  severity: 'Critical' | 'High' | 'Medium' | 'Low';
  rewardPool: string;
  status: 'Open Bounty' | 'Patched' | 'Under Audit';
  disclosedDate: string;
  reporter: string;
}

export interface GovProposal {
  id: string;
  title: string;
  sigGroup: 'SIG-01 Infrastructure' | 'SIG-02 Crypto & Privacy' | 'SIG-03 Rapid Defense' | 'SIG-04 Education';
  author: string;
  status: 'Active Voting' | 'Passed' | 'Executed';
  votesFor: number;
  votesAgainst: number;
  endsInDays: number;
  summary: string;
}

export interface ThreatAdvisory {
  id: string;
  timestamp: string;
  severity: 'Critical' | 'High' | 'Medium';
  title: string;
  affectedComponent: string;
  mitigationStatus: 'Patch Released' | 'Workaround Available' | 'Investigation';
}

export interface TreasuryAllocation {
  name: string;
  amountUsd: number;
  percentage: number;
  color: string;
}
