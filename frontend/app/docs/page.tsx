/**
 * DotRepute Documentation Page
 * Venture-style Documentation
 */

'use client';

import { useEffect, useState } from 'react';
import { VentureNavbar } from '@/components/venture-navbar';
import {
  FileText,
  Code,
  Database,
  Activity,
  Shield,
  GitBranch,
  Layers,
  Award,
  ChevronRight
} from 'lucide-react';

export default function DocsPage() {
  const [theme, setTheme] = useState<'light' | 'dark'>('dark');
  const [mounted, setMounted] = useState(false);
  const [activeSection, setActiveSection] = useState<string>('overview');

  useEffect(() => {
    setMounted(true);
    const savedTheme = localStorage.getItem('theme') as 'light' | 'dark' | null;
    const initialTheme = savedTheme || 'dark';
    setTheme(initialTheme);
    document.documentElement.classList.toggle('dark', initialTheme === 'dark');
  }, []);

  const toggleTheme = () => {
    const newTheme = theme === 'dark' ? 'light' : 'dark';
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    document.documentElement.classList.toggle('dark', newTheme === 'dark');
  };

  if (!mounted) return null;

  const sections = [
    { id: 'overview', label: 'Overview', icon: <FileText className="w-4 h-4" /> },
    { id: 'architecture', label: 'Architecture', icon: <Layers className="w-4 h-4" /> },
    { id: 'scoring', label: 'Scoring Model', icon: <Award className="w-4 h-4" /> },
    { id: 'data-sources', label: 'Data Sources', icon: <Database className="w-4 h-4" /> },
    { id: 'development', label: 'Development', icon: <Code className="w-4 h-4" /> },
  ];

  return (
    <div className={`relative min-h-screen transition-colors duration-300 ${
      theme === 'light' ? 'bg-white text-black' : 'bg-black text-white'
    }`}>
      <VentureNavbar theme={theme} onToggleTheme={toggleTheme} currentPath="/docs" />

      <main className="pt-24 min-h-screen">
        <div className="flex">
          {/* Sidebar Navigation */}
          <aside className={`w-64 border-r min-h-screen sticky top-24 ${
            theme === 'light' ? 'border-black/10' : 'border-white/5'
          }`}>
            <div className="p-6 space-y-2">
              <div className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}>
                Documentation
              </div>
              {sections.map((section) => (
                <button
                  key={section.id}
                  onClick={() => setActiveSection(section.id)}
                  className={`w-full flex items-center gap-3 px-4 py-3 text-sm transition-colors border ${
                    activeSection === section.id
                      ? theme === 'light'
                        ? 'border-black/40 bg-black/10 text-black'
                        : 'border-white/30 bg-white/10 text-white'
                      : theme === 'light'
                        ? 'border-black/10 hover:bg-black/5 text-gray-700'
                        : 'border-white/5 hover:bg-white/5 text-gray-400'
                  }`}
                >
                  {section.icon}
                  {section.label}
                </button>
              ))}
            </div>
          </aside>

          {/* Main Content */}
          <div className="flex-1 px-12 py-12 flex justify-center">
            <div className="max-w-4xl w-full">
              {activeSection === 'overview' && <OverviewSection theme={theme} />}
              {activeSection === 'architecture' && <ArchitectureSection theme={theme} />}
              {activeSection === 'scoring' && <ScoringSection theme={theme} />}
              {activeSection === 'data-sources' && <DataSourcesSection theme={theme} />}
              {activeSection === 'development' && <DevelopmentSection theme={theme} />}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

// Overview Section
function OverviewSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-5xl font-bold tracking-tight mb-4">Documentation</h1>
        <p className={`text-lg ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
          Complete guide to DotRepute's architecture and implementation
        </p>
      </div>

      <div className={`border p-8 space-y-4 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold">What is DotRepute?</h2>
        <p className={theme === 'light' ? 'text-gray-700' : 'text-gray-400'}>
          DotRepute is a <strong>Rust-powered Contributor Reputation System (CRS)</strong> designed
          for the Polkadot ecosystem. It aggregates identity, governance, staking, and activity
          signals across the network to generate a transparent, verifiable, and user-centric
          reputation score.
        </p>
      </div>

      <div className={`border p-8 space-y-4 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold">Key Philosophy</h2>
        <p className={theme === 'light' ? 'text-gray-700' : 'text-gray-400'}>
          Built <strong>without creating a parachain</strong> — instead, it layers a Rust-based
          scoring + contract system over existing parachains and Polkadot APIs.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        <FeatureCard
          theme={theme}
          icon={<Shield className="w-6 h-6" />}
          title="Rust-First Architecture"
          description="Core scoring logic implemented in Rust for performance and reliability"
        />
        <FeatureCard
          theme={theme}
          icon={<Activity className="w-6 h-6" />}
          title="Real-time Indexing"
          description="SubQuery integration for live blockchain data processing"
        />
        <FeatureCard
          theme={theme}
          icon={<Code className="w-6 h-6" />}
          title="ink! Smart Contracts"
          description="Optional WASM-based contracts for on-chain reputation storage"
        />
        <FeatureCard
          theme={theme}
          icon={<GitBranch className="w-6 h-6" />}
          title="Modular Design"
          description="Composable components that work with existing parachains"
        />
      </div>
    </div>
  );
}

// Architecture Section
function ArchitectureSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Architecture</h1>

      <div className={`border p-8 space-y-6 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold">System Components</h2>

        <div className="space-y-4">
          <ArchitectureItem
            theme={theme}
            title="Rust Scoring Engine"
            description="Core reputation calculation logic implemented as standalone Rust crate"
            items={[
              'Fully testable modules',
              'WASM-compilable',
              'Can integrate into ink! contracts'
            ]}
          />

          <ArchitectureItem
            theme={theme}
            title="SubQuery Indexer"
            description="Indexes on-chain events for scoring input"
            items={[
              'Governance participation tracking',
              'Identity verification events',
              'Staking behavior analysis'
            ]}
          />

          <ArchitectureItem
            theme={theme}
            title="Optional ink! Contract Layer"
            description="On-chain reputation storage and verification"
            items={[
              'Store reputation values',
              'Proof-of-reputation events',
              'User verification functions'
            ]}
          />

          <ArchitectureItem
            theme={theme}
            title="Frontend Dashboard"
            description="Next.js-based user interface"
            items={[
              'PolkadotJS API integration',
              'Real-time reputation display',
              'Interactive data visualization'
            ]}
          />
        </div>
      </div>

      <div className={`border p-8 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold mb-4">Tech Stack</h2>
        <div className="grid md:grid-cols-2 gap-6">
          <div>
            <h3 className={`text-xs uppercase tracking-wider font-mono mb-3 ${
              theme === 'light' ? 'text-gray-600' : 'text-gray-500'
            }`}>
              Core (Rust)
            </h3>
            <ul className={`space-y-2 text-sm ${
              theme === 'light' ? 'text-gray-700' : 'text-gray-400'
            }`}>
              <li>• Rust — backend + scoring</li>
              <li>• ink! — smart contracts</li>
              <li>• WASM — compiled modules</li>
              <li>• SubQuery — indexing</li>
            </ul>
          </div>
          <div>
            <h3 className={`text-xs uppercase tracking-wider font-mono mb-3 ${
              theme === 'light' ? 'text-gray-600' : 'text-gray-500'
            }`}>
              Frontend
            </h3>
            <ul className={`space-y-2 text-sm ${
              theme === 'light' ? 'text-gray-700' : 'text-gray-400'
            }`}>
              <li>• Next.js / React</li>
              <li>• TailwindCSS</li>
              <li>• TypeScript</li>
              <li>• PolkadotJS API</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

// Scoring Section
function ScoringSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Scoring Model</h1>

      <div className={`border p-8 space-y-6 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold">Reputation Formula</h2>

        <div className={`border p-6 font-mono text-sm ${
          theme === 'light'
            ? 'border-black/20 bg-black/5 text-black'
            : 'border-white/10 bg-white/5 text-white'
        }`}>
          crs_score = (identity_score * 0.25)<br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+ (governance_score * 0.25)<br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+ (staking_score * 0.20)<br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+ (activity_score * 0.20)<br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+ (dev_score * 0.10)
        </div>

        <p className={theme === 'light' ? 'text-gray-700' : 'text-gray-400'}>
          The scoring engine is implemented as a modular Rust crate that can be compiled to WASM
          and reused in backend services or smart contracts.
        </p>
      </div>

      <div className="space-y-6">
        <ScoreComponent
          theme={theme}
          title="Identity Score (25%)"
          description="Measures on-chain identity verification and trustworthiness"
          factors={[
            'Identity pallet registration',
            'Verification by registrars',
            'Account age and history',
            'Display name and social links'
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Governance Score (25%)"
          description="Tracks participation in Polkadot's OpenGov system"
          factors={[
            'Voting frequency and consistency',
            'Proposal submission quality',
            'Delegation relationships',
            'Council/technical committee participation'
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Staking Score (20%)"
          description="Evaluates staking behavior and network commitment"
          factors={[
            'Total staked amount',
            'Validator selection quality',
            'Nomination duration',
            'Reward claiming patterns'
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Activity Score (20%)"
          description="Measures overall network engagement"
          factors={[
            'Transaction frequency',
            'Cross-chain activity',
            'Smart contract interactions',
            'Time-weighted engagement'
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Developer Score (10%)"
          description="Optional: Tracks open-source contributions"
          factors={[
            'GitHub activity (optional)',
            'Code commits to ecosystem projects',
            'Documentation contributions',
            'Community support activity'
          ]}
        />
      </div>
    </div>
  );
}

// Data Sources Section
function DataSourcesSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Data Sources</h1>

      <div className={`border overflow-hidden ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <table className="w-full">
          <thead className={`border-b ${
            theme === 'light' ? 'border-black/10' : 'border-white/5'
          }`}>
            <tr>
              <th className={`text-left px-6 py-4 text-xs uppercase tracking-wider font-mono ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}>
                Domain
              </th>
              <th className={`text-left px-6 py-4 text-xs uppercase tracking-wider font-mono ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}>
                On-Chain Source
              </th>
              <th className={`text-left px-6 py-4 text-xs uppercase tracking-wider font-mono ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}>
                Purpose
              </th>
            </tr>
          </thead>
          <tbody>
            <DataRow
              theme={theme}
              domain="Identity"
              source="Identity Pallet"
              purpose="Real user trust verification"
            />
            <DataRow
              theme={theme}
              domain="Governance"
              source="OpenGov Referenda"
              purpose="Civic participation tracking"
            />
            <DataRow
              theme={theme}
              domain="Staking"
              source="Staking Pallet"
              purpose="Skin-in-the-game measurement"
            />
            <DataRow
              theme={theme}
              domain="Activity"
              source="Extrinsics"
              purpose="Frequency & engagement analysis"
            />
            <DataRow
              theme={theme}
              domain="Dev Signals"
              source="GitHub (optional)"
              purpose="Open-source contributions"
            />
          </tbody>
        </table>
      </div>

      <div className={`border p-8 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold mb-4">SubQuery Integration</h2>
        <p className={`mb-4 ${theme === 'light' ? 'text-gray-700' : 'text-gray-400'}`}>
          DotRepute uses SubQuery to index and process on-chain events in real-time:
        </p>
        <ul className={`space-y-2 text-sm ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
          <li>• Maps on-chain activity to Rust scoring engine</li>
          <li>• Time-weighted scoring calculations</li>
          <li>• GraphQL API for frontend queries</li>
          <li>• Efficient data aggregation and caching</li>
        </ul>
      </div>
    </div>
  );
}

// Development Section
function DevelopmentSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Development</h1>

      <div className={`border p-8 space-y-6 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold">Development Phases</h2>

        <PhaseCard
          theme={theme}
          phase="Phase 1"
          title="Rust Scoring Engine"
          items={[
            'Identity parsing logic',
            'Staking weight calculations',
            'Governance score algorithms',
            'WASM build configuration'
          ]}
        />

        <PhaseCard
          theme={theme}
          phase="Phase 2"
          title="SubQuery Integration"
          items={[
            'Event indexing setup',
            'Map on-chain activity to scoring',
            'Time-weighted calculations',
            'GraphQL schema design'
          ]}
        />

        <PhaseCard
          theme={theme}
          phase="Phase 3"
          title="ink! Contract (Optional)"
          items={[
            'On-chain reputation storage',
            'Verification functions',
            'Event emission system',
            'Contract testing suite'
          ]}
        />

        <PhaseCard
          theme={theme}
          phase="Phase 4"
          title="Dashboard & REST API"
          items={[
            'Rust or TypeScript backend',
            'Next.js UI implementation',
            'Real-time data display',
            'User authentication system'
          ]}
        />
      </div>

      <div className={`border p-8 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <h2 className="text-2xl font-bold mb-4">Repository Structure</h2>
        <pre className={`text-sm font-mono overflow-x-auto ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
{`crs-dapp/
├── contracts/          # ink! smart contracts
├── frontend/           # Next.js application
├── indexer/            # SubQuery indexer
├── backend/            # Rust microservices
├── docs/              # Documentation
├── scripts/           # Utility scripts
└── tests/             # Test suites`}
        </pre>
      </div>
    </div>
  );
}

// Helper Components
function FeatureCard({ theme, icon, title, description }: {
  theme: 'light' | 'dark';
  icon: React.ReactNode;
  title: string;
  description: string;
}) {
  return (
    <div className={`border p-6 space-y-3 ${
      theme === 'light'
        ? 'border-black/10 bg-white'
        : 'border-white/10 bg-black/40'
    }`}>
      <div className={`inline-flex border p-2 ${
        theme === 'light'
          ? 'border-black/20 bg-white'
          : 'border-white/10 bg-black/40'
      }`}>
        {icon}
      </div>
      <h3 className="font-bold">{title}</h3>
      <p className={`text-sm ${
        theme === 'light' ? 'text-gray-600' : 'text-gray-500'
      }`}>
        {description}
      </p>
    </div>
  );
}

function ArchitectureItem({ theme, title, description, items }: {
  theme: 'light' | 'dark';
  title: string;
  description: string;
  items: string[];
}) {
  return (
    <div className={`border p-6 space-y-3 ${
      theme === 'light'
        ? 'border-black/10 bg-white'
        : 'border-white/10 bg-black/40'
    }`}>
      <h3 className="font-bold text-lg">{title}</h3>
      <p className={`text-sm ${
        theme === 'light' ? 'text-gray-700' : 'text-gray-400'
      }`}>
        {description}
      </p>
      <ul className={`space-y-1 text-sm ${
        theme === 'light' ? 'text-gray-600' : 'text-gray-500'
      }`}>
        {items.map((item, i) => (
          <li key={i}>• {item}</li>
        ))}
      </ul>
    </div>
  );
}

function ScoreComponent({ theme, title, description, factors }: {
  theme: 'light' | 'dark';
  title: string;
  description: string;
  factors: string[];
}) {
  return (
    <div className={`border p-6 space-y-4 ${
      theme === 'light'
        ? 'border-black/10 bg-white'
        : 'border-white/5 bg-black/20'
    }`}>
      <div>
        <h3 className="font-bold text-xl mb-2">{title}</h3>
        <p className={`text-sm ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
          {description}
        </p>
      </div>
      <div>
        <div className={`text-xs uppercase tracking-wider font-mono mb-2 ${
          theme === 'light' ? 'text-gray-600' : 'text-gray-500'
        }`}>
          Factors
        </div>
        <ul className={`space-y-1 text-sm ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
          {factors.map((factor, i) => (
            <li key={i}>• {factor}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}

function DataRow({ theme, domain, source, purpose }: {
  theme: 'light' | 'dark';
  domain: string;
  source: string;
  purpose: string;
}) {
  return (
    <tr className={`border-b ${
      theme === 'light' ? 'border-black/10' : 'border-white/5'
    }`}>
      <td className={`px-6 py-4 font-medium ${
        theme === 'light' ? 'text-black' : 'text-white'
      }`}>
        {domain}
      </td>
      <td className={`px-6 py-4 ${
        theme === 'light' ? 'text-gray-700' : 'text-gray-400'
      }`}>
        {source}
      </td>
      <td className={`px-6 py-4 ${
        theme === 'light' ? 'text-gray-700' : 'text-gray-400'
      }`}>
        {purpose}
      </td>
    </tr>
  );
}

function PhaseCard({ theme, phase, title, items }: {
  theme: 'light' | 'dark';
  phase: string;
  title: string;
  items: string[];
}) {
  return (
    <div className={`border p-6 space-y-3 ${
      theme === 'light'
        ? 'border-black/10 bg-white'
        : 'border-white/10 bg-black/40'
    }`}>
      <div className={`inline-flex border px-3 py-1 text-xs uppercase tracking-wider font-mono ${
        theme === 'light'
          ? 'border-black/20 bg-black/5 text-gray-600'
          : 'border-white/10 bg-white/5 text-gray-400'
      }`}>
        {phase}
      </div>
      <h3 className="font-bold text-lg">{title}</h3>
      <ul className={`space-y-1 text-sm ${
        theme === 'light' ? 'text-gray-700' : 'text-gray-400'
      }`}>
        {items.map((item, i) => (
          <li key={i}>• {item}</li>
        ))}
      </ul>
    </div>
  );
}
