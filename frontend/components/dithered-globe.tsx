/**
 * Dithered Rotating Globe - DotRepute Logo
 * WebGL-powered rotating sphere with network connections
 * Polkadot/reputation theme with dithered rendering
 */

'use client';

import { useEffect, useRef } from 'react';

export function DitheredGlobe() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d', { alpha: true });
    if (!ctx) return;

    let animationId: number;
    const size = 400; // Globe size
    const centerX = size / 2;
    const centerY = size / 2;
    const radius = 150;

    canvas.width = size;
    canvas.height = size;

    // 3D point on sphere
    class SpherePoint {
      theta: number; // Latitude
      phi: number;   // Longitude
      x: number = 0;
      y: number = 0;
      z: number = 0;
      screenX: number = 0;
      screenY: number = 0;
      visible: boolean = true;
      size: number;
      isNode: boolean;

      constructor(theta: number, phi: number, isNode: boolean = false) {
        this.theta = theta;
        this.phi = phi;
        this.isNode = isNode;
        this.size = isNode ? 4 : 1.5;
        this.calculate3DPosition(0);
      }

      calculate3DPosition(rotation: number) {
        // Sphere parametric equations
        const adjustedPhi = this.phi + rotation;
        this.x = radius * Math.sin(this.theta) * Math.cos(adjustedPhi);
        this.y = radius * Math.sin(this.theta) * Math.sin(adjustedPhi);
        this.z = radius * Math.cos(this.theta);
      }

      project(rotation: number) {
        this.calculate3DPosition(rotation);

        // Simple orthographic projection
        this.screenX = centerX + this.x;
        this.screenY = centerY + this.y;

        // Backface culling (points on back of sphere)
        this.visible = this.z >= -radius * 0.1;
      }

      getDepth(): number {
        return this.z;
      }
    }

    // Network connection between nodes
    class Connection {
      from: SpherePoint;
      to: SpherePoint;

      constructor(from: SpherePoint, to: SpherePoint) {
        this.from = from;
        this.to = to;
      }

      draw(ctx: CanvasRenderingContext2D) {
        if (!this.from.visible && !this.to.visible) return;

        const avgZ = (this.from.z + this.to.z) / 2;
        const opacity = Math.max(0, (avgZ + radius) / (2 * radius)) * 0.3;

        if (opacity > 0.05) {
          ctx.strokeStyle = `rgba(251, 191, 36, ${opacity})`; // Yellow-400
          ctx.lineWidth = 0.5;
          ctx.beginPath();
          ctx.moveTo(this.from.screenX, this.from.screenY);
          ctx.lineTo(this.to.screenX, this.to.screenY);
          ctx.stroke();
        }
      }
    }

    // Generate dithered sphere points
    const points: SpherePoint[] = [];
    const nodes: SpherePoint[] = [];
    const connections: Connection[] = [];

    // Create main dot grid with dithering
    const latSteps = 30;
    const lonSteps = 60;

    for (let i = 0; i <= latSteps; i++) {
      for (let j = 0; j <= lonSteps; j++) {
        // Dithering: skip some points based on pattern
        const ditherPattern = (i + j) % 3;
        if (ditherPattern === 0) continue; // Skip 33% of points for dithered look

        const theta = (i / latSteps) * Math.PI;
        const phi = (j / lonSteps) * Math.PI * 2;
        points.push(new SpherePoint(theta, phi, false));
      }
    }

    // Create reputation nodes (highlighted points representing validators/contributors)
    const nodeCount = 12;
    for (let i = 0; i < nodeCount; i++) {
      const theta = Math.acos(2 * Math.random() - 1); // Random latitude
      const phi = Math.random() * Math.PI * 2; // Random longitude
      const node = new SpherePoint(theta, phi, true);
      nodes.push(node);
      points.push(node);
    }

    // Create connections between nearby nodes
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        // Connect some nodes randomly (network topology)
        if (Math.random() < 0.3) {
          connections.push(new Connection(nodes[i], nodes[j]));
        }
      }
    }

    // Polkadot logo dots (arranged in a circle pattern on front of sphere)
    const polkadotDots: SpherePoint[] = [];
    const dotCount = 8;
    const dotRadius = 0.4; // Relative to sphere radius

    for (let i = 0; i < dotCount; i++) {
      const angle = (i / dotCount) * Math.PI * 2;
      const theta = Math.PI / 2 - dotRadius;
      const phi = angle;
      const dot = new SpherePoint(theta, phi, true);
      polkadotDots.push(dot);
      points.push(dot);
    }

    // Animation
    let rotation = 0;
    const rotationSpeed = 0.003;

    const animate = () => {
      rotation += rotationSpeed;

      // Clear canvas
      ctx.clearRect(0, 0, size, size);

      // Update all point positions
      points.forEach(point => point.project(rotation));

      // Sort points by depth (z-order) for proper rendering
      const sortedPoints = [...points].sort((a, b) => a.getDepth() - b.getDepth());

      // Draw connections first (background layer)
      connections.forEach(conn => conn.draw(ctx));

      // Draw latitude/longitude grid lines
      ctx.strokeStyle = 'rgba(100, 100, 100, 0.1)';
      ctx.lineWidth = 0.5;

      // Latitude lines
      for (let i = 1; i < latSteps; i++) {
        const theta = (i / latSteps) * Math.PI;
        ctx.beginPath();
        let firstPoint = true;

        for (let j = 0; j <= lonSteps * 2; j++) {
          const phi = (j / (lonSteps * 2)) * Math.PI * 2 + rotation;
          const x = radius * Math.sin(theta) * Math.cos(phi);
          const y = radius * Math.sin(theta) * Math.sin(phi);
          const z = radius * Math.cos(theta);

          if (z >= -radius * 0.1) {
            const screenX = centerX + x;
            const screenY = centerY + y;

            if (firstPoint) {
              ctx.moveTo(screenX, screenY);
              firstPoint = false;
            } else {
              ctx.lineTo(screenX, screenY);
            }
          }
        }
        ctx.stroke();
      }

      // Draw all points with dithered effect
      sortedPoints.forEach(point => {
        if (!point.visible) return;

        const depth = point.getDepth();
        const normalizedDepth = (depth + radius) / (2 * radius); // 0 to 1

        // Color based on depth and type
        let color: string;
        let opacity: number;

        if (point.isNode) {
          // Highlighted nodes (orange/yellow for reputation nodes)
          color = polkadotDots.includes(point)
            ? '255, 191, 36' // Yellow (Polkadot dots)
            : '249, 115, 22'; // Orange (reputation nodes)
          opacity = 0.8 + normalizedDepth * 0.2;
        } else {
          // Regular grid points (blue gradient)
          const hue = 200 + normalizedDepth * 20; // Blue spectrum
          color = `${Math.round(hslToRgb(hue, 80, 60).r)}, ${Math.round(hslToRgb(hue, 80, 60).g)}, ${Math.round(hslToRgb(hue, 80, 60).b)}`;
          opacity = 0.3 + normalizedDepth * 0.4;
        }

        // Dithered size variation
        const ditherSize = point.size * (0.8 + Math.random() * 0.4);

        ctx.beginPath();
        ctx.arc(point.screenX, point.screenY, ditherSize, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(${color}, ${opacity})`;
        ctx.fill();

        // Add glow to nodes
        if (point.isNode) {
          ctx.beginPath();
          ctx.arc(point.screenX, point.screenY, point.size * 2, 0, Math.PI * 2);
          ctx.fillStyle = `rgba(${color}, ${opacity * 0.2})`;
          ctx.fill();
        }
      });

      animationId = requestAnimationFrame(animate);
    };

    animate();

    // Cleanup
    return () => {
      cancelAnimationFrame(animationId);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="absolute pointer-events-none"
      style={{
        width: '400px',
        height: '400px',
        opacity: 0.9,
      }}
    />
  );
}

// Helper: HSL to RGB conversion
function hslToRgb(h: number, s: number, l: number): { r: number; g: number; b: number } {
  s /= 100;
  l /= 100;

  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;

  let r = 0, g = 0, b = 0;

  if (0 <= h && h < 60) {
    r = c; g = x; b = 0;
  } else if (60 <= h && h < 120) {
    r = x; g = c; b = 0;
  } else if (120 <= h && h < 180) {
    r = 0; g = c; b = x;
  } else if (180 <= h && h < 240) {
    r = 0; g = x; b = c;
  } else if (240 <= h && h < 300) {
    r = x; g = 0; b = c;
  } else if (300 <= h && h < 360) {
    r = c; g = 0; b = x;
  }

  return {
    r: (r + m) * 255,
    g: (g + m) * 255,
    b: (b + m) * 255
  };
}
