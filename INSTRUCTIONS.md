# DotRepute - Setup and Running Instructions

## Overview
DotRepute is a Rust-powered Contributor Reputation System for the Polkadot ecosystem with a Next.js frontend dashboard.

---

## Prerequisites

### Required Software
- **Node.js** (v18 or higher) - [Download](https://nodejs.org/)
- **npm** or **yarn** - Package manager (comes with Node.js)
- **Git** - Version control
- **Rust** (optional, for backend development) - [Install](https://rustup.rs/)

---

## Frontend Setup

### 1. Clone the Repository
```bash
git clone https://github.com/adi0900/DotRepute.git
cd DotRepute
```

### 2. Navigate to Frontend Directory
```bash
cd frontend
```

### 3. Install Dependencies
```bash
npm install
```
or if using yarn:
```bash
yarn install
```

### 4. Run Development Server
```bash
npm run dev
```
or with yarn:
```bash
yarn dev
```

The application will be available at:
- **Local**: http://localhost:3000
- **Network**: Check terminal output for network URL

### 5. Build for Production
```bash
npm run build
npm start
```

---

## Project Structure

```
DotRepute/
├── frontend/                 # Next.js application
│   ├── app/                 # Next.js 14 app directory
│   │   ├── page.tsx        # Landing page
│   │   ├── dashboard/      # Dashboard page
│   │   ├── auth/           # Authentication page
│   │   ├── docs/           # Documentation page
│   │   └── resources/      # Resources page
│   ├── components/         # React components
│   │   ├── ui/            # ShadCN UI components
│   │   └── venture-navbar.tsx  # Reusable navbar
│   ├── public/            # Static assets
│   │   └── favicon.svg    # Site favicon
│   ├── styles/            # Global styles
│   ├── package.json       # Dependencies
│   └── tsconfig.json      # TypeScript config
│
├── Rust Backend Files/      # Rust scoring engine
│   ├── Standalone Rust Crate lib.rs
│   ├── Standalone Rust Crate scoring.rs
│   ├── WASM modules/
│   └── ink! contracts/
│
└── docs/                   # Documentation
    ├── architecture.md
    ├── scoring-model.md
    └── ...
```

---

## Available Pages

### 1. **Landing Page** (`/`)
- Hero section with Polkadot theme
- Features overview
- Team section
- CTA section
- Theme toggle (Light/Dark mode)

### 2. **Dashboard** (`/dashboard`)
- Interactive chat interface
- Bot conversation flow
- Left sidebar with:
  - Chat sessions management
  - Bookmarks functionality
- Chat persistence using localStorage
- Login/Signup section

### 3. **Authentication** (`/auth`)
- Login/Signup tabs
- Form validation
- Social login buttons (GitHub, Google)
- Venture-styled design

### 4. **Documentation** (`/docs`)
- Sidebar navigation with sections:
  - Overview
  - Architecture
  - Scoring Model
  - Data Sources
  - Development
- Center-aligned content

### 5. **Resources** (`/resources`)
- Sidebar navigation with sections:
  - Tech Stack
  - Repository Structure
  - Development Setup
  - Key Features
  - External Resources
  - Development Tools
- Center-aligned content
- External links to documentation

---

## Key Features

### Theme System
- **Light/Dark Mode Toggle** in navbar
- Theme persists across sessions using localStorage
- Consistent Venture design system

### Venture Design System
- Bordered boxes with clean lines
- Monospace typography for labels
- Uppercase tracking for headers
- Minimalist color palette (black/white with orange accents)
- Responsive grid layouts

### Chat System (Dashboard)
- **Natural language conversation flow**
- **Message types**: text, data, question
- **Session management**: Multiple chat sessions
- **Bookmarks**: Save important messages
- **Persistence**: All data stored in localStorage
- **Typing indicator**: Animated dots while bot responds

### Navigation
- **Reusable VentureNavbar component**
- Active page highlighting
- Responsive design
- Theme-aware styling
- Login/Signup or Logout based on auth state

---

## Tech Stack

### Frontend
- **Next.js 14** - React framework with App Router
- **React** - UI library
- **TypeScript** - Type safety
- **TailwindCSS** - Utility-first CSS
- **ShadCN UI** - Pre-built components
- **Lucide React** - Icon library
- **Framer Motion** - Animations

### Backend (Optional - Rust)
- **Rust** - Core scoring engine
- **ink!** - Smart contracts
- **WASM** - WebAssembly modules
- **SubQuery** - Blockchain indexing
- **PolkadotJS API** - Blockchain interaction

---

## Development Commands

### Frontend Commands
```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Lint code
npm run lint

# Type check
npm run type-check
```

### Git Commands
```bash
# Check status
git status

# Add files
git add .

# Commit changes
git commit -m "your message"

# Push to remote
git push origin branch-name

# Pull latest changes
git pull origin main

# Create new branch
git checkout -b branch-name
```

---

## Environment Variables

Create a `.env.local` file in the `frontend/` directory:

```env
# Optional: Add any environment variables here
NEXT_PUBLIC_APP_NAME=DotRepute
NEXT_PUBLIC_APP_URL=http://localhost:3000
```

---

## Troubleshooting

### Port Already in Use
If port 3000 is already in use, Next.js will automatically try 3001, 3002, etc.

To manually specify a port:
```bash
npm run dev -- -p 3001
```

### Module Not Found Errors
```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Build Errors
```bash
# Clear Next.js cache
rm -rf .next
npm run build
```

### Theme Not Persisting
- Check browser localStorage is enabled
- Clear browser cache and try again

---

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

MIT License - see LICENSE file for details

---

## Support

For issues or questions:
- **GitHub Issues**: [https://github.com/adi0900/DotRepute/issues](https://github.com/adi0900/DotRepute/issues)
- **Documentation**: Check the `/docs` folder

---

## Team

- **Aditya** - Product Designer & Product Manager
- **Steven Muanigo** - Backend & Infrastructure Developer

---

## Quick Start Summary

```bash
# 1. Clone repository
git clone https://github.com/adi0900/DotRepute.git

# 2. Navigate to frontend
cd DotRepute/frontend

# 3. Install dependencies
npm install

# 4. Run development server
npm run dev

# 5. Open browser to http://localhost:3000
```

**That's it! You're ready to go! 🚀**
