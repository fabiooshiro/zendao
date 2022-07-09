import * as anchor from "@project-serum/anchor";
import { BorshAccountsCoder, Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";
import { Solzen } from "../target/types/solzen";
import Factory from "./Factory";
const util = require('util')

describe("solzen", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env()

	anchor.setProvider(provider);

	const program = anchor.workspace.Solzen as Program<Solzen>;

	const connection = provider.connection

	it("should initialize the DAO", async () => {
		const { mint, payer } = await Factory.createMint();

		const program = await Factory.programPaidBy(payer)
		const [daoPubkey, _bump] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('dao'),
			mint.toBuffer()
		], program.programId);

		const [userAccount, _bump2] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			payer.publicKey.toBuffer()
		], program.programId);
		
		const tx = await program.methods
			.initialize(mint, new anchor.BN(1))
			.accounts({
				zendao: daoPubkey,
				validation: userAccount,
			})
			.rpc()
		console.log("Your transaction signature", tx);
		const daoAcc = await program.account.zendao.fetch(daoPubkey)
		expect(daoAcc.token.toBase58()).to.eq(mint.toBase58())
		expect(daoAcc.founder.toBase58()).to.eq(payer.publicKey.toBase58())
	})

	it("should validate an user as human", async () => {
		const { mint, payer, mintAuthority, daoPubkey } = await Factory.createDAO()
		const child = anchor.web3.Keypair.generate()
		const tokenAccount = await getOrCreateAssociatedTokenAccount(
			connection,
			payer,
			mint,
			child.publicKey
		)

		await mintTo(
			connection,
			payer,
			mint,
			tokenAccount.address,
			mintAuthority,
			// 100000000000 // because decimals for the mint are set to 9 
			10
		)

		const program = await Factory.programPaidBy(payer)

		const [userAccount, _bump] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			child.publicKey.toBuffer()
		], program.programId);

		const [parentValidation, _bump2] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			payer.publicKey.toBuffer()
		], program.programId);

		const tx = await program.methods
			.validateHuman(child.publicKey)
			.accounts({
				validation: userAccount,
				tokenAccount: tokenAccount.address,
				zendao: daoPubkey,
				parentValidation
			})
			.rpc()

		console.log("Your transaction signature", tx);

		const valAcc = await program.account.validation.fetch(userAccount)
		expect(valAcc.parent.toBase58()).to.eq(payer.publicKey.toBase58())
		expect(valAcc.child.toBase58()).to.eq(child.publicKey.toBase58())
	});

	it("should validate the parent signature", async () => {
		const { mint, payer, daoPubkey } = await Factory.createDAO()
		const wrongSigner = anchor.web3.Keypair.generate()
		const child = anchor.web3.Keypair.generate()
		const [userAccount, _bump] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			child.publicKey.toBuffer()
		], program.programId)

		const tokenAccount = await getOrCreateAssociatedTokenAccount(
			connection,
			payer,
			mint,
			child.publicKey
		)

		const [parentValidation, _bump2] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			payer.publicKey.toBuffer()
		], program.programId);

		const error = await program.methods
			.validateHuman(child.publicKey)
			.accounts({
				validation: userAccount,
				parent: wrongSigner.publicKey,
				tokenAccount: tokenAccount.address,
				zendao: daoPubkey,
				parentValidation
			})
			.rpc()
			.catch(e => e)

		expect(error.message).to.equal('Signature verification failed')
	});

	it("should validate only 1x", async () => {
		const { mint, payer, mintAuthority, daoPubkey } = await Factory.createDAO()
		const child = anchor.web3.Keypair.generate()
		const program = await Factory.programPaidBy(payer)
		const [userAccount, _bump] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			child.publicKey.toBuffer()
		], program.programId);

		const tokenAccount = await getOrCreateAssociatedTokenAccount(
			connection,
			payer,
			mint,
			child.publicKey
		)

		await mintTo(
			connection,
			payer,
			mint,
			tokenAccount.address,
			mintAuthority,
			// 100000000000 // because decimals for the mint are set to 9 
			10
		)

		const [parentValidation, _bump2] = findProgramAddressSync([
			anchor.utils.bytes.utf8.encode('child'),
			payer.publicKey.toBuffer()
		], program.programId);

		const tx = await program.methods
			.validateHuman(child.publicKey)
			.accounts({
				validation: userAccount,
				tokenAccount: tokenAccount.address,
				zendao: daoPubkey,
				parentValidation
			})
			.rpc()

		await new Promise((resolve) => setTimeout(() => resolve(1), 1000))

		expect(tx).to.be.a('string')
		console.log("Your transaction signature", tx);
		const error = await program.methods
			.validateHuman(child.publicKey)
			.accounts({
				validation: userAccount,
				tokenAccount: tokenAccount.address,
				zendao: daoPubkey,
				parentValidation
			})
			.rpc().catch(e => e)

		expect(error.message).to.equal('failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x0')
	})

	xdescribe('eating glass', () => {
		it("should dig a way to decode ZenDAO", async () => {
			const { mint, payer } = await Factory.createMint();

			const [daoPubkey, _bump] = findProgramAddressSync([
				anchor.utils.bytes.utf8.encode('dao'),
				mint.toBuffer()
			], program.programId);

			const tx = await program.methods
				.initialize(mint, new anchor.BN(1))
				.accounts({
					zendao: daoPubkey
				})
				.rpc()
			console.log("Your transaction signature", tx);
			const accInfo = await program.account.zendao.getAccountInfo(daoPubkey)
			const daoAcc = program.account.zendao.coder.accounts.decodeUnchecked('Zendao', accInfo.data)
			const disc = BorshAccountsCoder.accountDiscriminator('Zendao')
			const disc2 = BorshAccountsCoder.accountDiscriminator('Validator')
			console.log(accInfo.data.slice(0, 8).toString('hex'))
			console.log(disc.toString('hex'))
			console.log(disc2.toString('hex'))
		})
	})
});

