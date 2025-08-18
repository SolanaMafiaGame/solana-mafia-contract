use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::*;

/// Validate business type index
pub fn validate_business_type(business_type: u8) -> Result<()> {
    if business_type as usize >= BUSINESS_TYPES_COUNT {
        return Err(SolanaMafiaError::InvalidBusinessType.into());
    }
    Ok(())
}

/// Validate deposit amount against minimum
pub fn validate_deposit_amount(deposit_amount: u64, min_deposit: u64) -> Result<()> {
    if deposit_amount < min_deposit {
        return Err(SolanaMafiaError::InsufficientDeposit.into());
    }
    Ok(())
}

/// Validate upgrade level
pub fn validate_upgrade_level(level: u8) -> Result<()> {
    if level == 0 || level > MAX_UPGRADE_LEVEL {
        return Err(SolanaMafiaError::InvalidUpgradeLevel.into());
    }
    Ok(())
}