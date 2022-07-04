import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { expect } from "chai";
import { Solzen } from "../target/types/solzen";
const util = require('util')

describe("solzen", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()

  anchor.setProvider(provider);

  const program = anchor.workspace.Solzen as Program<Solzen>;

  it("should validate an user as human", async () => {
    const child = anchor.web3.Keypair.generate()
    const [userAccount, _bump] = findProgramAddressSync([
      anchor.utils.bytes.utf8.encode('child'),
      child.publicKey.toBuffer()
    ], program.programId);
    const tx = await program.methods
      .validateHuman(child.publicKey)
      .accounts({
        validation: userAccount,
      })
      .rpc()


    console.log("Your transaction signature", tx);

    const validationAccount = await program.account.validation.all();
    expect(validationAccount.length).eq(1)
    const valAcc = await program.account.validation.fetch(userAccount)
    expect(valAcc.parent.toBase58()).to.eq(provider.wallet.publicKey.toBase58())
    expect(valAcc.child.toBase58()).to.eq(child.publicKey.toBase58())
  });

  it("should validate the parent signature", async () => {
    const wrongSigner = anchor.web3.Keypair.generate()
    const child = anchor.web3.Keypair.generate()
    const [userAccount, _bump] = findProgramAddressSync([
      anchor.utils.bytes.utf8.encode('child'),
      child.publicKey.toBuffer()
    ], program.programId)

    const error = await program.methods
      .validateHuman(child.publicKey)
      .accounts({
        validation: userAccount,
        parent: wrongSigner.publicKey
      })
      .rpc()
      .catch(e => e)

    expect(error.message).to.equal('Signature verification failed')
  });

  it("should validate only 1x", async () => {
    const child = anchor.web3.Keypair.generate()
    const [userAccount, _bump] = findProgramAddressSync([
      anchor.utils.bytes.utf8.encode('child'),
      child.publicKey.toBuffer()
    ], program.programId);

    const tx = await program.methods
      .validateHuman(child.publicKey)
      .accounts({
        validation: userAccount,
      })
      .rpc()

    await new Promise((resolve) => setTimeout(() => resolve(1), 1000))

    console.log("Your transaction signature", tx);
    const tx2 = await program.methods
      .validateHuman(child.publicKey)
      .accounts({
        validation: userAccount,
      })
      .rpc()

    const validationAccount = await program.account.validation.all();
    expect(validationAccount.length).eq(1)
    const valAcc = await program.account.validation.fetch(userAccount)
    expect(valAcc.parent.toBase58()).to.eq(provider.wallet.publicKey.toBase58())
    expect(valAcc.child.toBase58()).to.eq(child.publicKey.toBase58())
  })
});
