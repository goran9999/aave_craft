use anchor_lang::prelude::*;

#[error_code]
pub enum InvestmentDaoError {
    #[msg("Invalid DAO authority")]
    InvalidDaoAuthority,
    #[msg("Invalid invitation stauts")]
    InvalidInvitationStatus,
    #[msg("Invalid investor wallet")]
    InvalidInvestorWallet,
}
