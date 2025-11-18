import { TechList } from "@/components/resources/TechList";
import { Box, Database, Layers, FileCode } from "lucide-react";

export function TechStackSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Tech Stack</h1>

      <div className="grid md:grid-cols-2 gap-8">
        {/* Core Technologies */}
        <div
          className={`border p-8 space-y-6 ${
            theme === "light"
              ? "border-black/10 bg-white"
              : "border-white/5 bg-black/20"
          }`}
        >
          <div className="flex items-center gap-3">
            <div
              className={`border p-2 ${
                theme === "light"
                  ? "border-black/20 bg-white"
                  : "border-white/10 bg-black/40"
              }`}
            >
              <Box className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Core (Rust)</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              {
                name: "Rust",
                description: "Primary backend + scoring + contract language",
              },
              {
                name: "ink!",
                description: "Smart contract framework for Polkadot",
              },
              {
                name: "WASM",
                description: "Compiled Rust modules for web integration",
              },
              {
                name: "PolkadotJS API",
                description: "Lightweight blockchain interaction",
              },
              {
                name: "SubQuery",
                description: "Indexing governance/identity/staking events",
              },
            ]}
          />
        </div>

        {/* Frontend Technologies */}
        <div
          className={`border p-8 space-y-6 ${
            theme === "light"
              ? "border-black/10 bg-white"
              : "border-white/5 bg-black/20"
          }`}
        >
          <div className="flex items-center gap-3">
            <div
              className={`border p-2 ${
                theme === "light"
                  ? "border-black/20 bg-white"
                  : "border-white/10 bg-black/40"
              }`}
            >
              <Layers className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Frontend</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              {
                name: "Next.js",
                description: "React framework with app router",
              },
              { name: "React", description: "UI component library" },
              {
                name: "TailwindCSS",
                description: "Utility-first CSS framework",
              },
              {
                name: "ShadCN UI",
                description: "Pre-built accessible components",
              },
              {
                name: "TypeScript",
                description: "Type-safe JavaScript (minimal usage)",
              },
            ]}
          />
        </div>

        {/* Backend Technologies */}
        <div
          className={`border p-8 space-y-6 ${
            theme === "light"
              ? "border-black/10 bg-white"
              : "border-white/5 bg-black/20"
          }`}
        >
          <div className="flex items-center gap-3">
            <div
              className={`border p-2 ${
                theme === "light"
                  ? "border-black/20 bg-white"
                  : "border-white/10 bg-black/40"
              }`}
            >
              <Database className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Backend (Optional)</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              {
                name: "Rust microservices",
                description: "Preferred via Cargo.toml",
              },
              {
                name: "Node.js/Express",
                description: "Alternative for non-core services",
              },
              {
                name: "Fastify",
                description: "High-performance web framework",
              },
            ]}
          />
        </div>

        {/* Smart Contract Technologies */}
        <div
          className={`border p-8 space-y-6 ${
            theme === "light"
              ? "border-black/10 bg-white"
              : "border-white/5 bg-black/20"
          }`}
        >
          <div className="flex items-center gap-3">
            <div
              className={`border p-2 ${
                theme === "light"
                  ? "border-black/20 bg-white"
                  : "border-white/10 bg-black/40"
              }`}
            >
              <FileCode className="w-5 h-5" />
            </div>
            <h3 className="text-xl font-bold">Smart Contracts</h3>
          </div>

          <TechList
            theme={theme}
            items={[
              { name: "ink!", description: "Rust-based smart contracts" },
              {
                name: "WASM",
                description: "WebAssembly for contract execution",
              },
              {
                name: "Contracts Parachain",
                description: "Astar, Phala, Shiden deployment",
              },
            ]}
          />
        </div>
      </div>
    </div>
  );
}
