export function PhaseCard({
  theme,
  phase,
  title,
  items,
}: {
  theme: "light" | "dark";
  phase: string;
  title: string;
  items: string[];
}) {
  return (
    <div
      className={`border p-6 space-y-3 ${
        theme === "light"
          ? "border-black/10 bg-white"
          : "border-white/10 bg-black/40"
      }`}
    >
      <div
        className={`inline-flex border px-3 py-1 text-xs uppercase tracking-wider font-mono ${
          theme === "light"
            ? "border-black/20 bg-black/5 text-gray-600"
            : "border-white/10 bg-white/5 text-gray-400"
        }`}
      >
        {phase}
      </div>
      <h3 className="font-bold text-lg">{title}</h3>
      <ul
        className={`space-y-1 text-sm ${
          theme === "light" ? "text-gray-700" : "text-gray-400"
        }`}
      >
        {items.map((item, i) => (
          <li key={i}>• {item}</li>
        ))}
      </ul>
    </div>
  );
}
