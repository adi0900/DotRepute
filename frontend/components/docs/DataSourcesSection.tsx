import { DataRow } from "@/components/docs/DataRow";

export function DataSourcesSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Data Sources</h1>

      <div
        className={`border overflow-hidden ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <table className="w-full">
          <thead
            className={`border-b ${
              theme === "light" ? "border-black/10" : "border-white/5"
            }`}
          >
            <tr>
              <th
                className={`text-left px-6 py-4 text-xs uppercase tracking-wider font-mono ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
                Domain
              </th>
              <th
                className={`text-left px-6 py-4 text-xs uppercase tracking-wider font-mono ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
                On-Chain Source
              </th>
              <th
                className={`text-left px-6 py-4 text-xs uppercase tracking-wider font-mono ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
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

      <div
        className={`border p-8 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <h2 className="text-2xl font-bold mb-4">SubQuery Integration</h2>
        <p
          className={`mb-4 ${theme === "light" ? "text-gray-700" : "text-gray-400"}`}
        >
          DotRepute uses SubQuery to index and process on-chain events in
          real-time:
        </p>
        <ul
          className={`space-y-2 text-sm ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
          <li>• Maps on-chain activity to Rust scoring engine</li>
          <li>• Time-weighted scoring calculations</li>
          <li>• GraphQL API for frontend queries</li>
          <li>• Efficient data aggregation and caching</li>
        </ul>
      </div>
    </div>
  );
}
