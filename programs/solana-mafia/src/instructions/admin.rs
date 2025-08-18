use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::SolanaMafiaError;
use crate::Initialize;

/// Initialize the game with treasury wallet
pub fn initialize(ctx: Context<Initialize>, treasury_wallet: Pubkey) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let game_config = &mut ctx.accounts.game_config;
    let treasury_pda = &mut ctx.accounts.treasury_pda;
    let clock = Clock::get()?;
    
    // Initialize GameState (БЕЗ is_paused!)
    **game_state = GameState::new(
        ctx.accounts.authority.key(),
        treasury_wallet,
        clock.unix_timestamp,
        ctx.bumps.game_state,
    );
    
    // Initialize GameConfig
    **game_config = GameConfig::new(
        ctx.accounts.authority.key(),
        ctx.bumps.game_config,
    );

    // Initialize Treasury PDA
    **treasury_pda = Treasury::new(ctx.bumps.treasury_pda);
    
    msg!("🎮 Solana Mafia initialized!");
    msg!("Authority: {}", ctx.accounts.authority.key());
    msg!("Treasury: {}", treasury_wallet);
    
    Ok(())
}


/// 🔒 Update entry fee (admin only) - ЗАХАРДКОЖЕННАЯ ПРОВЕРКА ДЛЯ ДЕЦЕНТРАЛИЗАЦИИ
pub fn update_entry_fee(ctx: Context<crate::UpdateEntryFee>, new_fee_lamports: u64) -> Result<()> {
    let game_config = &mut ctx.accounts.game_config;
    
    // 🚨 ЗАХАРДКОЖЕННАЯ ПРОВЕРКА - ТОЛЬКО ОДИН ADMIN МОЖЕТ МЕНЯТЬ FEE!
    if ctx.accounts.authority.key() != crate::constants::HARDCODED_ADMIN_PUBKEY {
        return Err(SolanaMafiaError::UnauthorizedAdmin.into());
    }
    
    // Update fee using the method
    game_config.update_entry_fee(new_fee_lamports)?;
    
    msg!("💰 Entry fee updated to {} lamports ({:.4} SOL)", new_fee_lamports, new_fee_lamports as f64 / 1_000_000_000.0);
    
    Ok(())
}

