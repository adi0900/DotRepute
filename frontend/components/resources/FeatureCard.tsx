export function FeatureCard({
  theme,
  title,
  description,
}: {
  theme: "light" | "dark";
  title: string;
  description: string;
}) {
  return (
    <div
      className={`border p-6 space-y-3 ${
        theme === "light"
          ? "border-black/10 bg-white"
          : "border-white/10 bg-black/40"
      }`}
    >
      <h3 className="font-bold text-lg">{title}</h3>
      <p
        className={`text-sm ${
          theme === "light" ? "text-gray-600" : "text-gray-500"
        }`}
      >
        {description}
      </p>
    </div>
  );
}
