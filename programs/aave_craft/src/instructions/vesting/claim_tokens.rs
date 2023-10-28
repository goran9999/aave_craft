use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    constants::VESTING_SEED,
    errors::InvestmentDaoError,
    state::{Currency, InvestmentDao, Vesting},
};

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account()]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    #[account()]
    pub investment_dao: Account<'info, InvestmentDao>,
    #[account(mut,seeds=[VESTING_SEED,vesting.key().as_ref()],bump)]
    ///CHECK: deserialized in ix
    pub vesting_treasury: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn claim_tokens<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ClaimTokens<'info>>,
) -> Result<()> {
    let vesting = &mut ctx.accounts.vesting;

    let current_timestamp = Clock::get().unwrap().unix_timestamp;

    require!(
        vesting.remaining_amount > 0,
        InvestmentDaoError::AllTokensClaimed
    );

    let start_unlock = vesting
        .created_at
        .checked_add(vesting.config.cliff as i64)
        .unwrap();

    require!(
        vesting.config.authority == ctx.accounts.payer.key(),
        InvestmentDaoError::InvalidVestingAuthority
    );

    //dont allow claiming until cliff is over
    require!(
        current_timestamp > start_unlock,
        InvestmentDaoError::VestingNotStarted
    );

    let time_passed_since_cliff = current_timestamp.checked_sub(start_unlock).unwrap();

    let passed_slots = time_passed_since_cliff
        .checked_div(vesting.config.period)
        .unwrap() as u64;

    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    if passed_slots == 0 {
        return Err(error!(InvestmentDaoError::NoClaimableTokens));
    }

    let claimable_amount = passed_slots
        .checked_mul(vesting.config.amount_per_period)
        .unwrap();

    match ctx.accounts.investment_dao.currency {
        Currency::Sol => {
            anchor_lang::system_program::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.vesting_treasury.to_account_info(),
                        to: ctx.accounts.payer.to_account_info(),
                    },
                    &[&[
                        VESTING_SEED,
                        vesting.key().as_ref(),
                        &[*ctx.bumps.get(&"vesting_treasury".to_string()).unwrap()],
                    ]],
                ),
                claimable_amount,
            )?;
        }
        Currency::Spl => {
            let raw_payer_token = next_account_info(remaining_accounts)?;

            let deserialized_token = Account::<TokenAccount>::try_from(raw_payer_token)?;

            anchor_spl::token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    anchor_spl::token::Transfer {
                        authority: ctx.accounts.vesting_treasury.to_account_info(),
                        from: ctx.accounts.vesting_treasury.to_account_info(),
                        to: deserialized_token.to_account_info(),
                    },
                    &[&[
                        VESTING_SEED,
                        vesting.key().as_ref(),
                        &[*ctx.bumps.get(&"vesting_treasury".to_string()).unwrap()],
                    ]],
                ),
                claimable_amount,
            )?;
        }
    }

    vesting.total_claimed = vesting.total_claimed.checked_add(claimable_amount).unwrap();
    vesting.last_claim_at = Clock::get().unwrap().unix_timestamp;
    vesting.remaining_amount = vesting
        .remaining_amount
        .checked_sub(claimable_amount)
        .unwrap();

    Ok(())
}
