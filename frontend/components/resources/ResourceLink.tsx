import { ExternalLink } from "lucide-react";

export function ResourceLink({
  theme,
  title,
  description,
  url,
}: {
  theme: "light" | "dark";
  title: string;
  description: string;
  url: string;
}) {
  return (
    <a
      href={url}
      target="_blank"
      rel="noopener noreferrer"
      className={`border p-6 space-y-3 transition-colors group ${
        theme === "light"
          ? "border-black/10 hover:bg-black/5"
          : "border-white/10 hover:bg-white/5"
      }`}
    >
      <div className="flex items-start justify-between">
        <h3 className="font-bold text-lg">{title}</h3>
        <ExternalLink
          className={`w-4 h-4 transition-transform group-hover:translate-x-1 group-hover:-translate-y-1 ${
            theme === "light" ? "text-gray-600" : "text-gray-500"
          }`}
        />
      </div>
      <p
        className={`text-sm ${
          theme === "light" ? "text-gray-600" : "text-gray-500"
        }`}
      >
        {description}
      </p>
    </a>
  );
}
