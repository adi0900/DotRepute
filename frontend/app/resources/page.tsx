/**
 * DotRepute Resources Page
 * Venture-style Resources and Tech Stack
 */

"use client";

import { useState } from "react";
import { useTheme } from "@/context/ThemeContext";
import { SECTIONS_RESOURCES } from "@/constants/data-default";
import { TechStackSection } from "@/components/resources/TechStackSection";
import { RepositorySection } from "@/components/resources/RepositorySection";
import { SetupSection } from "@/components/resources/SetupSection";
import { FeaturesSection } from "@/components/resources/FeaturesSection";
import { ExternalSection } from "@/components/resources/ExternalSection";
import { ToolsSection } from "@/components/resources/ToolsSection";

export default function ResourcesPage() {
  const [activeSection, setActiveSection] = useState<string>("tech-stack");
  const { theme, mounted } = useTheme();
  if (!mounted) return null;

  return (
    <>
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
                Resources
              </div>
              {SECTIONS_RESOURCES.map((section) => (
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
              {activeSection === "tech-stack" && (
                <TechStackSection theme={theme} />
              )}
              {activeSection === "repository" && (
                <RepositorySection theme={theme} />
              )}
              {activeSection === "setup" && <SetupSection theme={theme} />}
              {activeSection === "features" && (
                <FeaturesSection theme={theme} />
              )}
              {activeSection === "external" && (
                <ExternalSection theme={theme} />
              )}
              {activeSection === "tools" && <ToolsSection theme={theme} />}
            </div>
          </div>
        </div>
      </main>
    </>
  );
}
