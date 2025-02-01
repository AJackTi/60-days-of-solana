import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import {
  InitializationInsecure,
} from '../target/types/initialization_insecure';

describe("initialization_insecure", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .InitializationInsecure as Program<InitializationInsecure>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
