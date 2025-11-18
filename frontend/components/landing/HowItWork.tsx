import React from "react";
import { motion } from "framer-motion";
import { WORKFLOW } from "@/constants/data-default";

export default function HowItWork({ theme }: { theme: any }) {
  return (
    <>
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
            {WORKFLOW.map((step, index) => (
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
    </>
  );
}
