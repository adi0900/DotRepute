import React from "react";
import { motion } from "framer-motion";
import { FEATURES } from "@/constants/data-default";

export default function FeatureSection({ theme }: { theme: any }) {
  return (
    <>
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
            {FEATURES.map((feature, index) => (
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
    </>
  );
}
