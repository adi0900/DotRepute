# DotRepute Landing Page - Implementation Guide

## Quick Start

### Prerequisites
```bash
Node.js 18+
npm or yarn
```

### Installation

```bash
cd frontend
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000)

## File Structure

```
frontend/
├── app/
│   ├── page.tsx                          ✅ Main landing page (updated)
│   ├── layout.tsx                        ✅ Root layout
│   └── globals.css                       ✅ Global styles
├── components/
│   ├── ui/                               ✅ ShadCN components
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   └── badge.tsx
│   ├── animated-hero-background.tsx      ⚠️ Legacy (kept for reference)
│   └── enhanced-webgl-background.tsx     ✨ NEW - Enhanced animation
├── lib/
│   └── utils.ts                          ✅ Utilities
└── public/                               📁 Static assets
```

## What Changed

### 1. Enhanced Background Component

**New File**: `components/enhanced-webgl-background.tsx`

**Features**:
- Dot matrix pattern (Fumadocs style)
- Three animated gradient orbs
- Performance-optimized Canvas 2D
- Mouse tracking (prepared for interactivity)
- Responsive with device pixel ratio support
- Visibility API for battery saving

**Usage**:
```tsx
import { EnhancedWebGLBackground } from '@/components/enhanced-webgl-background';

<section className="relative min-h-screen overflow-hidden">
  <EnhancedWebGLBackground />
  <div className="relative z-10">
    {/* Content */}
  </div>
</section>
```

### 2. Updated Sections

All sections now use Fumadocs-inspired styling:

#### Navigation
- Glassmorphic with `backdrop-blur-2xl`
- Subtle border (`white/5`)
- Logo in gradient box

#### Hero
- Massive typography (text-8xl)
- Yellow primary CTA with shadow
- Gradient text on keywords
- Enhanced WebGL background

#### Features
- Compact card grid
- Gradient icon containers
- Arrow markers (→) for lists

#### Tech Stack
- Badge-based layout
- Uppercase category labels

#### Workflow
- Numbered step cards
- Connecting lines between steps

#### Team & CTA
- Minimal glassmorphic cards
- Gradient overlays

## Color System

### Primary Palette
```css
/* Fumadocs Orange Gradient */
--yellow: #FACC15 (Yellow-400)
--orange: #F97316 (Orange-500)
--red-orange: #EA580C (Orange-600)
--red: #EF4444 (Red-500)

/* Background */
--black: #000000
--dark-bg: rgba(0, 0, 0, 0.8)

/* Borders & Glass */
--border-subtle: rgba(255, 255, 255, 0.05)
--border-medium: rgba(255, 255, 255, 0.08)
--glass-bg: rgba(255, 255, 255, 0.02-0.03)

/* Text */
--text-primary: #FFFFFF
--text-secondary: #9CA3AF (Gray-400)
--text-muted: #6B7280 (Gray-500)
```

### Gradient Classes
```tsx
// Text gradients
className="text-transparent bg-clip-text bg-gradient-to-r from-yellow-300 via-orange-400 to-orange-600"

// Background gradients
className="bg-gradient-to-br from-yellow-400/10 via-orange-500/10 to-transparent"

// Icon containers
className="bg-gradient-to-br from-orange-400/10 to-orange-600/10"
```

## Typography Scale

```css
/* Headings */
text-xs      (0.75rem)   - Meta info, badges
text-sm      (0.875rem)  - Card descriptions
text-base    (1rem)      - Body text
text-lg      (1.125rem)  - Large body
text-xl      (1.25rem)   - Subheadings
text-2xl     (1.5rem)    - Section titles
text-4xl     (2.25rem)   - Page headings
text-5xl     (3rem)      - Hero secondary
text-6xl     (3.75rem)   - Hero primary (tablet)
text-7xl     (4.5rem)    - Hero primary (desktop)
text-8xl     (6rem)      - Hero primary (large)

/* Tracking */
tracking-tight          - Headings
tracking-wider          - Uppercase labels
```

## Spacing System

```css
/* Padding/Margin */
p-3, px-4, py-1.5      - Badges
p-5, p-6               - Card padding
p-12, p-16             - Large sections
gap-3, gap-4           - Grid gaps
space-y-4, space-y-8   - Vertical rhythm

/* Sections */
py-24                  - Standard section padding
pt-20                  - Hero top padding (nav offset)
```

## Animation System

### Framer Motion Variants

```tsx
// Fade in with slide up
initial={{ opacity: 0, y: 20 }}
animate={{ opacity: 1, y: 0 }}
transition={{ duration: 0.8 }}

// Scale in
initial={{ opacity: 0, scale: 0.95 }}
whileInView={{ opacity: 1, scale: 1 }}
viewport={{ once: true }}
transition={{ delay: index * 0.05 }}

// Slide from left
initial={{ opacity: 0, x: -20 }}
whileInView={{ opacity: 1, x: 0 }}
```

### CSS Transitions

```css
/* Hover effects */
transition-colors      - Color changes
transition-all         - Multi-property
duration-300          - 300ms timing

/* Transforms */
hover:scale-110       - Icon scale
group-hover:scale-110 - Group-based scale
```

## Icons (Lucide React)

### Usage Pattern
```tsx
import { IconName } from 'lucide-react';

<IconName className="w-4 h-4 text-gray-400" />
```

### Icon Sizes
- `w-3.5 h-3.5` - Small (badges, nav)
- `w-4 h-4` - Standard (buttons, links)
- `w-5 h-5` - Medium (cards)
- `w-5.5 h-5.5` - Feature icons
- `w-8 h-8` - Large icons

## Responsive Breakpoints

```tsx
// Mobile First
<div className="text-4xl md:text-5xl lg:text-6xl">

// Breakpoints
sm: 640px    - Small tablets
md: 768px    - Tablets
lg: 1024px   - Desktop
xl: 1280px   - Large desktop
2xl: 1536px  - Extra large
```

## Performance Tips

### 1. Image Optimization
```tsx
import Image from 'next/image';

<Image
  src="/hero-image.png"
  alt="Description"
  width={800}
  height={600}
  priority // For above-fold images
/>
```

### 2. Code Splitting
- Dynamic imports for heavy components
- Lazy load below-fold sections

### 3. Animation Performance
- Use `transform` and `opacity` only
- Avoid animating `width`, `height`, `top`, `left`
- Use `will-change` sparingly

### 4. Bundle Size
```bash
# Analyze bundle
npm run build
npm run analyze
```

## Testing Checklist

### Visual Testing
- [ ] Hero section displays correctly
- [ ] Gradient animations smooth
- [ ] Cards have proper glassmorphic effect
- [ ] Icons load and display
- [ ] Typography scales correctly
- [ ] Colors match design system

### Responsive Testing
- [ ] Mobile (375px, 414px)
- [ ] Tablet (768px, 1024px)
- [ ] Desktop (1280px, 1920px)
- [ ] Navigation collapses on mobile
- [ ] Grid layouts stack properly

### Performance Testing
- [ ] Lighthouse score > 90
- [ ] First Contentful Paint < 1.5s
- [ ] Time to Interactive < 3s
- [ ] Canvas animation at 60fps
- [ ] No layout shifts (CLS < 0.1)

### Browser Testing
- [ ] Chrome (latest)
- [ ] Firefox (latest)
- [ ] Safari (latest)
- [ ] Edge (latest)
- [ ] Mobile Safari
- [ ] Mobile Chrome

### Accessibility Testing
- [ ] Keyboard navigation works
- [ ] Focus indicators visible
- [ ] ARIA labels present
- [ ] Color contrast passes WCAG AA
- [ ] Screen reader compatible

## Deployment

### Build for Production
```bash
npm run build
npm run start
```

### Environment Variables
```env
# .env.local
NEXT_PUBLIC_API_URL=https://api.dotrepute.io
NEXT_PUBLIC_GITHUB_URL=https://github.com/adi0900/DotRepute
```

### Deployment Platforms
- **Vercel** (Recommended): Zero-config deployment
- **Netlify**: Alternative with edge functions
- **AWS Amplify**: Enterprise option
- **Cloudflare Pages**: Edge-optimized

## Troubleshooting

### Canvas Not Showing
```typescript
// Check if canvas ref is mounted
console.log(canvasRef.current); // Should not be null

// Verify canvas dimensions
console.log(canvas.width, canvas.height); // Should be > 0
```

### Animation Stuttering
```typescript
// Reduce particle count
const spacing = 50; // Increase from 40

// Cap device pixel ratio
const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
```

### Glassmorphism Not Working
```css
/* Ensure backdrop-blur is supported */
@supports (backdrop-filter: blur(10px)) {
  .glass {
    backdrop-filter: blur(10px);
  }
}

/* Fallback */
@supports not (backdrop-filter: blur(10px)) {
  .glass {
    background: rgba(0, 0, 0, 0.8);
  }
}
```

## Resources

- **Design Reference**: Fumadocs (G5OYQkibIAEsC2B.jpg)
- **Icons**: [Lucide React](https://lucide.dev/)
- **Components**: [ShadCN UI](https://ui.shadcn.com/)
- **Animations**: [Framer Motion](https://www.framer.com/motion/)
- **Inspiration**: [Unicorn Studio](https://unicorn.studio/)

## Support

For issues or questions:
- GitHub: https://github.com/adi0900/DotRepute
- Documentation: See README.md in project root

## License

MIT License - See LICENSE file
