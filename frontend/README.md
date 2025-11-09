# DotRepute Frontend

A premium, visually striking landing page for DotRepute - the Rust-powered contributor reputation system for Polkadot.

## Features

- **Animated Hero Section**: WebGL-powered gradient and particle effects inspired by Fumadocs
- **Glassmorphic Design**: Modern UI with soft gradients, blur effects, and neon accents
- **ShadCN UI Components**: Premium component library with TailwindCSS
- **Framer Motion Animations**: Smooth entry animations for sections and cards
- **Responsive Design**: Fully responsive across all device sizes
- **Performance Optimized**: Clean WebGL implementation with minimal overhead

## Tech Stack

- **Next.js 14**: React framework with App Router
- **React 18**: UI library
- **TypeScript**: Type-safe development
- **TailwindCSS**: Utility-first CSS framework
- **ShadCN UI**: Component library
- **Framer Motion**: Animation library
- **Lucide React**: Icon library

## Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

Open [http://localhost:3000](http://localhost:3000) to view the landing page.

## Project Structure

```
frontend/
├── app/
│   ├── layout.tsx          # Root layout
│   ├── page.tsx            # Landing page
│   └── globals.css         # Global styles
├── components/
│   ├── ui/                 # ShadCN UI components
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   └── badge.tsx
│   └── animated-hero-background.tsx  # WebGL animation
├── lib/
│   └── utils.ts            # Utility functions
├── public/                 # Static assets
├── tailwind.config.ts      # Tailwind configuration
├── tsconfig.json           # TypeScript configuration
└── package.json            # Dependencies
```

## Customization

### Colors

Edit the color palette in `tailwind.config.ts` and `app/globals.css` to match your brand.

### Content

All content is sourced from `claude.md` in the root directory. Update sections in `app/page.tsx`:
- Features array
- Tech stack array
- Workflow steps
- Team information

### Animations

Modify the WebGL animation in `components/animated-hero-background.tsx`:
- Particle count
- Orb colors and sizes
- Movement speeds
- Blur intensity

## Performance

The landing page is optimized for performance:
- Lazy-loaded animations
- Minimal WebGL overhead
- Optimized images and assets
- Code splitting via Next.js

## License

MIT License - see LICENSE file for details

## Credits

Design inspiration from Fumadocs
Built by the DotRepute team
