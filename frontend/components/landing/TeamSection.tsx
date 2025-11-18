import React from "react";
import { motion } from "framer-motion";

export default function TeamSection({ theme }: { theme: any }) {
  return (
    <>
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
    </>
  );
}
