"use client";
import { LunoKitProvider, ThemeMode } from "@luno-kit/ui";
import React from "react";
import { lunoConfig } from "@/lib/luno-config";
import { darkTheme, lightTheme } from "@/lib/luno-theme";

export const lunoKitTheme = {
  autoMode: true,
  defaultMode: "light" as ThemeMode,
  light: lightTheme,
  dark: darkTheme,
};

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <LunoKitProvider config={lunoConfig} theme={lunoKitTheme}>
      {children}
    </LunoKitProvider>
  );
}
