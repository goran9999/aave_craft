use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, InitSpace, Clone)]
pub struct VestingConfig {
    pub cliff: u64,
    pub period: u64,
    pub total_amount: u64,
    pub authority: Pubkey,
    pub amount_per_period: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Vesting {
    pub config: VestingConfig,
    pub created_at: i64,
    pub proposal: Pubkey,
    pub total_claimed: u64,
    pub last_claim_at: u64,
}
