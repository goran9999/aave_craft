use std::ops::{Div, Mul};

use anchor_lang::prelude::*;

use super::{InvestmentDao, VestingConfig};

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub dao: Pubkey,
    pub authority: Pubkey,
    #[max_len(20)]
    pub name: String,
    #[max_len(50)]
    pub description: String,
    pub proposal_type: ProposalType,
    pub withdraw_amount: Option<u64>,
    pub vesting_config: Option<VestingConfig>,
    pub proposal_state: ProposalState,
    pub vote_threshold: u64,
    pub voting_ends_at: i64,
    pub yes_votes_count: u32,
    pub no_votes_count: u32,
    pub created_at: i64,
}

impl Proposal {
    pub fn calculate_voting_treshold(investment_dao: &Account<InvestmentDao>) -> u64 {
        let voting_quorum = investment_dao.governance_config.voting_quorum;
        let max_voter_weight = investment_dao.total_deposited;

        let voting_threshold: f32 = (voting_quorum as f32)
            .div(100_f32)
            .mul(max_voter_weight as f32);

        voting_threshold as u64
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum ProposalState {
    Voting,
    Succeded,
    Defeated,
    Executed,
    Canceled,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum VoteOption {
    Yes,
    No,
}

#[account]
#[derive(InitSpace)]

pub struct VoteRecord {
    pub authority: Pubkey,
    pub proposal: Pubkey,
    pub vote_at: i64,
    pub vote_option: VoteOption,
    pub voter_weight: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum ProposalType {
    Investing,
    Withdrawal,
}
