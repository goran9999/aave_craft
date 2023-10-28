use std::ops::{Div, Mul};

use anchor_lang::prelude::*;

use crate::{
    constants::{VESTING_SEED, WITHDRWAL_SEED},
    errors::InvestmentDaoError,
};

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
    pub yes_votes_count: u64,
    pub no_votes_count: u64,
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

    pub fn try_tip_vote(
        proposal: &mut Account<Proposal>,
        voter_weight: u64,
        vote_option: &VoteOption,
    ) -> Result<()> {
        if proposal.proposal_state != ProposalState::Voting {
            return Err(error!(InvestmentDaoError::ProposalNotInVotingState));
        }

        match vote_option {
            VoteOption::No => {
                proposal.no_votes_count =
                    proposal.no_votes_count.checked_add(voter_weight).unwrap();

                //In order to proposal go to defeated,we need threshold + 1 vote
                if proposal.no_votes_count > proposal.vote_threshold + 1 {
                    proposal.proposal_state = ProposalState::Defeated;
                }
            }
            VoteOption::Yes => {
                proposal.yes_votes_count =
                    proposal.yes_votes_count.checked_add(voter_weight).unwrap();

                if proposal.yes_votes_count > proposal.vote_threshold + 1 {
                    proposal.proposal_state = ProposalState::Succeded;
                }
            }
        }

        Ok(())
    }
    pub fn check_withdrwal_data_seeds(
        &self,
        withdrawal_data: AccountInfo,
        proposal_address: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<u8> {
        let (address, bump) =
            Pubkey::find_program_address(&[WITHDRWAL_SEED, proposal_address.as_ref()], program_id);

        require!(
            address == withdrawal_data.key(),
            InvestmentDaoError::InvalidProposalData
        );
        Ok(bump)
    }

    pub fn check_vesting_data_seeds(
        &self,
        vesting_data: AccountInfo,
        proposal_address: &Pubkey,
        program_id: &Pubkey,
    ) -> Result<()> {
        let (address, _) =
            Pubkey::find_program_address(&[VESTING_SEED, proposal_address.as_ref()], program_id);
        require!(
            address == vesting_data.key(),
            InvestmentDaoError::InvalidProposalData
        );
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace, PartialEq)]
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

#[account]
#[derive(InitSpace)]
pub struct WithdrawalData {
    pub proposal: Pubkey,
    pub amount: u64,
    pub currency: Pubkey,
}
