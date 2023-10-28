use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{
    constants::INVESTMENT_DAO_SEED,
    state::{Governance, InvestmentDao},
};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct CreateInvestmentDao<'info> {
    #[account(mut)]
    pub dao_authority: Signer<'info>,
    #[account(init,seeds=[INVESTMENT_DAO_SEED,name.as_bytes()],bump,space=8+InvestmentDao::INIT_SPACE,payer=dao_authority)]
    pub investment_dao: Box<Account<'info, InvestmentDao>>,
    pub denominated_currency: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
}

pub fn create_investment_dao(
    ctx: Context<CreateInvestmentDao>,
    name: String,
    governance_config: Governance,
) -> Result<()> {
    let investment_dao = &mut ctx.accounts.investment_dao;

    investment_dao.authority = ctx.accounts.dao_authority.key();
    investment_dao.denominated_currency = ctx.accounts.denominated_currency.key();
    investment_dao.governance_config = governance_config;

    investment_dao.investors_count = 0;
    investment_dao.total_deposits_count = 0;
    investment_dao.name = name;
    Ok(())
}
