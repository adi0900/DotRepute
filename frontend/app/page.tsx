/**
 * DotRepute Landing Page
 * Design Reference: G5OYQkibIAEsC2B.jpg (Fumadocs-style dark theme with orange gradients)
 * Icons: Lucide React
 */

"use client";
import { PremiumWebGLLoader } from "@/components/premium-webgl-loader";
import HeroSection from "@/components/landing/HeroSection";
import FeatureSection from "@/components/landing/FeatureSection";
import TechStackSection from "@/components/landing/TechStackSection";
import HowItWork from "@/components/landing/HowItWork";
import TeamSection from "@/components/landing/TeamSection";
import CTASection from "@/components/landing/CTASection";
import { useTheme } from "@/context/ThemeContext";
import Footer from "@/components/footer";

export default function LandingPage() {
  const { theme, isLoading } = useTheme();
  return (
    <>
      {/* Premium WebGL Loading Effect */}
      {isLoading && <PremiumWebGLLoader theme={theme} />}

      {/* Hero Section - Venture style with WebGL background */}
      <HeroSection theme={theme} />

      {/* Features Section - Venture style */}
      <FeatureSection theme={theme} />

      {/* Tech Stack Section - Venture style */}
      <TechStackSection theme={theme} />

      {/* How It Works Section - Venture style */}
      <HowItWork theme={theme} />

      {/* Team Section - Venture style */}
      <TeamSection theme={theme} />

      {/* CTA Section - Venture style */}
      <CTASection theme={theme} />

      {/* Footer - Venture style minimalist design */}
      <Footer theme={theme} />
    </>
  );
}

// Feature cards with Lucide icons
