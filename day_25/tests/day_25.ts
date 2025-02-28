import privateKey from '/Users/ajackti/.config/solana/id.json';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day25 } from '../target/types/day_25';

// this airdrops sol to an address
async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(
      publicKey,
      amount * anchor.web3.LAMPORTS_PER_SOL
    );
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(airdropTx) {
  const latestBlockHash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: airdropTx,
  });
}

describe("day_25", () => {
  const deployer = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(privateKey)
  );

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day25 as Program<Day25>;

  it("Is initialized -- PDA version", async () => {
    const seeds = [];
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );
    console.log("the storage account address is", myPda.toBase58());
    const tx = await program.methods
      .initializePda()
      .accounts({ myPda: myPda })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  /*
  it("Writing to keypair account fails", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    var receiverWallet = anchor.web3.Keypair.generate();

    await airdropSol(newKeypair.publicKey, 10);

    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: receiverWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [newKeypair]
    );
    console.log("sent 1 lamport");

    await program.methods
      .initializeKeypairAccount()
      .accounts({
        myKeypairAccount: newKeypair.publicKey,
      })
      .signers([newKeypair])
      .rpc();

    console.log("initialized");
    // try to transfer again, this fails
    var transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: newKeypair.publicKey,
        toPubkey: receiverWallet.publicKey,
        lamports: 1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      transaction,
      [newKeypair]
    );
  });
  */

  it("Console log account owner", async () => {
    console.log(`The program address is ${program.programId}`);
    const newKeypair = anchor.web3.Keypair.generate();
    var receiverWallet = anchor.web3.Keypair.generate();

    // get account owner before initialization
    await airdropSol(newKeypair.publicKey, 10);
    const accountInfoBefore = await anchor
      .getProvider()
      .connection.getAccountInfo(newKeypair.publicKey);
    console.log(
      `initial keypair account is ${accountInfoBefore.owner.toBase58()}`
    );

    await program.methods
      .initializeKeypairAccount()
      .accounts({
        myKeypairAccount: newKeypair.publicKey,
      })
      .signers([newKeypair])
      .rpc();

    // get account owner after initialization
    const accountInfoAfter = await anchor
      .getProvider()
      .connection.getAccountInfo(newKeypair.publicKey);
    console.log(
      `initial keypair account is ${accountInfoAfter.owner.toBase58()}`
    );
  });
});
