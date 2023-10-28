export const INVESTMENT_DAO_SEED = Buffer.from("investment_dao");
export const INVESTMENT_DAO_TREASURY_SEED = Buffer.from(
  "investment_dao_treasury"
);
export const DAO_PROPOSAL_SEED = Buffer.from("dao_proposal");
export const WITHDRWAL_SEED = Buffer.from("withdrwal");
export const VESTING_SEED = Buffer.from("vesting");

export enum DaoAction {
  Accept,
  Reject,
}

export enum ProposalType {
  Investing,
  Withdrawal,
}

export enum VoteOption {
  Yes,
  No,
}
