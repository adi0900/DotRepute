import { FeatureCard } from "@/components/docs/FeatureCard";
import { Activity, Code, GitBranch, Shield } from "lucide-react";

// Overview Section
export function OverviewSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-5xl font-bold tracking-tight mb-4">
          Documentation
        </h1>
        <p
          className={`text-lg ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
          Complete guide to DotRepute's architecture and implementation
        </p>
      </div>

      <div
        className={`border p-8 space-y-4 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold">What is DotRepute?</h2>
        <p className={theme === "light" ? "text-gray-700" : "text-gray-400"}>
          DotRepute is a{" "}
          <strong>Rust-powered Contributor Reputation System (CRS)</strong>{" "}
          designed for the Polkadot ecosystem. It aggregates identity,
          governance, staking, and activity signals across the network to
          generate a transparent, verifiable, and user-centric reputation score.
        </p>
      </div>

      <div
        className={`border p-8 space-y-4 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold">Key Philosophy</h2>
        <p className={theme === "light" ? "text-gray-700" : "text-gray-400"}>
          Built <strong>without creating a parachain</strong> — instead, it
          layers a Rust-based scoring + contract system over existing parachains
          and Polkadot APIs.
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
