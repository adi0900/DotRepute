/**
 * Premium WebGL-Style Loading Effect
 * Advanced canvas animation matching Venture design
 * Theme-aware for both light and dark modes
 */

'use client';

import { useEffect, useRef, useState } from 'react';

interface PremiumWebGLLoaderProps {
  theme?: 'light' | 'dark';
}

export function PremiumWebGLLoader({ theme = 'dark' }: PremiumWebGLLoaderProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [progress, setProgress] = useState(0);
  const [isComplete, setIsComplete] = useState(false);

  // Animated background particles
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    let animationId: number;
    const particles: Array<{
      x: number;
      y: number;
      vx: number;
      vy: number;
      size: number;
    }> = [];

    // Responsive canvas
    const resize = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    };
    resize();
    window.addEventListener('resize', resize);

    // Create particles
    const particleCount = 30;
    for (let i = 0; i < particleCount; i++) {
      particles.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        vx: (Math.random() - 0.5) * 0.5,
        vy: (Math.random() - 0.5) * 0.5,
        size: Math.random() * 2 + 1,
      });
    }

    // Animation loop
    const animate = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Update and draw particles
      particles.forEach((p) => {
        p.x += p.vx;
        p.y += p.vy;

        // Wrap around edges
        if (p.x < 0) p.x = canvas.width;
        if (p.x > canvas.width) p.x = 0;
        if (p.y < 0) p.y = canvas.height;
        if (p.y > canvas.height) p.y = 0;

        // Draw particle
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);

        if (theme === 'light') {
          ctx.fillStyle = 'rgba(0, 0, 0, 0.15)';
        } else {
          ctx.fillStyle = 'rgba(255, 255, 255, 0.15)';
        }
        ctx.fill();
      });

      // Draw connections
      particles.forEach((p1, i) => {
        particles.slice(i + 1).forEach((p2) => {
          const dx = p1.x - p2.x;
          const dy = p1.y - p2.y;
          const distance = Math.sqrt(dx * dx + dy * dy);

          if (distance < 150) {
            ctx.beginPath();
            ctx.moveTo(p1.x, p1.y);
            ctx.lineTo(p2.x, p2.y);

            const opacity = (1 - distance / 150) * 0.1;
            if (theme === 'light') {
              ctx.strokeStyle = `rgba(0, 0, 0, ${opacity})`;
            } else {
              ctx.strokeStyle = `rgba(255, 255, 255, ${opacity})`;
            }
            ctx.lineWidth = 0.5;
            ctx.stroke();
          }
        });
      });

      animationId = requestAnimationFrame(animate);
    };

    animate();

    return () => {
      window.removeEventListener('resize', resize);
      cancelAnimationFrame(animationId);
    };
  }, [theme]);

  // Progress animation
  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          setTimeout(() => setIsComplete(true), 500);
          return 100;
        }
        const increment = Math.random() * 12 + 3;
        return Math.min(prev + increment, 100);
      });
    }, 150);

    return () => clearInterval(interval);
  }, []);

  if (isComplete) return null;

  return (
    <div
      className={`fixed inset-0 z-[100] flex items-center justify-center transition-opacity duration-500 ${
        progress >= 100 ? 'opacity-0' : 'opacity-100'
      } ${theme === 'light' ? 'bg-white' : 'bg-black'}`}
    >
      {/* Animated particle background */}
      <canvas
        ref={canvasRef}
        className="absolute inset-0 pointer-events-none"
        style={{ willChange: 'transform' }}
      />

      {/* Loader content */}
      <div className="relative z-10">
        {/* Logo with subtle pulsing animation */}
        <div
          className={`border px-6 py-4 mb-8 transition-all duration-300 ${
            theme === 'light'
              ? 'border-black/20 bg-white shadow-lg'
              : 'border-white/10 bg-black shadow-2xl shadow-white/5'
          }`}
          style={{
            animation: 'logoFloat 3s ease-in-out infinite',
          }}
        >
          <span
            className={`text-2xl font-bold tracking-tighter uppercase leading-none block ${
              theme === 'light' ? 'text-black' : 'text-white'
            }`}
            style={{ letterSpacing: '-0.02em' }}
          >
            DOT
            <br />
            .REPUTE
          </span>
        </div>

        {/* Advanced progress indicator */}
        <div className="relative w-[320px]">
          {/* Progress bar with gradient */}
          <div
            className={`border h-3 overflow-hidden relative ${
              theme === 'light'
                ? 'border-black/20 bg-gray-50'
                : 'border-white/10 bg-white/5'
            }`}
          >
            {/* Background grid pattern */}
            <div
              className="absolute inset-0 opacity-20"
              style={{
                backgroundImage: theme === 'light'
                  ? 'linear-gradient(90deg, rgba(0,0,0,0.1) 1px, transparent 1px), linear-gradient(rgba(0,0,0,0.1) 1px, transparent 1px)'
                  : 'linear-gradient(90deg, rgba(255,255,255,0.1) 1px, transparent 1px), linear-gradient(rgba(255,255,255,0.1) 1px, transparent 1px)',
                backgroundSize: '10px 10px',
              }}
            />

            {/* Animated progress fill */}
            <div
              className={`h-full relative transition-all duration-300 ease-out ${
                theme === 'light'
                  ? 'bg-gradient-to-r from-gray-900 via-gray-700 to-gray-900'
                  : 'bg-gradient-to-r from-white via-gray-200 to-white'
              }`}
              style={{
                width: `${progress}%`,
                backgroundSize: '200% 100%',
                animation: 'gradientShift 2s linear infinite',
              }}
            >
              {/* Glow effect */}
              <div
                className={`absolute right-0 top-0 bottom-0 w-8 ${
                  theme === 'light'
                    ? 'bg-gradient-to-l from-black/30 to-transparent'
                    : 'bg-gradient-to-l from-white/50 to-transparent'
                }`}
                style={{ filter: 'blur(4px)' }}
              />
            </div>
          </div>

          {/* Status text */}
          <div className="mt-4 flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span
                className={`text-[10px] uppercase tracking-wider font-mono ${
                  theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                }`}
              >
                Initializing
              </span>
              {/* Animated loading dots */}
              <div className="flex gap-1">
                {[0, 1, 2].map((i) => (
                  <div
                    key={i}
                    className={`w-1 h-1 rounded-full ${
                      theme === 'light' ? 'bg-black/40' : 'bg-white/40'
                    }`}
                    style={{
                      animation: `dotPulse 1.4s ease-in-out ${i * 0.2}s infinite`,
                    }}
                  />
                ))}
              </div>
            </div>
            <span
              className={`text-sm font-mono font-bold tabular-nums ${
                theme === 'light' ? 'text-black' : 'text-white'
              }`}
            >
              {Math.floor(progress)}%
            </span>
          </div>

          {/* Loading message */}
          <p
            className={`mt-6 text-center text-[10px] uppercase tracking-wider font-mono ${
              theme === 'light' ? 'text-gray-500' : 'text-gray-600'
            }`}
          >
            Rust-Powered Reputation System
          </p>
        </div>

        {/* Premium corner accents */}
        <div
          className={`absolute -top-6 -left-6 w-12 h-12 border-l-2 border-t-2 ${
            theme === 'light' ? 'border-black/20' : 'border-white/20'
          }`}
          style={{ animation: 'cornerFade 2s ease-in-out infinite' }}
        />
        <div
          className={`absolute -top-6 -right-6 w-12 h-12 border-r-2 border-t-2 ${
            theme === 'light' ? 'border-black/20' : 'border-white/20'
          }`}
          style={{ animation: 'cornerFade 2s ease-in-out 0.5s infinite' }}
        />
        <div
          className={`absolute -bottom-6 -left-6 w-12 h-12 border-l-2 border-b-2 ${
            theme === 'light' ? 'border-black/20' : 'border-white/20'
          }`}
          style={{ animation: 'cornerFade 2s ease-in-out 1s infinite' }}
        />
        <div
          className={`absolute -bottom-6 -right-6 w-12 h-12 border-r-2 border-b-2 ${
            theme === 'light' ? 'border-black/20' : 'border-white/20'
          }`}
          style={{ animation: 'cornerFade 2s ease-in-out 1.5s infinite' }}
        />
      </div>

      {/* Animations */}
      <style jsx>{`
        @keyframes logoFloat {
          0%, 100% {
            transform: translateY(0);
          }
          50% {
            transform: translateY(-8px);
          }
        }

        @keyframes gradientShift {
          0% {
            background-position: 0% 0%;
          }
          100% {
            background-position: 200% 0%;
          }
        }

        @keyframes dotPulse {
          0%, 100% {
            opacity: 0.3;
            transform: scale(1);
          }
          50% {
            opacity: 1;
            transform: scale(1.3);
          }
        }

        @keyframes cornerFade {
          0%, 100% {
            opacity: 0.3;
          }
          50% {
            opacity: 1;
          }
        }
      `}</style>
    </div>
  );
}
