import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { PublicKey } from "@solana/web3.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter2")],
    program.programId
  );

  it("Is initialized!", async () => {
    try {
      const txSig = await program.methods.initialize().rpc();

      const accountData = await program.account.counter.fetch(counterPDA);
      console.log(`Transaction Signature: ${txSig}`);
      console.log(`Count: ${accountData.count}`);
    } catch (error) {
      //If PDA account already created, then we expect an error
      console.log(error);
    }
  });


  it("Increment", async () => {
    //Invoke the increment instruction 
    const transactionSignature = await program.methods.increment().rpc();

    // read to Fetch counter account data
    const accountData = await program.account.counter.fetch(counterPDA);

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);

  });
});




// it("Is initialized!", async () => {
//   // // Add your test here.
//   // const tx = await program.methods.initialize().rpc();
//   // console.log("Your transaction signature", tx);

//   // init instruction
//   // acnhor knows user and system program
//   const transactionSignature = await program.methods
//     .initialize()
//     .accounts({
//       counter: counterAccount.publicKey,
//     })
//     .signers([counterAccount]) //include counter keypair as additional signer
//     .rpc({ skipPreflight: true });

//   // Fetch counter account data
//   const accountData = await program.account.counter.fetch(
//     counterAccount.publicKey
//   );

//   console.log(`Transaction Signature: ${transactionSignature}`);
//   console.log(`Count: ${accountData.count}`);
// });

// it("Increment", async () => {
//   //Invoke the increment instruction
//   const transactionSignature = await program.methods
//     .increment()
//     .accounts({
//       counter: counterAccount.publicKey,
//     })
//     .rpc();

//   // read to Fetch counter account data
//   const accountData = await program.account.counter.fetch(
//     counterAccount.publicKey
//   );

//   console.log(`Transaction Signature: ${transactionSignature}`);
//   console.log(`Count: ${accountData.count}`);



// });






// describe("counter", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.Counter as Program<Counter>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });
