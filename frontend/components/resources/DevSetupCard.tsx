export function DevSetupCard({
  theme,
  title,
  description,
  commands,
}: {
  theme: "light" | "dark";
  title: string;
  description: string;
  commands: string[];
}) {
  return (
    <div
      className={`border p-8 space-y-4 ${
        theme === "light"
          ? "border-black/10 bg-white"
          : "border-white/5 bg-black/20"
      }`}
    >
      <div>
        <h3 className="text-xl font-bold mb-2">{title}</h3>
        <p
          className={`text-sm ${
            theme === "light" ? "text-gray-700" : "text-gray-400"
          }`}
        >
          {description}
        </p>
      </div>

      <div
        className={`border p-4 font-mono text-sm space-y-1 ${
          theme === "light"
            ? "border-black/20 bg-black/5 text-black"
            : "border-white/10 bg-white/5 text-white"
        }`}
      >
        {commands.map((cmd, i) => (
          <div key={i}>$ {cmd}</div>
        ))}
      </div>
    </div>
  );
}
