use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
    Discriminator,
};
use anchor_spl::token::{InitializeAccount, Token, TokenAccount};

use crate::{
    constants::INVESTMENT_DAO_TREASURY_SEED,
    errors::InvestmentDaoError,
    state::{Currency, InvestmentDao, Proposal, ProposalState, ProposalType, WithdrawalData},
};

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account()]
    //no checks as anyone should be able to execute succeded proposal
    pub payer: Signer<'info>,
    #[account()]
    pub proposal: Account<'info, Proposal>,
    #[account()]
    ///CHECK:seeds checekd
    pub dao_treasury: UncheckedAccount<'info>,
    #[account()]
    pub investment_dao: Account<'info, InvestmentDao>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn execute_proposal<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ExecuteProposal<'info>>,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    let investment_dao = &ctx.accounts.investment_dao;

    require!(
        proposal.dao == investment_dao.key(),
        InvestmentDaoError::InvalidProposalData
    );

    require!(
        proposal.proposal_state == ProposalState::Succeded,
        InvestmentDaoError::InvalidProposalState
    );

    proposal.proposal_state = ProposalState::Executed;

    match proposal.proposal_type {
        ProposalType::Investing => {}
        ProposalType::Withdrawal => {
            let withdrawal_data = next_account_info(remaining_accounts)?;

            let withdrawal_treasury = next_account_info(remaining_accounts)?;

            proposal.check_withdrwal_data_seeds(
                withdrawal_data.clone(),
                &proposal.key(),
                ctx.program_id,
            )?;

            create_account(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    CreateAccount {
                        from: ctx.accounts.payer.to_account_info(),
                        to: withdrawal_data.to_account_info(),
                    },
                ),
                Rent::default().minimum_balance(8 + WithdrawalData::INIT_SPACE),
                8 + WithdrawalData::INIT_SPACE as u64,
                ctx.program_id,
            )?;

            let w_data = WithdrawalData {
                amount: proposal.withdraw_amount.unwrap(),
                currency: investment_dao.denominated_currency,
                proposal: proposal.key(),
            };
            let mut data: Vec<u8> = vec![];
            data.extend_from_slice(&WithdrawalData::discriminator());
            data.extend_from_slice(&w_data.try_to_vec().unwrap());

            withdrawal_data.data.borrow_mut().copy_from_slice(&data);

            let bump = InvestmentDao::check_treasury_seeds(
                &ctx.accounts.dao_treasury,
                &investment_dao.key(),
                investment_dao.denominated_currency,
                ctx.accounts.token_program.key,
            )?;

            match investment_dao.currency {
                Currency::Sol => {
                    anchor_lang::system_program::transfer(
                        CpiContext::new_with_signer(
                            ctx.accounts.system_program.to_account_info(),
                            anchor_lang::system_program::Transfer {
                                from: ctx.accounts.dao_treasury.to_account_info(),
                                to: withdrawal_treasury.to_account_info(),
                            },
                            &[&[]],
                        ),
                        proposal.withdraw_amount.unwrap(),
                    )?;
                }
                Currency::Spl => {
                    let withdrawal_mint = next_account_info(remaining_accounts)?;

                    create_account(
                        CpiContext::new(
                            ctx.accounts.system_program.to_account_info(),
                            CreateAccount {
                                from: ctx.accounts.payer.to_account_info(),
                                to: withdrawal_treasury.to_account_info(),
                            },
                        ),
                        Rent::default().minimum_balance(TokenAccount::LEN),
                        TokenAccount::LEN as u64,
                        ctx.accounts.token_program.key,
                    )?;

                    anchor_spl::token::initialize_account(CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        InitializeAccount {
                            account: withdrawal_treasury.to_account_info(),
                            authority: withdrawal_treasury.to_account_info(),
                            mint: withdrawal_mint.to_account_info(),
                            rent: ctx.accounts.rent.to_account_info(),
                        },
                    ))?;

                    anchor_spl::token::transfer(
                        CpiContext::new_with_signer(
                            ctx.accounts.token_program.to_account_info(),
                            anchor_spl::token::Transfer {
                                authority: ctx.accounts.dao_treasury.to_account_info(),
                                from: ctx.accounts.dao_treasury.to_account_info(),
                                to: withdrawal_treasury.to_account_info(),
                            },
                            &[&[
                                INVESTMENT_DAO_TREASURY_SEED,
                                investment_dao.key().as_ref(),
                                investment_dao.denominated_currency.as_ref(),
                                &[bump],
                            ]],
                        ),
                        proposal.withdraw_amount.unwrap(),
                    )?;
                }
            }
        }
    }

    Ok(())
}