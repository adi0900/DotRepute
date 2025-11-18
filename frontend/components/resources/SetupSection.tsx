// Setup Section

import { DevSetupCard } from "@/components/resources/DevSetupCard";

export function SetupSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Development Setup</h1>

      <div className="space-y-6">
        <DevSetupCard
          theme={theme}
          title="1. Rust Backend"
          description="Start the Rust backend services"
          commands={["cd backend", "cargo run"]}
        />

        <DevSetupCard
          theme={theme}
          title="2. ink! Contract (Optional)"
          description="Build and deploy smart contracts"
          commands={["cd contracts/crs_contract", "./build.sh"]}
        />

        <DevSetupCard
          theme={theme}
          title="3. SubQuery Indexer"
          description="Set up blockchain data indexing"
          commands={[
            "cd indexer",
            "npm install",
            "subql codegen",
            "subql build",
            "subql query",
          ]}
        />

        <DevSetupCard
          theme={theme}
          title="4. Frontend"
          description="Launch the Next.js development server"
          commands={["cd frontend", "npm install", "npm run dev"]}
        />
      </div>
    </div>
  );
}
