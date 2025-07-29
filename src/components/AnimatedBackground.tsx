'use client';

import { useEffect, useRef } from 'react';

export default function AnimatedBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const resizeCanvas = () => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    };

    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    // Geometric shapes and lines
    const shapes: Array<{
      type: 'line' | 'circle' | 'triangle' | 'square' | 'star';
      x: number;
      y: number;
      size: number;
      rotation: number;
      speed: number;
      opacity: number;
      color: string;
      life: number;
      direction: { x: number; y: number };
      pulse: number;
      pulseSpeed: number;
      fadeOut: boolean;
      targetX: number;
      targetY: number;
      targetSize: number;
      targetOpacity: number;
    }> = [];

    const colors = [
      'rgba(99, 102, 241, 0.5)', // Indigo
      'rgba(236, 72, 153, 0.5)', // Pink
      'rgba(251, 191, 36, 0.5)', // Amber
      'rgba(34, 197, 94, 0.5)', // Green
      'rgba(147, 51, 234, 0.5)', // Purple
      'rgba(59, 130, 246, 0.5)', // Blue
    ];

    const createShape = () => {
      const types: Array<'line' | 'circle' | 'triangle' | 'square' | 'star'> = [
        'line',
        'circle',
        'triangle',
        'square',
        'star',
      ];
      const type = types[Math.floor(Math.random() * types.length)];

      return {
        type,
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        size: Math.random() * 50 + 25,
        rotation: Math.random() * Math.PI * 2,
        speed: Math.random() * 0.03 + 0.008,
        opacity: Math.random() * 0.7 + 0.3,
        color: colors[Math.floor(Math.random() * colors.length)],
        life: 1.0,
        direction: {
          x: (Math.random() - 0.5) * 0.8,
          y: (Math.random() - 0.5) * 0.8,
        },
        pulse: Math.random() * Math.PI * 2,
        pulseSpeed: Math.random() * 0.05 + 0.02,
        fadeOut: false,
        targetX: 0,
        targetY: 0,
        targetSize: 0,
        targetOpacity: 0,
      };
    };

    // Initialize shapes
    for (let i = 0; i < 30; i++) {
      shapes.push(createShape());
    }

    // Create flowing lines
    const lines: Array<{
      x1: number;
      y1: number;
      x2: number;
      y2: number;
      opacity: number;
      color: string;
      life: number;
      width: number;
      fadeOut: boolean;
      targetOpacity: number;
    }> = [];

    const createLine = () => ({
      x1: Math.random() * canvas.width,
      y1: Math.random() * canvas.height,
      x2: Math.random() * canvas.width,
      y2: Math.random() * canvas.height,
      opacity: Math.random() * 0.3 + 0.1,
      color: colors[Math.floor(Math.random() * colors.length)],
      life: 1.0,
      width: Math.random() * 2 + 1,
      fadeOut: false,
      targetOpacity: 0,
    });

    for (let i = 0; i < 20; i++) {
      lines.push(createLine());
    }

    // Add floating particles
    const particles: Array<{
      x: number;
      y: number;
      size: number;
      opacity: number;
      color: string;
      life: number;
      direction: { x: number; y: number };
      fadeOut: boolean;
      targetOpacity: number;
    }> = [];

    const createParticle = () => ({
      x: Math.random() * canvas.width,
      y: Math.random() * canvas.height,
      size: Math.random() * 4 + 1,
      opacity: Math.random() * 0.6 + 0.2,
      color: colors[Math.floor(Math.random() * colors.length)],
      life: 1.0,
      direction: {
        x: (Math.random() - 0.5) * 0.3,
        y: (Math.random() - 0.5) * 0.3,
      },
      fadeOut: false,
      targetOpacity: 0,
    });

    for (let i = 0; i < 40; i++) {
      particles.push(createParticle());
    }

    const animate = () => {
      // Clear with pure black background and very subtle fade
      ctx.fillStyle = 'rgba(0, 0, 0, 0.95)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // Update and draw particles
      for (let i = particles.length - 1; i >= 0; i--) {
        const particle = particles[i];

        particle.x += particle.direction.x;
        particle.y += particle.direction.y;
        particle.life -= 0.001;

        // Wrap around screen
        if (particle.x < -particle.size) particle.x = canvas.width + particle.size;
        if (particle.x > canvas.width + particle.size) particle.x = -particle.size;
        if (particle.y < -particle.size) particle.y = canvas.height + particle.size;
        if (particle.y > canvas.height + particle.size) particle.y = -particle.size;

        // Smooth fade out when life is low
        if (particle.life <= 0.1 && !particle.fadeOut) {
          particle.fadeOut = true;
          particle.targetOpacity = 0;
        }

        if (particle.fadeOut) {
          particle.opacity = Math.max(0, particle.opacity - 0.02);
          if (particle.opacity <= 0) {
            particles.splice(i, 1);
            const newParticle = createParticle();
            // Smooth transition to new position
            newParticle.x = particle.x;
            newParticle.y = particle.y;
            newParticle.opacity = 0;
            newParticle.targetOpacity = newParticle.opacity;
            particles.push(newParticle);
            continue;
          }
        } else {
          // Smooth fade in
          particle.opacity = Math.min(
            particle.opacity + 0.01,
            particle.targetOpacity || particle.opacity
          );
        }

        ctx.save();
        ctx.globalAlpha = particle.opacity * particle.life;
        ctx.fillStyle = particle.color;
        ctx.beginPath();
        ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
        ctx.fill();
        ctx.restore();
      }

      // Update and draw lines
      for (let i = lines.length - 1; i >= 0; i--) {
        const line = lines[i];

        line.life -= 0.002;

        // Smooth fade out when life is low
        if (line.life <= 0.1 && !line.fadeOut) {
          line.fadeOut = true;
          line.targetOpacity = 0;
        }

        if (line.fadeOut) {
          line.opacity = Math.max(0, line.opacity - 0.02);
          if (line.opacity <= 0) {
            lines.splice(i, 1);
            const newLine = createLine();
            // Smooth transition
            newLine.opacity = 0;
            newLine.targetOpacity = newLine.opacity;
            lines.push(newLine);
            continue;
          }
        } else {
          // Smooth fade in
          line.opacity = Math.min(line.opacity + 0.01, line.targetOpacity || line.opacity);
        }

        ctx.save();
        ctx.globalAlpha = line.opacity * line.life;
        ctx.strokeStyle = line.color;
        ctx.lineWidth = line.width;
        ctx.lineCap = 'round';
        ctx.beginPath();
        ctx.moveTo(line.x1, line.y1);
        ctx.lineTo(line.x2, line.y2);
        ctx.stroke();
        ctx.restore();
      }

      // Update and draw shapes
      for (let i = shapes.length - 1; i >= 0; i--) {
        const shape = shapes[i];

        shape.rotation += shape.speed;
        shape.x += shape.direction.x;
        shape.y += shape.direction.y;
        shape.life -= 0.001;
        shape.pulse += shape.pulseSpeed;

        // Wrap around screen
        if (shape.x < -shape.size) shape.x = canvas.width + shape.size;
        if (shape.x > canvas.width + shape.size) shape.x = -shape.size;
        if (shape.y < -shape.size) shape.y = canvas.height + shape.size;
        if (shape.y > canvas.height + shape.size) shape.y = -shape.size;

        // Smooth fade out when life is low
        if (shape.life <= 0.1 && !shape.fadeOut) {
          shape.fadeOut = true;
          shape.targetOpacity = 0;
          shape.targetSize = shape.size * 0.5;
        }

        if (shape.fadeOut) {
          shape.opacity = Math.max(0, shape.opacity - 0.02);
          shape.size = Math.max(shape.size * 0.98, shape.targetSize);
          if (shape.opacity <= 0) {
            shapes.splice(i, 1);
            const newShape = createShape();
            // Smooth transition to new position
            newShape.x = shape.x;
            newShape.y = shape.y;
            newShape.opacity = 0;
            newShape.size = shape.size * 0.5;
            newShape.targetOpacity = newShape.opacity;
            newShape.targetSize = newShape.size;
            shapes.push(newShape);
            continue;
          }
        } else {
          // Smooth fade in and size transition
          shape.opacity = Math.min(shape.opacity + 0.01, shape.targetOpacity || shape.opacity);
          shape.size = Math.min(shape.size * 1.01, shape.targetSize || shape.size);
        }

        const pulseScale = 1 + Math.sin(shape.pulse) * 0.2;
        const currentSize = shape.size * pulseScale;

        ctx.save();
        ctx.globalAlpha = shape.opacity * shape.life;
        ctx.fillStyle = shape.color;
        ctx.strokeStyle = shape.color;
        ctx.lineWidth = 2;

        ctx.translate(shape.x, shape.y);
        ctx.rotate(shape.rotation);
        ctx.scale(pulseScale, pulseScale);

        switch (shape.type) {
          case 'circle':
            // Create gradient for circles
            const gradient = ctx.createRadialGradient(0, 0, 0, 0, 0, currentSize / 2);
            gradient.addColorStop(0, shape.color);
            gradient.addColorStop(1, 'rgba(255, 255, 255, 0)');
            ctx.fillStyle = gradient;
            ctx.beginPath();
            ctx.arc(0, 0, currentSize / 2, 0, Math.PI * 2);
            ctx.fill();
            ctx.strokeStyle = shape.color;
            ctx.lineWidth = 1;
            ctx.stroke();
            break;
          case 'square':
            ctx.strokeRect(-currentSize / 2, -currentSize / 2, currentSize, currentSize);
            break;
          case 'triangle':
            ctx.beginPath();
            ctx.moveTo(0, -currentSize / 2);
            ctx.lineTo(-currentSize / 2, currentSize / 2);
            ctx.lineTo(currentSize / 2, currentSize / 2);
            ctx.closePath();
            ctx.stroke();
            break;
          case 'line':
            ctx.beginPath();
            ctx.moveTo(-currentSize / 2, 0);
            ctx.lineTo(currentSize / 2, 0);
            ctx.stroke();
            break;
          case 'star':
            ctx.beginPath();
            for (let i = 0; i < 5; i++) {
              const angle = (i * 4 * Math.PI) / 5 - Math.PI / 2;
              const x = (Math.cos(angle) * currentSize) / 2;
              const y = (Math.sin(angle) * currentSize) / 2;
              if (i === 0) ctx.moveTo(x, y);
              else ctx.lineTo(x, y);
            }
            ctx.closePath();
            ctx.stroke();
            break;
        }

        ctx.restore();
      }

      // Add enhanced glow effect
      ctx.save();
      ctx.globalCompositeOperation = 'screen';
      ctx.fillStyle = 'rgba(99, 102, 241, 0.02)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.restore();

      requestAnimationFrame(animate);
    };

    animate();

    return () => {
      window.removeEventListener('resize', resizeCanvas);
    };
  }, []);

  return (
    <canvas
      ref={canvasRef}
      className="fixed inset-0 pointer-events-none z-0"
      style={{ background: 'transparent' }}
    />
  );
}
