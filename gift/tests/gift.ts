import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Gift } from "../target/types/gift";

describe("gift", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Gift as Program<Gift>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
