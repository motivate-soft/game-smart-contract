import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Wheel } from "../target/types/wheel";

describe("wheel", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Wheel as Program<Wheel>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
