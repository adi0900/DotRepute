# DotRepute - AI Context

## Project Overview
DotRepute is a **Rust-powered Contributor Reputation System (CRS)** designed for the Polkadot ecosystem. It aggregates identity, governance, staking, and activity signals across the network to generate a transparent, verifiable, and user-centric reputation score.

**Key Philosophy**: Built *without creating a parachain* — instead, it layers a Rust-based scoring + contract system over existing parachains and Polkadot APIs.

## Team
- **Aditya** — Product Designer & Product Manager (leads product direction, UX, flows, strategy)
- **Steven Muanigo** — Backend & Infrastructure Developer (builds Rust backend services, ink! modules, infra, and indexing)

## Tech Stack (Rust-First)

### Core (Rust)
- **Rust** — primary backend + scoring + contract language
- **ink!** — optional smart contracts
- **WASM** — compiled Rust modules
- **PolkadotJS API** — lightweight interaction from frontend
- **SubQuery** — indexing governance/identity/staking events

### Frontend
- Next.js / React
- TailwindCSS / ShadCN UI
- TypeScript (minimal usage, interface-only)

### Backend (Optional)
- Rust microservices (preferred via Cargo.toml)
- Alternative: Node.js/Express/Fastify for non-core services

### Smart Contract (Optional)
- ink! (Rust)
- WASM/Contracts Parachain (Astar, Phala, Shiden)

## Repository Structure

```
crs-dapp/
├── contracts/          # ink! smart contracts (Rust + WASM)
│   ├── Cargo.toml
│   └── crs_contract/
│       ├── lib.rs
│       ├── Cargo.toml
│       ├── build.sh
│       └── README.md
├── frontend/           # Next.js application
│   └── src/
│       ├── components/
│       ├── pages/ or app/
│       ├── lib/polkadot/
│       └── ...
├── indexer/            # SubQuery indexer (Rust-compatible data flow)
│   ├── subquery.yaml
│   ├── schema.graphql
│   └── src/
├── backend/            # OPTIONAL: Rust microservices (preferred)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── scoring/
│       ├── models/
│       └── api/
├── docs/              # Documentation
├── scripts/           # Utility scripts
├── tests/             # Test suites (rust/, frontend/, contracts/, e2e/)
└── .github/           # CI/CD workflows
```

## Key Features

### 1. Rust-based Reputation Engine
Core scoring logic implemented in Rust as:
- Standalone Rust crate
- Fully testable modules
- WASM-compilable if integrated into ink!

### 2. Optional ink! Smart Contract Layer
Rust/ink! module can store:
- Reputation values
- Proof-of-reputation events
- User verifications

### 3. SubQuery Indexing
Indexes governance + identity + staking events for scoring input

### 4. Rust-first Architecture
Where others use JS middleware, DotRepute uses Rust for:
- Scoring engine
- Data cleaning logic
- WASM-optimized utilities

### 5. React-based Dashboard (non-core layer)
Frontend is lightweight and only interacts with:
- PolkadotJS API
- Rust scoring engine
- Optional ink! contract

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
