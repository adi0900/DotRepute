export function RepositorySection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">
        Repository Structure
      </h1>

      <div
        className={`border p-8 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <pre
          className={`text-sm font-mono overflow-x-auto ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
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
