/**
 * Venture-style Navbar Component
 * Reusable navigation with theme support
 */

'use client';

import { Sun, Moon, Menu, Home, LogIn, UserPlus, Users, FileText, Layers, LogOut } from 'lucide-react';
import { Button } from '@/components/ui/button';
import Link from 'next/link';

interface VentureNavbarProps {
  theme: 'light' | 'dark';
  onToggleTheme: () => void;
  currentPath?: string;
  showAuthButtons?: boolean;
  isLoggedIn?: boolean;
  onLogout?: () => void;
}

export function VentureNavbar({ theme, onToggleTheme, currentPath = '/', showAuthButtons = true, isLoggedIn = false, onLogout }: VentureNavbarProps) {
  return (
    <nav className={`fixed top-0 left-0 right-0 z-50 border-b transition-colors duration-300 ${
      theme === 'light'
        ? 'bg-white border-black/10'
        : 'bg-black border-white/5'
    }`}>
      <div className="mx-auto px-8 py-4">
        <div className="flex items-center justify-between">
          {/* Logo - Venture style boxed design */}
          <div className="flex items-center gap-8">
            <Link href="/">
              <div className={`border px-4 py-3 transition-colors cursor-pointer hover:border-opacity-40 ${
                theme === 'light'
                  ? 'border-black/20 bg-white hover:bg-black/5'
                  : 'border-white/10 bg-black hover:bg-white/5'
              }`}>
                <span className="text-xl font-bold tracking-tighter uppercase leading-none block" style={{ letterSpacing: '-0.02em' }}>
                  DOT<br/>.REPUTE
                </span>
              </div>
            </Link>

            {/* Venture-style metadata */}
            <div className={`hidden lg:flex flex-col text-[10px] uppercase tracking-wider font-mono ${
              theme === 'light' ? 'text-gray-600' : 'text-gray-500'
            }`}>
              <span>Polkadot / Ver-2.0</span>
              <span>VENTURE</span>
            </div>
          </div>

          {/* Center Navigation */}
          <div className="hidden md:flex items-center gap-6">
            {/* Home Link */}
            <Link
              href="/"
              className={`flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                currentPath === '/'
                  ? theme === 'light'
                    ? 'border-black/40 bg-black/10 text-black'
                    : 'border-white/30 bg-white/10 text-white'
                  : theme === 'light'
                    ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
                    : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
              }`}
            >
              <Home className="w-4 h-4 mr-2" />
              Home
            </Link>

            {/* Docs Link */}
            <Link
              href="/docs"
              className={`flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                currentPath === '/docs'
                  ? theme === 'light'
                    ? 'border-black/40 bg-black/10 text-black'
                    : 'border-white/30 bg-white/10 text-white'
                  : theme === 'light'
                    ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
                    : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
              }`}
            >
              <FileText className="w-4 h-4 mr-2" />
              Docs
            </Link>

            {/* Resources Link */}
            <Link
              href="/resources"
              className={`flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                currentPath === '/resources'
                  ? theme === 'light'
                    ? 'border-black/40 bg-black/10 text-black'
                    : 'border-white/30 bg-white/10 text-white'
                  : theme === 'light'
                    ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
                    : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
              }`}
            >
              <Layers className="w-4 h-4 mr-2" />
              Resources
            </Link>
          </div>

          {/* Right side - Theme Toggle, Login/Signup */}
          <div className="flex items-center gap-4">
            {/* Theme Toggle Button - Venture boxed style */}
            <button
              onClick={onToggleTheme}
              aria-label={theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
              className={`hidden md:flex items-center justify-center border px-4 py-2 transition-all focus:outline-none focus:ring-2 focus:ring-offset-2 ${
                theme === 'light'
                  ? 'border-black/20 hover:bg-black/5 focus:ring-black/20'
                  : 'border-white/10 hover:bg-white/5 focus:ring-white/20'
              }`}
            >
              {theme === 'dark' ? (
                <Sun className="w-4 h-4 text-yellow-400" aria-hidden="true" />
              ) : (
                <Moon className="w-4 h-4 text-gray-700" aria-hidden="true" />
              )}
            </button>

            {/* Auth Buttons - Show Login/Signup when logged out, Logout when logged in */}
            {showAuthButtons && !isLoggedIn && (
              <>
                {/* Login Button */}
                <Link
                  href="/auth"
                  className={`hidden md:flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                    theme === 'light'
                      ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
                      : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
                  }`}
                >
                  <LogIn className="w-4 h-4 mr-2" />
                  Login
                </Link>

                {/* Sign Up Button - Primary Style */}
                <Link
                  href="/auth"
                  className={`hidden md:flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                    theme === 'light'
                      ? 'border-black/20 bg-black text-white hover:bg-black/90'
                      : 'border-white/10 bg-white text-black hover:bg-white/90'
                  }`}
                >
                  <UserPlus className="w-4 h-4 mr-2" />
                  Sign Up
                </Link>
              </>
            )}

            {/* Logout Button - Show when logged in */}
            {isLoggedIn && (
              <button
                onClick={onLogout}
                className={`hidden md:flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                  theme === 'light'
                    ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
                    : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
                }`}
              >
                <LogOut className="w-4 h-4 mr-2" />
                Logout
              </button>
            )}

            {/* Dashboard Button - Only show when auth buttons are hidden (on auth page) */}
            {!showAuthButtons && (
              <Link
                href="/dashboard"
                className={`hidden md:flex items-center border px-4 py-2 text-xs uppercase tracking-wider font-medium transition-colors ${
                  currentPath === '/dashboard'
                    ? theme === 'light'
                      ? 'border-black/40 bg-black/10 text-black'
                      : 'border-white/30 bg-white/10 text-white'
                    : theme === 'light'
                      ? 'border-black/20 hover:bg-black/5 text-gray-700 hover:text-black'
                      : 'border-white/10 hover:bg-white/5 text-gray-400 hover:text-white'
                }`}
              >
                Dashboard
              </Link>
            )}

            <Button variant="ghost" size="sm" className={`md:hidden border px-3 ${
              theme === 'light'
                ? 'border-black/20 hover:bg-black/5'
                : 'border-white/10 hover:bg-white/5'
            }`}>
              <Menu className="w-5 h-5" />
            </Button>
          </div>
        </div>
      </div>
    </nav>
  );
}
