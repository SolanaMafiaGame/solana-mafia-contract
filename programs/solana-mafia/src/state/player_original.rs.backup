// state/player.rs - ОБНОВЛЕННАЯ СТРУКТУРА С СЛОТАМИ
use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::business::Business;
use crate::error::SolanaMafiaError;

/// Структура слота для бизнеса
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct BusinessSlot {
    pub slot_type: SlotType,           // Тип слота (Basic/Premium/VIP/Legendary)
    pub is_unlocked: bool,             // Разблокирован ли слот
    pub business: Option<Business>,    // Бизнес в слоте (если есть)
    pub unlock_cost_paid: u64,         // Сколько заплачено за разблокировку
}

impl BusinessSlot {
    pub const SIZE: usize = 
        1 +  // slot_type (SlotType)
        1 +  // is_unlocked
        1 + Business::SIZE + // business Option<Business>
        8;   // unlock_cost_paid

    /// Создать новый базовый слот
    pub fn new_basic() -> Self {
        Self {
            slot_type: SlotType::Basic,
            is_unlocked: true, // Базовые слоты разблокированы сразу
            business: None,
            unlock_cost_paid: 0,
        }
    }

    /// Создать заблокированный слот
    pub fn new_locked() -> Self {
        Self {
            slot_type: SlotType::Basic,
            is_unlocked: false,
            business: None,
            unlock_cost_paid: 0,
        }
    }

    /// Создать премиум слот
    pub fn new_premium(slot_type: SlotType, cost: u64) -> Self {
        Self {
            slot_type,
            is_unlocked: true,
            business: None,
            unlock_cost_paid: cost,
        }
    }

    /// Разблокировать слот
    pub fn unlock(&mut self, cost: u64) -> Result<()> {
        if self.is_unlocked {
            return Err(SolanaMafiaError::SlotAlreadyUnlocked.into());
        }
        
        self.is_unlocked = true;
        self.unlock_cost_paid = cost;
        Ok(())
    }

    /// Поместить бизнес в слот
    pub fn place_business(&mut self, business: Business) -> Result<()> {
        if !self.is_unlocked {
            return Err(SolanaMafiaError::SlotNotUnlocked.into());
        }
        
        if self.business.is_some() {
            return Err(SolanaMafiaError::SlotOccupied.into());
        }
        
        self.business = Some(business);
        Ok(())
    }

    /// Удалить бизнес из слота
    pub fn remove_business(&mut self) -> Option<Business> {
        self.business.take()
    }

    /// Получить бонус доходности слота
    pub fn get_yield_bonus(&self) -> u16 {
        match self.slot_type {
            SlotType::Basic => 0,
            SlotType::Premium => PREMIUM_SLOT_YIELD_BONUSES[0],  // +1.5%
            SlotType::VIP => PREMIUM_SLOT_YIELD_BONUSES[1],      // +3%
            SlotType::Legendary => PREMIUM_SLOT_YIELD_BONUSES[2], // +5%
        }
    }

    /// Получить скидку на комиссию продажи
    pub fn get_sell_fee_discount(&self) -> u8 {
        match self.slot_type {
            SlotType::Basic => 0,
            SlotType::Premium => PREMIUM_SLOT_SELL_FEE_DISCOUNTS[0],  // 0%
            SlotType::VIP => PREMIUM_SLOT_SELL_FEE_DISCOUNTS[1],      // -50%
            SlotType::Legendary => PREMIUM_SLOT_SELL_FEE_DISCOUNTS[2], // -100%
        }
    }

    /// Рассчитать доходность бизнеса в слоте с учетом бонусов
    pub fn calculate_business_daily_earnings(&self) -> u64 {
        if let Some(business) = &self.business {
            let base_earnings = business.calculate_daily_earnings();
            let slot_bonus = self.get_yield_bonus();
            
            // Применяем бонус слота
            let bonus_earnings = (base_earnings as u128 * slot_bonus as u128) / 10000;
            base_earnings + bonus_earnings as u64
        } else {
            0
        }
    }
}

#[account]
pub struct Player {
    pub owner: Pubkey,
    
    /// 🆕 НОВАЯ СИСТЕМА СЛОТОВ вместо Vec<Business>
    pub business_slots: Vec<BusinessSlot>,     // Все слоты игрока
    pub unlocked_slots_count: u8,              // Количество разблокированных слотов
    pub premium_slots_count: u8,               // Количество премиум слотов
    
    /// Статистика (обновленная)
    pub total_invested: u64,                   // Общие инвестиции (базовая стоимость + улучшения)
    pub total_upgrade_spent: u64,              // 🆕 Потрачено на улучшения отдельно
    pub total_slot_spent: u64,                 // 🆕 Потрачено на слоты отдельно
    pub total_earned: u64,
    pub pending_earnings: u64,
    pub pending_referral_earnings: u64,
    
    /// Настройки
    pub has_paid_entry: bool,
    pub created_at: i64,
    pub next_earnings_time: i64,
    pub earnings_interval: i64,
    pub first_business_time: i64,
    pub last_auto_update: i64,
    pub bump: u8,
}

impl Player {
    /// 🔒 УВЕЛИЧЕННЫЙ размер с учетом слотов
    pub const SIZE: usize = 8 + // discriminator
        32 + // owner
        4 + (BusinessSlot::SIZE * 20) + // business_slots (максимум 20 слотов)
        1 + // unlocked_slots_count
        1 + // premium_slots_count
        8 + // total_invested
        8 + // total_upgrade_spent
        8 + // total_slot_spent
        8 + // total_earned
        8 + // pending_earnings
        8 + // pending_referral_earnings
        1 + // has_paid_entry
        8 + // created_at
        8 + // next_earnings_time
        8 + // earnings_interval
        8 + // first_business_time
        8 + // last_auto_update
        1 + // bump
        500; // 🔒 ЗАПАС

    /// Создать нового игрока с базовыми слотами
    pub fn new(owner: Pubkey, _referrer: Option<Pubkey>, current_time: i64, bump: u8) -> Self {
        let mut slots = Vec::new();
        
        // Добавляем 3 базовых разблокированных слота
        for _ in 0..BASE_BUSINESS_SLOTS {
            slots.push(BusinessSlot::new_basic());
        }
        
        // Добавляем 3 заблокированных слота
        for _ in 0..(MAX_REGULAR_SLOTS - BASE_BUSINESS_SLOTS) {
            slots.push(BusinessSlot::new_locked());
        }
        
        Self {
            owner,
            business_slots: slots,
            unlocked_slots_count: BASE_BUSINESS_SLOTS,
            premium_slots_count: 0,
            total_invested: 0,
            total_upgrade_spent: 0,
            total_slot_spent: 0,
            total_earned: 0,
            pending_earnings: 0,
            pending_referral_earnings: 0,
            has_paid_entry: false,
            created_at: current_time,
            next_earnings_time: 0,
            earnings_interval: 86_400,
            first_business_time: 0,
            last_auto_update: current_time,
            bump,
        }
    }

    /// 🆕 Найти свободный разблокированный слот
    pub fn find_free_slot(&self) -> Option<usize> {
        self.business_slots.iter().position(|slot| 
            slot.is_unlocked && slot.business.is_none()
        )
    }

    /// 🆕 Разблокировать следующий слот
    pub fn unlock_next_slot(&mut self, cost: u64) -> Result<usize> {
        let slot_index = self.business_slots.iter().position(|slot| !slot.is_unlocked)
            .ok_or(SolanaMafiaError::NoMoreSlotsToUnlock)?;
        
        self.business_slots[slot_index].unlock(cost)?;
        self.unlocked_slots_count += 1;
        self.total_slot_spent = self.total_slot_spent
            .checked_add(cost)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        Ok(slot_index)
    }

    /// 🆕 Добавить премиум слот
    pub fn add_premium_slot(&mut self, slot_type: SlotType, cost: u64) -> Result<usize> {
        let slot = BusinessSlot::new_premium(slot_type, cost);
        self.business_slots.push(slot);
        self.premium_slots_count += 1;
        self.total_slot_spent = self.total_slot_spent
            .checked_add(cost)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        Ok(self.business_slots.len() - 1)
    }

    /// 🆕 Поместить бизнес в слот
    pub fn place_business_in_slot(&mut self, slot_index: usize, business: Business) -> Result<()> {
        if slot_index >= self.business_slots.len() {
            return Err(SolanaMafiaError::InvalidSlotIndex.into());
        }
        
        self.business_slots[slot_index].place_business(business)?;
        
        // Обновляем статистику
        if let Some(business) = &self.business_slots[slot_index].business {
            self.total_invested = self.total_invested
                .checked_add(business.total_invested_amount)
                .ok_or(SolanaMafiaError::MathOverflow)?;
        }
        
        Ok(())
    }

    /// 🆕 Улучшить бизнес в слоте
    pub fn upgrade_business_in_slot(&mut self, slot_index: usize, upgrade_cost: u64, new_business: Business) -> Result<()> {
        if slot_index >= self.business_slots.len() {
            return Err(SolanaMafiaError::InvalidSlotIndex.into());
        }
        
        let slot = &mut self.business_slots[slot_index];
        
        // Удаляем старый бизнес (NFT будет сожжена в инструкции)
        slot.remove_business();
        
        // Размещаем новый улучшенный бизнес
        slot.place_business(new_business)?;
        
        // Обновляем статистику
        self.total_upgrade_spent = self.total_upgrade_spent
            .checked_add(upgrade_cost)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        self.total_invested = self.total_invested
            .checked_add(upgrade_cost)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        Ok(())
    }

    /// 🆕 Продать бизнес из слота
    pub fn sell_business_from_slot(&mut self, slot_index: usize) -> Result<(Business, u64)> {
        if slot_index >= self.business_slots.len() {
            return Err(SolanaMafiaError::InvalidSlotIndex.into());
        }
        
        let slot = &mut self.business_slots[slot_index];
        let business = slot.remove_business()
            .ok_or(SolanaMafiaError::BusinessNotFound)?;
        
        // Рассчитываем возврат с учетом бонусов слота
        let sell_fee_discount = slot.get_sell_fee_discount();
        
        Ok((business, sell_fee_discount as u64))
    }

    /// 🆕 Обновить earnings для всех бизнесов в слотах
    pub fn update_all_slot_earnings(&mut self, current_time: i64) -> Result<()> {
        let mut total_new_earnings = 0u64;
        
        for slot in &mut self.business_slots {
            // Get slot bonus before any borrowing
            let slot_bonus = slot.get_yield_bonus();
            
            if let Some(business) = &mut slot.business {
                if business.is_active {
                    let pending = business.calculate_pending_earnings(current_time);
                    
                    let bonus_earnings = (pending as u128 * slot_bonus as u128) / 10000;
                    let total_earnings = pending + bonus_earnings as u64;
                    
                    total_new_earnings = total_new_earnings
                        .checked_add(total_earnings)
                        .ok_or(SolanaMafiaError::MathOverflow)?;
                    
                    business.update_last_claim(current_time)?;
                }
            }
        }
        
        self.pending_earnings = self.pending_earnings
            .checked_add(total_new_earnings)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        Ok(())
    }

    /// 🆕 Получить общую стоимость всех инвестиций (для расчета при продаже)
    pub fn get_total_slot_invested_amount(&self, slot_index: usize) -> u64 {
        if let Some(slot) = self.business_slots.get(slot_index) {
            if let Some(business) = &slot.business {
                return business.total_invested_amount;
            }
        }
        0
    }

    /// 🆕 Получить количество активных бизнесов
    pub fn get_active_businesses_count(&self) -> u8 {
        self.business_slots.iter()
            .filter(|slot| {
                if let Some(business) = &slot.business {
                    business.is_active
                } else {
                    false
                }
            })
            .count() as u8
    }

    /// 🆕 Получить список всех бизнесов (для совместимости)
    pub fn get_all_businesses(&self) -> Vec<&Business> {
        self.business_slots.iter()
            .filter_map(|slot| slot.business.as_ref())
            .collect()
    }

    /// Остальные методы без изменений...
    pub fn get_claimable_amount(&self) -> Result<u64> {
        self.pending_earnings
            .checked_add(self.pending_referral_earnings)
            .ok_or(SolanaMafiaError::MathOverflow.into())
    }

    pub fn claim_all_earnings(&mut self) -> Result<()> {
        let total_claimed = self.pending_earnings
            .checked_add(self.pending_referral_earnings)
            .ok_or(SolanaMafiaError::MathOverflow)?;
            
        self.total_earned = self.total_earned
            .checked_add(total_claimed)
            .ok_or(SolanaMafiaError::MathOverflow)?;
            
        self.pending_earnings = 0;
        self.pending_referral_earnings = 0;
        Ok(())
    }

    pub fn add_referral_bonus(&mut self, amount: u64) -> Result<()> {
        self.pending_referral_earnings = self.pending_referral_earnings
            .checked_add(amount)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        Ok(())
    }

    /// Set earnings schedule for distributed updates
    pub fn set_earnings_schedule(&mut self, current_time: i64, player_seed: u64) -> Result<()> {
        self.first_business_time = current_time;
        
        // Distribute earnings times across 24 hours to prevent RPC overload
        let offset_seconds = (player_seed % 86400) as i64;
        self.next_earnings_time = current_time + offset_seconds;
        
        Ok(())
    }

    /// Check if earnings update is due
    pub fn is_earnings_due(&self, current_time: i64) -> bool {
        current_time >= self.next_earnings_time
    }

    /// Auto-update earnings and return amount added
    pub fn auto_update_earnings(&mut self, current_time: i64) -> Result<u64> {
        if !self.is_earnings_due(current_time) {
            return Ok(0);
        }

        let mut total_new_earnings = 0u64;
        
        for slot in &mut self.business_slots {
            // Get slot bonus before any borrowing
            let slot_bonus = slot.get_yield_bonus();
            
            if let Some(business) = &mut slot.business {
                if business.is_active {
                    let pending = business.calculate_pending_earnings(current_time);
                    
                    let bonus_earnings = (pending as u128 * slot_bonus as u128) / 10000;
                    let total_earnings = pending + bonus_earnings as u64;
                    
                    total_new_earnings = total_new_earnings
                        .checked_add(total_earnings)
                        .ok_or(SolanaMafiaError::MathOverflow)?;
                    
                    business.update_last_claim(current_time)?;
                }
            }
        }
        
        self.pending_earnings = self.pending_earnings
            .checked_add(total_new_earnings)
            .ok_or(SolanaMafiaError::MathOverflow)?;
        
        // Set next update time
        self.next_earnings_time = current_time + self.earnings_interval;
        self.last_auto_update = current_time;
        
        Ok(total_new_earnings)
    }

    /// Update pending earnings manually
    pub fn update_pending_earnings(&mut self, current_time: i64) -> Result<()> {
        self.auto_update_earnings(current_time)?;
        Ok(())
    }

    /// Health check for player data
    pub fn health_check(&self, current_time: i64) -> Result<()> {
        // Basic validation checks
        if self.total_invested == 0 && self.get_active_businesses_count() > 0 {
            return Err(SolanaMafiaError::MathOverflow.into());
        }
        
        if current_time < self.created_at {
            return Err(SolanaMafiaError::MathOverflow.into());
        }
        
        Ok(())
    }

    /// Get frontend data with filtered businesses
    pub fn get_frontend_data_with_filter(&self, current_time: i64, valid_business_indices: &[usize]) -> PlayerFrontendData {
        let active_businesses = valid_business_indices.len() as u8;
        
        let time_to_next_earnings = if self.next_earnings_time > current_time {
            self.next_earnings_time - current_time
        } else {
            0
        };

        PlayerFrontendData {
            wallet: self.owner,
            total_invested: self.total_invested,
            pending_earnings: self.pending_earnings,
            estimated_pending_earnings: self.pending_earnings, // Could calculate estimated
            businesses_count: active_businesses,
            next_earnings_time: self.next_earnings_time,
            time_to_next_earnings,
            active_businesses,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerFrontendData {
    pub wallet: Pubkey,
    pub total_invested: u64,
    pub pending_earnings: u64,
    pub estimated_pending_earnings: u64,
    pub businesses_count: u8,
    pub next_earnings_time: i64,
    pub time_to_next_earnings: i64,
    pub active_businesses: u8,
}