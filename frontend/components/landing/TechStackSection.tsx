import React from "react";
import { motion } from "framer-motion";
import { TECHSTACK } from "@/constants/data-default";

export default function TechStackSection({ theme }: { theme: any }) {
  return (
    <>
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
            {TECHSTACK.map((tech, index) => (
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
                          theme === "light" ? "text-gray-700" : "text-gray-300"
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
    </>
  );
}
