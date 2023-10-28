use anchor_lang::prelude::*;

use crate::{
    constants::{DAO_PROPOSAL_SEED, INVESTMENT_DAO_SEED},
    state::{InvestmentDao, InvestorData, Proposal, ProposalState, ProposalType, VestingConfig},
};

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub investment_dao: Account<'info, InvestmentDao>,
    #[account(seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),authority.key().as_ref()],bump)]
    pub investor_data: Account<'info, InvestorData>,
    #[account(init,seeds=[DAO_PROPOSAL_SEED,investment_dao.key().as_ref(),
    &investment_dao.proposals_count.to_le_bytes()],bump,space=8+Proposal::INIT_SPACE,payer=authority)]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    proposal_type: ProposalType,
    name: String,
    description: String,
    withdraw_amount: Option<u64>,
    vesting_config: Option<VestingConfig>,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    let investment_dao = &mut ctx.accounts.investment_dao;
    let investor_data = &mut ctx.accounts.investor_data;

    investor_data.created_proposal_count =
        investor_data.created_proposal_count.checked_add(1).unwrap();

    investment_dao.proposals_count = investment_dao.proposals_count.checked_add(1).unwrap();

    let voting_ends_at = Clock::get()
        .unwrap()
        .unix_timestamp
        .checked_add(investment_dao.governance_config.max_voting_time)
        .unwrap();

    proposal.voting_ends_at = voting_ends_at;

    proposal.vote_threshold = Proposal::calculate_voting_treshold(investment_dao);

    proposal.created_at = Clock::get().unwrap().unix_timestamp;
    proposal.name = name;
    proposal.description = description;

    proposal.proposal_type = proposal_type.clone();
    proposal.yes_votes_count = 0;
    proposal.no_votes_count = 0;

    proposal.proposal_state = ProposalState::Voting;

    match proposal_type {
        ProposalType::Investing => {
            proposal.vesting_config = vesting_config;
        }
        ProposalType::Withdrawal => {
            proposal.withdraw_amount = withdraw_amount;
        }
    }

    proposal.authority = ctx.accounts.authority.key();
    proposal.dao = ctx.accounts.investment_dao.key();
    Ok(())
}
