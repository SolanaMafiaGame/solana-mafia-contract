# 🎮 Solana Mafia Smart Contract

[![Security Audit](https://img.shields.io/badge/Security-Audited-green.svg)](SECURITY_AUDIT_REPORT_EN.md)
[![Solana](https://img.shields.io/badge/Solana-Anchor-blue.svg)](https://github.com/coral-xyz/anchor)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Decentralized Business Empire Game on Solana Blockchain**

A smart contract for the Solana Mafia game - a decentralized business simulation where players own NFT businesses, earn passive SOL income, and build their criminal empire on the Solana blockchain.

## 🔍 Security Audit

This smart contract has undergone an independent security audit. View the full reports:

- 🇺🇸 **[English Audit Report](SECURITY_AUDIT_REPORT_EN.md)** - Comprehensive security analysis
- 🇷🇺 **[Russian Audit Report](SECURITY_AUDIT_REPORT_RU.md)** - Полный отчет аудита безопасности

**Security Score: 88/100** ⭐⭐⭐⭐⭐

### Key Security Findings:
✅ **Low Rug Pull Risk** - Minimal admin rights, funds protected by PDA  
✅ **Excellent Code Quality** - Professional Rust/Anchor implementation  
✅ **Transparent Economics** - Honest ponzi with open rules  
✅ **Open Source** - Full code transparency and verification  

## 🏗️ Architecture

### Core Features
- **NFT-Based Business Ownership** - Each business is a unique NFT
- **Ultra-Optimized Data Structures** - 21.1% smaller binary through advanced optimizations
- **Distributed Earnings System** - Unique schedules to distribute RPC load
- **Dynamic Ownership Verification** - Real-time NFT ownership checks
- **Cost-Optimized Deployment** - Reduced from 4.12 SOL to 3.25 SOL deployment cost

### Business Types
1. **Tobacco Shop** (0.1 SOL) - 2.0% daily yield
2. **Funeral Service** (0.5 SOL) - 1.8% daily yield  
3. **Car Workshop** (2.0 SOL) - 1.6% daily yield
4. **Italian Restaurant** (5.0 SOL) - 1.4% daily yield
5. **Gentlemen Club** (10.0 SOL) - 1.2% daily yield
6. **Charity Fund** (50.0 SOL) - 1.0% daily yield

## 📊 Smart Contract Details

**Program ID (Mainnet)**: `9h2uDYXv48GAfSXzprXDgDKBCkxAv7yRY2pDbZeGnZXF`

### Key Components

#### Data Structures (`src/state/`)
- `player.rs` - Ultra-optimized PlayerCompact with bit packing
- `business.rs` - Business logic and NFT integration
- `game_state.rs` - Global game statistics
- `treasury.rs` - Protected treasury PDA

#### Instructions (`src/instructions/`)
- `player.rs` - Player creation and management
- `business.rs` - Business operations (create, upgrade, sell)
- `earnings.rs` - Earnings calculation and claiming
- `nft.rs` - NFT minting and burning

## 🛡️ Security Features

### PDA Protection
```rust
// All accounts protected by Program Derived Accounts
seeds = [PLAYER_SEED, owner.key().as_ref()]     // Player
seeds = [TREASURY_SEED]                         // Treasury
seeds = [GAME_CONFIG_SEED]                      // Configuration
```

### Overflow Protection
```rust
// All math operations use checked arithmetic
self.total_invested = self.total_invested
    .checked_add(amount)
    .ok_or(SolanaMafiaError::MathOverflow)?;
```

### Access Control
- Strict ownership verification for all operations
- PDA-based account validation
- Minimal admin rights (only entry fee modification)

## 💰 Economic Model

### Money Flow
```
Entry Fee: 100% → Team
Purchases: 80% → Treasury PDA, 20% → Team  
Upgrades: 80% → Treasury PDA, 20% → Team
Slot Unlocks: 100% → Team
```

### Early Exit Penalties
Progressive selling fees to prevent speculation:
- Days 0-6: 25% fee
- Days 7-13: 20% fee  
- Days 14-20: 15% fee
- Days 21-27: 10% fee
- Days 28-30: 5% fee
- Day 31+: 2% fee

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://book.anchor-lang.com/getting_started/installation.html)

### Building

```bash
# Clone repository
git clone https://github.com/SolanaMafiaGame/solana-mafia-contract.git
cd solana-mafia-contract

# Build (optimized for production)
RUSTFLAGS="-C target-cpu=generic" anchor build

# Test
anchor test --skip-local-validator
```

### Deployment

```bash
# Deploy to devnet
RUSTFLAGS="-C target-cpu=generic" anchor build
anchor deploy --provider.cluster devnet

# Initialize game state (only once)
anchor run initialize-game
```

## 📋 Testing

The contract includes comprehensive test suites:

```bash
# Run all tests
anchor test

# Run specific test files
anchor test --skip-local-validator tests/solana-mafia.js
```

## 🎯 Game Mechanics

### Business Ownership
- Each business is an NFT stored in player slots
- Real-time ownership verification before operations
- Businesses can be sold with progressive fees

### Earnings System
- Passive SOL earnings every 24 hours
- Distributed update schedule to prevent RPC overload
- Players can claim earnings anytime

### Slot System
- 9 total slots per player
- Unlock additional slots with SOL payments
- Different slot types with various benefits

## 🔧 Optimization Features

The contract implements several optimization techniques:

- **Bit Packing**: Efficient memory usage in data structures
- **u32 Types**: Reduced from u64 for appropriate fields
- **Fixed Arrays**: Instead of Vec for predictable sizes
- **Method Access**: Accessor methods for optimized field access

**Result**: 21.1% smaller binary size, saving 0.87 SOL per deployment

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

This is the official smart contract repository for Solana Mafia. For game-related inquiries or bug reports, please open an issue.

## ⚠️ Disclaimer

This smart contract implements a ponzi-style economic model. All participants should understand the risks involved:

- **Ponzi Nature**: Returns are paid from new player deposits
- **Sustainability**: Requires continuous player growth
- **Early Exit Costs**: High fees for early business sales
- **No Guarantees**: Participate only with funds you can afford to lose

The contract has been audited for security but economic sustainability depends on community participation.

---

**🎮 [Play Solana Mafia](https://solana-mafia.xyz)** | **📋 [View Full Audit](SECURITY_AUDIT_REPORT_EN.md)** | **💻 [Source Code](https://github.com/SolanaMafiaGame/solana-mafia-contract)**