use anchor_lang::prelude::*;

#[error_code]
pub enum InvestmentDaoError {
    #[msg("Invalid DAO authority")]
    InvalidDaoAuthority,
    #[msg("Invalid invitation stauts")]
    InvalidInvitationStatus,
    #[msg("Invalid investor wallet")]
    InvalidInvestorWallet,
    #[msg("Invalid treasury address")]
    InvalidTreasuryAddress,
    #[msg("Invalid deposit mint")]
    InvalidDepositMint,
    #[msg("Not part of DAO")]
    NotPartOfDao,
    #[msg("Invalid governance config")]
    InvalidGovernanceConfig,
    #[msg("Proposal not in voting state")]
    ProposalNotInVotingState,
}
