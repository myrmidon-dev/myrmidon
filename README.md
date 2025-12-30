# Myrmidon - Decentralized Swarm Orchestration

**Solana interface for massive-scale AI agent coordination using compressed NFT (cNFT) Merkle Trees.**
Enables a master entity ("The Overseer") to orchestrate 10,000+ worker agents with cryptographic proof-of-work verification, zero private key exposure, and atomic SOL settlements.

**Architecture:** Built using `mpl-bubblegum` (Metaplex) and `spl-account-compression`.

## Key Features

* **Zero-Trust Worker Verification**: Agents submit Merkle proofs to validate their identity; no private key sharing required.
* **O(1) On-Chain State**: Manages thousands of agents using a single concurrent Merkle Tree, minimizing rent costs.
* **Atomic Settlement**: Smart contract verifies work hash and proof, then streams SOL rewards in the same transaction.
* **High-Frequency Command**: Off-chain priority queue system (The Overseer) for optimizing task distribution.
* **Rust/Anchor & TypeScript SDK**: Type-safe integration for both on-chain logic and off-chain agents.

## Architecture Flow

The Overseer (Master Node)
       ↓ (Dispatches Task)
   Swarm Grid (10k+ Agents)
       ↓ (Computes & Hashes)
Myrmidon Program (Rust/Anchor)
       ↓ (Verifies Merkle Proof)
SPL Account Compression
       ↓ (Updates State)
Solana Blockchain (Settlement)

## Installation

### Prerequisites
* Rust 1.70+
* Solana CLI 1.16+
* Node.js 18+
* Yarn / NPM

### Build Source
# Clone repository
git clone https://github.com/myrmidon-dev/myrmidon.git
cd myrmidon

# Build On-Chain Program
anchor build

# Install Client Dependencies
npm install

## Usage Examples

### 1. Initialize the Overseer
Set up the master control unit and connect to the Solana cluster.

import { SwarmOverseer, NetworkConfig } from '@myrmidon/sdk';
import { Connection, Keypair } from '@solana/web3.js';

const connection = new Connection('https://api.mainnet-beta.solana.com');
const wallet = Keypair.fromSecretKey(/* ... */);

// Initialize the Overseer
const overseer = new SwarmOverseer(connection, wallet);

// Sync local Merkle Tree from on-chain events
await overseer.syncState();
console.log(`Swarm Ready. Active Agents: ${overseer.getAgentCount()}`);

### 2. Dispatch Tasks to the Swarm
Distribute computational tasks to agents based on priority and reputation.

import { Task, Priority } from '@myrmidon/types';

// Define a computational task
const task: Task = {
  id: 'TASK-8821',
  payload: Buffer.from('data-to-process'),
  reward: 0.05, // SOL
  priority: Priority.HIGH
};

// Queue the task
await overseer.dispatch(task);

// Monitor completion via event listener
overseer.on('TaskCompleted', (result) => {
  console.log(`Worker ${result.workerId} finished task. Hash: ${result.hash}`);
});

## API Reference

* `initialize_swarm(depth, buffer)`: Deploys a new Merkle Tree for agent management.
* `submit_work_proof(root, hash, proof)`: Core instruction for work verification and payout.
* `Overseer.sync()`: Rebuilds the off-chain state from Solana RPC logs.

## Security Model

* **Non-Custodial**: The Master agent never accesses worker private keys.
* **Cryptographic Integrity**: All work is verified against the Merkle Root. Invalid proofs are rejected at the protocol level.
* **Spending Limits**: Rewards are capped per instruction to prevent vault drainage.

## Testing

# Run Integration Tests
anchor test

# Unit Tests for Merkle Logic
npm run test:sdk

## License

MIT License
