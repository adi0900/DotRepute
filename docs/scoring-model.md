# DotRepute Scoring Model

## Table of Contents
1. [Overview](#overview)
2. [Scoring Formula](#scoring-formula)
3. [Component Scores](#component-scores)
4. [Score Normalization](#score-normalization)
5. [Time Decay](#time-decay)
6. [Edge Cases](#edge-cases)
7. [Examples](#examples)

---

## Overview

The **Contributor Reputation Score (CRS)** is a composite metric ranging from **0 to 100** that reflects a user's trustworthiness, activity, and contributions within the Polkadot ecosystem.

**Implementation**: The scoring engine is written in **Rust** as a standalone crate, compilable to WASM for maximum performance and portability.

### Design Principles

- **Rust-First**: Core logic in Rust for performance, safety, and WASM compatibility
- **Transparent**: All scoring logic is open and auditable
- **Balanced**: Multiple dimensions prevent gaming
- **Dynamic**: Scores update as behavior changes
- **Fair**: Accessible to all participant types (validators, nominators, developers, voters)
- **Weighted**: Different activities have different importance

---

## Scoring Formula

### Base Formula

```
CRS = (Identity × 0.25) + (Governance × 0.25) + (Staking × 0.20) +
      (Activity × 0.20) + (DevContributions × 0.10)
```

### Weight Distribution

| Component | Weight | Rationale |
|-----------|--------|-----------|
| Identity | 25% | Foundation of trust and accountability |
| Governance | 25% | Active participation in ecosystem decisions |
| Staking | 20% | Financial commitment and skin in the game |
| Activity | 20% | General ecosystem engagement |
| Dev Contributions | 10% | Optional: Technical contributions |

### Configurable Weights

Weights can be adjusted through governance:

```rust
// Rust implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub identity: f64,           // default: 0.25
    pub governance: f64,         // default: 0.25
    pub staking: f64,            // default: 0.20
    pub activity: f64,           // default: 0.20
    pub dev_contributions: f64,  // default: 0.10
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            identity: 0.25,
            governance: 0.25,
            staking: 0.20,
            activity: 0.20,
            dev_contributions: 0.10,
        }
    }
}
```

---

## Component Scores

Each component is scored from **0 to 100**.

### 1. Identity Score (0-100)

**Purpose**: Measure verifiability and trustworthiness of on-chain identity

**Data Sources**:
- Identity pallet (display name, legal name, email, Twitter, etc.)
- Judgements from registrars
- Account age

**Formula**:
```
IdentityScore = BaseIdentityScore + JudgementBonus + AgeFactor

Where:
- BaseIdentityScore = (fields_filled / total_fields) × 40
- JudgementBonus = judgement_score (0-50)
- AgeFactor = min(account_age_days / 365, 1) × 10
```

**Breakdown**:

| Criteria | Points | Description |
|----------|--------|-------------|
| Basic info set | 10 | Display name, legal name |
| Contact info | 10 | Email, website |
| Social accounts | 10 | Twitter, Matrix, Discord |
| Additional fields | 10 | Image, PGP fingerprint |
| Registrar judgements | 0-50 | See judgement scoring below |
| Account age | 0-10 | 1 point per 36.5 days, max 10 |

**Judgement Scoring (Rust)**:
```rust
pub fn get_judgement_score(judgement_type: &JudgementType) -> u32 {
    match judgement_type {
        JudgementType::Reasonable => 30,
        JudgementType::KnownGood => 50,
        JudgementType::FeePaid => 5,
        JudgementType::LowQuality => 10,
        JudgementType::OutOfDate => 5,
        JudgementType::Erroneous => 0,
    }
}

// Multiple judgements: take the highest
pub fn calculate_judgement_bonus(judgements: &[Judgement]) -> u32 {
    judgements
        .iter()
        .map(|j| get_judgement_score(&j.judgement_type))
        .max()
        .unwrap_or(0)
}
```

**Example**:
```
User has:
- Display name, email, Twitter ✓ (30 points)
- One "KnownGood" judgement (50 points)
- Account age: 730 days (10 points)
Total: 90/100
```

---

### 2. Governance Score (0-100)

**Purpose**: Measure participation in Polkadot governance (OpenGov)

**Data Sources**:
- Referendum votes
- Proposal submissions
- Conviction voting
- Delegation

**Formula**:
```
GovernanceScore = VoteParticipation + VoteQuality + ProposalActivity

Where:
- VoteParticipation = (votes_cast / recent_referenda) × 50
- VoteQuality = conviction_weighted_score × 30
- ProposalActivity = min(proposals_submitted, 5) × 4
```

**Vote Participation (Rust)** (0-50 points):
```rust
// Count votes in last N referenda (e.g., N = 20)
pub fn calculate_participation_score(address: &str, recent_referenda: u32) -> f64 {
    let votes_cast = count_votes_in_recent(address, recent_referenda);
    f64::min((votes_cast as f64 / recent_referenda as f64) * 50.0, 50.0)
}
```

**Vote Quality (Rust)** (0-30 points):
Based on conviction levels:
```rust
pub fn get_conviction_weight(conviction: &Conviction) -> f64 {
    match conviction {
        Conviction::None => 0.1,
        Conviction::Locked1x => 1.0,
        Conviction::Locked2x => 2.0,
        Conviction::Locked3x => 3.0,
        Conviction::Locked4x => 4.0,
        Conviction::Locked5x => 5.0,
        Conviction::Locked6x => 6.0,
    }
}

pub fn calculate_vote_quality_score(votes: &[Vote]) -> f64 {
    if votes.is_empty() {
        return 0.0;
    }

    let total_conviction: f64 = votes
        .iter()
        .map(|vote| get_conviction_weight(&vote.conviction))
        .sum();

    let avg_conviction = total_conviction / votes.len() as f64;
    (avg_conviction / 6.0) * 30.0
}
```

**Proposal Activity (Rust)** (0-20 points):
```rust
pub fn calculate_proposal_score(address: &str) -> f64 {
    let proposals_submitted = count_proposals(address);
    f64::min(proposals_submitted as f64 * 4.0, 20.0)
}
```

**Example**:
```
User has:
- Voted in 15/20 recent referenda (37.5 points)
- Average conviction: 3x (15 points)
- Submitted 2 proposals (8 points)
Total: 60.5/100
```

---

### 3. Staking Score (0-100)

**Purpose**: Measure financial commitment and validator/nominator participation

**Data Sources**:
- Total bonded amount
- Validator status
- Nominator activity
- Staking rewards earned

**Formula**:
```
StakingScore = StakeAmount + RoleBonus + ConsistencyFactor

Where:
- StakeAmount = normalized_stake × 60
- RoleBonus = validator_bonus OR nominator_bonus (0-25)
- ConsistencyFactor = staking_duration × 15
```

**Stake Amount** (0-60 points):
```typescript
// Normalize against median stake
const medianStake = getMedianStake(); // e.g., 100 DOT
const userStake = getTotalBonded(address);

const normalizedStake = Math.min(userStake / (medianStake * 10), 1);
const stakeScore = normalizedStake * 60;
```

**Role Bonus** (0-25 points):
```typescript
if (isValidator(address)) {
  // Validator bonus based on performance
  const commission = getCommission(address);
  const uptime = getValidatorUptime(address);

  roleBonus = (1 - commission / 100) * 15 + (uptime / 100) * 10;
} else if (isNominator(address)) {
  // Nominator bonus
  const activeNominations = getActiveNominations(address);
  roleBonus = Math.min(activeNominations.length * 5, 25);
}
```

**Consistency Factor** (0-15 points):
```typescript
// Reward continuous staking
const stakingDays = getDaysSinceFirstStake(address);
const consistencyScore = Math.min(stakingDays / 365, 1) * 15;
```

**Example**:
```
User has:
- 500 DOT staked (median = 100 DOT) → normalized to 0.5 (30 points)
- Active nominator with 5 validators (25 points)
- Staking for 200 days (8.2 points)
Total: 63.2/100
```

---

### 4. Activity Score (0-100)

**Purpose**: Measure general on-chain activity and engagement

**Data Sources**:
- Extrinsic count
- Transfer activity
- Cross-chain activity (XCM)
- Treasury interactions

**Formula**:
```
ActivityScore = ExtrinsicActivity + DiversityBonus + RecencyFactor

Where:
- ExtrinsicActivity = min(extrinsic_count / 100, 1) × 50
- DiversityBonus = unique_pallet_interactions × 3 (max 30)
- RecencyFactor = recent_activity_score × 20
```

**Extrinsic Activity** (0-50 points):
```typescript
const extrinsicCount = getExtrinsicCount(address);
const activityScore = Math.min(extrinsicCount / 100, 1) * 50;
```

**Diversity Bonus** (0-30 points):
Reward interaction with different pallets:
```typescript
const PALLET_CATEGORIES = [
  'balances', 'staking', 'governance', 'identity',
  'treasury', 'xcm', 'multisig', 'proxy', 'utility', 'crowdloan'
];

const uniquePallets = countUniquePalletInteractions(address);
const diversityScore = Math.min(uniquePallets * 3, 30);
```

**Recency Factor** (0-20 points):
```typescript
// Activity in last 30 days
const recentExtrinsics = getExtrinsicsInLast30Days(address);
const recencyScore = Math.min(recentExtrinsics / 10, 1) * 20;
```

**Example**:
```
User has:
- 250 total extrinsics (50 points)
- Interacted with 8 different pallets (24 points)
- 5 extrinsics in last 30 days (10 points)
Total: 84/100
```

---

### 5. Developer Contributions Score (0-100)

**Purpose**: Measure technical contributions to the ecosystem (optional)

**Data Sources**:
- GitHub commits to Polkadot/Substrate repos
- Pull requests merged
- Issues created/resolved
- Code review participation

**Formula**:
```
DevScore = CommitActivity + PRActivity + ReviewActivity

Where:
- CommitActivity = min(commits / 50, 1) × 40
- PRActivity = min(merged_PRs / 10, 1) × 40
- ReviewActivity = min(reviews / 20, 1) × 20
```

**Note**: This component is **optional** and requires explicit GitHub linking.

**Example**:
```
User has:
- 75 commits to ecosystem repos (40 points)
- 8 merged PRs (32 points)
- 15 code reviews (15 points)
Total: 87/100
```

---

## Score Normalization

### Percentile-Based Normalization

To ensure fairness, some scores are normalized against the network distribution:

```typescript
function normalizeByPercentile(value: number, allValues: number[]): number {
  const sorted = allValues.sort((a, b) => a - b);
  const percentile = sorted.findIndex(v => v >= value) / sorted.length;
  return percentile * 100;
}
```

### Logarithmic Scaling

For metrics with high variance (e.g., stake amount):

```typescript
function logNormalize(value: number, base: number = 10): number {
  return Math.log(value + 1) / Math.log(base) * 100;
}
```

---

## Time Decay

### Decay Function

Recent activity is weighted more heavily:

```typescript
function applyTimeDecay(score: number, daysAgo: number, halfLife: number = 180): number {
  const decayFactor = Math.pow(0.5, daysAgo / halfLife);
  return score * decayFactor;
}
```

### Example

```
Activity from 90 days ago:
- Original score: 50
- Decay factor: 0.5^(90/180) = 0.707
- Decayed score: 50 × 0.707 = 35.35
```

### Time Windows

Different components use different time windows:

| Component | Time Window | Decay Half-Life |
|-----------|-------------|-----------------|
| Governance | Last 6 months | 180 days |
| Activity | Last 3 months | 90 days |
| Staking | All-time | No decay |
| Identity | All-time | No decay |
| Dev Contributions | Last 1 year | 365 days |

---

## Edge Cases

### New Accounts

Accounts less than 30 days old receive a "new account" penalty:

```rust
pub fn apply_new_account_penalty(score: f64, account_age_days: u32) -> f64 {
    if account_age_days < 30 {
        score * 0.5  // 50% penalty
    } else {
        score
    }
}
```

### Inactive Accounts

Accounts with no activity in 180 days receive an inactivity penalty:

```rust
pub fn apply_inactivity_penalty(score: f64, days_since_last_activity: u32) -> f64 {
    if days_since_last_activity > 180 {
        let penalty_factor = f64::max(
            0.3,
            1.0 - ((days_since_last_activity - 180) as f64 / 365.0)
        );
        score * penalty_factor
    } else {
        score
    }
}
```

### Validators with Slashes

Slashing events negatively impact reputation:

```rust
pub fn calculate_slash_penalty(slash_events: &[SlashEvent], total_stake: u128) -> f64 {
    let penalty: f64 = slash_events
        .iter()
        .map(|slash| {
            let severity = slash.amount as f64 / total_stake as f64;
            severity * 20.0  // Max -20 points per slash
        })
        .sum();

    penalty
}

pub fn apply_slash_penalty(score: f64, penalty: f64) -> f64 {
    f64::max(0.0, score - penalty)
}
```

### Malicious Behavior

Accounts flagged for spam or malicious proposals:

```rust
pub fn apply_spam_penalty(governance_score: f64, has_spam: bool) -> f64 {
    if has_spam {
        governance_score * 0.5
    } else {
        governance_score
    }
}
```

---

## Examples

### Example 1: Active Validator

```
Identity: 80 (verified, good judgement)
Governance: 65 (voted in 13/20 referenda, avg conviction 2x)
Staking: 90 (validator, high stake, 100% uptime)
Activity: 70 (moderate on-chain activity)
Dev Contributions: 0 (not linked)

CRS = (80 × 0.25) + (65 × 0.25) + (90 × 0.20) + (70 × 0.20) + (0 × 0.10)
    = 20 + 16.25 + 18 + 14 + 0
    = 68.25
```

### Example 2: Governance Enthusiast

```
Identity: 70 (partial identity, one judgement)
Governance: 95 (voted in all referenda, high conviction, 3 proposals)
Staking: 40 (small nominator)
Activity: 60 (regular activity)
Dev Contributions: 0 (not linked)

CRS = (70 × 0.25) + (95 × 0.25) + (40 × 0.20) + (60 × 0.20) + (0 × 0.10)
    = 17.5 + 23.75 + 8 + 12 + 0
    = 61.25
```

### Example 3: Core Developer

```
Identity: 95 (fully verified, KnownGood)
Governance: 55 (occasional voter)
Staking: 30 (minimal stake)
Activity: 50 (low on-chain activity)
Dev Contributions: 90 (many commits and PRs)

CRS = (95 × 0.25) + (55 × 0.25) + (30 × 0.20) + (50 × 0.20) + (90 × 0.10)
    = 23.75 + 13.75 + 6 + 10 + 9
    = 62.5
```

### Example 4: New User

```
Identity: 20 (basic info, no judgement)
Governance: 10 (1-2 votes)
Staking: 15 (small stake)
Activity: 25 (few transactions)
Dev Contributions: 0
Account age: 20 days → 50% penalty

CRS = [(20 × 0.25) + (10 × 0.25) + (15 × 0.20) + (25 × 0.20) + 0] × 0.5
    = [5 + 2.5 + 3 + 5 + 0] × 0.5
    = 15.5 × 0.5
    = 7.75
```

---

## Score Interpretation

| Score Range | Rating | Description |
|-------------|--------|-------------|
| 90-100 | Exceptional | Highly trusted, core contributor |
| 75-89 | Excellent | Very active and trusted member |
| 60-74 | Good | Solid contributor with verified identity |
| 45-59 | Moderate | Active participant, some verification |
| 30-44 | Fair | Basic participation |
| 15-29 | Low | Minimal activity or new account |
| 0-14 | Very Low | No activity or unverified |

---

## Future Enhancements

### Planned Improvements

1. **Machine Learning**: Anomaly detection for gaming behavior
2. **Social Graph**: Reputation influenced by connections
3. **Cross-Chain**: Aggregate reputation across parachains
4. **Badges/Achievements**: Unlock special recognition at milestones
5. **Dynamic Weights**: AI-adjusted weights based on ecosystem needs

### Community Governance

The scoring model parameters will be governed by the community:

- Weight adjustments via referenda
- New component proposals
- Threshold modifications
- Penalty/bonus changes

---

## API Response Format

```json
{
  "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "overallScore": 68.25,
  "breakdown": {
    "identity": {
      "score": 80,
      "weight": 0.25,
      "contribution": 20
    },
    "governance": {
      "score": 65,
      "weight": 0.25,
      "contribution": 16.25
    },
    "staking": {
      "score": 90,
      "weight": 0.20,
      "contribution": 18
    },
    "activity": {
      "score": 70,
      "weight": 0.20,
      "contribution": 14
    },
    "devContributions": {
      "score": 0,
      "weight": 0.10,
      "contribution": 0
    }
  },
  "metadata": {
    "accountAge": 730,
    "lastUpdated": "2025-11-07T12:00:00Z",
    "rank": 1523,
    "percentile": 87.5
  }
}
```

---

## References

- [Polkadot Identity Pallet](https://wiki.polkadot.network/docs/learn-identity)
- [OpenGov Documentation](https://wiki.polkadot.network/docs/learn-opengov)
- [Staking Documentation](https://wiki.polkadot.network/docs/learn-staking)
