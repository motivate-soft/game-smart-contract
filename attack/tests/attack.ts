import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Attack } from "../target/types/attack";

describe("attack", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Attack as Program<Attack>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
