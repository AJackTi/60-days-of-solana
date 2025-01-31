import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day19 } from '../target/types/day_19';

describe("day_19", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day19 as Program<Day19>;

  it("Initialize mapping storage", async () => {
    const key = new anchor.BN(42);
    const value = new anchor.BN(1337);
    const seeds = [key.toArrayLike(Buffer, "le", 8)];

    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods.initialize(key).accounts({ val: valueAccount }).rpc();

    // set the account
    await program.methods.set(key, value).accounts({ val: valueAccount }).rpc();

    // read the account back
    let result = await program.account.val.fetch(valueAccount);

    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );
  });

  it("Initialize nested mapping storage", async () => {
    const key1 = new anchor.BN(42);
    const key2 = new anchor.BN(43);
    const value = new anchor.BN(1337);

    // seeds has two values
    const seeds = [
      key1.toArrayLike(Buffer, "le", 8),
      key2.toArrayLike(Buffer, "le", 8),
    ];
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    // functions now take two keys
    await program.methods
      .initializeNestedMapping(key1, key2)
      .accounts({ val: valueAccount })
      .rpc();

    await program.methods
      .setNestedMapping(key1, key2, value)
      .accounts({ val: valueAccount })
      .rpc();

    // read the account back
    let result = await program.account.val.fetch(valueAccount);
    console.log(
      `the value ${result.value} was stored in ${valueAccount.toBase58()}`
    );
  });

  it("Initialize Map 1 (Single Key)", async () => {
    const whichMap = new anchor.BN(0);
    const key1 = new anchor.BN(42);

    const seeds = [
      whichMap.toArrayLike(Buffer, "le", 8),
      key1.toArrayLike(Buffer, "le", 8),
    ];

    const [val1Account, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initializeMap1(whichMap, key1)
      .accounts({
        val1: val1Account,
      })
      .rpc();
  });

  it("Initialize Map 2 (Two Keys)", async () => {
    const whichMap = new anchor.BN(1);
    const key1 = new anchor.BN(42);
    const key2 = new anchor.BN(100);

    const seeds = [
      whichMap.toArrayLike(Buffer, "le", 8),
      key1.toArrayLike(Buffer, "le", 8),
      key2.toArrayLike(Buffer, "le", 8),
    ];

    const [val2Account, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initializeMap2(whichMap, key1, key2)
      .accounts({
        val2: val2Account,
      })
      .rpc();
  });
});
