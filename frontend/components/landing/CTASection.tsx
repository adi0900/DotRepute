import React from "react";
import { motion } from "framer-motion";
import { ArrowRight, Github } from "lucide-react";

export default function CTASection({ theme }: { theme: any }) {
  return (
    <>
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
    </>
  );
}
