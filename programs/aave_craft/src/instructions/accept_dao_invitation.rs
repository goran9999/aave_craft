use anchor_lang::prelude::*;

use crate::{
    constants::INVESTMENT_DAO_SEED,
    errors::InvestmentDaoError,
    state::{InvestmentDao, InvestorData, InvestorState, InvitationAction},
};

#[derive(Accounts)]
pub struct AcceptDaoInvitation<'info> {
    #[account()]
    ///CHECK: checked with has_one constraint
    pub authority: UncheckedAccount<'info>,
    #[account()]
    ///CHECK: checked in seeds of investor_data
    pub investor: UncheckedAccount<'info>,
    #[account(has_one=authority)]
    pub investment_dao: Box<Account<'info, InvestmentDao>>,
    #[account(seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),investor.key().as_ref()],bump)]
    pub investor_data: Account<'info, InvestorData>,
}

pub fn accept_or_reject_dao_invitation(
    ctx: Context<AcceptDaoInvitation>,
    action: InvitationAction,
) -> Result<()> {
    let investor_data = &mut ctx.accounts.investor_data;
    let investment_dao = &mut ctx.accounts.investment_dao;

    require!(
        investor_data.address == ctx.accounts.investor.key(),
        InvestmentDaoError::InvalidInvestorWallet
    );

    match action {
        InvitationAction::Accept => {
            investor_data.joined_at = Clock::get().unwrap().unix_timestamp;
            investor_data.state = InvestorState::Accepted;
            investment_dao.investors_count = investment_dao.investors_count.checked_add(1).unwrap();
        }
        InvitationAction::Reject => {
            investor_data.close(ctx.accounts.authority.to_account_info())?;
        }
    }

    Ok(())
}
