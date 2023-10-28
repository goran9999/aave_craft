import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AaveCraft } from "../target/types/aave_craft";

describe("aave_craft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AaveCraft as Program<AaveCraft>;

  it("tests solana path!", async () => {});
});
