/**
 * Particle Globe - High quality rotating globe effect
 * Inspired by globe reference - Clean particle-based world map
 */

'use client';

import { useEffect, useRef } from 'react';

interface ParticleGlobeProps {
  theme?: 'light' | 'dark';
}

export function ParticleGlobe({ theme = 'dark' }: ParticleGlobeProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    let animationId: number;
    const size = 500;
    const centerX = size / 2;
    const centerY = size / 2;
    const radius = 180;

    canvas.width = size;
    canvas.height = size;

    // Particle on sphere representing a point on Earth
    class Particle {
      lat: number;
      lon: number;
      x: number = 0;
      y: number = 0;
      z: number = 0;
      screenX: number = 0;
      screenY: number = 0;
      alpha: number = 1;
      size: number = 1;
      visible: boolean = true;

      constructor(lat: number, lon: number, size: number = 1) {
        this.lat = lat;
        this.lon = lon;
        this.size = size;
      }

      // Convert lat/lon to 3D coordinates
      update(rotation: number) {
        const phi = (this.lat * Math.PI) / 180;
        const theta = ((this.lon + rotation) * Math.PI) / 180;

        this.x = radius * Math.cos(phi) * Math.cos(theta);
        this.y = radius * Math.sin(phi);
        this.z = radius * Math.cos(phi) * Math.sin(theta);

        // Project to 2D
        const scale = 300 / (300 + this.z);
        this.screenX = centerX + this.x * scale;
        this.screenY = centerY - this.y * scale;

        // Calculate visibility and alpha based on z-depth
        this.visible = this.z > -radius * 0.3;
        this.alpha = this.visible ? Math.max(0, (this.z + radius) / (2 * radius)) : 0;
      }

      draw(ctx: CanvasRenderingContext2D, isLightMode: boolean) {
        if (!this.visible || this.alpha < 0.1) return;

        const brightness = 0.4 + this.alpha * 0.6;

        let r, g, b;
        if (isLightMode) {
          // Black/dark gray colors for light mode
          const grayValue = Math.floor(20 + (this.alpha * 40)); // 20-60 (dark gray to medium gray)
          r = grayValue;
          g = grayValue;
          b = grayValue;
        } else {
          // Orange/yellow theme colors for dark mode
          r = 255;
          g = Math.floor(191 + (this.alpha * 64)); // 191-255 (yellow to orange)
          b = Math.floor(36 + (this.alpha * 20)); // 36-56
        }

        ctx.beginPath();
        ctx.arc(this.screenX, this.screenY, this.size, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${this.alpha * brightness})`;
        ctx.fill();
      }
    }

    // Generate particles for latitude/longitude grid (optimized)
    const particles: Particle[] = [];

    // Latitude lines (reduced density)
    const latCount = 15; // Reduced from 25
    for (let i = 0; i < latCount; i++) {
      const lat = -90 + (180 / latCount) * i;
      const lonPoints = Math.floor(40 * Math.cos((lat * Math.PI) / 180)) + 20; // Reduced density

      for (let j = 0; j < lonPoints; j++) {
        const lon = (360 / lonPoints) * j;
        particles.push(new Particle(lat, lon, 0.8));
      }
    }

    // Longitude lines (reduced)
    const lonCount = 24; // Reduced from 36
    for (let i = 0; i < lonCount; i++) {
      const lon = (360 / lonCount) * i;
      for (let j = -90; j <= 90; j += 6) { // Increased step from 3 to 6
        particles.push(new Particle(j, lon, 0.8));
      }
    }

    // Add continent outlines (reduced density for performance)
    const continents = [
      // North America
      { lat: 40, lon: -100, density: 15 }, // Reduced from 30
      { lat: 50, lon: -100, density: 12 },

      // South America
      { lat: -10, lon: -60, density: 10 },

      // Europe
      { lat: 50, lon: 10, density: 12 },

      // Africa
      { lat: 0, lon: 20, density: 15 },
      { lat: -10, lon: 25, density: 12 },

      // Asia
      { lat: 30, lon: 80, density: 18 },
      { lat: 40, lon: 100, density: 15 },

      // Australia
      { lat: -25, lon: 135, density: 10 },
    ];

    // Generate continent particles
    continents.forEach(continent => {
      for (let i = 0; i < continent.density; i++) {
        const lat = continent.lat + (Math.random() - 0.5) * 20;
        const lon = continent.lon + (Math.random() - 0.5) * 30;
        particles.push(new Particle(lat, lon, 1.2));
      }
    });

    // Highlight dots (reduced count)
    const highlights: Particle[] = [];
    const highlightCount = 8; // Reduced from 15
    for (let i = 0; i < highlightCount; i++) {
      const lat = (Math.random() - 0.5) * 160;
      const lon = Math.random() * 360;
      const highlight = new Particle(lat, lon, 2);
      highlights.push(highlight);
      particles.push(highlight);
    }

    // Connection lines between highlights
    class Connection {
      from: Particle;
      to: Particle;

      constructor(from: Particle, to: Particle) {
        this.from = from;
        this.to = to;
      }

      draw(ctx: CanvasRenderingContext2D, isLightMode: boolean) {
        if (!this.from.visible || !this.to.visible) return;

        const avgAlpha = (this.from.alpha + this.to.alpha) / 2;
        if (avgAlpha < 0.2) return;

        // Connection lines - gray for light mode, yellow for dark mode
        if (isLightMode) {
          ctx.strokeStyle = `rgba(60, 60, 60, ${avgAlpha * 0.4})`;
        } else {
          ctx.strokeStyle = `rgba(251, 191, 36, ${avgAlpha * 0.3})`;
        }
        ctx.lineWidth = 0.8;
        ctx.beginPath();
        ctx.moveTo(this.from.screenX, this.from.screenY);
        ctx.lineTo(this.to.screenX, this.to.screenY);
        ctx.stroke();
      }
    }

    const connections: Connection[] = [];
    for (let i = 0; i < highlights.length; i++) {
      for (let j = i + 1; j < highlights.length; j++) {
        if (Math.random() < 0.2) {
          connections.push(new Connection(highlights[i], highlights[j]));
        }
      }
    }

    // Animation with performance optimization
    let rotation = 0;
    const rotationSpeed = 0.2; // Slightly faster
    let frameCount = 0;
    let sortedCache = particles;

    const isLightMode = theme === 'light';

    const animate = () => {
      rotation += rotationSpeed;
      frameCount++;

      // Clear canvas
      ctx.fillStyle = 'rgba(0, 0, 0, 0)';
      ctx.clearRect(0, 0, size, size);

      // Update all particles
      particles.forEach(p => p.update(rotation));

      // Sort by z-depth only every 3 frames for performance
      if (frameCount % 3 === 0) {
        sortedCache = [...particles].sort((a, b) => a.z - b.z);
      }

      // Draw connections first
      connections.forEach(c => c.draw(ctx, isLightMode));

      // Draw particles
      sortedCache.forEach(p => p.draw(ctx, isLightMode));

      // Draw subtle glow on highlights - only every other frame
      if (frameCount % 2 === 0) {
        highlights.forEach(h => {
          if (h.visible && h.alpha > 0.5) {
            const glowColor = isLightMode ? '60, 60, 60' : '251, 191, 36';

            ctx.beginPath();
            ctx.arc(h.screenX, h.screenY, 6, 0, Math.PI * 2);
            ctx.fillStyle = `rgba(${glowColor}, ${h.alpha * 0.2})`;
            ctx.fill();

            // Inner bright core
            ctx.beginPath();
            ctx.arc(h.screenX, h.screenY, 3, 0, Math.PI * 2);
            ctx.fillStyle = `rgba(${glowColor}, ${h.alpha * 0.4})`;
            ctx.fill();
          }
        });
      }

      animationId = requestAnimationFrame(animate);
    };

    animate();

    // Cleanup
    return () => {
      cancelAnimationFrame(animationId);
    };
  }, [theme]);

  return (
    <canvas
      ref={canvasRef}
      className="pointer-events-none"
      style={{
        width: '500px',
        height: '500px',
        willChange: 'transform',
        transform: 'translateZ(0)',
      }}
    />
  );
}
