import { FeatureCard } from "@/components/resources/FeatureCard";

// Features Section
export function FeaturesSection({ theme }: { theme: "light" | "dark" }) {
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
