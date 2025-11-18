import { ArchitectureItem } from "@/components/docs/ArchitectureItem";

export function ArchitectureSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Architecture</h1>

      <div
        className={`border p-8 space-y-6 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold">System Components</h2>

        <div className="space-y-4">
          <ArchitectureItem
            theme={theme}
            title="Rust Scoring Engine"
            description="Core reputation calculation logic implemented as standalone Rust crate"
            items={[
              "Fully testable modules",
              "WASM-compilable",
              "Can integrate into ink! contracts",
            ]}
          />

          <ArchitectureItem
            theme={theme}
            title="SubQuery Indexer"
            description="Indexes on-chain events for scoring input"
            items={[
              "Governance participation tracking",
              "Identity verification events",
              "Staking behavior analysis",
            ]}
          />

          <ArchitectureItem
            theme={theme}
            title="Optional ink! Contract Layer"
            description="On-chain reputation storage and verification"
            items={[
              "Store reputation values",
              "Proof-of-reputation events",
              "User verification functions",
            ]}
          />

          <ArchitectureItem
            theme={theme}
            title="Frontend Dashboard"
            description="Next.js-based user interface"
            items={[
              "PolkadotJS API integration",
              "Real-time reputation display",
              "Interactive data visualization",
            ]}
          />
        </div>
      </div>

      <div
        className={`border p-8 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold mb-4">Tech Stack</h2>
        <div className="grid md:grid-cols-2 gap-6">
          <div>
            <h3
              className={`text-xs uppercase tracking-wider font-mono mb-3 ${
                theme === "light" ? "text-gray-600" : "text-gray-500"
              }`}
            >
              Core (Rust)
            </h3>
            <ul
              className={`space-y-2 text-sm ${
                theme === "light" ? "text-gray-700" : "text-gray-400"
              }`}
            >
              <li>• Rust — backend + scoring</li>
              <li>• ink! — smart contracts</li>
              <li>• WASM — compiled modules</li>
              <li>• SubQuery — indexing</li>
            </ul>
          </div>
          <div>
            <h3
              className={`text-xs uppercase tracking-wider font-mono mb-3 ${
                theme === "light" ? "text-gray-600" : "text-gray-500"
              }`}
            >
              Frontend
            </h3>
            <ul
              className={`space-y-2 text-sm ${
                theme === "light" ? "text-gray-700" : "text-gray-400"
              }`}
            >
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
