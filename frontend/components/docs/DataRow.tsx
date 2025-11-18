export function DataRow({
  theme,
  domain,
  source,
  purpose,
}: {
  theme: "light" | "dark";
  domain: string;
  source: string;
  purpose: string;
}) {
  return (
    <tr
      className={`border-b ${
        theme === "light" ? "border-black/10" : "border-white/5"
      }`}
    >
      <td
        className={`px-6 py-4 font-medium ${
          theme === "light" ? "text-black" : "text-white"
        }`}
      >
        {domain}
      </td>
      <td
        className={`px-6 py-4 ${
          theme === "light" ? "text-gray-700" : "text-gray-400"
        }`}
      >
        {source}
      </td>
      <td
        className={`px-6 py-4 ${
          theme === "light" ? "text-gray-700" : "text-gray-400"
        }`}
      >
        {purpose}
      </td>
    </tr>
  );
}
