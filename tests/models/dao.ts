import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { v4 } from "uuid";
import { AaveCraft } from "../../target/types/aave_craft";
import {
  DaoAction,
  INVESTMENT_DAO_SEED,
  INVESTMENT_DAO_TREASURY_SEED,
} from "../constants";

export class Dao {
  name: string;
  authority: Keypair;
  program: Program<AaveCraft>;
  maxVotingTime: number;
  votingQuorum: number;
  daoMembers: PublicKey[];
  constructor(
    authority: Keypair,
    program: Program<AaveCraft>,
    maxVotingTime: number,
    votingQuorum: number
  ) {
    this.name = `DAO:` + v4().slice(0, 8);
    this.authority = authority;
    this.program = program;
    this.maxVotingTime = maxVotingTime;
    this.votingQuorum = votingQuorum;
    this.daoMembers = [authority.publicKey];
  }

  async createDao() {
    const daoAddress = this.getDaoPda();
    const ix = await this.program.methods
      .createInvestmentDao(this.name, {
        maxVotingTime: new BN(this.maxVotingTime),
        votingQuorum: this.votingQuorum,
      })
      .accounts({
        investmentDao: daoAddress,
        daoAuthority: this.authority.publicKey,
        denominatedCurrency: SystemProgram.programId,
        investorData: this.getInvestorDataAddress(this.authority.publicKey),
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    return ix;
  }

  getDaoPda() {
    const [daoAddress] = PublicKey.findProgramAddressSync(
      [INVESTMENT_DAO_SEED, Buffer.from(this.name)],
      this.program.programId
    );

    return daoAddress;
  }

  async inviteDaoMember(memberAddress: PublicKey) {
    const ix = await this.program.methods
      .inviteDaoInvestor()
      .accounts({
        authority: this.authority.publicKey,
        daoInvestor: this.getInvestorDataAddress(memberAddress),
        investmentDao: this.getDaoPda(),
        systemProgram: SystemProgram.programId,
        invitedInvestor: memberAddress,
      })
      .instruction();

    return ix;
  }

  getInvestorDataAddress(wallet: PublicKey) {
    const [address] = PublicKey.findProgramAddressSync(
      [INVESTMENT_DAO_SEED, this.getDaoPda().toBuffer(), wallet.toBuffer()],
      this.program.programId
    );

    return address;
  }

  getInvestorFinancialRecord(investorDataAddress: PublicKey) {
    const [address] = PublicKey.findProgramAddressSync(
      [INVESTMENT_DAO_SEED, investorDataAddress.toBuffer()],
      this.program.programId
    );

    return address;
  }

  async acceptOrRejectDaoMembership(invitedMember: Keypair, action: DaoAction) {
    const ix = await this.program.methods
      .acceptOrRejectDaoInvitation(
        action === DaoAction.Accept ? { accept: {} } : { reject: {} }
      )
      .accounts({
        investor: invitedMember.publicKey,
        investmentDao: this.getDaoPda(),
        authority: this.authority.publicKey,
        investorData: this.getInvestorDataAddress(invitedMember.publicKey),
      })
      .instruction();

    return ix;
  }

  addDaoMember(member: PublicKey) {
    this.daoMembers.push(member);
  }

  getDaoTreasuryAddress() {
    const [treasury] = PublicKey.findProgramAddressSync(
      [
        INVESTMENT_DAO_TREASURY_SEED,
        this.getDaoPda().toBuffer(),
        SystemProgram.programId.toBuffer(),
      ],
      this.program.programId
    );

    return treasury;
  }

  async depositToDao(amount: number, investor: PublicKey) {
    const dao = this.getDaoPda();
    const investorDataAddress = this.getInvestorDataAddress(investor);
    const ix = await this.program.methods
      .depositFunds(new BN(amount))
      .accounts({
        investmentDao: dao,
        investor,
        investorData: investorDataAddress,
        daoTreasury: this.getDaoTreasuryAddress(),
        systemProgram: SystemProgram.programId,
        investorFinancialRecord:
          this.getInvestorFinancialRecord(investorDataAddress),
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    return ix;
  }

  async getFinancialRecord(wallet: PublicKey) {
    const investorData = this.getInvestorDataAddress(wallet);
    const address = this.getInvestorFinancialRecord(investorData);

    const account = await this.program.account.investorFinancialRecord.fetch(
      address
    );

    return account;
  }

  async getTreasurySolBalance(connection: Connection) {
    const treasury = this.getDaoTreasuryAddress();

    const balance = await connection.getBalance(treasury);

    return balance / LAMPORTS_PER_SOL;
  }
}
