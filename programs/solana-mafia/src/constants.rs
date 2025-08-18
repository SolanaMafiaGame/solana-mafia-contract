// constants.rs
use anchor_lang::prelude::*;

// ============================================================================
// BUSINESS CONFIGURATION
// ============================================================================

/// Количество типов бизнесов
pub const BUSINESS_TYPES_COUNT: usize = 6;

/// Минимальные депозиты в lamports (базовые цены)
pub const MIN_DEPOSITS: [u64; 6] = [
    100_000_000,    // 0.1 SOL - TobaccoShop
    500_000_000,    // 0.5 SOL - FuneralService  
    2_000_000_000,  // 2 SOL - CarWorkshop
    5_000_000_000,  // 5 SOL - ItalianRestaurant 
    10_000_000_000, // 10 SOL - GentlemenClub
    50_000_000_000, // 50 SOL - CharityFund
];

/// Дневные ставки в базисных пунктах (базовые доходности)
pub const BUSINESS_RATES: [u16; 6] = [200, 180, 160, 140, 120, 100]; // 2.0%-1.0%, более дорогие = меньший %

// ============================================================================
// UPGRADE SYSTEM - НОВАЯ МОДЕЛЬ
// ============================================================================

/// Максимальный уровень улучшения для каждого бизнеса
pub const MAX_UPGRADE_LEVEL: u8 = 3;

/// Множители стоимости улучшений (% от базовой стоимости бизнеса)
pub const UPGRADE_COST_MULTIPLIERS: [u8; 3] = [20, 50, 100]; // 20%, 50%, 100%

/// Бонусы доходности за каждый уровень улучшения (в базисных пунктах)
pub const UPGRADE_YIELD_BONUSES: [u16; 3] = [10, 25, 50]; // +0.1%, +0.25%, +0.5%

// ============================================================================
// SLOT SYSTEM - НОВАЯ СИСТЕМА СЛОТОВ
// ============================================================================

/// Количество бесплатных слотов
pub const FREE_BUSINESS_SLOTS: u8 = 3;

/// Максимальное количество всех слотов в новой системе
pub const MAX_REGULAR_SLOTS: u8 = 9; // 3 бесплатных + 3 платных basic + 3 premium

/// Для совместимости со старым кодом
pub const BASE_BUSINESS_SLOTS: u8 = FREE_BUSINESS_SLOTS;

/// Множитель стоимости дополнительных слотов (% от стоимости бизнеса в слоте)
pub const SLOT_UNLOCK_COST_MULTIPLIER: u8 = 10; // 10% от стоимости

/// Типы слотов
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum SlotType {
    Basic,           // Обычный слот (базовые 3 + разблокированные)
    Premium,         // +0.5% к доходности, стоимость: 1 SOL
    VIP,             // +1% к доходности, -25% комиссия продажи, стоимость: 2 SOL
    Legendary,       // +2% к доходности, -50% комиссия продажи, стоимость: 5 SOL
}

/// Стоимость премиум слотов
pub const PREMIUM_SLOT_COSTS: [u64; 3] = [
    1_000_000_000,   // Premium: 1 SOL
    2_000_000_000,   // VIP: 2 SOL  
    5_000_000_000,   // Legendary: 5 SOL
];

/// Бонусы доходности для премиум слотов
pub const PREMIUM_SLOT_YIELD_BONUSES: [u16; 3] = [50, 100, 200]; // +0.5%, +1%, +2%

/// Скидки на комиссию продажи для премиум слотов
pub const PREMIUM_SLOT_SELL_FEE_DISCOUNTS: [u8; 3] = [0, 50, 100]; // 0%, -50%, -100%


// ============================================================================
// SELL FEES - БЕЗ ИЗМЕНЕНИЙ
// ============================================================================

/// Early sell fees by days held (убывающая комиссия)
pub const EARLY_SELL_FEES: [u8; 32] = [
    25, 25, 25, 25, 25, 25, 25, // Days 0-6: 25%
    20, 20, 20, 20, 20, 20, 20, // Days 7-13: 20%
    15, 15, 15, 15, 15, 15, 15, // Days 14-20: 15%
    10, 10, 10, 10, 10, 10, 10, // Days 21-27: 10%
    5,  5,  5,  2,              // Days 28-30: 5%, final: 2%
];

pub const FINAL_SELL_FEE_PERCENT: u8 = 2;


// ============================================================================
// GAME CONFIGURATION CONSTANTS
// ============================================================================

/// Earnings interval in seconds (24 hours)
pub const EARNINGS_INTERVAL: i64 = 86_400;

/// Initial entry fee ($2 at $162/SOL = ~0.012 SOL) - Backend will control actual value
pub const INITIAL_ENTRY_FEE: u64 = 12_345_679; // 0.012345679 SOL (~$2)

/// Legacy constants (kept for existing fields compatibility)
pub const BASE_ENTRY_FEE: u64 = INITIAL_ENTRY_FEE;
pub const MAX_ENTRY_FEE: u64 = 123_456_790; 
pub const FEE_INCREMENT: u64 = 6_000_000;
pub const PLAYERS_PER_MILESTONE: u64 = 100;
pub const ENTRY_FEE: u64 = INITIAL_ENTRY_FEE;

/// Treasury fee percentage
pub const TREASURY_FEE_PERCENT: u8 = 20; // 20%

/// Claim earnings fee percentage (goes to team wallet)
pub const CLAIM_EARNINGS_FEE_PERCENT: u8 = 2; // 2% от суммы claim

/// Auto claim purchase cost (one-time payment for unlimited auto accumulation)
pub const AUTO_CLAIM_COST: u64 = 50_000_000; // 0.05 SOL

/// Upgrade costs (legacy - using multipliers now)
pub const UPGRADE_COSTS: [u64; 3] = [
    100_000_000,   // 0.1 SOL for level 1
    500_000_000,   // 0.5 SOL for level 2
    2_000_000_000, // 2 SOL for level 3
];

/// Upgrade bonuses (legacy - using UPGRADE_YIELD_BONUSES now)
pub const UPGRADE_BONUSES: [u16; 3] = UPGRADE_YIELD_BONUSES;


// ============================================================================
// COMPATIBILITY CONSTANTS  
// ============================================================================

/// Maximum businesses per player (legacy)
pub const MAX_BUSINESSES_PER_PLAYER: u8 = 20;

// ============================================================================
// ADMIN CONFIGURATION - ДЛЯ МАКСИМАЛЬНОЙ ДЕЦЕНТРАЛИЗАЦИИ
// ============================================================================

/// 🔒 ЗАХАРДКОЖЕННЫЙ ADMIN PUBKEY - ЕДИНСТВЕННЫЙ КТО МОЖЕТ МЕНЯТЬ ENTRY FEE
pub const HARDCODED_ADMIN_PUBKEY: Pubkey = anchor_lang::prelude::Pubkey::from_str_const("HLWTn3BYB3jvgquBG323XLyqzEj11H4N5m6EMpPGCCG6");

// ============================================================================
// PDA SEEDS
// ============================================================================

pub const GAME_STATE_SEED: &[u8] = b"game_state";
pub const GAME_CONFIG_SEED: &[u8] = b"game_config"; 
pub const TREASURY_SEED: &[u8] = b"treasury";
pub const PLAYER_SEED: &[u8] = b"player";
