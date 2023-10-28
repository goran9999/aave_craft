import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { AaveCraft } from "../target/types/aave_craft";
import { DaoAction, ProposalType, VoteOption } from "./constants";
import {
  getAccountSolBalance,
  getActionLog,
  getKeypair,
  getLog,
  sendAndConfirmTransaction,
} from "./helpers";
import { Dao } from "./models/dao";
import { Proposal } from "./models/proposal";

describe("aave_craft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AaveCraft as Program<AaveCraft>;

  const connection = new Connection("http://localhost:8899", "confirmed");

  it("tests solana path!", async () => {
    const authority = await getKeypair(connection);

    const dao = new Dao(authority, program, 20, 51);

    const createDaoIx = await dao.createDao();

    try {
      getActionLog(`Creating dao`);
      await sendAndConfirmTransaction([createDaoIx], connection, [authority]);

      getLog(`Created DAO with name: ${dao.name}`);
      getLog(`DAO authority: ${dao.authority.publicKey.toString()}`);
    } catch (error) {
      console.log(error);
    }

    const daoMember1 = await getKeypair(connection);
    const daoMember2 = await getKeypair(connection);

    try {
      getActionLog("Inviting 2 DAO members");
      const ix1 = await dao.inviteDaoMember(daoMember1.publicKey);
      const ix2 = await dao.inviteDaoMember(daoMember2.publicKey);

      await sendAndConfirmTransaction([ix1, ix2], connection, [authority]);
      getLog(`Invited DAO member: ${daoMember1.publicKey}`);
      getLog(`Invited DAO member: ${daoMember2.publicKey}`);
    } catch (error) {
      console.log(error);
    }

    try {
      getActionLog("Accepting dao memberships");
      const ix1 = await dao.acceptOrRejectDaoMembership(
        daoMember1,
        DaoAction.Accept
      );
      const ix2 = await dao.acceptOrRejectDaoMembership(
        daoMember2,
        DaoAction.Accept
      );
      await sendAndConfirmTransaction([ix1], connection, [daoMember1]);
      await sendAndConfirmTransaction([ix2], connection, [daoMember2]);

      getLog(
        `Investor ${daoMember1.publicKey.toString()} accepted memberhip in ${
          dao.name
        }`
      );
      getLog(
        `Investor ${daoMember2.publicKey.toString()} accepted memberhip in ${
          dao.name
        }`
      );
    } catch (error) {
      console.log(error);
    }

    try {
      getActionLog(`Depositing to DAO treasury`);
      const ix1 = await dao.depositToDao(
        2 * LAMPORTS_PER_SOL,
        daoMember1.publicKey
      );
      const ix2 = await dao.depositToDao(
        3 * LAMPORTS_PER_SOL,
        daoMember2.publicKey
      );

      const ix3 = await dao.depositToDao(
        1 * LAMPORTS_PER_SOL,
        authority.publicKey
      );
      await sendAndConfirmTransaction([ix1], connection, [daoMember1]);
      await sendAndConfirmTransaction([ix2], connection, [daoMember2]);
      await sendAndConfirmTransaction([ix3], connection, [authority]);

      const fr1 = await dao.getFinancialRecord(daoMember1.publicKey);
      const fr2 = await dao.getFinancialRecord(daoMember2.publicKey);
      const fr3 = await dao.getFinancialRecord(authority.publicKey);

      const treasurySolBalance = await dao.getTreasurySolBalance(connection);
      getLog(
        `Investor ${daoMember1.publicKey} invested ${
          fr1.totalDepositAmount.toNumber() / LAMPORTS_PER_SOL
        } SOL at timestamp: ${fr1.lastDepositAt.toNumber()} and has ${(
          (fr1.totalDepositAmount.toNumber() /
            (treasurySolBalance * LAMPORTS_PER_SOL)) *
          100
        ).toFixed(2)} stake %`
      );

      getLog(
        `Investor ${daoMember2.publicKey} invested ${
          fr2.totalDepositAmount.toNumber() / LAMPORTS_PER_SOL
        } SOL at timestamp: ${fr2.lastDepositAt.toNumber()}  and has ${(
          (fr2.totalDepositAmount.toNumber() /
            (treasurySolBalance * LAMPORTS_PER_SOL)) *
          100
        ).toFixed(2)} stake %`
      );

      getLog(
        `Authority ${authority.publicKey} invested ${
          fr3.totalDepositAmount.toNumber() / LAMPORTS_PER_SOL
        } SOL at timestamp: ${fr3.lastDepositAt.toNumber()}  and has ${(
          (fr3.totalDepositAmount.toNumber() /
            (treasurySolBalance * LAMPORTS_PER_SOL)) *
          100
        ).toFixed(2)} stake %`
      );

      getLog(`Treasury SOL balance after deposits: ${treasurySolBalance}`);
    } catch (error) {
      console.log(error);
    }

    const proposal1 = new Proposal(
      dao,
      program,
      "SOL withdrawal",
      "Withdrawing potion of deposited sol",
      ProposalType.Withdrawal
    );
    try {
      getActionLog(`Creating withdrawal proposal`);

      const ix = await proposal1.createWithdrawalProposal(3 * LAMPORTS_PER_SOL);

      await sendAndConfirmTransaction([ix], connection, [authority]);
      const createdProposal = await proposal1.getProposal();

      getLog(
        `Created withdrawal proposal with name: ${
          createdProposal.name
        }, voting threshold: ${createdProposal.voteThreshold} votes and type: ${
          Object.keys(createdProposal.proposalType)[0]
        }`
      );
      getLog(
        `Withdrawing ${
          createdProposal.withdrawAmount.toNumber() / LAMPORTS_PER_SOL
        } SOLs through proposal`
      );
    } catch (error) {
      console.log(error);
    }

    try {
      getActionLog(`Casting votes`);
      const ix = await proposal1.castVote(VoteOption.Yes, authority.publicKey);
      await sendAndConfirmTransaction([ix], connection, [authority]);

      const proposalData = await proposal1.getProposal();
      getLog(
        `Voted with authority, proposal has ${
          proposalData.yesVotesCount
        } and state ${Object.keys(proposalData.proposalState)[0]}`
      );
      const ix2 = await proposal1.castVote(
        VoteOption.Yes,
        daoMember2.publicKey
      );
      await sendAndConfirmTransaction([ix2], connection, [daoMember2]);
      const updatedProposalData = await proposal1.getProposal();
      getLog(
        `Voted with authority, proposal has ${
          updatedProposalData.yesVotesCount
        } and state ${Object.keys(updatedProposalData.proposalState)[0]}`
      );
    } catch (error) {
      console.log(error);
    }

    try {
      getActionLog(`Executing proposal`);
      const ix = await proposal1.executeProposal(daoMember1.publicKey);
      await sendAndConfirmTransaction([ix], connection, [daoMember1]);
      const proposal = await proposal1.getProposal();

      getLog(
        `Executed proposal, with final state ${
          Object.keys(proposal.proposalState)[0]
        }`
      );
      const treasurySolBalance = await dao.getTreasurySolBalance(connection);
      getLog(
        `Treasury balance after executing proposal ${treasurySolBalance} SOL`
      );
    } catch (error) {
      console.log(error);
    }

    try {
      getActionLog(`Withdrawing funds`);
      const authBalanceBefore = await getAccountSolBalance(
        authority.publicKey,
        connection
      );
      getLog(`Authority balance before withdrawing: ${authBalanceBefore}`);
      const ix = await proposal1.withdrawMineFunds(authority.publicKey);
      await sendAndConfirmTransaction([ix], connection, [authority]);
      const authBalanceAfter = await getAccountSolBalance(
        authority.publicKey,
        connection
      );
      const fr1 = await dao.getFinancialRecord(authority.publicKey);
      getLog(`Authority balance after withdrawing: ${authBalanceAfter}`);
      const proposalData = await proposal1.getProposal();
      const totalFinPower = await dao.getTotalFinancialPower();
      getLog(
        `Authority earned ${
          authBalanceAfter - authBalanceBefore
        } SOL because his ownership was ${(
          fr1.totalDepositAmount.toNumber() / totalFinPower.toNumber()
        ).toFixed(4)}%, out of withdrawable ${
          proposalData.withdrawAmount.toNumber() / LAMPORTS_PER_SOL
        }`
      );
    } catch (error) {
      console.log(error);
    }
  });
});
