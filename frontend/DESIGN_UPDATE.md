# DotRepute Landing Page - Fumadocs Style Update

## Design Reference
**Image**: G5OYQkibIAEsC2B.jpg (Fumadocs dark theme with orange gradients)

## Changes Applied

### 🎨 Visual Style Updates

#### Color Palette (Matching Reference Image)
- **Primary**: Yellow/Gold (#FACC15) - Main CTA buttons
- **Accent**: Orange gradient (#F97316 to #EA580C)
- **Highlights**: Red accents (#EF4444)
- **Background**: Pure black with radial orange/yellow gradients
- **Text**: White with gray-400/gray-500 for secondary text
- **Borders**: Subtle white/[0.05] to white/[0.08] opacity

#### Typography
- **Headings**: Much larger (5xl to 8xl) with tighter tracking
- **Gradient Text**: Applied to key words like "reputation systems", "Polkadot", "Rust"
- **Font Weights**: More emphasis on semibold/bold
- **Line Heights**: Adjusted to [1.1] for large headings

### 🧩 Component Updates

#### Navigation Bar
- ✅ Glassmorphic effect with backdrop-blur-2xl
- ✅ Reduced border opacity (white/5)
- ✅ Logo in gradient box with Shield icon
- ✅ Subtle hover states on nav links
- ✅ Mobile menu button added

#### Hero Section
- ✅ **Badge**: Rounded-full pill style with Sparkles icon
- ✅ **Heading**: Massive 8xl font with gradient on "reputation systems"
- ✅ **CTA Buttons**: Yellow primary with shadow, outlined secondary
- ✅ **Meta Info**: Smaller, more subtle with green pulse dot
- ✅ **Right Card**: Glassmorphic "Quick Start" preview matching Fumadocs sidebar style

#### Features Section
- ✅ Grid layout with subtle cards (white/[0.03] backgrounds)
- ✅ Icon containers with gradient backgrounds
- ✅ Hover effects with border color transitions
- ✅ Arrow markers (→) instead of bullet points for highlights
- ✅ Smaller, more refined spacing

#### Tech Stack Section
- ✅ Badge-based tech items instead of large cards
- ✅ Uppercase category labels
- ✅ Minimal hover effects
- ✅ CircuitBoard icon in section badge

#### How It Works Section
- ✅ Numbered circles with gradient backgrounds
- ✅ Connecting lines between steps (purple gradient)
- ✅ More compact card design
- ✅ Workflow icon in section badge

#### Team Section
- ✅ Minimal 2-column card grid
- ✅ Colored role descriptions (yellow/orange)
- ✅ Users icon in section badge

#### CTA Section
- ✅ Large gradient card with subtle overlay
- ✅ Yellow/orange radial gradient background
- ✅ Prominent heading (6xl)
- ✅ Shadow effects on primary button

#### Footer
- ✅ Minimal single-row layout
- ✅ Logo in gradient box
- ✅ Dot separators (·) between links
- ✅ Subtle gray-500 text

### 🎭 Animation Enhancements

All original Framer Motion animations preserved:
- Hero section fade-in with slide up
- Staggered card reveals (viewport triggers)
- Icon scale on hover
- Border color transitions

**WebGL Background** (AnimatedHeroBackground.tsx):
- Particle system with 150+ dots
- Three gradient orbs (yellow, orange, red)
- Connection lines between nearby particles
- Smooth vignette effect
- Blur filter for soft aesthetic
- *Note: Performance optimized with requestAnimationFrame*

### 📱 Responsive Design

All breakpoints maintained:
- Mobile: Single column, smaller text
- Tablet (md): 2 columns where appropriate
- Desktop (lg): Full layout with 3-4 columns
- Extra Large (xl): Maximum readability

### 🔧 Technical Improvements

#### Icons (Lucide React)
- **Sparkles**: Badge decorations
- **CircuitBoard**: Tech section
- **Workflow**: How It Works section
- **Users**: Team section
- **Boxes**: Modular architecture
- **ExternalLink**: Demo button
- **Menu**: Mobile navigation

#### Tailwind Classes
- More precise opacity values: `white/[0.03]`, `white/[0.08]`
- Consistent rounded corners: `rounded-lg`, `rounded-xl`
- Tighter spacing: `gap-3`, `gap-4` instead of `gap-6`
- Better backdrop blur: `backdrop-blur-xl`, `backdrop-blur-2xl`

#### Performance
- Reduced animation delays (0.05s instead of 0.1s)
- Optimized gradient calculations
- Minimal DOM elements in cards

### ✨ Key Visual Features

1. **Glassmorphism**: All cards use gradient transparency with backdrop blur
2. **Radial Gradients**: Orange/yellow orbs in hero background
3. **Soft Borders**: Nearly invisible borders (white/5 to white/10)
4. **Hover States**: Subtle scale transforms and border color changes
5. **Typography Hierarchy**: Clear size progression (text-xs to text-8xl)
6. **Badge Style**: Consistent rounded-full pills across all sections
7. **Shadow Effects**: Subtle yellow glow on primary buttons

### 📦 File Structure

```
frontend/
├── app/
│   ├── page.tsx              ✅ Updated with Fumadocs style
│   ├── layout.tsx            ✅ Already set
│   └── globals.css           ✅ Already set
├── components/
│   ├── ui/                   ✅ ShadCN components
│   └── animated-hero-background.tsx  ✅ WebGL animation
└── lib/
    └── utils.ts              ✅ Utility functions
```

## Content Preservation

✅ **All original content maintained**:
- Hero headline and description
- Feature titles and descriptions
- Tech stack items
- Workflow steps
- Team member information
- CTA messaging

**No paraphrasing** - only visual/structural changes applied.

## Browser Support

- Modern browsers with CSS backdrop-filter support
- Graceful degradation for older browsers
- WebGL canvas with fallback

## Next Steps

1. Test responsive layout on multiple devices
2. Verify animation performance
3. Check accessibility (WCAG compliance)
4. Add meta tags and OG images
5. Optimize bundle size

## Credits

- **Design Inspiration**: Fumadocs (G5OYQkibIAEsC2B.jpg)
- **Icons**: Lucide React
- **UI Components**: ShadCN UI
- **Animations**: Framer Motion
- **Styling**: TailwindCSS
