use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    constants::{INVESTMENT_DAO_SEED, WITHDRWAL_SEED},
    state::{
        Currency, InvestmentDao, InvestorData, InvestorFinancialRecord, Proposal, WithdrawalData,
        WithdrawalRecord,
    },
};

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub investment_dao: Account<'info, InvestmentDao>,
    #[account(seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),authority.key().as_ref()],bump)]
    pub investor_data: Account<'info, InvestorData>,
    #[account(mut,seeds=[INVESTMENT_DAO_SEED,investor_data.key().as_ref()],bump)]
    pub investor_financial_record: Account<'info, InvestorFinancialRecord>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub withdrawal_data: Account<'info, WithdrawalData>,
    #[account(init,seeds=[WITHDRWAL_SEED,withdrawal_data.key().as_ref(),authority.key().as_ref()],bump
    ,payer=authority,space=8+WithdrawalRecord::INIT_SPACE)]
    pub withdrawal_record: Account<'info, WithdrawalRecord>,
    #[account(mut,seeds=[WITHDRWAL_SEED,withdrawal_data.key().as_ref()],bump)]
    ///CHECK:seeds checked
    pub withdrawal_treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn withdraw_funds<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, WithdrawFunds<'info>>,
) -> Result<()> {
    let withdrawal_record = &mut ctx.accounts.withdrawal_record;

    let withdrawal_data = &mut ctx.accounts.withdrawal_data;
    let financial_record = &mut ctx.accounts.investor_financial_record;

    let withdrawable_amount = withdrawal_data.calculate_withdrawal_amount(
        financial_record.total_deposit_amount,
        ctx.accounts.investment_dao.total_deposited,
    );

    withdrawal_record.amount_withdrawn = withdrawable_amount;
    withdrawal_record.proposal = ctx.accounts.proposal.key();
    withdrawal_record.withdraw_at = Clock::get().unwrap().unix_timestamp;

    financial_record.total_withdrawn_amount = financial_record
        .total_withdrawn_amount
        .checked_add(withdrawable_amount)
        .unwrap();

    match ctx.accounts.investment_dao.currency {
        Currency::Sol => {
            anchor_lang::system_program::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.withdrawal_treasury.to_account_info(),
                        to: ctx.accounts.authority.to_account_info(),
                    },
                    &[&[
                        WITHDRWAL_SEED,
                        ctx.accounts.withdrawal_data.key().as_ref(),
                        &[*ctx.bumps.get(&"withdrawal_treasury".to_string()).unwrap()],
                    ]],
                ),
                withdrawable_amount,
            )?;
        }
        Currency::Spl => {
            let remaining_accounts = &mut ctx.remaining_accounts.iter();
            let raw_payer_token = next_account_info(remaining_accounts)?;

            let payer_token = Account::<TokenAccount>::try_from(raw_payer_token)?;

            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        authority: ctx.accounts.withdrawal_treasury.to_account_info(),
                        from: ctx.accounts.withdrawal_treasury.to_account_info(),
                        to: payer_token.to_account_info(),
                    },
                    &[&[
                        WITHDRWAL_SEED,
                        ctx.accounts.withdrawal_data.key().as_ref(),
                        &[*ctx.bumps.get(&"withdrawal_treasury".to_string()).unwrap()],
                    ]],
                ),
                withdrawable_amount,
            )?;
        }
    }

    Ok(())
}
