/**
 * DotRepute Landing Page
 * Design Reference: G5OYQkibIAEsC2B.jpg (Fumadocs-style dark theme with orange gradients)
 * Icons: Lucide React
 */

"use client";

import { useEffect, useRef, useState } from "react";
import { motion } from "framer-motion";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  ArrowRight,
  Github,
  Shield,
  Sparkles,
  Database,
  Code2,
  Layers,
  Lock,
  Zap,
  FileCode2,
  Boxes,
  CircuitBoard,
  Workflow,
  Users,
  ExternalLink,
  Menu,
  Sun,
  Moon,
} from "lucide-react";
import { AnimatedHeroBackground } from "@/components/animated-hero-background";
import { EnhancedWebGLBackground } from "@/components/enhanced-webgl-background";
import { DitheredBackground } from "@/components/dithered-background";
import { ParticleGlobe } from "@/components/particle-globe";
import { PremiumWebGLLoader } from "@/components/premium-webgl-loader";
import { VentureNavbar } from "@/components/venture-navbar";
import { useLunoTheme } from "@luno-kit/ui";
import { ENDPOINTS } from "@/constants/endpoints";

export default function LandingPage() {
  const [theme, setTheme] = useState<"light" | "dark">("dark");
  const [mounted, setMounted] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const { themeMode, setThemeChoice } = useLunoTheme();
  const [data, setData] = useState<any>(null);

  const getData = async () => {
    const res = await fetch("/api/subscan");
    if (!res.ok) {
      throw new Error(`HTTP ${res.status}`);
    }
    const json = await res.json();
    setData(json.data);
  };

  useEffect(() => {
    getData();
  }, []);

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
        <section
          className={`relative min-h-screen flex items-center justify-center pt-32 overflow-hidden transition-colors ${
            theme === "light" ? "bg-gray-50" : "bg-black"
          }`}
          style={{ willChange: "transform" }}
        >
          {/* Enhanced WebGL-style background with dot matrix pattern - Only in dark mode */}
          {theme === "dark" && <EnhancedWebGLBackground />}

          {/* Light mode subtle gradient background with accent orb and dot pattern */}
          {theme === "light" && (
            <>
              <div className="absolute inset-0 bg-gradient-to-br from-orange-50 via-white to-yellow-50 opacity-60" />
              {/* Accent orb for depth in light mode (matching WebGL style) */}
              <div
                className="absolute top-[40%] right-[10%] w-[800px] h-[800px]"
                style={{
                  background:
                    "radial-gradient(circle, rgba(251, 146, 60, 0.2) 0%, rgba(252, 211, 77, 0.12) 35%, rgba(254, 240, 138, 0.06) 60%, transparent 80%)",
                  filter: "blur(80px)",
                }}
              />
              {/* Subtle dot pattern for light mode */}
              <div
                className="absolute inset-0 opacity-30"
                style={{
                  backgroundImage:
                    "radial-gradient(circle, rgba(251, 146, 60, 0.4) 1px, transparent 1px)",
                  backgroundSize: "50px 50px",
                }}
              />
            </>
          )}

          {/* Particle Globe - DotRepute logo - visible in both modes */}
          <div
            className={`absolute top-[20%] right-[5%] z-[2] hidden lg:block transition-opacity ${
              theme === "light" ? "opacity-90" : "opacity-100"
            }`}
            style={{
              filter:
                theme === "light"
                  ? "drop-shadow(0 0 20px rgba(60, 60, 60, 0.3))"
                  : "none",
            }}
          >
            <ParticleGlobe theme={theme} />
          </div>

          <div className="container mx-auto px-6 relative z-10">
            <div className="max-w-7xl mx-auto">
              {/* Main Hero Content */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.8 }}
                className="space-y-8 max-w-4xl"
              >
                {/* Badge - Venture boxed style with premium glow effect */}
                <div
                  className={`inline-flex border px-4 py-2 transition-colors ${
                    theme === "light"
                      ? "border-black/20 bg-white"
                      : "border-white/10 bg-black/40 backdrop-blur-sm"
                  }`}
                >
                  <span
                    className={`relative text-[10px] uppercase tracking-wider font-mono ${
                      theme === "light" ? "text-gray-600" : "text-gray-400"
                    }`}
                    style={{
                      textShadow:
                        theme === "light"
                          ? "0 0 20px rgba(251, 146, 60, 0), 0 0 40px rgba(252, 211, 77, 0)"
                          : "0 0 20px rgba(251, 146, 60, 0), 0 0 40px rgba(252, 211, 77, 0)",
                      animation:
                        theme === "light"
                          ? "text-glow-light 3s ease-in-out infinite"
                          : "text-glow-dark 3s ease-in-out infinite",
                    }}
                  >
                    the Rust-powered reputation system you trust
                  </span>
                </div>

                {/* Large heading - consistent with theme */}
                <h1 className="text-5xl md:text-6xl lg:text-7xl xl:text-8xl font-bold leading-[1.1] tracking-tight">
                  Build transparent{" "}
                  <span className="font-normal">reputation systems,</span>
                  <br />
                  <span className="font-normal">your way.</span>
                </h1>

                {/* Description with key terms highlighted */}
                <p
                  className={`text-lg md:text-xl leading-relaxed max-w-3xl ${
                    theme === "light" ? "text-gray-700" : "text-gray-400"
                  }`}
                >
                  DotRepute is a{" "}
                  <span
                    className={`font-medium ${
                      theme === "light" ? "text-black" : "text-white"
                    }`}
                  >
                    Rust-powered Contributor Reputation System
                  </span>{" "}
                  for the Polkadot ecosystem. Aggregating identity, governance,
                  staking, and activity signals to generate transparent,
                  verifiable, and user-centric reputation scores.
                </p>

                {/* CTA Buttons - Venture boxed style with premium hover effects */}
                <div className="flex flex-col sm:flex-row gap-4 pt-2">
                  <a
                    href="/dashboard"
                    className={`group relative border px-8 py-4 text-center transition-all duration-300 overflow-hidden ${
                      theme === "light"
                        ? "border-black/20 bg-white hover:border-black/40 hover:shadow-lg"
                        : "border-white/10 bg-black/40 backdrop-blur-sm hover:border-white/30 hover:shadow-xl hover:shadow-orange-500/10"
                    }`}
                  >
                    <span
                      className={`relative z-10 text-sm uppercase tracking-wider font-medium transition-colors ${
                        theme === "light"
                          ? "group-hover:text-black"
                          : "group-hover:text-white"
                      }`}
                    >
                      Getting Started
                    </span>
                    {/* Premium shine effect on hover */}
                    <div
                      className={`absolute inset-0 -translate-x-full group-hover:translate-x-full transition-transform duration-700 ${
                        theme === "light"
                          ? "bg-gradient-to-r from-transparent via-black/5 to-transparent"
                          : "bg-gradient-to-r from-transparent via-white/10 to-transparent"
                      }`}
                    />
                  </a>
                  <a
                    href="#"
                    className={`group relative border px-8 py-4 text-center transition-all duration-300 overflow-hidden ${
                      theme === "light"
                        ? "border-black/20 bg-white hover:border-black/40 hover:shadow-lg"
                        : "border-white/10 bg-black/40 backdrop-blur-sm hover:border-white/30 hover:shadow-xl hover:shadow-orange-500/10"
                    }`}
                  >
                    <span
                      className={`relative z-10 text-sm uppercase tracking-wider font-medium transition-colors ${
                        theme === "light"
                          ? "group-hover:text-black"
                          : "group-hover:text-white"
                      }`}
                    >
                      Open Demo
                    </span>
                    {/* Premium shine effect on hover */}
                    <div
                      className={`absolute inset-0 -translate-x-full group-hover:translate-x-full transition-transform duration-700 ${
                        theme === "light"
                          ? "bg-gradient-to-r from-transparent via-black/5 to-transparent"
                          : "bg-gradient-to-r from-transparent via-white/10 to-transparent"
                      }`}
                    />
                  </a>
                </div>
              </motion.div>

              {/* Bottom Preview Cards - Venture boxed style */}
              <motion.div
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.8, delay: 0.3 }}
                className="mt-32 grid md:grid-cols-2 gap-6 max-w-4xl"
              >
                {/* Quick Start Card */}
                <div
                  className={`border p-6 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white hover:border-black/20"
                      : "border-white/10 bg-black/40 backdrop-blur-sm hover:border-white/20"
                  }`}
                >
                  <div className="flex items-center gap-3 mb-4">
                    <Code2 className="w-5 h-5 text-orange-400" />
                    <h3 className="text-lg font-bold uppercase tracking-tight">
                      DotRepute Core
                    </h3>
                  </div>
                  <p
                    className={`text-xs font-mono leading-relaxed ${
                      theme === "light" ? "text-gray-700" : "text-gray-400"
                    }`}
                  >
                    Handles most of the logic including reputation scoring, data
                    cleaning, identity verification, and governance tracking
                    with Rust modules.
                  </p>
                </div>

                {/* Indexer Card */}
                <div
                  className={`border p-6 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white hover:border-black/20"
                      : "border-white/10 bg-black/40 backdrop-blur-sm hover:border-white/20"
                  }`}
                >
                  <div className="flex items-center gap-3 mb-4">
                    <Database className="w-5 h-5 text-blue-400" />
                    <h3 className="text-lg font-bold uppercase tracking-tight">
                      DotRepute Indexer
                    </h3>
                  </div>
                  <p
                    className={`text-xs font-mono leading-relaxed ${
                      theme === "light" ? "text-gray-700" : "text-gray-400"
                    }`}
                  >
                    The SubQuery indexer offers comprehensive blockchain data
                    indexing for identity, governance, and staking events across
                    Polkadot.
                  </p>
                </div>
              </motion.div>
            </div>
          </div>
        </section>

        {/* Features Section - Venture style */}
        <section
          id="features"
          className={`relative py-24 border-t transition-colors ${
            theme === "light" ? "border-black/5" : "border-white/5"
          }`}
        >
          <div className="mx-auto px-8">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="mb-16"
            >
              <div
                className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
                Core Features
              </div>
              <h2 className="text-5xl md:text-6xl font-bold tracking-tight mb-6">
                Built for the Polkadot Ecosystem
              </h2>
              <p
                className={`text-sm max-w-2xl uppercase tracking-wide font-mono ${
                  theme === "light" ? "text-gray-600" : "text-gray-400"
                }`}
              >
                A modular, Rust-first architecture designed for transparency,
                performance, and extensibility
              </p>
            </motion.div>

            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {features.map((feature, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: index * 0.05 }}
                >
                  <div
                    className={`h-full border p-6 transition-colors ${
                      theme === "light"
                        ? "border-black/10 bg-white hover:border-black/20"
                        : "border-white/10 bg-black hover:border-white/20"
                    }`}
                  >
                    <div className="flex items-center gap-3 mb-4">
                      {feature.icon}
                      <h3 className="text-base font-bold uppercase tracking-tight">
                        {feature.title}
                      </h3>
                    </div>
                    <p
                      className={`text-xs leading-relaxed mb-4 font-mono ${
                        theme === "light" ? "text-gray-600" : "text-gray-400"
                      }`}
                    >
                      {feature.description}
                    </p>
                    {feature.highlights && (
                      <ul className="space-y-2">
                        {feature.highlights.map((highlight, i) => (
                          <li
                            key={i}
                            className={`text-[10px] flex items-start gap-2 font-mono uppercase tracking-wider ${
                              theme === "light"
                                ? "text-gray-500"
                                : "text-gray-500"
                            }`}
                          >
                            <span className="text-yellow-400 mt-0.5">—</span>
                            <span>{highlight}</span>
                          </li>
                        ))}
                      </ul>
                    )}
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* Tech Stack Section - Venture style */}
        <section
          id="tech"
          className={`relative py-24 border-t transition-colors ${
            theme === "light" ? "border-black/5" : "border-white/5"
          }`}
        >
          <div className="mx-auto px-8">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="mb-16"
            >
              <div
                className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
                Technology Stack
              </div>
              <h2 className="text-5xl md:text-6xl font-bold tracking-tight">
                Powered by Rust
              </h2>
            </motion.div>

            <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
              {techStack.map((tech, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, scale: 0.95 }}
                  whileInView={{ opacity: 1, scale: 1 }}
                  viewport={{ once: true }}
                  transition={{ delay: index * 0.05 }}
                >
                  <div
                    className={`border p-6 transition-colors ${
                      theme === "light"
                        ? "border-black/10 bg-white"
                        : "border-white/10 bg-black"
                    }`}
                  >
                    <div
                      className={`text-[10px] font-medium uppercase tracking-wider mb-4 font-mono ${
                        theme === "light" ? "text-gray-600" : "text-gray-500"
                      }`}
                    >
                      {tech.category}
                    </div>
                    <ul className="space-y-2">
                      {tech.items.map((item, i) => (
                        <li
                          key={i}
                          className={`text-sm font-mono ${
                            theme === "light"
                              ? "text-gray-700"
                              : "text-gray-300"
                          }`}
                        >
                          {item}
                        </li>
                      ))}
                    </ul>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* How It Works Section - Venture style */}
        <section
          id="how-it-works"
          className={`relative py-24 border-t transition-colors ${
            theme === "light" ? "border-black/5" : "border-white/5"
          }`}
        >
          <div className="mx-auto px-8">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="mb-16"
            >
              <div
                className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
                Workflow
              </div>
              <h2 className="text-5xl md:text-6xl font-bold tracking-tight mb-6">
                How It Works
              </h2>
              <p
                className={`text-sm uppercase tracking-wide font-mono max-w-2xl ${
                  theme === "light" ? "text-gray-600" : "text-gray-400"
                }`}
              >
                From user request to reputation score in seven seamless steps
              </p>
            </motion.div>

            <div className="max-w-5xl mx-auto grid md:grid-cols-7 gap-6">
              {workflow.map((step, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: index * 0.05 }}
                >
                  <div
                    className={`border p-6 h-full transition-colors ${
                      theme === "light"
                        ? "border-black/10 bg-white"
                        : "border-white/10 bg-black"
                    }`}
                  >
                    <div
                      className={`text-[10px] font-mono uppercase tracking-wider mb-2 ${
                        theme === "light" ? "text-gray-600" : "text-gray-500"
                      }`}
                    >
                      Step {String(index + 1).padStart(2, "0")}
                    </div>
                    <h3 className="text-sm font-bold uppercase tracking-tight mb-3">
                      {step.title}
                    </h3>
                    <p
                      className={`text-[10px] font-mono leading-relaxed ${
                        theme === "light" ? "text-gray-600" : "text-gray-400"
                      }`}
                    >
                      {step.description}
                    </p>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* Team Section - Venture style */}
        <section
          id="team"
          className={`relative py-24 border-t transition-colors ${
            theme === "light" ? "border-black/5" : "border-white/5"
          }`}
        >
          <div className="mx-auto px-8">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="mb-16"
            >
              <div
                className={`text-[10px] uppercase tracking-wider font-mono mb-4 ${
                  theme === "light" ? "text-gray-600" : "text-gray-500"
                }`}
              >
                Team
              </div>
              <h2 className="text-5xl md:text-6xl font-bold tracking-tight">
                Built by Experts
              </h2>
            </motion.div>

            <div className="grid md:grid-cols-2 gap-6 max-w-4xl mx-auto">
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
              >
                <div
                  className={`border p-8 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white"
                      : "border-white/10 bg-black"
                  }`}
                >
                  <div className="flex items-start justify-between mb-4">
                    <div>
                      <h3 className="text-xl font-bold uppercase tracking-tight mb-2">
                        Aditya
                      </h3>
                      <div className="text-[10px] uppercase tracking-wider text-yellow-400 font-mono">
                        Product Designer & Product Manager
                      </div>
                    </div>
                    <div
                      className={`text-[10px] font-mono ${
                        theme === "light" ? "text-gray-600" : "text-gray-500"
                      }`}
                    >
                      01
                    </div>
                  </div>
                  <p
                    className={`text-xs font-mono leading-relaxed ${
                      theme === "light" ? "text-gray-600" : "text-gray-400"
                    }`}
                  >
                    Leads product direction, UX, flows, and strategy
                  </p>
                </div>
              </motion.div>

              <motion.div
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ delay: 0.05 }}
              >
                <div
                  className={`border p-8 transition-colors ${
                    theme === "light"
                      ? "border-black/10 bg-white"
                      : "border-white/10 bg-black"
                  }`}
                >
                  <div className="flex items-start justify-between mb-4">
                    <div>
                      <h3 className="text-xl font-bold uppercase tracking-tight mb-2">
                        Steven Muanigo
                      </h3>
                      <div className="text-[10px] uppercase tracking-wider text-orange-400 font-mono">
                        Backend & Infrastructure Developer
                      </div>
                    </div>
                    <div
                      className={`text-[10px] font-mono ${
                        theme === "light" ? "text-gray-600" : "text-gray-500"
                      }`}
                    >
                      02
                    </div>
                  </div>
                  <p
                    className={`text-xs font-mono leading-relaxed ${
                      theme === "light" ? "text-gray-600" : "text-gray-400"
                    }`}
                  >
                    Builds Rust backend services, ink! modules, infrastructure,
                    and indexing
                  </p>
                </div>
              </motion.div>
            </div>
          </div>
        </section>

        {/* CTA Section - Venture style */}
        <section
          className={`relative py-24 border-t transition-colors ${
            theme === "light" ? "border-black/5" : "border-white/5"
          }`}
        >
          <div className="mx-auto px-8">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="max-w-5xl mx-auto"
            >
              <div
                className={`border p-12 md:p-16 transition-colors ${
                  theme === "light"
                    ? "border-black/10 bg-white"
                    : "border-white/10 bg-black"
                }`}
              >
                <div
                  className={`text-[10px] uppercase tracking-wider font-mono mb-6 text-center ${
                    theme === "light" ? "text-gray-600" : "text-gray-500"
                  }`}
                >
                  Get Started
                </div>

                <h2 className="text-4xl md:text-5xl lg:text-6xl font-bold mb-6 tracking-tight text-center">
                  Ready to Build Transparent Reputation?
                </h2>

                <p
                  className={`text-sm uppercase tracking-wide font-mono mb-12 max-w-2xl mx-auto text-center ${
                    theme === "light" ? "text-gray-600" : "text-gray-400"
                  }`}
                >
                  Start using DotRepute today and bring trust to your Polkadot
                  ecosystem
                </p>

                <div className="flex flex-col sm:flex-row gap-4 justify-center">
                  <a
                    href="/dashboard"
                    className={`border px-8 py-4 transition-colors text-center ${
                      theme === "light"
                        ? "border-black/10 hover:bg-black/5"
                        : "border-white/10 hover:bg-white/5"
                    }`}
                  >
                    <div className="flex items-center justify-center gap-2">
                      <span className="text-sm uppercase tracking-wider font-medium">
                        Get Started
                      </span>
                      <ArrowRight className="w-4 h-4" />
                    </div>
                  </a>

                  <a
                    href="https://github.com/adi0900/DotRepute"
                    target="_blank"
                    rel="noopener noreferrer"
                    className={`border px-8 py-4 transition-colors text-center ${
                      theme === "light"
                        ? "border-black/10 hover:bg-black/5"
                        : "border-white/10 hover:bg-white/5"
                    }`}
                  >
                    <div className="flex items-center justify-center gap-2">
                      <Github className="w-4 h-4" />
                      <span className="text-sm uppercase tracking-wider font-medium">
                        View on GitHub
                      </span>
                    </div>
                  </a>
                </div>
              </div>
            </motion.div>
          </div>
        </section>

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
const features = [
  {
    icon: <Code2 className="w-5.5 h-5.5 text-orange-400" />,
    title: "Rust-based Reputation Engine",
    description:
      "Core scoring logic implemented in Rust with comprehensive weighted metrics, time decay, and modular architecture.",
    highlights: [
      "Fully testable modules",
      "WASM-compilable for portability",
      "Multi-metric scoring system",
    ],
  },
  {
    icon: <Database className="w-5.5 h-5.5 text-blue-400" />,
    title: "SubQuery Indexing",
    description:
      "Comprehensive blockchain data indexing for identity, governance, staking, and activity events.",
    highlights: [
      "Real-time event processing",
      "GraphQL API access",
      "Cross-domain correlation",
    ],
  },
  {
    icon: <Shield className="w-5.5 h-5.5 text-green-400" />,
    title: "Identity Verification",
    description:
      "Multi-source identity verification with Sybil protection and gradual trust level system.",
    highlights: [
      "On-chain and off-chain verification",
      "GDPR/KYC compliance",
      "Timestamped validity",
    ],
  },
  {
    icon: <Zap className="w-5.5 h-5.5 text-yellow-400" />,
    title: "Data Cleaning Engine",
    description:
      "Enterprise-grade data quality engine with validation, normalization, and anomaly detection.",
    highlights: [
      "Type validation",
      "Missing data handling",
      "Format standardization",
    ],
  },
  {
    icon: <Boxes className="w-5.5 h-5.5 text-purple-400" />,
    title: "Modular Architecture",
    description:
      "Extensible trait-based design allowing easy integration of custom metrics and scoring algorithms.",
    highlights: [
      "Plugin system",
      "Custom metric support",
      "Configurable weights",
    ],
  },
  {
    icon: <Lock className="w-5.5 h-5.5 text-red-400" />,
    title: "Transparent & Auditable",
    description:
      "All scoring logic is open-source and verifiable, with on-chain attestation support via ink! contracts.",
    highlights: [
      "Open-source scoring",
      "On-chain proofs",
      "Community governance",
    ],
  },
];

const techStack = [
  {
    category: "Core (Rust)",
    items: ["Rust", "ink!", "WASM", "Tokio"],
  },
  {
    category: "Blockchain",
    items: ["PolkadotJS API", "SubQuery", "Substrate"],
  },
  {
    category: "Frontend",
    items: ["Next.js", "React", "TailwindCSS", "ShadCN UI"],
  },
  {
    category: "Infrastructure",
    items: ["Redis", "PostgreSQL", "Docker", "GraphQL"],
  },
];

const workflow = [
  {
    title: "User Input",
    description:
      "User requests reputation data for an address or interacts with the system",
  },
  {
    title: "Intent Recognition",
    description:
      "Backend/Frontend parses the request and determines required data",
  },
  {
    title: "Validation",
    description:
      "Check if additional parameters are needed (address format, date ranges, etc.)",
  },
  {
    title: "Data Collection",
    description:
      "Query SubQuery indexer for on-chain data, fetch from cache (Redis) if available, retrieve from Rust scoring engine",
  },
  {
    title: "Score Calculation",
    description:
      "Rust modules process data through scoring algorithms with time decay and weighted metrics",
  },
  {
    title: "Response Generation",
    description:
      "Format results with breakdown, badges, levels, and component scores",
  },
  {
    title: "Delivery",
    description:
      "Return JSON/GraphQL response or render UI components for the user",
  },
];
