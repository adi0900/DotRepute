"use client";
import { VentureNavbar } from "@/components/venture-navbar";
import { useTheme } from "@/context/ThemeContext";
import { useLunoTheme } from "@luno-kit/ui";
import { usePathname } from "next/navigation";
import React, { useEffect, useState } from "react";

export default function LayoutWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  const pathname = usePathname();
  const { theme, toggleTheme } = useTheme();
  return (
    <>
      <div
        className={`relative min-h-screen overflow-hidden transition-colors duration-300 ${
          theme === "light" ? "bg-white text-black" : "bg-black text-white"
        }`}
      >
        <VentureNavbar
          theme={theme}
          onToggleTheme={toggleTheme}
          currentPath={pathname}
        />

        <main>{children}</main>
      </div>
    </>
  );
}
