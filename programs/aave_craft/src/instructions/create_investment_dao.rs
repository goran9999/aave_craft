use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::token::{InitializeAccount, Token, TokenAccount};

use crate::{
    constants::INVESTMENT_DAO_SEED,
    errors::InvestmentDaoError,
    state::{Currency, Governance, InvestmentDao, InvestorData, InvestorState},
};

#[derive(Accounts)]
#[instruction(name:String)]
pub struct CreateInvestmentDao<'info> {
    #[account(mut)]
    pub dao_authority: Signer<'info>,
    #[account(init,seeds=[INVESTMENT_DAO_SEED,name.as_bytes()],bump,space=8+InvestmentDao::INIT_SPACE,payer=dao_authority)]
    pub investment_dao: Box<Account<'info, InvestmentDao>>,
    ///CHECK: checked in ix
    pub denominated_currency: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(init,payer=dao_authority,space=8+InvestorData::INIT_SPACE
        ,seeds=[INVESTMENT_DAO_SEED,investment_dao.key().as_ref(),dao_authority.key().as_ref()],bump)]
    pub investor_data: Account<'info, InvestorData>,
}

pub fn create_investment_dao<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CreateInvestmentDao<'info>>,
    name: String,
    governance_config: Governance,
) -> Result<()> {
    let investment_dao = &mut ctx.accounts.investment_dao;

    let investor_data = &mut ctx.accounts.investor_data;

    investor_data.address = ctx.accounts.dao_authority.key();
    investor_data.created_proposal_count = 0;
    investor_data.invited_at = Clock::get().unwrap().unix_timestamp;
    investor_data.joined_at = Clock::get().unwrap().unix_timestamp;
    investor_data.state = InvestorState::Accepted;

    investment_dao.authority = ctx.accounts.dao_authority.key();
    investment_dao.denominated_currency = ctx.accounts.denominated_currency.key();

    require!(
        governance_config.voting_quorum <= 100,
        InvestmentDaoError::InvalidGovernanceConfig
    );

    investment_dao.governance_config = governance_config;
    let remaining_accounts = &mut ctx.remaining_accounts.iter();
    if ctx.accounts.denominated_currency.key() == Pubkey::default() {
        //If denominated currency is 111..111, we consider dao currency Solana
        investment_dao.currency = Currency::Sol;
    } else {
        investment_dao.currency = Currency::Spl;

        let dao_treasury = next_account_info(remaining_accounts)?;

        InvestmentDao::check_treasury_seeds(
            dao_treasury,
            &investment_dao.key(),
            ctx.accounts.denominated_currency.key(),
            ctx.accounts.token_program.key,
        )?;

        //If currency is token, we need to create treasury token account
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.dao_authority.to_account_info(),
                    to: dao_treasury.to_account_info(),
                },
            ),
            Rent::default().minimum_balance(TokenAccount::LEN),
            TokenAccount::LEN as u64,
            ctx.accounts.token_program.key,
        )?;

        //Authority of token account is token account itself (that way nobody can control it but smart contract logic)
        anchor_spl::token::initialize_account(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeAccount {
                account: dao_treasury.to_account_info(),
                authority: dao_treasury.to_account_info(),
                mint: ctx.accounts.denominated_currency.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ))?;
    }

    investment_dao.investors_count = 0;
    investment_dao.total_deposits_count = 0;
    investment_dao.name = name;
    investment_dao.proposals_count = 0;
    Ok(())
}
