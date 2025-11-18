// Quick Action Button Component
export function QuickActionButton({
  theme,
  label,
  onClick,
}: {
  theme: "light" | "dark";
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={`border px-3 py-1.5 text-xs uppercase tracking-wider font-medium transition-colors ${
        theme === "light"
          ? "border-black/20 hover:bg-black/5 text-gray-700 hover:text-black"
          : "border-white/10 hover:bg-white/5 text-gray-400 hover:text-white"
      }`}
    >
      {label}
    </button>
  );
}
