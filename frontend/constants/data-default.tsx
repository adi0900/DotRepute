import { Boxes, Code2, Database, Shield, Zap, Lock } from "lucide-react";

export const FEATURES = [
  {
    icon: <Code2 className="w-5.5 h-5.5 text-orange-400" />,
    title: "Rust-based Reputation Engine",
    description:
      "Core scoring logic implemented in Rust with comprehensive weighted metrics, time decay, and modular architecture.",
    highlights: [
      "Fully testable modules",
      "WASM-compilable for portability",
      "Multi-metric scoring system",
    ],
  },
  {
    icon: <Database className="w-5.5 h-5.5 text-blue-400" />,
    title: "SubQuery Indexing",
    description:
      "Comprehensive blockchain data indexing for identity, governance, staking, and activity events.",
    highlights: [
      "Real-time event processing",
      "GraphQL API access",
      "Cross-domain correlation",
    ],
  },
  {
    icon: <Shield className="w-5.5 h-5.5 text-green-400" />,
    title: "Identity Verification",
    description:
      "Multi-source identity verification with Sybil protection and gradual trust level system.",
    highlights: [
      "On-chain and off-chain verification",
      "GDPR/KYC compliance",
      "Timestamped validity",
    ],
  },
  {
    icon: <Zap className="w-5.5 h-5.5 text-yellow-400" />,
    title: "Data Cleaning Engine",
    description:
      "Enterprise-grade data quality engine with validation, normalization, and anomaly detection.",
    highlights: [
      "Type validation",
      "Missing data handling",
      "Format standardization",
    ],
  },
  {
    icon: <Boxes className="w-5.5 h-5.5 text-purple-400" />,
    title: "Modular Architecture",
    description:
      "Extensible trait-based design allowing easy integration of custom metrics and scoring algorithms.",
    highlights: [
      "Plugin system",
      "Custom metric support",
      "Configurable weights",
    ],
  },
  {
    icon: <Lock className="w-5.5 h-5.5 text-red-400" />,
    title: "Transparent & Auditable",
    description:
      "All scoring logic is open-source and verifiable, with on-chain attestation support via ink! contracts.",
    highlights: [
      "Open-source scoring",
      "On-chain proofs",
      "Community governance",
    ],
  },
];

export const TECHSTACK = [
  {
    category: "Core (Rust)",
    items: ["Rust", "ink!", "WASM", "Tokio"],
  },
  {
    category: "Blockchain",
    items: ["PolkadotJS API", "SubQuery", "Substrate"],
  },
  {
    category: "Frontend",
    items: ["Next.js", "React", "TailwindCSS", "ShadCN UI"],
  },
  {
    category: "Infrastructure",
    items: ["Redis", "PostgreSQL", "Docker", "GraphQL"],
  },
];

export const WORKFLOW = [
  {
    title: "User Input",
    description:
      "User requests reputation data for an address or interacts with the system",
  },
  {
    title: "Intent Recognition",
    description:
      "Backend/Frontend parses the request and determines required data",
  },
  {
    title: "Validation",
    description:
      "Check if additional parameters are needed (address format, date ranges, etc.)",
  },
  {
    title: "Data Collection",
    description:
      "Query SubQuery indexer for on-chain data, fetch from cache (Redis) if available, retrieve from Rust scoring engine",
  },
  {
    title: "Score Calculation",
    description:
      "Rust modules process data through scoring algorithms with time decay and weighted metrics",
  },
  {
    title: "Response Generation",
    description:
      "Format results with breakdown, badges, levels, and component scores",
  },
  {
    title: "Delivery",
    description:
      "Return JSON/GraphQL response or render UI components for the user",
  },
];
