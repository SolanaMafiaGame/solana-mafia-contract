use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::error::SolanaMafiaError;
use crate::constants::*;

/// 🆕 Claim earnings with new individual business tracking system
pub fn claim_earnings(ctx: Context<crate::ClaimEarnings>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    // 🚫 Проверка минимального интервала для пользователей без автонакоплений
    if !player.auto_claim_purchased {
        require!(
            player.can_claim_without_auto(current_time),
            SolanaMafiaError::ClaimTooEarly
        );
    }
    
    // 💰 Рассчитываем earnings: без auto-claim = полная суточная доходность, с auto-claim = точный расчет по времени
    let claimable_amount = player.get_claimable_amount(current_time)?;
    
    if claimable_amount == 0 {
        return Err(SolanaMafiaError::NoEarningsToClaim.into());
    }
    
    // Calculate claim fee (2% от суммы claim)
    let claim_fee = (claimable_amount as u128 * CLAIM_EARNINGS_FEE_PERCENT as u128 / 100) as u64;
    let net_amount = claimable_amount.saturating_sub(claim_fee);
    
    // Check treasury has enough funds
    let treasury_balance = ctx.accounts.treasury_pda.to_account_info().lamports();
    if treasury_balance < claimable_amount {
        return Err(ProgramError::InsufficientFunds.into());
    }
    
    // Transfer earnings from treasury PDA to player using manual lamports manipulation
    if net_amount > 0 {
        **ctx.accounts.treasury_pda.to_account_info().try_borrow_mut_lamports()? -= net_amount;
        **ctx.accounts.player_owner.to_account_info().try_borrow_mut_lamports()? += net_amount;
        msg!("💰 Transferred {} lamports earnings to player", net_amount);
    }
    
    // Transfer claim fee from treasury PDA to admins using manual lamports manipulation
    if claim_fee > 0 {
        **ctx.accounts.treasury_pda.to_account_info().try_borrow_mut_lamports()? -= claim_fee;
        **ctx.accounts.treasury_wallet.to_account_info().try_borrow_mut_lamports()? += claim_fee;
        msg!("💳 Claim fee {} lamports sent to admins", claim_fee);
    }
    
    // 🆕 Обновляем статистику игрока и времена claim всех бизнесов
    player.process_claim(claimable_amount, current_time)?;
    
    // Update game statistics
    game_state.add_withdrawal(claimable_amount);

    emit!(crate::EarningsClaimed {
        player: ctx.accounts.player_owner.key(),
        amount: claimable_amount,
        claimed_at: current_time,
    });
    
    msg!("💰 Claimed {} lamports (net: {}, fee: {} [{}%]) [auto_claim: {}]", 
         claimable_amount, net_amount, claim_fee, CLAIM_EARNINGS_FEE_PERCENT, player.auto_claim_purchased);
    Ok(())
}

/// 🆕 Купить автонакопления за 0.05 SOL (разовая покупка на всю игру)
pub fn purchase_auto_claim(ctx: Context<crate::PurchaseAutoClaim>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;
    
    // Проверяем что автонакопления еще не куплены
    require!(
        !player.auto_claim_purchased,
        SolanaMafiaError::AutoClaimAlreadyPurchased
    );
    
    // Переводим 0.05 SOL с игрока в treasury PDA
    let auto_claim_cost = AUTO_CLAIM_COST;
    
    // Transfer from player to treasury PDA
    let transfer_instruction = system_program::Transfer {
        from: ctx.accounts.player_owner.to_account_info(),
        to: ctx.accounts.treasury_pda.to_account_info(),
    };
    
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
    );
    
    system_program::transfer(cpi_context, auto_claim_cost)?;
    
    // Активируем автонакопления для игрока
    player.purchase_auto_claim()?;
    
    // Обновляем статистику treasury
    game_state.add_investment(auto_claim_cost);
    
    // Эмитим event для отслеживания
    emit!(crate::AutoClaimPurchased {
        player: player.owner,
        cost: auto_claim_cost,
        purchased_at: clock.unix_timestamp,
    });
    
    msg!("🚀 Auto claim purchased for {} lamports by player {}", 
         auto_claim_cost, player.owner);
    
    Ok(())
}

/// 🆕 Получить глобальную статистику (оставлено для совместимости)
pub fn get_global_stats(ctx: Context<crate::GetGlobalStats>) -> Result<()> {
    let game_state = &ctx.accounts.game_state;
    
    // Логируем статистику
    msg!("GLOBAL_STATS: players={}, invested={}, withdrawn={}, businesses={}, treasury={}", 
         game_state.total_players,
         game_state.total_invested,
         game_state.total_withdrawn,
         game_state.total_businesses,
         game_state.total_treasury_collected
    );
    
    Ok(())
}