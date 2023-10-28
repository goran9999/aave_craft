use anchor_lang::prelude::*;
mod state;
use state::*;
mod instructions;
use instructions::*;
mod constants;
declare_id!("BfWxKoznHmSzpGg75mUKq32nmys8tmADqSQoQjgFQRd3");

#[program]
pub mod aave_craft {
    use super::*;

    pub fn create_investment_dao(
        ctx: Context<CreateInvestmentDao>,
        name: String,
        governance_config: Governance,
    ) -> Result<()> {
        instructions::create_investment_dao(ctx, name, governance_config)
    }
}
