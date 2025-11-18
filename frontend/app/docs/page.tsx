/**
 * DotRepute Documentation Page
 * Venture-style Documentation
 */

"use client";

import { useState } from "react";
import { useTheme } from "@/context/ThemeContext";
import { SECTIONS_DOCS } from "@/constants/data-default";
import { OverviewSection } from "@/components/docs/OverviewSection";
import { ScoringSection } from "@/components/docs/ScoringSection";
import { ArchitectureSection } from "@/components/docs/ArchitectureSection";
import { DataSourcesSection } from "@/components/docs/DataSourcesSection";
import { DevelopmentSection } from "@/components/docs/DevelopmentSection";

export default function DocsPage() {
  const [activeSection, setActiveSection] = useState<string>("overview");
  const { theme, mounted } = useTheme();

  if (!mounted) return null;

  return (
    <main className="pt-24 min-h-screen">
      <div className="flex">
        {/* Sidebar Navigation */}
        <aside
          className={`w-64 border-r min-h-screen sticky top-24 ${
            theme === "light" ? "border-black/10" : "border-white/5"
          }`}
        >
          <div className="p-6 space-y-2">
            <div
              className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                theme === "light" ? "text-gray-600" : "text-gray-500"
              }`}
            >
              Documentation
            </div>
            {SECTIONS_DOCS.map((section) => (
              <button
                key={section.id}
                onClick={() => setActiveSection(section.id)}
                className={`w-full flex items-center gap-3 px-4 py-3 text-sm transition-colors border ${
                  activeSection === section.id
                    ? theme === "light"
                      ? "border-black/40 bg-black/10 text-black"
                      : "border-white/30 bg-white/10 text-white"
                    : theme === "light"
                      ? "border-black/10 hover:bg-black/5 text-gray-700"
                      : "border-white/5 hover:bg-white/5 text-gray-400"
                }`}
              >
                {section.icon}
                {section.label}
              </button>
            ))}
          </div>
        </aside>

        {/* Main Content */}
        <div className="flex-1 px-12 py-12 flex justify-center">
          <div className="max-w-4xl w-full">
            {activeSection === "overview" && <OverviewSection theme={theme} />}
            {activeSection === "architecture" && (
              <ArchitectureSection theme={theme} />
            )}
            {activeSection === "scoring" && <ScoringSection theme={theme} />}
            {activeSection === "data-sources" && (
              <DataSourcesSection theme={theme} />
            )}
            {activeSection === "development" && (
              <DevelopmentSection theme={theme} />
            )}
          </div>
        </div>
      </div>
    </main>
  );
}
