import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Donos } from "../target/types/donos";

describe("donos", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Donos as Program<Donos>;

  it("Is initialized!", async () => {
    // Add your test here.
    
    const tx = await program.methods
      .initializeJar()
      .accounts({})
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
