use anchor_lang::prelude::*;

/// Treasury PDA для хранения средств игроков (80% от депозитов)
#[account]
pub struct Treasury {
    pub bump: u8,
}

impl Treasury {
    pub const SIZE: usize = 8 + 1; // discriminator + bump
    
    /// Create new treasury
    pub fn new(bump: u8) -> Self {
        Self { bump }
    }
}