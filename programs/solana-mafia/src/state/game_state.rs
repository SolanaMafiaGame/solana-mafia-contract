use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub authority: Pubkey,
    pub treasury_wallet: Pubkey,
    pub total_players: u64,
    pub total_invested: u64,
    pub total_withdrawn: u64,
    pub total_treasury_collected: u64,
    pub total_businesses: u64,
    pub is_paused: bool,
    pub created_at: i64,
    pub bump: u8,
}

impl GameState {
    /// Size for account allocation
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        32 + // treasury_wallet
        8 + // total_players
        8 + // total_invested
        8 + // total_withdrawn
        8 + // total_treasury_collected
        8 + // total_businesses
        1 + // is_paused
        8 + // created_at
        1; // bump

    /// Create new game state
    pub fn new(
        authority: Pubkey,
        treasury_wallet: Pubkey,
        created_at: i64,
        bump: u8,
    ) -> Self {
        Self {
            authority,
            treasury_wallet,
            total_players: 0,
            total_invested: 0,
            total_withdrawn: 0,
            total_treasury_collected: 0,
            total_businesses: 0,
            is_paused: false,
            created_at,
            bump,
        }
    }

    /// Add new player
    pub fn add_player(&mut self) {
        self.total_players += 1;
    }

    /// Add investment
    pub fn add_investment(&mut self, amount: u64) {
        self.total_invested += amount;
    }

    /// Add withdrawal
    pub fn add_withdrawal(&mut self, amount: u64) {
        self.total_withdrawn += amount;
    }

    /// Add treasury collection
    pub fn add_treasury_collection(&mut self, amount: u64) {
        self.total_treasury_collected += amount;
    }

    /// Add business
    pub fn add_business(&mut self) {
        self.total_businesses += 1;
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

}