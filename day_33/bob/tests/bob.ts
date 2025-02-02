import { expect } from 'chai';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Alice } from '../target/types/alice';

describe("CPI from Alice to Bob", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const bobProgram = anchor.workspace.Bob as Program<Bob>;
  const aliceProgram = anchor.workspace.Alice as Program<Alice>;
  const dataAccountKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await bobProgram.methods
      .initialize()
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([dataAccountKeypair])
      .rpc();
  });

  it("Can add numbers!", async () => {
    const tx = await aliceProgram.methods
      .askBobToAdd(new anchor.BN(4), new anchor.BN(2))
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        bobProgram: bobProgram.programId,
      })
      .rpc();
  });

  it("Can assert value in Bob's data account equals 4 + 2", async () => {
    const BobAccountValue = await bobProgram.account.bobData.fetch(
      dataAccountKeypair.publicKey
    );

    expect(BobAccountValue.result.toNumber()).to.equal(6);
  });

  it("askBobToAdd version 2", async () => {
    const tx = await aliceProgram.methods
      .askBobToAdd2(new anchor.BN(4), new anchor.BN(4))
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        bobProgram: bobProgram.programId,
      })
      .rpc();

    const BobAccountValue = await bobProgram.account.bobData.fetch(
      dataAccountKeypair.publicKey
    );

    expect(BobAccountValue.result.toNumber()).to.equal(8);
  });
});
