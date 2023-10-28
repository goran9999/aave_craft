use anchor_lang::prelude::*;

use crate::{
    constants::INVESTMENT_DAO_SEED,
    state::{
        InvestmentDao, InvestorData, InvestorFinancialRecord, Proposal, VoteOption, VoteRecord,
    },
};

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    ///CHECK: seeds checked
    pub investor: UncheckedAccount<'info>,
    #[account()]
    pub investment_dao: Account<'info, InvestmentDao>,
    #[account(seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),investor.key().as_ref()],bump)]
    pub investor_data: Account<'info, InvestorData>,
    #[account(seeds=[INVESTMENT_DAO_SEED,investor_data.key().as_ref()],bump)]
    pub investor_financial_record: Account<'info, InvestorFinancialRecord>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(init,seeds=[INVESTMENT_DAO_SEED,proposal.key().as_ref(),investor.key().as_ref()],bump,space=8+VoteRecord::INIT_SPACE,payer=investor)]
    pub vote_record: Account<'info, VoteRecord>,
    pub system_program: Program<'info, System>,
}

pub fn cast_vote(ctx: Context<CastVote>, vote_option: VoteOption) -> Result<()> {
    let vote_record = &mut ctx.accounts.vote_record;
    let proposal = &mut ctx.accounts.proposal;

    let financial_record = &ctx.accounts.investor_financial_record;

    vote_record.authority = ctx.accounts.investor.key();
    vote_record.vote_at = Clock::get().unwrap().unix_timestamp;
    vote_record.proposal = proposal.key();
    vote_record.voter_weight = financial_record.total_deposit_amount;

    Proposal::try_tip_vote(
        proposal,
        financial_record.total_deposit_amount,
        &vote_option,
    )?;

    vote_record.vote_option = vote_option;
    Ok(())
}
