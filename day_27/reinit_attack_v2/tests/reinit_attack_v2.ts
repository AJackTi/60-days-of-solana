import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';

import { ReinitAttackV2 } from '../target/types/reinit_attack_v2';

describe("reinit_attack_v2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ReinitAttackV2 as Program<ReinitAttackV2>;

  it("Is initialized!", async () => {
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [],
      program.programId
    );
    await program.methods.initialize().accounts({ myPda: myPda }).rpc();
    console.log("account initialized after giving to system program");

    await program.methods.erase().accounts({ myPda: myPda }).rpc();

    // ERROR: It will fail because even though the account has no data, it is still owned by the program and has a non-zero lamport balance.
    // await program.methods
    //   .initialize()
    //   .accounts({ myPda: myPda })
    //   .rpc();
  });
});
