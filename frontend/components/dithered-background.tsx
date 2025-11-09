/**
 * Dithered Background Effect
 * Inspired by Launch.com - Creates a dynamic dithered dot pattern
 * Using blue/orange gradient theme for DotRepute
 */

'use client';

import { useEffect, useRef } from 'react';

export function DitheredBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    let animationId: number;
    let width: number, height: number;

    const resize = () => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
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

    // Dithered dot pattern system
    class DitheredPattern {
      dots: DitherDot[] = [];
      spacing = 8; // Tight spacing for dithered effect
      waveOffset = 0;

      constructor() {
        this.generate();
      }

      generate() {
        this.dots = [];
        const cols = Math.ceil(width / this.spacing) + 1;
        const rows = Math.ceil(height / this.spacing) + 1;

        for (let i = 0; i < cols; i++) {
          for (let j = 0; j < rows; j++) {
            const x = i * this.spacing;
            const y = j * this.spacing;
            this.dots.push(new DitherDot(x, y, i, j));
          }
        }
      }

      update(time: number) {
        this.waveOffset = time * 0.0003;
      }

      draw(time: number) {
        if (!ctx) return;
        this.dots.forEach(dot => dot.draw(ctx, time, this.waveOffset, width, height));
      }
    }

    class DitherDot {
      x: number;
      y: number;
      gridX: number;
      gridY: number;

      constructor(x: number, y: number, gridX: number, gridY: number) {
        this.x = x;
        this.y = y;
        this.gridX = gridX;
        this.gridY = gridY;
      }

      draw(ctx: CanvasRenderingContext2D, time: number, waveOffset: number, canvasWidth: number, canvasHeight: number) {
        // Create organic blob shape in center-right (matching gradient orb position)
        const centerX = canvasWidth * 0.75;
        const centerY = canvasHeight * 0.4;

        const dx = this.x - centerX;
        const dy = this.y - centerY;
        const distance = Math.sqrt(dx * dx + dy * dy);

        // Create organic blob boundary with perlin-like noise
        const angle = Math.atan2(dy, dx);
        const noise = Math.sin(angle * 3 + waveOffset) * 80 +
                     Math.sin(angle * 5 - waveOffset * 1.3) * 40 +
                     Math.cos(angle * 7 + waveOffset * 0.7) * 20;

        const blobRadius = Math.min(canvasWidth, canvasHeight) * 0.35 + noise;

        // Calculate if dot is inside the blob
        const isInside = distance < blobRadius;

        // Dithering effect based on distance (Floyd-Steinberg inspired)
        const ditherThreshold = (this.gridX % 4) * 0.25 + (this.gridY % 4) * 0.063;
        const normalizedDistance = distance / blobRadius;

        // Create graduated dither pattern
        let shouldDraw = false;
        let size = 1.5;
        let opacity = 0.8;

        if (isInside) {
          // Core: dense dithering
          if (normalizedDistance < 0.5) {
            shouldDraw = true;
            size = 2;
            opacity = 0.9;
          }
          // Mid: medium dithering
          else if (normalizedDistance < 0.8) {
            shouldDraw = (this.gridX + this.gridY) % 2 === 0 || ditherThreshold > normalizedDistance;
            size = 1.5;
            opacity = 0.7;
          }
          // Edge: sparse dithering
          else {
            shouldDraw = ditherThreshold > normalizedDistance * 1.5;
            size = 1;
            opacity = 0.5;
          }
        } else {
          // Outside blob: very sparse ambient dots
          const edgeDistance = distance - blobRadius;
          if (edgeDistance < 150) {
            shouldDraw = ditherThreshold > 0.85 && (this.gridX + this.gridY) % 3 === 0;
            size = 0.8;
            opacity = Math.max(0, 1 - edgeDistance / 150) * 0.3;
          }
        }

        if (shouldDraw) {
          // Color gradient: orange/blue spectrum matching DotRepute theme
          const hue = isInside
            ? 25 - normalizedDistance * 15  // Orange to blue gradient (25-10)
            : 200 + Math.random() * 20; // Ambient blue dots

          const saturation = isInside ? 95 : 80;
          const lightness = isInside
            ? 55 + (1 - normalizedDistance) * 15
            : 50;

          ctx.beginPath();
          ctx.arc(this.x, this.y, size, 0, Math.PI * 2);
          ctx.fillStyle = `hsla(${hue}, ${saturation}%, ${lightness}%, ${opacity})`;
          ctx.fill();
        }
      }
    }

    const ditheredPattern = new DitheredPattern();

    // Animation loop
    const animate = (time: number) => {
      // Clear canvas
      ctx.fillStyle = 'rgba(0, 0, 0, 0)';
      ctx.clearRect(0, 0, width, height);

      // Update and draw
      ditheredPattern.update(time);
      ditheredPattern.draw(time);

      animationId = requestAnimationFrame(animate);
    };

    animate(0);

    // Cleanup
    return () => {
      window.removeEventListener('resize', resize);
      cancelAnimationFrame(animationId);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="absolute inset-0 w-full h-full pointer-events-none"
      style={{
        opacity: 0.6,
        mixBlendMode: 'normal',
      }}
    />
  );
}
