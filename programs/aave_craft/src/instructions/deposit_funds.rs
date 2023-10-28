use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    constants::{INVESTMENT_DAO_SEED, INVESTMENT_DAO_TREASURY_SEED},
    errors::InvestmentDaoError,
    state::{
        investment_dao::{Currency, InvestorFinancialRecord, InvestorState},
        InvestmentDao, InvestorData,
    },
};

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub investor: Signer<'info>,
    #[account(mut)]
    pub investment_dao: Account<'info, InvestmentDao>,
    #[account(mut,seeds=[INVESTMENT_DAO_TREASURY_SEED,investment_dao.key().as_ref(),investment_dao.denominated_currency.as_ref()],bump)]
    ///CHECK: seeds checked
    pub dao_treasury: UncheckedAccount<'info>,
    #[account(init_if_needed,seeds=[INVESTMENT_DAO_SEED,investor_data.key().as_ref()],bump,space=8+InvestorFinancialRecord::INIT_SPACE,payer=investor)]
    pub investor_financial_record: Account<'info, InvestorFinancialRecord>,
    #[account(mut,seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),investor.key().as_ref()],bump)]
    pub investor_data: Account<'info, InvestorData>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_funds<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, DepositFunds<'info>>,
    amount: u64,
) -> Result<()> {
    let investment_dao = &mut ctx.accounts.investment_dao;

    investment_dao.total_deposited = investment_dao.total_deposited.checked_add(amount).unwrap();
    investment_dao.total_deposits_count =
        investment_dao.total_deposits_count.checked_add(1).unwrap();

    let financial_record = &mut ctx.accounts.investor_financial_record;

    //check that only dao member can deposit funds
    require!(
        ctx.accounts.investor_data.state == InvestorState::Accepted,
        InvestmentDaoError::NotPartOfDao
    );

    financial_record.authority = ctx.accounts.investor.key();
    financial_record.last_deposit_at = Clock::get().unwrap().unix_timestamp;

    financial_record.total_deposit_amount = financial_record
        .total_deposit_amount
        .checked_add(amount)
        .unwrap();

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    match investment_dao.currency {
        Currency::Sol => {
            anchor_lang::system_program::transfer(
                CpiContext::new(
                    ctx.accounts.investor.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.investor.to_account_info(),
                        to: ctx.accounts.dao_treasury.to_account_info(),
                    },
                ),
                amount,
            )?;
        }
        Currency::Spl => {
            let raw_investor_token = next_account_info(remaining_accounts)?;

            let investor_token = Account::<TokenAccount>::try_from(raw_investor_token)?;

            require!(
                investor_token.mint == investment_dao.denominated_currency,
                InvestmentDaoError::InvalidDepositMint
            );

            //We deserialize dao_treasury as additional check to confirm it is token account
            Account::<TokenAccount>::try_from(&ctx.accounts.dao_treasury.to_account_info())?;

            anchor_spl::token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        authority: ctx.accounts.investor.to_account_info(),
                        from: raw_investor_token.to_account_info(),
                        to: ctx.accounts.dao_treasury.to_account_info(),
                    },
                ),
                amount,
            )?;
        }
    }
    Ok(())
}
