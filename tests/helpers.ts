import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";

export async function sendAndConfirmTransaction(
  instructions: TransactionInstruction[],
  connection: Connection,
  signers: Keypair[]
) {
  const blockhashData = await connection.getLatestBlockhash();
  const txMessage = new TransactionMessage({
    instructions,
    payerKey: signers[0].publicKey,
    recentBlockhash: blockhashData.blockhash,
  }).compileToV0Message();
  const versionedTx = new VersionedTransaction(txMessage);

  versionedTx.sign(signers);
  const txSig = await connection.sendRawTransaction(versionedTx.serialize());

  await connection.confirmTransaction({
    signature: txSig,
    blockhash: blockhashData.blockhash,
    lastValidBlockHeight: blockhashData.lastValidBlockHeight,
  });
}

export async function getKeypair(connection: Connection): Promise<Keypair> {
  const keypair = Keypair.generate();

  const txSig = await connection.requestAirdrop(
    keypair.publicKey,
    10 * LAMPORTS_PER_SOL
  );

  await connection.confirmTransaction(txSig);

  return keypair;
}

export const getLog = (log: string) => {
  const length = log.length + 4;
  const border = Array(length).fill("-").join("");

  console.log(border);
  console.log(`| ${log} |`);
  console.log(border);
};

export function getActionLog(action: string) {
  const b = Array(20).fill("#").join("");

  console.log("\n \n");

  console.log(b + ` ${action.toLocaleUpperCase()} ` + b);
}

export async function getAccountSolBalance(
  account: PublicKey,
  connection: Connection
) {
  const balance = await connection.getBalance(account);

  return balance / LAMPORTS_PER_SOL;
}
