use anchor_lang::prelude::*;

use crate::{constants::INVESTMENT_DAO_TREASURY_SEED, errors::InvestmentDaoError};

#[account]
#[derive(InitSpace)]
pub struct InvestmentDao {
    pub authority: Pubkey,
    #[max_len(20)]
    pub name: String,
    pub investors_count: u32,
    pub total_deposits_count: u32,
    pub total_deposited: u64,
    pub currency: Currency,
    pub governance_config: Governance,
    pub denominated_currency: Pubkey,
    pub proposals_count: u32,
}

impl InvestmentDao {
    pub fn check_treasury_seeds<'a, 'c>(
        dao_treasury: &'a AccountInfo<'c>,
        investment_dao_address: &Pubkey,
        denominated_currency: Pubkey,
        program_id: &Pubkey,
    ) -> Result<u8> {
        let (address, bump) = Pubkey::find_program_address(
            &[
                INVESTMENT_DAO_TREASURY_SEED,
                investment_dao_address.as_ref(),
                denominated_currency.as_ref(),
            ],
            program_id,
        );

        require!(
            dao_treasury.key() == address,
            InvestmentDaoError::InvalidTreasuryAddress
        );
        Ok(bump)
    }
}

#[derive(InitSpace, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Governance {
    //In percentages (0-100)
    pub voting_quorum: u8,
    pub max_voting_time: i64,
}

#[derive(InitSpace, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Currency {
    Sol,
    Spl,
}

#[derive(InitSpace, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum InvestorState {
    Invited,
    Accepted,
    Rejected,
}

#[account]
#[derive(InitSpace)]
pub struct InvestorData {
    pub address: Pubkey,
    pub state: InvestorState,
    pub joined_at: i64,
    pub invited_at: i64,
    pub total_deposits_count: u32,
    pub created_proposal_count: u32,
}

#[derive(InitSpace, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum InvitationAction {
    Accept,
    Reject,
}

#[account]
#[derive(InitSpace)]
pub struct InvestorFinancialRecord {
    pub authority: Pubkey,
    pub total_deposit_amount: u64,
    pub last_deposit_at: i64,
    pub total_withdrawn_amount: u64,
}
