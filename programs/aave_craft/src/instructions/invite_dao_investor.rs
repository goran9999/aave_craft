use anchor_lang::prelude::*;

use crate::{
    constants::INVESTMENT_DAO_SEED,
    errors::InvestmentDaoError,
    state::{InvestmentDao, InvestorData, InvestorState},
};

#[derive(Accounts)]
pub struct InviteDaoInvestor<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(has_one=authority)]
    pub investment_dao: Box<Account<'info, InvestmentDao>>,
    #[account()]
    ///CHECK: no need to check,can be any wallet
    pub invited_investor: UncheckedAccount<'info>,
    #[account(init,seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),invited_investor.key().as_ref()],bump,payer=authority,space=8+InvestorData::INIT_SPACE)]
    pub dao_investor: Box<Account<'info, InvestorData>>,
    pub system_program: Program<'info, System>,
}

pub fn invite_dao_investor(ctx: Context<InviteDaoInvestor>) -> Result<()> {
    let dao_investor = &mut ctx.accounts.dao_investor;
    let investment_dao = &ctx.accounts.investment_dao;

    let authority = ctx.accounts.authority.key();

    //Only DAO authority can invite new investors
    require!(
        authority == investment_dao.authority,
        InvestmentDaoError::InvalidDaoAuthority
    );

    dao_investor.address = ctx.accounts.invited_investor.key();
    dao_investor.invited_at = Clock::get().unwrap().unix_timestamp;
    dao_investor.joined_at = 0;
    dao_investor.state = InvestorState::Invited;
    Ok(())
}
