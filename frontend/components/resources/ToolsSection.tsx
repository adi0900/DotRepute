// Tools Section
export function ToolsSection({ theme }: { theme: "light" | "dark" }) {
  return (
    <div className="space-y-8">
      <h1 className="text-5xl font-bold tracking-tight">Development Tools</h1>

      <div
        className={`border p-8 ${
          theme === "light"
            ? "border-black/10 bg-white"
            : "border-white/5 bg-black/20"
        }`}
      >
        <div className="grid md:grid-cols-3 gap-8">
          <div>
            <h3
              className={`text-xs uppercase tracking-wider font-mono mb-3 ${
                theme === "light" ? "text-gray-600" : "text-gray-500"
              }`}
            >
              Rust Tools
            </h3>
            <ul
              className={`space-y-2 text-sm ${
                theme === "light" ? "text-gray-700" : "text-gray-400"
              }`}
            >
              <li>• cargo (package manager)</li>
              <li>• rustc (compiler)</li>
              <li>• rustfmt (formatter)</li>
              <li>• clippy (linter)</li>
            </ul>
          </div>

          <div>
            <h3
              className={`text-xs uppercase tracking-wider font-mono mb-3 ${
                theme === "light" ? "text-gray-600" : "text-gray-500"
              }`}
            >
              Contract Tools
            </h3>
            <ul
              className={`space-y-2 text-sm ${
                theme === "light" ? "text-gray-700" : "text-gray-400"
              }`}
            >
              <li>• cargo-contract</li>
              <li>• substrate-contracts-node</li>
              <li>• polkadot.js apps</li>
            </ul>
          </div>

          <div>
            <h3
              className={`text-xs uppercase tracking-wider font-mono mb-3 ${
                theme === "light" ? "text-gray-600" : "text-gray-500"
              }`}
            >
              Frontend Tools
            </h3>
            <ul
              className={`space-y-2 text-sm ${
                theme === "light" ? "text-gray-700" : "text-gray-400"
              }`}
            >
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
