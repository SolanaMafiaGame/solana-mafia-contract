// error.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum SolanaMafiaError {
    // 💸 ENTRY / DEPOSIT ERRORS
    #[msg("Entry fee not paid")]
    EntryFeeNotPaid,

    #[msg("Insufficient deposit amount")]
    InsufficientDeposit,

    #[msg("Invalid referrer")]
    InvalidReferrer,

    #[msg("Cannot refer yourself")]
    CannotReferYourself,

    #[msg("Referrer chain too deep")]
    ReferrerChainTooDeep,

    #[msg("Player already exists")]
    PlayerAlreadyExists,

    // 🧱 BUSINESS SYSTEM
    #[msg("Invalid business type")]
    InvalidBusinessType,

    #[msg("Maximum businesses limit reached")]
    MaxBusinessesReached,

    #[msg("Business not found")]
    BusinessNotFound,

    #[msg("Business not owned by player")]
    BusinessNotOwned,

    #[msg("Business still has active earnings")]
    BusinessHasActiveEarnings,

    // 💰 EARNINGS / CLAIMING
    #[msg("No earnings to claim")]
    NoEarningsToClaim,

    #[msg("Too early to claim earnings")]
    TooEarlyToClaim,

    #[msg("Too early to update earnings")]
    TooEarlyToUpdate,

    #[msg("Earnings update not due yet")]
    EarningsNotDue,

    // ⚙️ UPGRADE SYSTEM
    #[msg("Invalid upgrade level")]
    InvalidUpgradeLevel,

    #[msg("Insufficient funds for upgrade")]
    InsufficientFundsForUpgrade,

    #[msg("Business already at max upgrade level")]
    BusinessMaxLevel,

    #[msg("Cannot upgrade inactive business")]
    CannotUpgradeInactive,

    #[msg("Invalid upgrade level sequence")]
    InvalidUpgradeSequence,

    #[msg("Upgrade cost mismatch")]
    UpgradeCostMismatch,

    // 🪪 NFT SYSTEM
    #[msg("NFT upgrade failed")]
    NFTUpgradeFailed,

    #[msg("NFT burn failed")]
    NFTBurnFailed,

    #[msg("NFT metadata creation failed")]
    NFTMetadataFailed,

    // 🧩 SLOT SYSTEM
    #[msg("Slot already unlocked")]
    SlotAlreadyUnlocked,

    #[msg("Slot not unlocked")]
    SlotNotUnlocked,

    #[msg("Slot is occupied")]
    SlotOccupied,

    #[msg("Invalid slot index")]
    InvalidSlotIndex,

    #[msg("No more slots to unlock")]
    NoMoreSlotsToUnlock,

    #[msg("Invalid slot type")]
    InvalidSlotType,

    #[msg("Insufficient funds for slot unlock")]
    InsufficientFundsForSlot,

    #[msg("Slot is empty")]
    SlotEmpty,

    #[msg("Slot already occupied")]
    SlotAlreadyOccupied,

    #[msg("Slot already paid")]
    SlotAlreadyPaid,

    #[msg("Business not active")]
    BusinessNotActive,

    #[msg("Maximum level reached")]
    MaxLevelReached,

    #[msg("No slots to unlock")]
    NoSlotsToUnlock,

    #[msg("No slots available")]
    NoSlotsAvailable,

    // 🚫 ADMIN & EMERGENCY
    #[msg("Unauthorized admin action")]
    UnauthorizedAdmin,

    #[msg("Emergency pause activated")]
    EmergencyPauseActive,

    #[msg("Only emergency admin can use this function")]
    UnauthorizedEmergencyAdmin,

    #[msg("Game is paused")]
    GamePaused,

    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,

    // 🧮 MISC
    #[msg("Too early to create another business")]
    TooEarlyToCreateBusiness,

    #[msg("Math overflow")]
    MathOverflow,

    // 🆕 NEW CLAIM SYSTEM ERRORS
    #[msg("Auto claim already purchased")]
    AutoClaimAlreadyPurchased,

    #[msg("Claim too early - need 24 hours between claims")]
    ClaimTooEarly,

    #[msg("Auto claim not purchased")]
    AutoClaimNotPurchased,
}
