/**
 * PolkadotJS API Infrastructure Module
 *
 * Professional implementation of Polkadot/Substrate chain interaction layer
 * with comprehensive runtime access, key management, and multi-network support.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

/**
 * Network Configuration
 */
export const NETWORKS = {
  POLKADOT: {
    name: 'Polkadot',
    endpoint: 'wss://rpc.polkadot.io',
    ss58Format: 0,
  },
  KUSAMA: {
    name: 'Kusama',
    endpoint: 'wss://kusama-rpc.polkadot.io',
    ss58Format: 2,
  },
  WESTEND: {
    name: 'Westend',
    endpoint: 'wss://westend-rpc.polkadot.io',
    ss58Format: 42,
  },
};

/**
 * Main Infrastructure Class for Browser
 */
export class PolkadotInfrastructure {
  private networkConfig: typeof NETWORKS.POLKADOT;
  private api: ApiPromise | null = null;
  private provider: WsProvider | null = null;
  public isConnected: boolean = false;

  constructor(networkConfig = NETWORKS.POLKADOT) {
    this.networkConfig = networkConfig;
  }

  /**
   * 🔗 Direct Chain Connection
   */
  async connect() {
    try {
      this.provider = new WsProvider(this.networkConfig.endpoint);
      this.api = await ApiPromise.create({
        provider: this.provider,
        throwOnConnect: false,
      });

      await this.api.isReady;
      this.isConnected = true;

      console.log(`✅ Connected to ${this.networkConfig.name}`);
      console.log(`📡 Runtime version: ${this.api.runtimeVersion.specName.toString()} v${this.api.runtimeVersion.specVersion.toString()}`);

      return this.api;
    } catch (error) {
      console.error('❌ Connection failed:', error);
      throw error;
    }
  }

  /**
   * Query account balance
   */
  async getBalance(address: string) {
    if (!this.api) throw new Error('API not connected');

    const { data: { free, reserved, frozen } } = await this.api.query.system.account(address);
    return {
      free: free.toString(),
      reserved: reserved.toString(),
      frozen: frozen.toString(),
      total: free.add(reserved).toString(),
    };
  }

  /**
   * Query governance referenda participation
   */
  async getGovernanceParticipation(address: string) {
    if (!this.api) throw new Error('API not connected');

    try {
      // Get voting info from referenda pallet
      const voting = await this.api.query.convictionVoting?.votingFor?.entries(address);

      let totalVotes = 0;
      let activeVotes = 0;

      if (voting) {
        voting.forEach(([, votingInfo]: any) => {
          const info = votingInfo.toJSON();
          if (info && typeof info === 'object' && 'casting' in info) {
            const casting = (info as any).casting;
            if (casting?.votes) {
              totalVotes += casting.votes.length;
              activeVotes += casting.votes.length;
            }
          }
        });
      }

      return {
        totalVotes,
        activeVotes,
        participationScore: Math.min(100, Math.floor((totalVotes / 20) * 100)),
      };
    } catch (error) {
      console.error('Governance query error:', error);
      return {
        totalVotes: 0,
        activeVotes: 0,
        participationScore: 0,
      };
    }
  }

  /**
   * Query staking information
   */
  async getStakingInfo(address: string) {
    if (!this.api) throw new Error('API not connected');

    try {
      const [ledger, nominations] = await Promise.all([
        this.api.query.staking.ledger(address),
        this.api.query.staking.nominators(address),
      ]);

      const ledgerData = ledger.toJSON() as any;
      const nominationsData = nominations.toJSON() as any;

      const totalStaked = ledgerData?.active || '0';
      const stakingScore = Math.min(100, Math.floor((parseInt(totalStaked) / 1e12 / 1000) * 100));

      return {
        ledger: ledgerData,
        nominations: nominationsData,
        totalStaked,
        stakingScore,
      };
    } catch (error) {
      console.error('Staking query error:', error);
      return {
        ledger: null,
        nominations: null,
        totalStaked: '0',
        stakingScore: 0,
      };
    }
  }

  /**
   * Query identity information
   */
  async getIdentity(address: string) {
    if (!this.api) throw new Error('API not connected');

    try {
      const identity = await this.api.query.identity.identityOf(address);
      const identityData = identity.toJSON() as any;

      let identityScore = 0;
      if (identityData) {
        identityScore = 50; // Base score for having identity

        if (identityData.judgements && identityData.judgements.length > 0) {
          identityScore += 35; // Verified identity
        }

        if (identityData.info) {
          const info = identityData.info;
          if (info.display) identityScore += 5;
          if (info.email) identityScore += 5;
          if (info.twitter) identityScore += 5;
        }
      }

      return {
        identity: identityData,
        identityScore: Math.min(100, identityScore),
        isVerified: identityData?.judgements?.length > 0,
      };
    } catch (error) {
      console.error('Identity query error:', error);
      return {
        identity: null,
        identityScore: 0,
        isVerified: false,
      };
    }
  }

  /**
   * Calculate complete reputation score
   */
  async getReputationScore(address: string) {
    if (!this.api) throw new Error('API not connected');

    try {
      const [governance, staking, identity] = await Promise.all([
        this.getGovernanceParticipation(address),
        this.getStakingInfo(address),
        this.getIdentity(address),
      ]);

      // Activity score based on governance + staking
      const activityScore = Math.min(100, Math.floor(
        (governance.totalVotes * 2) + (parseInt(staking.totalStaked) > 0 ? 20 : 0)
      ));

      // Total score with weights from reputation.rs
      const totalScore = Math.floor(
        (identity.identityScore * 0.25) +
        (governance.participationScore * 0.25) +
        (staking.stakingScore * 0.20) +
        (activityScore * 0.20) +
        (10 * 0.10) // Dev score placeholder
      );

      return {
        totalScore,
        maxScore: 100,
        breakdown: {
          identity: { score: identity.identityScore, max: 100 },
          governance: { score: governance.participationScore, max: 100 },
          staking: { score: staking.stakingScore, max: 100 },
          activity: { score: activityScore, max: 100 },
        },
        details: {
          referendaVoted: governance.totalVotes,
          totalStaked: (parseInt(staking.totalStaked) / 1e10).toFixed(4),
          isIdentityVerified: identity.isVerified,
        },
      };
    } catch (error) {
      console.error('Reputation calculation error:', error);
      throw error;
    }
  }

  /**
   * Disconnect and cleanup
   */
  async disconnect() {
    if (this.api) {
      await this.api.disconnect();
      this.isConnected = false;
      console.log('🔌 Disconnected from chain');
    }
  }
}

/**
 * Factory function for quick initialization
 */
export async function createPolkadotApi(network: keyof typeof NETWORKS = 'POLKADOT') {
  const infrastructure = new PolkadotInfrastructure(NETWORKS[network]);
  await infrastructure.connect();
  return infrastructure;
}

export default PolkadotInfrastructure;
