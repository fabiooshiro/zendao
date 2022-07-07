import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";
import { Solzen } from "../target/types/solzen";
const util = require('util')

describe("solzen", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env()

	anchor.setProvider(provider);

	const program = anchor.workspace.Solzen as Program<Solzen>;

	it.only("should validate an user as human", async () => {
		const connection = provider.connection

		const Keypair = anchor.web3.Keypair;
		const payer = Keypair.generate();
		const mintAuthority = Keypair.generate();
		const freezeAuthority = Keypair.generate();

		const airdropSignature = await connection.requestAirdrop(
			payer.publicKey,
			LAMPORTS_PER_SOL,
		);

		await connection.confirmTransaction(airdropSignature);
		const mint = await createMint(
			connection,
			payer,
			mintAuthority.publicKey,
			freezeAuthority.publicKey,
			9 // We are using 9 to match the CLI decimal default exactly
		);

		console.log(mint.toBase58());
		const child = anchor.web3.Keypair.generate()

		const tokenAccount = await getOrCreateAssociatedTokenAccount(
			connection,
			payer,
			mint,
			child.publicKey
		)
		console.log('owner', child.publicKey.toBase58())
		console.log(tokenAccount.address.toBase58());

		await mintTo(
			connection,
			payer,
			mint,
			tokenAccount.address,
			mintAuthority,
			100000000000 // because decimals for the mint are set to 9 
		)

		const [userAccount, _bump] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			child.publicKey.toBuffer()
		], program.programId);

		const tx = await program.methods
			.validateHuman(child.publicKey)
			.accounts({
				validation: userAccount,
				tokenAccount: tokenAccount.address,
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

		expect(tx).to.be.a('string')
		console.log("Your transaction signature", tx);
		const error = await program.methods
			.validateHuman(child.publicKey)
			.accounts({
				validation: userAccount,
			})
			.rpc().catch(e => e)

		expect(error.message).to.equal('failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x0')
	})
});

