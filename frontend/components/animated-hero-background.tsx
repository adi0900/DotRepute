/**
 * Enhanced WebGL-style Animated Hero Background
 * Design Reference: Fumadocs orange gradient with dot matrix pattern
 * Inspired by Unicorn Studio WebGL generators
 */

'use client';

import { useEffect, useRef } from 'react';

export function AnimatedHeroBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    // Set canvas size with device pixel ratio for sharp rendering
    const setCanvasSize = () => {
      const dpr = window.devicePixelRatio || 1;
      const rect = canvas.getBoundingClientRect();

      canvas.width = rect.width * dpr;
      canvas.height = rect.height * dpr;

      ctx.scale(dpr, dpr);

      canvas.style.width = rect.width + 'px';
      canvas.style.height = rect.height + 'px';
    };
    setCanvasSize();
    window.addEventListener('resize', setCanvasSize);

    // Particle system for animated dots
    class Particle {
      x: number;
      y: number;
      size: number;
      speedX: number;
      speedY: number;
      opacity: number;
      hue: number;

      constructor() {
        this.x = Math.random() * canvas.width;
        this.y = Math.random() * canvas.height;
        this.size = Math.random() * 2 + 0.5;
        this.speedX = Math.random() * 0.5 - 0.25;
        this.speedY = Math.random() * 0.5 - 0.25;
        this.opacity = Math.random() * 0.5 + 0.2;
        this.hue = Math.random() * 60 + 20; // Orange/Yellow range
      }

      update() {
        this.x += this.speedX;
        this.y += this.speedY;

        // Wrap around edges
        if (this.x > canvas.width) this.x = 0;
        if (this.x < 0) this.x = canvas.width;
        if (this.y > canvas.height) this.y = 0;
        if (this.y < 0) this.y = canvas.height;

        // Pulse opacity
        this.opacity += Math.sin(Date.now() * 0.001) * 0.001;
        this.opacity = Math.max(0.1, Math.min(0.6, this.opacity));
      }

      draw() {
        if (!ctx) return;
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
        ctx.fillStyle = `hsla(${this.hue}, 100%, 60%, ${this.opacity})`;
        ctx.fill();
      }
    }

    // Create particles
    const particleCount = 150;
    const particles: Particle[] = [];
    for (let i = 0; i < particleCount; i++) {
      particles.push(new Particle());
    }

    // Gradient orbs
    class GradientOrb {
      x: number;
      y: number;
      radius: number;
      color: string;
      speedX: number;
      speedY: number;
      targetX: number;
      targetY: number;

      constructor(x: number, y: number, radius: number, color: string) {
        this.x = x;
        this.y = y;
        this.radius = radius;
        this.color = color;
        this.speedX = 0;
        this.speedY = 0;
        this.targetX = x;
        this.targetY = y;
      }

      update() {
        // Smooth movement towards target
        const dx = this.targetX - this.x;
        const dy = this.targetY - this.y;
        this.x += dx * 0.02;
        this.y += dy * 0.02;

        // Set new random target occasionally
        if (Math.random() < 0.01) {
          this.targetX = Math.random() * canvas.width;
          this.targetY = Math.random() * canvas.height;
        }
      }

      draw() {
        if (!ctx) return;
        const gradient = ctx.createRadialGradient(
          this.x,
          this.y,
          0,
          this.x,
          this.y,
          this.radius
        );
        gradient.addColorStop(0, this.color);
        gradient.addColorStop(1, 'rgba(0, 0, 0, 0)');

        ctx.fillStyle = gradient;
        ctx.fillRect(
          this.x - this.radius,
          this.y - this.radius,
          this.radius * 2,
          this.radius * 2
        );
      }
    }

    const orbs = [
      new GradientOrb(
        canvas.width * 0.7,
        canvas.height * 0.3,
        400,
        'rgba(251, 191, 36, 0.15)' // Yellow
      ),
      new GradientOrb(
        canvas.width * 0.3,
        canvas.height * 0.6,
        350,
        'rgba(249, 115, 22, 0.12)' // Orange
      ),
      new GradientOrb(
        canvas.width * 0.8,
        canvas.height * 0.7,
        300,
        'rgba(239, 68, 68, 0.08)' // Red
      )
    ];

    // Noise overlay
    let noiseOffset = 0;
    function drawNoise() {
      if (!ctx) return;
      const imageData = ctx.createImageData(canvas.width, canvas.height);
      const data = imageData.data;

      for (let i = 0; i < data.length; i += 4) {
        const noise = Math.random() * 50;
        data[i] = noise; // R
        data[i + 1] = noise; // G
        data[i + 2] = noise; // B
        data[i + 3] = 3; // A (very low opacity)
      }

      ctx.putImageData(imageData, 0, 0);
    }

    // Animation loop
    function animate() {
      if (!ctx) return;

      // Clear canvas
      ctx.fillStyle = 'rgba(0, 0, 0, 1)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Draw and update gradient orbs
      orbs.forEach(orb => {
        orb.update();
        orb.draw();
      });

      // Draw and update particles
      particles.forEach(particle => {
        particle.update();
        particle.draw();
      });

      // Draw connections between nearby particles
      particles.forEach((particleA, indexA) => {
        particles.slice(indexA + 1).forEach(particleB => {
          const dx = particleA.x - particleB.x;
          const dy = particleA.y - particleB.y;
          const distance = Math.sqrt(dx * dx + dy * dy);

          if (distance < 120) {
            ctx.beginPath();
            ctx.moveTo(particleA.x, particleA.y);
            ctx.lineTo(particleB.x, particleB.y);
            ctx.strokeStyle = `rgba(251, 191, 36, ${0.15 * (1 - distance / 120)})`;
            ctx.lineWidth = 0.5;
            ctx.stroke();
          }
        });
      });

      // Add subtle noise
      if (Math.random() < 0.1) {
        drawNoise();
      }

      // Vignette effect
      const gradient = ctx.createRadialGradient(
        canvas.width / 2,
        canvas.height / 2,
        0,
        canvas.width / 2,
        canvas.height / 2,
        canvas.width / 1.5
      );
      gradient.addColorStop(0, 'rgba(0, 0, 0, 0)');
      gradient.addColorStop(1, 'rgba(0, 0, 0, 0.8)');
      ctx.fillStyle = gradient;
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      requestAnimationFrame(animate);
    }

    animate();

    return () => {
      window.removeEventListener('resize', setCanvasSize);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full"
      style={{ filter: 'blur(60px)' }}
    />
  );
}
