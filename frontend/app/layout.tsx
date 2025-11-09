import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Dot.Repute - Home",
  description: "A transparent, verifiable, and user-centric reputation system for the Polkadot ecosystem. Built with Rust, PolkadotJS API, and SubQuery.",
  keywords: ["Polkadot", "Rust", "Reputation", "Web3", "Blockchain", "DotRepute", "ink!", "WASM"],
  authors: [{ name: "DotRepute Team" }],
  icons: {
    icon: '/favicon.svg',
  },
  openGraph: {
    title: "Dot.Repute - Rust-Powered Contributor Reputation System",
    description: "Build transparent reputation systems for the Polkadot ecosystem",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body className={inter.className}>
        {children}
      </body>
    </html>
  );
}
