export function TechList({
  theme,
  items,
}: {
  theme: "light" | "dark";
  items: Array<{ name: string; description: string }>;
}) {
  return (
    <div className="space-y-4">
      {items.map((item, i) => (
        <div key={i} className="space-y-1">
          <div className="font-semibold">{item.name}</div>
          <div
            className={`text-sm ${
              theme === "light" ? "text-gray-600" : "text-gray-500"
            }`}
          >
            {item.description}
          </div>
        </div>
      ))}
    </div>
  );
}
