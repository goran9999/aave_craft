use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, InitSpace, Clone)]
pub struct VestingConfig {
    pub cliff: u64,
    pub period: u64,
    pub total_amount: u64,
    pub amount_per_period: u64,
}
