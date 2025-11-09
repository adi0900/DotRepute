/**
 * Enhanced WebGL-Inspired Background Animation
 * Design Reference: Fumadocs radial orange gradient with dot matrix
 * Performance-optimized Canvas 2D alternative to WebGL
 * Unicorn Studio style: https://unicorn.studio/
 */

'use client';

import { useEffect, useRef, useState } from 'react';

export function EnhancedWebGLBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d', { alpha: true, desynchronized: true });
    if (!ctx) return;

    let animationId: number;
    let width: number, height: number;

    // Responsive canvas sizing with device pixel ratio
    const resize = () => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2); // Cap at 2x for performance
      width = window.innerWidth;
      height = window.innerHeight;

      canvas.width = width * dpr;
      canvas.height = height * dpr;
      canvas.style.width = `${width}px`;
      canvas.style.height = `${height}px`;

      ctx.scale(dpr, dpr);
    };

    resize();
    window.addEventListener('resize', resize);

    // Enhanced Dot Matrix System - Fumadocs inspired (optimized)
    class DotMatrix {
      dots: Dot[] = [];
      spacing = 60; // Increased spacing for fewer dots
      maxRadius = 3;

      constructor() {
        this.generate();
      }

      generate() {
        this.dots = [];
        const cols = Math.ceil(width / this.spacing) + 2;
        const rows = Math.ceil(height / this.spacing) + 2;

        for (let i = 0; i < cols; i++) {
          for (let j = 0; j < rows; j++) {
            const x = i * this.spacing - this.spacing;
            const y = j * this.spacing - this.spacing;
            const delay = Math.random() * Math.PI * 2;
            this.dots.push(new Dot(x, y, delay));
          }
        }
      }

      update(time: number) {
        this.dots.forEach(dot => dot.update(time));
      }

      draw() {
        this.dots.forEach(dot => dot.draw(ctx, width, height));
      }
    }

    class Dot {
      baseX: number;
      baseY: number;
      x: number;
      y: number;
      delay: number;
      radius: number;

      constructor(x: number, y: number, delay: number) {
        this.baseX = x;
        this.baseY = y;
        this.x = x;
        this.y = y;
        this.delay = delay;
        this.radius = 1;
      }

      update(time: number) {
        // Subtle wave motion
        const wave = Math.sin(time * 0.001 + this.delay) * 3;
        this.x = this.baseX + wave;
        this.y = this.baseY + Math.cos(time * 0.0008 + this.delay) * 2;

        // Pulsing radius
        this.radius = 1 + Math.sin(time * 0.002 + this.delay) * 0.5;
      }

      draw(ctx: CanvasRenderingContext2D, canvasWidth: number, canvasHeight: number) {
        // Calculate distance from right-bottom side (where the large gradient orb is)
        const centerX = canvasWidth * 0.80;
        const centerY = canvasHeight * 0.65;
        const dx = this.x - centerX;
        const dy = this.y - centerY;
        const distance = Math.sqrt(dx * dx + dy * dy);
        const maxDistance = Math.max(canvasWidth, canvasHeight) * 0.7;

        // Opacity based on distance from gradient center
        const opacity = Math.max(0, 1 - distance / maxDistance) * 0.7;

        if (opacity > 0.05) {
          // Color based on position - orange to red gradient
          const hue = 15 + (distance / maxDistance) * 20; // 15-35 (orange-red range)
          const saturation = 100;
          const lightness = 45 + (1 - distance / maxDistance) * 25;

          ctx.beginPath();
          ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
          ctx.fillStyle = `hsla(${hue}, ${saturation}%, ${lightness}%, ${opacity})`;
          ctx.fill();
        }
      }
    }

    // Large gradient orbs - Fumadocs style
    class GradientOrb {
      x: number;
      y: number;
      targetX: number;
      targetY: number;
      radius: number;
      color: string;
      hue: number;

      constructor(x: number, y: number, radius: number, hue: number) {
        this.x = x;
        this.y = y;
        this.targetX = x;
        this.targetY = y;
        this.radius = radius;
        this.hue = hue;
        this.color = `hsla(${hue}, 100%, 50%, 0.15)`;
      }

      update() {
        // Very subtle movement for the main orb
        this.x += (this.targetX - this.x) * 0.005;
        this.y += (this.targetY - this.y) * 0.005;

        // Set new target occasionally (very subtle range)
        if (Math.random() < 0.002) {
          this.targetX = width * (0.75 + Math.random() * 0.15);
          this.targetY = height * (0.55 + Math.random() * 0.25);
        }
      }

      draw() {
        const gradient = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, this.radius);
        // More vibrant orange gradient matching Fumadocs
        gradient.addColorStop(0, `hsla(${this.hue}, 100%, 60%, 0.25)`);
        gradient.addColorStop(0.3, `hsla(${this.hue + 5}, 100%, 55%, 0.18)`);
        gradient.addColorStop(0.6, `hsla(${this.hue + 15}, 95%, 50%, 0.08)`);
        gradient.addColorStop(1, 'rgba(0, 0, 0, 0)');

        ctx.fillStyle = gradient;
        ctx.fillRect(this.x - this.radius, this.y - this.radius, this.radius * 2, this.radius * 2);
      }
    }

    // Initialize elements
    const dotMatrix = new DotMatrix();
    // Single large orb positioned on the right-bottom side (avoid globe overlap)
    const orbs = [
      new GradientOrb(width * 0.80, height * 0.65, Math.min(width, height) * 0.5, 20), // Large orange orb
    ];

    // Mouse interaction
    let mouseX = width / 2;
    let mouseY = height / 2;
    let mouseInfluence = 0;

    const handleMouseMove = (e: MouseEvent) => {
      mouseX = e.clientX;
      mouseY = e.clientY;
      mouseInfluence = 1;
    };

    const handleMouseLeave = () => {
      mouseInfluence = 0;
    };

    canvas.addEventListener('mousemove', handleMouseMove);
    canvas.addEventListener('mouseleave', handleMouseLeave);

    // Animation loop with performance optimization
    let lastTime = 0;
    let frameCount = 0;
    const animate = (currentTime: number) => {
      if (!isVisible) {
        animationId = requestAnimationFrame(animate);
        return;
      }

      const deltaTime = currentTime - lastTime;

      // Throttle to 30fps for better performance
      if (deltaTime < 33) {
        animationId = requestAnimationFrame(animate);
        return;
      }

      lastTime = currentTime;
      frameCount++;

      // Clear with black background
      ctx.fillStyle = 'rgba(0, 0, 0, 1)';
      ctx.fillRect(0, 0, width, height);

      // Draw gradient orbs first (background layer)
      orbs.forEach(orb => {
        orb.update();
        orb.draw();
      });

      // Draw dot matrix on top
      dotMatrix.update(currentTime);
      dotMatrix.draw();

      // Smooth vignette effect (cached, only recreate every 60 frames)
      if (frameCount % 60 === 0 || !vignetteCache) {
        vignetteCache = ctx.createRadialGradient(
          width / 2,
          height / 2,
          0,
          width / 2,
          height / 2,
          Math.max(width, height) * 0.8
        );
        vignetteCache.addColorStop(0, 'rgba(0, 0, 0, 0)');
        vignetteCache.addColorStop(0.7, 'rgba(0, 0, 0, 0.3)');
        vignetteCache.addColorStop(1, 'rgba(0, 0, 0, 0.85)');
      }

      ctx.fillStyle = vignetteCache;
      ctx.fillRect(0, 0, width, height);

      // Reduce grain frequency for performance
      if (frameCount % 20 === 0) {
        ctx.fillStyle = `rgba(255, 255, 255, 0.01)`;
        for (let i = 0; i < 25; i++) { // Reduced from 50 to 25
          const x = Math.random() * width;
          const y = Math.random() * height;
          ctx.fillRect(x, y, 1, 1);
        }
      }

      mouseInfluence *= 0.95;

      animationId = requestAnimationFrame(animate);
    };

    let vignetteCache: CanvasGradient;

    animate(0);

    // Visibility API for performance
    const handleVisibilityChange = () => {
      setIsVisible(!document.hidden);
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);

    // Cleanup
    return () => {
      window.removeEventListener('resize', resize);
      canvas.removeEventListener('mousemove', handleMouseMove);
      canvas.removeEventListener('mouseleave', handleMouseLeave);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      cancelAnimationFrame(animationId);
    };
  }, [isVisible]);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full pointer-events-none"
      style={{
        filter: 'blur(80px) saturate(1.2)',
        opacity: 0.9,
        mixBlendMode: 'normal',
        willChange: 'transform',
        transform: 'translateZ(0)',
      }}
    />
  );
}
