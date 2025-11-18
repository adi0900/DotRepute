export function ScoreComponent({
  theme,
  title,
  description,
  factors,
}: {
  theme: "light" | "dark";
  title: string;
  description: string;
  factors: string[];
}) {
  return (
    <div
      className={`border p-6 space-y-4 ${
        theme === "light"
          ? "border-black/10 bg-white"
          : "border-white/5 bg-black/20"
      }`}
    >
      <div>
        <h3 className="font-bold text-xl mb-2">{title}</h3>
        <p
          className={`text-sm ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
          {description}
        </p>
      </div>
      <div>
        <div
          className={`text-xs uppercase tracking-wider font-mono mb-2 ${
            theme === "light" ? "text-gray-600" : "text-gray-500"
          }`}
        >
          Factors
        </div>
        <ul
          className={`space-y-1 text-sm ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
          {factors.map((factor, i) => (
            <li key={i}>• {factor}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}
