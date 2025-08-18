use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::*;
use crate::constants::*;
use crate::error::SolanaMafiaError;
// Импорты контекстов убраны - используем прямо через lib.rs

/// 🏪 Create business in specific slot (without NFT)
pub fn create_business(
    ctx: Context<crate::CreateBusinessInSlot>,
    business_type: u8,
    deposit_amount: u64,
    slot_index: u8,
) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game_state = &mut ctx.accounts.game_state;
    let game_config = &ctx.accounts.game_config;
    let clock = Clock::get()?;

    // 🔧 FIX: Инициализировать слоты если игрок новый (created_at == 0)
    if player.created_at == 0 {
        msg!("🆕 Initializing new player slots...");
        
        // 🚨 ENTRY FEE: Взимаем entry fee как в create_player
        let current_total_players = game_state.total_players;
        let entry_fee = game_config.get_current_entry_fee(current_total_players);
        
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.treasury_wallet.to_account_info(),
                },
            ),
            entry_fee,
        )?;
        
        // 🚨 ИСПРАВЛЕНО: Используем правильную инициализацию PlayerCompact::new()
        ***player = PlayerCompact::new(
            ctx.accounts.owner.key(),
            ctx.bumps.player,
            clock.unix_timestamp
        );
        
        // Устанавливаем has_paid_entry=true так как entry fee оплачен
        player.set_has_paid_entry(true);
        
        // Обновляем game_state как в create_player
        game_state.add_player();
        game_state.total_treasury_collected = game_state.total_treasury_collected
            .checked_add(entry_fee)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        msg!("✅ New player initialized with 9 slots (3 free + 3 basic paid + 3 premium), entry fee: {} lamports", entry_fee);
    }

    // Validate slot index
    if slot_index >= MAX_REGULAR_SLOTS {
        return Err(SolanaMafiaError::InvalidSlotIndex.into());
    }

    // Check if slot is available (все слоты разблокированы по умолчанию)
    let slot = &player.business_slots[slot_index as usize];
    if slot.business.is_some() {
        return Err(SolanaMafiaError::SlotAlreadyOccupied.into());
    }

    // Validate business type
    if (business_type as usize) >= BUSINESS_TYPES_COUNT {
        return Err(SolanaMafiaError::InvalidBusinessType.into());
    }

    let business_enum = BusinessType::from_index(business_type).unwrap();
    let min_deposit = business_enum.get_base_cost();
    let daily_rate = business_enum.get_base_rate();

    // Validate deposit amount
    if deposit_amount < min_deposit {
        return Err(SolanaMafiaError::InsufficientDeposit.into());
    }

    // 🏪 НОВОЕ: Проверяем стоимость слота заранее
    let slot_cost = slot.get_slot_cost(deposit_amount);
    let total_payment = deposit_amount
        .checked_add(slot_cost)
        .ok_or(SolanaMafiaError::MathOverflow)?;
    
    if slot_cost > 0 {
        msg!("🏪 Slot cost required: {} lamports ({:.3} SOL)", slot_cost, slot_cost as f64 / 1_000_000_000.0);
    }

    // Calculate fees: 20% to team wallet, 80% to treasury PDA (from business price only)
    let team_fee = deposit_amount
        .checked_mul(20) // 20% to team
        .ok_or(SolanaMafiaError::MathOverflow)?
        .checked_div(100)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    let treasury_amount = deposit_amount
        .checked_sub(team_fee)
        .ok_or(SolanaMafiaError::InsufficientDeposit)?; // 80% to treasury PDA
        
    // 🏪 Комиссия за слот идет полностью команде
    let total_team_fee = team_fee.checked_add(slot_cost)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // Transfer team fee + slot cost to team wallet via CPI
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.treasury_wallet.to_account_info(),
            },
        ),
        total_team_fee,
    )?;

    // Transfer treasury amount to treasury PDA via CPI
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.treasury_pda.to_account_info(),
            },
        ),
        treasury_amount,
    )?;

    // 🔧 ИСПРАВЛЕНО: slot_cost - невозвратный донат, НЕ входит в стоимость бизнеса
    let business_value = deposit_amount; // Только депозит, БЕЗ slot_cost
    
    let business = Business::new(
        business_enum,
        business_value, // Только депозит - возвратная часть
        clock.unix_timestamp,
    );

    // 🏪 Используем новый метод для оплаты слота при необходимости
    let actual_slot_cost = player.pay_slot_if_needed(slot_index as usize, deposit_amount)?;
    
    // Place business in slot
    player.place_business_in_slot(slot_index as usize, business)?;

    // 🔧 ИСПРАВЛЕНО: Учитываем только депозит (slot_cost = невозвратный донат)
    player.total_invested = player.total_invested
        .checked_add(deposit_amount)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // Update game state
    game_state.add_investment(deposit_amount);
    game_state.add_treasury_collection(team_fee); // Track team fees
    game_state.add_business();

    // Set earnings schedule if this is first business
    if player.first_business_time == 0 {
        player.first_business_time = Player::timestamp_to_u32(clock.unix_timestamp);
    }

    emit!(crate::BusinessCreatedInSlot {
        player: ctx.accounts.owner.key(),
        slot_index,
        business_type,
        level: 0, // Базовая функция создает level 0
        base_cost: deposit_amount, // Только депозит (возвратная часть)
        slot_cost: actual_slot_cost,
        total_paid: total_payment,
        daily_rate,
        created_at: clock.unix_timestamp,
    });

    Ok(())
}

/// ⬆️ Upgrade business in slot (simplified without NFT)
pub fn upgrade_business(
    ctx: Context<crate::UpgradeBusinessInSlot>,
    slot_index: u8,
) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let _game_state = &mut ctx.accounts.game_state;
    let _game_config = &ctx.accounts.game_config;
    let clock = Clock::get()?;

    // Validate slot
    if slot_index >= MAX_REGULAR_SLOTS {
        return Err(SolanaMafiaError::InvalidSlotIndex.into());
    }

    let slot = &mut player.business_slots[slot_index as usize];
    let current_business = slot.business.as_mut()
        .ok_or(SolanaMafiaError::BusinessNotFound)?;

    if !current_business.is_active {
        return Err(SolanaMafiaError::BusinessNotActive.into());
    }

    // Check if can upgrade
    let next_level = current_business.upgrade_level + 1;
    if next_level >= MAX_UPGRADE_LEVEL {
        return Err(SolanaMafiaError::MaxLevelReached.into());
    }

    // Calculate upgrade cost
    let upgrade_cost = current_business.get_upgrade_cost(next_level)?;
    
    // Transfer upgrade cost to treasury via CPI
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.player_owner.to_account_info(),
                to: ctx.accounts.treasury_wallet.to_account_info(),
            },
        ),
        upgrade_cost,
    )?;

    // Upgrade business
    current_business.upgrade_to_level(next_level, upgrade_cost)?;
    let new_daily_rate = current_business.daily_rate;

    // 🚨 ИСПРАВЛЕНО: Используем u64 напрямую без конвертации
    player.total_upgrade_spent = player.total_upgrade_spent
        .checked_add(upgrade_cost)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // 🔧 КРИТИЧЕСКОЕ ИСПРАВЛЕНИЕ: Увеличиваем total_invested при апгрейде
    player.total_invested = player.total_invested
        .checked_add(upgrade_cost)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    emit!(crate::BusinessUpgradedInSlot {
        player: ctx.accounts.player_owner.key(),
        slot_index,
        old_level: next_level - 1,
        new_level: next_level,
        upgrade_cost,
        new_daily_rate,
        upgraded_at: clock.unix_timestamp,
    });

    Ok(())
}

/// 🔥 Sell business from slot (simplified without NFT burning)
pub fn sell_business(
    ctx: Context<crate::SellBusinessFromSlot>,
    slot_index: u8,
) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game_state = &mut ctx.accounts.game_state;
    let clock = Clock::get()?;

    // Validate slot
    if slot_index >= MAX_REGULAR_SLOTS {
        return Err(SolanaMafiaError::InvalidSlotIndex.into());
    }

    let slot = &mut player.business_slots[slot_index as usize];
    let business = slot.business.take()
        .ok_or(SolanaMafiaError::BusinessNotFound)?;

    // 🔧 CRITICAL FIX: Reset slot occupied flag after removing business
    slot.set_has_business(false);

    if !business.is_active {
        return Err(SolanaMafiaError::BusinessNotActive.into());
    }

    // Calculate how long business was held
    let days_held = (clock.unix_timestamp - business.created_at) / 86400;
    let days_held_capped = std::cmp::min(days_held as usize, EARLY_SELL_FEES.len() - 1);

    // Calculate sell fee with slot discount
    let base_fee_percent = if days_held_capped < EARLY_SELL_FEES.len() {
        EARLY_SELL_FEES[days_held_capped]
    } else {
        FINAL_SELL_FEE_PERCENT
    };

    let slot_discount = slot.get_sell_fee_discount();
    let final_fee_percent = base_fee_percent.saturating_sub(slot_discount);

    // Calculate return amount
    let total_invested = business.get_total_investment_for_refund();
    let sell_fee = total_invested
        .checked_mul(final_fee_percent as u64)
        .ok_or(SolanaMafiaError::MathOverflow)?
        .checked_div(100)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    let return_amount = total_invested
        .checked_sub(sell_fee)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // Return funds to player from treasury PDA using manual lamports manipulation
    // (System Program can't transfer from accounts with data, so we do it manually)
    **ctx.accounts.treasury_pda.to_account_info().try_borrow_mut_lamports()? -= return_amount;
    **ctx.accounts.player_owner.to_account_info().try_borrow_mut_lamports()? += return_amount;

    // 🔧 КРИТИЧЕСКОЕ ИСПРАВЛЕНИЕ: Уменьшаем total_invested при продаже
    player.total_invested = player.total_invested
        .checked_sub(total_invested)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // Update statistics
    game_state.add_withdrawal(return_amount);

    emit!(crate::BusinessSoldFromSlot {
        player: ctx.accounts.player_owner.key(),
        slot_index,
        business_type: business.business_type.to_index() as u8,
        total_invested,
        days_held: days_held as u64,
        base_fee_percent,
        slot_discount,
        final_fee_percent,
        return_amount,
        sold_at: clock.unix_timestamp,
    });

    Ok(())
}

/// 🆕 Create business with target level (immediate upgrades)
pub fn create_business_with_level(
    ctx: Context<crate::CreateBusinessInSlot>,
    business_type: u8,
    deposit_amount: u64,
    slot_index: u8,
    target_level: u8,
) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game_state = &mut ctx.accounts.game_state;
    let game_config = &ctx.accounts.game_config;
    let clock = Clock::get()?;

    // Validate target level
    if target_level > MAX_UPGRADE_LEVEL {
        return Err(SolanaMafiaError::MaxLevelReached.into());
    }

    // 🔧 FIX: Инициализировать слоты если игрок новый (код такой же как в create_business)
    if player.created_at == 0 {
        msg!("🆕 Initializing new player slots...");
        
        // 🚨 ENTRY FEE: Взимаем entry fee
        let current_total_players = game_state.total_players;
        let entry_fee = game_config.get_current_entry_fee(current_total_players);
        
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.treasury_wallet.to_account_info(),
                },
            ),
            entry_fee,
        )?;
        
        // 🚨 ИСПРАВЛЕНО: Используем правильную инициализацию PlayerCompact::new()
        ***player = PlayerCompact::new(
            ctx.accounts.owner.key(),
            ctx.bumps.player,
            clock.unix_timestamp
        );
        
        // Устанавливаем has_paid_entry=true так как entry fee оплачен
        player.set_has_paid_entry(true);
        
        game_state.add_player();
        game_state.total_treasury_collected = game_state.total_treasury_collected
            .checked_add(entry_fee)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        msg!("✅ New player initialized with 9 slots, entry fee: {} lamports", entry_fee);
    }

    // Validate slot and business type (код как в create_business)
    if slot_index >= MAX_REGULAR_SLOTS {
        return Err(SolanaMafiaError::InvalidSlotIndex.into());
    }

    let slot = &player.business_slots[slot_index as usize];
    if slot.business.is_some() {
        return Err(SolanaMafiaError::SlotAlreadyOccupied.into());
    }

    if (business_type as usize) >= BUSINESS_TYPES_COUNT {
        return Err(SolanaMafiaError::InvalidBusinessType.into());
    }

    let business_enum = BusinessType::from_index(business_type).unwrap();
    let base_cost = business_enum.get_base_cost();
    
    // 🆕 Рассчитываем стоимость апгрейдов
    let mut upgrade_costs = [0u64; 3];
    for level in 1..=target_level {
        let multiplier = UPGRADE_COST_MULTIPLIERS[(level - 1) as usize];
        upgrade_costs[(level - 1) as usize] = base_cost
            .checked_mul(multiplier as u64)
            .and_then(|x| x.checked_div(100))
            .ok_or(SolanaMafiaError::MathOverflow)?;
    }

    // 🔧 ИСПРАВЛЕНИЕ: deposit_amount уже включает полную стоимость (base + upgrades)
    // Проверяем минимум базовой стоимости
    if deposit_amount < base_cost {
        return Err(SolanaMafiaError::InsufficientDeposit.into());
    }
    
    // Валидация что deposit_amount соответствует ожидаемой цене
    let total_upgrade_cost: u64 = upgrade_costs.iter().take(target_level as usize).sum();
    let expected_total_cost = base_cost
        .checked_add(total_upgrade_cost)
        .ok_or(SolanaMafiaError::MathOverflow)?;
    
    // Логируем для отладки
    msg!("💰 Price validation: deposit={}, base={}, upgrades={}, expected={}", 
         deposit_amount, base_cost, total_upgrade_cost, expected_total_cost);

    // 🏪 Проверяем стоимость слота
    let slot_cost = slot.get_slot_cost(deposit_amount);
    // 🔧 ИСПРАВЛЕНО: slot_cost - невозвратный донат, НЕ входит в стоимость бизнеса
    let _business_value = deposit_amount; // Только депозит, БЕЗ slot_cost

    // Распределение платежей
    let team_fee = deposit_amount
        .checked_mul(20)
        .ok_or(SolanaMafiaError::MathOverflow)?
        .checked_div(100)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    let treasury_amount = deposit_amount
        .checked_sub(team_fee)
        .ok_or(SolanaMafiaError::InsufficientDeposit)?;
        
    let total_team_fee = team_fee.checked_add(slot_cost)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // Переводы
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.treasury_wallet.to_account_info(),
            },
        ),
        total_team_fee,
    )?;

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.treasury_pda.to_account_info(),
            },
        ),
        treasury_amount,
    )?;

    // 🆕 Создать бизнес с апгрейдами
    let business = if target_level > 0 {
        Business::create_upgraded(
            business_enum,
            base_cost,
            target_level,
            upgrade_costs,
            clock.unix_timestamp,
        )?
    } else {
        Business::new(business_enum, base_cost, clock.unix_timestamp)
    };

    // Оплатить слот и поместить бизнес
    let actual_slot_cost = player.pay_slot_if_needed(slot_index as usize, deposit_amount)?;
    player.place_business_in_slot(slot_index as usize, business)?;

    // 🔧 ИСПРАВЛЕНО: Учитываем только депозит (slot_cost = невозвратный донат)
    player.total_invested = player.total_invested
        .checked_add(deposit_amount)
        .ok_or(SolanaMafiaError::MathOverflow)?;

    // Обновить игровую статистику
    game_state.add_investment(deposit_amount);
    game_state.add_treasury_collection(team_fee);
    game_state.add_business();

    if player.first_business_time == 0 {
        player.first_business_time = Player::timestamp_to_u32(clock.unix_timestamp);
    }

    // Эмитируем событие с правильным уровнем
    emit!(crate::BusinessCreatedInSlot {
        player: ctx.accounts.owner.key(),
        slot_index,
        business_type,
        level: business.upgrade_level, // 🆕 ИСПРАВЛЕНИЕ: передаем реальный уровень!
        base_cost: deposit_amount, // Только депозит (возвратная часть)
        slot_cost: actual_slot_cost,
        total_paid: deposit_amount.checked_add(slot_cost).unwrap_or(deposit_amount),
        daily_rate: business.daily_rate, // Уже включает апгрейды
        created_at: clock.unix_timestamp,
    });

    msg!("🚀 Business created with level {} (target: {})", business.upgrade_level, target_level);
    Ok(())
}