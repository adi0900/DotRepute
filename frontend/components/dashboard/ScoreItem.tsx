// Score Item Component
export function ScoreItem({
  icon,
  label,
  score,
  max,
  theme,
}: {
  icon: React.ReactNode;
  label: string;
  score: number;
  max: number;
  theme: "light" | "dark";
}) {
  return (
    <div
      className={`border p-3 ${
        theme === "light"
          ? "border-black/10 bg-gray-50"
          : "border-white/5 bg-black/20"
      }`}
    >
      <div className="flex items-center gap-2 mb-2">
        <div className={theme === "light" ? "text-gray-600" : "text-gray-500"}>
          {icon}
        </div>
        <span
          className={`text-xs uppercase tracking-wider font-mono ${
            theme === "light" ? "text-gray-600" : "text-gray-500"
          }`}
        >
          {label}
        </span>
      </div>
      <div className="flex items-baseline gap-1">
        <span className="text-xl font-bold">{score}</span>
        <span
          className={`text-sm ${theme === "light" ? "text-gray-600" : "text-gray-500"}`}
        >
          /{max}
        </span>
      </div>
    </div>
  );
}
