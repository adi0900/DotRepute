/**
 * DotRepute Resources Page
 * Venture-style Resources and Tech Stack
 */

'use client';

import { useEffect, useState } from 'react';
import { VentureNavbar } from '@/components/venture-navbar';
import {
  Code,
  Box,
  Database,
  Layers,
  Terminal,
  FileCode,
  Package,
  GitBranch,
  ExternalLink,
  BookOpen,
  Wrench
} from 'lucide-react';

export default function ResourcesPage() {
  const [theme, setTheme] = useState<'light' | 'dark'>('dark');
  const [mounted, setMounted] = useState(false);
  const [activeSection, setActiveSection] = useState<string>('tech-stack');

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
    { id: 'tech-stack', label: 'Tech Stack', icon: <Code className="w-4 h-4" /> },
    { id: 'repository', label: 'Repository', icon: <GitBranch className="w-4 h-4" /> },
    { id: 'setup', label: 'Development Setup', icon: <Terminal className="w-4 h-4" /> },
    { id: 'features', label: 'Key Features', icon: <Package className="w-4 h-4" /> },
    { id: 'external', label: 'External Resources', icon: <BookOpen className="w-4 h-4" /> },
    { id: 'tools', label: 'Development Tools', icon: <Wrench className="w-4 h-4" /> },
  ];

  return (
    <div className={`relative min-h-screen transition-colors duration-300 ${
      theme === 'light' ? 'bg-white text-black' : 'bg-black text-white'
    }`}>
      <VentureNavbar theme={theme} onToggleTheme={toggleTheme} currentPath="/resources" />

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
                Resources
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
              {activeSection === 'tech-stack' && <TechStackSection theme={theme} />}
              {activeSection === 'repository' && <RepositorySection theme={theme} />}
              {activeSection === 'setup' && <SetupSection theme={theme} />}
              {activeSection === 'features' && <FeaturesSection theme={theme} />}
              {activeSection === 'external' && <ExternalSection theme={theme} />}
              {activeSection === 'tools' && <ToolsSection theme={theme} />}
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

// Tech Stack Section
function TechStackSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Tech Stack</h1>

      <div className="grid md:grid-cols-2 gap-8">
        {/* Core Technologies */}
        <div className={`border p-8 space-y-6 ${
          theme === 'light'
            ? 'border-black/10 bg-white'
            : 'border-white/5 bg-black/20'
        }`}>
          <div className="flex items-center gap-3">
            <div className={`border p-2 ${
              theme === 'light'
                ? 'border-black/20 bg-white'
                : 'border-white/10 bg-black/40'
            }`}>
              <Box className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Core (Rust)</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              { name: 'Rust', description: 'Primary backend + scoring + contract language' },
              { name: 'ink!', description: 'Smart contract framework for Polkadot' },
              { name: 'WASM', description: 'Compiled Rust modules for web integration' },
              { name: 'PolkadotJS API', description: 'Lightweight blockchain interaction' },
              { name: 'SubQuery', description: 'Indexing governance/identity/staking events' }
            ]}
          />
        </div>

        {/* Frontend Technologies */}
        <div className={`border p-8 space-y-6 ${
          theme === 'light'
            ? 'border-black/10 bg-white'
            : 'border-white/5 bg-black/20'
        }`}>
          <div className="flex items-center gap-3">
            <div className={`border p-2 ${
              theme === 'light'
                ? 'border-black/20 bg-white'
                : 'border-white/10 bg-black/40'
            }`}>
              <Layers className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Frontend</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              { name: 'Next.js', description: 'React framework with app router' },
              { name: 'React', description: 'UI component library' },
              { name: 'TailwindCSS', description: 'Utility-first CSS framework' },
              { name: 'ShadCN UI', description: 'Pre-built accessible components' },
              { name: 'TypeScript', description: 'Type-safe JavaScript (minimal usage)' }
            ]}
          />
        </div>

        {/* Backend Technologies */}
        <div className={`border p-8 space-y-6 ${
          theme === 'light'
            ? 'border-black/10 bg-white'
            : 'border-white/5 bg-black/20'
        }`}>
          <div className="flex items-center gap-3">
            <div className={`border p-2 ${
              theme === 'light'
                ? 'border-black/20 bg-white'
                : 'border-white/10 bg-black/40'
            }`}>
              <Database className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Backend (Optional)</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              { name: 'Rust microservices', description: 'Preferred via Cargo.toml' },
              { name: 'Node.js/Express', description: 'Alternative for non-core services' },
              { name: 'Fastify', description: 'High-performance web framework' }
            ]}
          />
        </div>

        {/* Smart Contract Technologies */}
        <div className={`border p-8 space-y-6 ${
          theme === 'light'
            ? 'border-black/10 bg-white'
            : 'border-white/5 bg-black/20'
        }`}>
          <div className="flex items-center gap-3">
            <div className={`border p-2 ${
              theme === 'light'
                ? 'border-black/20 bg-white'
                : 'border-white/10 bg-black/40'
            }`}>
              <FileCode className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Smart Contracts</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              { name: 'ink!', description: 'Rust-based smart contracts' },
              { name: 'WASM', description: 'WebAssembly for contract execution' },
              { name: 'Contracts Parachain', description: 'Astar, Phala, Shiden deployment' }
            ]}
          />
        </div>
      </div>
    </div>
  );
}

// Repository Section
function RepositorySection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Repository Structure</h1>

      <div className={`border p-8 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <pre className={`text-sm font-mono overflow-x-auto ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
{`crs-dapp/
├── contracts/          # ink! smart contracts (Rust + WASM)
│   ├── Cargo.toml
│   └── crs_contract/
│       ├── lib.rs
│       ├── Cargo.toml
│       ├── build.sh
│       └── README.md
│
├── frontend/           # Next.js application
│   └── src/
│       ├── components/
│       ├── pages/ or app/
│       ├── lib/polkadot/
│       └── ...
│
├── indexer/            # SubQuery indexer
│   ├── subquery.yaml
│   ├── schema.graphql
│   └── src/
│
├── backend/            # Rust microservices (preferred)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── scoring/
│       ├── models/
│       └── api/
│
├── docs/              # Documentation
├── scripts/           # Utility scripts
├── tests/             # Test suites
└── .github/           # CI/CD workflows`}
        </pre>
      </div>
    </div>
  );
}

// Setup Section
function SetupSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Development Setup</h1>

      <div className="space-y-6">
        <DevSetupCard
          theme={theme}
          title="1. Rust Backend"
          description="Start the Rust backend services"
          commands={[
            'cd backend',
            'cargo run'
          ]}
        />

        <DevSetupCard
          theme={theme}
          title="2. ink! Contract (Optional)"
          description="Build and deploy smart contracts"
          commands={[
            'cd contracts/crs_contract',
            './build.sh'
          ]}
        />

        <DevSetupCard
          theme={theme}
          title="3. SubQuery Indexer"
          description="Set up blockchain data indexing"
          commands={[
            'cd indexer',
            'npm install',
            'subql codegen',
            'subql build',
            'subql query'
          ]}
        />

        <DevSetupCard
          theme={theme}
          title="4. Frontend"
          description="Launch the Next.js development server"
          commands={[
            'cd frontend',
            'npm install',
            'npm run dev'
          ]}
        />
      </div>
    </div>
  );
}

// Features Section
function FeaturesSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Key Features</h1>

      <div className="grid md:grid-cols-2 gap-6">
        <FeatureCard
          theme={theme}
          title="Rust-based Reputation Engine"
          description="Core scoring logic implemented in Rust as standalone crate, fully testable and WASM-compilable"
        />

        <FeatureCard
          theme={theme}
          title="Optional ink! Smart Contracts"
          description="Rust/ink! modules can store reputation values, proof-of-reputation events, and user verifications"
        />

        <FeatureCard
          theme={theme}
          title="SubQuery Indexing"
          description="Indexes governance, identity, and staking events for real-time scoring input"
        />

        <FeatureCard
          theme={theme}
          title="Rust-first Architecture"
          description="Uses Rust for scoring engine, data cleaning logic, and WASM-optimized utilities"
        />

        <FeatureCard
          theme={theme}
          title="React-based Dashboard"
          description="Lightweight frontend interacting with PolkadotJS API, Rust scoring engine, and optional contracts"
        />

        <FeatureCard
          theme={theme}
          title="Modular Design"
          description="Composable components that work with existing parachains without creating a new one"
        />
      </div>
    </div>
  );
}

// External Section
function ExternalSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">External Resources</h1>

      <div className="grid md:grid-cols-2 gap-6">
        <ResourceLink
          theme={theme}
          title="Rust Documentation"
          description="Official Rust programming language documentation"
          url="https://doc.rust-lang.org/"
        />

        <ResourceLink
          theme={theme}
          title="ink! Documentation"
          description="Smart contract framework for Polkadot"
          url="https://use.ink/"
        />

        <ResourceLink
          theme={theme}
          title="Polkadot Wiki"
          description="Comprehensive guide to Polkadot ecosystem"
          url="https://wiki.polkadot.network/"
        />

        <ResourceLink
          theme={theme}
          title="SubQuery Docs"
          description="Blockchain indexing and querying"
          url="https://academy.subquery.network/"
        />

        <ResourceLink
          theme={theme}
          title="Next.js Documentation"
          description="React framework for production"
          url="https://nextjs.org/docs"
        />

        <ResourceLink
          theme={theme}
          title="TailwindCSS"
          description="Utility-first CSS framework"
          url="https://tailwindcss.com/"
        />
      </div>
    </div>
  );
}

// Tools Section
function ToolsSection({ theme }: { theme: 'light' | 'dark' }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Development Tools</h1>

      <div className={`border p-8 ${
        theme === 'light'
          ? 'border-black/10 bg-white'
          : 'border-white/5 bg-black/20'
      }`}>
        <div className="grid md:grid-cols-3 gap-8">
          <div>
            <h3 className={`text-xs uppercase tracking-wider font-mono mb-3 ${
              theme === 'light' ? 'text-gray-600' : 'text-gray-500'
            }`}>
              Rust Tools
            </h3>
            <ul className={`space-y-2 text-sm ${
              theme === 'light' ? 'text-gray-700' : 'text-gray-400'
            }`}>
              <li>• cargo (package manager)</li>
              <li>• rustc (compiler)</li>
              <li>• rustfmt (formatter)</li>
              <li>• clippy (linter)</li>
            </ul>
          </div>

          <div>
            <h3 className={`text-xs uppercase tracking-wider font-mono mb-3 ${
              theme === 'light' ? 'text-gray-600' : 'text-gray-500'
            }`}>
              Contract Tools
            </h3>
            <ul className={`space-y-2 text-sm ${
              theme === 'light' ? 'text-gray-700' : 'text-gray-400'
            }`}>
              <li>• cargo-contract</li>
              <li>• substrate-contracts-node</li>
              <li>• polkadot.js apps</li>
            </ul>
          </div>

          <div>
            <h3 className={`text-xs uppercase tracking-wider font-mono mb-3 ${
              theme === 'light' ? 'text-gray-600' : 'text-gray-500'
            }`}>
              Frontend Tools
            </h3>
            <ul className={`space-y-2 text-sm ${
              theme === 'light' ? 'text-gray-700' : 'text-gray-400'
            }`}>
              <li>• npm/yarn (package managers)</li>
              <li>• TypeScript compiler</li>
              <li>• ESLint (linter)</li>
              <li>• Prettier (formatter)</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

// Helper Components
function TechList({ theme, items }: {
  theme: 'light' | 'dark';
  items: Array<{ name: string; description: string }>;
}) {
  return (
    <div className="space-y-4">
      {items.map((item, i) => (
        <div key={i} className="space-y-1">
          <div className="font-semibold">{item.name}</div>
          <div className={`text-sm ${
            theme === 'light' ? 'text-gray-600' : 'text-gray-500'
          }`}>
            {item.description}
          </div>
        </div>
      ))}
    </div>
  );
}

function DevSetupCard({ theme, title, description, commands }: {
  theme: 'light' | 'dark';
  title: string;
  description: string;
  commands: string[];
}) {
  return (
    <div className={`border p-8 space-y-4 ${
      theme === 'light'
        ? 'border-black/10 bg-white'
        : 'border-white/5 bg-black/20'
    }`}>
      <div>
        <h3 className="text-xl font-bold mb-2">{title}</h3>
        <p className={`text-sm ${
          theme === 'light' ? 'text-gray-700' : 'text-gray-400'
        }`}>
          {description}
        </p>
      </div>

      <div className={`border p-4 font-mono text-sm space-y-1 ${
        theme === 'light'
          ? 'border-black/20 bg-black/5 text-black'
          : 'border-white/10 bg-white/5 text-white'
      }`}>
        {commands.map((cmd, i) => (
          <div key={i}>$ {cmd}</div>
        ))}
      </div>
    </div>
  );
}

function FeatureCard({ theme, title, description }: {
  theme: 'light' | 'dark';
  title: string;
  description: string;
}) {
  return (
    <div className={`border p-6 space-y-3 ${
      theme === 'light'
        ? 'border-black/10 bg-white'
        : 'border-white/10 bg-black/40'
    }`}>
      <h3 className="font-bold text-lg">{title}</h3>
      <p className={`text-sm ${
        theme === 'light' ? 'text-gray-600' : 'text-gray-500'
      }`}>
        {description}
      </p>
    </div>
  );
}

function ResourceLink({ theme, title, description, url }: {
  theme: 'light' | 'dark';
  title: string;
  description: string;
  url: string;
}) {
  return (
    <a
      href={url}
      target="_blank"
      rel="noopener noreferrer"
      className={`border p-6 space-y-3 transition-colors group ${
        theme === 'light'
          ? 'border-black/10 hover:bg-black/5'
          : 'border-white/10 hover:bg-white/5'
      }`}
    >
      <div className="flex items-start justify-between">
        <h3 className="font-bold text-lg">{title}</h3>
        <ExternalLink className={`w-4 h-4 transition-transform group-hover:translate-x-1 group-hover:-translate-y-1 ${
          theme === 'light' ? 'text-gray-600' : 'text-gray-500'
        }`} />
      </div>
      <p className={`text-sm ${
        theme === 'light' ? 'text-gray-600' : 'text-gray-500'
      }`}>
        {description}
      </p>
    </a>
  );
}
