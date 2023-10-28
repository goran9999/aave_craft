import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  AccountMeta,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { AaveCraft } from "../../target/types/aave_craft";
import {
  DAO_PROPOSAL_SEED,
  INVESTMENT_DAO_SEED,
  ProposalType,
  VESTING_SEED,
  VoteOption,
  WITHDRWAL_SEED,
} from "../constants";
import { Dao } from "./dao";

export class Proposal {
  program: Program<AaveCraft>;
  dao: Dao;
  name: string;
  description: string;
  proposalAddress: PublicKey;
  proposalType: ProposalType;
  proposalIndex: number;
  constructor(
    dao: Dao,
    program: Program<AaveCraft>,
    name: string,
    description: string,
    proposalType: ProposalType
  ) {
    this.dao = dao;
    this.program = program;
    this.name = name;
    this.description = description;

    this.proposalType = proposalType;
  }

  async createWithdrawalProposal(withdrawAmount: number) {
    const { proposalAddress, proposalIndex } = await this.getNewProposalPda();
    this.proposalAddress = proposalAddress;
    this.proposalIndex = proposalIndex;
    const ix = await this.program.methods
      .createProposal(
        { withdrawal: {} },
        this.name,
        this.description,
        new BN(withdrawAmount),
        null
      )
      .accounts({
        authority: this.dao.authority.publicKey,
        investmentDao: this.dao.getDaoPda(),
        investorData: this.dao.getInvestorDataAddress(
          this.dao.authority.publicKey
        ),
        systemProgram: SystemProgram.programId,
        proposal: this.proposalAddress,
      })
      .instruction();

    return ix;
  }

  async getNewProposalPda() {
    const daoAddress = this.dao.getDaoPda();
    const daoAccount = await this.program.account.investmentDao.fetch(
      daoAddress
    );

    const countBuffer = Buffer.alloc(4);
    countBuffer.writeUint32LE(daoAccount.proposalsCount);

    const [proposalAddress] = PublicKey.findProgramAddressSync(
      [DAO_PROPOSAL_SEED, daoAddress.toBuffer(), countBuffer],
      this.program.programId
    );

    return { proposalAddress, proposalIndex: daoAccount.proposalsCount + 1 };
  }

  getWithdrawalDataAddress() {
    const [withdrawalData] = PublicKey.findProgramAddressSync(
      [WITHDRWAL_SEED, this.proposalAddress.toBuffer()],
      this.program.programId
    );

    const [withdrawalTreasury] = PublicKey.findProgramAddressSync(
      [WITHDRWAL_SEED, withdrawalData.toBuffer()],
      this.program.programId
    );

    return [withdrawalData, withdrawalTreasury];
  }

  async createInvestingProposal(
    cliffAmount: number,
    totalAmount: number,
    receiver: PublicKey,
    amountPerPeriod: number,
    period: number
  ) {
    const { proposalAddress, proposalIndex } = await this.getNewProposalPda();
    this.proposalAddress = proposalAddress;
    this.proposalIndex = proposalIndex;
    const ix = await this.program.methods
      .createProposal({ investing: {} }, this.name, this.description, null, {
        cliff: new BN(cliffAmount),
        authority: receiver,
        totalAmount: new BN(totalAmount),
        amountPerPeriod: new BN(amountPerPeriod),
        period: new BN(period),
      })
      .accounts({
        authority: this.dao.authority.publicKey,
        investmentDao: this.dao.getDaoPda(),
        investorData: this.dao.getInvestorDataAddress(
          this.dao.authority.publicKey
        ),
        systemProgram: SystemProgram.programId,
        proposal: proposalAddress,
      })
      .instruction();

    return ix;
  }

  getWithdrawalRecordAddress(wallet: PublicKey) {
    const [withdrawalData] = this.getWithdrawalDataAddress();

    const [withdrawalRecord] = PublicKey.findProgramAddressSync(
      [WITHDRWAL_SEED, withdrawalData.toBuffer(), wallet.toBuffer()],
      this.program.programId
    );

    return withdrawalRecord;
  }

  async withdrawMineFunds(authority: PublicKey) {
    const daoAddress = this.dao.getDaoPda();
    const investorData = this.dao.getInvestorDataAddress(authority);

    const [withdrawalData, withdrawalTreasury] =
      this.getWithdrawalDataAddress();

    const ix = await this.program.methods
      .withdrawFunds()
      .accounts({
        authority: authority,
        investmentDao: daoAddress,
        proposal: this.proposalAddress,
        investorData,
        withdrawalData,
        withdrawalTreasury,
        withdrawalRecord: this.getWithdrawalRecordAddress(authority),
        investorFinancialRecord:
          this.dao.getInvestorFinancialRecord(investorData),
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    return ix;
  }

  getVoteRecordAddress(authority: PublicKey) {
    const [voteRecordAddress] = PublicKey.findProgramAddressSync(
      [
        INVESTMENT_DAO_SEED,
        this.proposalAddress.toBuffer(),
        authority.toBuffer(),
      ],
      this.program.programId
    );

    return voteRecordAddress;
  }

  async castVote(voteOption: VoteOption, authority: PublicKey) {
    const dao = this.dao.getDaoPda();
    const investorData = this.dao.getInvestorDataAddress(authority);
    const investorFr = this.dao.getInvestorFinancialRecord(investorData);
    const ix = await this.program.methods
      .castVote(voteOption === VoteOption.No ? { no: {} } : { yes: {} })
      .accounts({
        investmentDao: dao,
        investor: authority,
        investorData,
        investorFinancialRecord: investorFr,
        proposal: this.proposalAddress,
        systemProgram: SystemProgram.programId,
        voteRecord: this.getVoteRecordAddress(authority),
      })
      .instruction();

    return ix;
  }

  getVestingDataAddresses() {
    const [vestingData] = PublicKey.findProgramAddressSync(
      [VESTING_SEED, this.proposalAddress.toBuffer()],
      this.program.programId
    );

    const [vestingTreasury] = PublicKey.findProgramAddressSync(
      [VESTING_SEED, vestingData.toBuffer()],
      this.program.programId
    );

    return [vestingData, vestingTreasury];
  }

  async executeProposal(authority: PublicKey) {
    const dao = this.dao.getDaoPda();
    const daoTreasury = this.dao.getDaoTreasuryAddress();

    const remainingAccounts: AccountMeta[] = [];

    if (this.proposalType === ProposalType.Withdrawal) {
      const [withdrawalData, withdrawalTreasury] =
        this.getWithdrawalDataAddress();

      remainingAccounts.push(
        {
          isSigner: false,
          isWritable: true,
          pubkey: withdrawalData,
        },
        {
          isSigner: false,
          isWritable: true,
          pubkey: withdrawalTreasury,
        }
      );
    } else {
      const [vestingData, vestingTreasury] = this.getVestingDataAddresses();
      remainingAccounts.push(
        {
          isSigner: false,
          isWritable: true,
          pubkey: vestingData,
        },
        {
          isSigner: false,
          isWritable: true,
          pubkey: vestingTreasury,
        }
      );
    }

    const ix = await this.program.methods
      .executeProposal()
      .accounts({
        payer: authority,
        proposal: this.proposalAddress,
        rent: SYSVAR_RENT_PUBKEY,
        daoTreasury,
        investmentDao: dao,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .remainingAccounts(remainingAccounts)
      .instruction();

    return ix;
  }

  async getProposal() {
    const proposal = await this.program.account.proposal.fetch(
      this.proposalAddress
    );

    return proposal;
  }
}
