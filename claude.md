# DotRepute - AI Context

## Project Overview
DotRepute is a **Blockchain-Powered Contributor Reputation System (CRS)** designed for the Polkadot ecosystem. It aggregates identity, governance, staking, and activity signals across the network to generate a transparent, verifiable, and user-centric reputation score with real-time on-chain data integration.

**Key Philosophy**: Built *without creating a parachain* — instead, it layers a sophisticated scoring system over existing Polkadot infrastructure using direct blockchain queries and intelligent AI-powered analytics.

**Current Status**: Fully functional MVP with production-ready frontend, live blockchain integration, AI chatbot interface, and comprehensive reputation analytics.

## Team
- **Aditya** — Product Designer & Product Manager (leads product direction, UX, flows, strategy)
- **Steven Muanigo** — Backend & Infrastructure Developer (builds Rust backend services, ink! modules, infra, and indexing)

## Tech Stack (Production Implementation)

### Core Infrastructure
- **PolkadotJS API** (v16.5.2) — Direct blockchain queries to Polkadot network
- **Real-time On-Chain Data** — Live integration with Identity, Governance, Staking pallets
- **Rust-Based Scoring Algorithm** — Weighted reputation calculations (implemented in TypeScript for MVP)

### Frontend (Live Production)
- **Next.js 14** — React-based framework with App Router
- **TypeScript** — Full type safety across application
- **TailwindCSS v4** — Modern utility-first styling
- **Luno Kit** — Polkadot wallet integration (@luno-kit/react, @luno-kit/ui)
- **Framer Motion** — Smooth animations and transitions
- **Lucide React** — Icon system
- **docx & file-saver** — Export chat history to Word documents

### Backend (Node.js/TypeScript)
- **Node.js/Express** — REST API server
- **TypeScript** — Type-safe backend implementation
- **PostgreSQL** — User data and workspace management
- **Authentication** — Secure session management

### Blockchain Integration
- **@polkadot/api** — Core Polkadot connectivity
- **@polkadot/extension-inject** — Wallet extension integration
- **@polkadot/util** & **@polkadot/util-crypto** — Cryptographic utilities

### Future Roadmap
- **Rust Microservices** — High-performance scoring engine
- **ink! Smart Contracts** — On-chain reputation storage
- **WASM Modules** — Browser-based reputation calculations
- **SubQuery Indexing** — Historical data aggregation

## Repository Structure (Current Implementation)

```
DotRepute/
├── frontend/                    # Next.js 14 Application (PRODUCTION)
│   ├── app/                    # App Router
│   │   ├── page.tsx           # Landing page
│   │   ├── dashboard/         # Main dashboard with AI chatbot
│   │   │   └── page.tsx       # Interactive reputation interface
│   │   └── layout.tsx         # Root layout
│   ├── components/            # React components
│   │   ├── venture-navbar.tsx # Navigation with wallet integration
│   │   └── ui/               # Reusable UI components
│   ├── lib/                  # Core libraries
│   │   └── polkadot-api.ts   # PolkadotJS API infrastructure
│   ├── styles/               # TailwindCSS configuration
│   └── package.json          # Dependencies
│
├── backend/                   # Node.js/TypeScript API (PRODUCTION)
│   ├── src/
│   │   ├── routes/           # API routes
│   │   │   └── index.ts      # Main route definitions
│   │   ├── handlers/         # Request handlers
│   │   │   ├── auth/        # Authentication
│   │   │   ├── workspaces/  # Workspace management
│   │   │   ├── memberships/ # Team member management
│   │   │   └── reputation/  # Reputation endpoints
│   │   ├── middleware/       # Auth, rate limiting
│   │   └── database/        # PostgreSQL models
│   └── package.json
│
├── contracts/                # Future: ink! smart contracts
├── indexer/                  # Future: SubQuery indexer
├── docs/                     # Documentation
└── CLAUDE.md                # This file
```

## Key Features (Live Implementation)

### 1. AI-Powered Reputation Chatbot 🤖
Interactive conversational interface providing:
- **Real-time Reputation Analysis** — Complete breakdown of all 5 scoring components
- **Governance Insights** — Detailed voting history, participation metrics, community impact
- **Staking Analytics** — Comprehensive staking data, nomination status, rewards tracking
- **Identity Verification** — On-chain identity parsing with registrar verification status
- **Leaderboard Comparison** — Rank estimation, percentile placement, badge system
- **Historical Trends** — 30-day and 90-day score progression analysis
- **Personalized Recommendations** — AI-generated improvement suggestions with point estimates
- **Chat Export** — Download complete conversation history as formatted Word documents

### 2. Direct Blockchain Integration
Real-time on-chain data queries via PolkadotJS API:
- **Identity Pallet** — Parse display name, legal name, email, Twitter, web fields
- **Conviction Voting** — Track governance participation across all referendum votes
- **Staking Pallet** — Query ledger, nominations, validator selections, staked amounts
- **System Account** — Balance tracking (free, reserved, frozen balances)
- **Registrar Judgements** — Identity verification status from trusted registrars

### 3. Sophisticated Scoring Algorithm
Rust-inspired weighted calculation system:
```
Total Score = (Identity × 25%) + (Governance × 25%) + (Staking × 20%) + (Activity × 20%) + (Development × 10%)
```
- **Identity Score (25%)**: Based on fields set + registrar verification
- **Governance Score (25%)**: Vote count with ~20 votes = 100% score
- **Staking Score (20%)**: Proportional to DOT staked (1000+ DOT = excellent)
- **Activity Score (20%)**: On-chain transaction frequency and engagement
- **Development Score (10%)**: GitHub contributions (future enhancement)

### 4. Badge & Tier System
Six-tier reputation ranking:
- 🥇 **Elite** (90-100): Top 1% of contributors
- 🥈 **Advanced** (80-89): Top 5% of contributors
- 🥉 **Proficient** (70-79): Top 15% of contributors
- 🎖️ **Competent** (60-69): Top 35% of contributors
- ⭐ **Active** (50-59): Top 50% of contributors
- 🌱 **Growing** (<50): Top 65% of contributors

### 5. Wallet Integration & Session Management
- **Luno Kit Wallet** — Seamless Polkadot wallet connection
- **Multi-Session Chat** — Save and switch between multiple chat sessions
- **Bookmark System** — Mark important messages for later reference
- **Real-time Updates** — Live blockchain data synchronization

## Rust-Based Scoring Formula

Full formula in `docs/scoring-model.md`.

The scoring engine is a Rust crate:
```
crs_score = (identity_score * 0.25)
          + (governance_score * 0.25)
          + (staking_score * 0.20)
          + (activity_score * 0.20)
          + (dev_score * 0.10)
```

**Features**:
- Fully modular
- Compile to WASM
- Reusable in backend or contracts
- Testable with Rust unit tests

## Data Sources
| Domain | On-Chain Source | Purpose |
|--------|-----------------|---------|
| Identity | Identity Pallet | Real user trust |
| Governance | OpenGov Referenda | Civic participation |
| Staking | Staking Pallet | Skin-in-the-game |
| Activity | Extrinsics | Frequency & engagement |
| Dev Signals | GitHub (optional) | Open-source contributions |

## Development Setup

### 1. Rust Backend (if enabled)
```bash
cd backend
cargo run
```

### 2. ink! Contract (optional)
```bash
cd contracts/crs_contract
./build.sh
```

### 3. SubQuery Indexer
```bash
cd indexer
npm install
subql codegen
subql build
subql query
```

### 4. Frontend
```bash
cd frontend
npm install
npm run dev
```

## Key Directories

### Frontend (`frontend/src/`)
- `components/` - UI components (ui, layout, charts)
- `pages/` or `app/` - Next.js pages/app directory
- `hooks/` - React hooks
- `lib/polkadot/` - Polkadot integration
  - `api.ts` - API connections
  - `chain-data.ts` - Chain data fetching
  - `reputation.ts` - Reputation logic
- `styles/` - Styling
- `constants/` - Constants
- `store/` - State management
- `types/` - TypeScript types

### Indexer (`indexer/`)
- `subquery.yaml` - SubQuery configuration
- `schema.graphql` - GraphQL schema
- `src/mappings/` - Event handlers
- `src/utils/` - Utilities

### Backend (`backend/src/`)
- `routes/` - API routes
- `services/` - Business logic
- `models/` - Data models

### Documentation (`docs/`)
- `architecture.md` - System architecture
- `scoring-model.md` - Scoring formula details
- `data-sources.md` - Data source specifications
- `ui-wireframes.md` - UI designs
- `api-spec.md` - API specifications
- `installation.md` - Installation guide
- `roadmap.md` - Project roadmap

## Development Phases

### Phase 1 — Rust Scoring Engine
- Identity parsing
- Staking weight
- Governance score
- WASM build

### Phase 2 — SubQuery Integration
- Map on-chain activity to Rust engine
- Time-weighted scoring

### Phase 3 — ink! Contract (Optional)
- Store reputation on chain
- Verification functions
- Emit events

### Phase 4 — Dashboard + Rest API
- Rust or lightweight TS backend
- Next.js UI

## License
MIT License
