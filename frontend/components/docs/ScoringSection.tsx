import { ScoreComponent } from "@/components/docs/ScoreComponent";

// Scoring Section
export function ScoringSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Scoring Model</h1>

      <div
        className={`border p-8 space-y-6 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold">Reputation Formula</h2>

        <div
          className={`border p-6 font-mono text-sm ${
            theme === "light"
              ? "border-black/20 bg-black/5 text-black"
              : "border-white/10 bg-white/5 text-white"
          }`}
        >
          crs_score = (identity_score * 0.25)
          <br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+
          (governance_score * 0.25)
          <br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+
          (staking_score * 0.20)
          <br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+
          (activity_score * 0.20)
          <br />
          &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;+
          (dev_score * 0.10)
        </div>

        <p className={theme === "light" ? "text-gray-700" : "text-gray-400"}>
          The scoring engine is implemented as a modular Rust crate that can be
          compiled to WASM and reused in backend services or smart contracts.
        </p>
      </div>

      <div className="space-y-6">
        <ScoreComponent
          theme={theme}
          title="Identity Score (25%)"
          description="Measures on-chain identity verification and trustworthiness"
          factors={[
            "Identity pallet registration",
            "Verification by registrars",
            "Account age and history",
            "Display name and social links",
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Governance Score (25%)"
          description="Tracks participation in Polkadot's OpenGov system"
          factors={[
            "Voting frequency and consistency",
            "Proposal submission quality",
            "Delegation relationships",
            "Council/technical committee participation",
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Staking Score (20%)"
          description="Evaluates staking behavior and network commitment"
          factors={[
            "Total staked amount",
            "Validator selection quality",
            "Nomination duration",
            "Reward claiming patterns",
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Activity Score (20%)"
          description="Measures overall network engagement"
          factors={[
            "Transaction frequency",
            "Cross-chain activity",
            "Smart contract interactions",
            "Time-weighted engagement",
          ]}
        />

        <ScoreComponent
          theme={theme}
          title="Developer Score (10%)"
          description="Optional: Tracks open-source contributions"
          factors={[
            "GitHub activity (optional)",
            "Code commits to ecosystem projects",
            "Documentation contributions",
            "Community support activity",
          ]}
        />
      </div>
    </div>
  );
}
