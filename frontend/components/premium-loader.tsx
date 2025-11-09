/**
 * Premium Loading Effect - Venture Style
 * Consistent design for both light and dark modes
 * Matches navbar boxed aesthetic
 */

'use client';

import { useEffect, useState } from 'react';

interface PremiumLoaderProps {
  theme?: 'light' | 'dark';
}

export function PremiumLoader({ theme = 'dark' }: PremiumLoaderProps) {
  const [progress, setProgress] = useState(0);
  const [isComplete, setIsComplete] = useState(false);

  useEffect(() => {
    // Simulate loading progress
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          setTimeout(() => setIsComplete(true), 500);
          return 100;
        }
        // Variable speed for realistic feel
        const increment = Math.random() * 15 + 5;
        return Math.min(prev + increment, 100);
      });
    }, 200);

    return () => clearInterval(interval);
  }, []);

  if (isComplete) return null;

  return (
    <div
      className={`fixed inset-0 z-[100] flex items-center justify-center transition-opacity duration-500 ${
        progress >= 100 ? 'opacity-0' : 'opacity-100'
      } ${theme === 'light' ? 'bg-white' : 'bg-black'}`}
    >
      {/* Premium loader container - Venture boxed style */}
      <div className="relative">
        {/* Logo/Brand */}
        <div
          className={`border px-6 py-4 mb-8 transition-all duration-300 ${
            theme === 'light'
              ? 'border-black/20 bg-white'
              : 'border-white/10 bg-black'
          }`}
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

        {/* Loading progress bar - Venture style */}
        <div className="relative w-[280px]">
          {/* Progress bar container */}
          <div
            className={`border h-2 overflow-hidden ${
              theme === 'light'
                ? 'border-black/20 bg-gray-50'
                : 'border-white/10 bg-white/5'
            }`}
          >
            {/* Progress fill with shimmer effect */}
            <div
              className={`h-full relative transition-all duration-300 ease-out ${
                theme === 'light' ? 'bg-black' : 'bg-white'
              }`}
              style={{ width: `${progress}%` }}
            >
              {/* Shimmer overlay */}
              <div
                className={`absolute inset-0 animate-shimmer ${
                  theme === 'light'
                    ? 'bg-gradient-to-r from-transparent via-white/20 to-transparent'
                    : 'bg-gradient-to-r from-transparent via-white/30 to-transparent'
                }`}
                style={{
                  backgroundSize: '200% 100%',
                  animation: 'shimmer 2s infinite',
                }}
              />
            </div>
          </div>

          {/* Progress percentage - monospace */}
          <div className="mt-4 flex items-center justify-between">
            <span
              className={`text-[10px] uppercase tracking-wider font-mono ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}
            >
              Loading
            </span>
            <span
              className={`text-[10px] font-mono tabular-nums ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}
            >
              {Math.floor(progress)}%
            </span>
          </div>
        </div>

        {/* Animated dots */}
        <div className="mt-6 flex gap-2 justify-center">
          {[0, 1, 2].map((i) => (
            <div
              key={i}
              className={`w-1.5 h-1.5 border transition-all ${
                theme === 'light'
                  ? 'border-black/20 bg-black/20'
                  : 'border-white/20 bg-white/20'
              }`}
              style={{
                animation: `pulse 1.5s ease-in-out ${i * 0.2}s infinite`,
              }}
            />
          ))}
        </div>

        {/* Decorative corner elements - Venture style */}
        <div
          className={`absolute -top-4 -left-4 w-8 h-8 border-l border-t ${
            theme === 'light' ? 'border-black/10' : 'border-white/10'
          }`}
        />
        <div
          className={`absolute -top-4 -right-4 w-8 h-8 border-r border-t ${
            theme === 'light' ? 'border-black/10' : 'border-white/10'
          }`}
        />
        <div
          className={`absolute -bottom-4 -left-4 w-8 h-8 border-l border-b ${
            theme === 'light' ? 'border-black/10' : 'border-white/10'
          }`}
        />
        <div
          className={`absolute -bottom-4 -right-4 w-8 h-8 border-r border-b ${
            theme === 'light' ? 'border-black/10' : 'border-white/10'
          }`}
        />
      </div>

      <style jsx>{`
        @keyframes shimmer {
          0% {
            background-position: -200% 0;
          }
          100% {
            background-position: 200% 0;
          }
        }

        @keyframes pulse {
          0%,
          100% {
            opacity: 0.3;
            transform: scale(1);
          }
          50% {
            opacity: 1;
            transform: scale(1.2);
          }
        }
      `}</style>
    </div>
  );
}
