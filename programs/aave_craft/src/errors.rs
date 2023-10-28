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
    #[msg("Invalid proposal state")]
    InvalidProposalState,
    #[msg("Invalid proposal data")]
    InvalidProposalData,
    #[msg("Vesting not started yet")]
    VestingNotStarted,
    #[msg("No claimable tokens")]
    NoClaimableTokens,
    #[msg("Invalid vesting authority")]
    InvalidVestingAuthority,
    #[msg("No tokens left to claim")]
    AllTokensClaimed,
}
