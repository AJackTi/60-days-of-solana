import fs from 'fs';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

let idl = JSON.parse(fs.readFileSync("target/idl/day_5.json", "utf-8"));

describe("day_5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programID = "EGmeRcY1BTCttuvK1CktC3KKUQwEvKTqrWa2y1HovUBU";
  const program = new Program(idl, programID, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
