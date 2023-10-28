use crate::state::{Currency, InvestmentDao};
use anchor_lang::prelude::*;

pub fn get_treasury_owning_program<'a>(
    investment_dao: &Account<InvestmentDao>,
    token_program: &'a Pubkey,
    program_id: &'a Pubkey,
) -> &'a Pubkey {
    match investment_dao.currency {
        Currency::Sol => program_id,
        Currency::Spl => token_program,
    }
}
