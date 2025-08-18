use anchor_lang::prelude::*;

pub mod constants; 
pub mod error;
pub mod instructions;
pub mod state; 

use state::*;
use constants::*;

// ============ EVENTS ============
#[event]
pub struct PlayerCreated {
    pub wallet: Pubkey,
    pub entry_fee: u64,
    pub created_at: i64,
}

#[event]
pub struct BusinessCreated {
    pub player: Pubkey,
    pub business_type: u8,
    pub invested_amount: u64,
    pub daily_rate: u16,
    pub treasury_fee: u64,
    pub created_at: i64,
}

// ❌ УДАЛЕНО: EarningsUpdated event больше не нужен в новой системе
// Теперь earnings рассчитываются только при claim, нет автообновлений

#[event]
pub struct AutoClaimPurchased {
    pub player: Pubkey,
    pub cost: u64,
    pub purchased_at: i64,
}

#[event]
pub struct EarningsClaimed {
    pub player: Pubkey,
    pub amount: u64,
    pub claimed_at: i64,
}

#[event]
pub struct BusinessUpgraded {
    pub player: Pubkey,
    pub business_index: u8,
    pub new_level: u8,
    pub upgrade_cost: u64,
    pub new_daily_rate: u16,
}

#[event]
pub struct BusinessSold {
    pub player: Pubkey,
    pub business_index: u8,
    pub days_held: u64,
    pub sell_fee_percent: u8,
    pub return_amount: u64,
}

// 🏪 Старые slot events удалены - теперь используется только BusinessCreatedInSlot с slot_cost

#[event]
pub struct BusinessCreatedInSlot {
    pub player: Pubkey,
    pub slot_index: u8,
    pub business_type: u8,
    pub level: u8,  // 🆕 Добавляем уровень бизнеса
    pub base_cost: u64,
    pub slot_cost: u64,
    pub total_paid: u64,
    pub daily_rate: u16,
    pub created_at: i64,
}

#[event]
pub struct BusinessUpgradedInSlot {
    pub player: Pubkey,
    pub slot_index: u8,
    pub old_level: u8,
    pub new_level: u8,
    pub upgrade_cost: u64,
    pub new_daily_rate: u16,
    pub upgraded_at: i64,
}

#[event]
pub struct BusinessSoldFromSlot {
    pub player: Pubkey,
    pub slot_index: u8,
    pub business_type: u8,
    pub total_invested: u64,
    pub days_held: u64,
    pub base_fee_percent: u8,
    pub slot_discount: u8,
    pub final_fee_percent: u8,
    pub return_amount: u64,
    pub sold_at: i64,
}

// ============ FRONTEND DATA STRUCTURES ============
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerFrontendData {
    pub wallet: Pubkey,
    pub total_invested: u64,
    pub total_earned: u64,
    pub claimable_earnings: u64,       // 🆕 Заменяет pending_earnings
    pub businesses_count: u8,
    pub active_businesses: u8,
    pub auto_claim_purchased: bool,    // 🆕 Показать куплены ли автонакопления
    pub can_claim: bool,               // 🆕 Можно ли клэймить сейчас
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalStats {
    pub total_players: u64,
    pub total_invested: u64,
    pub total_withdrawn: u64,
    pub total_businesses: u64,
    pub total_treasury_collected: u64,
}

declare_id!("9h2uDYXv48GAfSXzprXDgDKBCkxAv7yRY2pDbZeGnZXF");

#[program]
pub mod solana_mafia {
    use super::*;

    /// Initialize the game with treasury wallet
    pub fn initialize(ctx: Context<Initialize>, treasury_wallet: Pubkey) -> Result<()> {
        instructions::admin::initialize(ctx, treasury_wallet)
    }

    /// Create new player (with entry fee)
    pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
        instructions::player::create_player(ctx)
    }

    /// 🏪 Create business in specific slot
    pub fn create_business(
        ctx: Context<CreateBusinessInSlot>,
        business_type: u8,
        deposit_amount: u64,
        slot_index: u8,
    ) -> Result<()> {
        instructions::business::create_business(ctx, business_type, deposit_amount, slot_index)
    }

    /// 🆕 Create business with target level (immediate upgrades)
    pub fn create_business_with_level(
        ctx: Context<CreateBusinessInSlot>,
        business_type: u8,
        deposit_amount: u64,
        slot_index: u8,
        target_level: u8,
    ) -> Result<()> {
        instructions::business::create_business_with_level(ctx, business_type, deposit_amount, slot_index, target_level)
    }

    /// Claim earnings with new individual business tracking system
    pub fn claim_earnings(ctx: Context<ClaimEarnings>) -> Result<()> {
        instructions::earnings::claim_earnings(ctx)
    }

    /// 🆕 Purchase auto claim feature for 0.05 SOL (one-time payment)
    pub fn purchase_auto_claim(ctx: Context<PurchaseAutoClaim>) -> Result<()> {
        instructions::earnings::purchase_auto_claim(ctx)
    }

    /// Health check for player data
    pub fn health_check_player(ctx: Context<HealthCheckPlayer>) -> Result<()> {
        instructions::player::health_check_player(ctx)
    }

    /// 🔥 Sell business from slot (with early exit fees)
    pub fn sell_business(
        ctx: Context<SellBusinessFromSlot>,
        slot_index: u8,
    ) -> Result<()> {
        instructions::business::sell_business(ctx, slot_index)
    }




    /// 🆕 Обновить индекс данных игрока (для бэкенда) - фронтенд должен читать напрямую!
    pub fn update_player_index(ctx: Context<GetPlayerData>) -> Result<()> {
        instructions::player::get_player_data(ctx)
    }

    /// 🆕 Получить глобальную статистику
    pub fn get_global_stats(ctx: Context<GetGlobalStats>) -> Result<()> {
        instructions::earnings::get_global_stats(ctx)
    }

    /// 🆕 Получить только валидные (принадлежащие) бизнесы игрока
    pub fn get_valid_player_businesses(ctx: Context<GetValidPlayerBusinesses>) -> Result<()> {
        instructions::player::get_valid_player_businesses(ctx)
    }

    // 🏪 Старые функции unlock_business_slot и buy_premium_slot удалены
    // В новой системе все слоты автоматически доступны и оплачиваются при первом использовании

    /// ⬆️ Upgrade business in slot
    pub fn upgrade_business(
        ctx: Context<UpgradeBusinessInSlot>,
        slot_index: u8,
    ) -> Result<()> {
        instructions::business::upgrade_business(ctx, slot_index)
    }
    
    /// 💰 Update entry fee (admin only) - for backend control and promotions
    pub fn update_entry_fee(ctx: Context<UpdateEntryFee>, new_fee_lamports: u64) -> Result<()> {
        instructions::admin::update_entry_fee(ctx, new_fee_lamports)
    }
}

// ===== ACCOUNT CONTEXTS =====

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = GameState::SIZE,
        seeds = [GAME_STATE_SEED],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        init,
        payer = authority,
        space = GameConfig::SIZE,
        seeds = [GAME_CONFIG_SEED],
        bump
    )]
    pub game_config: Account<'info, GameConfig>,
    
    #[account(
        init,
        payer = authority,
        space = Treasury::SIZE,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury_pda: Account<'info, Treasury>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Player::SIZE,
        seeds = [PLAYER_SEED, owner.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(
        seeds = [GAME_CONFIG_SEED],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        mut,
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,

    /// Treasury wallet where entry fee goes
    /// CHECK: This is validated against game_state.treasury_wallet in create_player instruction
    #[account(mut)]
    pub treasury_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct PurchaseAutoClaim<'info> {
    #[account(mut)]
    pub player_owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PLAYER_SEED, player_owner.key().as_ref()],
        bump = player.bump,
        constraint = player.owner == player_owner.key()
    )]
    pub player: Box<Account<'info, Player>>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury_pda.bump
    )]
    pub treasury_pda: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimEarnings<'info> {
    #[account(mut)]
    pub player_owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PLAYER_SEED, player_owner.key().as_ref()],
        bump = player.bump,
        constraint = player.owner == player_owner.key()
    )]
    pub player: Box<Account<'info, Player>>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury_pda.bump
    )]
    pub treasury_pda: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,
    
    /// 💰 Treasury wallet для получения claim fee (0.01 SOL)
    /// CHECK: Address is validated against game_state.treasury_wallet constraint
    #[account(
        mut,
        address = game_state.treasury_wallet
    )]
    pub treasury_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HealthCheckPlayer<'info> {
    #[account(
        seeds = [PLAYER_SEED, player.owner.as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, Player>,
}




#[derive(Accounts)]
pub struct GetPlayerData<'info> {
    #[account(
        seeds = [PLAYER_SEED, player.owner.as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, Player>,
}

#[derive(Accounts)]
pub struct GetGlobalStats<'info> {
    #[account(
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,
}









#[derive(Accounts)]
pub struct GetValidPlayerBusinesses<'info> {
    #[account(
        seeds = [PLAYER_SEED, player.owner.as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, Player>,
}

// 🏪 Старые context структуры UnlockBusinessSlot и BuyPremiumSlot удалены
// В новой системе все слоты автоматически доступны и оплачиваются в create_business

#[derive(Accounts)]
pub struct CreateBusinessInSlot<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = Player::SIZE,
        seeds = [PLAYER_SEED, owner.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,

    #[account(
        seeds = [GAME_CONFIG_SEED],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        mut,
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,

    /// CHECK: Treasury wallet validated against game_state.treasury_wallet address constraint
    #[account(
        mut,
        address = game_state.treasury_wallet
    )]
    pub treasury_wallet: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury_pda.bump
    )]
    pub treasury_pda: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradeBusinessInSlot<'info> {
    #[account(mut)]
    pub player_owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PLAYER_SEED, player_owner.key().as_ref()],
        bump = player.bump,
        constraint = player.owner == player_owner.key()
    )]
    pub player: Box<Account<'info, Player>>,
    
    #[account(
        mut,
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,
    
    #[account(
        seeds = [GAME_CONFIG_SEED],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,
    
    /// CHECK: Treasury wallet validated against game_state.treasury_wallet address constraint
    #[account(
        mut,
        address = game_state.treasury_wallet
    )]
    pub treasury_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SellBusinessFromSlot<'info> {
    #[account(mut)]
    pub player_owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PLAYER_SEED, player_owner.key().as_ref()],
        bump = player.bump,
        constraint = player.owner == player_owner.key()
    )]
    pub player: Box<Account<'info, Player>>,
    
    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump = treasury_pda.bump
    )]
    pub treasury_pda: Box<Account<'info, Treasury>>,
    
    #[account(
        mut,
        seeds = [GAME_STATE_SEED],
        bump = game_state.bump
    )]
    pub game_state: Box<Account<'info, GameState>>,

    #[account(
        seeds = [GAME_CONFIG_SEED],
        bump = game_config.bump
    )]
    pub game_config: Box<Account<'info, GameConfig>>,

    /// CHECK: Treasury wallet for simulation visibility in Phantom
    #[account(
        mut,
        address = game_state.treasury_wallet
    )]
    pub treasury_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetPlayerSlotData<'info> {
    #[account(
        seeds = [PLAYER_SEED, player.owner.as_ref()],
        bump = player.bump
    )]
    pub player: Account<'info, Player>,
}

#[derive(Accounts)]
pub struct UpdateEntryFee<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [GAME_CONFIG_SEED],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,
}


