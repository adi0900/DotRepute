# DotRepute Architecture

## Table of Contents
1. [System Overview](#system-overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Component Architecture](#component-architecture)
4. [Data Flow](#data-flow)
5. [Technology Stack](#technology-stack)
6. [Security Considerations](#security-considerations)
7. [Scalability](#scalability)

---

## System Overview

DotRepute is a **Rust-powered** decentralized application (dApp) that computes and displays contributor reputation scores within the Polkadot ecosystem. The system aggregates on-chain data from multiple sources, processes it through a **Rust-based scoring engine**, and presents it through a user-friendly web interface.

**Key Philosophy**: Built *without creating a parachain* — instead, it layers a Rust-based scoring + contract system over existing parachains and Polkadot APIs.

### Core Principles

- **Rust-First**: Core logic written in Rust for performance, safety, and WASM compatibility
- **Transparency**: All scoring logic is open-source and auditable
- **Decentralization**: Data sourced directly from blockchain
- **User-Centric**: Privacy-respecting, wallet-controlled identity
- **Extensibility**: Modular design for easy feature additions
- **Performance**: Efficient indexing and caching strategies

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface                          │
│                    (Next.js Frontend)                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Dashboard  │  │   Profile    │  │   Search     │         │
│  │   Component  │  │   Component  │  │   Component  │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 │ REST API / GraphQL
                 │
┌────────────────┴────────────────────────────────────────────────┐
│                     Application Layer                           │
│                 (Rust Backend Services)                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Rust Scoring │  │    Cache     │  │   Rust API   │         │
│  │    Engine    │  │   (Redis)    │  │   Gateway    │         │
│  │   (WASM)     │  │              │  │  (Actix-Web) │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 │ GraphQL Queries
                 │
┌────────────────┴────────────────────────────────────────────────┐
│                      Data Layer                                 │
│                   (SubQuery Indexer)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Identity   │  │  Governance  │  │   Staking    │         │
│  │   Indexer    │  │   Indexer    │  │   Indexer    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 │ WebSocket / RPC
                 │
┌────────────────┴────────────────────────────────────────────────┐
│                    Blockchain Layer                             │
│             (Polkadot / Kusama / Parachains)                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Identity   │  │   OpenGov    │  │   Staking    │         │
│  │   Pallet     │  │   Pallet     │  │   Pallet     │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘

         ┌──────────────────────────────────────┐
         │  Optional: Smart Contract Layer      │
         │  (ink! on Contracts Parachain)       │
         │  - Reputation attestation            │
         │  - Verifiable credentials            │
         └──────────────────────────────────────┘
```

---

## Component Architecture

### 1. Frontend (Next.js)

**Purpose**: User interface for interacting with reputation data

**Key Responsibilities**:
- Display reputation scores and breakdowns
- Connect to Polkadot wallets (via Polkadot.js extension)
- Query indexed data from SubQuery
- Interact with backend API
- Visualize historical reputation trends

**Technology**:
- Next.js 14+ (App Router)
- React 18+
- TypeScript
- Polkadot.js API
- ShadCN UI + TailwindCSS
- Recharts for data visualization

**Key Modules**:
```
frontend/src/
├── app/                    # Next.js App Router
│   ├── page.tsx           # Home/Dashboard
│   ├── profile/[address]  # User profile pages
│   └── search/            # Search functionality
├── components/
│   ├── ui/                # Reusable UI components
│   ├── layout/            # Layout components
│   ├── charts/            # Chart components
│   └── wallet/            # Wallet connection
├── lib/
│   ├── polkadot/          # Blockchain interaction
│   │   ├── api.ts         # API initialization
│   │   ├── chain-data.ts  # Data fetching
│   │   └── reputation.ts  # Reputation calculation
│   └── utils/             # Utility functions
└── hooks/                 # Custom React hooks
```

---

### 2. Indexer (SubQuery)

**Purpose**: Index and aggregate on-chain events for efficient querying

**Key Responsibilities**:
- Listen to blockchain events
- Extract relevant data (identity, governance, staking)
- Store in PostgreSQL database
- Expose GraphQL API

**Technology**:
- SubQuery SDK
- PostgreSQL
- GraphQL
- TypeScript

**Indexed Events**:
```typescript
// Identity Events
- identity.IdentitySet
- identity.IdentityCleared
- identity.JudgementGiven

// Governance Events
- referenda.Submitted
- convictionVoting.Voted
- referenda.DecisionDepositPlaced

// Staking Events
- staking.Bonded
- staking.Unbonded
- staking.Rewarded
- staking.ValidatorPrefsSet

// General Activity
- system.ExtrinsicSuccess
- balances.Transfer
```

**Schema**:
```graphql
type Account @entity {
  id: ID!
  address: String! @index
  identityInfo: IdentityInfo
  stakingInfo: StakingInfo
  governanceActivity: [GovernanceVote!]
  extrinsicCount: Int!
  lastActive: Date
  reputationScore: Float
}

type IdentityInfo @entity {
  id: ID!
  account: Account!
  displayName: String
  legalName: String
  email: String
  twitter: String
  judgements: [Judgement!]
  verificationLevel: Int!
}

type StakingInfo @entity {
  id: ID!
  account: Account!
  totalStaked: BigInt!
  activeStake: BigInt!
  isValidator: Boolean!
  nominatorCount: Int
}

type GovernanceVote @entity {
  id: ID!
  account: Account!
  referendumId: Int!
  vote: String!
  conviction: Int!
  timestamp: Date!
}
```

---

### 3. Backend (Rust)

**Purpose**: Compute reputation scores and provide REST API

**Key Responsibilities**:
- Aggregate data from SubQuery
- Calculate reputation scores using Rust scoring engine
- Cache results (Redis)
- Provide REST API endpoints
- Optional: GitHub contribution integration

**Technology**:
- **Rust** (primary language)
- **Actix-Web** or **Rocket** (web framework)
- **Tokio** (async runtime)
- Redis for caching
- PostgreSQL (optional)
- WASM-compiled scoring modules

**Alternative**: Node.js/TypeScript for non-core services

**API Endpoints**:
```
GET  /api/reputation/:address        # Get reputation score
GET  /api/reputation/:address/breakdown  # Get score breakdown
GET  /api/reputation/:address/history    # Historical scores
GET  /api/leaderboard                # Top contributors
GET  /api/search?query=              # Search addresses
POST /api/reputation/refresh/:address    # Force recalculation
GET  /api/stats                      # Global statistics
```

**Scoring Engine Architecture (Rust)**:
```rust
// Rust scoring engine crate
pub struct ReputationEngine {
    weights: ScoringWeights,
}

impl ReputationEngine {
    pub async fn calculate_score(&self, address: &str) -> Result<ReputationScore> {
        // Fetch all component scores in parallel
        let (identity, governance, staking, activity, dev_contrib) = tokio::join!(
            self.get_identity_score(address),
            self.get_governance_score(address),
            self.get_staking_score(address),
            self.get_activity_score(address),
            self.get_dev_contribution_score(address),
        );

        // Calculate weighted score
        self.weighted_score(ComponentScores {
            identity: identity?,
            governance: governance?,
            staking: staking?,
            activity: activity?,
            dev_contrib: dev_contrib?,
        })
    }

    fn weighted_score(&self, scores: ComponentScores) -> Result<ReputationScore> {
        let total = scores.identity * self.weights.identity
            + scores.governance * self.weights.governance
            + scores.staking * self.weights.staking
            + scores.activity * self.weights.activity
            + scores.dev_contrib * self.weights.dev_contrib;

        Ok(ReputationScore {
            total,
            breakdown: scores,
        })
    }
}
```

---

### 4. Smart Contracts (Optional - ink!)

**Purpose**: On-chain reputation attestation and verification

**Key Responsibilities**:
- Store reputation attestations
- Issue verifiable credentials
- Enable reputation-gated features

**Technology**:
- ink! (Rust)
- WASM
- Contracts Parachain (Astar, Phala)

**Contract Interface**:
```rust
#[ink::contract]
mod reputation_registry {
    #[ink(storage)]
    pub struct ReputationRegistry {
        scores: Mapping<AccountId, ReputationData>,
        attestations: Mapping<u64, Attestation>,
    }

    #[ink(message)]
    pub fn register_score(&mut self, score: u32, proof: Hash) -> Result<()>;

    #[ink(message)]
    pub fn get_score(&self, account: AccountId) -> Option<ReputationData>;

    #[ink(message)]
    pub fn attest(&mut self, account: AccountId, attestation_type: u8) -> Result<()>;
}
```

---

## Data Flow

### Score Calculation Flow

```
1. User requests reputation for address "5ABC..."
   ↓
2. Frontend → Backend API: GET /api/reputation/5ABC...
   ↓
3. Backend checks cache (Redis)
   ├─ Cache Hit: Return cached score
   └─ Cache Miss: Continue to step 4
   ↓
4. Backend → SubQuery: GraphQL query for all data
   ↓
5. SubQuery → PostgreSQL: Fetch indexed data
   ↓
6. Backend: Compute reputation score
   - Calculate identity score (0-100)
   - Calculate governance score (0-100)
   - Calculate staking score (0-100)
   - Calculate activity score (0-100)
   - Apply weighted formula
   ↓
7. Backend: Store in cache (TTL: 1 hour)
   ↓
8. Backend → Frontend: Return ReputationScore
   ↓
9. Frontend: Display score and breakdown
```

### Real-Time Updates Flow

```
1. New block finalized on Polkadot
   ↓
2. SubQuery indexer detects relevant events
   ↓
3. SubQuery processes and stores in PostgreSQL
   ↓
4. WebSocket notification (optional)
   ↓
5. Frontend subscribes to updates
   ↓
6. Cache invalidation triggered
   ↓
7. Frontend refetches updated score
```

---

## Technology Stack

### Core (Rust)
- **Language**: Rust (primary)
- **Web Framework**: Actix-Web / Rocket
- **Async Runtime**: Tokio
- **WASM**: wasm-bindgen, wasm-pack
- **Smart Contracts**: ink! 4.x
- **Testing**: cargo test, criterion (benchmarks)

### Frontend
- **Framework**: Next.js 14+ (App Router)
- **Language**: TypeScript (minimal, interface-only)
- **UI Library**: React 18+
- **Styling**: TailwindCSS + ShadCN
- **State Management**: React Context / Zustand
- **Blockchain**: Polkadot.js API
- **Charts**: Recharts / Victory
- **Testing**: Jest, React Testing Library, Playwright

### Backend (Optional Alternative)
- **Runtime**: Node.js 18+ (for non-core services)
- **Framework**: Express / Fastify
- **Language**: TypeScript
- **Cache**: Redis
- **Database**: PostgreSQL (optional)
- **API**: REST + GraphQL client
- **Testing**: Jest, Supertest

### Indexer
- **Framework**: SubQuery
- **Database**: PostgreSQL 12+
- **API**: GraphQL (Apollo)
- **Language**: TypeScript

### Infrastructure
- **Container**: Docker
- **Orchestration**: Docker Compose / Kubernetes
- **CI/CD**: GitHub Actions
- **Hosting**: Vercel (frontend), AWS/GCP (backend)

---

## Security Considerations

### 1. Data Integrity
- All data sourced from blockchain (immutable)
- SubQuery indexer validates events
- Scoring logic is deterministic and auditable

### 2. Privacy
- No PII stored beyond on-chain data
- Users control their wallet addresses
- Optional off-chain data (GitHub) requires explicit consent

### 3. API Security
- Rate limiting on all endpoints
- CORS configuration
- API key authentication for sensitive operations
- Input validation and sanitization

### 4. Smart Contract Security (ink!)
- Formal verification of ink! contracts
- Audit by third-party security firms
- Upgradeable proxy pattern
- Multi-sig admin controls
- Memory-safe Rust prevents common vulnerabilities

### 5. Rust Security Benefits
- Memory safety without garbage collection
- No null pointer dereferencing
- Thread safety guaranteed at compile time
- Zero-cost abstractions prevent performance vulnerabilities

### 6. Frontend Security
- Content Security Policy (CSP)
- Subresource Integrity (SRI)
- XSS protection
- Secure wallet connections (no private key exposure)

---

## Scalability

### Horizontal Scaling

**Frontend**:
- Static site generation (SSG) where possible
- CDN distribution (Vercel Edge)
- Image optimization

**Backend**:
- Stateless API servers
- Load balancer (nginx/HAProxy)
- Multiple instances behind load balancer

**Indexer**:
- Sharding by parachain
- Read replicas for PostgreSQL
- Separate indexers per network (Polkadot/Kusama)

### Caching Strategy

```
┌─────────────┐
│   Browser   │ ← Client-side cache (1 min)
└──────┬──────┘
       │
┌──────┴──────┐
│     CDN     │ ← Edge cache (5 min)
└──────┬──────┘
       │
┌──────┴──────┐
│    Redis    │ ← Server cache (1 hour)
└──────┬──────┘
       │
┌──────┴──────┐
│  PostgreSQL │ ← Indexed data (permanent)
└─────────────┘
```

### Performance Targets

- API response time: < 200ms (p95)
- Frontend initial load: < 2s
- Time to interactive (TTI): < 3s
- SubQuery sync lag: < 5 blocks

### Database Optimization

- Indexed columns: `address`, `timestamp`, `referendumId`
- Materialized views for leaderboard
- Partitioning by time range
- Query result pagination

---

## Future Enhancements

### Phase 2
- Multi-chain support (Kusama, parachains)
- Advanced filtering and search
- Reputation badges/NFTs

### Phase 3
- Machine learning for fraud detection
- Predictive reputation modeling
- Cross-chain identity aggregation

### Phase 4
- DAO integration
- Reputation-gated features
- Validator performance scoring
- Community governance of weights

---

## Deployment Architecture

### Development
```
localhost:3000 → Frontend
localhost:4000 → Backend API
localhost:3001 → SubQuery GraphQL
localhost:6379 → Redis
localhost:5432 → PostgreSQL
```

### Production
```
dotrepute.io → Frontend (Vercel)
api.dotrepute.io → Backend (AWS ECS)
indexer.dotrepute.io → SubQuery (SubQuery Network)
Redis (AWS ElastiCache)
PostgreSQL (AWS RDS)
```

---

## Monitoring & Observability

### Metrics
- API request rate and latency
- Cache hit ratio
- Indexer sync progress
- Error rates

### Logging
- Structured logging (JSON)
- Log aggregation (CloudWatch/Datadog)
- Error tracking (Sentry)

### Alerting
- API downtime
- Indexer sync lag > threshold
- High error rate
- Cache eviction rate

---

## References

- [Polkadot Architecture](https://wiki.polkadot.network/docs/learn-architecture)
- [SubQuery Documentation](https://academy.subquery.network/)
- [ink! Smart Contracts](https://use.ink/)
- [Next.js Documentation](https://nextjs.org/docs)
