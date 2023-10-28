use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct InvestmentDao {
    pub authority: Pubkey,
    #[max_len(20)]
    pub name: String,
    pub investors_count: u32,
    pub total_deposits_count: u32,
    pub currency: Currency,
    pub governance_config: Governance,
    pub denominated_currency: Pubkey,
    pub proposals_count: u32,
}

#[derive(InitSpace, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Governance {
    pub voting_quorum: u8,
    pub max_voting_time: i64,
}

#[derive(InitSpace, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Currency {
    Sol,
    Spl,
}

#[derive(InitSpace, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum InvestorState {
    Invited,
    Accepted,
    Rejected,
}

#[account]
#[derive(InitSpace)]
pub struct InvestorData {
    pub address: Pubkey,
    pub state: InvestorState,
    pub joined_at: i64,
    pub invited_at: i64,
}

#[derive(InitSpace, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum InvitationAction {
    Accept,
    Reject,
}
