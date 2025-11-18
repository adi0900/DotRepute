import { PhaseCard } from "@/components/docs/PhaseCard";

// Development Section
export function DevelopmentSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Development</h1>

      <div
        className={`border p-8 space-y-6 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold">Development Phases</h2>

        <PhaseCard
          theme={theme}
          phase="Phase 1"
          title="Rust Scoring Engine"
          items={[
            "Identity parsing logic",
            "Staking weight calculations",
            "Governance score algorithms",
            "WASM build configuration",
          ]}
        />

        <PhaseCard
          theme={theme}
          phase="Phase 2"
          title="SubQuery Integration"
          items={[
            "Event indexing setup",
            "Map on-chain activity to scoring",
            "Time-weighted calculations",
            "GraphQL schema design",
          ]}
        />

        <PhaseCard
          theme={theme}
          phase="Phase 3"
          title="ink! Contract (Optional)"
          items={[
            "On-chain reputation storage",
            "Verification functions",
            "Event emission system",
            "Contract testing suite",
          ]}
        />

        <PhaseCard
          theme={theme}
          phase="Phase 4"
          title="Dashboard & REST API"
          items={[
            "Rust or TypeScript backend",
            "Next.js UI implementation",
            "Real-time data display",
            "User authentication system",
          ]}
        />
      </div>

      <div
        className={`border p-8 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold mb-4">Repository Structure</h2>
        <pre
          className={`text-sm font-mono overflow-x-auto ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
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
