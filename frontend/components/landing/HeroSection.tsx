import { EnhancedWebGLBackground } from "@/components/enhanced-webgl-background";
import { ParticleGlobe } from "@/components/particle-globe";
import React from "react";
import { motion } from "framer-motion";
import { Code2, Database } from "lucide-react";

export default function HeroSection({ theme }: { theme: any }) {
  return (
    <>
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
                  cleaning, identity verification, and governance tracking with
                  Rust modules.
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
    </>
  );
}
