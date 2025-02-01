import chai from 'chai';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day24 } from '../target/types/day_24';

const expect = chai.expect;

// this airdrops sol to an address
async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(publicKey, amount);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    signature: tx,
  });
}

describe("day_24", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day24 as Program<Day24>;

  it("update value", async () => {
    /*
    const newKeypair = anchor.web3.Keypair.generate();
    await airdropSol(newKeypair.publicKey, 1e9); // 1 SOL

    let seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({
        myStorage,
        signer: newKeypair.publicKey, // ** THIS MUST BE THE EXPLICITLY SPECIFIED **
      })
      .signers([newKeypair])
      .rpc();
    */
    // OR
    /*
      await program.methods
        .initialize()
        .accounts({
          myStorage,
        })
        .rpc();
    */

    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();

    const airdropAliceTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        alice.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropAliceTx);

    const airdropBobTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        bob.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropBobTx);

    let seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    // ALICE INITIALIZE ACCOUNT
    await program.methods
      .initialize()
      .accounts({
        myStorage,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    // BOB WRITE TO ACCOUNT
    await program.methods
      .updateValue(new anchor.BN(3))
      .accounts({
        myStorage,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    let value = await program.account.myStorage.fetch(myStorage);
    console.log(`value stored is: ${value.x}`);
  });

  it("points", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();

    const airdropAliceTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        alice.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropAliceTx);

    const airdropBobTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        bob.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropBobTx);

    let seedsAlice = [alice.publicKey.toBytes()];
    const [playerAlice, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsAlice,
      program.programId
    );

    let seedsBob = [bob.publicKey.toBytes()];
    const [playerBob, _bump2] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsBob,
      program.programId
    );

    // Alice and Bob initialize their accounts
    await program.methods
      .initialize2()
      .accounts({
        player: playerAlice,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .initialize2()
      .accounts({
        player: playerBob,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    // Alice transfers 5 points to Bob. Note that this is a u32
    // so we don't need a BigNum
    await program.methods
      .transferPoints(5)
      .accounts({
        from: playerAlice,
        to: playerBob,
        authority: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    console.log(
      `Alice has ${
        (await program.account.player.fetch(playerAlice)).points
      } points`
    );
    console.log(
      `Bob has ${(await program.account.player.fetch(playerBob)).points} points`
    );
  });

  it("mallory steal points", async () => {
    const alice = anchor.web3.Keypair.generate();
    const bob = anchor.web3.Keypair.generate();
    const mallory = anchor.web3.Keypair.generate();

    const airdropAliceTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        alice.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropAliceTx);

    const airdropBobTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        bob.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropBobTx);

    const airdropMalloryTx = await anchor
      .getProvider()
      .connection.requestAirdrop(
        mallory.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
    await confirmTransaction(airdropMalloryTx);

    let seedsAlice = [alice.publicKey.toBytes()];
    const [playerAlice, _bumpA] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsAlice,
      program.programId
    );

    let seedsBob = [bob.publicKey.toBytes()];
    const [playerBob, _bumpB] = anchor.web3.PublicKey.findProgramAddressSync(
      seedsBob,
      program.programId
    );

    let seedsMallory = [mallory.publicKey.toBytes()];
    const [playerMallory, _bumpC] =
      anchor.web3.PublicKey.findProgramAddressSync(
        seedsMallory,
        program.programId
      );

    // Alice, Bob, and Mallory initialize their accounts
    await program.methods
      .initialize2()
      .accounts({
        player: playerAlice,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .initialize2()
      .accounts({
        player: playerBob,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    await program.methods
      .initialize2()
      .accounts({
        player: playerMallory,
        signer: mallory.publicKey,
      })
      .signers([mallory])
      .rpc();

    // This will throw an error
    const stealFromAlice = program.methods
      .transferPoints(5)
      .accounts({
        from: playerAlice,
        to: playerMallory,
        authority: mallory.publicKey,
      })
      .signers([mallory])
      .rpc();

    await expect(stealFromAlice).to.be.Throw;

    // This will also throw an error
    const stealFromBob = program.methods
      .transferPoints(5)
      .accounts({
        from: playerBob,
        to: playerMallory,
        authority: mallory.publicKey,
      })
      .signers([mallory])
      .rpc();

    await expect(stealFromBob).to.be.Throw;

    console.log(
      `Alice has ${
        (await program.account.player.fetch(playerAlice)).points
      } points`
    );
    console.log(
      `Bob has ${(await program.account.player.fetch(playerBob)).points} points`
    );
    console.log(
      `Mallory has ${
        (await program.account.player.fetch(playerMallory)).points
      } points`
    );
  });
});
