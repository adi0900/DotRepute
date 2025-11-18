/**
 * DotRepute Landing Page
 * Design Reference: G5OYQkibIAEsC2B.jpg (Fumadocs-style dark theme with orange gradients)
 * Icons: Lucide React
 */

"use client";

import { useEffect, useState } from "react";
import { Github, Shield, Layers, Zap } from "lucide-react";
import { PremiumWebGLLoader } from "@/components/premium-webgl-loader";
import { VentureNavbar } from "@/components/venture-navbar";
import { useLunoTheme } from "@luno-kit/ui";
import HeroSection from "@/components/landing/HeroSection";
import FeatureSection from "@/components/landing/FeatureSection";
import TechStackSection from "@/components/landing/TechStackSection";
import HowItWork from "@/components/landing/HowItWork";
import TeamSection from "@/components/landing/TeamSection";
import CTASection from "@/components/landing/CTASection";

export default function LandingPage() {
  const [theme, setTheme] = useState<"light" | "dark">("dark");
  const [mounted, setMounted] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const { themeMode, setThemeChoice } = useLunoTheme();

  // Initialize theme from localStorage on mount
  useEffect(() => {
    setMounted(true);
    const savedTheme = localStorage.getItem("theme") as "light" | "dark" | null;
    const initialTheme = savedTheme || "dark";
    setTheme(initialTheme);
    document.documentElement.classList.toggle("dark", initialTheme === "dark");

    // Simulate loading (remove when you have actual data to load)
    const loadingTimer = setTimeout(() => {
      setIsLoading(false);
    }, 2500);

    return () => clearTimeout(loadingTimer);
  }, []);

  // Toggle theme and persist to localStorage
  const toggleTheme = () => {
    const newTheme = theme === "dark" ? "light" : "dark";
    setTheme(newTheme);
    setThemeChoice(newTheme);
    localStorage.setItem("theme", newTheme);
    document.documentElement.classList.toggle("dark", newTheme === "dark");
  };

  return (
    <>
      {/* Premium WebGL Loading Effect */}
      {isLoading && <PremiumWebGLLoader theme={theme} />}

      <div
        className={`relative min-h-screen overflow-hidden transition-colors duration-300 ${
          theme === "light" ? "bg-white text-black" : "bg-black text-white"
        }`}
      >
        {/* Venture Navbar */}
        <VentureNavbar
          theme={theme}
          onToggleTheme={toggleTheme}
          currentPath="/"
        />

        {/* Hero Section - Venture style with WebGL background */}
        <HeroSection theme={theme} />

        {/* Features Section - Venture style */}
        <FeatureSection theme={theme} />

        {/* Tech Stack Section - Venture style */}
        <TechStackSection theme={theme} />

        {/* How It Works Section - Venture style */}
        <HowItWork theme={theme} />

        {/* Team Section - Venture style */}
        <TeamSection theme={theme} />

        {/* CTA Section - Venture style */}
        <CTASection theme={theme} />

        {/* Footer - Venture style minimalist design */}
        <footer
          className={`relative border-t transition-colors ${
            theme === "light"
              ? "border-black/5 bg-white"
              : "border-white/5 bg-black"
          }`}
        >
          {/* Stats Section */}
          <div
            className={`border-b transition-colors ${
              theme === "light" ? "border-black/5" : "border-white/5"
            }`}
          >
            <div className="mx-auto px-8 py-16">
              <div className="grid md:grid-cols-3 gap-8 max-w-6xl mx-auto">
                {/* Stat 1 */}
                <div
                  className={`border p-8 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white"
                      : "border-white/10 bg-black"
                  }`}
                >
                  <div
                    className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                      theme === "light" ? "text-gray-600" : "text-gray-500"
                    }`}
                  >
                    Network Validators
                  </div>
                  <div className="flex items-center gap-3">
                    <Zap className="w-5 h-5 text-yellow-400" />
                    <span className="text-4xl font-bold tracking-tight">
                      1000+
                    </span>
                  </div>
                </div>

                {/* Stat 2 */}
                <div
                  className={`border p-8 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white"
                      : "border-white/10 bg-black"
                  }`}
                >
                  <div
                    className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                      theme === "light" ? "text-gray-600" : "text-gray-500"
                    }`}
                  >
                    Reputation Scores
                  </div>
                  <div className="flex items-center gap-3">
                    <Shield className="w-5 h-5 text-orange-400" />
                    <span className="text-4xl font-bold tracking-tight">
                      50K+
                    </span>
                  </div>
                </div>

                {/* Stat 3 */}
                <div
                  className={`border p-8 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white"
                      : "border-white/10 bg-black"
                  }`}
                >
                  <div
                    className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                      theme === "light" ? "text-gray-600" : "text-gray-500"
                    }`}
                  >
                    Parachains Support
                  </div>
                  <div className="flex items-center gap-3">
                    <Layers className="w-5 h-5 text-yellow-400" />
                    <span className="text-4xl font-bold tracking-tight">
                      15+
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Footer Bottom */}
          <div className="mx-auto px-8 py-6">
            <div className="flex flex-col md:flex-row justify-between items-center gap-6">
              {/* Left - Logo */}
              <div className="flex items-center gap-8">
                <div
                  className={`border px-4 py-3 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white"
                      : "border-white/10 bg-black"
                  }`}
                >
                  <span
                    className="text-xl font-bold tracking-tighter uppercase leading-none block"
                    style={{ letterSpacing: "-0.02em" }}
                  >
                    DOT
                    <br />
                    .REPUTE
                  </span>
                </div>

                <div
                  className={`hidden md:flex flex-col text-[10px] uppercase tracking-wider font-mono ${
                    theme === "light" ? "text-gray-600" : "text-gray-500"
                  }`}
                >
                  <span>Ver-2.0 / 2025</span>
                  <span>POLKADOT ECOSYSTEM</span>
                </div>
              </div>

              {/* Right - Links */}
              <div className="flex items-center gap-6">
                <a
                  href="https://github.com/adi0900/DotRepute"
                  target="_blank"
                  rel="noopener noreferrer"
                  className={`text-xs uppercase tracking-wider font-medium transition-colors ${
                    theme === "light"
                      ? "text-gray-600 hover:text-black"
                      : "text-gray-400 hover:text-white"
                  }`}
                >
                  GitHub
                </a>
                <span
                  className={
                    theme === "light" ? "text-gray-400" : "text-gray-700"
                  }
                >
                  |
                </span>
                <span
                  className={`text-xs uppercase tracking-wider font-mono ${
                    theme === "light" ? "text-gray-600" : "text-gray-500"
                  }`}
                >
                  MIT License
                </span>
                <span
                  className={
                    theme === "light" ? "text-gray-400" : "text-gray-700"
                  }
                >
                  |
                </span>
                <a
                  href="#team"
                  className={`text-xs uppercase tracking-wider font-medium transition-colors ${
                    theme === "light"
                      ? "text-gray-600 hover:text-black"
                      : "text-gray-400 hover:text-white"
                  }`}
                >
                  Team
                </a>
              </div>
            </div>
          </div>
        </footer>
      </div>
    </>
  );
}

// Feature cards with Lucide icons
