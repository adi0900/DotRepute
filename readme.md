# 🦀 **DotRepute — Contributor Reputation System (CRS)**

### *A Rust-Powered Reputation Layer for the Polkadot Ecosystem*

**Team:**

* **Aditya** — Product Designer & Product Manager
* **Steven Muanigo** — Backend & Infrastructure Developer

---

## 🚀 **Overview**

**DotRepute** is a **Rust-powered Contributor Reputation System (CRS)** designed for the Polkadot ecosystem.
It aggregates identity, governance, staking, and activity signals across the network to generate a **transparent, verifiable, and user-centric reputation score.**

DotRepute intentionally uses **Rust as the core development language**, leveraging:

* ✅ **ink! smart contracts** (optional)
* ✅ **Rust-based microservices** (optional)
* ✅ **Rust-native tooling in Polkadot ecosystem**
* ✅ **WASM runtimes & PolkadotJS API bindings**

This dApp is built *without creating a parachain or using the Polkadot SDK to build a new blockchain* — instead, it layers a Rust-based scoring + contract system over existing parachains and Polkadot APIs.

DotRepute aligns with Polkadot’s hackathon themes:
✅ **User-Centric Apps**
✅ Bring Web2 → Web3
✅ Rust-first development for real-world impact

---

# 🦀 **Why Rust?**

Rust is used as the primary language for DotRepute because it provides:

* ⚡ **High-performance WASM execution**
* 🔐 **Memory safety and zero-cost abstractions**
* 🔧 **Native compatibility with Polkadot / Substrate / ink!**
* 📦 **The best developer experience for blockchain runtime-like logic**

---

# 🎯 **Key Features**

### ✅ **1. Rust-based Reputation Engine**

Core scoring logic is implemented in Rust as:

* A standalone Rust crate
* Fully testable modules
* WASM-compilable if integrated into ink!

### ✅ **2. Optional ink! Smart Contract Layer**

A Rust/ink! module can store:

* Reputation values
* Proof-of-reputation events
* User verifications

### ✅ **3. SubQuery Indexing**

Indexes governance + identity + staking events for scoring input.

### ✅ **4. Rust-first Architecture**

Where others use JS middleware, DotRepute uses Rust for:

* Scoring engine
* Data cleaning logic
* WASM-optimized utilities

### ✅ **5. React-based Dashboard (non-core layer)**

Frontend is lightweight and only interacts with:

* PolkadotJS API
* Rust scoring engine
* Optional ink! contract

---

# 📦 **Repository Structure**

```
crs-dapp/
│
├── contracts/                     # ink! smart contracts (Rust + WASM)
│   ├── Cargo.toml
│   └── crs_contract/
│       ├── lib.rs
│       ├── Cargo.toml
│       ├── build.sh
│       └── README.md
│
├── frontend/                      # Frontend (React/Next.js)
│   ├── public/
│   ├── src/
│   │   ├── components/
│   │   ├── pages/ or app/
│   │   ├── lib/
│   │   │   ├── polkadot/
│   │   │   └── utils.ts
│   │   ├── styles/
│   │   ├── constants/
│   │   ├── store/
│   │   ├── types/
│   │   └── index.tsx
│   └── package.json
│
├── indexer/                       # SubQuery Indexer (Rust-compatible data flow)
│   ├── subquery.yaml
│   ├── schema.graphql
│   └── src/
│       ├── mappings/
│       └── utils/
│
├── backend/                       # OPTIONAL: Rust microservices (preferred)
│   ├── Cargo.toml                 # If using Rust-based scoring services
│   └── src/
│       ├── main.rs
│       ├── scoring/
│       ├── models/
│       └── api/
│
├── docs/
│   ├── architecture.md
│   ├── scoring-model.md
│   ├── data-sources.md
│   ├── ui-wireframes.md
│   ├── api-spec.md
│   ├── installation.md
│   ├── roadmap.md
│   └── README.md
│
├── scripts/
│   ├── deploy-contract.sh
│   ├── generate-types.sh
│   ├── index-chain-data.ts
│   └── seed-demo-data.ts
│
├── tests/
│   ├── rust/
│   ├── frontend/
│   ├── contracts/
│   └── e2e/
│
├── .github/
│   ├── workflows/
│   └── ISSUE_TEMPLATE/
│
├── LICENSE
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
└── README.md
```

---

# 🛠️ **Tech Stack (Rust First)**

### 🦀 **Core**

* **Rust** — primary backend + scoring + contract language
* **ink!** — optional smart contracts
* **WASM** — compiled Rust modules
* **PolkadotJS API** — lightweight interaction from frontend
* **SubQuery** — indexing governance/identity/staking events

### 🎨 **Frontend**

* Next.js / React
* TailwindCSS / ShadCN UI
* TypeScript (minimal usage, interface-only)

---

# 🔗 **Data Sources (CRS Inputs)**

DotRepute aggregates Rust-parsed data from:

| Domain                 | On-Chain Source   | Purpose                   |
| ---------------------- | ----------------- | ------------------------- |
| Identity               | Identity Pallet   | Real user trust           |
| Governance             | OpenGov Referenda | Civic participation       |
| Staking                | Staking Pallet    | Skin-in-the-game          |
| Activity               | Extrinsics        | Frequency & engagement    |
| Dev Signals (optional) | GitHub            | Open-source contributions |

---

# 🧠 **Rust-Based Scoring Model (Summary)**

Full formula in `docs/scoring-model.md`.

The scoring engine is a Rust crate:

```
crs_score = (identity_score * 0.25)
          + (governance_score * 0.25)
          + (staking_score * 0.20)
          + (activity_score * 0.20)
          + (dev_score * 0.10)
```

* Fully modular
* Compile to WASM
* Reusable in backend or contracts
* Testable with Rust unit tests

---

# 🧭 **Roadmap**

### ✅ **Phase 1 — Rust Scoring Engine**

* Identity parsing
* Staking weight
* Governance score
* WASM build

### ✅ **Phase 2 — SubQuery Integration**

* Map on-chain activity to Rust engine
* Time-weighted scoring

### ✅ **Phase 3 — ink! Contract (Optional)**

* Store reputation on chain
* Verification functions
* Emit events

### ✅ **Phase 4 — Dashboard + Rest API**

* Rust or lightweight TS backend
* Next.js UI

---

# 🛠️ **Running the Project**

## ✅ **1. Start Rust Backend (if enabled)**

From `/backend`:

```
cargo run
```

## ✅ **2. Build ink! Contract (optional)**

```
cd contracts/crs_contract
./build.sh
```

## ✅ **3. Start SubQuery Indexer**

```
cd indexer
npm install
subql codegen
subql build
subql query
```

## ✅ **4. Start Frontend**

```
cd frontend
npm install
npm run dev
```

---

# 👨‍💻 **Contributors**

### 🧑‍🎨 **Aditya**

Product Designer & Product Manager
Leads product direction, UX, flows, strategy, and user research.

### 🧑‍💻 **Steven Muanigo**

Backend & Infrastructure Developer
Builds Rust backend services, ink! modules, infra, and indexing.

---

# 🤝 **Contributing**

See the guidelines in **`CONTRIBUTING.md`**.

---

