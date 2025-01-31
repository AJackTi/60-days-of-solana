import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day21 } from '../target/types/day_21';

describe("day_21", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day21 as Program<Day21>;

  let pubkey = new anchor.web3.PublicKey(
    "AzuiWapU4pttFt7qQLHaiQMcuhzVb2mwEok5LRWgZJZx"
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Tests the balance", async () => {
    const tx = await program.methods
      .readBalance()
      .accounts({ account: pubkey })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
