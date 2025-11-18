export function FeatureCard({
  theme,
  icon,
  title,
  description,
}: {
  theme: "light" | "dark";
  icon: React.ReactNode;
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
      <div
        className={`inline-flex border p-2 ${
          theme === "light"
            ? "border-black/20 bg-white"
            : "border-white/10 bg-black/40"
        }`}
      >
        {icon}
      </div>
      <h3 className="font-bold">{title}</h3>
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
