import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { OtherProgram } from '../target/types/other_program';

describe("other_program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OtherProgram as Program<OtherProgram>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [trueOrFalse, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    console.log("address: ", program.programId.toBase58());

    await program.methods
      .initialize()
      .accounts({ trueOrFalse: trueOrFalse })
      .rpc();
    await program.methods
      .setbool(true)
      .accounts({ trueOrFalse: trueOrFalse })
      .rpc();

    let myStorageStruct = await program.account.trueOrFalse.fetch(trueOrFalse);
    console.log("The value of flag is:", myStorageStruct.flag.toString());
  });
});
