# Aave craft task - Investment DAO platform

## Introduction

This is project build as craft task during Aave company selection process. Project is Solana smart contract written in Rust programming language,
using Anchor framework. Main idea is enabling group investments through decentralized autonomous organizations, also known as DAOs.
Anyone can create DAO, invite investors, who can deposit selected denominated currency (SOL or SPL tokens) and get ownership in related DAO.
System also implements proposals, where each DAO member can create proposal to invest funds or withdraw deposited funds. Voting power is
calculated based on deposited amount of tokens, such as ownership. Claiming invested tokens is realized through concept of **vesting**,
where proposal creator defines cliff period, unlocking period and amount of tokens that is being unlocked.

## Running tests

In order to run tests of this projects (writtend in ts-mocha), host OS needs to be MacOS or Linux. Instruction for running tests is **bash script.sh**
from root folder of project. This command will install all required depenedencies such as:

- rust and cargo v1.70.0
- solana v1.14.18
- anchor and avm v0.28.0

At the end, given command runs **anchor test** command that will run tests on local network validator. During running of tests, logs can be seeing explaining
account state changes such as SOL balances.

## Instructions

- [Create investment DAO](#create-investment-dao)
- [Invite DAO investor](#invite-dao-investor)
- [Accept DAO invitation](#accept-dao-invitation)
- [Deposit funds](#deposit-funds)
- [Create proposal](#create-proposal)
- [Cast vote](#cast-vote)
- [Execute proposal](#execute-proposal)
- [Withdraw funds](#withdraw-funds)
- [Claim tokens](#claim-tokens)

### Create investment DAO

Instruction where any wallet can create DAO with specific configuration, such as **voting_quorum** and **max_voting_time** on each proposal
created inside DAO. Apart from that, in this instruction is defined **name** of DAO such as **denominated_currency**, which defines whether it
is SOL or SPL token.

### Invite DAO investor

In this instruction, DAO authority (wallet who created it) have permissions to invite new members to be part of this DAO, specifying their
wallet addresses.

### Accept DAO invitation

Wallets that were invited by DAO authorities can trigger this instruction in order to confirm membership inside related DAO. In this instruction,
timestamp of joining specific DAO is stored, such as state filed, that is set to **Accepted**.

### Deposit funds

Depositing tokens to DAO treasury is done through this instruction. Only constraint is that wallet that is depositing, needs to be part of given
DAO. Apart from that, user is forced to deposit only denominated currency, defined during DAO creation. After depositing, based on amount, ownership
of related user is stored on account, which is used afterwards for calculating withdrawable amount such as voting rights on proposals.

### Create proposal

Proposal can be created by any DAO member, with no authorization checks. In this system, 2 types of proposals are allowed: **withdrawal proposals** and
**investment proposals**. Max voting time and voting quourum of proposal are used from DAO configuration, from parameters defined during DAO creation.

#### Withdrawal proposal

This is type of proposal that can be created and executed when DAO members want to withdraw tokens that are currently deposited to DAO treasury wallet.
During proposal creation, withdraw amount is specified, and if proposal goes to **succeded** state, members would be able to withdraw potion of withdrawal
amount, calculated based on their total DAO treasury ownership. Both SOL and SPL withdrawals are supported in current system.

#### Investment proposals

Investment type of proposal is used when DAO members want to invest deposited tokens to some external entity (wallet or treasury), by specified conditions.
This system supports vesting of tokens, so after proposal execution, all tokens are not immediately transferred to destination, but under specific vesting
rules defined during proposal creation. This type of proposal requires adding following parameters:

- cliff - amount of time that need to pass so unlocking can start
- authority - destination wallet that will have permissions of claiming vested tokens
- amount_per_period - amount of tokens that is unlocked per specified period
- period - duration of single periods
- total_amount - total amount of tokens that is unlocked in given vesting

### Cast vote

Casting vote on proposal is performed in this instruction, with constraints that proposal needs to be in voting state, such as user who is casting vote,
needs to be part of given DAO organization. Voting power is calculated directly by amount of deposited tokens, where percentage is derived from **total_deposited**
parameter from dao, and **total_deposit_amount** value from financial record account, presenting deposit of DAO member. If number of votes for specific option, after
casting vote, will outweight voting quorum, proposal state is automatically changed to **Succeded** / **Defeated**, based on choosen vote option.

### Execute proposal

This instruction can be triggered by any wallet, with constraint that proposal needs to be in Succeded state, in order to be executed, which prevents execution of
failed proposals. Based on proposal type,specific set of actions is performed in this proposal that will execute desired action and change states.

### Withdraw funds

Instruction used after successfull withdrawal proposal, to withdraw potion of withdrawal amount specified through proposal. Each DAO member needs to trigger this
instruction in order to get his amount of tokens, calculated by total ownership inside DAO.

### Claim tokens

After investment proposal is succeded, wallet defined as **authority** inside investment proposal is allowed to trigger this instruction and claim amount of vested tokens.
Given instruction has multiple constraints checking that specified cliff has passed, such as calculations for defining claimable amount of tokens based on passed amount of time.
After all tokens are closed, account is closed and rent SOLs are retreived to authority.
