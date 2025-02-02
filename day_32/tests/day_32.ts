import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { Day32 } from '../target/types/day_32';

describe("day_32", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day32 as Program<Day32>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [storage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods.initialize().accounts({ storage: storage }).rpc();

    let storageStruct = await program.account.storage.fetch(storage);

    console.log("The value of x is:", storageStruct.x.toString());
    console.log("Storage account address:", storage.toBase58());
  });

  it("read storage", async () => {
    const seeds = [];
    const [storage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    const tx = await program.methods
      .readOtherData()
      .accounts({ otherData: storage.toBase58() })
      .rpc();
  });
});
