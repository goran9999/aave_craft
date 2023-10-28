use anchor_lang::prelude::*;
mod state;
use state::*;
mod instructions;
use instructions::*;
mod constants;
mod errors;
mod utils;
declare_id!("BfWxKoznHmSzpGg75mUKq32nmys8tmADqSQoQjgFQRd3");

#[program]
pub mod aave_craft {
    use super::*;

    pub fn create_investment_dao<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CreateInvestmentDao<'info>>,
        name: String,
        governance_config: Governance,
    ) -> Result<()> {
        instructions::create_investment_dao(ctx, name, governance_config)
    }

    pub fn invite_dao_investor(ctx: Context<InviteDaoInvestor>) -> Result<()> {
        instructions::invite_dao_investor(ctx)
    }

    pub fn accept_or_reject_dao_invitation(
        ctx: Context<AcceptDaoInvitation>,
        action: InvitationAction,
    ) -> Result<()> {
        instructions::accept_or_reject_dao_invitation(ctx, action)
    }

    pub fn deposit_funds<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DepositFunds<'info>>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_funds(ctx, amount)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_type: ProposalType,
        name: String,
        description: String,
        withdraw_amount: Option<u64>,
        vesting_config: Option<VestingConfig>,
    ) -> Result<()> {
        instructions::create_proposal(
            ctx,
            proposal_type,
            name,
            description,
            withdraw_amount,
            vesting_config,
        )
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote_option: VoteOption) -> Result<()> {
        instructions::cast_vote(ctx, vote_option)
    }
}
