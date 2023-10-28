use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct InvestmentDao {
    pub authority: Pubkey,
    #[max_len(20)]
    pub name: String,
    pub investors_count: u32,
    pub total_deposits_count: u32,
    pub governance_config: Governance,
    pub denominated_currency: Pubkey,
}

#[derive(InitSpace, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Governance {
    pub voting_quorum: u8,
    pub max_voting_time: i64,
}
