/**
 * PolkadotJS API Infrastructure Module
 * 
 * Professional implementation of Polkadot/Substrate chain interaction layer
 * with comprehensive runtime access, key management, and multi-network support.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { cryptoWaitReady } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';

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
  ROCOCO: {
    name: 'Rococo',
    endpoint: 'wss://rococo-rpc.polkadot.io',
    ss58Format: 42,
  },
  LOCAL: {
    name: 'Local',
    endpoint: 'ws://127.0.0.1:9944',
    ss58Format: 42,
  },
};

/**
 * Main Infrastructure Class
 */
export class PolkadotInfrastructure {
  constructor(networkConfig = NETWORKS.LOCAL) {
    this.networkConfig = networkConfig;
    this.api = null;
    this.provider = null;
    this.keyring = null;
    this.isConnected = false;
    this.eventListeners = new Map();
  }

  /**
   * 🔗 Direct Chain Connection
   */
  async connect() {
    try {
      await cryptoWaitReady();
      
      this.provider = new WsProvider(this.networkConfig.endpoint);
      this.api = await ApiPromise.create({ 
        provider: this.provider,
        throwOnConnect: false,
      });

      await this.api.isReady;
      this.isConnected = true;

      this.keyring = new Keyring({ 
        type: 'sr25519',
        ss58Format: this.networkConfig.ss58Format,
      });

      console.log(`✅ Connected to ${this.networkConfig.name}`);
      console.log(`📡 Runtime version: ${this.api.runtimeVersion.specName.toString()} v${this.api.runtimeVersion.specVersion.toString()}`);
      
      return this.api;
    } catch (error) {
      console.error('❌ Connection failed:', error);
      throw error;
    }
  }

  /**
   * Update metadata dynamically on runtime upgrades
   */
  async updateMetadata() {
    if (!this.api) throw new Error('API not connected');
    
    const metadata = await this.api.rpc.state.getMetadata();
    console.log('🔄 Metadata updated');
    return metadata;
  }

  /**
   * Reconnect with automatic retry
   */
  async reconnect(maxRetries = 3) {
    for (let i = 0; i < maxRetries; i++) {
      try {
        await this.disconnect();
        await this.connect();
        return true;
      } catch (error) {
        console.error(`Retry ${i + 1}/${maxRetries} failed:`, error);
        if (i === maxRetries - 1) throw error;
        await new Promise(resolve => setTimeout(resolve, 2000 * (i + 1)));
      }
    }
    return false;
  }

  /**
   * 🧠 Runtime Module Access - Query
   */
  async queryStorage(module, method, ...params) {
    if (!this.api) throw new Error('API not connected');
    
    try {
      const result = await this.api.query[module][method](...params);
      return result.toJSON();
    } catch (error) {
      console.error(`Query failed [${module}.${method}]:`, error);
      throw error;
    }
  }

  /**
   * Query account balance
   */
  async getBalance(address) {
    const { data: { free, reserved, frozen } } = await this.api.query.system.account(address);
    return {
      free: free.toString(),
      reserved: reserved.toString(),
      frozen: frozen.toString(),
      total: free.add(reserved).toString(),
    };
  }

  /**
   * Query governance proposals
   */
  async getGovernanceProposals() {
    const proposals = await this.api.query.democracy.publicProps();
    return proposals.toJSON();
  }

  /**
   * Query staking information
   */
  async getStakingInfo(address) {
    const [ledger, nominations] = await Promise.all([
      this.api.query.staking.ledger(address),
      this.api.query.staking.nominators(address),
    ]);
    
    return {
      ledger: ledger.toJSON(),
      nominations: nominations.toJSON(),
    };
  }

  /**
   * Query identity information
   */
  async getIdentity(address) {
    const identity = await this.api.query.identity.identityOf(address);
    return identity.toJSON();
  }

  /**
   * 🧠 Runtime Module Access - Transactions
   */
  async submitExtrinsic(signer, module, method, ...params) {
    if (!this.api) throw new Error('API not connected');
    
    return new Promise(async (resolve, reject) => {
      try {
        const tx = this.api.tx[module][method](...params);
        
        const unsub = await tx.signAndSend(signer, ({ status, events, dispatchError }) => {
          if (status.isInBlock) {
            console.log(`📦 Included in block: ${status.asInBlock.toHex()}`);
          }
          
          if (status.isFinalized) {
            console.log(`✅ Finalized in block: ${status.asFinalized.toHex()}`);
            
            if (dispatchError) {
              if (dispatchError.isModule) {
                const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs}`));
              } else {
                reject(new Error(dispatchError.toString()));
              }
            } else {
              resolve({
                blockHash: status.asFinalized.toHex(),
                events: events.map(e => e.toHuman()),
              });
            }
            
            unsub();
          }
        });
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Transfer balance
   */
  async transfer(signer, destination, amount) {
    return await this.submitExtrinsic(signer, 'balances', 'transfer', destination, amount);
  }

  /**
   * Submit governance vote
   */
  async voteOnProposal(signer, proposalIndex, aye, conviction = 1) {
    return await this.submitExtrinsic(
      signer,
      'democracy',
      'vote',
      proposalIndex,
      { Standard: { vote: { aye, conviction }, balance: 1000000000000 } }
    );
  }

  /**
   * Bond tokens for staking
   */
  async bondTokens(signer, controller, value, payee) {
    return await this.submitExtrinsic(signer, 'staking', 'bond', controller, value, payee);
  }

  /**
   * 🧠 Runtime Module Access - RPC Calls
   */
  async rpcCall(section, method, ...params) {
    if (!this.api) throw new Error('API not connected');
    
    try {
      const result = await this.api.rpc[section][method](...params);
      return result.toJSON();
    } catch (error) {
      console.error(`RPC call failed [${section}.${method}]:`, error);
      throw error;
    }
  }

  /**
   * Get block by hash or number
   */
  async getBlock(hashOrNumber) {
    const hash = typeof hashOrNumber === 'number'
      ? await this.api.rpc.chain.getBlockHash(hashOrNumber)
      : hashOrNumber;
    
    const block = await this.api.rpc.chain.getBlock(hash);
    return block.toJSON();
  }

  /**
   * Get latest block header
   */
  async getLatestBlock() {
    const header = await this.api.rpc.chain.getHeader();
    return header.toJSON();
  }

  /**
   * Get runtime version
   */
  async getRuntimeVersion() {
    const version = await this.api.rpc.state.getRuntimeVersion();
    return version.toJSON();
  }

  /**
   * 🧠 Runtime Module Access - Constants
   */
  getConstant(module, constantName) {
    if (!this.api) throw new Error('API not connected');
    
    const constant = this.api.consts[module][constantName];
    return constant.toJSON();
  }

  /**
   * Get minimum staking amount
   */
  getMinimumStake() {
    return this.getConstant('staking', 'minNominatorBond');
  }

  /**
   * Get existential deposit
   */
  getExistentialDeposit() {
    return this.getConstant('balances', 'existentialDeposit');
  }

  /**
   * Get maximum validators
   */
  getMaxValidators() {
    return this.getConstant('staking', 'maxValidatorCount');
  }

  /**
   * 🔐 Key & Signature Management
   */
  createAccount(mnemonic = null, keyType = 'sr25519') {
    if (!this.keyring) throw new Error('Keyring not initialized');
    
    const pair = mnemonic
      ? this.keyring.addFromMnemonic(mnemonic, {}, keyType)
      : this.keyring.addFromUri(`//${Math.random().toString(36).substring(7)}`, {}, keyType);
    
    return {
      address: pair.address,
      publicKey: u8aToHex(pair.publicKey),
      pair,
    };
  }

  /**
   * Import account from seed
   */
  importAccount(seed, keyType = 'sr25519') {
    if (!this.keyring) throw new Error('Keyring not initialized');
    
    const pair = this.keyring.addFromUri(seed, {}, keyType);
    return {
      address: pair.address,
      publicKey: u8aToHex(pair.publicKey),
      pair,
    };
  }

  /**
   * Sign message
   */
  signMessage(account, message) {
    const signature = account.sign(message);
    return {
      signature: u8aToHex(signature),
      message,
      address: account.address,
    };
  }

  /**
   * Verify signature
   */
  verifySignature(message, signature, address) {
    const pair = this.keyring.getPair(address);
    return pair.verify(message, signature);
  }

  /**
   * Switch signature scheme
   */
  switchKeyType(keyType) {
    this.keyring = new Keyring({ 
      type: keyType,
      ss58Format: this.networkConfig.ss58Format,
    });
    console.log(`🔐 Switched to ${keyType} signature scheme`);
  }

  /**
   * 📊 Data Monitoring & Event Tracking
   */
  async subscribeToEvents(callback) {
    if (!this.api) throw new Error('API not connected');
    
    const unsub = await this.api.query.system.events((events) => {
      const processedEvents = events.map(record => {
        const { event, phase } = record;
        return {
          section: event.section,
          method: event.method,
          data: event.data.toJSON(),
          phase: phase.toJSON(),
        };
      });
      
      callback(processedEvents);
    });
    
    this.eventListeners.set('system.events', unsub);
    return unsub;
  }

  /**
   * Subscribe to new block headers
   */
  async subscribeToNewHeads(callback) {
    if (!this.api) throw new Error('API not connected');
    
    const unsub = await this.api.rpc.chain.subscribeNewHeads((header) => {
      callback({
        number: header.number.toNumber(),
        hash: header.hash.toHex(),
        parentHash: header.parentHash.toHex(),
        stateRoot: header.stateRoot.toHex(),
        extrinsicsRoot: header.extrinsicsRoot.toHex(),
      });
    });
    
    this.eventListeners.set('chain.newHeads', unsub);
    return unsub;
  }

  /**
   * Subscribe to finalized blocks
   */
  async subscribeToFinalizedHeads(callback) {
    if (!this.api) throw new Error('API not connected');
    
    const unsub = await this.api.rpc.chain.subscribeFinalizedHeads((header) => {
      callback({
        number: header.number.toNumber(),
        hash: header.hash.toHex(),
      });
    });
    
    this.eventListeners.set('chain.finalizedHeads', unsub);
    return unsub;
  }

  /**
   * Monitor governance events
   */
  async monitorGovernance(callback) {
    return await this.subscribeToEvents((events) => {
      const governanceEvents = events.filter(e => 
        e.section === 'democracy' || 
        e.section === 'council' || 
        e.section === 'treasury'
      );
      
      if (governanceEvents.length > 0) {
        callback(governanceEvents);
      }
    });
  }

  /**
   * Monitor staking events
   */
  async monitorStaking(callback) {
    return await this.subscribeToEvents((events) => {
      const stakingEvents = events.filter(e => e.section === 'staking');
      
      if (stakingEvents.length > 0) {
        callback(stakingEvents);
      }
    });
  }

  /**
   * Monitor identity events
   */
  async monitorIdentity(callback) {
    return await this.subscribeToEvents((events) => {
      const identityEvents = events.filter(e => e.section === 'identity');
      
      if (identityEvents.length > 0) {
        callback(identityEvents);
      }
    });
  }

  /**
   * Unsubscribe from specific listener
   */
  unsubscribe(listenerKey) {
    const unsub = this.eventListeners.get(listenerKey);
    if (unsub) {
      unsub();
      this.eventListeners.delete(listenerKey);
      console.log(`🔕 Unsubscribed from ${listenerKey}`);
    }
  }

  /**
   * Unsubscribe from all listeners
   */
  unsubscribeAll() {
    this.eventListeners.forEach((unsub, key) => {
      unsub();
      console.log(`🔕 Unsubscribed from ${key}`);
    });
    this.eventListeners.clear();
  }

  /**
   * 🌐 Multi-Network Compatibility
   */
  async switchNetwork(networkConfig) {
    console.log(`🔄 Switching to ${networkConfig.name}...`);
    
    await this.disconnect();
    this.networkConfig = networkConfig;
    await this.connect();
    
    console.log(`✅ Switched to ${networkConfig.name}`);
  }

  /**
   * Connect to custom parachain
   */
  async connectToParachain(endpoint, ss58Format = 42) {
    const customConfig = {
      name: 'Custom Parachain',
      endpoint,
      ss58Format,
    };
    
    await this.switchNetwork(customConfig);
  }

  /**
   * Get network info
   */
  async getNetworkInfo() {
    if (!this.api) throw new Error('API not connected');
    
    const [chain, nodeName, nodeVersion, properties] = await Promise.all([
      this.api.rpc.system.chain(),
      this.api.rpc.system.name(),
      this.api.rpc.system.version(),
      this.api.rpc.system.properties(),
    ]);
    
    return {
      chain: chain.toString(),
      nodeName: nodeName.toString(),
      nodeVersion: nodeVersion.toString(),
      properties: properties.toJSON(),
      runtimeVersion: this.api.runtimeVersion.toJSON(),
    };
  }

  /**
   * 🧪 Testing & Simulation Support
   */
  async simulateExtrinsic(signer, module, method, ...params) {
    if (!this.api) throw new Error('API not connected');
    
    const tx = this.api.tx[module][method](...params);
    const paymentInfo = await tx.paymentInfo(signer);
    
    return {
      weight: paymentInfo.weight.toJSON(),
      class: paymentInfo.class.toString(),
      partialFee: paymentInfo.partialFee.toString(),
    };
  }

  /**
   * Dry run extrinsic
   */
  async dryRun(signer, module, method, ...params) {
    if (!this.api) throw new Error('API not connected');
    
    const tx = this.api.tx[module][method](...params);
    const signedTx = await tx.signAsync(signer);
    
    const result = await this.api.rpc.system.dryRun(signedTx.toHex());
    return result.toJSON();
  }

  /**
   * Mock chain behavior for testing
   */
  createMockApi(customTypes = {}) {
    return {
      query: new Proxy({}, {
        get: (target, module) => new Proxy({}, {
          get: (t, method) => async (...params) => {
            console.log(`[MOCK] query.${module}.${method}`, params);
            return null;
          }
        })
      }),
      tx: new Proxy({}, {
        get: (target, module) => new Proxy({}, {
          get: (t, method) => (...params) => {
            console.log(`[MOCK] tx.${module}.${method}`, params);
            return {
              signAndSend: async (signer, callback) => {
                console.log('[MOCK] Transaction signed and sent');
                callback({ 
                  status: { isFinalized: true, asFinalized: { toHex: () => '0xmock' } },
                  events: [],
                });
              }
            };
          }
        })
      }),
      rpc: new Proxy({}, {
        get: (target, section) => new Proxy({}, {
          get: (t, method) => async (...params) => {
            console.log(`[MOCK] rpc.${section}.${method}`, params);
            return { toJSON: () => null };
          }
        })
      }),
    };
  }

  /**
   * Disconnect and cleanup
   */
  async disconnect() {
    if (this.api) {
      this.unsubscribeAll();
      await this.api.disconnect();
      this.isConnected = false;
      console.log('🔌 Disconnected from chain');
    }
  }

  /**
   * Health check
   */
  async healthCheck() {
    if (!this.api || !this.isConnected) {
      return { healthy: false, error: 'Not connected' };
    }
    
    try {
      const health = await this.api.rpc.system.health();
      return {
        healthy: true,
        peers: health.peers.toNumber(),
        isSyncing: health.isSyncing.valueOf(),
        shouldHavePeers: health.shouldHavePeers.valueOf(),
      };
    } catch (error) {
      return { healthy: false, error: error.message };
    }
  }
}

/**
 * Factory function for quick initialization
 */
export async function createPolkadotApi(network = 'LOCAL') {
  const infrastructure = new PolkadotInfrastructure(NETWORKS[network]);
  await infrastructure.connect();
  return infrastructure;
}

/**
 * Batch operations helper
 */
export class BatchOperations {
  constructor(infrastructure) {
    this.infrastructure = infrastructure;
  }

  async batchQuery(queries) {
    const results = await Promise.all(
      queries.map(({ module, method, params }) =>
        this.infrastructure.queryStorage(module, method, ...params)
      )
    );
    return results;
  }

  async batchTransactions(signer, transactions) {
    const txs = transactions.map(({ module, method, params }) =>
      this.infrastructure.api.tx[module][method](...params)
    );
    
    const batch = this.infrastructure.api.tx.utility.batch(txs);
    
    return new Promise((resolve, reject) => {
      batch.signAndSend(signer, ({ status, events, dispatchError }) => {
        if (status.isFinalized) {
          if (dispatchError) {
            reject(dispatchError);
          } else {
            resolve({ blockHash: status.asFinalized.toHex(), events });
          }
        }
      });
    });
  }
}

export default PolkadotInfrastructure;
