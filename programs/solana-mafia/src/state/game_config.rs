// state/game_config.rs
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::SolanaMafiaError;

#[account]
pub struct GameConfig {
    /// Admin authority who can update config
    pub authority: Pubkey,
    
    /// Daily rates for each business type (basis points)
    pub business_rates: [u16; BUSINESS_TYPES_COUNT],
    
    /// Minimum deposits for each business type
    pub min_deposits: [u64; BUSINESS_TYPES_COUNT],
    
    /// Base entry fee (starts from this amount)
    pub base_entry_fee: u64,
    
    /// Maximum entry fee (caps at this amount)
    pub max_entry_fee: u64,
    
    /// Fee increment per milestone
    pub fee_increment: u64,
    
    /// Players per milestone
    pub players_per_milestone: u64,
    
    /// Treasury fee percentage (what goes to team)
    pub treasury_fee_percent: u8,
    
    /// Upgrade costs for each level
    pub upgrade_costs: [u64; MAX_UPGRADE_LEVEL as usize],
    
    /// Upgrade bonuses for each level (basis points)
    pub upgrade_bonuses: [u16; MAX_UPGRADE_LEVEL as usize],
    
    
    /// Maximum businesses per player
    pub max_businesses_per_player: u8,
    
    /// Whether new registrations are allowed
    pub registrations_open: bool,
    
    /// Current entry fee in lamports (controlled by backend)
    pub current_entry_fee: u64,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl GameConfig {
    /// Size for account allocation
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        2 * BUSINESS_TYPES_COUNT + // business_rates
        8 * BUSINESS_TYPES_COUNT + // min_deposits
        8 + // base_entry_fee
        8 + // max_entry_fee
        8 + // fee_increment
        8 + // players_per_milestone
        1 + // treasury_fee_percent
        8 * (MAX_UPGRADE_LEVEL as usize) + // upgrade_costs
        2 * (MAX_UPGRADE_LEVEL as usize) + // upgrade_bonuses
        1 + // max_businesses_per_player
        1 + // registrations_open
        8 + // current_entry_fee
        1; // bump

    /// Create new config with default values
    pub fn new(authority: Pubkey, bump: u8) -> Self {
        Self {
            authority,
            business_rates: BUSINESS_RATES,
            min_deposits: MIN_DEPOSITS,
            base_entry_fee: BASE_ENTRY_FEE,
            max_entry_fee: MAX_ENTRY_FEE,
            fee_increment: FEE_INCREMENT,
            players_per_milestone: PLAYERS_PER_MILESTONE,
            treasury_fee_percent: TREASURY_FEE_PERCENT,
            upgrade_costs: UPGRADE_COSTS,
            upgrade_bonuses: UPGRADE_BONUSES,
            max_businesses_per_player: MAX_BUSINESSES_PER_PLAYER,
            registrations_open: true,
            current_entry_fee: INITIAL_ENTRY_FEE, // Start with initial fee, backend will control
            bump,
        }
    }

    /// Get daily rate for business type
    pub fn get_business_rate(&self, business_type_index: usize) -> u16 {
        if business_type_index < BUSINESS_TYPES_COUNT {
            self.business_rates[business_type_index]
        } else {
            0
        }
    }

    /// Get minimum deposit for business type
    pub fn get_min_deposit(&self, business_type_index: usize) -> u64 {
        if business_type_index < BUSINESS_TYPES_COUNT {
            self.min_deposits[business_type_index]
        } else {
            0
        }
    }

    /// Get upgrade cost for level
    pub fn get_upgrade_cost(&self, level: u8) -> Option<u64> {
        if level > 0 && level <= MAX_UPGRADE_LEVEL {
            Some(self.upgrade_costs[(level - 1) as usize])
        } else {
            None
        }
    }

    /// Get upgrade bonus for level
    pub fn get_upgrade_bonus(&self, level: u8) -> u16 {
        if level > 0 && level <= MAX_UPGRADE_LEVEL {
            self.upgrade_bonuses[(level - 1) as usize]
        } else {
            0
        }
    }


    /// Get current entry fee with FOMO calculation based on total players
    pub fn get_current_entry_fee(&self, total_players: u64) -> u64 {
        // Use manual override if set by admin, otherwise calculate FOMO
        if self.current_entry_fee > self.base_entry_fee {
            self.current_entry_fee
        } else {
            self.calculate_fomo_entry_fee(total_players)
        }
    }
    
    /// Get next fee level based on player milestone
    pub fn get_next_entry_fee(&self, total_players: u64) -> u64 {
        self.calculate_fomo_entry_fee(total_players + self.players_per_milestone)
    }
    
    /// Calculate dynamic FOMO entry fee based on total players
    pub fn calculate_fomo_entry_fee(&self, total_players: u64) -> u64 {
        if total_players == 0 {
            return self.base_entry_fee;
        }
        
        // Calculate milestones reached
        let milestones_reached = total_players / self.players_per_milestone;
        
        // Calculate fee with increment per milestone
        let calculated_fee = self.base_entry_fee + (milestones_reached * self.fee_increment);
        
        // Cap at maximum
        if calculated_fee > self.max_entry_fee {
            self.max_entry_fee
        } else {
            calculated_fee
        }
    }
    
    /// Update entry fee (admin only) - for backend control and promotions
    pub fn update_entry_fee(&mut self, new_fee_lamports: u64) -> Result<()> {
        if new_fee_lamports == 0 {
            return Err(SolanaMafiaError::InsufficientDeposit.into());
        }
        self.current_entry_fee = new_fee_lamports;
        Ok(())
    }
}