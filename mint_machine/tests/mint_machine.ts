import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MintMachine } from "../target/types/mint_machine";

describe("mint_machine", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MintMachine as Program<MintMachine>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
