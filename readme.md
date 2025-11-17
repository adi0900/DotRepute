# **DotRepute — A Polkadot-Native Contributor Reputation System (CRS)**

### *Bringing Web2 Reputation Infrastructure Into Web3*

---

## **📌 Overview**

**DotRepute** is a **real-time, blockchain-powered reputation engine** built natively for the Polkadot ecosystem.
It transforms fragmented on-chain activity into **transparent, verifiable, user-owned reputation profiles** — similar to how LinkedIn, GitHub, and StackOverflow shape identity in Web2.

DotRepute leverages **PolkadotJS API**, **Identity**, **OpenGov**, and **Staking pallets** to compute a multi-dimensional reputation score for any address on the network.
It includes a fully interactive **AI-powered dashboard**, wallet connection, real-time score computation, and clear reputation breakdowns.

This project was built for the **Polkadot “Bring Web2 Applications to Web3” Hackathon** under the theme:

> **User-Centric Apps — Real-World Web3 Impact Using Polkadot Technology Stack**

---

## **🎯 Key Features**

### **1. Real-Time On-Chain Reputation Scoring**

A multi-dimensional scoring model using five weighted components:

* **Identity Score (25%)** – Fields completeness + registrar verification
* **Governance Score (25%)** – Referenda participation and voting activity
* **Staking Score (20%)** – DOT staked, nominations, validator trust
* **Activity Score (20%)** – Frequency of extrinsics and on-chain operations
* **Development Score (10%)** – GitHub contribution signals *(Roadmap)*

The system pulls data **directly from Polkadot** — no caching, no intermediaries.

---

### **2. AI-Powered Reputation Assistant**

A conversational interface that explains:

* Why a user received their score
* Identity breakdown
* Governance participation metrics
* Staking insights
* Trust tier and percentile ranking
* Personalized improvement recommendations
* 30-day and 90-day score trends
* Validator and nominator analysis

Users can **chat with their own reputation**, similar to ChatGPT — but powered by Polkadot data.

---

### **3. Wallet Integration (Luno Kit)**

* Seamless Polkadot wallet connection
* Automatic score generation on connect
* Real-time updates when wallet activity changes

---

### **4. Exportable Reports**

* One-click export of entire conversation
* Generates professional .docx reports
* Useful for grants, job applications, DAO membership, validator profiles

---

### **5. Beautiful, User-Centric Interface**

* Next.js 14 App Router
* TailwindCSS v4
* Framer Motion animations
* Dark modern dashboard
* Fast, responsive interactions

---

## **🏗️ Tech Stack**

### **Frontend**

* **Next.js 14**
* **React + TypeScript**
* **TailwindCSS v4**
* **Luno Kit** for Polkadot wallet integration
* **Framer Motion**
* **Lucide Icons**

### **Backend**

* **Node.js + Express**
* **TypeScript**
* **PostgreSQL**
* **REST API**
* (Roadmap) Rust microservices for high-performance scoring

### **Blockchain Integration**

* **@polkadot/api v16.5.2**
* **Direct RPC queries**
* **Identity, Staking, Governance pallets**
* **@polkadot/extension-inject**
* **@polkadot/util / util-crypto**

### **Future Enhancements**

* ink! smart-contract-based on-chain reputation storage
* SubQuery indexing for historical data
* WASM modules for client-side scoring
* GitHub-based Development Score

---

## **📦 Repository Structure**

```
DotRepute/
│
├── frontend/               # Next.js 14 dashboard (production)
│   ├── app/                # App Router pages
│   ├── components/         # UI components
│   ├── lib/                # Polkadot API interfaces
│   ├── styles/             # TailwindCSS
│   └── package.json
│
├── backend/                # Node.js REST API (production)
│   ├── src/routes/         # Endpoint definitions
│   ├── src/handlers/       # Business logic
│   ├── src/database/       # PostgreSQL schemas
│   └── package.json
│
├── contracts/              # ink! smart contracts (roadmap)
├── indexer/                # SubQuery indexer (roadmap)
└── docs/                   # Technical documentation
```

---

## **⚙️ Setup & Installation**

### **1. Clone Repository**

```bash
git clone https://github.com/<your-repo>/DotRepute.git
cd DotRepute
```

---

### **2. Frontend Setup**

```bash
cd frontend
npm install
npm run dev
```

Runs at: **[http://localhost:3000](http://localhost:3000)**

---

### **3. Backend Setup**

```bash
cd backend
npm install
npm run dev
```

Runs at: **[http://localhost:5000](http://localhost:5000)**

---

### **4. Polkadot Wallet**

Install any Polkadot-compatible wallet extension:

* Talisman
* Polkadot.js Extension
* SubWallet

DotRepute works seamlessly with them via **Luno Kit**.

---

## **🧠 Scoring Algorithm**

### **Weighted Reputation Formula**

```typescript
score =
  identity * 0.25 +
  governance * 0.25 +
  staking * 0.20 +
  activity * 0.20 +
  development * 0.10
```

Each component is computed using **verifiable, real-time on-chain data**.

Identity fields include:

* Display name
* Legal name
* Email
* Web
* Twitter
* Riot/Matrix
* Registrar judgements

Governance pulls from:
`convictionVoting.votingFor(address)`

Staking pulls from:
`staking.ledger(address)`
`staking.nominators(address)`

## **📚 Documentation**

All technical documents are available under `/docs` including:

* scoring-model.md
* architecture.md
* data-sources.md
* api-spec.md
* roadmap.md

---

## **🧪 Example Queries in the App**

* “What is my identity score?”
* “Show my governance participation.”
* “Breakdown my staking score.”
* “How can I increase my reputation?”
* “Show my 30-day reputation trend.”
* “Explain my DotRepute score like I’m 12.”

---

## **🎉 Hackathon Theme Fit**

DotRepute perfectly aligns with the Polkadot hackathon pillars:

| Theme                  | How DotRepute Fits                                                                        |
| ---------------------- | ----------------------------------------------------------------------------------------- |
| **User-Centric Apps**  | Gives users ownership of their on-chain reputation                                        |
| **Build a Blockchain** | Integrates deeply with Polkadot pallets and is designed for future chain-level deployment |
| **Polkadot Tinkerers** | Uses PolkadotJS API, identity, governance, staking, and real-time RPC calls               |

---

## **👥 Team**

**Aditya (Team Lead)**

* Polkadot API Integration
* Frontend & Backend
* UI/UX & System Architecture

**@openguildwtf || Dustin (Helper)**

* Wallet Connection Integration
* Critical workflow & Luno Kit setup

**Steven Muanigo**

* Rust Backend & Scoring Engine
* Infrastructure logic
* Future smart-contract extensions

A special thanks to the entire Polkadot community for documentation, tooling, and ecosystem support.

---

## **📄 License**

MIT License

---

## **🌐 Live Demo**

**[https://dotrepute.vercel.app](https://dotrepute.vercel.app)**

---

Ready for submission ✔
Optimized for judges ✔
Technically sound ✔
Clear impact on Web3 ✔

---
