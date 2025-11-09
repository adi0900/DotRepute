/**
 * DotRepute Authentication Page
 * Venture-style Login and Sign Up
 */

'use client';

import { useEffect, useState } from 'react';
import { VentureNavbar } from '@/components/venture-navbar';
import Link from 'next/link';
import {
  Mail,
  Lock,
  User,
  Eye,
  EyeOff,
  Github,
  Chrome,
  ArrowRight,
  Shield,
  CheckCircle2
} from 'lucide-react';

export default function AuthPage() {
  const [theme, setTheme] = useState<'light' | 'dark'>('dark');
  const [mounted, setMounted] = useState(false);
  const [authMode, setAuthMode] = useState<'login' | 'signup'>('login');
  const [showPassword, setShowPassword] = useState(false);
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    name: '',
    confirmPassword: ''
  });

  // Initialize theme from localStorage on mount
  useEffect(() => {
    setMounted(true);
    const savedTheme = localStorage.getItem('theme') as 'light' | 'dark' | null;
    const initialTheme = savedTheme || 'dark';
    setTheme(initialTheme);
    document.documentElement.classList.toggle('dark', initialTheme === 'dark');
  }, []);

  // Toggle theme and persist to localStorage
  const toggleTheme = () => {
    const newTheme = theme === 'dark' ? 'light' : 'dark';
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    document.documentElement.classList.toggle('dark', newTheme === 'dark');
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Handle authentication logic here
    console.log('Form submitted:', formData);
    // Redirect to dashboard after successful auth
    window.location.href = '/dashboard';
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData(prev => ({
      ...prev,
      [e.target.name]: e.target.value
    }));
  };

  // Prevent flash of unstyled content
  if (!mounted) return null;

  return (
    <div className={`relative min-h-screen transition-colors duration-300 ${
      theme === 'light' ? 'bg-white text-black' : 'bg-black text-white'
    }`}>
      {/* Venture Navbar */}
      <VentureNavbar theme={theme} onToggleTheme={toggleTheme} currentPath="/auth" showAuthButtons={false} />

      {/* Main Auth Content */}
      <main className="min-h-screen flex items-center justify-center px-6 py-24">
        <div className="w-full max-w-6xl grid lg:grid-cols-2 gap-12 items-center">
          {/* Left Side - Branding & Info */}
          <div className="space-y-8">
            {/* Logo Badge */}
            <div className={`inline-flex border px-4 py-2 ${
              theme === 'light'
                ? 'border-black/20 bg-white'
                : 'border-white/10 bg-black/40'
            }`}>
              <span className={`text-[10px] uppercase tracking-wider font-mono ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-400'
              }`}>
                Rust-powered reputation system
              </span>
            </div>

            <h1 className="text-5xl md:text-6xl font-bold leading-tight tracking-tight">
              Welcome to<br />
              <span className="font-normal">DotRepute</span>
            </h1>

            <p className={`text-lg leading-relaxed max-w-xl ${
              theme === 'light' ? 'text-gray-700' : 'text-gray-400'
            }`}>
              Build transparent reputation systems for the Polkadot ecosystem.
              Track identity, governance, staking, and activity signals in one unified platform.
            </p>

            {/* Features List */}
            <div className="space-y-4 pt-4">
              <FeatureItem
                theme={theme}
                icon={<Shield className="w-5 h-5" />}
                title="Secure & Verifiable"
                description="On-chain identity verification"
              />
              <FeatureItem
                theme={theme}
                icon={<CheckCircle2 className="w-5 h-5" />}
                title="Transparent Scoring"
                description="Clear reputation metrics"
              />
              <FeatureItem
                theme={theme}
                icon={<ArrowRight className="w-5 h-5" />}
                title="Real-time Updates"
                description="Track your reputation live"
              />
            </div>
          </div>

          {/* Right Side - Auth Form */}
          <div className={`border p-8 md:p-12 ${
            theme === 'light'
              ? 'border-black/10 bg-white'
              : 'border-white/5 bg-black/20'
          }`}>
            {/* Tab Switcher */}
            <div className={`flex border mb-8 ${
              theme === 'light' ? 'border-black/10' : 'border-white/5'
            }`}>
              <button
                onClick={() => setAuthMode('login')}
                className={`flex-1 px-6 py-3 text-sm uppercase tracking-wider font-medium transition-colors ${
                  authMode === 'login'
                    ? theme === 'light'
                      ? 'border-b-2 border-black bg-white text-black'
                      : 'border-b-2 border-white bg-black text-white'
                    : theme === 'light'
                      ? 'text-gray-600 hover:bg-gray-50'
                      : 'text-gray-500 hover:bg-white/5'
                }`}
              >
                Login
              </button>
              <button
                onClick={() => setAuthMode('signup')}
                className={`flex-1 px-6 py-3 text-sm uppercase tracking-wider font-medium transition-colors ${
                  authMode === 'signup'
                    ? theme === 'light'
                      ? 'border-b-2 border-black bg-white text-black'
                      : 'border-b-2 border-white bg-black text-white'
                    : theme === 'light'
                      ? 'text-gray-600 hover:bg-gray-50'
                      : 'text-gray-500 hover:bg-white/5'
                }`}
              >
                Sign Up
              </button>
            </div>

            {/* Auth Form */}
            <form onSubmit={handleSubmit} className="space-y-6">
              {/* Name Field - Only for Sign Up */}
              {authMode === 'signup' && (
                <div className="space-y-2">
                  <label className={`block text-xs uppercase tracking-wider font-mono ${
                    theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                  }`}>
                    Full Name
                  </label>
                  <div className="relative">
                    <div className="absolute left-4 top-1/2 -translate-y-1/2">
                      <User className={`w-4 h-4 ${
                        theme === 'light' ? 'text-gray-400' : 'text-gray-600'
                      }`} />
                    </div>
                    <input
                      type="text"
                      name="name"
                      value={formData.name}
                      onChange={handleInputChange}
                      placeholder="Enter your full name"
                      required
                      className={`w-full border pl-12 pr-4 py-3 text-sm bg-transparent focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all ${
                        theme === 'light'
                          ? 'border-black/20 placeholder-gray-500 focus:ring-black/20 focus:border-black/40'
                          : 'border-white/10 placeholder-gray-600 focus:ring-white/20 focus:border-white/30'
                      }`}
                    />
                  </div>
                </div>
              )}

              {/* Email Field */}
              <div className="space-y-2">
                <label className={`block text-xs uppercase tracking-wider font-mono ${
                  theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                }`}>
                  Email Address
                </label>
                <div className="relative">
                  <div className="absolute left-4 top-1/2 -translate-y-1/2">
                    <Mail className={`w-4 h-4 ${
                      theme === 'light' ? 'text-gray-400' : 'text-gray-600'
                    }`} />
                  </div>
                  <input
                    type="email"
                    name="email"
                    value={formData.email}
                    onChange={handleInputChange}
                    placeholder="your@email.com"
                    required
                    className={`w-full border pl-12 pr-4 py-3 text-sm bg-transparent focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all ${
                      theme === 'light'
                        ? 'border-black/20 placeholder-gray-500 focus:ring-black/20 focus:border-black/40'
                        : 'border-white/10 placeholder-gray-600 focus:ring-white/20 focus:border-white/30'
                    }`}
                  />
                </div>
              </div>

              {/* Password Field */}
              <div className="space-y-2">
                <label className={`block text-xs uppercase tracking-wider font-mono ${
                  theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                }`}>
                  Password
                </label>
                <div className="relative">
                  <div className="absolute left-4 top-1/2 -translate-y-1/2">
                    <Lock className={`w-4 h-4 ${
                      theme === 'light' ? 'text-gray-400' : 'text-gray-600'
                    }`} />
                  </div>
                  <input
                    type={showPassword ? 'text' : 'password'}
                    name="password"
                    value={formData.password}
                    onChange={handleInputChange}
                    placeholder="Enter your password"
                    required
                    className={`w-full border pl-12 pr-12 py-3 text-sm bg-transparent focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all ${
                      theme === 'light'
                        ? 'border-black/20 placeholder-gray-500 focus:ring-black/20 focus:border-black/40'
                        : 'border-white/10 placeholder-gray-600 focus:ring-white/20 focus:border-white/30'
                    }`}
                  />
                  <button
                    type="button"
                    onClick={() => setShowPassword(!showPassword)}
                    className="absolute right-4 top-1/2 -translate-y-1/2"
                  >
                    {showPassword ? (
                      <EyeOff className={`w-4 h-4 ${
                        theme === 'light' ? 'text-gray-400' : 'text-gray-600'
                      }`} />
                    ) : (
                      <Eye className={`w-4 h-4 ${
                        theme === 'light' ? 'text-gray-400' : 'text-gray-600'
                      }`} />
                    )}
                  </button>
                </div>
              </div>

              {/* Confirm Password - Only for Sign Up */}
              {authMode === 'signup' && (
                <div className="space-y-2">
                  <label className={`block text-xs uppercase tracking-wider font-mono ${
                    theme === 'light' ? 'text-gray-600' : 'text-gray-500'
                  }`}>
                    Confirm Password
                  </label>
                  <div className="relative">
                    <div className="absolute left-4 top-1/2 -translate-y-1/2">
                      <Lock className={`w-4 h-4 ${
                        theme === 'light' ? 'text-gray-400' : 'text-gray-600'
                      }`} />
                    </div>
                    <input
                      type={showPassword ? 'text' : 'password'}
                      name="confirmPassword"
                      value={formData.confirmPassword}
                      onChange={handleInputChange}
                      placeholder="Confirm your password"
                      required
                      className={`w-full border pl-12 pr-4 py-3 text-sm bg-transparent focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all ${
                        theme === 'light'
                          ? 'border-black/20 placeholder-gray-500 focus:ring-black/20 focus:border-black/40'
                          : 'border-white/10 placeholder-gray-600 focus:ring-white/20 focus:border-white/30'
                      }`}
                    />
                  </div>
                </div>
              )}

              {/* Remember Me / Forgot Password */}
              {authMode === 'login' && (
                <div className="flex items-center justify-between">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="checkbox"
                      className={`w-4 h-4 border transition-colors ${
                        theme === 'light'
                          ? 'border-black/20 bg-white'
                          : 'border-white/10 bg-black/40'
                      }`}
                    />
                    <span className={`text-sm ${
                      theme === 'light' ? 'text-gray-700' : 'text-gray-400'
                    }`}>
                      Remember me
                    </span>
                  </label>
                  <a
                    href="#"
                    className={`text-sm transition-colors ${
                      theme === 'light'
                        ? 'text-gray-700 hover:text-black'
                        : 'text-gray-400 hover:text-white'
                    }`}
                  >
                    Forgot password?
                  </a>
                </div>
              )}

              {/* Submit Button */}
              <button
                type="submit"
                className={`w-full border px-6 py-4 text-sm uppercase tracking-wider font-medium transition-all ${
                  theme === 'light'
                    ? 'border-black/20 bg-black text-white hover:bg-black/90'
                    : 'border-white/10 bg-white text-black hover:bg-white/90'
                }`}
              >
                {authMode === 'login' ? 'Login to Dashboard' : 'Create Account'}
                <ArrowRight className="w-4 h-4 inline ml-2" />
              </button>
            </form>

            {/* Divider */}
            <div className="relative my-8">
              <div className={`absolute inset-0 flex items-center ${
                theme === 'light' ? 'border-black/10' : 'border-white/5'
              }`}>
                <div className={`w-full border-t ${
                  theme === 'light' ? 'border-black/10' : 'border-white/5'
                }`} />
              </div>
              <div className="relative flex justify-center">
                <span className={`px-4 text-xs uppercase tracking-wider font-mono ${
                  theme === 'light' ? 'bg-white text-gray-600' : 'bg-black/20 text-gray-500'
                }`}>
                  Or continue with
                </span>
              </div>
            </div>

            {/* Social Login */}
            <div className="grid grid-cols-2 gap-3">
              <button
                type="button"
                className={`border px-4 py-3 text-sm transition-colors flex items-center justify-center gap-2 ${
                  theme === 'light'
                    ? 'border-black/20 hover:bg-black/5'
                    : 'border-white/10 hover:bg-white/5'
                }`}
              >
                <Github className="w-4 h-4" />
                GitHub
              </button>
              <button
                type="button"
                className={`border px-4 py-3 text-sm transition-colors flex items-center justify-center gap-2 ${
                  theme === 'light'
                    ? 'border-black/20 hover:bg-black/5'
                    : 'border-white/10 hover:bg-white/5'
                }`}
              >
                <Chrome className="w-4 h-4" />
                Google
              </button>
            </div>

            {/* Terms */}
            {authMode === 'signup' && (
              <p className={`text-xs text-center mt-6 ${
                theme === 'light' ? 'text-gray-600' : 'text-gray-500'
              }`}>
                By signing up, you agree to our{' '}
                <a href="#" className={`underline ${
                  theme === 'light' ? 'hover:text-black' : 'hover:text-white'
                }`}>
                  Terms of Service
                </a>{' '}
                and{' '}
                <a href="#" className={`underline ${
                  theme === 'light' ? 'hover:text-black' : 'hover:text-white'
                }`}>
                  Privacy Policy
                </a>
              </p>
            )}
          </div>
        </div>
      </main>
    </div>
  );
}

// Feature Item Component
function FeatureItem({
  theme,
  icon,
  title,
  description
}: {
  theme: 'light' | 'dark';
  icon: React.ReactNode;
  title: string;
  description: string;
}) {
  return (
    <div className="flex items-start gap-4">
      <div className={`border p-2 ${
        theme === 'light'
          ? 'border-black/20 bg-white'
          : 'border-white/10 bg-black/40'
      }`}>
        {icon}
      </div>
      <div>
        <div className="font-medium mb-1">{title}</div>
        <div className={`text-sm ${
          theme === 'light' ? 'text-gray-600' : 'text-gray-500'
        }`}>
          {description}
        </div>
      </div>
    </div>
  );
}
