import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { use } from "chai";
import { Hello } from "../target/types/hello";
import * as spl from '@solana/spl-token';


describe("Hello", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.Hello as Program<Hello>;

  it("Is initialized!", async () => {
    const mintkeypair = anchor.web3.Keypair.generate();
    const user = provider.wallet;

     await program.rpc.initializeMint(user.publicKey , 100,{
       accounts: {
          mintAccount: mintkeypair.publicKey,
          author: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          splProgram: spl.TOKEN_PROGRAM_ID
        },
        //signers:[user.publicKey]
      });
     // let mintaccount = await program.account.mintAccount.fetch(mintkeypair);
      //console.log(mintaccount);


    //const tx = await program.methods.initialize().rpc();
  //  console.log("Your transaction signature", tx);
  });
});

